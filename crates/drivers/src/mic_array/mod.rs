//! Continuous, heapless PDM microphone acquisition.
//!
//! The DMA storage lives in [`MicrophoneResources`], so consumers can inspect a
//! completed half without copying it. Capture is deliberately best-effort: a
//! slow consumer may observe a half after DMA has started overwriting it.

use core::cell::UnsafeCell;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::{Receiver, Watch};

#[cfg(feature = "stm32")]
pub mod stm32;
#[cfg(feature = "stm32")]
mod stm32_config;

pub const MAX_MICROPHONES: usize = 6;
pub const DEFAULT_WATCHERS: usize = 4;

pub type FrameReceiver<'a, const WATCHERS: usize> =
    Receiver<'a, CriticalSectionRawMutex, CaptureState, WATCHERS>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicrophoneMode {
    Mono,
    Hexaphonic,
}

impl MicrophoneMode {
    pub const fn channel_count(self) -> usize {
        match self {
            Self::Mono => 1,
            Self::Hexaphonic => 6,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SampleRate {
    Hz8000,
    Hz16000,
    Hz32000,
    Hz44100,
    Hz96000,
}

impl SampleRate {
    pub const fn hz(self) -> u32 {
        match self {
            Self::Hz8000 => 8_000,
            Self::Hz16000 => 16_000,
            Self::Hz32000 => 32_000,
            Self::Hz44100 => 44_100,
            Self::Hz96000 => 96_000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BitDepth {
    Bits16,
    Bits24,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SamplePacking {
    Bits16,
    Bits24,
    Bits32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SincFilter {
    Sinc4,
    Sinc5,
}

impl SincFilter {
    /// Maximum CIC decimation for the one-bit SITF PDM input (RM0456 table 376).
    pub const fn max_pdm_decimation(self) -> u16 {
        match self {
            Self::Sinc4 => 76,
            Self::Sinc5 => 32,
        }
    }

    pub const fn order(self) -> u8 {
        match self {
            Self::Sinc4 => 4,
            Self::Sinc5 => 5,
        }
    }
}

/// CIC scale encodings and gains from RM0456 table 377.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CicScale {
    DbMinus48_2 = 0x20,
    DbMinus44_6 = 0x21,
    DbMinus42_1 = 0x22,
    DbMinus38_6 = 0x23,
    DbMinus36_1 = 0x24,
    DbMinus32_6 = 0x25,
    DbMinus30_1 = 0x26,
    DbMinus26_6 = 0x27,
    DbMinus24_1 = 0x28,
    DbMinus20_6 = 0x29,
    DbMinus18_1 = 0x2a,
    DbMinus14_5 = 0x2b,
    DbMinus12_0 = 0x2c,
    DbMinus8_5 = 0x2d,
    DbMinus6_0 = 0x2e,
    DbMinus2_5 = 0x2f,
    Db0_0 = 0x00,
    DbPlus3_5 = 0x01,
    DbPlus6_0 = 0x02,
    DbPlus9_5 = 0x03,
    DbPlus12_0 = 0x04,
    DbPlus15_6 = 0x05,
    DbPlus18_1 = 0x06,
    DbPlus21_6 = 0x07,
    DbPlus24_1 = 0x08,
    DbPlus27_6 = 0x09,
    DbPlus30_1 = 0x0a,
    DbPlus33_6 = 0x0b,
    DbPlus36_1 = 0x0c,
    DbPlus39_6 = 0x0d,
    DbPlus42_1 = 0x0e,
    DbPlus45_7 = 0x0f,
    DbPlus48_2 = 0x10,
    DbPlus51_7 = 0x11,
    DbPlus54_2 = 0x12,
    DbPlus57_7 = 0x13,
    DbPlus60_2 = 0x14,
    DbPlus63_7 = 0x15,
    DbPlus66_2 = 0x16,
    DbPlus69_7 = 0x17,
    DbPlus72_2 = 0x18,
}

impl CicScale {
    pub const fn bits(self) -> u8 {
        self as u8
    }

    /// Gain in tenths of a decibel, avoiding floating point in firmware.
    pub const fn gain_tenths_db(self) -> i16 {
        match self {
            Self::DbMinus48_2 => -482,
            Self::DbMinus44_6 => -446,
            Self::DbMinus42_1 => -421,
            Self::DbMinus38_6 => -386,
            Self::DbMinus36_1 => -361,
            Self::DbMinus32_6 => -326,
            Self::DbMinus30_1 => -301,
            Self::DbMinus26_6 => -266,
            Self::DbMinus24_1 => -241,
            Self::DbMinus20_6 => -206,
            Self::DbMinus18_1 => -181,
            Self::DbMinus14_5 => -145,
            Self::DbMinus12_0 => -120,
            Self::DbMinus8_5 => -85,
            Self::DbMinus6_0 => -60,
            Self::DbMinus2_5 => -25,
            Self::Db0_0 => 0,
            Self::DbPlus3_5 => 35,
            Self::DbPlus6_0 => 60,
            Self::DbPlus9_5 => 95,
            Self::DbPlus12_0 => 120,
            Self::DbPlus15_6 => 156,
            Self::DbPlus18_1 => 181,
            Self::DbPlus21_6 => 216,
            Self::DbPlus24_1 => 241,
            Self::DbPlus27_6 => 276,
            Self::DbPlus30_1 => 301,
            Self::DbPlus33_6 => 336,
            Self::DbPlus36_1 => 361,
            Self::DbPlus39_6 => 396,
            Self::DbPlus42_1 => 421,
            Self::DbPlus45_7 => 457,
            Self::DbPlus48_2 => 482,
            Self::DbPlus51_7 => 517,
            Self::DbPlus54_2 => 542,
            Self::DbPlus57_7 => 577,
            Self::DbPlus60_2 => 602,
            Self::DbPlus63_7 => 637,
            Self::DbPlus66_2 => 662,
            Self::DbPlus69_7 => 697,
            Self::DbPlus72_2 => 722,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ReshapeFilter {
    Bypass,
    /// RSFLTD=0: decimation by four.
    DecimateBy4,
}

impl ReshapeFilter {
    pub const fn decimation(self) -> u16 {
        match self {
            Self::Bypass => 1,
            Self::DecimateBy4 => 4,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Decimation {
    Auto,
    Ratio(u16),
}

/// CIC output width for a one-bit PDM input, rounded up to whole bits.
/// Implements `ceil(N * log2(D)) + DS_IN` from RM0456 with `DS_IN = 1`.
pub const fn cic_output_bits(filter: SincFilter, decimation: u16) -> u8 {
    if decimation == 0 {
        return 0;
    }
    let mut gain = 1u64;
    let mut order = 0;
    while order < filter.order() {
        gain *= decimation as u64;
        order += 1;
    }
    let ceil_log2 = if gain <= 1 {
        0
    } else {
        (u64::BITS - (gain - 1).leading_zeros()) as u8
    };
    ceil_log2 + 1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MicrophoneConfig {
    pub mode: MicrophoneMode,
    pub sample_rate: SampleRate,
    pub bit_depth: BitDepth,
    pub sample_packing: SamplePacking,
    pub high_pass_filter: bool,
    pub sinc_filter: SincFilter,
    pub decimation: Decimation,
    pub cic_scale: CicScale,
    pub reshape_filter: ReshapeFilter,
}

/// Selectable, internally consistent examples derived from RM0456 table 384.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MicrophonePreset {
    Table384Config1_8Khz,
    Table384Config2_16Khz,
    Table384Config3_8Khz,
    Table384Config7_16Khz,
    Table384Config8_16Khz,
}

impl MicrophoneConfig {
    pub const fn from_preset(preset: MicrophonePreset) -> Self {
        let (sample_rate, sinc_filter, decimation, cic_scale, reshape_filter) = match preset {
            MicrophonePreset::Table384Config1_8Khz => (
                SampleRate::Hz8000,
                SincFilter::Sinc4,
                Decimation::Ratio(64),
                CicScale::DbMinus8_5,
                ReshapeFilter::Bypass,
            ),
            MicrophonePreset::Table384Config2_16Khz => (
                SampleRate::Hz16000,
                SincFilter::Sinc5,
                Decimation::Ratio(32),
                CicScale::DbMinus14_5,
                ReshapeFilter::Bypass,
            ),
            MicrophonePreset::Table384Config3_8Khz => (
                SampleRate::Hz8000,
                SincFilter::Sinc5,
                Decimation::Ratio(16),
                CicScale::DbPlus3_5,
                ReshapeFilter::DecimateBy4,
            ),
            MicrophonePreset::Table384Config7_16Khz => (
                SampleRate::Hz16000,
                SincFilter::Sinc5,
                Decimation::Ratio(24),
                CicScale::DbMinus12_0,
                ReshapeFilter::DecimateBy4,
            ),
            MicrophonePreset::Table384Config8_16Khz => (
                SampleRate::Hz16000,
                SincFilter::Sinc5,
                Decimation::Ratio(32),
                CicScale::DbMinus26_6,
                ReshapeFilter::DecimateBy4,
            ),
        };
        Self {
            mode: MicrophoneMode::Hexaphonic,
            sample_rate,
            bit_depth: BitDepth::Bits24,
            sample_packing: SamplePacking::Bits32,
            high_pass_filter: true,
            sinc_filter,
            decimation,
            cic_scale,
            reshape_filter,
        }
    }
}

impl Default for MicrophoneConfig {
    fn default() -> Self {
        // Config #7 is the closest Table 384 16 kHz setting obtainable from
        // the board's 80 MHz MDF kernel clock (about 0.16% clock error).
        Self::from_preset(MicrophonePreset::Table384Config7_16Khz)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    InvalidBufferSize,
    InvalidDecimation,
    CicOutputTooWide,
    InvalidSamplePacking,
    MicrophoneClockOutOfRange,
    Dma,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CaptureState {
    pub running: bool,
    pub sequence: u64,
    pub half: u8,
    /// Embassy system-time ticks immediately before the MDF filters were enabled.
    pub started_at_ticks: u64,
    /// Embassy system-time ticks observed when the final DMA channel completed.
    pub completed_at_ticks: u64,
    pub channel_count: u8,
    pub error: Option<Error>,
}

/// Static DMA buffers and publication state shared by driver and consumers.
pub struct MicrophoneResources<const BUFFER: usize, const WATCHERS: usize = DEFAULT_WATCHERS> {
    buffers: UnsafeCell<[[u32; BUFFER]; MAX_MICROPHONES]>,
    sync: UnsafeCell<[u32; BUFFER]>,
    state: Watch<CriticalSectionRawMutex, CaptureState, WATCHERS>,
}

// DMA and consumers intentionally share this storage. Volatile reads and the
// DMA fences in the STM32 implementation provide visibility; overwrite is an
// explicit part of the best-effort contract.
unsafe impl<const BUFFER: usize, const WATCHERS: usize> Sync
    for MicrophoneResources<BUFFER, WATCHERS>
{
}

impl<const BUFFER: usize, const WATCHERS: usize> MicrophoneResources<BUFFER, WATCHERS> {
    pub const fn new() -> Self {
        Self {
            buffers: UnsafeCell::new([[0; BUFFER]; MAX_MICROPHONES]),
            sync: UnsafeCell::new([0; BUFFER]),
            state: Watch::new_with(CaptureState {
                running: false,
                sequence: 0,
                half: 0,
                started_at_ticks: 0,
                completed_at_ticks: 0,
                channel_count: 0,
                error: None,
            }),
        }
    }

    pub fn frame_receiver(&self) -> Option<FrameReceiver<'_, WATCHERS>> {
        self.state.receiver()
    }

    pub fn state(&self) -> CaptureState {
        self.state.try_get().unwrap_or_default()
    }

    /// Returns channel-major views of the half described by `state`.
    ///
    /// The slices are zero-copy and may be overwritten two frame periods later.
    pub fn frame(&self, state: CaptureState) -> MicrophoneFrame<'_, BUFFER> {
        let half = BUFFER / 2;
        let offset = usize::from(state.half) * half;
        let buffers = unsafe { &*self.buffers.get() };
        let channels = core::array::from_fn(|channel| &buffers[channel][offset..offset + half]);
        MicrophoneFrame { state, channels }
    }
}

impl<const BUFFER: usize, const WATCHERS: usize> Default for MicrophoneResources<BUFFER, WATCHERS> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MicrophoneFrame<'a, const BUFFER: usize> {
    pub state: CaptureState,
    pub channels: [&'a [u32]; MAX_MICROPHONES],
}

impl<const BUFFER: usize> MicrophoneFrame<'_, BUFFER> {
    pub fn active_channels(&self) -> &[&[u32]] {
        &self.channels[..usize::from(self.state.channel_count)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_the_adr() {
        let config = MicrophoneConfig::default();
        assert_eq!(config.sample_rate, SampleRate::Hz16000);
        assert_eq!(config.bit_depth, BitDepth::Bits24);
        assert_eq!(config.sample_packing, SamplePacking::Bits32);
        assert_eq!(config.sinc_filter, SincFilter::Sinc5);
        assert_eq!(config.cic_scale, CicScale::DbMinus12_0);
        assert_eq!(config.reshape_filter, ReshapeFilter::DecimateBy4);
        assert!(config.high_pass_filter);
    }

    #[test]
    fn scale_table_has_expected_reference_entries() {
        assert_eq!(CicScale::DbMinus26_6.bits(), 0x27);
        assert_eq!(CicScale::DbMinus26_6.gain_tenths_db(), -266);
        assert_eq!(CicScale::Db0_0.bits(), 0x00);
        assert_eq!(CicScale::DbPlus72_2.bits(), 0x18);
    }

    #[test]
    fn cic_width_matches_table_376_boundaries() {
        assert_eq!(cic_output_bits(SincFilter::Sinc4, 76), 26);
        assert_eq!(cic_output_bits(SincFilter::Sinc4, 77), 27);
        assert_eq!(cic_output_bits(SincFilter::Sinc5, 32), 26);
        assert_eq!(cic_output_bits(SincFilter::Sinc5, 33), 27);
    }
}
