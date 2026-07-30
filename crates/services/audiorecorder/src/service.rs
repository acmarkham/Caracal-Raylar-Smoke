use raylar_audiosource::{AudioSource, Error as AudioSourceError, Reader, ReaderStart};
use raylar_storage_service::StorageLayout;

use crate::wav::WavError;
use crate::{MetadataSource, RecordingStorage, WavContainer};

/// Amortises SD/filesystem call latency while reserving only 256 ms of mono
/// 16 kHz, 32-bit PCM.
pub const DEFAULT_ENCODE_BUFFER_BYTES: usize = 16 * 1_024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AudioRecorderConfig {
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
    samples_per_recording: usize,
    samples_in_recording: usize,
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
            samples_per_recording,
            samples_in_recording: 0,
            encoded: [0; ENCODE_BYTES],
        })
    }

    pub async fn start(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        if self.stream.is_some() {
            return Ok(());
        }
        self.begin_recording().await?;
        // Storage setup may take seconds. The logical recording begins only
        // after its header is durable, so setup-time audio is not an overrun.
        self.reader.seek_to_latest();
        Ok(())
    }

    pub const fn storage(&self) -> &S {
        &self.storage
    }

    pub fn storage_mut(&mut self) -> &mut S {
        &mut self.storage
    }

    pub async fn record_next(&mut self) -> Result<RecorderProgress, AudioRecorderError<S::Error>> {
        let stream = self.stream.ok_or(AudioRecorderError::NotStarted)?;
        let sample_capacity = ENCODE_BYTES / 4;
        let remaining = self.samples_per_recording - self.samples_in_recording;
        let requested = sample_capacity.min(remaining);
        self.reader
            .wait_for_samples(requested)
            .await
            .map_err(AudioRecorderError::AudioSource)?;

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
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        self.start().await?;
        loop {
            self.record_next().await?;
        }
    }

    async fn begin_recording(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        let metadata = self
            .metadata
            .snapshot()
            .ok_or(AudioRecorderError::TimeUnavailable)?;
        let stream = self
            .storage
            .begin_audio_stream(self.storage_layout)
            .await
            .map_err(AudioRecorderError::Storage)?;
        let header = self.container.header(&metadata);
        if let Err(error) = self.storage.append_audio(stream, &header).await {
            let _ = self.storage.finish_audio(stream).await;
            return Err(AudioRecorderError::Storage(error));
        }
        self.stream = Some(stream);
        self.samples_in_recording = 0;
        Ok(())
    }

    async fn finish_recording(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        let stream = self.stream.take().ok_or(AudioRecorderError::NotStarted)?;
        self.storage
            .finish_audio(stream)
            .await
            .map_err(AudioRecorderError::Storage)
    }
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
        finishes: usize,
        next_handle: u8,
    }

    impl RecordingStorage for MockStorage {
        type Error = ();
        type Handle = u8;

        async fn begin_audio_stream(
            &mut self,
            layout: StorageLayout,
        ) -> Result<Self::Handle, Self::Error> {
            self.layouts.push(layout);
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
}
