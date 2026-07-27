use raylar_audiosource::{AudioSource, Error as AudioSourceError, Reader, ReaderStart};
use raylar_storage_service::{AppendOutcome, StreamLifecycleEvent};

use crate::wav::WavError;
use crate::{MetadataSource, RecordingStorage, WavContainer};

/// Amortises SD/filesystem call latency while reserving only 256 ms of mono
/// 16 kHz, 32-bit PCM.
pub const DEFAULT_ENCODE_BUFFER_BYTES: usize = 16 * 1_024;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AudioRecorderConfig {
    /// Nominal PCM duration advertised in each fixed WAV header.
    pub declared_file_seconds: u32,
}

impl Default for AudioRecorderConfig {
    fn default() -> Self {
        Self {
            declared_file_seconds: 60,
        }
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
        rotate_requested: bool,
        reject_next_append: bool,
        rotations: usize,
        closed: bool,
    }

    impl RecordingStorage for MockStorage {
        type Error = ();
        type Handle = u8;

        async fn create_audio_stream(&mut self) -> Result<Self::Handle, Self::Error> {
            Ok(7)
        }

        fn lifecycle_event(
            &self,
            _stream: Self::Handle,
        ) -> Result<Option<StreamLifecycleEvent>, Self::Error> {
            Ok(self
                .rotate_requested
                .then_some(StreamLifecycleEvent::RotateRequested))
        }

        async fn append_audio(
            &mut self,
            _stream: Self::Handle,
            bytes: &[u8],
        ) -> Result<AppendOutcome, Self::Error> {
            if self.reject_next_append {
                self.reject_next_append = false;
                return Ok(AppendOutcome::RotationRequired);
            }
            self.writes.push(bytes.to_vec());
            Ok(AppendOutcome::Written)
        }

        async fn rotate_audio(&mut self, _stream: Self::Handle) -> Result<bool, Self::Error> {
            self.rotate_requested = false;
            self.rotations += 1;
            Ok(true)
        }

