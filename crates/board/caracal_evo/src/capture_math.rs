//! Pure arithmetic at the TIM3/TIM2 capture and UTC-label boundaries.

pub const MASK48: u64 = (1u64 << 48) - 1;

/// Resolve CCR2 against a coherent live 48-bit count. Requires capture service
/// latency below one TIM3 period (65.536 ms); this is not a 48-bit hardware latch.
pub fn captured_count(live: u64, ccr2: u16) -> u64 {
    let candidate = (live & !0xffff) | ccr2 as u64;
    if ccr2 > live as u16 {
        candidate.wrapping_sub(1 << 16) & MASK48
    } else {
        candidate & MASK48
    }
}

#[derive(Default)]
pub struct Labels {
    last: Option<(u64, i64)>,
}

impl Labels {
    /// One UTC label per edge, bounded serial latency, and matching sequence /
    /// UTC increments. A discontinuity needs a fresh pair before admission.
    pub fn accept(
        &mut self,
        sequence: u64,
        edge_us: u64,
        arrival_us: u64,
        utc_seconds: i64,
    ) -> bool {
        let Some(age) = arrival_us.checked_sub(edge_us) else {
            return false;
        };
        if age > 750_000 {
            return false;
        }
        if let Some((previous, utc)) = self.last {
            if sequence <= previous {
                return false;
            }
            if utc_seconds.checked_sub(utc) != i64::try_from(sequence - previous).ok() {
                self.last = Some((sequence, utc_seconds));
                return false;
            }
        }
        self.last = Some((sequence, utc_seconds));
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_capture_before_and_after_carry() {
        assert_eq!(captured_count(0x12_fff0, 0xffe0), 0x12_ffe0);
        assert_eq!(captured_count(0x13_0020, 0xfffe), 0x12_fffe);
        assert_eq!(captured_count(0x13_0020, 0x0010), 0x13_0010);
        assert_eq!(captured_count(0x0000_0001_0010, 0xffff), 0x0000_0000_ffff);
        assert_eq!(captured_count(0x20, 0xfffe), MASK48 - 1);
    }

    #[test]
    fn every_low_word_resolves_across_a_carry() {
        for low in 0..=u16::MAX {
            let captured = (0x1234_5678u64 << 16) | low as u64;
            for latency in [0, 1, 8, 99, 10_000, 32_000, 65_535] {
                assert_eq!(captured_count((captured + latency) & MASK48, low), captured);
            }
        }
    }

    #[test]
    fn labels_reject_duplicates_old_edges_and_future_edges() {
        let mut l = Labels::default();
        assert!(!l.accept(1, 100, 99, 1000));
        assert!(!l.accept(1, 100, 750_101, 1000));
        assert!(l.accept(1, 100, 750_100, 1000));
        assert!(!l.accept(1, 100, 200, 1000));
        assert!(l.accept(2, 1_000_100, 1_100_100, 1001));
    }

    #[test]
    fn labels_handle_skipped_sentences_and_reject_second_slips() {
        let mut l = Labels::default();
        assert!(l.accept(1, 0, 100_000, 1000));
        assert!(l.accept(4, 3_000_000, 3_100_000, 1003));
        assert!(!l.accept(5, 4_000_000, 4_100_000, 1005));
        assert!(l.accept(6, 5_000_000, 5_100_000, 1006));
        assert!(!l.accept(7, 6_000_000, 6_100_000, 1000));
    }
}
