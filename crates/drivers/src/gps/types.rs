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
    /// Waiting for the first navigation fix after startup.
    Searching,
    /// Keeping GPS continuously powered after the first fix so the time
    /// service can estimate the local oscillator frequency from PPS.
    Calibrating,
    /// Searching after a completed initial calibration and holdover period.
    Reacquiring,
    Acquired,
    Standby,
    PoweringOff,
    Error,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PpsTimingSource {
    /// Edge timing uses the Embassy monotonic clock sampled after EXTI wake-up.
    #[default]
    EmbassyInstant,
    /// Edge timing uses the STM32U59xxx 32-bit TIM4_CH4 capture register at
    /// 1 MHz (DS13633 Rev 3, section 3.44, table 19).
    Tim4Capture,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GpsCommand {
    Start,
    Stop,
    /// Release continuous initial tracking after the Time Service has locked
    /// its oscillator-frequency calibration.
    FrequencyCalibrationLocked,
    /// Latest Time Service quality after a GPS PPS anchor was evaluated.
    /// `observation_sequence` includes accepted and rejected anchors so a
    /// rejected/gated edge breaks, rather than merely pauses, a good streak.
    PhaseQuality {
        observation_sequence: u64,
        accepted: bool,
        residual_us: Option<i64>,
        uncertainty_us: u64,
        pps_gate_active: bool,
    },
    ForceSearch,
    ColdStart,
    WarmStart,
    HotStart,
}

/// Conditions for ending a post-calibration GPS tracking window.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhaseQualifiedShutdownConfig {
    /// Safety bound on the complete powered-on interval: return to standby
    /// even if convergence is not achieved.
    pub maximum_on_time: Duration,
    /// Maximum absolute Time Service phase residual accepted as "small".
    pub residual_threshold_us: u64,
    /// Maximum UTC uncertainty accepted when evaluating the residual.
    pub uncertainty_threshold_us: u64,
    /// Consecutive newly accepted PPS anchors that must meet both thresholds.
    pub consecutive_anchors: u8,
}

impl Default for PhaseQualifiedShutdownConfig {
    fn default() -> Self {
        Self {
            maximum_on_time: Duration::from_secs(180),
            residual_threshold_us: 250,
            uncertainty_threshold_us: 500,
            consecutive_anchors: 5,
        }
    }
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
            standby: Some(&b"$PMTK161,0*28\r\n"[..]),
            wake: Some(&b"\r\n"[..]),
            hot_start: None,
            warm_start: None,
            cold_start: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GpsConfig {
    /// Minimum powered-on interval. When `phase_qualified_shutdown` is
    /// configured, GPS remains active beyond this duration until timing
    /// quality qualifies or the configured maximum is reached.
    pub gps_on_time: Duration,
    pub gps_off_time: Duration,
    pub first_search_time: Duration,
    /// Continuous on-time after the first fix, before duty cycling starts.
    /// Used when `wait_for_frequency_calibration_lock` is false.
    pub initial_calibration_time: Duration,
    /// Keep the receiver continuously active after its first fix until a
    /// `FrequencyCalibrationLocked` command is received from the Time Service.
    pub wait_for_frequency_calibration_lock: bool,
    /// Optional phase-qualified post-calibration shutdown policy.
    pub phase_qualified_shutdown: Option<PhaseQualifiedShutdownConfig>,
    pub search_time: Duration,
    /// Reserved for a future search-window backoff/escalation policy. Search
    /// timeouts currently retain the standard fixed on/off duty cycle and do
    /// not stop autonomous retries at this threshold.
    pub search_failure_threshold: u32,
    pub initial_start_mode: StartMode,
    pub power_settle_time: Duration,
    pub serial_poll_interval: Duration,
    /// Selects the STM32 PPS timing backend used by `Stm32Pps::from_config`.
    pub pps_timing_source: PpsTimingSource,
    pub module_commands: GpsModuleCommands,
}

impl Default for GpsConfig {
    fn default() -> Self {
        Self {
            gps_on_time: Duration::from_secs(30),
            gps_off_time: Duration::from_secs(30),
            first_search_time: Duration::from_secs(15 * 60),
            initial_calibration_time: Duration::from_secs(10 * 60),
            wait_for_frequency_calibration_lock: false,
            phase_qualified_shutdown: None,
            search_time: Duration::from_secs(90),
            search_failure_threshold: 10,
            initial_start_mode: StartMode::Hot,
            power_settle_time: Duration::from_millis(250),
            serial_poll_interval: Duration::from_millis(100),
            pps_timing_source: PpsTimingSource::EmbassyInstant,
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
    pub timing_source: PpsTimingSource,
    /// Coarse monotonic timestamp retained for UTC correlation.
    pub timestamp: Instant,
    /// Wrap-extended hardware capture count, when timer capture is enabled.
    pub capture_ticks: Option<u64>,
    /// Hardware capture ticks elapsed since the preceding PPS edge.
    pub capture_delta_ticks: Option<u64>,
    /// Frequency required to interpret capture tick fields.
    pub capture_frequency_hz: Option<u32>,
    /// Coarse Embassy-clock interval retained for comparison and fallback.
    pub delta_time: Option<Duration>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TimeCorrelation {
    pub utc_time: UtcDateTime,
    pub local_timestamp: Instant,
    pub pps_timestamp: Option<Instant>,
    /// PPS sequence number provides an unambiguous join to the raw edge log.
    pub pps_count: Option<u64>,
    /// Fine PPS fields are forwarded unchanged for a future Time Service
    /// frequency estimator; the GPS driver does not interpret their drift.
    pub pps_capture_ticks: Option<u64>,
    pub pps_capture_delta_ticks: Option<u64>,
    pub pps_capture_frequency_hz: Option<u32>,
    /// Coarse monotonic interval from the preceding raw PPS edge. Unlike
    /// correlations, the raw edge stream is continuous while GPS is awake.
    pub pps_delta_time: Option<Duration>,
    pub pps_timing_source: Option<PpsTimingSource>,
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
    pub last_pps_timing_source: Option<PpsTimingSource>,
    pub num_pps_timeouts: u32,
    pub num_search_timeouts: u32,
    pub initial_calibration_complete: bool,
    pub num_reacquisition_attempts: u32,
    pub num_reacquisition_successes: u32,
    pub phase_qualification_active: bool,
    pub phase_qualification_streak: u8,
    pub last_phase_residual_us: Option<i64>,
    pub last_phase_uncertainty_us: u64,
    pub num_phase_qualified_shutdowns: u32,
    pub num_phase_convergence_timeouts: u32,
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
            last_pps_timing_source: None,
            num_pps_timeouts: 0,
            num_search_timeouts: 0,
            initial_calibration_complete: false,
            num_reacquisition_attempts: 0,
            num_reacquisition_successes: 0,
            phase_qualification_active: false,
            phase_qualification_streak: 0,
            last_phase_residual_us: None,
            last_phase_uncertainty_us: u64::MAX,
            num_phase_qualified_shutdowns: 0,
            num_phase_convergence_timeouts: 0,
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
