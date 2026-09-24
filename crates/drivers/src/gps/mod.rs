mod capture_counter;
mod framer;
pub mod nmea;
#[cfg(feature = "stm32")]
pub mod stm32;
mod types;

use core::future::{Future, poll_fn};
use core::pin::pin;
use core::task::Poll;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::pubsub::{ImmediatePublisher, PubSubChannel, Subscriber};
use embassy_sync::watch::Watch;
use embassy_time::{Duration, Instant, Timer, with_timeout};
use embedded_io_async::{Read, Write};
use framer::{FramerEvent, NmeaFramer};
use nmea::{NavigationEvent, NmeaParser};

pub use types::*;

#[cfg(feature = "stm32")]
pub(crate) use capture_counter::resolve_periodic_capture_delta;

pub type GpsMutex = CriticalSectionRawMutex;
const MANAGER_COMMAND_POLL: Duration = Duration::from_secs(1);
const NMEA_MAX_DELAY_AFTER_PPS: Duration = Duration::from_millis(750);

pub struct GpsResources<
    const SENTENCE_LEN: usize = DEFAULT_NMEA_SENTENCE_LEN,
    const WATCHERS: usize = DEFAULT_WATCHERS,
    const COMMAND_DEPTH: usize = DEFAULT_COMMAND_DEPTH,
    const RAW_DEPTH: usize = DEFAULT_RAW_NMEA_DEPTH,
> {
    commands: Channel<GpsMutex, GpsCommand, COMMAND_DEPTH>,
    manager_events: Channel<GpsMutex, ManagerEvent, COMMAND_DEPTH>,
    serial_requests: Channel<GpsMutex, SerialRequest, COMMAND_DEPTH>,
    fixes: Watch<GpsMutex, GpsFix, WATCHERS>,
    pps: Watch<GpsMutex, PpsInfo, WATCHERS>,
    pps_events: PubSubChannel<GpsMutex, PpsInfo, RAW_DEPTH, WATCHERS, 1>,
    stats: Watch<GpsMutex, GpsStats, WATCHERS>,
    time: Watch<GpsMutex, TimeCorrelation, WATCHERS>,
    time_events: PubSubChannel<GpsMutex, TimeCorrelation, RAW_DEPTH, WATCHERS, 1>,
    raw_nmea: PubSubChannel<GpsMutex, RawNmeaLog<SENTENCE_LEN>, RAW_DEPTH, WATCHERS, 1>,
}

impl<
    const SENTENCE_LEN: usize,
    const WATCHERS: usize,
    const COMMAND_DEPTH: usize,
    const RAW_DEPTH: usize,
> GpsResources<SENTENCE_LEN, WATCHERS, COMMAND_DEPTH, RAW_DEPTH>
{
    pub const fn new() -> Self {
        Self {
            commands: Channel::new(),
            manager_events: Channel::new(),
            serial_requests: Channel::new(),
            fixes: Watch::new(),
            pps: Watch::new(),
            pps_events: PubSubChannel::new(),
            stats: Watch::new(),
            time: Watch::new(),
            time_events: PubSubChannel::new(),
            raw_nmea: PubSubChannel::new(),
        }
    }

    pub fn command_sender(
        &self,
    ) -> embassy_sync::channel::Sender<'_, GpsMutex, GpsCommand, COMMAND_DEPTH> {
        self.commands.sender()
    }

    pub fn fix_receiver(
        &self,
    ) -> Option<embassy_sync::watch::Receiver<'_, GpsMutex, GpsFix, WATCHERS>> {
        self.fixes.receiver()
    }

    pub fn pps_receiver(
        &self,
    ) -> Option<embassy_sync::watch::Receiver<'_, GpsMutex, PpsInfo, WATCHERS>> {
        self.pps.receiver()
    }

    /// Subscribe to every PPS event. Unlike `pps_receiver`, this bounded
    /// stream retains individual edges when the consumer is briefly delayed.
    pub fn pps_event_subscriber(
        &self,
    ) -> Result<
        Subscriber<'_, GpsMutex, PpsInfo, RAW_DEPTH, WATCHERS, 1>,
        embassy_sync::pubsub::Error,
    > {
        self.pps_events.subscriber()
    }

    pub fn stats_receiver(
        &self,
    ) -> Option<embassy_sync::watch::Receiver<'_, GpsMutex, GpsStats, WATCHERS>> {
        self.stats.receiver()
    }

    /// Returns the latest GPS manager statistics without consuming a watcher.
    pub fn stats(&self) -> GpsStats {
        self.stats.try_get().unwrap_or_default()
    }

    /// Returns the most recently published navigation fix without consuming a
    /// watcher. Intended for low-rate health and correlation diagnostics.
    pub fn latest_fix(&self) -> Option<GpsFix> {
        self.fixes.try_get()
    }

    /// Returns the most recently captured PPS edge without consuming a
    /// watcher. The monotonically increasing count and capture deltas make it
    /// possible to distinguish a stopped PPS input from a pairing failure.
    pub fn latest_pps(&self) -> Option<PpsInfo> {
        self.pps.try_get()
    }

    /// Returns the latest NMEA time correlation emitted by the GPS driver.
    /// `pps_timestamp == None` means that NMEA time was published without a
    /// matching PPS edge inside the configured correlation window.
    pub fn latest_time_correlation(&self) -> Option<TimeCorrelation> {
        self.time.try_get()
    }

    pub fn time_receiver(
        &self,
    ) -> Option<embassy_sync::watch::Receiver<'_, GpsMutex, TimeCorrelation, WATCHERS>> {
        self.time.receiver()
    }

    /// Subscribe to every emitted NMEA/PPS correlation for persistent
    /// diagnostics and post-hoc clock reconstruction.
    pub fn time_event_subscriber(
        &self,
    ) -> Result<
        Subscriber<'_, GpsMutex, TimeCorrelation, RAW_DEPTH, WATCHERS, 1>,
        embassy_sync::pubsub::Error,
    > {
        self.time_events.subscriber()
    }

    pub fn raw_nmea_subscriber(
        &self,
    ) -> Result<
        embassy_sync::pubsub::Subscriber<
            '_,
            GpsMutex,
            RawNmeaLog<SENTENCE_LEN>,
            RAW_DEPTH,
            WATCHERS,
            1,
        >,
        embassy_sync::pubsub::Error,
    > {
        self.raw_nmea.subscriber()
    }
}

