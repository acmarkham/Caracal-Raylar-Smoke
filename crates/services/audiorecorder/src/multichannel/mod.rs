//! Paired recording lives alongside the existing mono recorder, under the same service.
mod lifecycle;
mod reference;
mod stream;

use crate::{
    AudioRecorderConfig, AudioRecorderError, MetadataSource, RecorderProgress, RecordingStorage,
};
use embassy_time::Duration;
use raylar_audiosource::{AudioSource, Reader, ReaderStart};
use raylar_crcl_encoder_service::{Config, Encoder, Sha256};
use raylar_flac_encoder_service::{Config as FlacConfig, Encoder as FlacEncoder};
use raylar_time_service::UtcTimestamp;

#[derive(Clone, Copy, Debug)]
pub enum ReferenceFormat {
    Wav,
    Flac(FlacConfig),
}

#[derive(Clone, Copy, Debug)]
pub struct CaptureParameters {
    pub pdm_clock_hz: u32,
    pub sinc_order: u8,
    pub decimation: u16,
    pub cic_scale: u8,
    pub hpf_enabled: bool,
    pub pcm_shift: i8,
    pub gain_tenths_db: i16,
    pub origin_ticks: Option<fn() -> u64>,
}

#[derive(Clone, Copy, Debug)]
pub struct HexaphonicConfig {
    pub recorder: AudioRecorderConfig,
    pub spatial: Config,
    pub reference: ReferenceFormat,
    pub capture: CaptureParameters,
    pub calibration_id: Option<&'static str>,
}

/// All sample and output buffers are supplied once by the caller. Quantization
/// overwrites the five non-reference channels in `pcm`, without sample scratch.
pub struct HexaphonicBuffers<'a> {
    pub pcm: &'a mut [i32],
    pub spatial_output: &'a mut [u8],
    pub reference_pcm: &'a mut [i32],
    pub reference_output: &'a mut [u8],
}

pub struct HexaphonicRecorder<
    'a,
    S: RecordingStorage,
    M,
    const CAPACITY: usize,
    const READERS: usize,
> {
    reader: Reader<'a, CAPACITY, READERS>,
    storage: S,
    metadata: M,
    config: HexaphonicConfig,
    buffers: HexaphonicBuffers<'a>,
    spatial: Encoder,
    flac: Option<FlacEncoder>,
    rate: u32,
    tick_hz: u32,
    started_utc: Option<UtcTimestamp>,
    seeking_start: bool,
    reference_handle: Option<S::Handle>,
    spatial_handle: Option<S::Handle>,
    frames_target: usize,
    frames_accepted: usize,
    pcm_frames: usize,
    reference_len: usize,
    reference_frames: usize,
    reference_bytes: u64,
    hash: Sha256,
    expected_position: Option<u64>,
    expected_ticks: Option<u64>,
    capture_origin: u64,
    first_ticks: u64,
    last_checkpoint_second: u64,
}

