use raylar_audiosource::{AudioSource, Error as AudioSourceError, Reader, ReaderStart};
use raylar_storage_service::StorageLayout;
use raylar_time_service::UtcTimestamp;
use embassy_time::{Duration, with_timeout};

use crate::wav::WavError;
use crate::{MetadataSource, RecordingStorage, WavContainer};

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
    Storage(E),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RecorderProgress {
    pub pcm_samples: usize,
    pub dropped_samples: u64,
    pub rotated: bool,
}

pub struct AudioRecorder<
    'a,
    S: RecordingStorage,
    M,
    const AUDIO_CAPACITY: usize,
    const READERS: usize,
    const ENCODE_BYTES: usize = DEFAULT_ENCODE_BUFFER_BYTES,
> {
    reader: Reader<'a, AUDIO_CAPACITY, READERS>,
    storage: S,
    metadata: M,
    container: WavContainer,
    storage_layout: StorageLayout,
    stream: Option<S::Handle>,
    sample_rate_hz: u32,
    frame_samples: usize,
    regular_samples_per_recording: usize,
    samples_per_recording: usize,
    samples_in_recording: usize,
    next_recording_started_utc: Option<UtcTimestamp>,
    encoded: [u8; ENCODE_BYTES],
}

impl<'a, S, M, const AUDIO_CAPACITY: usize, const READERS: usize, const ENCODE_BYTES: usize>
    AudioRecorder<'a, S, M, AUDIO_CAPACITY, READERS, ENCODE_BYTES>
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
            || !ENCODE_BYTES.is_multiple_of(frame_bytes)
            || samples_per_recording == 0
        {
            return Err(AudioRecorderError::InvalidConfig);
        }
        let container =
            WavContainer::new(format, config.recording_seconds).map_err(|error| match error {
                WavError::InvalidFormat | WavError::DataSizeOverflow => {
                    AudioRecorderError::InvalidConfig
                }
            })?;
        let reader = source
            .register(ReaderStart::Latest)
            .map_err(AudioRecorderError::AudioSource)?;
        Ok(Self {
            reader,
            storage,
            metadata,
            container,
            storage_layout: config.storage_layout,
            stream: None,
            sample_rate_hz: format.sample_rate_hz,
            frame_samples,
            regular_samples_per_recording: samples_per_recording,
            samples_per_recording,
            samples_in_recording: 0,
            next_recording_started_utc: None,
            encoded: [0; ENCODE_BYTES],
        })
    }

    pub async fn start(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        if self.stream.is_some() {
            return Ok(());
        }
        // Establish the audio cursor immediately before taking the UTC
        // snapshot in `begin_recording`. Samples produced while the storage
        // stream is opened and its header is written remain in the source's
        // retention buffer, preserving the planned UTC boundary.
        self.reader.seek_to_latest();
        self.begin_recording().await?;
        Ok(())
    }

    pub async fn start_at(&mut self, utc: UtcTimestamp) -> Result<(), AudioRecorderError<S::Error>> {
        if self.stream.is_some() { return Ok(()); }
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
    pub async fn record_next_with_source_timeout(&mut self, timeout: Duration) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        self.record_next_inner(Some(timeout)).await
    }

    async fn record_next_inner(&mut self, timeout: Option<Duration>) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        let stream = self.stream.ok_or(AudioRecorderError::NotStarted)?;
        let sample_capacity = ENCODE_BYTES / 4;
        let remaining = self.samples_per_recording - self.samples_in_recording;
        let requested = sample_capacity.min(remaining);
        if let Some(timeout) = timeout {
            with_timeout(timeout, self.reader.wait_for_samples(requested)).await
                .map_err(|_| AudioRecorderError::SourceTimeout)?
                .map_err(AudioRecorderError::AudioSource)?;
        } else {
            self.reader.wait_for_samples(requested).await.map_err(AudioRecorderError::AudioSource)?;
        }

        let encoded = &mut self.encoded;
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
            .append_audio(stream, &self.encoded[..status.consumed_samples * 4])
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
        })
    }

    pub async fn stop(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        if self.stream.is_some() {
            self.finish_recording().await?;
        }
        // A later start is a new recording set whose first file may begin at
        // its then-current (unaligned) UTC time.
        self.next_recording_started_utc = None;
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
                    .checked_add(self.container.declared_file_seconds().into())
                    .and_then(|seconds| UtcTimestamp::new(seconds, 0))
                    .ok_or(AudioRecorderError::InvalidConfig)?;
                (self.regular_samples_per_recording, next)
            } else {
                first_recording_plan(
                    metadata.started_utc,
                    self.container.declared_file_seconds(),
                    self.sample_rate_hz,
                    self.frame_samples,
                )
                .ok_or(AudioRecorderError::InvalidConfig)?
            };
        metadata.started_system_ticks = self.metadata.system_ticks_at(metadata.started_utc).or(metadata.started_system_ticks);
        let header = self
            .container
            .header_for_samples(&metadata, samples_per_recording)
            .map_err(|_| AudioRecorderError::InvalidConfig)?;
        let stream = self
            .storage
            .begin_audio_stream(self.storage_layout, metadata.started_utc)
            .await
            .map_err(AudioRecorderError::Storage)?;
        if let Err(error) = self.storage.append_audio(stream, &header).await {
            let _ = self.storage.finish_audio(stream).await;
            return Err(AudioRecorderError::Storage(error));
        }
        self.stream = Some(stream);
        self.samples_per_recording = samples_per_recording;
        self.samples_in_recording = 0;
        self.next_recording_started_utc = Some(next_recording_started_utc);
        Ok(())
    }

    async fn finish_recording(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        let stream = self.stream.take().ok_or(AudioRecorderError::NotStarted)?;
        self.storage
            .finalize_audio(stream, self.samples_in_recording)
            .await
            .map_err(AudioRecorderError::Storage)
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
mod tests {
    use std::sync::Arc;
    use std::task::{Context, Poll, Wake, Waker};
    use std::vec::Vec;

    use raylar_audiosource::{AudioFormat, AudioSource};
    use raylar_time_service::UtcTimestamp;

    use super::*;
    use crate::RecordingMetadata;

    struct FixedMetadata;

    impl MetadataSource for FixedMetadata {
        fn snapshot(&self) -> Option<RecordingMetadata> {
            Some(RecordingMetadata::new(
                UtcTimestamp::new(1_700_000_000, 0).unwrap(),
            ))
        }
    }

    #[derive(Default)]
    struct MockStorage {
        writes: Vec<Vec<u8>>,
        layouts: Vec<StorageLayout>,
        starts: Vec<UtcTimestamp>,
        finishes: usize,
        next_handle: u8,
    }

    impl RecordingStorage for MockStorage {
        type Error = ();
        type Handle = u8;

        async fn begin_audio_stream(
            &mut self,
            layout: StorageLayout,
            started_utc: UtcTimestamp,
        ) -> Result<Self::Handle, Self::Error> {
            self.layouts.push(layout);
            self.starts.push(started_utc);
            self.next_handle += 1;
            Ok(self.next_handle)
        }

        async fn append_audio(
            &mut self,
            _stream: Self::Handle,
            bytes: &[u8],
        ) -> Result<(), Self::Error> {
            self.writes.push(bytes.to_vec());
            Ok(())
        }

        async fn finish_audio(&mut self, _stream: Self::Handle) -> Result<(), Self::Error> {
            self.finishes += 1;
            Ok(())
        }
    }

    struct NoopWake;

    impl Wake for NoopWake {
        fn wake(self: Arc<Self>) {}
    }

    fn block_on<F: core::future::Future>(future: F) -> F::Output {
        let waker = Waker::from(Arc::new(NoopWake));
        let mut context = Context::from_waker(&waker);
        let mut future = core::pin::pin!(future);
        loop {
            match future.as_mut().poll(&mut context) {
                Poll::Ready(output) => return output,
                Poll::Pending => std::thread::yield_now(),
            }
        }
    }

    #[test]
    fn recorder_owns_rotation_at_an_exact_sample_boundary() {
        let source = AudioSource::<16, 1>::new(AudioFormat::new(4, 1, 1_000_000));
        let config = AudioRecorderConfig {
            recording_seconds: 1,
            storage_layout: StorageLayout::DailyFolders,
        };
        let mut recorder = AudioRecorder::<_, _, 16, 1, 16>::new(
            &source,
            MockStorage::default(),
            FixedMetadata,
            config,
        )
        .unwrap();

        source.write(&[99, 99], 0).unwrap();
        block_on(recorder.start()).unwrap();
        source.write(&[1, -2, 3, -4], 250).unwrap();
        let progress = block_on(recorder.record_next()).unwrap();

        assert_eq!(progress.pcm_samples, 4);
        assert!(progress.rotated);
        assert_eq!(recorder.storage().finishes, 1);
        assert_eq!(recorder.storage().layouts, [StorageLayout::DailyFolders; 2]);
        assert_eq!(&recorder.storage().writes[0][..4], b"RIFF");
        assert_eq!(
            recorder.storage().writes[1],
            [1i32, -2, 3, -4]
                .into_iter()
                .flat_map(i32::to_le_bytes)
                .collect::<Vec<_>>()
        );
        assert_eq!(&recorder.storage().writes[2][..4], b"RIFF");

        block_on(recorder.stop()).unwrap();
        assert_eq!(recorder.storage().finishes, 2);
    }

    #[test]
    fn read_is_split_instead_of_crossing_the_recording_boundary() {
        let source = AudioSource::<16, 1>::new(AudioFormat::new(2, 1, 1_000_000));
        let config = AudioRecorderConfig {
            recording_seconds: 1,
            storage_layout: StorageLayout::Flat,
        };
        let mut recorder = AudioRecorder::<_, _, 16, 1, 16>::new(
            &source,
            MockStorage::default(),
            FixedMetadata,
            config,
        )
        .unwrap();

        block_on(recorder.start()).unwrap();
        source.write(&[1, 2, 3, 4], 0).unwrap();
        let first = block_on(recorder.record_next()).unwrap();
        let second = block_on(recorder.record_next()).unwrap();

        assert_eq!(first.pcm_samples, 2);
        assert_eq!(second.pcm_samples, 2);
        assert!(first.rotated);
        assert!(second.rotated);
    }

    struct PartialMinuteMetadata;

    impl MetadataSource for PartialMinuteMetadata {
        fn snapshot(&self) -> Option<RecordingMetadata> {
            Some(RecordingMetadata::new(
                UtcTimestamp::new(1_700_000_038, 500_000).unwrap(),
            ))
        }
    }

    #[test]
    fn first_recording_is_shortened_then_successors_start_on_the_boundary() {
        let source = AudioSource::<128, 1>::new(AudioFormat::new(4, 1, 1_000_000));
        let config = AudioRecorderConfig {
            recording_seconds: 60,
            storage_layout: StorageLayout::HourlyFolders,
        };
        let mut recorder = AudioRecorder::<_, _, 128, 1, 64>::new(
            &source,
            MockStorage::default(),
            PartialMinuteMetadata,
            config,
        )
        .unwrap();

        block_on(recorder.start()).unwrap();
        // 1_700_000_040 is the next multiple of 60: 1.5 seconds, or six frames.
        source.write(&[1, 2, 3, 4, 5, 6], 0).unwrap();
        let progress = block_on(recorder.record_next()).unwrap();

        assert!(progress.rotated);
        assert_eq!(recorder.storage().starts.len(), 2);
        assert_eq!(recorder.storage().starts[0].seconds, 1_700_000_038);
        assert_eq!(recorder.storage().starts[0].microseconds, 500_000);
        assert_eq!(recorder.storage().starts[1].seconds, 1_700_000_040);
        assert_eq!(recorder.storage().starts[1].microseconds, 0);
        assert_eq!(recorder.storage().starts[1].seconds % 60, 0);
        assert_eq!(
            u32::from_le_bytes(recorder.storage().writes[0][508..512].try_into().unwrap()),
            24
        );
        assert_eq!(
            u32::from_le_bytes(recorder.storage().writes[2][508..512].try_into().unwrap()),
            960
        );
    }
}
