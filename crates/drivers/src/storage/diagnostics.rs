//! Opt-in observation of one serialized SD device. No allocation or logging.
//! Start a scope only while holding the filesystem's exclusive ownership lock.
use core::{cell::RefCell, future::Future};
use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, Mutex};
use embassy_time::Instant;

#[derive(Clone, Copy, Debug, Default)]
pub struct Transfers {
    pub calls: u32,
    pub sectors: u32,
    pub singles: u32,
    pub errors: u32,
    pub wall_us: u64,
    /// Wall time inside future polls; includes interrupts during a poll.
    /// Time outside polls includes hardware waiting AND executor scheduling.
    pub poll_us: u64,
    pub max_us: u32,
    pub max_lba: u32,
    pub max_sectors: u32,
    pub max_poll_us: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Io {
    pub reads: Transfers,
    pub writes: Transfers,
    /// Longest run of adjacent single-sector writes, allowing intervening reads.
    pub sequential_writes_max: u32,
    previous_write: Option<u32>,
    sequential_writes: u32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Snapshot {
    /// All commands outside the explicit directory-preparation call. Directory
    /// expansion within file creation is included here, not in `directory`.
    pub file: Io,
    pub directory: Io,
    pub directory_calls: u32,
    pub directory_us: u64,
}

#[derive(Clone, Copy)]
pub enum Direction {
    Read,
    Write,
}

struct State {
    generation: u32,
    active: bool,
    directory_depth: u32,
    snapshot: Snapshot,
}
static STATE: Mutex<CriticalSectionRawMutex, RefCell<State>> = Mutex::new(RefCell::new(State {
    generation: 0,
    active: false,
    directory_depth: 0,
    snapshot: Snapshot {
        file: Io {
            reads: ZERO,
            writes: ZERO,
            sequential_writes_max: 0,
            previous_write: None,
            sequential_writes: 0,
        },
        directory: Io {
            reads: ZERO,
            writes: ZERO,
            sequential_writes_max: 0,
            previous_write: None,
            sequential_writes: 0,
        },
        directory_calls: 0,
        directory_us: 0,
    },
}));
const ZERO: Transfers = Transfers {
    calls: 0,
    sectors: 0,
    singles: 0,
    errors: 0,
    wall_us: 0,
    poll_us: 0,
    max_us: 0,
    max_lba: 0,
    max_sectors: 0,
    max_poll_us: 0,
};

fn with_state<T>(f: impl FnOnce(&mut State) -> T) -> T {
    STATE.lock(|cell| f(&mut cell.borrow_mut()))
}

/// Cancellation-safe window; nesting is rejected rather than corrupting counts.
pub struct Scope {
    generation: u32,
}
impl Scope {
    pub fn begin() -> Option<Self> {
        with_state(|s| {
            if s.active {
                return None;
            }
            s.generation = s.generation.wrapping_add(1);
            s.active = true;
            s.directory_depth = 0;
            s.snapshot = Snapshot::default();
            Some(Self {
                generation: s.generation,
            })
        })
    }
    pub fn finish(self) -> Snapshot {
        with_state(|s| {
            if !s.active || s.generation != self.generation {
                return Snapshot::default();
            }
            s.active = false;
            s.snapshot
        })
    }
}
impl Drop for Scope {
    fn drop(&mut self) {
        with_state(|s| {
            if s.generation == self.generation {
                s.active = false;
            }
        });
    }
}
pub fn active() -> bool {
    with_state(|s| s.active)
}

/// Directory preparation may only be attributed inside an active operation.
pub(crate) struct DirectorySpan {
    generation: Option<u32>,
    started: Instant,
}
impl DirectorySpan {
    pub(crate) fn new() -> Self {
        let generation = with_state(|s| {
            if !s.active {
                return None;
            }
            s.directory_depth += 1;
            s.snapshot.directory_calls += 1;
            Some(s.generation)
        });
        Self {
            generation,
            started: Instant::now(),
        }
    }
}
impl Drop for DirectorySpan {
    fn drop(&mut self) {
        let elapsed = self.started.elapsed().as_micros();
        with_state(|s| {
            if self.generation == Some(s.generation) && s.active {
                s.directory_depth -= 1;
                if s.directory_depth == 0 {
                    s.snapshot.directory_us += elapsed;
                }
            }
        });
    }
}

fn record(
    generation: u32,
    directory: bool,
    direction: Direction,
    lba: u32,
    sectors: u32,
    elapsed: u64,
    poll_us: u64,
    max_poll_us: u32,
    failed: bool,
) {
    with_state(|s| {
        if !s.active || s.generation != generation {
            return;
        }
        let io = if directory {
            &mut s.snapshot.directory
        } else {
            &mut s.snapshot.file
        };
        let t = match direction {
            Direction::Read => &mut io.reads,
            Direction::Write => &mut io.writes,
        };
        t.calls = t.calls.saturating_add(1);
        t.sectors = t.sectors.saturating_add(sectors);
        t.singles = t.singles.saturating_add(u32::from(sectors == 1));
        t.errors = t.errors.saturating_add(u32::from(failed));
        t.wall_us = t.wall_us.saturating_add(elapsed);
        t.poll_us = t.poll_us.saturating_add(poll_us);
        t.max_poll_us = t.max_poll_us.max(max_poll_us);
        if t.calls == 1 || elapsed > u64::from(t.max_us) {
            t.max_us = elapsed.min(u64::from(u32::MAX)) as u32;
            t.max_lba = lba;
            t.max_sectors = sectors;
        }
        if matches!(direction, Direction::Write) {
            if sectors == 1 {
                io.sequential_writes =
                    if io.previous_write.and_then(|v| v.checked_add(1)) == Some(lba) {
                        io.sequential_writes.saturating_add(1)
                    } else {
                        1
                    };
                io.previous_write = Some(lba);
                io.sequential_writes_max = io.sequential_writes_max.max(io.sequential_writes);
            } else {
                io.previous_write = None;
                io.sequential_writes = 0;
            }
        }
    });
}

pub async fn measure<T, E>(
    direction: Direction,
    lba: u32,
    sectors: usize,
    future: impl Future<Output = Result<T, E>>,
) -> Result<T, E> {
    let context = with_state(|s| s.active.then_some((s.generation, s.directory_depth != 0)));
    let Some((generation, directory)) = context else {
        return future.await;
    };
    let started = Instant::now();
    let mut future = core::pin::pin!(future);
    let mut poll_us = 0u64;
    let mut max_poll_us = 0u32;
    let result = core::future::poll_fn(|cx| {
        let poll = Instant::now();
        let result = future.as_mut().poll(cx);
        let elapsed = poll.elapsed().as_micros();
        poll_us = poll_us.saturating_add(elapsed);
        max_poll_us = max_poll_us.max(elapsed.min(u64::from(u32::MAX)) as u32);
        result
    })
    .await;
    record(
        generation,
        directory,
        direction,
        lba,
        sectors as u32,
        started.elapsed().as_micros(),
        poll_us,
        max_poll_us,
        result.is_err(),
    );
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::task::{Context, Poll, Waker};

    #[test]
    fn attributes_directory_commands_preserves_file_counts_and_cleans_up_cancellation() {
        let scope = Scope::begin().unwrap();
        assert!(Scope::begin().is_none());
        let generation = scope.generation;
        record(generation, false, Direction::Read, 9, 4, 400, 20, 12, false);
        {
            let _directory = DirectorySpan::new();
            assert!(with_state(|s| s.directory_depth == 1));
            for lba in 100..356 {
                record(
                    generation,
                    true,
                    Direction::Write,
                    lba,
                    1,
                    2000,
                    15,
                    10,
                    false,
                );
            }
        }
        record(
            generation,
            false,
            Direction::Write,
            77,
            32,
            6000,
            200,
            150,
            true,
        );
        let snap = scope.finish();
        assert!(!active());
        assert_eq!(snap.directory_calls, 1);
        assert_eq!(snap.directory.writes.calls, 256);
        assert_eq!(snap.directory.writes.singles, 256);
        assert_eq!(snap.directory.sequential_writes_max, 256);
        assert_eq!(snap.directory.writes.wall_us, 512_000);
        assert_eq!(snap.file.reads.sectors, 4);
        assert_eq!(snap.file.writes.sectors, 32);
        assert_eq!(snap.file.writes.errors, 1);
        assert_eq!(snap.file.writes.max_lba, 77);

        // A pending SD future is cancelled while its operation scope is dropped.
        let scope = Scope::begin().unwrap();
        let mut pending = core::pin::pin!(measure(
            Direction::Read,
            88,
            1,
            core::future::pending::<Result<(), ()>>()
        ));
        let mut context = Context::from_waker(Waker::noop());
        assert!(pending.as_mut().poll(&mut context).is_pending());
        drop(scope);
        assert!(!active());
        let next = Scope::begin().unwrap();
        // Late completion from an old generation must not contaminate the next.
        record(
            generation,
            false,
            Direction::Read,
            99,
            1,
            9000,
            20,
            20,
            false,
        );
        let mut ready = core::pin::pin!(measure(
            Direction::Read,
            90,
            1,
            core::future::ready(Ok::<(), ()>(()))
        ));
        assert_eq!(ready.as_mut().poll(&mut context), Poll::Ready(Ok(())));
        let snap = next.finish();
        assert_eq!(snap.file.reads.calls, 1);
        assert_eq!(snap.file.reads.max_lba, 90);
    }
}
