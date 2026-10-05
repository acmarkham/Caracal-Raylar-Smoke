use core::fmt::Write as _;
use core::sync::atomic::{AtomicU32, Ordering};

use defmt::{error, info, warn};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Instant, Timer};
use heapless::String;
use raylar_board_v1p0::SdCard;
use raylar_logging_service::{
    LogLevel, LogOutcome, LoggingResources, LoggingService, ProcessOutcome, StorageLogSink,
};
use raylar_radio_service::{
    BootId, Epoch, FrameType, JobId, NodeId, RadioMode, RadioServiceError, RadioServiceStats,
    RendezvousPurpose, ScheduleError, Sequence,
};
use raylar_storage_service::StorageService;
use raylar_time_service::{TimeSource, UtcStatus, UtcTimestamp};
use static_cell::StaticCell;

use crate::common;
use integration004_radioheartbeat::policy::{Role, WindowClass};

const MESSAGE_LENGTH: usize = 512;
const LOG_QUEUE_DEPTH: usize = 64;
const LINE_LENGTH: usize = 640;
const RECORD_QUEUE_DEPTH: usize = 128;
const LOG_WRITE_BUFFER_BYTES: usize = 8 * 1024;
const FLUSH_INTERVAL: Duration = Duration::from_secs(10);
const FORMATTED_RECORD_LENGTH: usize = 2_048;
const LOG_PART_LENGTH: usize = 448;

type BoardStorage = StorageService<
    common::BoardStorageBackend,
    &'static raylar_time_service::TimeResources<4, 8>,
    512,
    1,
    LOG_WRITE_BUFFER_BYTES,
>;
type BoardLogSink = StorageLogSink<
    'static,
    common::BoardStorageBackend,
    &'static raylar_time_service::TimeResources<4, 8>,
    512,
    1,
    LOG_WRITE_BUFFER_BYTES,
>;

static RECORDS: Channel<CriticalSectionRawMutex, Record, RECORD_QUEUE_DEPTH> = Channel::new();
static LOGGING: LoggingResources<MESSAGE_LENGTH, LOG_QUEUE_DEPTH> = LoggingResources::new();
static STORAGE: StaticCell<BoardStorage> = StaticCell::new();
static NODE_ID: AtomicU32 = AtomicU32::new(0);
static BOOT_ID: AtomicU32 = AtomicU32::new(0);
static ENQUEUE_DROPS: AtomicU32 = AtomicU32::new(0);
static LOG_DROPS: AtomicU32 = AtomicU32::new(0);
static LOG_TRUNCATIONS: AtomicU32 = AtomicU32::new(0);
static RECORD_SEQUENCE: AtomicU32 = AtomicU32::new(0);

