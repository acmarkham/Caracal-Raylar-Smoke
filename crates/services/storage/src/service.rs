use raylar_drivers::storage::BLOCK_BYTES;
use raylar_time_service::{TimeResources, UtcTimestamp};

use crate::backend::StorageBackend;
use crate::policy::{file_policy, folder_path};
use crate::types::{
    AppendOutcome, StorageConfig, StorageServiceError, StreamHandle, StreamSlot, StreamType,
};

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
    config: StorageConfig,
    slots: [Option<StreamSlot<BLOCK_SIZE>>; MAX_STREAMS],
    generations: [u8; MAX_STREAMS],
}

impl<B, C, const BLOCK_SIZE: usize, const MAX_STREAMS: usize>
    StorageService<B, C, BLOCK_SIZE, MAX_STREAMS>
where
    B: StorageBackend<BLOCK_SIZE>,
    C: UtcClock,
{
    pub fn new(
        backend: B,
        clock: C,
        config: StorageConfig,
    ) -> Result<Self, StorageServiceError<B::Error>> {
        if BLOCK_SIZE == 0 || !config.audio.is_valid() || !config.gps_timing.is_valid() {
            return Err(StorageServiceError::InvalidConfig);
        }
        Ok(Self {
            backend,
            clock,
            config,
            slots: core::array::from_fn(|_| None),
            generations: [0; MAX_STREAMS],
        })
    }

    pub async fn mount(&mut self) -> Result<(), StorageServiceError<B::Error>> {
        self.backend
            .mount()
            .await
            .map_err(StorageServiceError::Backend)
    }

    pub async fn create_stream(
        &mut self,
        kind: StreamType,
    ) -> Result<StreamHandle, StorageServiceError<B::Error>> {
        if self.slots.iter().flatten().any(|slot| slot.kind == kind) {
            return Err(StorageServiceError::StreamAlreadyOpen);
        }
        let index = self
            .slots
            .iter()
            .position(Option::is_none)
            .ok_or(StorageServiceError::TooManyStreams)?;
        let generation = self.generations[index].wrapping_add(1);
        self.generations[index] = generation;
        let mut slot = StreamSlot::new(kind, generation);

        match kind {
            StreamType::Log => {
                slot.path
                    .push_str("/syslog.txt")
                    .map_err(|_| StorageServiceError::InvalidPath)?;
                slot.file = Some(
                    self.backend
                        .open_for_append(slot.path.as_str())
                        .await
                        .map_err(StorageServiceError::Backend)?,
                );
            }
            StreamType::Audio | StreamType::GpsTiming => {
                if let Some(now) = self.clock.current_utc() {
                    open_rolling_file(&mut self.backend, self.config, &mut slot, now.seconds, true)
                        .await?;
                }
            }
        }

        self.slots[index] = Some(slot);
        Ok(StreamHandle::new(index, generation))
    }

    pub async fn append(
        &mut self,
        stream: StreamHandle,
        data: &[u8],
    ) -> Result<AppendOutcome, StorageServiceError<B::Error>> {
        let index = self.validate(stream)?;
        let kind = self.slots[index]
            .as_ref()
            .ok_or(StorageServiceError::InvalidStream)?
            .kind;
        let now = match kind {
            StreamType::Log => None,
            StreamType::Audio | StreamType::GpsTiming => match self.clock.current_utc() {
                Some(now) => Some(now.seconds),
                None => return Ok(AppendOutcome::DroppedUtcUnavailable),
            },
        };

        let slot = self.slots[index]
            .as_mut()
            .ok_or(StorageServiceError::InvalidStream)?;
        if let Some(timestamp) = now {
            let needs_open = slot.file.is_none();
            let needs_rollover = slot.rollover_at.is_some_and(|at| timestamp >= at)
                || slot.file_start.is_some_and(|start| timestamp < start);
            if needs_rollover {
                close_slot_file(&mut self.backend, slot).await?;
            }
            if needs_open || needs_rollover {
                open_rolling_file(&mut self.backend, self.config, slot, timestamp, needs_open)
                    .await?;
            }
        }

        append_bytes(&mut self.backend, slot, data).await?;
        Ok(AppendOutcome::Written)
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

    pub async fn close(
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

async fn open_rolling_file<B, const BLOCK_SIZE: usize>(
    backend: &mut B,
    config: StorageConfig,
    slot: &mut StreamSlot<BLOCK_SIZE>,
    timestamp: i64,
    initial_open: bool,
) -> Result<(), StorageServiceError<B::Error>>
where
    B: StorageBackend<BLOCK_SIZE>,
{
    let rolling = match slot.kind {
        StreamType::Audio => config.audio,
        StreamType::GpsTiming => config.gps_timing,
        StreamType::Log => return Err(StorageServiceError::InvalidStream),
    };
    let policy = file_policy(slot.kind, timestamp, rolling, initial_open)?;
    let folder = folder_path::<B::Error>(policy.path.as_str())?;
    backend
        .create_directory(folder.as_str())
        .await
        .map_err(StorageServiceError::Backend)?;
    let handle = backend
        .open_for_append(policy.path.as_str())
        .await
        .map_err(StorageServiceError::Backend)?;
    slot.path = policy.path;
    slot.file = Some(handle);
    slot.file_start = Some(policy.file_start);
    slot.rollover_at = Some(policy.rollover_at);
    Ok(())
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
