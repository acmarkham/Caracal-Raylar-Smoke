use embassy_time::Instant;
use raylar_drivers::gps::{Coordinate, UtcDateTime};

pub const DEFAULT_MIN_SATELLITES: u8 = 4;
pub const DEFAULT_MAX_HDOP_CENTI: u16 = 500;
pub const DEFAULT_MIN_ACCEPTED_FIXES: usize = 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum LocationSource {
    #[default]
    None,
    Gps,
    Manual,
    FutureExternal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocationConfig {
    pub minimum_satellites: u8,
    pub maximum_hdop_centi: Option<u16>,
    pub minimum_accepted_fixes: usize,
}

impl Default for LocationConfig {
    fn default() -> Self {
        Self {
            minimum_satellites: DEFAULT_MIN_SATELLITES,
            maximum_hdop_centi: Some(DEFAULT_MAX_HDOP_CENTI),
            minimum_accepted_fixes: DEFAULT_MIN_ACCEPTED_FIXES,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LocationState {
    pub valid: bool,
    pub latitude: Coordinate,
    pub longitude: Coordinate,
    pub source: LocationSource,
    pub fix_count_used: u8,
    pub total_fix_count_seen: u64,
    pub last_fix_system_time: Instant,
    pub last_fix_utc_time: Option<UtcDateTime>,
    pub hdop_centi: Option<u16>,
    pub satellites: Option<u8>,
    pub uncertainty_meters: Option<u32>,
}

impl LocationState {
    pub const fn invalid() -> Self {
        Self {
            valid: false,
            latitude: Coordinate { degrees_e7: 0 },
            longitude: Coordinate { degrees_e7: 0 },
            source: LocationSource::None,
            fix_count_used: 0,
            total_fix_count_seen: 0,
            last_fix_system_time: Instant::from_ticks(0),
            last_fix_utc_time: None,
            hdop_centi: None,
            satellites: None,
            uncertainty_meters: None,
        }
    }
}
