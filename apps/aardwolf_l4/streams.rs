//! Two stream clients, one unchanged StorageService/filesystem owner.
use crate::{sd, LOGGING, TIME};
use aardwolf_l4::*;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering::Relaxed};
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex};
use embassy_time::Instant;
use raylar_audio_recorder_service::RecordingStorage;
use raylar_drivers::storage::{PartitionedBlockDevice, StorageDriver};
use raylar_logging_service::{LogOutcome, LogSink, LoggerHandle};
use raylar_storage_service::{
    StorageBackend, StorageLayout, StorageService, StorageServiceError, StreamHandle,
};
use raylar_time_service::{TimeResources, UtcTimestamp};

pub type Backend = StorageDriver<PartitionedBlockDevice<sd::L4Sd<'static, 'static>>, 512, 1, 96>;
pub type Service = StorageService<Backend, &'static TimeResources<2, 2>, 512, 2, 512>;
pub type StorageError = StorageServiceError<<Backend as StorageBackend<512>>::Error>;
pub static STORAGE: Mutex<CriticalSectionRawMutex, Option<Service>> = Mutex::new(None);
pub static CLOSED: AtomicU32 = AtomicU32::new(0);
pub static OPEN: AtomicU32 = AtomicU32::new(0);
pub static MAX_OPEN: AtomicU32 = AtomicU32::new(0);
pub static AUDIO_OPEN: AtomicBool = AtomicBool::new(false);
pub static LOG_OPEN: AtomicBool = AtomicBool::new(false);
pub static STOP_LOG: AtomicBool = AtomicBool::new(false);
pub static LOG_CHECKPOINTS_DURING_AUDIO: AtomicU32 = AtomicU32::new(0);
pub static MAX_APPEND_US: AtomicU32 = AtomicU32::new(0);
pub static CARD_SERIAL: AtomicU32 = AtomicU32::new(0);
pub type Logger = LoggerHandle<'static, 192, 8>;
pub fn logger() -> Logger {
    LOGGING.register("L4Record")
}
pub fn log(args: core::fmt::Arguments<'_>) {
    if crate::fault::active() {
        return;
    }
    if logger().log_at(Instant::now(), raylar_logging_service::LogLevel::Info, args)
        != LogOutcome::Enqueued
    {
        // The logging service counts dropped/truncated messages for telemetry.
        defmt::warn!("AARDWOLF L4 syslog queue full or message truncated");
    }
}
pub fn opened() {
    let n = OPEN.fetch_add(1, Relaxed) + 1;
    MAX_OPEN.fetch_max(n, Relaxed);
    defmt::assert!(n <= 2);
}
pub fn closed() {
    defmt::assert!(OPEN.fetch_sub(1, Relaxed) > 0);
}