#[allow(dead_code)] // Fields are consumed by the derived Debug formatter in the logger task.
#[derive(Clone, Copy, Debug)]
pub enum DiagnosticKind {
    Boot {
        test_name: &'static str,
        firmware_version: &'static str,
        firmware_hash: Option<&'static str>,
        role: Role,
        network_id: u32,
        schedule_version: u8,
        configuration_id: u16,
        frequency_hz: u32,
        tx_power_dbm: i8,
    },
    TimeTransition {
        previous: UtcStatus,
        current: UtcStatus,
    },
    TimeCalibration {
        status: UtcStatus,
        source: TimeSource,
        uncertainty_us: u64,
        accepted_anchors: u32,
        rejected_anchors: u32,
        calibration_samples: u8,
        calibration_locked: bool,
        calibrated_error_ppb: i64,
    },
    LocationStatus {
        valid: bool,
        latitude_e7: i32,
        longitude_e7: i32,
        fixes_seen: u64,
        fixes_used: u8,
        satellites: Option<u8>,
        hdop_centi: Option<u16>,
        uncertainty_meters: Option<u32>,
    },
    EpochScheduled {
        epoch: Epoch,
        scan: bool,
        presence_slot: u32,
        heartbeat_slot: u32,
        receive_windows: u8,
    },
    WindowSkipped {
        epoch: Epoch,
        reason: ScheduleError,
    },
    TxSubmitted {
        id: JobId,
        epoch: Epoch,
        purpose: RendezvousPurpose,
        slot: u32,
        sequence: Sequence,
    },
    TxSubmitFailed {
        epoch: Epoch,
        purpose: RendezvousPurpose,
        slot: u32,
        error: ScheduleError,
    },
    TxCompleted {
        id: JobId,
        epoch: Epoch,
        purpose: RendezvousPurpose,
        slot: u32,
        sequence: Sequence,
    },
    TxRejected {
        id: JobId,
        epoch: Epoch,
        purpose: RendezvousPurpose,
        slot: u32,
        error: RadioServiceError,
    },
    Rx {
        frame_type: FrameType,
        source: NodeId,
        source_boot: BootId,
        sequence: Sequence,
        epoch: Option<Epoch>,
        expected_slot: Option<u32>,
        class: WindowClass,
        rssi_dbm_x2: i16,
        snr_db_x4: Option<i16>,
        valid: bool,
    },
    RxWindow {
        id: JobId,
        opened: bool,
        scan: bool,
        predicted: bool,
        receptions: u16,
    },
    RadioFailure {
        id: JobId,
        error: RadioServiceError,
    },
    PacketRejected {
        id: JobId,
    },
    NeighbourChanged {
        node: NodeId,
        boot: BootId,
        discovered: bool,
        boot_changed: bool,
        base_station: bool,
        count: u16,
    },
    NeighboursExpired {
        count: u16,
    },
    NeighbourTable {
        epoch: Epoch,
        count: u16,
    },
    NeighbourEntry {
        epoch: Epoch,
        index: u16,
        node: NodeId,
        boot: BootId,
        base_station: bool,
        schedule_version: u8,
        last_seen_utc: UtcTimestamp,
        location: Option<raylar_radio_service::CompactLocation>,
        location_uncertainty_meters: Option<u32>,
        rssi_dbm_x2: Option<i16>,
        snr_db_x4: Option<i16>,
        received_packets: u32,
        failed_packets: u32,
    },
    TopologyWarning {
        base_station_count: u8,
    },
    Summary {
        epoch: Epoch,
        mode: RadioMode,
        radio: RadioServiceStats,
        heartbeat_rx: u32,
        unknown_heartbeat_rx: u32,
        predicted_rx: u32,
        predicted_misses: u32,
        scan_rx: u32,
        outside_rx: u32,
        unverifiable_rx: u32,
        application_malformed: u32,
        diagnostic_drops: u32,
        logging_drops: u32,
        logging_truncations: u32,
    },
    StorageReady {
        queued_record_drops: u32,
    },
    LoggingHealth {
        queue_drops: u64,
        truncations: u64,
        write_failures: u64,
        maximum_depth: usize,
    },
}

#[derive(Clone, Copy, Debug)]
struct Record {
    sequence: u32,
    at: Instant,
    node_id: NodeId,
    boot_id: BootId,
    utc: Option<UtcTimestamp>,
    utc_status: UtcStatus,
    utc_uncertainty_us: u64,
    kind: DiagnosticKind,
}

pub fn initialize_identity(node_id: NodeId, boot_id: BootId) {
    NODE_ID.store(node_id.0, Ordering::Release);
    BOOT_ID.store(boot_id.0, Ordering::Release);
}

pub fn emit(kind: DiagnosticKind) -> bool {
    let at = Instant::now();
    let time = common::TIME_RESOURCES.time_state();
    let record = Record {
        sequence: RECORD_SEQUENCE.fetch_add(1, Ordering::Relaxed),
        at,
        node_id: NodeId(NODE_ID.load(Ordering::Acquire)),
        boot_id: BootId(BOOT_ID.load(Ordering::Acquire)),
        utc: time.system_to_utc(at).ok(),
        utc_status: time.utc_status,
        utc_uncertainty_us: time.uncertainty_us,
        kind,
    };
    if RECORDS.try_send(record).is_err() {
        ENQUEUE_DROPS.fetch_add(1, Ordering::Relaxed);
        error!("integration004 diagnostic queue full");
        false
    } else {
        true
    }
}

pub fn enqueue_drops() -> u32 {
    ENQUEUE_DROPS.load(Ordering::Relaxed)
}

pub fn logging_drops() -> u32 {
    LOG_DROPS.load(Ordering::Relaxed)
}

pub fn logging_truncations() -> u32 {
    LOG_TRUNCATIONS.load(Ordering::Relaxed)
}

