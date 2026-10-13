mod flac;

use embassy_time::{with_timeout, Duration};
use raylar_audiosource::{AudioSource, Error as AudioSourceError, Reader, ReaderStart};
use raylar_storage_service::StorageLayout;
use raylar_time_service::UtcTimestamp;

use crate::container::{flac_comment, Container};
use crate::wav::WavError;
use crate::{BorrowedBuffers, OwnedBuffers, RecorderBuffers};
use crate::{MetadataSource, RecordingStorage, WavContainer};
use raylar_flac_encoder_service::{
    frame_capacity, Config as FlacConfig, Encoder, Error as FlacError,
};

/// Amortises SD/filesystem call latency while reserving only 256 ms of mono
/// 16 kHz, 32-bit PCM.
pub const DEFAULT_ENCODE_BUFFER_BYTES: usize = 16 * 1_024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AudioRecorderConfig {
    /// Recording duration and UTC alignment interval in seconds.
    pub recording_seconds: u32,
    pub storage_layout: StorageLayout,
}

impl Default for AudioRecorderConfig {
    fn default() -> Self {
        Self {
            recording_seconds: 3_600,
            storage_layout: StorageLayout::HourlyFolders,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum AudioRecorderError<E> {
    InvalidConfig,
    TimeUnavailable,
    NotStarted,
    AudioSource(AudioSourceError),
    SourceTimeout,
    Codec(FlacError),
    Discontinuity { dropped_samples: u64 },
    Storage(E),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RecorderProgress {
    pub pcm_samples: usize,
    pub dropped_samples: u64,
    pub rotated: bool,
    /// Wall time spent analyzing and encoding the frame, including executor yields.
    pub codec_elapsed_us: u64,
    /// Wall time for the completed frame's append, including storage lock wait.
    pub storage_elapsed_us: u64,
    /// Samples still retained for this reader after processing the packet/frame.
    pub backlog_samples: usize,
}

pub struct AudioRecorder<
    'a,
    S: RecordingStorage,
    M,
    const AUDIO_CAPACITY: usize,
    const READERS: usize,
    const ENCODE_BYTES: usize = DEFAULT_ENCODE_BUFFER_BYTES,
    const PCM_SAMPLES: usize = 0,
    W = OwnedBuffers<ENCODE_BYTES, PCM_SAMPLES>,
> {
    reader: Reader<'a, AUDIO_CAPACITY, READERS>,
    storage: S,
    metadata: M,
    container: Container,
    recording_seconds: u32,
    storage_layout: StorageLayout,
    stream: Option<S::Handle>,
    started: bool,
    pending_start: Option<UtcTimestamp>,
    pending_header_len: usize,
    sample_rate_hz: u32,
    frame_samples: usize,
    regular_samples_per_recording: usize,
    samples_per_recording: usize,
    samples_in_recording: usize,
    next_recording_started_utc: Option<UtcTimestamp>,
    buffers: W,
    pcm_len: usize,
    expected_position: Option<u64>,
    expected_timestamp: Option<u64>,
    timestamp_hz: u32,
}

impl<
        'a,
        S,
        M,
        const AUDIO_CAPACITY: usize,
        const READERS: usize,
        const ENCODE_BYTES: usize,
        const PCM_SAMPLES: usize,
    > AudioRecorder<'a, S, M, AUDIO_CAPACITY, READERS, ENCODE_BYTES, PCM_SAMPLES>
where
    S: RecordingStorage,
    M: MetadataSource,
{
    pub fn new(
        source: &'a AudioSource<AUDIO_CAPACITY, READERS>,
        storage: S,
        metadata: M,
        config: AudioRecorderConfig,
    ) -> Result<Self, AudioRecorderError<S::Error>> {
        Self::construct(source, storage, metadata, config, None, OwnedBuffers::new())
    }

    /// Explicit FLAC selection; the existing `new` constructor remains WAV.
    pub fn new_flac(
        source: &'a AudioSource<AUDIO_CAPACITY, READERS>,
        storage: S,
        metadata: M,
        config: AudioRecorderConfig,
        flac: FlacConfig,
    ) -> Result<Self, AudioRecorderError<S::Error>> {
        Self::construct(
            source,
            storage,
            metadata,
            config,
            Some(flac),
            OwnedBuffers::new(),
        )
    }

    /// Use this constructor in firmware to avoid copying block-sized arrays.
    pub fn new_flac_with_buffers(
        source: &'a AudioSource<AUDIO_CAPACITY, READERS>,
        storage: S,
        metadata: M,
        config: AudioRecorderConfig,
        flac: FlacConfig,
        encoded: &'a mut [u8; ENCODE_BYTES],
        pcm: &'a mut [i32; PCM_SAMPLES],
    ) -> Result<
        AudioRecorder<
            'a,
            S,
            M,
            AUDIO_CAPACITY,
            READERS,
            ENCODE_BYTES,
            PCM_SAMPLES,
            BorrowedBuffers<'a, ENCODE_BYTES, PCM_SAMPLES>,
        >,
        AudioRecorderError<S::Error>,
    > {
        AudioRecorder::construct(
            source,
            storage,
            metadata,
            config,
            Some(flac),
            BorrowedBuffers { encoded, pcm },
        )
    }
}

impl<
        'a,
        S,
        M,
        const AUDIO_CAPACITY: usize,
        const READERS: usize,
        const ENCODE_BYTES: usize,
        const PCM_SAMPLES: usize,
        W,
    > AudioRecorder<'a, S, M, AUDIO_CAPACITY, READERS, ENCODE_BYTES, PCM_SAMPLES, W>
where
    S: RecordingStorage,
    M: MetadataSource,
    W: RecorderBuffers<ENCODE_BYTES, PCM_SAMPLES>,
{
    fn construct(
        source: &'a AudioSource<AUDIO_CAPACITY, READERS>,
        storage: S,
        metadata: M,
        config: AudioRecorderConfig,
        flac: Option<FlacConfig>,
        buffers: W,
    ) -> Result<Self, AudioRecorderError<S::Error>> {
        let format = source.format();
        let frame_samples = usize::from(format.channels);
        let frame_bytes = frame_samples * 4;
        let samples_per_recording = usize::try_from(format.sample_rate_hz)
            .ok()
            .and_then(|rate| rate.checked_mul(frame_samples))
            .and_then(|rate| {
                usize::try_from(config.recording_seconds)
                    .ok()
                    .and_then(|seconds| rate.checked_mul(seconds))
            })
            .ok_or(AudioRecorderError::InvalidConfig)?;
        if frame_bytes == 0
            || ENCODE_BYTES == 0
            || (flac.is_none() && !ENCODE_BYTES.is_multiple_of(frame_bytes))
            || samples_per_recording == 0
        {
            return Err(AudioRecorderError::InvalidConfig);
        }
        let container = if let Some(flac) = flac {
            flac.validate().map_err(AudioRecorderError::Codec)?;
            if format.channels != 1
                || flac.sample_rate_hz != format.sample_rate_hz
                || PCM_SAMPLES < flac.block_samples
                || ENCODE_BYTES < frame_capacity(flac.block_samples)
            {
                return Err(AudioRecorderError::InvalidConfig);
            }
            Container::Flac(Encoder::new(flac).map_err(AudioRecorderError::Codec)?)
        } else {
            Container::Wav(WavContainer::new(format, config.recording_seconds).map_err(
                |error| match error {
                    WavError::InvalidFormat | WavError::DataSizeOverflow => {
                        AudioRecorderError::InvalidConfig
                    }
                },
            )?)
        };
        let reader = source
            .register(ReaderStart::Latest)
            .map_err(AudioRecorderError::AudioSource)?;
        Ok(Self {
            reader,
            storage,
            metadata,
            container,
            recording_seconds: config.recording_seconds,
            storage_layout: config.storage_layout,
            stream: None,
            started: false,
            pending_start: None,
            pending_header_len: 0,
            sample_rate_hz: format.sample_rate_hz,
            frame_samples,
            regular_samples_per_recording: samples_per_recording,
            samples_per_recording,
            samples_in_recording: 0,
            next_recording_started_utc: None,
            buffers,
            pcm_len: 0,
            expected_position: None,
            expected_timestamp: None,
            timestamp_hz: format.timestamp_hz,
        })
    }

    pub async fn start(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        if self.started {
            return Ok(());
        }
        // Establish the audio cursor immediately before taking the UTC
        // snapshot in `begin_recording`. Samples produced while the storage
        // stream is opened and its header is written remain in the source's
        // retention buffer, preserving the planned UTC boundary.
        self.reader.seek_to_latest();
        self.expected_position = None;
        self.expected_timestamp = None;
        self.begin_recording().await?;
        Ok(())
    }

    pub async fn start_at(
        &mut self,
        utc: UtcTimestamp,
    ) -> Result<(), AudioRecorderError<S::Error>> {
        if self.started {
            return Ok(());
        }
        self.next_recording_started_utc = Some(utc);
        self.start().await
    }

    pub const fn storage(&self) -> &S {
        &self.storage
    }

    pub fn storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }

    pub async fn record_next(&mut self) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        self.record_next_inner(None).await
    }

    /// Bound only the source wait. Storage writes run to completion so stopping
    /// the recorder cannot cancel a filesystem operation partway through.
    pub async fn record_next_with_source_timeout(
        &mut self,
        timeout: Duration,
    ) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        self.record_next_inner(Some(timeout)).await
    }

    async fn record_next_inner(
        &mut self,
        timeout: Option<Duration>,
    ) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        if matches!(self.container, Container::Flac(_)) {
            return self.record_flac(timeout).await;
        }
        let stream = self.stream.ok_or(AudioRecorderError::NotStarted)?;
        let sample_capacity = ENCODE_BYTES / 4;
        let remaining = self.samples_per_recording - self.samples_in_recording;
        let requested = sample_capacity.min(remaining);
        if let Some(timeout) = timeout {
            with_timeout(timeout, self.reader.wait_for_samples(requested))
                .await
                .map_err(|_| AudioRecorderError::SourceTimeout)?
                .map_err(AudioRecorderError::AudioSource)?;
        } else {
            self.reader
                .wait_for_samples(requested)
                .await
                .map_err(AudioRecorderError::AudioSource)?;
        }

        let encoded = self.buffers.encoded();
        let status = self
            .reader
            .read(requested, |chunk| {
                let mut output = 0;
                for sample in chunk.first.iter().chain(chunk.second) {
                    encoded[output..output + 4].copy_from_slice(&sample.to_le_bytes());
                    output += 4;
                }
                chunk.len()
            })
            .map_err(AudioRecorderError::AudioSource)?;

        self.storage
            .append_audio(
                stream,
                &self.buffers.encoded()[..status.consumed_samples * 4],
            )
            .await
            .map_err(AudioRecorderError::Storage)?;
        self.samples_in_recording += status.consumed_samples;

        let rotated = if self.samples_in_recording == self.samples_per_recording {
            self.finish_recording().await?;
            self.begin_recording().await?;
            true
        } else {
            false
        };

        Ok(RecorderProgress {
            pcm_samples: status.consumed_samples,
            dropped_samples: status.dropped_since_last_read,
            rotated,
            backlog_samples: self.reader.stats().available_samples,
            ..RecorderProgress::default()
        })
    }

