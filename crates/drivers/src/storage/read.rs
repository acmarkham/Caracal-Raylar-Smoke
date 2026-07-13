use super::directory::normalize_path;
use super::driver::{ReadSlot, StorageDriver};
use super::{ReadHandle, StorageError, StorageResult};
use exfat_slim::asynchronous::file::OpenOptions;
use exfat_slim::asynchronous::BlockDevice;

impl<D, const SIZE: usize, const CACHE: usize, const PATH_LEN: usize>
    StorageDriver<D, SIZE, CACHE, PATH_LEN>
where
    D: BlockDevice<SIZE>,
{
    pub async fn open_for_read(&mut self, path: &str) -> StorageResult<ReadHandle, D::Error> {
        let path = normalize_path::<PATH_LEN, D::Error>(path)?;
        if self.read_slot.is_some() {
            return Err(StorageError::ReadAlreadyOpen);
        }

        let options = OpenOptions::new().read(true);
        let file = self
            .fs
            .open(path.as_str(), options)
            .await
            .map_err(StorageError::from)?;
        self.read_generation = self.read_generation.wrapping_add(1);
        let handle = ReadHandle::new(self.read_generation);
        self.read_slot = Some(ReadSlot {
            file,
            generation: self.read_generation,
        });
        Ok(handle)
    }

    pub async fn read(
        &mut self,
        handle: ReadHandle,
        buf: &mut [u8],
    ) -> StorageResult<usize, D::Error> {
        self.validate_read_handle(handle)?;
        let Self { fs, read_slot, .. } = self;
        let slot = read_slot.as_mut().ok_or(StorageError::InvalidHandle)?;
        let bytes = slot
            .file
            .read(fs, buf)
            .await
            .map_err(StorageError::from)?
            .unwrap_or(0);
        Ok(bytes)
    }

    pub fn close_read(&mut self, handle: ReadHandle) -> StorageResult<(), D::Error> {
        let slot = self.read_slot.as_ref().ok_or(StorageError::InvalidHandle)?;
        if slot.generation != handle.generation() {
            return Err(StorageError::InvalidHandle);
        }
        self.read_slot = None;
        Ok(())
    }

    fn validate_read_handle(&self, handle: ReadHandle) -> StorageResult<(), D::Error> {
        let slot = self.read_slot.as_ref().ok_or(StorageError::InvalidHandle)?;
        if slot.generation != handle.generation() {
            return Err(StorageError::InvalidHandle);
        }
        Ok(())
    }
}
