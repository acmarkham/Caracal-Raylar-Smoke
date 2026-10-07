/// An exFAT date and time in UTC. Zero means no valid timestamp.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timestamp {
    pub packed: u32,
    pub ten_ms: u8,
}

impl Timestamp {
    /// Converts Unix UTC time to the exFAT range (1980 through 2107).
    pub fn from_unix(seconds: i64, microseconds: u32) -> Option<Self> {
        if microseconds >= 1_000_000 || !(315_532_800..4_354_819_200).contains(&seconds) {
            return None;
        }
        let days = seconds.div_euclid(86_400);
        let day_seconds = seconds.rem_euclid(86_400) as u32;
        // Civil date from days since 1970-01-01 (proleptic Gregorian calendar).
        let z = days + 719_468;
        let era = z.div_euclid(146_097);
        let day_of_era = z - era * 146_097;
        let year_of_era =
            (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
        let mut year = year_of_era + era * 400;
        let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
        let month_prime = (5 * day_of_year + 2) / 153;
        let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
        let month = month_prime + if month_prime < 10 { 3 } else { -9 };
        year += i64::from(month <= 2);
        if !(1980..=2107).contains(&year) {
            return None;
        }
        let hour = day_seconds / 3_600;
        let minute = day_seconds % 3_600 / 60;
        let second = day_seconds % 60;
        Some(Self {
            packed: ((year as u32 - 1980) << 25)
                | ((month as u32) << 21)
                | ((day as u32) << 16)
                | (hour << 11)
                | (minute << 5)
                | (second / 2),
            // The low bit of seconds and hundredths are stored separately.
            ten_ms: (second % 2 * 100 + microseconds / 10_000) as u8,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Timestamp;

    #[test]
    fn utc_dates_and_range() {
        assert_eq!(
            Timestamp::from_unix(315_532_800, 0).unwrap().packed,
            0x0021_0000
        );
        assert_eq!(
            Timestamp::from_unix(1_704_067_199, 990_000),
            Some(Timestamp {
                packed: 0x579f_bf7d,
                ten_ms: 199,
            })
        );
        assert!(Timestamp::from_unix(315_532_799, 0).is_none());
        assert!(Timestamp::from_unix(4_354_819_200, 0).is_none());
        assert!(Timestamp::from_unix(i64::MAX, 0).is_none());
        assert!(Timestamp::from_unix(1_704_067_199, 1_000_000).is_none());
        let leap_day = Timestamp::from_unix(951_782_400, 0).unwrap();
        assert_eq!((leap_day.packed >> 25) + 1980, 2000);
        assert_eq!((leap_day.packed >> 21) & 15, 2);
        assert_eq!((leap_day.packed >> 16) & 31, 29);
    }
}