        async fn close_audio(&mut self, _stream: Self::Handle) -> Result<(), Self::Error> {
            self.closed = true;
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
    fn records_pcm_and_starts_a_new_wav_after_rotation() {
        let source = AudioSource::<16, 1>::new(AudioFormat::new(16_000, 1, 1_000_000));
        let mut recorder = AudioRecorder::<_, _, 16, 1, 16>::new(
            &source,
            MockStorage::default(),
            FixedMetadata,
            AudioRecorderConfig::default(),
        )
        .unwrap();

        source.write(&[99, 99, 99, 99], 0).unwrap();
        block_on(recorder.start()).unwrap();
        source.write(&[1, -2, 3, -4], 250).unwrap();
        let progress = block_on(recorder.record_next()).unwrap();
        assert_eq!(progress.pcm_samples, 4);
        assert_eq!(recorder.storage().writes.len(), 2);
        assert_eq!(&recorder.storage().writes[0][..4], b"RIFF");
        assert_eq!(
            recorder.storage().writes[1],
            [1i32, -2, 3, -4]
                .into_iter()
                .flat_map(i32::to_le_bytes)
                .collect::<Vec<_>>()
        );

        recorder.storage_mut().rotate_requested = true;
        source.write(&[5, 6, 7, 8], 250).unwrap();
        let progress = block_on(recorder.record_next()).unwrap();
        assert!(progress.rotated);
        assert_eq!(recorder.storage().rotations, 1);
        assert_eq!(&recorder.storage().writes[2][..4], b"RIFF");
        assert_eq!(recorder.storage().writes.len(), 4);

        block_on(recorder.stop()).unwrap();
        assert!(recorder.storage().closed);
    }

    #[test]
    fn startup_rotates_and_retries_if_utc_crosses_a_boundary() {
        let source = AudioSource::<16, 1>::new(AudioFormat::new(16_000, 1, 1_000_000));
        let storage = MockStorage {
            reject_next_append: true,
            ..MockStorage::default()
        };
        let mut recorder = AudioRecorder::<_, _, 16, 1, 16>::new(
            &source,
            storage,
            FixedMetadata,
            AudioRecorderConfig::default(),
        )
        .unwrap();

        block_on(recorder.start()).unwrap();
        assert_eq!(recorder.storage().rotations, 1);
        assert_eq!(recorder.storage().writes.len(), 1);
        assert_eq!(&recorder.storage().writes[0][..4], b"RIFF");
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
    StorageRejectedWrite,
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
    stream: Option<S::Handle>,
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
        let frame_bytes = usize::from(source.format().channels) * 4;
        if frame_bytes == 0 || ENCODE_BYTES == 0 || !ENCODE_BYTES.is_multiple_of(frame_bytes) {
            return Err(AudioRecorderError::InvalidConfig);
        }
        let container =
            WavContainer::new(source.format(), config.declared_file_seconds).map_err(|error| {
                match error {
                    WavError::InvalidFormat | WavError::DataSizeOverflow => {
                        AudioRecorderError::InvalidConfig
                    }
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
            stream: None,
            encoded: [0; ENCODE_BYTES],
        })
    }

    pub async fn start(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        if self.stream.is_some() {
            return Ok(());
        }
        let metadata = self
            .metadata
            .snapshot()
            .ok_or(AudioRecorderError::TimeUnavailable)?;
        let stream = self
            .storage
            .create_audio_stream()
            .await
            .map_err(AudioRecorderError::Storage)?;
        let header = self.container.header(&metadata);
        match self.append(stream, &header).await {
            Ok(()) => {}
            Err(AudioRecorderError::StorageRejectedWrite) => {
                // UTC may cross a storage boundary while the initial file is
                // being opened. Finalise that empty file and start the header
                // in the newly requested file.
                self.rotate(stream).await?;
            }
            Err(error) => return Err(error),
        }
        // SD setup may take seconds. The logical recording begins only after
        // its header is durable, so setup-time audio is not an overrun.
        self.reader.seek_to_latest();
        self.stream = Some(stream);
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
        let mut rotated = self.rotate_if_requested(stream).await?;
        let sample_capacity = ENCODE_BYTES / 4;
        self.reader
            .wait_for_samples(sample_capacity)
            .await
            .map_err(AudioRecorderError::AudioSource)?;

        let encoded = &mut self.encoded;
        let status = self
            .reader
            .read(sample_capacity, |chunk| {
                let mut output = 0;
                for sample in chunk.first.iter().chain(chunk.second) {
                    encoded[output..output + 4].copy_from_slice(&sample.to_le_bytes());
                    output += 4;
                }
                chunk.len()
            })
            .map_err(AudioRecorderError::AudioSource)?;

        match self
            .storage
            .append_audio(stream, &self.encoded[..status.consumed_samples * 4])
            .await
            .map_err(AudioRecorderError::Storage)?
        {
            AppendOutcome::Written => {}
            AppendOutcome::RotationRequired => {
                self.rotate(stream).await?;
                rotated = true;
                match self
                    .storage
                    .append_audio(stream, &self.encoded[..status.consumed_samples * 4])
                    .await
                    .map_err(AudioRecorderError::Storage)?
                {
                    AppendOutcome::Written => {}
                    AppendOutcome::DroppedUtcUnavailable => {
                        return Err(AudioRecorderError::TimeUnavailable);
                    }
                    AppendOutcome::RotationRequired => {
                        return Err(AudioRecorderError::StorageRejectedWrite);
                    }
                }
            }
            AppendOutcome::DroppedUtcUnavailable => {
                return Err(AudioRecorderError::TimeUnavailable);
            }
        }

        Ok(RecorderProgress {
            pcm_samples: status.consumed_samples,
            dropped_samples: status.dropped_since_last_read,
            rotated,
        })
    }

    pub async fn stop(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        if let Some(stream) = self.stream.take() {
            self.storage
                .close_audio(stream)
                .await
                .map_err(AudioRecorderError::Storage)?;
        }
        Ok(())
    }

    pub async fn run(&mut self) -> Result<(), AudioRecorderError<S::Error>> {
        self.start().await?;
        loop {
            self.record_next().await?;
        }
    }

    async fn rotate_if_requested(
        &mut self,
        stream: S::Handle,
    ) -> Result<bool, AudioRecorderError<S::Error>> {
        match self
            .storage
            .lifecycle_event(stream)
            .map_err(AudioRecorderError::Storage)?
        {
            Some(StreamLifecycleEvent::RotateRequested) => {
                self.rotate(stream).await?;
                Ok(true)
            }
            None => Ok(false),
        }
    }

    async fn rotate(&mut self, stream: S::Handle) -> Result<(), AudioRecorderError<S::Error>> {
        let metadata = self
            .metadata
            .snapshot()
            .ok_or(AudioRecorderError::TimeUnavailable)?;
        let rotated = self
            .storage
            .rotate_audio(stream)
            .await
            .map_err(AudioRecorderError::Storage)?;
        if !rotated {
            return Err(AudioRecorderError::StorageRejectedWrite);
        }
        let header = self.container.header(&metadata);
        self.append(stream, &header).await
    }

    async fn append(
        &mut self,
        stream: S::Handle,
        bytes: &[u8],
    ) -> Result<(), AudioRecorderError<S::Error>> {
        match self
            .storage
            .append_audio(stream, bytes)
            .await
            .map_err(AudioRecorderError::Storage)?
        {
            AppendOutcome::Written => Ok(()),
            AppendOutcome::DroppedUtcUnavailable => Err(AudioRecorderError::TimeUnavailable),
            AppendOutcome::RotationRequired => Err(AudioRecorderError::StorageRejectedWrite),
        }
    }
}
