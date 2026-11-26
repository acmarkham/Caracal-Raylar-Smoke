//! Bounded integer Goertzel measurement for the bench's acoustic loopback.
pub struct Tone {
    coefficient: i64,
    s1: i64,
    s2: i64,
}
impl Tone {
    pub const fn new(coefficient: i64) -> Self {
        Self {
            coefficient,
            s1: 0,
            s2: 0,
        }
    }
    /// Up to 3200 samples of signed 24-bit PCM, optionally centered by a
    /// constant 24-bit offset. Keep captured low bits during this analysis.
    pub fn add(&mut self, sample: i32) {
        let s = i64::from(sample) + ((self.coefficient * self.s1) >> 15) - self.s2;
        self.s2 = self.s1;
        self.s1 = s;
    }
    pub fn power(&self) -> u64 {
        let a = i128::from(self.s1);
        let b = i128::from(self.s2);
        ((a * a + b * b - ((i128::from(self.coefficient) * a) >> 15) * b).max(0) >> 16) as u64
    }
}
pub const HZ800: i64 = 62328;
pub const HZ1000: i64 = 60547;
pub const HZ1200: i64 = 58393;
pub const HZ2000: i64 = 46341;
pub const HZ2200: i64 = 42562;
pub const HZ3000: i64 = 25080;
pub const HZ3200: i64 = 20252;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn detects_a_full_scale_1khz_tone_and_rejects_neighboring_bands() {
        let wave = [
            0, 12539, 23170, 30273, 32767, 30273, 23170, 12539, 0, -12539, -23170, -30273, -32768,
            -30273, -23170, -12539,
        ];
        let mut a = Tone::new(HZ800);
        let mut b = Tone::new(HZ1000);
        let mut c = Tone::new(HZ1200);
        for n in 0..3200 {
            a.add(wave[n % 16] * 256);
            b.add(wave[n % 16] * 256);
            c.add(wave[n % 16] * 256);
        }
        assert!(b.power() > a.power().max(c.power()) * 1000);
    }
    #[test]
    fn low_pcm_bits_survive_measurement_without_an_i16_quantization_step() {
        let wave = [
            0, 12, 23, 30, 32, 30, 23, 12, 0, -12, -23, -30, -32, -30, -23, -12,
        ];
        let mut tone = Tone::new(HZ1000);
        let mut neighbor = Tone::new(HZ1200);
        for n in 0..3200 {
            tone.add(wave[n % 16]);
            neighbor.add(wave[n % 16]);
        }
        assert!(tone.power() > neighbor.power().max(1) * 1000);
    }
}
