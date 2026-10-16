//! Bounded storage diagnostics; no per-write logging or extra sample buffers.
use core::sync::atomic::{AtomicU32, Ordering};
use embassy_time::{Duration, Timer};
use raylar_logging_service::{LoggerHandle, info};

#[derive(Clone, Copy)]
#[repr(usize)]
pub(super) enum Op {
    Admit,
    OpenReference,
    OpenCompanion,
    Write,
    Rewrite,
    CloseReference,
    CloseCompanion,
    LogWrite,
    LogCheckpoint,
    LogFlush,
}
const NAMES: [&str; 10] = [
    "admit",
    "open_ref",
    "open_crcl",
    "append",
    "rewrite",
    "close_ref",
    "close_crcl",
    "log_append",
    "log_checkpoint",
    "log_flush",
];
struct Timing {
    calls: AtomicU32,
    wait: AtomicU32,
    execution: AtomicU32,
    total: AtomicU32,
}
impl Timing {
    const fn new() -> Self {
        Self {
            calls: AtomicU32::new(0),
            wait: AtomicU32::new(0),
            execution: AtomicU32::new(0),
            total: AtomicU32::new(0),
        }
    }
}
static TIMINGS: [Timing; 10] = [const { Timing::new() }; 10];
/// Bytes submitted successfully to backend append, excluding buffered bytes,
/// headers rewritten in place, and log traffic. Directory durability still
/// requires a successful close. Counter wraps modulo 2^32.
pub static AUDIO_BACKEND_BYTES: AtomicU32 = AtomicU32::new(0);
pub static AUDIO_FILES_CLOSED: AtomicU32 = AtomicU32::new(0);

pub(super) fn record(op: Op, wait: u64, execution: u64) {
    let t = &TIMINGS[op as usize];
    t.calls.fetch_add(1, Ordering::Relaxed);
    t.wait
        .fetch_max(wait.min(u32::MAX as u64) as u32, Ordering::Relaxed);
    t.execution
        .fetch_max(execution.min(u32::MAX as u64) as u32, Ordering::Relaxed);
    t.total.fetch_max(
        wait.saturating_add(execution).min(u32::MAX as u64) as u32,
        Ordering::Relaxed,
    );
}

#[embassy_executor::task]
pub async fn diagnostics_task(log: LoggerHandle<'static, 384, 32>) -> ! {
    loop {
        Timer::after(Duration::from_secs(10)).await;
        for (name, t) in NAMES.iter().zip(&TIMINGS) {
            let calls = t.calls.swap(0, Ordering::Relaxed);
            let wait = t.wait.swap(0, Ordering::Relaxed);
            let execution = t.execution.swap(0, Ordering::Relaxed);
            let total = t.total.swap(0, Ordering::Relaxed);
            if calls != 0 {
                let _ = info!(
                    log,
                    "storage timing op={} calls={} wait_max_us={} execution_max_us={} total_max_us={}",
                    name,
                    calls,
                    wait,
                    execution,
                    total
                );
            }
        }
    }
}
