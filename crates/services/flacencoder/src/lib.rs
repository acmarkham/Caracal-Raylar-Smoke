//! Bounded native FLAC encoding. No allocation and no firmware I/O.
#![no_std]

mod bitstream;
mod metadata;
mod predict;
use bitstream::{Bits, crc8, crc16};
pub use predict::Analysis;
#[cfg(feature = "benchmark")]
pub use predict::benchmark::{Options as BenchmarkOptions, RiceMethod, Search};
use predict::{fold, residual};

pub const DEFAULT_BLOCK_SAMPLES: usize = 2048;
pub const METADATA_BYTES: usize = 1024;
pub const fn frame_capacity(block: usize) -> usize {
    block.saturating_mul(4).saturating_add(32)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    InvalidConfig,
    SampleOutOfRange,
    OutputTooSmall,
    CounterOverflow,
    AnalysisIncomplete,
    InvalidFrame,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Prediction {
    /// Choose the smallest frame across fixed orders 0 through 4.
    Auto,
    /// Search only this order, retaining constant/verbatim fallback.
    Fixed(u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    pub sample_rate_hz: u32,
    pub bits_per_sample: u8,
    pub block_samples: usize,
    pub prediction: Prediction,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            sample_rate_hz: 16_000,
            bits_per_sample: 32,
            block_samples: DEFAULT_BLOCK_SAMPLES,
            prediction: Prediction::Auto,
        }
    }
}

