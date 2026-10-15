//! Experimental independent-channel quantization and temporal Rice profile.
//! Only scalar state; all PCM transforms use native i32 in the existing block.
use crate::bits::fold;
use crate::{Channel, Error, Representation, Transform};
use raylar_stage_profiling::{sample, Span, Stage};

#[cfg(any(test, feature = "loop-benchmark"))]
#[path = "cheap_reference.rs"]
mod reference;
#[cfg(test)]
#[path = "cheap_tests.rs"]
mod tests;

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
            // Bound the work once per phase/channel chunk. The slice and stride
            // skip MIC1 and avoid rebuilding/checking a scalar index each time.
            let start = self.offset;
            let count = budget.min(frames - start);
            let end = start + count;
            let channel = self.channel;
            if !self.quantizing {
                let bound = self.config.max_abs_pcm;
                let mut peak = self.peak;
                for (n, &value) in self.pcm[start * 6 + channel..end * 6]
                    .iter()
                    .step_by(6)
                    .enumerate()
                {
                    let sampled = sample(start + n + channel * 7);
                    let _profile = sampled.then(|| Span::new(Stage::SpatialDeltaSample));
                    if value < -bound || value > bound {
                        // Match the previous loop's exact progress on failure:
                        // the rejected sample itself has not consumed budget.
                        self.offset = start + n;
                        self.peak = peak;
                        return Err(Error::SampleOutOfRange);
                    }
                    peak = peak.max(value.abs());
                }
                self.peak = peak;
            } else {
                let shift = self.shift;
                let half = if shift == 0 { 0 } else { 1 << (shift - 1) };
                let mut previous = self.previous;
                let mut sum_folded = self.sum_folded;
                for (n, value) in self.pcm[start * 6 + channel..end * 6]
                    .iter_mut()
                    .step_by(6)
                    .enumerate()
                {
                    let sampled = sample(start + n + channel * 7);
                    let q = {
                        let _profile = sampled.then(|| Span::new(Stage::QuantizeFastSample));
                        // The validated PCM bound leaves i32 rounding headroom.
                        (*value + half) >> shift
                    };
                    *value = q;
                    let _profile = sampled.then(|| Span::new(Stage::RiceStatsSample));
                    if start + n != 0 {
                        sum_folded += fold(q - previous);
                    }
                    previous = q;
                }
                self.previous = previous;
                self.sum_folded = sum_folded;
            }
            self.offset = end;
            budget -= count;
            if end == frames {
                self.offset = 0;
                if !self.quantizing {
                    // Use one absolute shift: never quantize twice or add the
                    // noise-floor discard to the peak-derived discard.
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
