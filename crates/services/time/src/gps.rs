use embassy_sync::watch::DynReceiver;
use embassy_time::Duration;
use raylar_drivers::gps::{PpsTimingSource, TimeCorrelation, UtcDate, UtcDateTime};

use crate::{Anchor, AnchorQuality, AnchorSender, TimeSource, UtcTimestamp};

pub const GPS_PPS_UNCERTAINTY: Duration = Duration::from_micros(100);
// Hardware capture removes edge-to-edge interrupt jitter, but its first edge
// is still aligned to Embassy time in software. Retain the conservative phase
// bound until those clock domains have a fully hardware-defined epoch.
pub const GPS_CAPTURE_PPS_UNCERTAINTY: Duration = Duration::from_micros(100);

pub async fn run_gps_time_source<const ANCHOR_DEPTH: usize>(
    mut correlations: DynReceiver<'static, TimeCorrelation>,
    anchors: AnchorSender<'static, ANCHOR_DEPTH>,
) -> ! {
    loop {
        if let Some(anchor) = correlation_to_anchor(correlations.changed().await) {
            anchors.send(anchor).await;
        }
    }
}

pub fn correlation_to_anchor(correlation: TimeCorrelation) -> Option<Anchor> {
    let utc = gps_utc_to_timestamp(correlation.utc_time)?;
    // NMEA supplies the UTC second associated with the edge, but its serial
    // arrival timestamp is not a sufficiently precise clock anchor. A
    // correlation without a matched PPS edge is deliberately ignored.
    let pps_timestamp = correlation.pps_timestamp?;
    Some(Anchor {
        system_time: pps_timestamp,
        utc,
        quality: AnchorQuality::new(
            if correlation.pps_timing_source == Some(PpsTimingSource::Tim4Capture) {
                GPS_CAPTURE_PPS_UNCERTAINTY.as_micros()
            } else {
                GPS_PPS_UNCERTAINTY.as_micros()
            },
        ),
        source: TimeSource::GpsPps,
        capture_ticks: correlation.pps_capture_ticks,
    })
}

pub fn gps_utc_to_timestamp(value: UtcDateTime) -> Option<UtcTimestamp> {
    let date = value.date?;
    if !valid_date(date) || value.time.hour > 23 || value.time.minute > 59 || value.time.second > 60
    {
        return None;
    }
    let days = days_from_civil(date.year as i64, date.month as i64, date.day as i64);
    let seconds = days
        .checked_mul(86_400)?
        .checked_add(value.time.hour as i64 * 3_600)?
        .checked_add(value.time.minute as i64 * 60)?
        .checked_add(value.time.second.min(59) as i64)?;
    UtcTimestamp::new(seconds, 0)
}

fn valid_date(date: UtcDate) -> bool {
    if date.year < 1970 || !(1..=12).contains(&date.month) || date.day == 0 {
        return false;
    }
    let leap = date.year % 4 == 0 && (date.year % 100 != 0 || date.year % 400 == 0);
    let max_day = match date.month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    date.day <= max_day
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let year = year - i64::from(month <= 2);
    let era = year.div_euclid(400);
    let year_of_era = year - era * 400;
    let adjusted_month = month + if month > 2 { -3 } else { 9 };
    let day_of_year = (153 * adjusted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;
    use embassy_time::Instant;
    use raylar_drivers::gps::{PpsTimingSource, UtcTime};

    fn correlation(pps_timestamp: Option<Instant>) -> TimeCorrelation {
        TimeCorrelation {
            utc_time: UtcDateTime {
                date: Some(UtcDate {
                    year: 2024,
                    month: 1,
                    day: 1,
                }),
                time: UtcTime {
                    hour: 0,
                    minute: 0,
                    second: 0,
                },
            },
            local_timestamp: Instant::from_ticks(1_100),
            pps_timestamp,
            pps_capture_ticks: None,
            pps_capture_delta_ticks: None,
            pps_capture_frequency_hz: None,
            pps_timing_source: Some(PpsTimingSource::EmbassyInstant),
        }
    }

    #[test]
    fn ignores_nmea_without_a_matched_pps_edge() {
        assert_eq!(correlation_to_anchor(correlation(None)), None);
    }

    #[test]
    fn uses_only_the_matched_pps_timestamp_as_an_anchor() {
        let pps = Instant::from_ticks(1_000);
        let anchor = correlation_to_anchor(correlation(Some(pps))).unwrap();
        assert_eq!(anchor.system_time, pps);
        assert_eq!(anchor.source, TimeSource::GpsPps);
        assert_eq!(anchor.quality.uncertainty_us, 100);
    }

    #[test]
    fn hardware_capture_retains_cross_clock_phase_bound() {
        let pps = Instant::from_ticks(1_000);
        let mut value = correlation(Some(pps));
        value.pps_timing_source = Some(PpsTimingSource::Tim4Capture);
        let anchor = correlation_to_anchor(value).unwrap();
        assert_eq!(anchor.quality.uncertainty_us, 100);
    }

    #[test]
    fn converts_unix_epoch() {
        let value = UtcDateTime {
            date: Some(UtcDate {
                year: 1970,
                month: 1,
                day: 1,
            }),
            time: UtcTime {
                hour: 0,
                minute: 0,
                second: 0,
            },
        };
        assert_eq!(gps_utc_to_timestamp(value), UtcTimestamp::new(0, 0));
    }

    #[test]
    fn converts_known_date() {
        let value = UtcDateTime {
            date: Some(UtcDate {
                year: 2024,
                month: 1,
                day: 1,
            }),
            time: UtcTime {
                hour: 0,
                minute: 0,
                second: 0,
            },
        };
        assert_eq!(
            gps_utc_to_timestamp(value),
            UtcTimestamp::new(1_704_067_200, 0)
        );
    }
}
