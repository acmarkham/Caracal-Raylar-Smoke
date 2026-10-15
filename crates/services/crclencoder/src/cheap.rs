//! Experimental independent-channel quantization and temporal Rice profile.
//! Only scalar state; all PCM transforms use native i32 in the existing block.
use crate::bits::fold;
use crate::{Channel, Error, Representation, Transform};
use raylar_stage_profiling::{sample, Span, Stage};

/// Smallest conservative shift whose rounded positive peak fits the symmetric
/// signed precision. Caller has validated bits and the bounded microphone range.
pub fn shift_for_peak(peak: u32, bits: u8) -> u8 {
    let mut shift = (32 - peak.leading_zeros()).saturating_sub(u32::from(bits - 1));
    if shift != 0 && (peak + (1 << (shift - 1))) >> shift > (1 << (bits - 1)) - 1 {
        shift += 1;
    }
    shift as u8
}

/// Geometric-magnitude estimate: floor(log2(ceil(mean folded residual))).
/// One integer division and CLZ per channel/block; no histogram or k search.
pub fn estimate_rice_k(sum_folded: u32, residuals: usize) -> u8 {
    if sum_folded == 0 || residuals == 0 {
        return 0;
    }
    let mean = sum_folded.div_ceil(residuals as u32);
    (31 - mean.leading_zeros()) as u8
}

impl Transform<'_> {
    pub(crate) fn advance_cheap(&mut self, mut budget: usize) -> Result<bool, Error> {
        let frames = self.pcm.len() / 6;
        while budget != 0 && self.channel < 6 {
            let index = self.offset * 6 + self.channel;
            let sampled = sample(self.offset + self.channel * 7);
            if !self.quantizing {
                let _profile = sampled.then(|| Span::new(Stage::SpatialDeltaSample));
                let value = self.pcm[index];
                let bound = self.config.max_abs_pcm;
                if value < -bound || value > bound {
                    return Err(Error::SampleOutOfRange);
                }
                self.peak = self.peak.max(value.abs());
            } else {
                let q = {
                    let _profile = sampled.then(|| Span::new(Stage::QuantizeFastSample));
                    let half = if self.shift == 0 {
                        0
                    } else {
                        1 << (self.shift - 1)
                    };
                    // Bound <= i32::MAX/2 leaves room for rounding. Ties toward +infinity.
                    (self.pcm[index] + half) >> self.shift
                };
                self.pcm[index] = q;
                let _profile = sampled.then(|| Span::new(Stage::RiceStatsSample));
                if self.offset != 0 {
                    self.sum_folded += fold(q - self.previous);
                }
                self.previous = q;
            }
            self.offset += 1;
            budget -= 1;
            if self.offset == frames {
                self.offset = 0;
                if !self.quantizing {
                    self.shift = shift_for_peak(self.peak as u32, self.config.bits);
                    self.quantizing = true;
                } else {
                    let _profile = Span::new(Stage::RiceSelect);
                    self.channels[self.channel - 1] = Channel {
                        scale: u32::from(self.shift),
                        representation: Representation::Temporal,
                        k: estimate_rice_k(self.sum_folded, frames - 1),
                    };
                    self.channel += 1;
                    self.quantizing = false;
                    self.peak = 0;
                    self.previous = 0;
                    self.sum_folded = 0;
                }
            }
        }
        Ok(self.channel == 6)
    }
}
