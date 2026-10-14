#![no_std]
//! Bounded CRCL v1 spatial codec. MIC1 is immutable; other channels are transformed in-place.

mod bits;
mod format;
mod quantize;
mod sha256;

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
    pub block_frames: usize,
    pub bits: u8,
    pub prediction: Prediction,
    /// Proven bound after microphone packing/shift. Arbitrary i32 PCM is unsupported.
    pub max_abs_pcm: i32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            block_frames: 1024,
            bits: 10,
            prediction: Prediction::DirectRice,
            max_abs_pcm: 1 << 23,
        }
    }
}

impl Config {
    pub fn validate(self) -> Result<(), Error> {
        if !matches!(self.block_frames, 256 | 512 | 1024 | 2048 | 4096)
            || !matches!(self.bits, 6 | 8 | 10 | 12)
            || self.max_abs_pcm <= 0
            || self.max_abs_pcm > i32::MAX / 2
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
