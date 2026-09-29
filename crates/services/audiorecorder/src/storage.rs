use raylar_storage_service::{
    StorageBackend, StorageLayout, StorageService, StorageServiceError, StreamHandle, StreamKind,
    UtcClock,
};
use raylar_time_service::UtcTimestamp;

#[allow(async_fn_in_trait)]
pub trait RecordingStorage {
    type Error;
    type Handle: Copy;

    async fn begin_audio_stream(
        &mut self,
        layout: StorageLayout,
        started_utc: UtcTimestamp,
    ) -> Result<Self::Handle, Self::Error>;
    async fn append_audio(&mut self, stream: Self::Handle, bytes: &[u8])
        -> Result<(), Self::Error>;
    async fn finish_audio(&mut self, stream: Self::Handle) -> Result<(), Self::Error>;
}

impl<B, C, const BLOCK_SIZE: usize, const MAX_STREAMS: usize, const WRITE_BUFFER_BYTES: usize>
    RecordingStorage for StorageService<B, C, BLOCK_SIZE, MAX_STREAMS, WRITE_BUFFER_BYTES>
where
    B: StorageBackend<BLOCK_SIZE>,
    C: UtcClock,
{
    type Error = StorageServiceError<B::Error>;
    type Handle = StreamHandle;

    async fn begin_audio_stream(
        &mut self,
        layout: StorageLayout,
        started_utc: UtcTimestamp,
    ) -> Result<StreamHandle, Self::Error> {
        self.begin_stream_at(StreamKind::Audio, layout, started_utc)
            .await
    }

    async fn append_audio(
        &mut self,
        stream: StreamHandle,
        bytes: &[u8],
    ) -> Result<(), Self::Error> {
        self.write(stream, bytes).await
    }

    async fn finish_audio(&mut self, stream: StreamHandle) -> Result<(), Self::Error> {
        self.finish(stream).await
    }
}
