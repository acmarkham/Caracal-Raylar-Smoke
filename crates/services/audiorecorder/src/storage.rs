use raylar_storage_service::{
    AppendOutcome, StorageBackend, StorageService, StorageServiceError, StreamHandle,
    StreamLifecycleEvent, StreamType, UtcClock,
};

#[allow(async_fn_in_trait)]
pub trait RecordingStorage {
    type Error;
    type Handle: Copy;

    async fn create_audio_stream(&mut self) -> Result<Self::Handle, Self::Error>;
    fn lifecycle_event(
        &self,
        stream: Self::Handle,
    ) -> Result<Option<StreamLifecycleEvent>, Self::Error>;
    async fn append_audio(
        &mut self,
        stream: Self::Handle,
        bytes: &[u8],
    ) -> Result<AppendOutcome, Self::Error>;
    async fn rotate_audio(&mut self, stream: Self::Handle) -> Result<bool, Self::Error>;
    async fn close_audio(&mut self, stream: Self::Handle) -> Result<(), Self::Error>;
}

impl<B, C, const BLOCK_SIZE: usize, const MAX_STREAMS: usize> RecordingStorage
    for StorageService<B, C, BLOCK_SIZE, MAX_STREAMS>
where
    B: StorageBackend<BLOCK_SIZE>,
    C: UtcClock,
{
    type Error = StorageServiceError<B::Error>;
    type Handle = StreamHandle;

    async fn create_audio_stream(&mut self) -> Result<StreamHandle, Self::Error> {
        self.create_client_managed_stream(StreamType::Audio).await
    }

    fn lifecycle_event(
        &self,
        stream: StreamHandle,
    ) -> Result<Option<StreamLifecycleEvent>, Self::Error> {
        StorageService::lifecycle_event(self, stream)
    }

    async fn append_audio(
        &mut self,
        stream: StreamHandle,
        bytes: &[u8],
    ) -> Result<AppendOutcome, Self::Error> {
        self.append(stream, bytes).await
    }

    async fn rotate_audio(&mut self, stream: StreamHandle) -> Result<bool, Self::Error> {
        self.rotate(stream).await
    }

    async fn close_audio(&mut self, stream: StreamHandle) -> Result<(), Self::Error> {
        self.close(stream).await
    }
}