impl Config {
    pub fn validate(self) -> Result<(), Error> {
        if !matches!(self.bits_per_sample, 8 | 16 | 24 | 32)
            || !matches!(self.block_samples, 1024 | 2048 | 4096)
            || self.sample_rate_hz == 0
            || self.sample_rate_hz >= 1 << 20
            || rate_code(self.sample_rate_hz).is_none()
            || matches!(self.prediction, Prediction::Fixed(order) if order > 4)
        {
            return Err(Error::InvalidConfig);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Subframe {
    Constant,
    Verbatim,
    Fixed {
        order: u8,
        rice_bits: u8,
        parameter: u8,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FrameStats {
    pub bytes: usize,
    pub samples: usize,
    pub subframe: Subframe,
    frame_number: u32,
}

/// Encoding is transactional: call `commit` only after the frame was appended.
pub struct Encoder {
    config: Config,
    frames: u32,
    samples: u64,
    min_frame: u32,
    max_frame: u32,
    ended: bool,
}

impl Encoder {
    pub fn new(config: Config) -> Result<Self, Error> {
        config.validate()?;
        Ok(Self {
            config,
            frames: 0,
            samples: 0,
            min_frame: 0,
            max_frame: 0,
            ended: false,
        })
    }
    pub const fn config(&self) -> Config {
        self.config
    }
    pub const fn samples(&self) -> u64 {
        self.samples
    }
    pub fn reset(&mut self) {
        self.frames = 0;
        self.samples = 0;
        self.min_frame = 0;
        self.max_frame = 0;
        self.ended = false;
    }
    pub fn analyze<'a>(&self, samples: &'a [i32]) -> Result<Analysis<'a>, Error> {
        if samples.is_empty() || samples.len() > self.config.block_samples || self.ended {
            return Err(Error::InvalidFrame);
        }
        if self.frames > 0x7fff_ffff || self.samples + samples.len() as u64 >= 1 << 36 {
            return Err(Error::CounterOverflow);
        }
        Analysis::new(samples, self.config.bits_per_sample, self.config.prediction)
    }
    pub fn encode(&self, samples: &[i32], output: &mut [u8]) -> Result<FrameStats, Error> {
        let mut analysis = self.analyze(samples)?;
        while !analysis.advance(64) {}
        self.encode_analyzed(analysis, output)
    }
    /// Experimental selection controls, compiled only into benchmark builds.
    #[cfg(feature = "benchmark")]
    pub fn analyze_variant<'a>(
        &self,
        samples: &'a [i32],
        options: BenchmarkOptions,
    ) -> Result<Analysis<'a>, Error> {
        let mut analysis = self.analyze(samples)?;
        analysis.select_variant(options)?;
        Ok(analysis)
    }
    pub fn encode_analyzed(
        &self,
        analysis: Analysis<'_>,
        output: &mut [u8],
    ) -> Result<FrameStats, Error> {
        let subframe = analysis.result()?;
        let samples = analysis.samples();
        // Also validate counters/config when an analysis was created by another encoder.
        self.analyze(samples)?;
        if analysis.depth() != self.config.bits_per_sample {
            return Err(Error::InvalidConfig);
        }
        if matches!((self.config.prediction, subframe),
            (Prediction::Fixed(expected), Subframe::Fixed { order, .. }) if order != expected)
        {
            return Err(Error::InvalidConfig);
        }
        let b = self.config.bits_per_sample;
        let mut bits = Bits::new(output);
        bits.write(0xfff8, 16)?;
        let n = samples.len();
        let block_code = if n == self.config.block_samples {
            n.ilog2() as u8
        } else if n <= 256 {
            6
        } else {
            7
        };
        let (rate, rate_value, rate_width) =
            rate_code(self.config.sample_rate_hz).ok_or(Error::InvalidConfig)?;
        bits.write(block_code.into(), 4)?;
        bits.write(rate.into(), 4)?;
        bits.write(0, 4)?; // mono
        bits.write(
            match b {
                8 => 1,
                16 => 4,
                24 => 6,
                32 => 7,
                _ => return Err(Error::InvalidConfig),
            },
            3,
        )?;
        bits.write(0, 1)?;
        bits.number(self.frames)?;
        if block_code == 6 || block_code == 7 {
            bits.write((n - 1) as u64, if block_code == 6 { 8 } else { 16 })?;
        }
        bits.write(rate_value.into(), rate_width)?;
        let crc = crc8(bits.bytes());
        bits.write(crc.into(), 8)?;
        let kind = match subframe {
            Subframe::Constant => 0,
            Subframe::Verbatim => 1,
            Subframe::Fixed { order, .. } => 8 + order,
        };
        bits.write(u64::from(kind) << 1, 8)?; // reserved zero, type, no wasted bits
        match subframe {
            Subframe::Constant => bits.write(samples[0] as u64, b)?,
            Subframe::Verbatim => {
                for sample in samples {
                    bits.write(*sample as u64, b)?;
                }
            }
            Subframe::Fixed {
                order,
                rice_bits,
                parameter,
            } => {
                for sample in &samples[..order as usize] {
                    bits.write(*sample as u64, b)?;
                }
                bits.write(if rice_bits == 4 { 0 } else { 1 }, 2)?;
                bits.write(0, 4)?; // one residual partition
                bits.write(parameter.into(), rice_bits)?;
                for index in order as usize..n {
                    let u = fold(residual(samples, index, order));
                    bits.unary(u >> parameter)?;
                    bits.write(u & ((1u64 << parameter) - 1), parameter)?;
                }
            }
        }
        bits.align()?;
        let crc = crc16(bits.bytes());
        bits.write(crc.into(), 16)?;
        Ok(FrameStats {
            bytes: bits.bytes().len(),
            samples: n,
            subframe,
            frame_number: self.frames,
        })
    }
    pub fn commit(&mut self, frame: FrameStats) -> Result<(), Error> {
        if frame.frame_number != self.frames
            || frame.samples == 0
            || frame.samples > self.config.block_samples
            || frame.bytes > frame_capacity(self.config.block_samples)
            || self.ended
        {
            return Err(Error::InvalidFrame);
        }
        if self.frames > 0x7fff_ffff || self.samples + frame.samples as u64 >= 1 << 36 {
            return Err(Error::CounterOverflow);
        }
        self.frames += 1;
        self.samples += frame.samples as u64;
        self.min_frame = if self.min_frame == 0 {
            frame.bytes as u32
        } else {
            self.min_frame.min(frame.bytes as u32)
        };
        self.max_frame = self.max_frame.max(frame.bytes as u32);
        self.ended = frame.samples < self.config.block_samples;
        Ok(())
    }
}

fn rate_code(rate: u32) -> Option<(u8, u32, u8)> {
    let common = [
        88200, 176400, 192000, 8000, 16000, 22050, 24000, 32000, 44100, 48000, 96000,
    ];
    if let Some(index) = common.iter().position(|x| *x == rate) {
        return Some((index as u8 + 1, 0, 0));
    }
    if rate > 0 && rate <= 65535 {
        Some((13, rate, 16))
    } else if rate > 0 && rate <= 655350 && rate.is_multiple_of(10) {
        Some((14, rate / 10, 16))
    } else {
        None
    }
}

#[cfg(test)]
extern crate std;
#[cfg(test)]
mod tests;
