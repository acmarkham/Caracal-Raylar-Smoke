use heapless::Vec;
use raylar_drivers::gps::{Coordinate, GpsFix};

use crate::{LocationConfig, LocationSource, LocationState};

const LATITUDE_LIMIT_E7: i32 = 900_000_000;
const LONGITUDE_LIMIT_E7: i32 = 1_800_000_000;
const LATITUDE_METERS_PER_DEGREE: u32 = 111_320;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct LocationSample {
    latitude: Coordinate,
    longitude: Coordinate,
    satellites: u8,
    hdop_centi: Option<u16>,
}

impl LocationSample {
    fn from_fix(fix: GpsFix) -> Self {
        Self {
            latitude: fix.latitude,
            longitude: fix.longitude,
            satellites: fix.satellites,
            hdop_centi: fix.hdop_centi,
        }
    }
}

pub struct LocationFilter<const HISTORY: usize> {
    history: Vec<LocationSample, HISTORY>,
    config: LocationConfig,
    state: LocationState,
}

impl<const HISTORY: usize> LocationFilter<HISTORY> {
    pub const fn new(config: LocationConfig) -> Self {
        Self {
            history: Vec::new(),
            config,
            state: LocationState::invalid(),
        }
    }

    pub const fn state(&self) -> LocationState {
        self.state
    }

    pub fn ingest(&mut self, fix: GpsFix) -> LocationState {
        let seen = self.state.total_fix_count_seen.saturating_add(1);
        if !self.accepts(fix) || HISTORY == 0 {
            self.state.total_fix_count_seen = seen;
            return self.state;
        }

        if self.history.is_full() {
            let _ = self.history.remove(0);
        }
        let _ = self.history.push(LocationSample::from_fix(fix));

        let latitude = median_coordinate(&self.history, |sample| sample.latitude.degrees_e7);
        let longitude = median_coordinate(&self.history, |sample| sample.longitude.degrees_e7);
        let accepted = self.history.len();
        let valid = accepted >= self.config.minimum_accepted_fixes.min(HISTORY);

        self.state = LocationState {
            valid,
            latitude: Coordinate {
                degrees_e7: latitude,
            },
            longitude: Coordinate {
                degrees_e7: longitude,
            },
            source: if valid {
                LocationSource::Gps
            } else {
                LocationSource::None
            },
            fix_count_used: accepted.min(u8::MAX as usize) as u8,
            total_fix_count_seen: seen,
            last_fix_system_time: fix.system_timestamp,
            last_fix_utc_time: Some(fix.utc_time),
            hdop_centi: median_optional_u16(&self.history, |sample| sample.hdop_centi),
            satellites: Some(median_u8(&self.history, |sample| sample.satellites)),
            uncertainty_meters: estimate_uncertainty_meters(&self.history),
        };
        self.state
    }

    fn accepts(&self, fix: GpsFix) -> bool {
        valid_coordinate(fix.latitude, LATITUDE_LIMIT_E7)
            && valid_coordinate(fix.longitude, LONGITUDE_LIMIT_E7)
            && fix.satellites >= self.config.minimum_satellites
            && self
                .config
                .maximum_hdop_centi
                .map(|max| fix.hdop_centi.map(|hdop| hdop <= max).unwrap_or(true))
                .unwrap_or(true)
    }
}

fn valid_coordinate(coordinate: Coordinate, limit: i32) -> bool {
    (-limit..=limit).contains(&coordinate.degrees_e7)
}

fn median_coordinate<const HISTORY: usize>(
    samples: &Vec<LocationSample, HISTORY>,
    f: impl Fn(LocationSample) -> i32,
) -> i32 {
    let mut values = [0i32; HISTORY];
    for (index, sample) in samples.iter().copied().enumerate() {
        values[index] = f(sample);
    }
    insertion_sort(&mut values[..samples.len()]);
    values[samples.len() / 2]
}

fn median_u8<const HISTORY: usize>(
    samples: &Vec<LocationSample, HISTORY>,
    f: impl Fn(LocationSample) -> u8,
) -> u8 {
    let mut values = [0u8; HISTORY];
    for (index, sample) in samples.iter().copied().enumerate() {
        values[index] = f(sample);
    }
    insertion_sort(&mut values[..samples.len()]);
    values[samples.len() / 2]
}

