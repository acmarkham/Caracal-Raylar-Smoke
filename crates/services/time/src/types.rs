use embassy_time::{Duration, Instant, TICK_HZ};

pub const DEFAULT_WATCHERS: usize = 4;
pub const DEFAULT_ANCHOR_DEPTH: usize = 8;
const NANOS_PER_SECOND: i128 = 1_000_000_000;
const MICROS_PER_SECOND: i64 = 1_000_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct UtcTimestamp {
    pub seconds: i64,
    pub microseconds: u32,
}

impl UtcTimestamp {
    pub const fn new(seconds: i64, microseconds: u32) -> Option<Self> {
        if microseconds < MICROS_PER_SECOND as u32 {
            Some(Self {
                seconds,
                microseconds,
            })
        } else {
            None
        }
    }

    pub const fn from_micros(micros: i64) -> Self {
        let mut seconds = micros / MICROS_PER_SECOND;
        let mut remainder = micros % MICROS_PER_SECOND;
        if remainder < 0 {
            seconds -= 1;
            remainder += MICROS_PER_SECOND;
        }
        Self {
            seconds,
            microseconds: remainder as u32,
        }
    }

    pub const fn as_micros(self) -> i64 {
        self.seconds
            .saturating_mul(MICROS_PER_SECOND)
            .saturating_add(self.microseconds as i64)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimeSource {
    #[default]
    None,
    GpsNmea,
    GpsPps,
    Radio,
    Usb,
    Network,
    Laboratory,
    Other(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AnchorQuality {
    pub uncertainty_us: u64,
}

impl AnchorQuality {
    pub const fn new(uncertainty_us: u64) -> Self {
        Self { uncertainty_us }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Anchor {
    pub system_time: Instant,
    pub utc: UtcTimestamp,
    pub quality: AnchorQuality,
    pub source: TimeSource,
    pub capture_ticks: Option<u64>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeConfig {
    pub max_uncertainty_us: u64,
    pub holdover_stability_ppb: u64,
    pub frequency_ewma_weight_per_mille: u16,
    pub max_frequency_error_ppb: i64,
    pub max_anchor_residual_us: u64,
    pub minimum_frequency_baseline: Duration,
    pub publish_interval: Duration,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            max_uncertainty_us: 5_000_000,
            holdover_stability_ppb: 10_000,
            frequency_ewma_weight_per_mille: 125,
            max_frequency_error_ppb: 100_000,
            max_anchor_residual_us: 2_000_000,
            minimum_frequency_baseline: Duration::from_secs(10),
            publish_interval: Duration::from_secs(1),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TimeError {
    NotValid,
    OutOfRange,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TimeState {
    pub utc_valid: bool,
    pub reference_system_time: Instant,
    pub reference_utc: UtcTimestamp,
    pub estimated_frequency_error_ppb: i64,
    pub uncertainty_us: u64,
    pub last_anchor_system_time: Option<Instant>,
    pub last_anchor_utc: Option<UtcTimestamp>,
    pub holdover_duration: Duration,
    pub active_time_source: TimeSource,
    pub accepted_anchors: u32,
    pub rejected_anchors: u32,
}

impl TimeState {
    pub const fn invalid() -> Self {
        Self {
            utc_valid: false,
            reference_system_time: Instant::from_ticks(0),
            reference_utc: UtcTimestamp {
                seconds: 0,
                microseconds: 0,
            },
            estimated_frequency_error_ppb: 0,
            uncertainty_us: u64::MAX,
            last_anchor_system_time: None,
            last_anchor_utc: None,
            holdover_duration: Duration::from_ticks(0),
            active_time_source: TimeSource::None,
            accepted_anchors: 0,
            rejected_anchors: 0,
        }
    }

    pub fn system_to_utc(&self, system_time: Instant) -> Result<UtcTimestamp, TimeError> {
        if !self.utc_valid {
            return Err(TimeError::NotValid);
        }
        let delta_ticks =
            system_time.as_ticks() as i128 - self.reference_system_time.as_ticks() as i128;
        let scale = NANOS_PER_SECOND + self.estimated_frequency_error_ppb as i128;
        let delta_us = delta_ticks
            .checked_mul(1_000_000)
            .and_then(|v| v.checked_mul(scale))
            .map(|v| v / (TICK_HZ as i128 * NANOS_PER_SECOND))
            .ok_or(TimeError::OutOfRange)?;
        let utc_us = (self.reference_utc.as_micros() as i128)
            .checked_add(delta_us)
            .ok_or(TimeError::OutOfRange)?;
        Ok(UtcTimestamp::from_micros(
            i64::try_from(utc_us).map_err(|_| TimeError::OutOfRange)?,
        ))
    }

    pub fn utc_to_system(&self, utc: UtcTimestamp) -> Result<Instant, TimeError> {
        if !self.utc_valid {
            return Err(TimeError::NotValid);
        }
        let scale = NANOS_PER_SECOND + self.estimated_frequency_error_ppb as i128;
        if scale <= 0 {
            return Err(TimeError::OutOfRange);
        }
        let delta_us = utc.as_micros() as i128 - self.reference_utc.as_micros() as i128;
        let delta_ticks = delta_us
            .checked_mul(TICK_HZ as i128)
            .and_then(|v| v.checked_mul(NANOS_PER_SECOND))
            .map(|v| v / (1_000_000 * scale))
            .ok_or(TimeError::OutOfRange)?;
        let ticks = (self.reference_system_time.as_ticks() as i128)
            .checked_add(delta_ticks)
            .ok_or(TimeError::OutOfRange)?;
        Ok(Instant::from_ticks(
            u64::try_from(ticks).map_err(|_| TimeError::OutOfRange)?,
        ))
    }
}
