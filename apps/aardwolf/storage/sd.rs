//! One bounded record per critical operation, emitted after the rotation.
//! File ownership stays with the storage mutex. Never log from SD I/O itself.
use super::{Op, NAMES};
use core::{
    cell::RefCell,
    sync::atomic::{AtomicU32, Ordering},
};
use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, Mutex};
use embassy_time::{Duration, Instant};
use raylar_drivers::storage::diagnostics::{self, Io, Scope, Snapshot, Transfers};
use raylar_logging_service::{info, LoggerHandle};

type Log = LoggerHandle<'static, 384, 32>;
static NEXT_ID: AtomicU32 = AtomicU32::new(1);
static LOST: AtomicU32 = AtomicU32::new(0);
struct Report {
    id: u32,
    op: Op,
    start: Instant,
    end: Instant,
    wait_us: u64,
    execution_us: u64,
    ok: bool,
    snapshot: Snapshot,
}
struct Queue {
    reports: heapless::Deque<Report, 16>,
    last_completed: Option<Instant>,
}
static QUEUE: Mutex<CriticalSectionRawMutex, RefCell<Queue>> = Mutex::new(RefCell::new(Queue {
    reports: heapless::Deque::new(),
    last_completed: None,
}));

pub(crate) struct Operation {
    id: u32,
    op: Op,
    start: Instant,
    wait_us: u64,
    scope: Scope,
}
impl Operation {
    pub(crate) fn begin(op: Op, wait_us: u64) -> Option<Self> {
        if !matches!(
            op,
            Op::OpenReference
                | Op::OpenCompanion
                | Op::Rewrite
                | Op::CloseReference
                | Op::CloseCompanion
        ) {
            return None;
        }
        let Some(scope) = Scope::begin() else {
            LOST.fetch_add(1, Ordering::Relaxed);
            return None;
        };
        Some(Self {
            id: NEXT_ID.fetch_add(1, Ordering::Relaxed),
            op,
            start: Instant::now(),
            wait_us,
            scope,
        })
    }
    pub(crate) fn finish(self, ok: bool, execution_us: u64) {
        let end = Instant::now();
        let report = Report {
            id: self.id,
            op: self.op,
            start: self.start,
            end,
            wait_us: self.wait_us,
            execution_us,
            ok,
            snapshot: self.scope.finish(),
        };
        QUEUE.lock(|q| {
            let mut q = q.borrow_mut();
            q.last_completed = Some(end);
            if q.reports.push_back(report).is_err() {
                LOST.fetch_add(1, Ordering::Relaxed);
            }
        });
    }
}

pub(super) fn drain_one(log: Log) {
    if diagnostics::active() {
        return;
    }
    let report = QUEUE.lock(|q| {
        let mut q = q.borrow_mut();
        // Leave the entire close/reopen burst and immediate catch-up unlogged.
        if q.last_completed
            .is_some_and(|at| at.elapsed() < Duration::from_secs(2))
        {
            return None;
        }
        q.reports.pop_front()
    });
    if let Some(r) = report {
        let s = r.snapshot;
        let _ = info!(
            log,
            "storage sd id={} op={} start_ticks={} end_ticks={} wait_us={} execution_us={} ok={} dir_calls={} dir_us={}",
            r.id,
            NAMES[r.op as usize],
            r.start.as_ticks(),
            r.end.as_ticks(),
            r.wait_us,
            r.execution_us,
            r.ok,
            s.directory_calls,
            s.directory_us
        );
        log_io(log, r.id, "directory", s.directory);
        log_io(log, r.id, "file", s.file);
    }
    let lost = LOST.swap(0, Ordering::Relaxed);
    if lost != 0 {
        let _ = info!(log, "storage sd diagnostics_lost={}", lost);
    }
}

fn log_io(log: Log, id: u32, phase: &str, io: Io) {
    if io.reads.calls != 0 {
        log_transfers(log, id, phase, "read", io.reads, 0);
    }
    if io.writes.calls != 0 {
        log_transfers(log, id, phase, "write", io.writes, io.sequential_writes_max);
    }
}
fn log_transfers(log: Log, id: u32, phase: &str, rw: &str, t: Transfers, seq: u32) {
    let _ = info!(
        log,
        "storage sd_io id={} phase={} rw={} calls={} sectors={} single={} err={} us={} poll_us={} max_us={} lba={} blocks={} max_poll_us={} seq={}",
        id,
        phase,
        rw,
        t.calls,
        t.sectors,
        t.singles,
        t.errors,
        t.wall_us,
        t.poll_us,
        t.max_us,
        t.max_lba,
        t.max_sectors,
        t.max_poll_us,
        seq
    );
}
