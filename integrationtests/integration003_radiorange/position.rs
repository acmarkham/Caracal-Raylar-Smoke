use embassy_time::Instant;
use raylar_location_service::LocationState;

use crate::radio_test_config::{
    EAST_MM_PER_E7, MAX_LOCAL_OFFSET_METRES, MAX_LOCATION_AGE, NORTH_MM_PER_E7, ORIGIN_LATITUDE_E7,
    ORIGIN_LONGITUDE_E7, POSITION_QUANTIZATION_METRES,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalPosition {
    pub east_10m: i16,
    pub north_10m: i16,
    pub uncertainty_m: Option<u32>,
    pub age_ms: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionError {
    Invalid,
    Stale,
    OutsideConfiguredArea,
}

pub fn from_location(state: LocationState, now: Instant) -> Result<LocalPosition, PositionError> {
    if !state.valid {
        return Err(PositionError::Invalid);
    }
    let age = now.saturating_duration_since(state.last_fix_system_time);
    if age > MAX_LOCATION_AGE {
        return Err(PositionError::Stale);
    }
    let north_delta = i64::from(state.latitude.degrees_e7) - i64::from(ORIGIN_LATITUDE_E7);
    let east_delta = i64::from(state.longitude.degrees_e7) - i64::from(ORIGIN_LONGITUDE_E7);
    let north_mm = north_delta.saturating_mul(NORTH_MM_PER_E7);
    let east_mm = east_delta.saturating_mul(EAST_MM_PER_E7);
    let divisor = POSITION_QUANTIZATION_METRES * 1_000;
    let north = rounded_div(north_mm, divisor);
    let east = rounded_div(east_mm, divisor);
    let maximum_units = MAX_LOCAL_OFFSET_METRES / POSITION_QUANTIZATION_METRES;
    if east.abs() > maximum_units || north.abs() > maximum_units {
        return Err(PositionError::OutsideConfiguredArea);
    }
    Ok(LocalPosition {
        east_10m: east as i16,
        north_10m: north as i16,
        uncertainty_m: state.uncertainty_meters,
        age_ms: age.as_millis(),
    })
}

pub fn distance_metres(a: LocalPosition, east_10m: i16, north_10m: i16) -> u32 {
    let east_m = (i64::from(a.east_10m) - i64::from(east_10m)) * 10;
    let north_m = (i64::from(a.north_10m) - i64::from(north_10m)) * 10;
    integer_sqrt((east_m * east_m + north_m * north_m) as u64) as u32
}

fn rounded_div(value: i64, divisor: i64) -> i64 {
    if value >= 0 {
        (value + divisor / 2) / divisor
    } else {
        (value - divisor / 2) / divisor
    }
}

fn integer_sqrt(value: u64) -> u64 {
    if value < 2 {
        return value;
    }
    let mut low = 1u64;
    let mut high = value.min(u64::from(u32::MAX));
    while low <= high {
        let middle = low + (high - low) / 2;
        if middle <= value / middle {
            low = middle + 1;
        } else {
            high = middle - 1;
        }
    }
    high
}
