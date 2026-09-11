use raylar_storage_service::{
    StorageBackend, StorageLayout, StorageService, StorageServiceError, StreamHandle, StreamKind,
    UtcClock,
};

#[allow(async_fn_in_trait)]
pub trait LogSink {
    type Error;

    async fn append(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    async fn flush(&mut self) -> Result<(), Self::Error>;
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, PartialEq, Eq)]
pub enum StorageLogSinkError<E> {
    Storage(StorageServiceError<E>),
}

pub struct StorageLogSink<'a, B, C, const BLOCK_SIZE: usize, const MAX_STREAMS: usize> {
    storage: &'a mut StorageService<B, C, BLOCK_SIZE, MAX_STREAMS>,
    stream: StreamHandle,
}

impl<'a, B, C, const BLOCK_SIZE: usize, const MAX_STREAMS: usize>
    StorageLogSink<'a, B, C, BLOCK_SIZE, MAX_STREAMS>
where
    B: StorageBackend<BLOCK_SIZE>,
    C: UtcClock,
{
    pub async fn open(
        storage: &'a mut StorageService<B, C, BLOCK_SIZE, MAX_STREAMS>,
    ) -> Result<Self, StorageLogSinkError<B::Error>> {
        let stream = storage
            .begin_stream(StreamKind::Log, StorageLayout::Flat)
            .await
            .map_err(StorageLogSinkError::Storage)?;
        Ok(Self { storage, stream })
    }

    pub fn storage_mut(&mut self) -> &mut StorageService<B, C, BLOCK_SIZE, MAX_STREAMS> {
        self.storage
    }

    pub async fn close(self) -> Result<(), StorageLogSinkError<B::Error>> {
        self.storage
            .finish(self.stream)
            .await
            .map_err(StorageLogSinkError::Storage)
    }
}

impl<B, C, const BLOCK_SIZE: usize, const MAX_STREAMS: usize> LogSink
    for StorageLogSink<'_, B, C, BLOCK_SIZE, MAX_STREAMS>
where
    B: StorageBackend<BLOCK_SIZE>,
    C: UtcClock,
{
    type Error = StorageLogSinkError<B::Error>;

    async fn append(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        self.storage
            .write(self.stream, data)
            .await
            .map_err(StorageLogSinkError::Storage)
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.storage
            .flush(self.stream)
            .await
            .map_err(StorageLogSinkError::Storage)
    }
}
