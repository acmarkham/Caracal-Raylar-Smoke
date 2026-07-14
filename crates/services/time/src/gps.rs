use embassy_sync::watch::DynReceiver;
use embassy_time::Duration;
use raylar_drivers::gps::{TimeCorrelation, UtcDate, UtcDateTime};

use crate::{Anchor, AnchorQuality, AnchorSender, TimeSource, UtcTimestamp};

pub const GPS_NMEA_UNCERTAINTY: Duration = Duration::from_secs(1);
pub const GPS_PPS_UNCERTAINTY: Duration = Duration::from_micros(100);

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
    let fine = correlation.pps_timestamp.is_some();
    Some(Anchor {
        system_time: correlation
            .pps_timestamp
            .unwrap_or(correlation.local_timestamp),
        utc,
        quality: AnchorQuality::new(if fine {
            GPS_PPS_UNCERTAINTY.as_micros()
        } else {
            GPS_NMEA_UNCERTAINTY.as_micros()
        }),
        source: if fine {
            TimeSource::GpsPps
        } else {
            TimeSource::GpsNmea
        },
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
    use raylar_drivers::gps::UtcTime;

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