impl<
    const SENTENCE_LEN: usize,
    const WATCHERS: usize,
    const COMMAND_DEPTH: usize,
    const RAW_DEPTH: usize,
> Default for GpsResources<SENTENCE_LEN, WATCHERS, COMMAND_DEPTH, RAW_DEPTH>
{
    fn default() -> Self {
        Self::new()
    }
}

pub trait GpsPowerControl {
    type Error;

    fn set_enabled(&mut self, enabled: bool) -> Result<(), Self::Error>;
    fn set_reset_asserted(&mut self, asserted: bool) -> Result<(), Self::Error>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PpsCapture {
    pub timing_source: PpsTimingSource,
    pub timestamp: Instant,
    pub capture_ticks: Option<u64>,
    pub capture_frequency_hz: Option<u32>,
}

pub trait PpsSource {
    type Error;

    fn wait_for_pps(&mut self) -> impl Future<Output = Result<PpsCapture, Self::Error>> + '_;
}

pub struct GpsDriver<
    UART,
    PPS,
    POWER,
    const SENTENCE_LEN: usize = DEFAULT_NMEA_SENTENCE_LEN,
    const WATCHERS: usize = DEFAULT_WATCHERS,
    const COMMAND_DEPTH: usize = DEFAULT_COMMAND_DEPTH,
    const RAW_DEPTH: usize = DEFAULT_RAW_NMEA_DEPTH,
> {
    uart: UART,
    pps: PPS,
    power: POWER,
    resources: &'static GpsResources<SENTENCE_LEN, WATCHERS, COMMAND_DEPTH, RAW_DEPTH>,
    config: GpsConfig,
}

impl<
    UART,
    PPS,
    POWER,
    const SENTENCE_LEN: usize,
    const WATCHERS: usize,
    const COMMAND_DEPTH: usize,
    const RAW_DEPTH: usize,
> GpsDriver<UART, PPS, POWER, SENTENCE_LEN, WATCHERS, COMMAND_DEPTH, RAW_DEPTH>
{
    pub fn new(
        uart: UART,
        pps: PPS,
        power: POWER,
        resources: &'static GpsResources<SENTENCE_LEN, WATCHERS, COMMAND_DEPTH, RAW_DEPTH>,
        config: GpsConfig,
    ) -> Self {
        Self {
            uart,
            pps,
            power,
            resources,
            config,
        }
    }
}

impl<
    UART,
    PPS,
    POWER,
    const SENTENCE_LEN: usize,
    const WATCHERS: usize,
    const COMMAND_DEPTH: usize,
    const RAW_DEPTH: usize,
> GpsDriver<UART, PPS, POWER, SENTENCE_LEN, WATCHERS, COMMAND_DEPTH, RAW_DEPTH>
where
    UART: Read + Write,
    PPS: PpsSource,
    POWER: GpsPowerControl,
{
    pub async fn run(self) -> ! {
        let Self {
            uart,
            pps,
            power,
            resources,
            config,
        } = self;

        let manager = manager_task(power, resources, config);
        let serial = serial_rx_task(uart, resources, config.serial_poll_interval);
        let pps = pps_task(pps, resources);

        let mut manager = pin!(manager);
        let mut serial = pin!(serial);
        let mut pps = pin!(pps);

        poll_fn(|cx| {
            let _ = manager.as_mut().poll(cx);
            let _ = serial.as_mut().poll(cx);
            let _ = pps.as_mut().poll(cx);
            Poll::<()>::Pending
        })
        .await;

        unreachable!()
    }
}

async fn manager_task<
    POWER,
    const SENTENCE_LEN: usize,
    const WATCHERS: usize,
    const COMMAND_DEPTH: usize,
    const RAW_DEPTH: usize,
>(
    mut power: POWER,
    resources: &'static GpsResources<SENTENCE_LEN, WATCHERS, COMMAND_DEPTH, RAW_DEPTH>,
    config: GpsConfig,
) -> !
where
    POWER: GpsPowerControl,
{
    let commands = resources.commands.receiver();
    let manager_events = resources.manager_events.receiver();
    let serial = resources.serial_requests.sender();
    let stats_pub = resources.stats.sender();
    let mut first_search = true;
    let mut initial_calibration_pending = true;

    modify_stats(&stats_pub, |stats| {
        *stats = GpsStats::default();
        stats.operating_state = OperatingState::Off;
    });

    loop {
        match commands.receive().await {
            GpsCommand::Start => {
                run_search_cycle(
                    &mut power,
                    &commands,
                    &manager_events,
                    &serial,
                    &stats_pub,
                    &config,
                    &mut first_search,
                    &mut initial_calibration_pending,
                    config.initial_start_mode,
                )
                .await;
            }
            GpsCommand::ForceSearch => {
                run_search_cycle(
                    &mut power,
                    &commands,
                    &manager_events,
                    &serial,
                    &stats_pub,
                    &config,
                    &mut first_search,
                    &mut initial_calibration_pending,
                    config.initial_start_mode,
                )
                .await;
            }
            GpsCommand::ColdStart => {
                run_search_cycle(
                    &mut power,
                    &commands,
                    &manager_events,
                    &serial,
                    &stats_pub,
                    &config,
                    &mut first_search,
                    &mut initial_calibration_pending,
                    StartMode::Cold,
                )
                .await;
            }
            GpsCommand::WarmStart => {
                run_search_cycle(
                    &mut power,
                    &commands,
                    &manager_events,
                    &serial,
                    &stats_pub,
                    &config,
                    &mut first_search,
                    &mut initial_calibration_pending,
                    StartMode::Warm,
                )
                .await;
            }
            GpsCommand::HotStart => {
                run_search_cycle(
                    &mut power,
                    &commands,
                    &manager_events,
                    &serial,
                    &stats_pub,
                    &config,
                    &mut first_search,
                    &mut initial_calibration_pending,
                    StartMode::Hot,
                )
                .await;
            }
            GpsCommand::Stop => {
                enter_low_power(&mut power, &serial, &stats_pub, &config).await;
            }
            // This is normally consumed while initial calibration is active.
            // If received while idle there is no tracking window to release.
            GpsCommand::FrequencyCalibrationLocked | GpsCommand::PhaseQuality { .. } => {}
        }
    }
}

async fn run_search_cycle<POWER, const WATCHERS: usize, const COMMAND_DEPTH: usize>(
    power: &mut POWER,
    commands: &embassy_sync::channel::Receiver<'_, GpsMutex, GpsCommand, COMMAND_DEPTH>,
    manager_events: &embassy_sync::channel::Receiver<'_, GpsMutex, ManagerEvent, COMMAND_DEPTH>,
    serial: &embassy_sync::channel::Sender<'_, GpsMutex, SerialRequest, COMMAND_DEPTH>,
    stats_pub: &embassy_sync::watch::Sender<'_, GpsMutex, GpsStats, WATCHERS>,
    config: &GpsConfig,
    first_search: &mut bool,
    initial_calibration_pending: &mut bool,
    start_mode: StartMode,
) where
    POWER: GpsPowerControl,
{
    let mut cycle_start_mode = start_mode;

    loop {
        // Fix notifications produced while the previous on-window was already
        // tracking must not satisfy a later reacquisition attempt.
        drain_manager_events(manager_events);
        let search_time = if *first_search {
            config.first_search_time
        } else {
            config.search_time
        };

        *first_search = false;
        let on_started = Instant::now();
        modify_stats(stats_pub, |stats| {
            stats.last_fix_attempt_time = Some(on_started);
            stats.num_search_attempts = stats.num_search_attempts.saturating_add(1);
            if !*initial_calibration_pending {
                stats.num_reacquisition_attempts =
                    stats.num_reacquisition_attempts.saturating_add(1);
            }
            stats.operating_state = OperatingState::PoweringOn;
        });

        let _ = power.set_enabled(true);
        let _ = power.set_reset_asserted(false);
        modify_stats(stats_pub, |stats| stats.powered = true);
        Timer::after(config.power_settle_time).await;

        send_start_mode(serial, config, cycle_start_mode).await;
        modify_stats(stats_pub, |stats| {
            stats.operating_state = if *initial_calibration_pending {
                OperatingState::Searching
            } else {
                OperatingState::Reacquiring
            };
        });

        match wait_for_search_outcome(commands, manager_events, serial, config, search_time).await {
            SearchOutcome::Fix => {
                let now = Instant::now();
                modify_stats(stats_pub, |stats| {
                    stats.got_first_fix = true;
                    stats.last_successful_fix_time = Some(now);
                    stats.total_on_time += now.saturating_duration_since(on_started);
                    if *initial_calibration_pending {
                        stats.operating_state = OperatingState::Calibrating;
                    } else {
                        stats.num_reacquisition_successes =
                            stats.num_reacquisition_successes.saturating_add(1);
                        stats.operating_state = OperatingState::Acquired;
                    }
                });

                let stopped =
                    if *initial_calibration_pending && config.wait_for_frequency_calibration_lock {
                        wait_for_frequency_calibration_lock(commands, serial, config).await
                    } else if !*initial_calibration_pending {
                        if let Some(phase_config) = config.phase_qualified_shutdown {
                            wait_for_phase_qualified_shutdown(
                                commands,
                                serial,
                                stats_pub,
                                config,
                                phase_config,
                                on_started,
                            )
                            .await
                        } else {
                            sleep_or_stop(commands, serial, config, config.gps_on_time).await
                        }
                    } else {
                        let tracking_time = if *initial_calibration_pending {
                            config.initial_calibration_time
                        } else {
                            config.gps_on_time
                        };
                        sleep_or_stop(commands, serial, config, tracking_time).await
                    };
                if stopped {
                    enter_low_power(power, serial, stats_pub, config).await;
                    return;
                }
                if *initial_calibration_pending {
                    *initial_calibration_pending = false;
                    modify_stats(stats_pub, |stats| {
                        stats.initial_calibration_complete = true;
                        stats.operating_state = OperatingState::Acquired;
                    });
                }
                enter_low_power(power, serial, stats_pub, config).await;
                if sleep_or_stop(commands, serial, config, config.gps_off_time).await {
                    return;
                }
                cycle_start_mode = StartMode::Hot;
            }
            SearchOutcome::Stop => {
                enter_low_power(power, serial, stats_pub, config).await;
                return;
            }
            SearchOutcome::Timeout => {
                let now = Instant::now();
                modify_stats(stats_pub, |stats| {
                    stats.num_search_failures = stats.num_search_failures.saturating_add(1);
                    stats.num_search_timeouts = stats.num_search_timeouts.saturating_add(1);
                    stats.total_on_time += now.saturating_duration_since(on_started);
                    stats.operating_state = OperatingState::Error;
                });
                #[cfg(feature = "defmt")]
                defmt::warn!(
                    "GPS search timed out after {} seconds; entering standard {} second standby before retry",
                    search_time.as_secs(),
                    config.gps_off_time.as_secs()
                );

                // A missed acquisition is recoverable. Preserve the normal
                // fixed duty cycle: return to standby for the configured off
                // interval, then retry with the ordinary hot-start search
                // window. `search_failure_threshold` is intentionally not
                // applied here; it is reserved for a future backoff/search
                // escalation policy and must not make one timeout terminal.
                enter_low_power(power, serial, stats_pub, config).await;
                if sleep_or_stop(commands, serial, config, config.gps_off_time).await {
                    return;
                }
                cycle_start_mode = StartMode::Hot;
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SearchOutcome {
    Fix,
    Stop,
    Timeout,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PendingNmeaTime {
    utc_time: UtcDateTime,
    local_timestamp: Instant,
}

type CorrelationEventPublisher<'a, const DEPTH: usize, const WATCHERS: usize> =
    ImmediatePublisher<'a, GpsMutex, TimeCorrelation, DEPTH, WATCHERS, 1>;

async fn wait_for_search_outcome<const COMMAND_DEPTH: usize>(
    commands: &embassy_sync::channel::Receiver<'_, GpsMutex, GpsCommand, COMMAND_DEPTH>,
    manager_events: &embassy_sync::channel::Receiver<'_, GpsMutex, ManagerEvent, COMMAND_DEPTH>,
    serial: &embassy_sync::channel::Sender<'_, GpsMutex, SerialRequest, COMMAND_DEPTH>,
    config: &GpsConfig,
    timeout: Duration,
) -> SearchOutcome {
    let deadline = Instant::now() + timeout;

    loop {
        while let Ok(command) = commands.try_receive() {
            if handle_runtime_command(serial, config, command).await {
                return SearchOutcome::Stop;
            }
        }

        let now = Instant::now();
        if now >= deadline {
            return SearchOutcome::Timeout;
        }

        let wait = min_duration(
            deadline.saturating_duration_since(now),
            MANAGER_COMMAND_POLL,
        );
        if with_timeout(wait, manager_events.receive()).await.is_ok() {
            return SearchOutcome::Fix;
        }
    }
}

async fn sleep_or_stop<const COMMAND_DEPTH: usize>(
    commands: &embassy_sync::channel::Receiver<'_, GpsMutex, GpsCommand, COMMAND_DEPTH>,
    serial: &embassy_sync::channel::Sender<'_, GpsMutex, SerialRequest, COMMAND_DEPTH>,
    config: &GpsConfig,
    duration: Duration,
) -> bool {
    let deadline = Instant::now() + duration;
    #[cfg(feature = "defmt")]
    defmt::info!("GPS sleep_or_stop for {} seconds", duration.as_secs());
    loop {
        while let Ok(command) = commands.try_receive() {
            #[cfg(feature = "defmt")]
            defmt::info!("GPS sleep_or_stop command: {}", command);
            if handle_runtime_command(serial, config, command).await {
                #[cfg(feature = "defmt")]
                defmt::info!("GPS sleep_or_stop command done");
                return true;
            }
        }

        let now = Instant::now();
        if now >= deadline {
            return false;
        }
        #[cfg(feature = "defmt")]
        defmt::info!(
            "GPS sleep_or_stop waiting for {} seconds",
            deadline.saturating_duration_since(now).as_secs()
        );
        Timer::after(min_duration(
            deadline.saturating_duration_since(now),
            MANAGER_COMMAND_POLL,
        ))
        .await;
    }
}

/// Keep the GPS continuously active until the Time Service explicitly reports
/// that its frequency estimate is locked. Other runtime commands remain
/// responsive, and `Stop` still terminates the search cycle immediately.
async fn wait_for_frequency_calibration_lock<const COMMAND_DEPTH: usize>(
    commands: &embassy_sync::channel::Receiver<'_, GpsMutex, GpsCommand, COMMAND_DEPTH>,
    serial: &embassy_sync::channel::Sender<'_, GpsMutex, SerialRequest, COMMAND_DEPTH>,
    config: &GpsConfig,
) -> bool {
    #[cfg(feature = "defmt")]
    defmt::info!("GPS waiting for Time Service frequency calibration lock");
    loop {
        let command = commands.receive().await;
        #[cfg(feature = "defmt")]
        defmt::info!("GPS calibration wait command: {}", command);
        if command == GpsCommand::FrequencyCalibrationLocked {
            #[cfg(feature = "defmt")]
            defmt::info!("GPS frequency calibration lock confirmed");
            return false;
        }
        if handle_runtime_command(serial, config, command).await {
            return true;
        }
    }
}

/// Keep a reacquired receiver powered for at least `gps_on_time`, then enter
/// standby only after consecutive admitted PPS anchors show that UTC phase and
/// uncertainty have converged. Both deadlines are measured from power-on so
/// the maximum remains a genuine energy-use bound even after a slow fix.
async fn wait_for_phase_qualified_shutdown<
    const WATCHERS: usize,
    const COMMAND_DEPTH: usize,
>(
    commands: &embassy_sync::channel::Receiver<'_, GpsMutex, GpsCommand, COMMAND_DEPTH>,
    serial: &embassy_sync::channel::Sender<'_, GpsMutex, SerialRequest, COMMAND_DEPTH>,
    stats_pub: &embassy_sync::watch::Sender<'_, GpsMutex, GpsStats, WATCHERS>,
    config: &GpsConfig,
    phase_config: PhaseQualifiedShutdownConfig,
    on_started: Instant,
) -> bool {
    let minimum_deadline = on_started + config.gps_on_time;
    let maximum_on_time = if phase_config.maximum_on_time < config.gps_on_time {
        config.gps_on_time
    } else {
        phase_config.maximum_on_time
    };
    let maximum_deadline = on_started + maximum_on_time;
    let required_anchors = phase_config.consecutive_anchors.max(1);
    let mut streak = 0u8;
    let mut previous_observation_sequence = None;

    modify_stats(stats_pub, |stats| {
        stats.phase_qualification_active = true;
        stats.phase_qualification_streak = 0;
        stats.last_phase_residual_us = None;
        stats.last_phase_uncertainty_us = u64::MAX;
    });
    #[cfg(feature = "defmt")]
    defmt::info!(
        "GPS phase qualification started: minimum_s={} maximum_s={} residual_us={} uncertainty_us={} consecutive={}",
        config.gps_on_time.as_secs(),
        maximum_on_time.as_secs(),
        phase_config.residual_threshold_us,
        phase_config.uncertainty_threshold_us,
        required_anchors
    );

    loop {
        let now = Instant::now();
        if now >= minimum_deadline && streak >= required_anchors {
            modify_stats(stats_pub, |stats| {
                stats.phase_qualification_active = false;
                stats.num_phase_qualified_shutdowns =
                    stats.num_phase_qualified_shutdowns.saturating_add(1);
            });
            #[cfg(feature = "defmt")]
            defmt::info!(
                "GPS phase-qualified shutdown: on_s={} streak={}",
                now.saturating_duration_since(on_started).as_secs(),
                streak
            );
            return false;
        }
        if now >= maximum_deadline {
            modify_stats(stats_pub, |stats| {
                stats.phase_qualification_active = false;
                stats.num_phase_convergence_timeouts =
                    stats.num_phase_convergence_timeouts.saturating_add(1);
            });
            #[cfg(feature = "defmt")]
            defmt::warn!(
                "GPS phase convergence timed out after {} seconds: streak={} residual_us={:?}",
                maximum_on_time.as_secs(),
                streak,
                stats_pub.try_get().and_then(|stats| stats.last_phase_residual_us)
            );
            return false;
        }

        let wait = min_duration(
            maximum_deadline.saturating_duration_since(now),
            MANAGER_COMMAND_POLL,
        );
        let Ok(command) = with_timeout(wait, commands.receive()).await else {
            continue;
        };

        if let GpsCommand::PhaseQuality {
            observation_sequence,
            accepted,
            residual_us,
            uncertainty_us,
            pps_gate_active,
        } = command
        {
            if previous_observation_sequence == Some(observation_sequence) {
                continue;
            }
            let sequence_is_consecutive = previous_observation_sequence
                .map(|previous| previous.wrapping_add(1) == observation_sequence)
                .unwrap_or(true);
            previous_observation_sequence = Some(observation_sequence);
            let qualifies = phase_quality_qualifies(
                accepted,
                residual_us,
                uncertainty_us,
                pps_gate_active,
                &phase_config,
            );
            streak = if sequence_is_consecutive && qualifies {
                streak.saturating_add(1)
            } else if qualifies {
                1
            } else {
                0
            };
            modify_stats(stats_pub, |stats| {
                stats.phase_qualification_streak = streak;
                stats.last_phase_residual_us = residual_us;
                stats.last_phase_uncertainty_us = uncertainty_us;
            });
            continue;
        }

        if handle_runtime_command(serial, config, command).await {
            modify_stats(stats_pub, |stats| {
                stats.phase_qualification_active = false;
                stats.phase_qualification_streak = 0;
            });
            return true;
        }
    }
}

fn phase_quality_qualifies(
    accepted: bool,
    residual_us: Option<i64>,
    uncertainty_us: u64,
    pps_gate_active: bool,
    config: &PhaseQualifiedShutdownConfig,
) -> bool {
    accepted
        && !pps_gate_active
        && residual_us
            .map(|residual| residual.unsigned_abs() <= config.residual_threshold_us)
            .unwrap_or(false)
        && uncertainty_us <= config.uncertainty_threshold_us
}

async fn handle_runtime_command<const COMMAND_DEPTH: usize>(
    serial: &embassy_sync::channel::Sender<'_, GpsMutex, SerialRequest, COMMAND_DEPTH>,
    config: &GpsConfig,
    command: GpsCommand,
) -> bool {
    match command {
        GpsCommand::Stop => true,
        GpsCommand::ColdStart => {
            send_start_mode(serial, config, StartMode::Cold).await;
            false
        }
        GpsCommand::WarmStart => {
            send_start_mode(serial, config, StartMode::Warm).await;
            false
        }
        GpsCommand::HotStart | GpsCommand::ForceSearch | GpsCommand::Start => {
            send_start_mode(serial, config, StartMode::Hot).await;
            false
        }
        // Consumed by `wait_for_frequency_calibration_lock`. It is harmless
        // in search, ordinary tracking, or standby windows.
        GpsCommand::FrequencyCalibrationLocked | GpsCommand::PhaseQuality { .. } => false,
    }
}

fn min_duration(a: Duration, b: Duration) -> Duration {
    if a < b { a } else { b }
}

async fn enter_low_power<POWER, const WATCHERS: usize, const COMMAND_DEPTH: usize>(
    power: &mut POWER,
    serial: &embassy_sync::channel::Sender<'_, GpsMutex, SerialRequest, COMMAND_DEPTH>,
    stats_pub: &embassy_sync::watch::Sender<'_, GpsMutex, GpsStats, WATCHERS>,
    config: &GpsConfig,
) where
    POWER: GpsPowerControl,
{
    let off_started = Instant::now();
    modify_stats(stats_pub, |stats| {
        stats.operating_state = OperatingState::PoweringOff
    });

    if let Some(standby) = config.module_commands.standby {
        #[cfg(feature = "defmt")]
        defmt::info!("GPS enter_low_power: sending standby command");
        serial.send(SerialRequest::Write(standby)).await;
        modify_stats(stats_pub, |stats| {
            stats.powered = true;
            stats.total_off_time += Instant::now().saturating_duration_since(off_started);
            stats.operating_state = OperatingState::Standby;
        });
        return;
    }

    let _ = power.set_reset_asserted(true);
    let _ = power.set_enabled(false);
    modify_stats(stats_pub, |stats| {
        stats.powered = false;
        stats.total_off_time += Instant::now().saturating_duration_since(off_started);
        stats.operating_state = OperatingState::Off;
    });
}

async fn send_start_mode<const COMMAND_DEPTH: usize>(
    serial: &embassy_sync::channel::Sender<'_, GpsMutex, SerialRequest, COMMAND_DEPTH>,
    config: &GpsConfig,
    start_mode: StartMode,
) {
    if let Some(wake) = config.module_commands.wake {
        serial.send(SerialRequest::Write(wake)).await;
    }

    let command = match start_mode {
        StartMode::Hot => config.module_commands.hot_start,
        StartMode::Warm => config.module_commands.warm_start,
        StartMode::Cold => config.module_commands.cold_start,
    };

    if let Some(command) = command {
        serial.send(SerialRequest::Write(command)).await;
    }
}

fn drain_manager_events<const COMMAND_DEPTH: usize>(
    manager_events: &embassy_sync::channel::Receiver<'_, GpsMutex, ManagerEvent, COMMAND_DEPTH>,
) {
    while manager_events.try_receive().is_ok() {}
}

fn modify_stats<const WATCHERS: usize, F>(
    stats_pub: &embassy_sync::watch::Sender<'_, GpsMutex, GpsStats, WATCHERS>,
    f: F,
) where
    F: Fn(&mut GpsStats),
{
    stats_pub.send_modify(|slot| {
        let stats = slot.get_or_insert_with(GpsStats::default);
        f(stats);
    });
}

async fn serial_rx_task<
    UART,
    const SENTENCE_LEN: usize,
    const WATCHERS: usize,
    const COMMAND_DEPTH: usize,
    const RAW_DEPTH: usize,
>(
    mut uart: UART,
    resources: &'static GpsResources<SENTENCE_LEN, WATCHERS, COMMAND_DEPTH, RAW_DEPTH>,
    poll_interval: Duration,
) -> !
where
    UART: Read + Write,
{
    let serial_requests = resources.serial_requests.receiver();
    let manager_events = resources.manager_events.sender();
    let fix_pub = resources.fixes.sender();
    let stats_pub = resources.stats.sender();
    let time_pub = resources.time.sender();
    let time_event_pub = resources.time_events.immediate_publisher();
    let raw_pub = resources.raw_nmea.immediate_publisher();

    let mut parser = NmeaParser::new();
    let mut framer = NmeaFramer::<SENTENCE_LEN>::new();
    let mut last_pps: Option<PpsInfo> = None;
    let mut pending_time: Option<PendingNmeaTime> = None;
    let mut pps_rx = resources.pps.dyn_receiver();
    let mut buf = [0u8; 32];

    loop {
        if let Some(rx) = pps_rx.as_mut() {
            while let Some(pps) = rx.try_changed() {
                record_pps_update(
                    &time_pub,
                    &time_event_pub,
                    &mut pending_time,
                    &mut last_pps,
                    pps,
                );
            }
        }

        while let Ok(SerialRequest::Write(bytes)) = serial_requests.try_receive() {
            if uart.write_all(bytes).await.is_err() {
                modify_stats(&stats_pub, |stats| {
                    stats.num_uart_errors = stats.num_uart_errors.saturating_add(1)
                });
            }
        }

        match with_timeout(poll_interval, uart.read(&mut buf)).await {
            Ok(Ok(n)) => {
                let timestamp = Instant::now();
                for byte in &buf[..n] {
                    match framer.push(*byte) {
                        Some(FramerEvent::Sentence(sentence)) => {
                            raw_pub.publish_immediate(RawNmeaLog {
                                sentence: sentence.clone(),
                                timestamp,
                            });

                            match parser.parse(sentence.as_bytes()) {
                                Ok(Some(NavigationEvent::Fix(parsed_fix))) => {
                                    let fix = GpsFix {
                                        latitude: parsed_fix.latitude,
                                        longitude: parsed_fix.longitude,
                                        utc_time: parsed_fix.utc_time,
                                        satellites: parsed_fix.satellites,
                                        hdop_centi: parsed_fix.hdop_centi,
                                        system_timestamp: timestamp,
                                    };
                                    fix_pub.send(fix);
                                    modify_stats(&stats_pub, |stats| {
                                        stats.num_fixes = stats.num_fixes.saturating_add(1);
                                        stats.got_first_fix = true;
                                        stats.last_successful_fix_time = Some(timestamp);
                                    });
                                    let _ = manager_events.try_send(ManagerEvent::FixAcquired);

                                    record_nmea_time(
                                        &time_pub,
                                        &time_event_pub,
                                        &mut pending_time,
                                        &mut last_pps,
                                        fix.utc_time,
                                        fix.system_timestamp,
                                    );
                                }
                                Ok(Some(NavigationEvent::Time(utc_time))) => {
                                    record_nmea_time(
                                        &time_pub,
                                        &time_event_pub,
                                        &mut pending_time,
                                        &mut last_pps,
                                        utc_time,
                                        timestamp,
                                    );
                                }
                                Ok(Some(NavigationEvent::FixStatus { .. })) | Ok(None) => {}
                                Err(_) => {}
                            }
                        }
                        Some(FramerEvent::ChecksumError) => {
                            modify_stats(&stats_pub, |stats| {
                                stats.num_checksum_errors =
                                    stats.num_checksum_errors.saturating_add(1)
                            });
                        }
                        Some(FramerEvent::Overflow) => {
                            modify_stats(&stats_pub, |stats| {
                                stats.num_buffer_overflows =
                                    stats.num_buffer_overflows.saturating_add(1)
                            });
                        }
                        None => {}
                    }
                }
            }
            Ok(Err(_)) => {
                modify_stats(&stats_pub, |stats| {
                    stats.num_uart_errors = stats.num_uart_errors.saturating_add(1)
                });
            }
            Err(_) => {}
        }

        if let Some(rx) = pps_rx.as_mut() {
            while let Some(pps) = rx.try_changed() {
                record_pps_update(
                    &time_pub,
                    &time_event_pub,
                    &mut pending_time,
                    &mut last_pps,
                    pps,
                );
            }
        }

        publish_expired_pending_time(
            &time_pub,
            &time_event_pub,
            &mut pending_time,
            Instant::now(),
        );
    }
}

fn record_nmea_time<const WATCHERS: usize, const DEPTH: usize>(
    time_pub: &embassy_sync::watch::Sender<'_, GpsMutex, TimeCorrelation, WATCHERS>,
    time_event_pub: &CorrelationEventPublisher<'_, DEPTH, WATCHERS>,
    pending_time: &mut Option<PendingNmeaTime>,
    last_pps: &mut Option<PpsInfo>,
    utc_time: UtcDateTime,
    local_timestamp: Instant,
) {
    if let Some(pending) = *pending_time {
        if local_timestamp.saturating_duration_since(pending.local_timestamp)
            <= NMEA_MAX_DELAY_AFTER_PPS
        {
            pending_time.replace(PendingNmeaTime {
                utc_time: select_utc_with_date(pending.utc_time, utc_time),
                local_timestamp,
            });
            return;
        }

        publish_pending_time(time_pub, time_event_pub, pending_time, None);
    }

    if let Some(pps) = *last_pps {
        if pps_matches_nmea(pps, local_timestamp) {
            *last_pps = None;
            publish_time_correlation(
                time_pub,
                time_event_pub,
                utc_time,
                local_timestamp,
                Some(pps),
            );
            return;
        }

        if pps.timestamp <= local_timestamp {
            *last_pps = None;
        } else {
            publish_time_correlation(time_pub, time_event_pub, utc_time, local_timestamp, None);
            return;
        }
    }

    *pending_time = Some(PendingNmeaTime {
        utc_time,
        local_timestamp,
    });
}

fn select_utc_with_date(existing: UtcDateTime, new: UtcDateTime) -> UtcDateTime {
    if existing.date.is_none() && new.date.is_some() {
        new
    } else {
        existing
    }
}

fn record_pps_update<const WATCHERS: usize, const DEPTH: usize>(
    time_pub: &embassy_sync::watch::Sender<'_, GpsMutex, TimeCorrelation, WATCHERS>,
    time_event_pub: &CorrelationEventPublisher<'_, DEPTH, WATCHERS>,
    pending_time: &mut Option<PendingNmeaTime>,
    last_pps: &mut Option<PpsInfo>,
    pps: PpsInfo,
) {
    if let Some(pending) = *pending_time {
        if pps_matches_nmea(pps, pending.local_timestamp) {
            *pending_time = None;
            publish_time_correlation(
                time_pub,
                time_event_pub,
                pending.utc_time,
                pending.local_timestamp,
                Some(pps),
            );
            return;
        }

        if pps.timestamp > pending.local_timestamp {
            publish_pending_time(time_pub, time_event_pub, pending_time, None);
            *last_pps = Some(pps);
            return;
        }
    }

    *last_pps = Some(pps);
}

fn publish_expired_pending_time<const WATCHERS: usize, const DEPTH: usize>(
    time_pub: &embassy_sync::watch::Sender<'_, GpsMutex, TimeCorrelation, WATCHERS>,
    time_event_pub: &CorrelationEventPublisher<'_, DEPTH, WATCHERS>,
    pending_time: &mut Option<PendingNmeaTime>,
    now: Instant,
) {
    if let Some(pending) = *pending_time {
        if now.saturating_duration_since(pending.local_timestamp) > NMEA_MAX_DELAY_AFTER_PPS {
            publish_pending_time(time_pub, time_event_pub, pending_time, None);
        }
    }
}

fn publish_pending_time<const WATCHERS: usize, const DEPTH: usize>(
    time_pub: &embassy_sync::watch::Sender<'_, GpsMutex, TimeCorrelation, WATCHERS>,
    time_event_pub: &CorrelationEventPublisher<'_, DEPTH, WATCHERS>,
    pending_time: &mut Option<PendingNmeaTime>,
    pps: Option<PpsInfo>,
) {
    if let Some(pending) = pending_time.take() {
        publish_time_correlation(
            time_pub,
            time_event_pub,
            pending.utc_time,
            pending.local_timestamp,
            pps,
        );
    }
}

fn publish_time_correlation<const WATCHERS: usize, const DEPTH: usize>(
    time_pub: &embassy_sync::watch::Sender<'_, GpsMutex, TimeCorrelation, WATCHERS>,
    time_event_pub: &CorrelationEventPublisher<'_, DEPTH, WATCHERS>,
    utc_time: UtcDateTime,
    local_timestamp: Instant,
    pps: Option<PpsInfo>,
) {
    let correlation = TimeCorrelation {
        utc_time,
        local_timestamp,
        pps_timestamp: pps.map(|p| p.timestamp),
        pps_count: pps.map(|p| p.pps_count),
        pps_capture_ticks: pps.and_then(|p| p.capture_ticks),
        pps_capture_delta_ticks: pps.and_then(|p| p.capture_delta_ticks),
        pps_capture_frequency_hz: pps.and_then(|p| p.capture_frequency_hz),
        pps_delta_time: pps.and_then(|p| p.delta_time),
        pps_timing_source: pps.map(|p| p.timing_source),
    };
    time_pub.send(correlation);
    time_event_pub.publish_immediate(correlation);
}

fn pps_matches_nmea(pps: PpsInfo, nmea_timestamp: Instant) -> bool {
    pps.timestamp <= nmea_timestamp
        && nmea_timestamp.saturating_duration_since(pps.timestamp) <= NMEA_MAX_DELAY_AFTER_PPS
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pps_at(timestamp: Instant) -> PpsInfo {
        PpsInfo {
            pps_count: 1,
            timing_source: PpsTimingSource::EmbassyInstant,
            timestamp,
            capture_ticks: None,
            capture_delta_ticks: None,
            capture_frequency_hz: None,
            delta_time: None,
        }
    }

    #[test]
    fn pps_matches_nmea_inside_same_second_window() {
        let pps_timestamp = Instant::from_ticks(1_000);
        let pps = pps_at(pps_timestamp);

        assert!(pps_matches_nmea(
            pps,
            pps_timestamp + Duration::from_millis(750)
        ));
        assert!(!pps_matches_nmea(
            pps,
            pps_timestamp + Duration::from_millis(751)
        ));
    }

    #[test]
    fn pps_after_nmea_does_not_match() {
        let nmea_timestamp = Instant::from_ticks(1_000);
        let pps = pps_at(nmea_timestamp + Duration::from_millis(1));

        assert!(!pps_matches_nmea(pps, nmea_timestamp));
    }

    #[test]
    fn phase_quality_requires_residual_uncertainty_and_open_gate() {
        let config = PhaseQualifiedShutdownConfig::default();

        assert!(phase_quality_qualifies(true, Some(250), 500, false, &config));
        assert!(phase_quality_qualifies(true, Some(-250), 500, false, &config));
        assert!(!phase_quality_qualifies(true, Some(251), 500, false, &config));
        assert!(!phase_quality_qualifies(true, Some(250), 501, false, &config));
        assert!(!phase_quality_qualifies(true, None, 100, false, &config));
        assert!(!phase_quality_qualifies(true, Some(0), 100, true, &config));
        assert!(!phase_quality_qualifies(false, Some(0), 100, false, &config));
    }
}

async fn pps_task<
    PPS,
    const SENTENCE_LEN: usize,
    const WATCHERS: usize,
    const COMMAND_DEPTH: usize,
    const RAW_DEPTH: usize,
>(
    mut pps: PPS,
    resources: &'static GpsResources<SENTENCE_LEN, WATCHERS, COMMAND_DEPTH, RAW_DEPTH>,
) -> !
where
    PPS: PpsSource,
{
    let pps_pub = resources.pps.sender();
    let pps_event_pub = resources.pps_events.immediate_publisher();
    let stats_pub = resources.stats.sender();
    let mut count = 0u64;
    let mut previous: Option<Instant> = None;
    let mut previous_capture_ticks: Option<u64> = None;

    loop {
        match pps.wait_for_pps().await {
            Ok(capture) => {
                count = count.saturating_add(1);
                let info = PpsInfo {
                    pps_count: count,
                    timing_source: capture.timing_source,
                    timestamp: capture.timestamp,
                    capture_ticks: capture.capture_ticks,
                    capture_delta_ticks: match (previous_capture_ticks, capture.capture_ticks) {
                        (Some(previous), Some(current)) => Some(current.saturating_sub(previous)),
                        _ => None,
                    },
                    capture_frequency_hz: capture.capture_frequency_hz,
                    delta_time: previous
                        .map(|last| capture.timestamp.saturating_duration_since(last)),
                };
                previous = Some(capture.timestamp);
                previous_capture_ticks = capture.capture_ticks;
                pps_pub.send(info);
                pps_event_pub.publish_immediate(info);
                modify_stats(&stats_pub, |stats| {
                    stats.num_pps_events = count;
                    stats.last_pps_timing_source = Some(capture.timing_source);
                });
            }
            Err(_) => {
                modify_stats(&stats_pub, |stats| {
                    stats.num_pps_timeouts = stats.num_pps_timeouts.saturating_add(1)
                });
            }
        }
    }
}
