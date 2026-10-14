use crate::{bits::fold, Channel, Config, Error, Prediction, Representation};

/// Exact round(d*(2^(bits-1)-1)/peak), ties away from zero, without wide products.
/// The caller proves abs(d) <= peak <= i32::MAX and bits is 6/8/10/12.
pub fn quantize_ratio(d: i32, bits: u8, peak: i32) -> Result<i32, Error> {
    if !matches!(bits, 6 | 8 | 10 | 12) || peak < 0 {
        return Err(Error::InvalidConfig);
    }
    let a = d.checked_abs().ok_or(Error::SampleOutOfRange)?;
    if a > peak {
        return Err(Error::SampleOutOfRange);
    }
    if peak == 0 {
        return Ok(0);
    }
    let limit = (1 << (bits - 1)) - 1;
    if a == peak {
        return Ok(if d < 0 { -limit } else { limit });
    }
    // Common microphone blocks have ample product headroom. One native division
    // recovers both quotient and remainder; the modular path handles larger peaks.
    if let Some(product) = a.checked_mul(limit) {
        let mut q = product / peak;
        let remainder = product - q * peak;
        if remainder >= peak - remainder {
            q += 1;
        }
        return Ok(if d < 0 { -q } else { q });
    }
    // Obtain quotient/remainder for a*2^(bits-1), keeping remainder < peak.
    let mut quotient = 0;
    let mut remainder = a;
    for _ in 0..bits - 1 {
        quotient <<= 1;
        if remainder >= peak - remainder {
            remainder -= peak - remainder;
            quotient += 1;
        } else {
            remainder += remainder;
        }
    }
    // Subtract a to obtain a*(2^(bits-1)-1), then round without doubling overflow.
    if remainder < a {
        quotient -= 1;
        remainder = peak - (a - remainder);
    } else {
        remainder -= a;
    }
    if remainder >= peak - remainder {
        quotient += 1;
    }
    Ok(if d < 0 { -quotient } else { quotient })
}

#[derive(Default)]
struct RiceStats {
    sum: u32,
    ones: [u16; 13],
}
impl RiceStats {
    fn add(&mut self, value: i32) {
        let mut u = fold(value);
        self.sum += u;
        while u != 0 {
            self.ones[u.trailing_zeros() as usize] += 1;
            u &= u - 1;
        }
    }
    fn best(&self, count: usize, bits: u8, warmup: u32) -> (usize, u8) {
        let mut quotients = self.sum;
        let mut best = (usize::MAX, 0);
        for k in 0..=bits {
            let cost = warmup + quotients + count as u32 * (1 + u32::from(k));
            let bytes = (cost as usize).div_ceil(8);
            if bytes < best.0 {
                best = (bytes, k);
            }
            quotients = (quotients - u32::from(self.ones[k as usize])) >> 1;
        }
        best
    }
}

/// Bounded, resumable two-pass transform over interleaved recorder-owned PCM.
/// No extra sample arrays; MIC1 stays unchanged even when block sizes differ.
pub struct Transform<'a> {
    pub(crate) pcm: &'a mut [i32],
    pub(crate) channels: [Channel; 5],
    pub(crate) config: Config,
    channel: usize,
    offset: usize,
    quantizing: bool,
    peak: i32,
    previous: i32,
    direct: RiceStats,
    temporal: RiceStats,
}
impl<'a> Transform<'a> {
    pub fn new(pcm: &'a mut [i32], config: Config) -> Result<Self, Error> {
        config.validate()?;
        if pcm.is_empty() || !pcm.len().is_multiple_of(6) || pcm.len() / 6 > config.block_frames {
            return Err(Error::InvalidConfig);
        }
        Ok(Self {
            pcm,
            channels: [Channel::ZERO; 5],
            config,
            channel: 1,
            offset: 0,
            quantizing: false,
            peak: 0,
            previous: 0,
            direct: RiceStats::default(),
            temporal: RiceStats::default(),
        })
    }
    pub fn advance(&mut self, mut budget: usize) -> Result<bool, Error> {
        let frames = self.pcm.len() / 6;
        while budget != 0 && self.channel < 6 {
            let index = self.offset * 6 + self.channel;
            if !self.quantizing {
                let reference = self.pcm[self.offset * 6];
                let value = self.pcm[index];
                let bound = self.config.max_abs_pcm;
                if reference < -bound || reference > bound || value < -bound || value > bound {
                    return Err(Error::SampleOutOfRange);
                }
                let delta = value - reference;
                self.pcm[index] = delta;
                self.peak = self.peak.max(delta.abs());
            } else {
                let q = quantize_ratio(self.pcm[index], self.config.bits, self.peak)?;
                self.pcm[index] = q;
                if self.config.prediction != Prediction::TemporalRice {
                    self.direct.add(q);
                }
                if self.offset != 0 && self.config.prediction != Prediction::DirectRice {
                    self.temporal.add(q - self.previous);
                }
                self.previous = q;
            }
            self.offset += 1;
            budget -= 1;
            if self.offset == frames {
                self.offset = 0;
                if !self.quantizing {
                    self.quantizing = true;
                } else {
                    self.finish_channel(frames);
                }
            }
        }
        Ok(self.channel == 6)
    }
    fn finish_channel(&mut self, frames: usize) {
        let mut selected = Channel {
            scale: self.peak as u32,
            representation: Representation::Packed,
            k: 0,
        };
        let mut bytes = (frames * self.config.bits as usize).div_ceil(8);
        if self.peak == 0 {
            selected = Channel::ZERO;
        } else {
            if self.config.prediction != Prediction::TemporalRice {
                let (cost, k) = self.direct.best(frames, self.config.bits, 0);
                if cost < bytes {
                    bytes = cost;
                    selected.representation = Representation::Direct;
                    selected.k = k;
                }
            }
            if self.config.prediction != Prediction::DirectRice {
                let (cost, k) =
                    self.temporal
                        .best(frames - 1, self.config.bits, self.config.bits.into());
                if cost < bytes {
                    selected.representation = Representation::Temporal;
                    selected.k = k;
                }
            }
        }
        self.channels[self.channel - 1] = selected;
        self.channel += 1;
        self.quantizing = false;
        self.peak = 0;
        self.previous = 0;
        self.direct = RiceStats::default();
        self.temporal = RiceStats::default();
    }
    pub(crate) fn ready(&self) -> bool {
        self.channel == 6
    }
}
