//! Timing from SpecAndDesign/Policy/ui.yaml; portable for boundary tests.
pub const STARTUP_VOLUME: u8 = 96;
pub const ERROR_VOLUME: u8 = 96;
pub fn startup_hz(ms: u64) -> u32 {
    let n = (ms / 105) as usize;
    if ms % 105 < 70 {
        [1047, 1319, 1568].get(n).copied().unwrap_or(0)
    } else {
        0
    }
}
pub fn error_hz(ms: u64) -> u32 {
    let phase = ms % 10_000;
    let n = (phase / 210) as usize;
    if phase % 210 < 140 {
        [440, 330, 220].get(n).copied().unwrap_or(0)
    } else {
        0
    }
}
pub fn red_on(ms: u64) -> bool {
    ms % 1000 < 500
}

#[derive(Default)]
pub struct CardDebounce(u8);
impl CardDebounce {
    /// Six consecutive absent samples, taken every 10 ms (at most 60 ms).
    pub fn removed(&mut self, absent: bool) -> bool {
        self.0 = if absent { self.0.saturating_add(1) } else { 0 };
        self.0 >= 6
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn policy_notes_gaps_and_repeat_boundaries() {
        for (ms, hz) in [
            (0, 1047),
            (69, 1047),
            (70, 0),
            (105, 1319),
            (210, 1568),
            (280, 0),
            (315, 0),
        ] {
            assert_eq!(startup_hz(ms), hz);
        }
        for (ms, hz) in [
            (0, 440),
            (139, 440),
            (140, 0),
            (210, 330),
            (420, 220),
            (560, 0),
            (9999, 0),
            (10000, 440),
        ] {
            assert_eq!(error_hz(ms), hz);
        }
        assert!(red_on(0) && red_on(499) && red_on(1000));
        assert!(!red_on(500) && !red_on(999));
    }
    #[test]
    fn contact_bounce_does_not_hide_sustained_removal() {
        let mut card = CardDebounce::default();
        for _ in 0..5 {
            assert!(!card.removed(true));
        }
        assert!(!card.removed(false));
        for _ in 0..5 {
            assert!(!card.removed(true));
        }
        assert!(card.removed(true));
        assert!(card.removed(true));
    }
}
