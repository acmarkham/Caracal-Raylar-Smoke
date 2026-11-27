//! Audio and syslog clients share one serialized StorageService/filesystem owner.
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
pub type Service =
    StorageService<Backend, &'static TimeResources<2, 2>, 512, { crate::STREAM_COUNT }, 512>;
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
    defmt::assert!(n <= crate::STREAM_COUNT as u32);
}
pub fn closed() {
    defmt::assert!(OPEN.fetch_sub(1, Relaxed) > 0);
}

#[cfg(feature = "mono")]
#[path = "mono_stream.rs"]
mod recording;
#[cfg(not(feature = "mono"))]
#[path = "quad_stream.rs"]
mod recording;
pub use recording::{RecordError, Recording};
pub static BOOT_ID: AtomicU32 = AtomicU32::new(0);

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
        .with_boot_id(BOOT_ID.load(Relaxed))
        .with_node_id(caracal_evo::identity::uid().serials().serial_32)
        .with_card_id(&card)
        .with_device_id(embassy_stm32::uid::uid_hex())
        .unwrap()
        .with_location_source(|| {
            let l = crate::LOCATION.state();
            l.valid
                .then_some((l.latitude.degrees_e7, l.longitude.degrees_e7))
        })
        .with_firmware_hash(if cfg!(feature = "local-time") {
            concat!(
                "L4Evo:LOCAL-TIME-BENCH:",
                env!("L4_GIT"),
                ":",
                env!("L4_GIT_STATE")
            )
        } else {
            concat!("L4Evo:", env!("L4_GIT"), ":", env!("L4_GIT_STATE"))
        })
}
