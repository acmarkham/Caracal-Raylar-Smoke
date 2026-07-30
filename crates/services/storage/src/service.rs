use raylar_drivers::storage::BLOCK_BYTES;
use raylar_time_service::{TimeResources, UtcTimestamp};

use crate::backend::StorageBackend;
use crate::policy::{folder_path, stream_path};
use crate::types::{StorageLayout, StorageServiceError, StreamHandle, StreamKind, StreamSlot};

pub const DEFAULT_MAX_STREAMS: usize = 4;

pub trait UtcClock {
    fn current_utc(&self) -> Option<UtcTimestamp>;
}

impl<const WATCHERS: usize, const ANCHOR_DEPTH: usize> UtcClock
    for &TimeResources<WATCHERS, ANCHOR_DEPTH>
{
    fn current_utc(&self) -> Option<UtcTimestamp> {
        TimeResources::current_utc(self).ok()
    }
}

pub struct StorageService<
    B,
    C,
    const BLOCK_SIZE: usize = BLOCK_BYTES,
    const MAX_STREAMS: usize = DEFAULT_MAX_STREAMS,
> {
    backend: B,
    clock: C,
    slots: [Option<StreamSlot<BLOCK_SIZE>>; MAX_STREAMS],
    generations: [u8; MAX_STREAMS],
    stream_sequence: u32,
}

impl<B, C, const BLOCK_SIZE: usize, const MAX_STREAMS: usize>
    StorageService<B, C, BLOCK_SIZE, MAX_STREAMS>
where
    B: StorageBackend<BLOCK_SIZE>,
    C: UtcClock,
{
    pub fn new(backend: B, clock: C) -> Result<Self, StorageServiceError<B::Error>> {
        if BLOCK_SIZE == 0 || MAX_STREAMS == 0 || MAX_STREAMS > usize::from(u8::MAX) + 1 {
            return Err(StorageServiceError::InvalidConfig);
        }
        Ok(Self {
            backend,
            clock,
            slots: core::array::from_fn(|_| None),
            generations: [0; MAX_STREAMS],
            stream_sequence: 0,
        })
    }

    pub async fn mount(&mut self) -> Result<(), StorageServiceError<B::Error>> {
        self.backend
            .mount()
            .await
            .map_err(StorageServiceError::Backend)
    }

    pub async fn begin_stream(
        &mut self,
        kind: StreamKind,
        layout: StorageLayout,
    ) -> Result<StreamHandle, StorageServiceError<B::Error>> {
        let index = self
            .slots
            .iter()
            .position(Option::is_none)
            .ok_or(StorageServiceError::TooManyStreams)?;
        let generation = self.generations[index].wrapping_add(1);
        let sequence = self.stream_sequence.wrapping_add(1);
        let timestamp = self.clock.current_utc().map(|now| now.seconds);
        let path = stream_path(kind, layout, timestamp, sequence)?;
        if let Some(folder) = folder_path::<B::Error>(path.as_str())? {
            self.backend
                .create_directory(folder.as_str())
                .await
                .map_err(StorageServiceError::Backend)?;
        }
        let file = self
            .backend
            .open_for_append(path.as_str())
            .await
            .map_err(StorageServiceError::Backend)?;

        self.generations[index] = generation;
        self.stream_sequence = sequence;
        let mut slot = StreamSlot::new(generation);
        slot.path = path;
        slot.file = Some(file);
        self.slots[index] = Some(slot);
        Ok(StreamHandle::new(index, generation))
    }

    pub async fn write(
        &mut self,
        stream: StreamHandle,
        data: &[u8],
    ) -> Result<(), StorageServiceError<B::Error>> {
        let index = self.validate(stream)?;
        let slot = self.slots[index]
            .as_mut()
            .ok_or(StorageServiceError::InvalidStream)?;
        append_bytes(&mut self.backend, slot, data).await
    }

    pub async fn flush(
        &mut self,
        stream: StreamHandle,
    ) -> Result<(), StorageServiceError<B::Error>> {
        let index = self.validate(stream)?;
        let slot = self.slots[index]
            .as_mut()
            .ok_or(StorageServiceError::InvalidStream)?;
        let handle = slot.file.ok_or(StorageServiceError::InvalidStream)?;
        if slot.pending_len == 0 {
            return self
                .backend
                .flush(handle)
                .await
                .map_err(StorageServiceError::Backend);
        }

        let valid = slot.pending_len;
        self.backend
            .append(handle, &slot.pending)
            .await
            .map_err(StorageServiceError::Backend)?;
        slot.pending_len = 0;
        slot.file = None;
        self.backend
            .close(handle, valid)
            .await
            .map_err(StorageServiceError::Backend)?;
        slot.file = Some(
            self.backend
                .open_for_append(slot.path.as_str())
                .await
                .map_err(StorageServiceError::Backend)?,
        );
        Ok(())
    }

    pub async fn finish(
        &mut self,
        stream: StreamHandle,
    ) -> Result<(), StorageServiceError<B::Error>> {
        let index = self.validate(stream)?;
        let mut slot = self.slots[index]
            .take()
            .ok_or(StorageServiceError::InvalidStream)?;
        close_slot_file(&mut self.backend, &mut slot).await
    }

    fn validate(&self, stream: StreamHandle) -> Result<usize, StorageServiceError<B::Error>> {
        let index = stream.index();
        let slot = self
            .slots
            .get(index)
            .and_then(Option::as_ref)
            .ok_or(StorageServiceError::InvalidStream)?;
        if slot.generation != stream.generation() {
            return Err(StorageServiceError::InvalidStream);
        }
        Ok(index)
    }
}

async fn append_bytes<B, const BLOCK_SIZE: usize>(
    backend: &mut B,
    slot: &mut StreamSlot<BLOCK_SIZE>,
    mut data: &[u8],
) -> Result<(), StorageServiceError<B::Error>>
where
    B: StorageBackend<BLOCK_SIZE>,
{
    let handle = slot.file.ok_or(StorageServiceError::InvalidStream)?;
    while !data.is_empty() {
        if slot.pending_len == 0 && data.len() >= BLOCK_SIZE {
            let direct_len = data.len() / BLOCK_SIZE * BLOCK_SIZE;
            backend
                .append(handle, &data[..direct_len])
                .await
                .map_err(StorageServiceError::Backend)?;
            data = &data[direct_len..];
            continue;
        }
        let copy_len = (BLOCK_SIZE - slot.pending_len).min(data.len());
        slot.pending[slot.pending_len..slot.pending_len + copy_len]
            .copy_from_slice(&data[..copy_len]);
        slot.pending_len += copy_len;
        data = &data[copy_len..];
        if slot.pending_len == BLOCK_SIZE {
            backend
                .append(handle, &slot.pending)
                .await
                .map_err(StorageServiceError::Backend)?;
            slot.pending_len = 0;
        }
    }
    Ok(())
}

async fn close_slot_file<B, const BLOCK_SIZE: usize>(
    backend: &mut B,
    slot: &mut StreamSlot<BLOCK_SIZE>,
) -> Result<(), StorageServiceError<B::Error>>
where
    B: StorageBackend<BLOCK_SIZE>,
{
    let Some(handle) = slot.file.take() else {
        return Ok(());
    };
    let valid = if slot.pending_len == 0 {
        BLOCK_SIZE
    } else {
        let valid = slot.pending_len;
        backend
            .append(handle, &slot.pending)
            .await
            .map_err(StorageServiceError::Backend)?;
        slot.pending_len = 0;
        valid
    };
    backend
        .close(handle, valid)
        .await
        .map_err(StorageServiceError::Backend)
}
