//! Frozen pre-chunk transform for tests and same-image benchmark controls only.
use crate::bits::fold;
use crate::{estimate_rice_k, shift_for_peak, Channel, Error, Representation, Transform};
use raylar_stage_profiling::{sample, Span, Stage};

impl Transform<'_> {
    pub(crate) fn advance_cheap_reference(&mut self, mut budget: usize) -> Result<bool, Error> {
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
                    self.shift = shift_for_peak(self.peak as u32, self.config.bits)
                        .max(self.config.min_shift);
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
