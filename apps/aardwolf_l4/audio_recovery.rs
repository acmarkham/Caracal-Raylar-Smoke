//! Rejoin a continuously running DMA ring without restarting the microphone clock.
//! TIM5 and DFSDM derive from the same PLL, so elapsed banks are not CPU time.
pub fn rejoin_bank(
    first_irq_us: u64,
    now_us: u64,
    remaining: [usize; 4],
    half: usize,
) -> Option<u32> {
    let ring = 2 * half;
    if remaining.iter().any(|&n| n == 0 || n > ring) {
        return None;
    }
    let positions = remaining.map(|n| ring - n);
    let bank = positions[0] / half;
    // Stay at least 1 ms away from a bank boundary while clearing all readers.
    // Every lane will then skip to the same following full aligned bank.
    if positions
        .iter()
        .any(|&p| p / half != bank || p % half < 16 || p % half > half - 16)
    {
        return None;
    }
    let bank_us = half as u64 * 1_000_000 / 16_000;
    let completed = now_us.checked_add(bank_us)?.checked_sub(first_irq_us)? / bank_us;
    if completed % 2 != bank as u64 {
        return None;
    }
    u32::try_from(completed).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejoin_uses_elapsed_samples_even_when_irq_flags_coalesce() {
        for half in [128, 256] {
            let us = half as u64 * 1_000_000 / 16000;
            for completed in [1, 2, 7, 1000, 10001] {
                let pos = completed as usize % 2 * half + 32;
                let now = 1_000_000 + (completed - 1) * us + 2000;
                assert_eq!(
                    rejoin_bank(1_000_000, now, [half * 2 - pos; 4], half),
                    Some(completed as u32)
                );
            }
            assert_eq!(
                rejoin_bank(1_000_000, 1_002_000, [half * 2 - 32; 4], half),
                None
            );
            assert_eq!(rejoin_bank(1_000_000, 1_002_000, [half - 1; 4], half), None);
            assert_eq!(rejoin_bank(1_000_000, 1_002_000, [0; 4], half), None);
            assert_eq!(
                rejoin_bank(
                    1_000_000,
                    1_002_000,
                    [half - 32, half + 32, half - 32, half - 32],
                    half
                ),
                None
            );
        }
    }
}
