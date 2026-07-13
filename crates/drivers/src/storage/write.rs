use super::directory::normalize_path;
use super::driver::{StorageDriver, WriteSlot, MAX_WRITE_HANDLES};
use super::{FileHandle, StorageError, StorageResult};
use exfat_slim::asynchronous::file::OpenOptions;
use exfat_slim::asynchronous::BlockDevice;
use heapless::String;

impl<D, const SIZE: usize, const CACHE: usize, const PATH_LEN: usize>
    StorageDriver<D, SIZE, CACHE, PATH_LEN>
where
    D: BlockDevice<SIZE>,
{
    pub async fn open_for_append(&mut self, path: &str) -> StorageResult<FileHandle, D::Error> {
        let normalized_path = normalize_path::<PATH_LEN, D::Error>(path)?;

        let slot_index = self
            .write_slots
            .iter()
            .position(Option::is_none)
            .ok_or(StorageError::TooManyOpenFiles)?;

        if self
            .write_slots
            .iter()
            .flatten()
            .any(|slot| slot.path.as_str() == normalized_path.as_str())
        {
            return Err(StorageError::InvalidState);
        }

        let mut stored_path = String::new();
        stored_path
            .push_str(normalized_path.as_str())
            .map_err(|_| StorageError::InvalidPath)?;

        let options = OpenOptions::new().create(true).append(true).write(true);
        let file = self
            .fs
            .open(normalized_path.as_str(), options)
            .await
            .map_err(StorageError::from)?;
        let len = file.metadata().len();

        let generation =
            self.write_generations[slot_index].wrapping_add(1) & FileHandle::GENERATION_MASK;
        self.write_generations[slot_index] = generation;

        self.write_slots[slot_index] = Some(WriteSlot {
            file,
            path: stored_path,
            generation,
            logical_len: len,
            committed_len: len,
            last_flushed_len: len,
            pending_block: [0; SIZE],
            pending_len: 0,
            dirty: false,
        });

        Ok(FileHandle::new(slot_index, generation))
    }

    pub async fn append(&mut self, handle: FileHandle, data: &[u8]) -> StorageResult<(), D::Error> {
        if data.is_empty() {
            return Ok(());
        }
        if data.len() % SIZE != 0 {
            return Err(StorageError::InvalidBufferLength);
        }

        let index = self.validate_write_handle(handle)?;
        let Self {
            fs, write_slots, ..
        } = self;
        let slot = write_slots[index]
            .as_mut()
            .ok_or(StorageError::InvalidHandle)?;

        if slot.pending_len != 0 {
            slot.file
                .write(fs, &slot.pending_block[..slot.pending_len])
                .await
                .map_err(StorageError::from)?;
            slot.committed_len = slot.committed_len.saturating_add(slot.pending_len as u64);
            slot.pending_len = 0;
        }

        let direct_len = data.len() - SIZE;
        if direct_len != 0 {
            slot.file
                .write(fs, &data[..direct_len])
                .await
                .map_err(StorageError::from)?;
            slot.committed_len = slot.committed_len.saturating_add(direct_len as u64);
        }

        slot.pending_block.copy_from_slice(&data[direct_len..]);
        slot.pending_len = SIZE;
        slot.logical_len = slot.logical_len.saturating_add(data.len() as u64);
        slot.dirty = true;
        Ok(())
    }

    pub async fn flush(&mut self, handle: FileHandle) -> StorageResult<(), D::Error> {
        let index = self.validate_write_handle(handle)?;
        let Self {
            fs, write_slots, ..
        } = self;
        let slot = write_slots[index]
            .as_mut()
            .ok_or(StorageError::InvalidHandle)?;

        if slot.pending_len != 0 {
            slot.file
                .write(fs, &slot.pending_block[..slot.pending_len])
                .await
                .map_err(StorageError::from)?;
            slot.committed_len = slot.committed_len.saturating_add(slot.pending_len as u64);
            slot.pending_len = 0;
        }

        slot.file.flush(fs).await.map_err(StorageError::from)?;
        slot.last_flushed_len = slot.committed_len;
        slot.dirty = false;
        Ok(())
    }

    pub async fn close(
        &mut self,
        handle: FileHandle,
        valid_bytes_last_block: usize,
    ) -> StorageResult<(), D::Error> {
        if valid_bytes_last_block > SIZE {
            return Err(StorageError::InvalidBufferLength);
        }

        let index = self.validate_write_handle(handle)?;
        let mut slot = self.write_slots[index]
            .take()
            .ok_or(StorageError::InvalidHandle)?;
        let fs = &mut self.fs;

        if slot.pending_len != 0 {
            let valid = valid_bytes_last_block.min(slot.pending_len);
            if valid != 0 {
                slot.file
                    .write(fs, &slot.pending_block[..valid])
                    .await
                    .map_err(StorageError::from)?;
            }
            slot.committed_len = slot.committed_len.saturating_add(valid as u64);
            slot.logical_len = slot
                .logical_len
                .saturating_sub(slot.pending_len as u64)
                .saturating_add(valid as u64);
            slot.pending_len = 0;
        } else if valid_bytes_last_block != 0 && valid_bytes_last_block != SIZE {
            return Err(StorageError::InvalidState);
        }

        slot.file.flush(fs).await.map_err(StorageError::from)?;
        Ok(())
    }

    pub fn write_file_size(&self, handle: FileHandle) -> StorageResult<u64, D::Error> {
        let slot = self.write_slot(handle)?;
        Ok(slot.logical_len)
    }

    fn write_slot(
        &self,
        handle: FileHandle,
    ) -> StorageResult<&WriteSlot<SIZE, PATH_LEN>, D::Error> {
        let index = self.validate_write_handle(handle)?;
        self.write_slots[index]
            .as_ref()
            .ok_or(StorageError::InvalidHandle)
    }

    fn validate_write_handle(&self, handle: FileHandle) -> StorageResult<usize, D::Error> {
        let index = handle.index();
        if index >= MAX_WRITE_HANDLES {
            return Err(StorageError::InvalidHandle);
        }

        let slot = self.write_slots[index]
            .as_ref()
            .ok_or(StorageError::InvalidHandle)?;
        if slot.generation != handle.generation() {
            return Err(StorageError::InvalidHandle);
        }

        Ok(index)
    }
}
