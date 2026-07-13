use super::{StorageDriver, StorageError, StorageResult};
use exfat_slim::asynchronous::BlockDevice;

impl<D, const SIZE: usize, const CACHE: usize, const PATH_LEN: usize>
    StorageDriver<D, SIZE, CACHE, PATH_LEN>
where
    D: BlockDevice<SIZE>,
{
    pub async fn mount(&mut self) -> StorageResult<(), D::Error> {
        self.fs.mount().await.map_err(StorageError::from)
    }
}
