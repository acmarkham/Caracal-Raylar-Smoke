#[cfg(not(test))]
use core::future::pending;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Channel, Receiver as ChannelReceiver};
use embassy_sync::watch::{Receiver as WatchReceiver, Watch};
#[cfg(not(test))]
use embassy_time::Timer;
use embassy_time::{Duration, Instant};
use heapless::Vec;

use crate::composites::{CompositeEvaluation, CompositeSensor, CompositeValidationError};
use crate::source::{RegisteredSource, SensorRegistration};
use crate::thresholds::ThresholdEngine;
use crate::{
    ReadingError, ReadingStatus, SensorEvent, SensorId, SensorKind, SensorReading, SensorSnapshot,
    SensorValue, ThresholdError, ThresholdRule,
};

pub const DEFAULT_SENSOR_SOURCES: usize = 8;
pub const DEFAULT_SENSOR_CAPACITY: usize = 16;
pub const DEFAULT_COMPOSITES: usize = 8;
pub const DEFAULT_SENSOR_WATCHERS: usize = 4;
pub const DEFAULT_SENSOR_THRESHOLDS: usize = 8;
pub const DEFAULT_EVENT_DEPTH: usize = 8;
pub const DEFAULT_FAILURES_BEFORE_FAULT: u32 = 3;
pub const DEFAULT_SENSOR_POLL_INTERVAL: Duration = Duration::from_secs(10);

pub type SensorMutex = CriticalSectionRawMutex;
pub type SensorStateReceiver<'a, const SENSORS: usize, const WATCHERS: usize> =
    WatchReceiver<'a, SensorMutex, SensorSnapshot<SENSORS>, WATCHERS>;
pub type SensorEventReceiver<'a, const EVENTS: usize> =
    ChannelReceiver<'a, SensorMutex, SensorEvent, EVENTS>;

pub struct SensorResources<
    const SENSORS: usize = DEFAULT_SENSOR_CAPACITY,
    const WATCHERS: usize = DEFAULT_SENSOR_WATCHERS,
    const EVENTS: usize = DEFAULT_EVENT_DEPTH,
> {
    latest: Watch<SensorMutex, SensorSnapshot<SENSORS>, WATCHERS>,
    events: Channel<SensorMutex, SensorEvent, EVENTS>,
}

impl<const SENSORS: usize, const WATCHERS: usize, const EVENTS: usize>
    SensorResources<SENSORS, WATCHERS, EVENTS>
{
    pub const fn new() -> Self {
        Self {
            latest: Watch::new_with(SensorSnapshot::empty()),
            events: Channel::new(),
        }
    }

    pub fn state(&self) -> SensorSnapshot<SENSORS> {
        self.latest.try_get().unwrap_or_else(SensorSnapshot::empty)
    }

    pub fn state_receiver(&self) -> Option<SensorStateReceiver<'_, SENSORS, WATCHERS>> {
        self.latest.receiver()
    }

    pub fn event_receiver(&self) -> SensorEventReceiver<'_, EVENTS> {
        self.events.receiver()
    }
}

impl<const SENSORS: usize, const WATCHERS: usize, const EVENTS: usize> Default
    for SensorResources<SENSORS, WATCHERS, EVENTS>
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RegistrationError {
    SourceCapacity,
    SensorCapacity,
    CompositeCapacity,
    DuplicateId(SensorId),
    ZeroPollInterval,
    Composite(CompositeValidationError),
}

pub struct SensorService<
    'a,
    const SOURCES: usize = DEFAULT_SENSOR_SOURCES,
    const SENSORS: usize = DEFAULT_SENSOR_CAPACITY,
    const COMPOSITES: usize = DEFAULT_COMPOSITES,
    const WATCHERS: usize = DEFAULT_SENSOR_WATCHERS,
    const RULES: usize = DEFAULT_SENSOR_THRESHOLDS,
    const EVENTS: usize = DEFAULT_EVENT_DEPTH,
> {
    resources: &'a SensorResources<SENSORS, WATCHERS, EVENTS>,
    sources: Vec<RegisteredSource<'a>, SOURCES>,
    composites: Vec<CompositeSensor, COMPOSITES>,
    thresholds: ThresholdEngine<RULES>,
    snapshot: SensorSnapshot<SENSORS>,
    failures_before_fault: u32,
}

