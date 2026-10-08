use core::sync::atomic::{AtomicU16, AtomicU32, Ordering};

use embassy_futures::select::{select, Either};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_sync::watch::{Receiver as WatchReceiver, Watch};
use embassy_time::{Duration, Instant, Timer};
use embedded_hal::digital::{InputPin, OutputPin};
use embedded_hal_async::{digital::Wait, spi::SpiDevice};
use heapless::Vec;
use raylar_drivers::radio::{Error as DriverError, GfskPacketStatus, RadioDriver, RxMetrics};
use raylar_time_service::{TimeState, UtcStatus, UtcTimestamp};

use crate::link::ChannelProfile;
use crate::{
    guard_interval, FrameError, FrameHeader, FrameType, RadioPriority, RadioServiceConfig,
    RadioServiceError, RadioServiceStats, Reservation, ScheduleError, Scheduler,
};

pub const MAX_FRAME_LEN: usize = u8::MAX as usize;
pub const DEFAULT_JOB_DEPTH: usize = 8;
pub const DEFAULT_EVENT_DEPTH: usize = 8;
pub const DEFAULT_STATE_WATCHERS: usize = 4;

pub type RadioMutex = CriticalSectionRawMutex;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct JobId(pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FrameBuffer(Vec<u8, MAX_FRAME_LEN>);

impl FrameBuffer {
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    pub fn from_slice(bytes: &[u8]) -> Result<Self, FrameError> {
        let mut buffer = Vec::new();
        buffer
            .extend_from_slice(bytes)
            .map_err(|_| FrameError::FrameTooLarge)?;
        Ok(Self(buffer))
    }

    pub fn encode(header: FrameHeader, payload: &[u8]) -> Result<Self, FrameError> {
        let mut bytes = [0u8; MAX_FRAME_LEN];
        let length = header.encode(payload, &mut bytes)?;
        Self::from_slice(&bytes[..length])
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn frame_type(&self) -> Result<FrameType, FrameError> {
        crate::heartbeat_v4::frame_type(self.as_slice())
    }
}

impl Default for FrameBuffer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioTxJob {
    pub earliest: Instant,
    pub deadline: Instant,
    pub profile: ChannelProfile,
    pub priority: RadioPriority,
    pub payload: FrameBuffer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RxPurpose {
    Broadcast,
    Presence,
    Peer(NodeId),
}

use crate::NodeId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioRxJob {
    pub start: Instant,
    pub end: Instant,
    pub profile: ChannelProfile,
    pub priority: RadioPriority,
    pub purpose: RxPurpose,
}

impl RadioRxJob {
    /// Convert a UTC receive window to monotonic time and widen both sides by
    /// the uncertainty budget supplied by the authoritative Time Service.
    pub fn guarded_utc_window(
        start_utc: UtcTimestamp,
        end_utc: UtcTimestamp,
        time: &TimeState,
        profile: ChannelProfile,
        priority: RadioPriority,
        purpose: RxPurpose,
        config: &RadioServiceConfig,
    ) -> Result<Self, ScheduleError> {
        if time.utc_status == UtcStatus::Invalid {
            return Err(ScheduleError::UtcUnavailable);
        }
        if time.uncertainty_us > config.maximum_scheduled_uncertainty.as_micros() {
            return Err(ScheduleError::UtcUncertaintyTooHigh);
        }
        if end_utc.as_micros() <= start_utc.as_micros() {
            return Err(ScheduleError::InvalidSchedule);
        }
        let start = time
            .utc_to_system(start_utc)
            .map_err(|_| ScheduleError::UtcUnavailable)?;
        let end = time
            .utc_to_system(end_utc)
            .map_err(|_| ScheduleError::UtcUnavailable)?;
        let guard = guard_interval(
            Duration::from_micros(time.uncertainty_us),
            config.expected_remote_uncertainty,
            config.scheduling_uncertainty,
            config.propagation_allowance,
            config.engineering_margin,
        );
        Ok(Self {
            start: Instant::from_ticks(start.as_ticks().saturating_sub(guard.as_ticks())),
            end: Instant::from_ticks(end.as_ticks().saturating_add(guard.as_ticks())),
            profile,
            priority,
            purpose,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
// Keeping the payload inline is intentional: the service is heapless and the
// resulting channel storage remains statically visible in RadioResources.
#[allow(clippy::large_enum_variant)]
pub enum RadioJob {
    Transmit { id: JobId, job: RadioTxJob },
    Receive { id: JobId, job: RadioRxJob },
    Cancel { id: JobId },
}

impl RadioJob {
    pub const fn id(&self) -> JobId {
        match self {
            Self::Transmit { id, .. } | Self::Receive { id, .. } | Self::Cancel { id } => *id,
        }
    }

    fn reservation(&self) -> Reservation {
        match self {
            Self::Transmit { id, job } => Reservation {
                job_id: *id,
                start: job.earliest,
                end: job.deadline,
                priority: job.priority,
            },
            Self::Receive { id, job } => Reservation {
                job_id: *id,
                start: job.start,
                end: job.end,
                priority: job.priority,
            },
            Self::Cancel { .. } => unreachable!("cancel requests are never scheduled"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DriverPacketMetadata {
    pub packet_complete_at: Instant,
    pub frequency_hz: u32,
    pub rssi_dbm_x2: i16,
    pub snr_db_x4: Option<i16>,
    pub gfsk_status: Option<GfskPacketStatus>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
// Received frames remain inline so event capacity has deterministic storage.
#[allow(clippy::large_enum_variant)]
pub enum RadioEvent {
    Completed {
        id: JobId,
    },
    Received {
        id: JobId,
        frame: FrameBuffer,
        metadata: DriverPacketMetadata,
    },
    RxWindowClosed {
        id: JobId,
    },
    PacketRejected {
        id: JobId,
    },
    Cancelled {
        id: JobId,
    },
    Rejected {
        id: JobId,
        error: RadioServiceError,
    },
    Failed {
        id: JobId,
        error: RadioServiceError,
    },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadioMode {
    #[default]
    Initializing,
    Idle,
    Preparing,
    Transmitting,
    Receiving,
    Recovering,
    Sleeping,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RadioServiceState {
    pub mode: RadioMode,
    pub current_job: Option<JobId>,
    pub stats: RadioServiceStats,
}

pub type RadioEventReceiver<'a, const EVENTS: usize> = Receiver<'a, RadioMutex, RadioEvent, EVENTS>;
pub type RadioStateReceiver<'a, const WATCHERS: usize> =
    WatchReceiver<'a, RadioMutex, RadioServiceState, WATCHERS>;

pub struct RadioResources<
    const JOBS: usize = DEFAULT_JOB_DEPTH,
    const EVENTS: usize = DEFAULT_EVENT_DEPTH,
    const WATCHERS: usize = DEFAULT_STATE_WATCHERS,
> {
    requests: Channel<RadioMutex, RadioJob, JOBS>,
    events: Channel<RadioMutex, RadioEvent, EVENTS>,
    state: Watch<RadioMutex, RadioServiceState, WATCHERS>,
    next_job_id: AtomicU32,
    neighbour_count: AtomicU16,
    enabled: Watch<RadioMutex, bool, 1>,
}

impl<const JOBS: usize, const EVENTS: usize, const WATCHERS: usize>
    RadioResources<JOBS, EVENTS, WATCHERS>
{
    pub const fn new() -> Self {
        Self {
            requests: Channel::new(),
            events: Channel::new(),
            state: Watch::new_with(RadioServiceState {
                mode: RadioMode::Initializing,
                current_job: None,
                stats: RadioServiceStats {
                    frames_tx: 0,
                    frames_rx: 0,
                    heartbeat_tx: 0,
                    presence_tx: 0,
                    presence_rx: 0,
                    malformed_frames: 0,
                    unsupported_frames: 0,
                    schedule_misses: 0,
                    scheduler_conflicts: 0,
                    queue_drops: 0,
                    neighbour_count: 0,
                    radio_errors: 0,
                },
            }),
            next_job_id: AtomicU32::new(1),
            neighbour_count: AtomicU16::new(0),
            enabled: Watch::new_with(true),
        }
    }

    pub fn handle(&self) -> RadioHandle<'_, JOBS> {
        RadioHandle {
            requests: self.requests.sender(),
            next_job_id: &self.next_job_id,
        }
    }

    /// Disable cancels pending/in-flight work and puts the sole-owned radio
    /// into retained sleep. Re-enable wakes it before accepting fresh jobs.
    pub fn set_enabled(&self, enabled: bool) {
        if self.enabled.try_get() != Some(enabled) {
            self.enabled.sender().send(enabled);
        }
    }

    pub fn event_receiver(&self) -> RadioEventReceiver<'_, EVENTS> {
        self.events.receiver()
    }

    pub fn state_receiver(&self) -> Option<RadioStateReceiver<'_, WATCHERS>> {
        self.state.receiver()
    }

    pub fn state(&self) -> RadioServiceState {
        self.state.try_get().unwrap_or_default()
    }

    /// Publish the latest bounded neighbour-table size into service stats.
    pub fn set_neighbour_count(&self, count: usize) {
        let count = count.min(u16::MAX as usize) as u16;
        self.neighbour_count.store(count, Ordering::Relaxed);
        let mut state = self.state();
        state.stats.neighbour_count = count;
        self.state.sender().send(state);
    }
}

impl<const JOBS: usize, const EVENTS: usize, const WATCHERS: usize> Default
    for RadioResources<JOBS, EVENTS, WATCHERS>
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct RadioHandle<'a, const JOBS: usize> {
    requests: Sender<'a, RadioMutex, RadioJob, JOBS>,
    next_job_id: &'a AtomicU32,
}

impl<'a, const JOBS: usize> Clone for RadioHandle<'a, JOBS> {
    fn clone(&self) -> Self {
        Self {
            requests: self.requests,
            next_job_id: self.next_job_id,
        }
    }
}

impl<'a, const JOBS: usize> RadioHandle<'a, JOBS> {
    pub fn try_submit_tx(&self, job: RadioTxJob) -> Result<JobId, ScheduleError> {
        let id = self.allocate_id();
        self.requests
            .try_send(RadioJob::Transmit { id, job })
            .map_err(|_| ScheduleError::QueueFull)?;
        Ok(id)
    }

    pub fn try_reserve_rx(&self, job: RadioRxJob) -> Result<JobId, ScheduleError> {
        let id = self.allocate_id();
        self.requests
            .try_send(RadioJob::Receive { id, job })
            .map_err(|_| ScheduleError::QueueFull)?;
        Ok(id)
    }

    pub async fn submit_tx(&self, job: RadioTxJob) -> JobId {
        let id = self.allocate_id();
        self.requests.send(RadioJob::Transmit { id, job }).await;
        id
    }

    pub async fn reserve_rx(&self, job: RadioRxJob) -> JobId {
        let id = self.allocate_id();
        self.requests.send(RadioJob::Receive { id, job }).await;
        id
    }

    /// Cancel a queued reservation. Cancellation is best-effort: a driver
    /// operation that has already started cannot be interrupted safely.
    pub fn try_cancel(&self, id: JobId) -> Result<(), ScheduleError> {
        self.requests
            .try_send(RadioJob::Cancel { id })
            .map_err(|_| ScheduleError::QueueFull)
    }

    fn allocate_id(&self) -> JobId {
        let id = self.next_job_id.fetch_add(1, Ordering::Relaxed);
        JobId(if id == 0 { 1 } else { id })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadioDeviceError {
    Timeout,
    PacketRejected,
    DeadlineMissed,
    Driver,
}

pub trait RadioDevice {
    async fn initialize(&mut self) -> Result<(), RadioDeviceError>;
    async fn prepare(&mut self, profile: &ChannelProfile) -> Result<(), RadioDeviceError>;
    async fn transmit(
        &mut self,
        start: Instant,
        profile: &ChannelProfile,
        payload: &[u8],
    ) -> Result<(), RadioDeviceError>;
    async fn receive(
        &mut self,
        start: Instant,
        end: Instant,
        buffer: &mut [u8],
    ) -> Result<(usize, DriverPacketMetadata), RadioDeviceError>;
    async fn recover(&mut self) -> Result<(), RadioDeviceError>;
    async fn sleep(&mut self) -> Result<(), RadioDeviceError> { Ok(()) }
    async fn wake(&mut self) -> Result<(), RadioDeviceError> { Ok(()) }
}

impl<SPI, BUSY, RESET, IRQ> RadioDevice for RadioDriver<SPI, BUSY, RESET, IRQ>
where
    SPI: SpiDevice<u8>,
    BUSY: InputPin + Wait,
    RESET: OutputPin,
    IRQ: InputPin + Wait,
{
    async fn initialize(&mut self) -> Result<(), RadioDeviceError> {
        RadioDriver::initialize(self)
            .await
            .map_err(map_driver_error)
    }

    async fn prepare(&mut self, profile: &ChannelProfile) -> Result<(), RadioDeviceError> {
        self.prepare_channel(profile.driver_channel())
            .await
            .map_err(map_driver_error)
    }

    async fn transmit(
        &mut self,
        start: Instant,
        profile: &ChannelProfile,
        payload: &[u8],
    ) -> Result<(), RadioDeviceError> {
        self.transmit_at(start, payload, profile.driver_tx())
            .await
            .map(|_| ())
            .map_err(map_driver_error)
    }

    async fn receive(
        &mut self,
        start: Instant,
        end: Instant,
        buffer: &mut [u8],
    ) -> Result<(usize, DriverPacketMetadata), RadioDeviceError> {
        let packet = self
            .receive_at(start, end, buffer)
            .await
            .map_err(map_driver_error)?;
        let (rssi_dbm_x2, snr_db_x4, gfsk_status) = match packet.metadata.metrics {
            RxMetrics::LoRa {
                rssi_dbm_x2,
                snr_db_x4,
                ..
            } => (rssi_dbm_x2, Some(snr_db_x4), None),
            RxMetrics::Gfsk {
                rssi_dbm_x2,
                status,
            } => (rssi_dbm_x2, None, Some(status)),
        };
        Ok((
            packet.payload.len(),
            DriverPacketMetadata {
                packet_complete_at: packet.metadata.packet_complete_at,
                frequency_hz: packet.metadata.frequency_hz,
                rssi_dbm_x2,
                snr_db_x4,
                gfsk_status,
            },
        ))
    }

    async fn recover(&mut self) -> Result<(), RadioDeviceError> {
        RadioDriver::recover(self).await.map_err(map_driver_error)
    }

    async fn sleep(&mut self) -> Result<(), RadioDeviceError> {
        RadioDriver::sleep(self).await.map_err(map_driver_error)
    }

    async fn wake(&mut self) -> Result<(), RadioDeviceError> {
        RadioDriver::wake(self).await.map_err(map_driver_error)
    }
}

fn map_driver_error(error: DriverError) -> RadioDeviceError {
    match error {
        DriverError::RxTimeout { .. } | DriverError::TxTimeout { .. } => RadioDeviceError::Timeout,
        DriverError::CrcRejected { .. }
        | DriverError::HeaderRejected { .. }
        | DriverError::GfskLengthRejected { .. }
        | DriverError::GfskAddressRejected { .. } => RadioDeviceError::PacketRejected,
        DriverError::DeadlineMissed { .. } => RadioDeviceError::DeadlineMissed,
        _ => RadioDeviceError::Driver,
    }
}

pub struct RadioService<
    'a,
    D,
    const JOBS: usize = DEFAULT_JOB_DEPTH,
    const EVENTS: usize = DEFAULT_EVENT_DEPTH,
    const WATCHERS: usize = DEFAULT_STATE_WATCHERS,
> {
    driver: D,
    resources: &'a RadioResources<JOBS, EVENTS, WATCHERS>,
    scheduler: Scheduler<JOBS>,
    pending: Vec<RadioJob, JOBS>,
    state: RadioServiceState,
}

impl<'a, D, const JOBS: usize, const EVENTS: usize, const WATCHERS: usize>
    RadioService<'a, D, JOBS, EVENTS, WATCHERS>
where
    D: RadioDevice,
{
    pub fn new(
        driver: D,
        resources: &'a RadioResources<JOBS, EVENTS, WATCHERS>,
        preparation_guard: embassy_time::Duration,
    ) -> Self {
        Self {
            driver,
            resources,
            scheduler: Scheduler::new(preparation_guard),
            pending: Vec::new(),
            state: RadioServiceState::default(),
        }
    }

    pub async fn run(mut self) -> ! {
        let resources = self.resources;
        let mut enabled = resources.enabled.receiver().expect("sole radio owner");
        self.initialize_until_ready().await;
        loop {
            if !resources.enabled.try_get().unwrap_or(true) {
                self.cancel_all();
                if self.driver.sleep().await.is_err() {
                    self.state.mode = RadioMode::Recovering;
                    self.publish();
                    self.initialize_until_ready().await;
                    continue;
                }
                self.state.mode = RadioMode::Sleeping;
                self.publish();
                while !resources.enabled.try_get().unwrap_or(true) {
                    enabled.changed().await;
                }
                if self.driver.wake().await.is_err() {
                    self.initialize_until_ready().await;
                }
                self.state.mode = RadioMode::Idle;
                self.publish();
            }
            let _ = select(enabled.changed(), self.work_once()).await;
        }
    }

    fn cancel_all(&mut self) {
        if let Some(id) = self.state.current_job.take() {
            self.emit(RadioEvent::Cancelled { id });
        }
        while let Some(job) = self.pending.pop() {
            self.scheduler.release(job.id());
            self.emit(RadioEvent::Cancelled { id: job.id() });
        }
        while let Ok(job) = self.resources.requests.try_receive() {
            self.emit(RadioEvent::Cancelled { id: job.id() });
        }
    }

    async fn work_once(&mut self) {
            while let Ok(job) = self.resources.requests.try_receive() {
                self.accept(job);
            }
            if self.pending.is_empty() {
                let job = self.resources.requests.receive().await;
                self.accept(job);
                return;
            }

            let next = self.next_job_index();
            let job_id = self.pending[next].id();
            let prepare_at = self
                .scheduler
                .preparation_time(job_id)
                .unwrap_or_else(Instant::now);
            if Instant::now() < prepare_at {
                match select(self.resources.requests.receive(), Timer::at(prepare_at)).await {
                    Either::First(job) => self.accept(job),
                    Either::Second(_) => {}
                }
                return;
            }

            let job = self.pending.remove(next);
            self.scheduler.release(job.id());
            self.execute(job).await;
    }

    async fn initialize_until_ready(&mut self) {
        loop {
            self.state.mode = RadioMode::Initializing;
            self.publish();
            if self.driver.initialize().await.is_ok() {
                self.state.mode = RadioMode::Idle;
                self.publish();
                return;
            }
            RadioServiceStats::increment(&mut self.state.stats.radio_errors);
            self.state.mode = RadioMode::Recovering;
            self.publish();
            Timer::after_millis(500).await;
        }
    }

    fn accept(&mut self, job: RadioJob) {
        let id = job.id();
        if matches!(&job, RadioJob::Cancel { .. }) {
            self.scheduler.release(id);
            if let Some(index) = self.pending.iter().position(|pending| pending.id() == id) {
                self.pending.remove(index);
            }
            self.emit(RadioEvent::Cancelled { id });
            self.publish();
            return;
        }
        if let RadioJob::Transmit { job: tx, .. } = &job {
            if let Err(error) = crate::heartbeat_v4::frame_type(tx.payload.as_slice()) {
                self.record_frame_error(error);
                self.emit(RadioEvent::Rejected {
                    id,
                    error: error.into(),
                });
                self.publish();
                return;
            }
        }
        let reservation = job.reservation();
        match self.scheduler.reserve(reservation, Instant::now()) {
            Ok(outcome) => {
                for evicted in outcome.evicted {
                    if let Some(index) = self.pending.iter().position(|job| job.id() == evicted) {
                        self.pending.remove(index);
                    }
                    RadioServiceStats::increment(&mut self.state.stats.scheduler_conflicts);
                    self.emit(RadioEvent::Rejected {
                        id: evicted,
                        error: ScheduleError::Conflict.into(),
                    });
                }
                if self.pending.push(job).is_err() {
                    self.scheduler.release(id);
                    RadioServiceStats::increment(&mut self.state.stats.queue_drops);
                    self.emit(RadioEvent::Rejected {
                        id,
                        error: ScheduleError::QueueFull.into(),
                    });
                }
            }
            Err(error) => {
                match error {
                    ScheduleError::Conflict => {
                        RadioServiceStats::increment(&mut self.state.stats.scheduler_conflicts)
                    }
                    ScheduleError::MissedSlot => {
                        RadioServiceStats::increment(&mut self.state.stats.schedule_misses)
                    }
                    ScheduleError::QueueFull => {
                        RadioServiceStats::increment(&mut self.state.stats.queue_drops)
                    }
                    _ => {}
                }
                self.emit(RadioEvent::Rejected {
                    id,
                    error: error.into(),
                });
            }
        }
        self.publish();
    }

    fn next_job_index(&self) -> usize {
        self.pending
            .iter()
            .enumerate()
            .min_by(|(_, left), (_, right)| {
                let left = left.reservation();
                let right = right.reservation();
                left.start
                    .cmp(&right.start)
                    .then_with(|| right.priority.cmp(&left.priority))
            })
            .map(|(index, _)| index)
            .unwrap_or(0)
    }

    async fn execute(&mut self, job: RadioJob) {
        let id = job.id();
        self.state.current_job = Some(id);
        self.state.mode = RadioMode::Preparing;
        self.publish();
        let profile = match &job {
            RadioJob::Transmit { job, .. } => &job.profile,
            RadioJob::Receive { job, .. } => &job.profile,
            RadioJob::Cancel { .. } => unreachable!("cancel requests are never executed"),
        };
        if let Err(error) = self.driver.prepare(profile).await {
            self.handle_driver_error(id, error).await;
            return;
        }

        match job {
            RadioJob::Transmit { job, .. } => {
                self.state.mode = RadioMode::Transmitting;
                self.publish();
                match self
                    .driver
                    .transmit(job.earliest, &job.profile, job.payload.as_slice())
                    .await
                {
                    Ok(()) => {
                        RadioServiceStats::increment(&mut self.state.stats.frames_tx);
                        match job.payload.frame_type() {
                            Ok(FrameType::Heartbeat) => {
                                RadioServiceStats::increment(&mut self.state.stats.heartbeat_tx)
                            }
                            Ok(FrameType::Presence) => {
                                RadioServiceStats::increment(&mut self.state.stats.presence_tx)
                            }
                            _ => {}
                        }
                        self.emit(RadioEvent::Completed { id });
                    }
                    Err(error) => {
                        self.handle_driver_error(id, error).await;
                        return;
                    }
                }
            }
            RadioJob::Receive { job, .. } => {
                self.state.mode = RadioMode::Receiving;
                self.publish();
                let mut bytes = [0u8; MAX_FRAME_LEN];
                match self.driver.receive(job.start, job.end, &mut bytes).await {
                    Ok((length, metadata)) => match FrameBuffer::from_slice(&bytes[..length]) {
                        Ok(frame) => match frame.frame_type() {
                            Ok(frame_type) => {
                                RadioServiceStats::increment(&mut self.state.stats.frames_rx);
                                if frame_type == FrameType::Presence {
                                    RadioServiceStats::increment(&mut self.state.stats.presence_rx);
                                }
                                self.emit(RadioEvent::Received {
                                    id,
                                    frame,
                                    metadata,
                                });
                            }
                            Err(error) => {
                                self.record_frame_error(error);
                                self.emit(RadioEvent::Failed {
                                    id,
                                    error: error.into(),
                                });
                            }
                        },
                        Err(error) => {
                            self.record_frame_error(error);
                            self.emit(RadioEvent::Failed {
                                id,
                                error: error.into(),
                            });
                        }
                    },
                    Err(RadioDeviceError::Timeout) => {
                        self.emit(RadioEvent::RxWindowClosed { id });
                    }
                    Err(RadioDeviceError::PacketRejected) => {
                        self.emit(RadioEvent::PacketRejected { id });
                    }
                    Err(error) => {
                        self.handle_driver_error(id, error).await;
                        return;
                    }
                }
            }
            RadioJob::Cancel { .. } => unreachable!("cancel requests are never executed"),
        }
        self.state.current_job = None;
        self.state.mode = RadioMode::Idle;
        self.publish();
    }

    async fn handle_driver_error(&mut self, id: JobId, error: RadioDeviceError) {
        let service_error = match error {
            RadioDeviceError::DeadlineMissed => {
                RadioServiceStats::increment(&mut self.state.stats.schedule_misses);
                ScheduleError::MissedSlot.into()
            }
            RadioDeviceError::Timeout
            | RadioDeviceError::PacketRejected
            | RadioDeviceError::Driver => {
                RadioServiceStats::increment(&mut self.state.stats.radio_errors);
                RadioServiceError::RadioDriver
            }
        };
        self.emit(RadioEvent::Failed {
            id,
            error: service_error,
        });
        self.state.mode = RadioMode::Recovering;
        self.publish();
        if self.driver.recover().await.is_err() {
            RadioServiceStats::increment(&mut self.state.stats.radio_errors);
            self.initialize_until_ready().await;
        }
        self.state.current_job = None;
        self.state.mode = RadioMode::Idle;
        self.publish();
    }

    fn emit(&mut self, event: RadioEvent) {
        if self.resources.events.try_send(event).is_err() {
            RadioServiceStats::increment(&mut self.state.stats.queue_drops);
        }
    }

    fn record_frame_error(&mut self, error: FrameError) {
        match error {
            FrameError::UnsupportedVersion(_) | FrameError::UnknownFrameType(_) => {
                RadioServiceStats::increment(&mut self.state.stats.unsupported_frames)
            }
            _ => RadioServiceStats::increment(&mut self.state.stats.malformed_frames),
        }
    }

    fn publish(&self) {
        let mut state = self.state;
        state.stats.neighbour_count = self.resources.neighbour_count.load(Ordering::Relaxed);
        self.resources.state.sender().send(state);
    }
}