#[derive(Debug, defmt::Format)]
pub enum RecordError {
    Storage(StorageError),
    BenchComplete,
    FaultLatched,
}
pub struct Recording {
    pub opened: u32,
    pub hashes: [u32; BENCH_SEGMENTS],
    pub starts: [i64; BENCH_SEGMENTS],
    bytes: u64,
    hash: u32,
}
impl Recording {
    pub const fn new() -> Self {
        Self {
            opened: 0,
            hashes: [0; BENCH_SEGMENTS],
            starts: [0; BENCH_SEGMENTS],
            bytes: 0,
            hash: FNV_INITIAL,
        }
    }
}
impl RecordingStorage for Recording {
    type Error = RecordError;
    type Handle = StreamHandle;
    async fn begin_audio_stream(
        &mut self,
        layout: StorageLayout,
        utc: UtcTimestamp,
        suffix: &str,
    ) -> Result<StreamHandle, RecordError> {
        // The preceding segment has already been finalized by AudioRecorder.
        // Stop the bench before allocating another file, without cancelling I/O.
        if (cfg!(feature = "bench-gps-cycle") && crate::gnss::BENCH_FINISH.load(Relaxed))
            || (cfg!(feature = "bench-audio-ui") && self.opened >= 2)
        {
            return Err(RecordError::BenchComplete);
        }
        if crate::fault::active() {
            return Err(RecordError::FaultLatched);
        }
        defmt::assert!(
            !cfg!(feature = "bench-gps-cycle") || self.opened < BENCH_SEGMENTS as u32,
            "AARDWOLF L4 bench GPS cycle timeout"
        );
        let mut guard = STORAGE.lock().await;
        let storage = guard.as_mut().unwrap();
        let (_, free, cluster) = storage
            .space_info()
            .await
            .map_err(RecordError::Storage)?
            .unwrap();
        if !aardwolf_l4::policy::can_start_audio(free, cluster, storage.pending_bytes()) {
            return Err(RecordError::Storage(StorageServiceError::OutOfSpace));
        }
        let handle = storage
            .begin_new_audio_with_suffix_at(layout, utc, suffix)
            .await
            .map_err(RecordError::Storage)?;
        if (self.opened as usize) < BENCH_SEGMENTS {
            self.starts[self.opened as usize] = utc.seconds;
        }
        self.opened += 1;
        self.bytes = 0;
        self.hash = FNV_INITIAL;
        AUDIO_OPEN.store(true, Relaxed);
        opened();
        log(format_args!(
            "segment_begin index={} utc={} sample_rate=16000 channels=1 wav_bits=32 source_bits=24",
            self.opened, utc.seconds
        ));
        defmt::info!(
            "AARDWOLF L4 begin index={} utc={} open_streams={}",
            self.opened,
            utc.seconds,
            OPEN.load(Relaxed)
        );
        Ok(handle)
    }
    async fn append_audio(
        &mut self,
        stream: StreamHandle,
        bytes: &[u8],
    ) -> Result<(), RecordError> {
        if crate::fault::active() {
            return Err(RecordError::FaultLatched);
        }
        let start = Instant::now();
        STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .write(stream, bytes)
            .await
            .map_err(RecordError::Storage)?;
        self.bytes += bytes.len() as u64;
        self.hash = hash_bytes(self.hash, bytes);
        MAX_APPEND_US.fetch_max((Instant::now() - start).as_micros() as u32, Relaxed);
        Ok(())
    }
    async fn rewrite_audio(
        &mut self,
        stream: StreamHandle,
        offset: u64,
        bytes: &[u8],
    ) -> Result<(), RecordError> {
        if crate::fault::active() {
            return Err(RecordError::FaultLatched);
        }
        // Every finished segment must be a complete minute, so these RIFF
        // patches equal the original header and do not change its running hash.
        defmt::assert!(
            (offset == 4 && bytes == ((FILE_BYTES - 8) as u32).to_le_bytes())
                || (offset == 508 && bytes == (PAYLOAD_BYTES as u32).to_le_bytes())
        );
        STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .rewrite(stream, offset, bytes)
            .await
            .map_err(RecordError::Storage)
    }
    async fn finish_audio(&mut self, stream: StreamHandle) -> Result<(), RecordError> {
        if crate::fault::active() {
            return Err(RecordError::FaultLatched);
        }
        STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .finish(stream)
            .await
            .map_err(RecordError::Storage)?;
        defmt::assert!(self.bytes == FILE_BYTES && LOG_OPEN.load(Relaxed));
        if (self.opened as usize) <= BENCH_SEGMENTS {
            self.hashes[self.opened as usize - 1] = self.hash;
        }
        CLOSED.fetch_add(1, Relaxed);
        AUDIO_OPEN.store(false, Relaxed);
        closed();
        log(format_args!(
            "segment_closed index={} bytes={} samples=960000 hash={:08x}",
            self.opened, self.bytes, self.hash
        ));
        defmt::info!(
            "AARDWOLF L4 closed index={} bytes={} samples=960000 hash={:08x}",
            self.opened,
            self.bytes,
            self.hash
        );
        Ok(())
    }
}

pub struct Syslog {
    pub stream: StreamHandle,
    pub bytes: u64,
    pub hash: u32,
    pub old_length: u64,
}
#[derive(Debug, defmt::Format)]
pub enum LogError {
    Storage(StorageError),
    FaultLatched,
}
impl LogSink for Syslog {
    type Error = LogError;
    async fn append(&mut self, bytes: &[u8]) -> Result<(), LogError> {
        if crate::fault::active() {
            return Err(LogError::FaultLatched);
        }
        if self
            .old_length
            .saturating_add(self.bytes)
            .saturating_add(bytes.len() as u64)
            > aardwolf_l4::config::LOG_BUDGET_BYTES
        {
            return Err(LogError::Storage(StorageServiceError::OutOfSpace));
        }
        STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .write(self.stream, bytes)
            .await
            .map_err(LogError::Storage)?;
        self.hash = hash_bytes(self.hash, bytes);
        self.bytes += bytes.len() as u64;
        Ok(())
    }
    async fn flush(&mut self) -> Result<(), LogError> {
        if crate::fault::active() {
            return Err(LogError::FaultLatched);
        }
        STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .flush(self.stream)
            .await
            .map_err(LogError::Storage)
    }
    async fn checkpoint(&mut self) -> Result<(), LogError> {
        if crate::fault::active() {
            return Err(LogError::FaultLatched);
        }
        STORAGE
            .lock()
            .await
            .as_mut()
            .unwrap()
            .checkpoint(self.stream)
            .await
            .map_err(LogError::Storage)?;
        if AUDIO_OPEN.load(Relaxed) {
            LOG_CHECKPOINTS_DURING_AUDIO.fetch_add(1, Relaxed);
        }
        Ok(())
    }
}

pub fn metadata() -> raylar_audio_recorder_service::TimeMetadataSource<'static, 2, 2> {
    use core::fmt::Write;
    let mut card = heapless::String::<16>::new();
    write!(card, "{:08x}", CARD_SERIAL.load(Relaxed)).unwrap();
    raylar_audio_recorder_service::TimeMetadataSource::new(&TIME)
        .with_node_id(caracal_evo::identity::uid().serials().serial_32)
        .with_card_id(&card)
        .with_device_id(embassy_stm32::uid::uid_hex())
        .unwrap()
        .with_location_source(|| {
            let l = crate::LOCATION.state();
            l.valid
                .then_some((l.latitude.degrees_e7, l.longitude.degrees_e7))
        })
        .with_firmware_hash(concat!("L4Evo:", env!("L4_GIT"), ":", env!("L4_GIT_STATE")))
}