impl<
        'a,
        const SOURCES: usize,
        const SENSORS: usize,
        const COMPOSITES: usize,
        const WATCHERS: usize,
        const RULES: usize,
        const EVENTS: usize,
    > SensorService<'a, SOURCES, SENSORS, COMPOSITES, WATCHERS, RULES, EVENTS>
{
    pub const fn new(resources: &'a SensorResources<SENSORS, WATCHERS, EVENTS>) -> Self {
        Self {
            resources,
            sources: Vec::new(),
            composites: Vec::new(),
            thresholds: ThresholdEngine::new(),
            snapshot: SensorSnapshot::empty(),
            failures_before_fault: DEFAULT_FAILURES_BEFORE_FAULT,
        }
    }

    pub fn set_failures_before_fault(&mut self, failures: u32) {
        self.failures_before_fault = failures.max(1);
    }

    pub fn register_source(
        &mut self,
        registration: SensorRegistration<'a>,
    ) -> Result<(), RegistrationError> {
        if registration.poll_interval.as_ticks() == 0 {
            return Err(RegistrationError::ZeroPollInterval);
        }
        self.ensure_new_id(registration.descriptor.id)?;
        if self.sources.is_full() {
            return Err(RegistrationError::SourceCapacity);
        }
        self.add_reading(registration.descriptor)?;
        self.sources
            .push(RegisteredSource {
                descriptor: registration.descriptor,
                poll_interval: registration.poll_interval,
                next_due: Instant::from_ticks(0),
                source: registration.source,
            })
            .map_err(|_| RegistrationError::SourceCapacity)
    }

    pub fn register_composite(
        &mut self,
        composite: impl Into<CompositeSensor>,
    ) -> Result<(), RegistrationError> {
        let composite = composite.into();
        let descriptor = composite.descriptor();
        self.ensure_new_id(descriptor.id)?;
        if self.composites.is_full() {
            return Err(RegistrationError::CompositeCapacity);
        }
        composite
            .validate(|id| self.sensor_configuration(id))
            .map_err(RegistrationError::Composite)?;
        self.add_reading(descriptor)?;
        self.composites
            .push(composite)
            .map_err(|_| RegistrationError::CompositeCapacity)
    }

    pub fn set_threshold(&mut self, rule: ThresholdRule) -> Result<(), ThresholdError> {
        let kind = self
            .snapshot
            .reading(rule.sensor())
            .map(|reading| reading.descriptor.kind)
            .ok_or(ThresholdError::UnknownSensor(rule.sensor()))?;
        self.thresholds.add(rule, kind)
    }

    pub fn state(&self) -> &SensorSnapshot<SENSORS> {
        &self.snapshot
    }

    pub fn poll_one_due(&mut self, now: Instant) -> Option<Vec<SensorEvent, RULES>> {
        let index = self
            .sources
            .iter()
            .position(|source| source.next_due <= now)?;
        let (descriptor, result) = {
            let source = &mut self.sources[index];
            source.next_due = now + source.poll_interval;
            (source.descriptor, source.source.sample())
        };
        self.snapshot.stats.polls_attempted = self.snapshot.stats.polls_attempted.saturating_add(1);
        let mut events = Vec::new();
        match result {
            Ok(value) if value.compatible_with(descriptor.kind) => {
                self.record_success(descriptor.id, value, now, &mut events);
            }
            Ok(_) => self.record_failure(descriptor.id, now, ReadingError::IncompatibleValue),
            Err(error) => {
                self.record_failure(descriptor.id, now, ReadingError::Source(error.code));
            }
        }
        self.evaluate_composites(now, &mut events);
        self.snapshot.generation = self.snapshot.generation.wrapping_add(1);
        self.snapshot.published_at = now;
        self.snapshot.stats.threshold_events_emitted = self
            .snapshot
            .stats
            .threshold_events_emitted
            .saturating_add(events.len() as u64);
        let anticipated_depth = self.resources.events.len().saturating_add(events.len());
        self.snapshot.stats.event_queue_high_water = self
            .snapshot
            .stats
            .event_queue_high_water
            .max(anticipated_depth.min(EVENTS).min(u16::MAX as usize) as u16);
        self.resources.latest.sender().send(self.snapshot.clone());
        Some(events)
    }

    #[cfg(not(test))]
    pub async fn run(mut self) -> ! {
        loop {
            let Some(deadline) = self.sources.iter().map(|source| source.next_due).min() else {
                pending::<()>().await;
                unreachable!();
            };
            Timer::at(deadline).await;
            if let Some(events) = self.poll_one_due(Instant::now()) {
                for event in events {
                    self.resources.events.send(event).await;
                }
            }
        }
    }

    fn ensure_new_id(&self, id: SensorId) -> Result<(), RegistrationError> {
        if self.snapshot.reading(id).is_some() {
            Err(RegistrationError::DuplicateId(id))
        } else {
            Ok(())
        }
    }

    fn add_reading(
        &mut self,
        descriptor: crate::SensorDescriptor,
    ) -> Result<(), RegistrationError> {
        self.snapshot
            .readings
            .push(SensorReading::unavailable(descriptor))
            .map_err(|_| RegistrationError::SensorCapacity)
    }

    fn sensor_configuration(&self, id: SensorId) -> Option<(SensorKind, Option<Duration>)> {
        let reading = self.snapshot.reading(id)?;
        let interval = self
            .sources
            .iter()
            .find(|source| source.descriptor.id == id)
            .map(|source| source.poll_interval);
        Some((reading.descriptor.kind, interval))
    }

    fn record_success(
        &mut self,
        id: SensorId,
        value: SensorValue,
        now: Instant,
        events: &mut Vec<SensorEvent, RULES>,
    ) {
        self.snapshot.stats.polls_succeeded = self.snapshot.stats.polls_succeeded.saturating_add(1);
        let reading = self.snapshot.reading_mut(id).expect("registered source");
        reading.value = Some(value);
        reading.status = ReadingStatus::Current;
        reading.sequence = reading.sequence.wrapping_add(1);
        reading.last_attempt = now;
        reading.last_success = Some(now);
        reading.consecutive_errors = 0;
        reading.last_error = None;
        self.thresholds.evaluate(*reading, events);
    }

    fn record_failure(&mut self, id: SensorId, now: Instant, error: ReadingError) {
        self.snapshot.stats.polls_failed = self.snapshot.stats.polls_failed.saturating_add(1);
        let reading = self.snapshot.reading_mut(id).expect("registered source");
        reading.last_attempt = now;
        reading.consecutive_errors = reading.consecutive_errors.saturating_add(1);
        reading.total_errors = reading.total_errors.saturating_add(1);
        reading.last_error = Some(error);
        reading.status = if reading.consecutive_errors >= self.failures_before_fault {
            ReadingStatus::Fault
        } else if reading.value.is_some() {
            ReadingStatus::Stale
        } else {
            ReadingStatus::Unavailable
        };
    }

    fn evaluate_composites(&mut self, now: Instant, events: &mut Vec<SensorEvent, RULES>) {
        for composite in &mut self.composites {
            self.snapshot.stats.composites_evaluated =
                self.snapshot.stats.composites_evaluated.saturating_add(1);
            let id = composite.descriptor().id;
            match composite.evaluate(&self.snapshot, now) {
                CompositeEvaluation::NoChange => {}
                CompositeEvaluation::Unavailable => {
                    let reading = self.snapshot.reading_mut(id).expect("registered composite");
                    reading.last_attempt = now;
                    reading.consecutive_errors = reading.consecutive_errors.saturating_add(1);
                    reading.total_errors = reading.total_errors.saturating_add(1);
                    reading.last_error = Some(ReadingError::CompositeInputUnavailable);
                    reading.status = if reading.value.is_some() {
                        ReadingStatus::Stale
                    } else {
                        ReadingStatus::Unavailable
                    };
                }
                CompositeEvaluation::Updated(sample) => {
                    let reading = self.snapshot.reading_mut(id).expect("registered composite");
                    reading.value = Some(sample.value);
                    reading.status = ReadingStatus::Current;
                    reading.sequence = reading.sequence.wrapping_add(1);
                    reading.last_attempt = now;
                    reading.last_success = Some(sample.observed_at);
                    reading.consecutive_errors = 0;
                    reading.last_error = None;
                    self.thresholds.evaluate(*reading, events);
                }
            }
        }
    }
}