    pub async fn stop(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        if self.stream.is_some() {
            self.finish_recording().await?;
        }
        // A later start is a new recording set whose first file may begin at
        // its then-current (unaligned) UTC time.
        self.next_recording_started_utc = None;
        self.pending_start = None;
        self.pending_header_len = 0;
        self.started = false;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        self.start().await?;
        loop {
            self.record_next().await?;
        }
    }

    async fn begin_recording(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        let mut metadata = self
            .metadata
            .snapshot()
            .ok_or(AudioRecorderError::TimeUnavailable)?;
        let (samples_per_recording, next_recording_started_utc) =
            if let Some(started_utc) = self.next_recording_started_utc {
                metadata.started_utc = started_utc;
                let next = started_utc
                    .seconds
                    .checked_add(self.recording_seconds.into())
                    .and_then(|seconds| UtcTimestamp::new(seconds, 0))
                    .ok_or(AudioRecorderError::InvalidConfig)?;
                (self.regular_samples_per_recording, next)
            } else {
                first_recording_plan(
                    metadata.started_utc,
                    self.recording_seconds,
                    self.sample_rate_hz,
                    self.frame_samples,
                )
                .ok_or(AudioRecorderError::InvalidConfig)?
            };
        metadata.started_system_ticks = self
            .metadata
            .system_ticks_at(metadata.started_utc)
            .or(metadata.started_system_ticks);
        let header_len = match &mut self.container {
            Container::Wav(wav) => {
                let header = wav
                    .header_for_samples(&metadata, samples_per_recording)
                    .map_err(|_| AudioRecorderError::InvalidConfig)?;
                // WAV tests can use a smaller payload buffer than the header.
                let stream = self
                    .storage
                    .begin_audio_stream(self.storage_layout, metadata.started_utc, "wav")
                    .await
                    .map_err(AudioRecorderError::Storage)?;
                if let Err(error) = self.storage.append_audio(stream, &header).await {
                    let _ = self.storage.finish_audio(stream).await;
                    return Err(AudioRecorderError::Storage(error));
                }
                self.stream = Some(stream);
                0
            }
            Container::Flac(encoder) => {
                encoder.reset();
                let comment = flac_comment(&metadata, encoder.config())
                    .map_err(|_| AudioRecorderError::InvalidConfig)?;
                encoder
                    .header(self.buffers.encoded(), &[comment.as_str()])
                    .map_err(AudioRecorderError::Codec)?
            }
        };
        if header_len != 0 {
            // Materialize FLAC only when the first PCM packet is consumed.
            self.pending_start = Some(metadata.started_utc);
            self.pending_header_len = header_len;
        }
        self.started = true;
        self.samples_per_recording = samples_per_recording;
        self.samples_in_recording = 0;
        self.pcm_len = 0;
        self.next_recording_started_utc = Some(next_recording_started_utc);
        Ok(())
    }

