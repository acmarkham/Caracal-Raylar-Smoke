use raylar_drivers::storage::{FileHandle, StorageBlockDevice, StorageDriver, StorageError};

/// Filesystem operations required by the storage service.
///
/// `StorageDriver` is the production implementation. The trait keeps policy
/// tests independent of SDMMC hardware while retaining one owner of filesystem
/// state in production.
#[allow(async_fn_in_trait)]
pub trait StorageBackend<const BLOCK_SIZE: usize> {
    type Error;

    async fn mount(&mut self) -> Result<(), Self::Error>;
    async fn create_directory(&mut self, path: &str) -> Result<(), Self::Error>;
    async fn open_for_append(&mut self, path: &str) -> Result<FileHandle, Self::Error>;
    async fn append(&mut self, handle: FileHandle, data: &[u8]) -> Result<(), Self::Error>;
    async fn flush(&mut self, handle: FileHandle) -> Result<(), Self::Error>;
    async fn close(
        &mut self,
        handle: FileHandle,
        valid_bytes_last_block: usize,
    ) -> Result<(), Self::Error>;
}

impl<D, const BLOCK_SIZE: usize, const CACHE: usize, const PATH_LEN: usize>
    StorageBackend<BLOCK_SIZE> for StorageDriver<D, BLOCK_SIZE, CACHE, PATH_LEN>
where
    D: StorageBlockDevice<BLOCK_SIZE>,
{
    type Error = StorageError<D::Error>;

    async fn mount(&mut self) -> Result<(), Self::Error> {
        StorageDriver::mount(self).await
    }

    async fn create_directory(&mut self, path: &str) -> Result<(), Self::Error> {
        StorageDriver::create_directory(self, path).await
    }

    async fn open_for_append(&mut self, path: &str) -> Result<FileHandle, Self::Error> {
        StorageDriver::open_for_append(self, path).await
    }

    async fn append(&mut self, handle: FileHandle, data: &[u8]) -> Result<(), Self::Error> {
        StorageDriver::append(self, handle, data).await
    }

    async fn flush(&mut self, handle: FileHandle) -> Result<(), Self::Error> {
        StorageDriver::flush(self, handle).await
    }

    async fn close(
        &mut self,
        handle: FileHandle,
        valid_bytes_last_block: usize,
    ) -> Result<(), Self::Error> {
        StorageDriver::close(self, handle, valid_bytes_last_block).await
    }
}
