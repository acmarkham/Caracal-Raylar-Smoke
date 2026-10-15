#![no_std]
//! Bounded CRCL v1 spatial codec. MIC1 is immutable; other channels are transformed in-place.

mod bits;
mod cheap;
mod format;
mod quantize;
mod sha256;

pub use cheap::{estimate_rice_k, shift_for_peak};
pub use format::{crc32c, segment_identity, Completion, Encoder, Header, TimeAnchor};
pub use quantize::{quantize_ratio, Transform};
pub use sha256::Sha256;

pub const CHANNELS: usize = 6;
pub const HEADER_CAPACITY: usize = 4096;
pub const fn record_capacity(block: usize, bits: u8) -> usize {
    96 + 5 * (block * bits as usize).div_ceil(8)
}
pub const fn workspace_capacity(block: usize, bits: u8) -> usize {
    let data = record_capacity(block, bits);
    if data > HEADER_CAPACITY {
        data
    } else {
        HEADER_CAPACITY
    }
}

/// With the estimated k, mean quotient is below two. At k=bits each
/// quotient is at most one; therefore total Rice payload fits bits+2 per value.
pub const fn cheap_record_capacity(block: usize, bits: u8) -> usize {
    88 + 5 * (bits as usize + block.saturating_sub(1) * (bits as usize + 2)).div_ceil(8)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Algorithm {
    SpatialV1,
    TemporalShiftV2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    InvalidConfig,
    SampleOutOfRange,
    OutputTooSmall,
    InvalidMetadata,
    CounterOverflow,
    NotReady,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prediction {
    DirectRice,
    TemporalRice,
    Auto,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    pub algorithm: Algorithm,
    pub block_frames: usize,
    pub bits: u8,
    pub prediction: Prediction,
    /// V2 minimum absolute PCM right shift for MIC2..MIC6, before prediction.
    /// Combined with peak scaling in one rounding operation. Must be zero in v1.
    pub min_shift: u8,
    /// Proven bound after microphone packing/shift. Arbitrary i32 PCM is unsupported.
    pub max_abs_pcm: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::SpatialV1,
            block_frames: 1024,
            bits: 10,
            prediction: Prediction::DirectRice,
            min_shift: 0,
            max_abs_pcm: 1 << 23,
        }
    }
}

impl Config {
    pub const fn cheap() -> Self {
        Self {
            algorithm: Algorithm::TemporalShiftV2,
            block_frames: 1024,
            bits: 10,
            prediction: Prediction::TemporalRice,
            min_shift: 0,
            max_abs_pcm: 1 << 23,
        }
    }
    pub const fn is_cheap(self) -> bool {
        matches!(self.algorithm, Algorithm::TemporalShiftV2)
    }
    pub const fn version(self) -> u16 {
        if self.is_cheap() {
            2
        } else {
            1
        }
    }
    pub const fn record_capacity(self, frames: usize) -> usize {
        if self.is_cheap() {
            cheap_record_capacity(frames, self.bits)
        } else {
            record_capacity(frames, self.bits)
        }
    }
    pub const fn workspace_capacity(self) -> usize {
        let capacity = self.record_capacity(self.block_frames);
        if capacity > HEADER_CAPACITY {
            capacity
        } else {
            HEADER_CAPACITY
        }
    }
    pub fn validate(self) -> Result<(), Error> {
        if !matches!(self.block_frames, 256 | 512 | 1024 | 2048 | 4096)
            || !matches!(self.bits, 6 | 8 | 10 | 12)
            || self.max_abs_pcm <= 0
            || self.max_abs_pcm > i32::MAX / 2
            || self.min_shift > 30
            || (!self.is_cheap() && self.min_shift != 0)
            || (self.is_cheap() && self.prediction != Prediction::TemporalRice)
        {
            return Err(Error::InvalidConfig);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Representation {
    Zero,
    Direct,
    Temporal,
    Packed,
}

#[derive(Clone, Copy)]
pub(crate) struct Channel {
    pub scale: u32,
    pub representation: Representation,
    pub k: u8,
}

impl Channel {
    const ZERO: Self = Self {
        scale: 0,
        representation: Representation::Zero,
        k: 0,
    };
}

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests;
