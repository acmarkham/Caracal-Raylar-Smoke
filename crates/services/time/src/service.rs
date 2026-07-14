use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Channel, Sender};
use embassy_sync::watch::{Receiver, Watch};
use embassy_time::{with_timeout, Instant};

use crate::{Anchor, TimeConfig, TimeError, TimeEstimator, TimeState, UtcTimestamp};

pub type TimeMutex = CriticalSectionRawMutex;
pub type AnchorSender<'a, const ANCHOR_DEPTH: usize> = Sender<'a, TimeMutex, Anchor, ANCHOR_DEPTH>;
pub type TimeStateReceiver<'a, const WATCHERS: usize> =
    Receiver<'a, TimeMutex, TimeState, WATCHERS>;

pub struct TimeResources<const WATCHERS: usize, const ANCHOR_DEPTH: usize> {
    anchors: Channel<TimeMutex, Anchor, ANCHOR_DEPTH>,
    state: Watch<TimeMutex, TimeState, WATCHERS>,
}

impl<const WATCHERS: usize, const ANCHOR_DEPTH: usize> TimeResources<WATCHERS, ANCHOR_DEPTH> {
    pub const fn new() -> Self {
        Self {
            anchors: Channel::new(),
            state: Watch::new_with(TimeState::invalid()),
        }
    }

    pub fn anchor_sender(&self) -> AnchorSender<'_, ANCHOR_DEPTH> {
        self.anchors.sender()
    }
    pub fn state_receiver(&self) -> Option<TimeStateReceiver<'_, WATCHERS>> {
        self.state.receiver()
    }
    pub fn time_state(&self) -> TimeState {
        self.state.try_get().unwrap_or_else(TimeState::invalid)
    }

    pub fn system_to_utc(&self, system_time: Instant) -> Result<UtcTimestamp, TimeError> {
        self.time_state().system_to_utc(system_time)
    }

    pub fn utc_to_system(&self, utc: UtcTimestamp) -> Result<Instant, TimeError> {
        self.time_state().utc_to_system(utc)
    }

    pub fn current_utc(&self) -> Result<UtcTimestamp, TimeError> {
        self.system_to_utc(Instant::now())
    }
}

impl<const WATCHERS: usize, const ANCHOR_DEPTH: usize> Default
    for TimeResources<WATCHERS, ANCHOR_DEPTH>
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct TimeService<const WATCHERS: usize, const ANCHOR_DEPTH: usize> {
    resources: &'static TimeResources<WATCHERS, ANCHOR_DEPTH>,
    estimator: TimeEstimator,
    config: TimeConfig,
}

impl<const WATCHERS: usize, const ANCHOR_DEPTH: usize> TimeService<WATCHERS, ANCHOR_DEPTH> {
    pub const fn new(
        resources: &'static TimeResources<WATCHERS, ANCHOR_DEPTH>,
        config: TimeConfig,
    ) -> Self {
        Self {
            resources,
            estimator: TimeEstimator::new(config),
            config,
        }
    }

    pub async fn run(mut self) -> ! {
        let anchors = self.resources.anchors.receiver();
        let state = self.resources.state.sender();
        state.send(self.estimator.state());
        loop {
            if let Ok(anchor) = with_timeout(self.config.publish_interval, anchors.receive()).await
            {
                defmt::info!(
                    "received anchor: {:?}, quality: {:?}",
                    anchor, anchor.quality
                );
                self.estimator.ingest(anchor);
            }
            state.send(self.estimator.update_holdover(Instant::now()));
        }
    }
}