fn median_optional_u16<const HISTORY: usize>(
    samples: &Vec<LocationSample, HISTORY>,
    f: impl Fn(LocationSample) -> Option<u16>,
) -> Option<u16> {
    let mut values = [0u16; HISTORY];
    let mut count = 0;
    for sample in samples.iter().copied() {
        if let Some(value) = f(sample) {
            values[count] = value;
            count += 1;
        }
    }
    if count == 0 {
        return None;
    }
    insertion_sort(&mut values[..count]);
    Some(values[count / 2])
}

fn estimate_uncertainty_meters<const HISTORY: usize>(
    samples: &Vec<LocationSample, HISTORY>,
) -> Option<u32> {
    if samples.is_empty() {
        return None;
    }

    let lat_min = samples
        .iter()
        .map(|sample| sample.latitude.degrees_e7)
        .min()
        .unwrap_or(0);
    let lat_max = samples
        .iter()
        .map(|sample| sample.latitude.degrees_e7)
        .max()
        .unwrap_or(0);
    let lon_min = samples
        .iter()
        .map(|sample| sample.longitude.degrees_e7)
        .min()
        .unwrap_or(0);
    let lon_max = samples
        .iter()
        .map(|sample| sample.longitude.degrees_e7)
        .max()
        .unwrap_or(0);
    let spread_e7 = lat_max
        .saturating_sub(lat_min)
        .max(lon_max.saturating_sub(lon_min)) as u32;
    let spread_m = spread_e7.saturating_mul(LATITUDE_METERS_PER_DEGREE) / 10_000_000;
    let hdop_m = median_optional_u16(samples, |sample| sample.hdop_centi)
        .map(|hdop| (hdop as u32).saturating_mul(5) / 100)
        .unwrap_or(0);

    Some(spread_m.max(hdop_m).max(1))
}

fn insertion_sort<T: Ord + Copy>(values: &mut [T]) {
    for i in 1..values.len() {
        let value = values[i];
        let mut j = i;
        while j > 0 && values[j - 1] > value {
            values[j] = values[j - 1];
            j -= 1;
        }
        values[j] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_time::Instant;
    use raylar_drivers::gps::{UtcDateTime, UtcTime};

    fn fix(latitude: i32, longitude: i32, satellites: u8, hdop_centi: Option<u16>) -> GpsFix {
        GpsFix {
            latitude: Coordinate {
                degrees_e7: latitude,
            },
            longitude: Coordinate {
                degrees_e7: longitude,
            },
            utc_time: UtcDateTime {
                date: None,
                time: UtcTime {
                    hour: 12,
                    minute: 0,
                    second: 0,
                },
            },
            satellites,
            hdop_centi,
            system_timestamp: Instant::from_ticks(1),
        }
    }

    #[test]
    fn rejects_poor_fixes_before_history() {
        let mut filter = LocationFilter::<5>::new(LocationConfig::default());

        filter.ingest(fix(100, 200, 3, Some(100)));
        filter.ingest(fix(100, 200, 6, Some(900)));
        filter.ingest(fix(1_000_000_000, 200, 6, Some(100)));

        let state = filter.state();
        assert!(!state.valid);
        assert_eq!(state.fix_count_used, 0);
        assert_eq!(state.total_fix_count_seen, 3);
    }

    #[test]
    fn publishes_median_after_minimum_accepted_fixes() {
        let mut filter = LocationFilter::<5>::new(LocationConfig::default());

        filter.ingest(fix(520_000_010, -10_000_010, 8, Some(120)));
        filter.ingest(fix(520_000_020, -10_000_020, 7, Some(130)));
        let state = filter.ingest(fix(520_000_000, -10_000_000, 9, Some(110)));

        assert!(state.valid);
        assert_eq!(state.latitude.degrees_e7, 520_000_010);
        assert_eq!(state.longitude.degrees_e7, -10_000_010);
        assert_eq!(state.fix_count_used, 3);
        assert_eq!(state.source, LocationSource::Gps);
    }

    #[test]
    fn median_suppresses_single_outlier() {
        let mut filter = LocationFilter::<5>::new(LocationConfig::default());

        filter.ingest(fix(520_000_000, -10_000_000, 8, Some(100)));
        filter.ingest(fix(520_000_010, -10_000_010, 8, Some(100)));
        filter.ingest(fix(800_000_000, 900_000_000, 8, Some(100)));
        let state = filter.ingest(fix(520_000_020, -10_000_020, 8, Some(100)));

        assert!(state.valid);
        assert_eq!(state.latitude.degrees_e7, 520_000_020);
        assert_eq!(state.longitude.degrees_e7, -10_000_000);
    }
}
