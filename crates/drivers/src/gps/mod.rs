mod framer;
pub mod nmea;
#[cfg(feature = "stm32")]
pub mod stm32;
mod types;

use core::future::{poll_fn, Future};
use core::pin::pin;
use core::task::Poll;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::pubsub::PubSubChannel;
use embassy_sync::watch::Watch;
use embassy_time::{with_timeout, Duration, Instant, Timer};
use embedded_io_async::{Read, Write};
use framer::{FramerEvent, NmeaFramer};
use nmea::{NavigationEvent, NmeaParser};

pub use types::*;

pub type GpsMutex = CriticalSectionRawMutex;
const MANAGER_COMMAND_POLL: Duration = Duration::from_secs(1);

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
    stats: Watch<GpsMutex, GpsStats, WATCHERS>,
    time: Watch<GpsMutex, TimeCorrelation, WATCHERS>,
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
            stats: Watch::new(),
            time: Watch::new(),
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

    pub fn stats_receiver(
        &self,
    ) -> Option<embassy_sync::watch::Receiver<'_, GpsMutex, GpsStats, WATCHERS>> {
        self.stats.receiver()
    }

    pub fn time_receiver(
        &self,
    ) -> Option<embassy_sync::watch::Receiver<'_, GpsMutex, TimeCorrelation, WATCHERS>> {
        self.time.receiver()
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
                    StartMode::Hot,
                )
                .await;
            }
            GpsCommand::Stop => {
                power_off(&mut power, &serial, &stats_pub, &config).await;
            }
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
    start_mode: StartMode,
) where
    POWER: GpsPowerControl,
{
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
        stats.operating_state = OperatingState::PoweringOn;
    });

    let _ = power.set_enabled(true);
    let _ = power.set_reset_asserted(false);
    modify_stats(stats_pub, |stats| stats.powered = true);
    Timer::after(config.power_settle_time).await;

    send_start_mode(serial, config, start_mode).await;
    modify_stats(stats_pub, |stats| {
        stats.operating_state = OperatingState::Searching
    });

    match wait_for_search_outcome(commands, manager_events, serial, config, search_time).await {
        SearchOutcome::Fix => {
            let now = Instant::now();
            modify_stats(stats_pub, |stats| {
                stats.got_first_fix = true;
                stats.last_successful_fix_time = Some(now);
                stats.total_on_time += now.saturating_duration_since(on_started);
                stats.operating_state = OperatingState::Acquired;
            });
            if sleep_or_stop(commands, serial, config, config.gps_on_time).await {
                power_off(power, serial, stats_pub, config).await;
                return;
            }
            modify_stats(stats_pub, |stats| {
                stats.operating_state = OperatingState::Standby
            });
            let _ = sleep_or_stop(commands, serial, config, config.gps_off_time).await;
        }
        SearchOutcome::Stop => {
            power_off(power, serial, stats_pub, config).await;
            return;
        }
        SearchOutcome::Timeout => {
            modify_stats(stats_pub, |stats| {
                stats.num_search_failures = stats.num_search_failures.saturating_add(1);
                stats.num_search_timeouts = stats.num_search_timeouts.saturating_add(1);
                stats.operating_state = OperatingState::Error;
            });
        }
    }

    drain_stop_command(commands);
    power_off(power, serial, stats_pub, config).await;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SearchOutcome {
    Fix,
    Stop,
    Timeout,
}

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

    loop {
        while let Ok(command) = commands.try_receive() {
            if handle_runtime_command(serial, config, command).await {
                return true;
            }
        }

        let now = Instant::now();
        if now >= deadline {
            return false;
        }

        Timer::after(min_duration(
            deadline.saturating_duration_since(now),
            MANAGER_COMMAND_POLL,
        ))
        .await;
    }
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
    }
}

fn min_duration(a: Duration, b: Duration) -> Duration {
    if a < b {
        a
    } else {
        b
    }
}

async fn power_off<POWER, const WATCHERS: usize, const COMMAND_DEPTH: usize>(
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
        serial.send(SerialRequest::Write(standby)).await;
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

fn drain_stop_command<const COMMAND_DEPTH: usize>(
    commands: &embassy_sync::channel::Receiver<'_, GpsMutex, GpsCommand, COMMAND_DEPTH>,
) {
    while let Ok(command) = commands.try_receive() {
        if command == GpsCommand::Stop {
            break;
        }
    }
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
    let raw_pub = resources.raw_nmea.immediate_publisher();

    let mut parser = NmeaParser::new();
    let mut framer = NmeaFramer::<SENTENCE_LEN>::new();
    let mut last_pps: Option<PpsInfo> = None;
    let mut pps_rx = resources.pps.dyn_receiver();
    let mut buf = [0u8; 32];

    loop {
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

                                    let pps = last_pps.take();
                                    time_pub.send(TimeCorrelation {
                                        utc_time: fix.utc_time,
                                        local_timestamp: fix.system_timestamp,
                                        pps_timestamp: pps.map(|p| p.timestamp),
                                        pps_capture_ticks: pps.and_then(|p| p.capture_ticks),
                                        pps_capture_delta_ticks: pps
                                            .and_then(|p| p.capture_delta_ticks),
                                        pps_capture_frequency_hz: pps
                                            .and_then(|p| p.capture_frequency_hz),
                                        pps_timing_source: pps.map(|p| p.timing_source),
                                    });
                                }
                                Ok(Some(NavigationEvent::Time(utc_time))) => {
                                    let pps = last_pps.take();
                                    time_pub.send(TimeCorrelation {
                                        utc_time,
                                        local_timestamp: timestamp,
                                        pps_timestamp: pps.map(|p| p.timestamp),
                                        pps_capture_ticks: pps.and_then(|p| p.capture_ticks),
                                        pps_capture_delta_ticks: pps
                                            .and_then(|p| p.capture_delta_ticks),
                                        pps_capture_frequency_hz: pps
                                            .and_then(|p| p.capture_frequency_hz),
                                        pps_timing_source: pps.map(|p| p.timing_source),
                                    });
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
            if let Some(pps) = rx.try_changed() {
                last_pps = Some(pps);
            }
        }
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
                modify_stats(&stats_pub, |stats| stats.num_pps_events = count);
            }
            Err(_) => {
                modify_stats(&stats_pub, |stats| {
                    stats.num_pps_timeouts = stats.num_pps_timeouts.saturating_add(1)
                });
            }
        }
    }
}
