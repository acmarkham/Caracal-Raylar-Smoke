use embassy_time::{Duration, Instant};
use heapless::Vec;

pub const DEFAULT_NMEA_SENTENCE_LEN: usize = 128;
pub const DEFAULT_WATCHERS: usize = 4;
pub const DEFAULT_COMMAND_DEPTH: usize = 8;
pub const DEFAULT_RAW_NMEA_DEPTH: usize = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum StartMode {
    Hot,
    Warm,
    Cold,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OperatingState {
    Off,
    PoweringOn,
    Searching,
    Acquired,
    Standby,
    PoweringOff,
    Error,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpsCommand {
    Start,
    Stop,
    ForceSearch,
    ColdStart,
    WarmStart,
    HotStart,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpsModuleCommands {
    pub standby: Option<&'static [u8]>,
    pub wake: Option<&'static [u8]>,
    pub hot_start: Option<&'static [u8]>,
    pub warm_start: Option<&'static [u8]>,
    pub cold_start: Option<&'static [u8]>,
}

impl Default for GpsModuleCommands {
    fn default() -> Self {
        Self {
            standby: None,
            wake: None,
            hot_start: None,
            warm_start: None,
            cold_start: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpsConfig {
    pub gps_on_time: Duration,
    pub gps_off_time: Duration,
    pub first_search_time: Duration,
    pub search_time: Duration,
    pub search_failure_threshold: u32,
    pub initial_start_mode: StartMode,
    pub power_settle_time: Duration,
    pub serial_poll_interval: Duration,
    pub module_commands: GpsModuleCommands,
}

impl Default for GpsConfig {
    fn default() -> Self {
        Self {
            gps_on_time: Duration::from_secs(30),
            gps_off_time: Duration::from_secs(5 * 60),
            first_search_time: Duration::from_secs(15 * 60),
            search_time: Duration::from_secs(30),
            search_failure_threshold: 10,
            initial_start_mode: StartMode::Hot,
            power_settle_time: Duration::from_millis(250),
            serial_poll_interval: Duration::from_millis(100),
            module_commands: GpsModuleCommands::default(),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UtcTime {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UtcDate {
    pub day: u8,
    pub month: u8,
    pub year: u16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UtcDateTime {
    pub date: Option<UtcDate>,
    pub time: UtcTime,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Coordinate {
    pub degrees_e7: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GpsFix {
    pub latitude: Coordinate,
    pub longitude: Coordinate,
    pub utc_time: UtcDateTime,
    pub satellites: u8,
    pub hdop_centi: Option<u16>,
    pub system_timestamp: Instant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PpsInfo {
    pub pps_count: u64,
    pub timestamp: Instant,
    pub capture_ticks: Option<u64>,
    pub delta_time: Option<Duration>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TimeCorrelation {
    pub utc_time: UtcDateTime,
    pub local_timestamp: Instant,
    pub pps_timestamp: Option<Instant>,
    pub pps_capture_ticks: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GpsStats {
    pub powered: bool,
    pub got_first_fix: bool,
    pub operating_state: OperatingState,
    pub last_fix_attempt_time: Option<Instant>,
    pub last_successful_fix_time: Option<Instant>,
    pub num_fixes: u64,
    pub total_on_time: Duration,
    pub total_off_time: Duration,
    pub num_search_attempts: u32,
    pub num_search_failures: u32,
    pub num_checksum_errors: u32,
    pub num_uart_errors: u32,
    pub num_buffer_overflows: u32,
    pub num_pps_events: u64,
    pub num_pps_timeouts: u32,
    pub num_search_timeouts: u32,
}

impl Default for GpsStats {
    fn default() -> Self {
        Self {
            powered: false,
            got_first_fix: false,
            operating_state: OperatingState::Off,
            last_fix_attempt_time: None,
            last_successful_fix_time: None,
            num_fixes: 0,
            total_on_time: Duration::from_ticks(0),
            total_off_time: Duration::from_ticks(0),
            num_search_attempts: 0,
            num_search_failures: 0,
            num_checksum_errors: 0,
            num_uart_errors: 0,
            num_buffer_overflows: 0,
            num_pps_events: 0,
            num_pps_timeouts: 0,
            num_search_timeouts: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawNmeaSentence<const N: usize = DEFAULT_NMEA_SENTENCE_LEN> {
    bytes: Vec<u8, N>,
}

impl<const N: usize> RawNmeaSentence<N> {
    pub fn new(bytes: &[u8]) -> Result<Self, ()> {
        let mut out = Vec::new();
        out.extend_from_slice(bytes).map_err(|_| ())?;
        Ok(Self { bytes: out })
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn as_str(&self) -> Result<&str, core::str::Utf8Error> {
        core::str::from_utf8(&self.bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RawNmeaLog<const N: usize = DEFAULT_NMEA_SENTENCE_LEN> {
    pub sentence: RawNmeaSentence<N>,
    pub timestamp: Instant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ManagerEvent {
    FixAcquired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SerialRequest {
    Write(&'static [u8]),
}