impl<'a, S: RecordingStorage, M: MetadataSource, const C: usize, const R: usize>
    HexaphonicRecorder<'a, S, M, C, R>
{
    pub fn new(
        source: &'a AudioSource<C, R>,
        storage: S,
        metadata: M,
        config: HexaphonicConfig,
        buffers: HexaphonicBuffers<'a>,
    ) -> Result<Self, AudioRecorderError<S::Error>> {
        let format = source.format();
        config
            .spatial
            .validate()
            .map_err(AudioRecorderError::Spatial)?;
        let max_frames = usize::try_from(format.sample_rate_hz)
            .ok()
            .and_then(|rate| rate.checked_mul(config.recorder.recording_seconds as usize + 1))
            .ok_or(AudioRecorderError::InvalidConfig)?;
        if format.channels != 6
            || format.sample_rate_hz == 0
            || format.timestamp_hz == 0
            || !C.is_multiple_of(6)
            || config.recorder.recording_seconds == 0
            || buffers.pcm.len() < config.spatial.block_frames * 6
            || buffers.spatial_output.len() < config.spatial.workspace_capacity()
            || buffers.reference_output.len() < 1024
            || max_frames > u32::MAX as usize / 4
        {
            return Err(AudioRecorderError::InvalidConfig);
        }
        let flac = match config.reference {
            ReferenceFormat::Wav => None,
            ReferenceFormat::Flac(c) => {
                if c.sample_rate_hz != format.sample_rate_hz
                    || c.bits_per_sample != 32
                    || buffers.reference_pcm.len() < c.block_samples
                    || buffers.reference_output.len()
                        < raylar_flac_encoder_service::frame_capacity(c.block_samples)
                {
                    return Err(AudioRecorderError::InvalidConfig);
                }
                Some(FlacEncoder::new(c).map_err(AudioRecorderError::Codec)?)
            }
        };
        let spatial = Encoder::new(config.spatial).map_err(AudioRecorderError::Spatial)?;
        let reader = source
            .register(ReaderStart::Latest)
            .map_err(AudioRecorderError::AudioSource)?;
        Ok(Self {
            reader,
            storage,
            metadata,
            config,
            buffers,
            spatial,
            flac,
            rate: format.sample_rate_hz,
            tick_hz: format.timestamp_hz,
            started_utc: None,
            seeking_start: false,
            reference_handle: None,
            spatial_handle: None,
            frames_target: 0,
            frames_accepted: 0,
            pcm_frames: 0,
            reference_len: 0,
            reference_frames: 0,
            reference_bytes: 0,
            hash: Sha256::new(),
            expected_position: None,
            expected_ticks: None,
            capture_origin: 0,
            first_ticks: 0,
            last_checkpoint_second: 0,
        })
    }
    pub fn storage(&self) -> &S {
        &self.storage
    }
    pub fn storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }
    pub async fn start_at(
        &mut self,
        utc: UtcTimestamp,
    ) -> Result<(), AudioRecorderError<S::Error>> {
        if self.started_utc.is_some() {
            return Err(AudioRecorderError::InvalidConfig);
        }
        if utc.seconds < 0 || self.metadata.snapshot().is_none() {
            return Err(AudioRecorderError::TimeUnavailable);
        }
        self.reader.seek_to_latest();
        self.reset_segment(utc);
        self.seeking_start = true;
        self.expected_position = None;
        self.expected_ticks = None;
        Ok(())
    }
    pub async fn record_next(&mut self) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        self.record(None).await
    }
    pub async fn record_next_with_source_timeout(
        &mut self,
        timeout: Duration,
    ) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        self.record(Some(timeout)).await
    }
    async fn record(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        let result = self.read_next(timeout).await;
        if let Err(error) = &result {
            if matches!(error, AudioRecorderError::Discontinuity { .. }) {
                if let Err(finalize) = self.finish(crate::crcl::Completion::Discontinuity).await {
                    self.abort().await;
                    return Err(finalize);
                }
                self.started_utc = None;
            } else if !matches!(
                error,
                AudioRecorderError::SourceTimeout | AudioRecorderError::NotStarted
            ) {
                self.abort().await;
            }
        }
        result
    }
    pub async fn stop(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        let result = self.finish(crate::crcl::Completion::EarlyStop).await;
        self.started_utc = None;
        if result.is_err() {
            self.abort().await;
        }
        result
    }
    fn reset_segment(&mut self, utc: UtcTimestamp) {
        self.started_utc = Some(utc);
        self.frames_target = 0;
        self.frames_accepted = 0;
        self.pcm_frames = 0;
        self.reference_len = 0;
        self.reference_frames = 0;
        self.reference_bytes = 0;
        self.hash = Sha256::new();
        self.spatial.reset();
        self.last_checkpoint_second = 0;
        if let Some(flac) = &mut self.flac {
            flac.reset();
        }
    }
    async fn abort(&mut self) {
        if let Some(handle) = self.reference_handle.take() {
            let _ = self.storage.finish_audio(handle).await;
        }
        if let Some(handle) = self.spatial_handle.take() {
            let _ = self.storage.finish_audio(handle).await;
        }
        self.started_utc = None;
        self.pcm_frames = 0;
        self.reference_len = 0;
    }
}

#[cfg(test)]
mod tests;