fn account(outcome: LogOutcome) {
    match outcome {
        LogOutcome::Enqueued => {}
        LogOutcome::EnqueuedTruncated => {
            LOG_TRUNCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        LogOutcome::DroppedQueueFull => {
            LOG_DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }
}

#[embassy_executor::task]
pub async fn logging_task(sd: SdCard<'static>) -> ! {
    let backend = common::storage_driver(sd).await;
    let mut storage: BoardStorage = match StorageService::new(backend, &common::TIME_RESOURCES) {
        Ok(storage) => storage,
        Err(value) => {
            error!("integration004 storage construction failed: {:?}", value);
            common::pending_forever().await
        }
    };
    loop {
        match storage.mount().await {
            Ok(()) => break,
            Err(value) => {
                error!("integration004 storage mount failed: {:?}", value);
                Timer::after_secs(5).await;
            }
        }
    }
    let storage = STORAGE.init(storage);
    let sink: BoardLogSink = match StorageLogSink::open(storage).await {
        Ok(sink) => sink,
        Err(value) => {
            error!("integration004 log stream open failed: {:?}", value);
            common::pending_forever().await
        }
    };
    let mut logging =
        LoggingService::<_, MESSAGE_LENGTH, LOG_QUEUE_DEPTH, LINE_LENGTH>::new(&LOGGING, sink);
    let logger = logging.register("RadioHeartbeat");
    let early_drops = enqueue_drops();
    emit(DiagnosticKind::StorageReady {
        queued_record_drops: early_drops,
    });
    info!("Integration004 persistent logger ready");

    let mut next_flush = Instant::now() + FLUSH_INTERVAL;
    loop {
        let mut progressed = false;
        while let Ok(record) = RECORDS.try_receive() {
            let mut formatted = String::<FORMATTED_RECORD_LENGTH>::new();
            let format_result = write!(
                formatted,
                "node={:#010x} boot={:#010x} utc={:?} utc_status={:?} utc_uncertainty_us={} event={:?}",
                record.node_id.0,
                record.boot_id.0,
                record.utc,
                record.utc_status,
                record.utc_uncertainty_us,
                record.kind,
            );
            if format_result.is_err() {
                LOG_TRUNCATIONS.fetch_add(1, Ordering::Relaxed);
                error!("integration004 diagnostic formatting capacity exceeded");
            }
            let part_count = formatted.len().max(1).div_ceil(LOG_PART_LENGTH);
            let mut start = 0;
            let mut part = 1;
            while start < formatted.len() {
                let mut end = (start + LOG_PART_LENGTH).min(formatted.len());
                while !formatted.is_char_boundary(end) {
                    end -= 1;
                }
                account(logger.log_at(
                    record.at,
                    LogLevel::Info,
                    format_args!(
                        "record={} part={}/{} {}",
                        record.sequence,
                        part,
                        part_count,
                        &formatted[start..end],
                    ),
                ));
                start = end;
                part += 1;
            }
            progressed = true;
            if LOGGING.stats().queue_depth >= LOG_QUEUE_DEPTH / 2 {
                break;
            }
        }

        match logging.process_one().await {
            Ok(ProcessOutcome::Written) => progressed = true,
            Ok(ProcessOutcome::Empty) => {}
            Err(value) => {
                error!("integration004 log append failed: {:?}", value);
                Timer::after_millis(100).await;
            }
        }

        if Instant::now() >= next_flush {
            if let Err(value) = logging.flush().await {
                error!("integration004 log flush failed: {:?}", value);
            }
            let stats = logging.stats();
            if stats.dropped_messages != 0
                || stats.truncated_messages != 0
                || stats.write_failures != 0
                || enqueue_drops() != 0
            {
                warn!(
                    "Integration004 log incomplete: record_drops={} queue_drops={} truncations={} failures={}",
                    enqueue_drops(),
                    stats.dropped_messages,
                    stats.truncated_messages,
                    stats.write_failures,
                );
                emit(DiagnosticKind::LoggingHealth {
                    queue_drops: stats
                        .dropped_messages
                        .saturating_add(u64::from(enqueue_drops())),
                    truncations: stats.truncated_messages,
                    write_failures: stats.write_failures,
                    maximum_depth: stats.maximum_queue_depth,
                });
            }
            next_flush = Instant::now() + FLUSH_INTERVAL;
        }

        if !progressed {
            Timer::after_millis(10).await;
        }
    }
}
