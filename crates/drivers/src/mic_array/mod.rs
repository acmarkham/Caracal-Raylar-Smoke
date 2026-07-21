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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Decimation {
    Auto,
    Ratio(u16),
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
}

impl Default for MicrophoneConfig {
    fn default() -> Self {
        Self {
            mode: MicrophoneMode::Hexaphonic,
            sample_rate: SampleRate::Hz16000,
            bit_depth: BitDepth::Bits24,
            sample_packing: SamplePacking::Bits32,
            high_pass_filter: true,
            sinc_filter: SincFilter::Sinc5,
            decimation: Decimation::Auto,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    InvalidBufferSize,
    InvalidDecimation,
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
        assert!(config.high_pass_filter);
    }
}