    async fn finish_recording(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        if matches!(self.container, Container::Flac(_)) && self.pcm_len != 0 {
            self.append_flac_frame().await?;
        }
        let stream = self.stream.take().ok_or(AudioRecorderError::NotStarted)?;
        let rewritten = match &self.container {
            Container::Flac(encoder) => {
                self.storage
                    .rewrite_audio(stream, 8, &encoder.streaminfo())
                    .await
            }
            Container::Wav(_) => {
                let bytes = u32::try_from(
                    self.samples_in_recording
                        .checked_mul(4)
                        .ok_or(AudioRecorderError::InvalidConfig)?,
                )
                .map_err(|_| AudioRecorderError::InvalidConfig)?;
                match self
                    .storage
                    .rewrite_audio(stream, 4, &(504 + bytes).to_le_bytes())
                    .await
                {
                    Ok(()) => {
                        self.storage
                            .rewrite_audio(stream, 508, &bytes.to_le_bytes())
                            .await
                    }
                    Err(error) => Err(error),
                }
            }
        };
        // Close even if patching failed; never report the segment as finalized.
        let closed = self.storage.finish_audio(stream).await;
        rewritten.map_err(AudioRecorderError::Storage)?;
        closed.map_err(AudioRecorderError::Storage)
    }
}

fn first_recording_plan(
    started_utc: UtcTimestamp,
    recording_seconds: u32,
    sample_rate_hz: u32,
    frame_samples: usize,
) -> Option<(usize, UtcTimestamp)> {
    let interval_us = i128::from(recording_seconds) * 1_000_000;
    let started_us =
        i128::from(started_utc.seconds) * 1_000_000 + i128::from(started_utc.microseconds);
    let offset_us = started_us.rem_euclid(interval_us);
    let duration_us = if offset_us == 0 {
        interval_us
    } else {
        interval_us - offset_us
    };
    let frames = duration_us
        .checked_mul(i128::from(sample_rate_hz))?
        .checked_add(999_999)?
        / 1_000_000;
    let interleaved_samples = usize::try_from(frames).ok()?.checked_mul(frame_samples)?;
    let next_started_us = i64::try_from(started_us.checked_add(duration_us)?).ok()?;
    Some((
        interleaved_samples,
        UtcTimestamp::from_micros(next_started_us),
    ))
}

#[cfg(test)]
mod tests;
