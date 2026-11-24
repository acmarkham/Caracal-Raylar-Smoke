//! Portable PCM interpretation and the exact nominal clock/gain budget.
#![no_std]

pub const SYS_HZ: u32 = 80_000_000;
pub const CLOCK_DIVIDER: u32 = 25;
pub const PDM_HZ: u32 = SYS_HZ / CLOCK_DIVIDER;
pub const SINC_ORDER: u32 = 5;
pub const FOSR: u32 = 50;
pub const IOSR: u32 = 4;
pub const RIGHT_SHIFT: u32 = 8;
pub const SAMPLE_RATE: u32 = PDM_HZ / FOSR / IOSR;
pub const CIC_GAIN: u64 = (FOSR as u64).pow(SINC_ORDER) * IOSR as u64;
pub const HALF: usize = 512;
pub const RING: usize = HALF * 2;
pub const SNAPSHOT_WORDS: usize = 8192;
pub const WARMUP_BLOCKS: u32 = 64;
pub const QUALIFICATION_BLOCKS: u32 = 1875; // Exactly 960000 samples / 60 seconds.

/// Same conversion used by the U5 Aardwolf producer: signed 24-bit PCM in i32.
pub const fn pcm24(word: u32) -> i32 {
    (word as i32) >> 8
}

#[derive(Clone, Copy)]
pub struct Stats {
    pub count: u32,
    pub min: i32,
    pub max: i32,
    pub sum: i64,
    pub squares: u64,
    pub positive: u32,
    pub negative: u32,
    pub clipped: u32,
    pub bad_metadata: u32,
    pub low_or: u8,
    pub low_and: u8,
}
impl Default for Stats {
    fn default() -> Self {
        Self {
            count: 0,
            min: i32::MAX,
            max: i32::MIN,
            sum: 0,
            squares: 0,
            positive: 0,
            negative: 0,
            clipped: 0,
            bad_metadata: 0,
            low_or: 0,
            low_and: 0xff,
        }
    }
}
impl Stats {
    pub fn add(&mut self, word: u32) {
        let value = pcm24(word);
        self.count += 1;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.sum += value as i64;
        self.squares += (value as i64 * value as i64) as u64;
        self.positive += u32::from(value > 0);
        self.negative += u32::from(value < 0);
        self.clipped += u32::from(value == -8_388_608 || value == 8_388_607);
        // ES0250 2.12.1: L476 RDATACH[2:0] is not implemented. Channel
        // identity comes from the fixed RCH configuration, never these bits.
        // Still reject pending injected conversions and nonzero reserved bits.
        self.bad_metadata += u32::from(word & 0xf8 != 0);
        self.low_or |= value as u8;
        self.low_and &= value as u8;
    }
    pub fn mean_power(&self) -> u64 {
        self.squares / self.count.max(1) as u64
    }
    /// Approximate AC power; integer mean rounding can leave a small residual.
    pub fn ac_mean_power(&self) -> u64 {
        let mean = self.sum / self.count.max(1) as i64;
        self.mean_power().saturating_sub((mean * mean) as u64)
    }
    pub fn peak(&self) -> u32 {
        self.min.unsigned_abs().max(self.max.unsigned_abs())
    }
}

pub fn hash_words(words: &[u32]) -> u32 {
    let mut hash = 0x811c9dc5u32;
    for word in words {
        for byte in word.to_le_bytes() {
            hash = (hash ^ byte as u32).wrapping_mul(0x01000193);
        }
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pcm_boundaries_and_metadata_do_not_change_samples() {
        for (word, value) in [
            (0x7fffff01, 8_388_607),
            (0x80000001, -8_388_608),
            (0xffffff01, -1),
            (0x00000101, 1),
            (1, 0),
        ] {
            assert_eq!(pcm24(word), value);
            assert_eq!(pcm24(word | 0xfe), value);
        }
    }
    #[test]
    fn exact_rate_and_internal_gain_fit_signed_widths() {
        assert_eq!(PDM_HZ, 3_200_000);
        assert_eq!(SAMPLE_RATE, 16_000);
        assert_eq!(SYS_HZ % (CLOCK_DIVIDER * FOSR * IOSR), 0);
        assert!(FOSR <= 73);
        assert_eq!(CIC_GAIN, 1_250_000_000);
        assert!(CIC_GAIN <= i32::MAX as u64 && CIC_GAIN >> RIGHT_SHIFT < (1 << 23));
        assert_eq!(QUALIFICATION_BLOCKS as usize * HALF, 960_000);
    }
    #[test]
    fn full_scale_power_needs_more_than_32_bits_and_preserves_sign() {
        let mut s = Stats::default();
        for _ in 0..HALF / 2 {
            s.add(0x80000001);
            s.add(0x7fffff01);
        }
        assert_eq!(s.negative, 256);
        assert_eq!(s.positive, 256);
        assert_eq!(s.sum, -256);
        assert_eq!(s.clipped, 512);
        assert_eq!(s.bad_metadata, 0);
        assert!(s.squares > u32::MAX as u64);
        assert_eq!(s.peak(), 8_388_608);
    }
    #[test]
    fn flags_and_low_pcm_bits_are_checked_independently() {
        let mut s = Stats::default();
        for word in [0x00000101, 0xffffff00, 0x00000211, 0x00000380] {
            s.add(word);
        }
        assert_eq!(s.bad_metadata, 2);
        assert_ne!(s.low_or ^ s.low_and, 0);
        assert_ne!(hash_words(&[0x12345601]), hash_words(&[0x12345701]));
    }
    #[test]
    fn dc_offset_does_not_hide_audio_variation() {
        let mut constant = Stats::default();
        let mut varying = Stats::default();
        for _ in 0..HALF / 2 {
            constant.add(1000 << 8);
            constant.add(1000 << 8);
            varying.add(900 << 8);
            varying.add(1100 << 8);
        }
        assert_eq!(constant.ac_mean_power(), 0);
        assert_eq!(varying.ac_mean_power(), 10_000);
        assert_eq!(varying.negative, 0); // Valid unipolar PCM around a DC bias.
    }
}
