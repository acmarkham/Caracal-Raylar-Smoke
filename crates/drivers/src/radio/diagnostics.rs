//! Bounded diagnostics for the sole radio owner. Never logs or allocates in I/O.
//! Wall time inside polls includes interrupts; outside-poll time includes device
//! waits AND executor scheduling. Nested command/SPI/BUSY totals overlap.
use core::future::Future;

#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum Op {
    Initialize,
    Prepare,
    Transmit,
    Receive,
    Recover,
    Sleep,
    Wake,
}
impl Op {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Initialize => "init",
            Self::Prepare => "prepare",
            Self::Transmit => "tx",
            Self::Receive => "rx",
            Self::Recover => "recover",
            Self::Sleep => "sleep",
            Self::Wake => "wake",
        }
    }
}
#[derive(Clone, Copy, Debug)]
#[repr(usize)]
pub enum Kind {
    Command,
    Spi,
    Busy,
}
impl Kind {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Command => "command",
            Self::Spi => "spi",
            Self::Busy => "busy",
        }
    }
}

pub async fn operation<T, E>(
    op: Op,
    reason: &'static str,
    job: u32,
    future: impl Future<Output = Result<T, E>>,
    classify: fn(&E) -> &'static str,
) -> Result<T, E> {
    #[cfg(feature = "radio-diagnostics")]
    let mut span = enabled::Operation::begin(op, reason, job);
    #[cfg(not(feature = "radio-diagnostics"))]
    let _ = (op, reason, job, classify);
    #[cfg(feature = "radio-diagnostics")]
    let result = enabled::poll_measured(future, &mut span.poll).await;
    #[cfg(not(feature = "radio-diagnostics"))]
    let result = future.await;
    #[cfg(feature = "radio-diagnostics")]
    span.finish(result.as_ref().err().map(classify).unwrap_or("ok"));
    result
}

pub(crate) async fn stage<T, E>(
    kind: Kind,
    tag: u16,
    future: impl Future<Output = Result<T, E>>,
) -> Result<T, E> {
    #[cfg(feature = "radio-diagnostics")]
    let mut span = enabled::Stage::begin(kind, tag);
    #[cfg(not(feature = "radio-diagnostics"))]
    let _ = (kind, tag);
    #[cfg(feature = "radio-diagnostics")]
    let result = enabled::poll_measured(future, &mut span.poll).await;
    #[cfg(not(feature = "radio-diagnostics"))]
    let result = future.await;
    #[cfg(feature = "radio-diagnostics")]
    span.finish(result.is_err());
    result
}

pub fn driver_error(error: super::Error) {
    #[cfg(feature = "radio-diagnostics")]
    enabled::with(|s| {
        if let Some(active) = &mut s.active {
            active.driver_error = Some(error);
        }
    });
    #[cfg(not(feature = "radio-diagnostics"))]
    let _ = error;
}
pub fn state(mode: &'static str, job: u32) {
    #[cfg(feature = "radio-diagnostics")]
    enabled::state(mode, job);
    #[cfg(not(feature = "radio-diagnostics"))]
    let _ = (mode, job);
}
pub fn request(enabled: bool) {
    #[cfg(feature = "radio-diagnostics")]
    enabled::request(enabled);
    #[cfg(not(feature = "radio-diagnostics"))]
    let _ = enabled;
}

#[cfg(feature = "radio-diagnostics")]
pub use enabled::{pop, totals, Record, Report, Totals, Transfers};

#[cfg(feature = "radio-diagnostics")]
mod enabled {
    use super::*;
    use core::cell::RefCell;
    use embassy_sync::blocking_mutex::{raw::CriticalSectionRawMutex, Mutex};
    use embassy_time::Instant;

    #[derive(Clone, Copy, Debug, Default)]
    pub struct Polls {
        pub us: u64,
        pub max_us: u64,
        pub count: u32,
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Transfers {
        pub calls: u32,
        pub errors: u32,
        pub cancelled: u32,
        pub us: u64,
        pub poll_us: u64,
        pub max_poll_us: u64,
        pub max_us: u64,
        pub tag: u16,
        pub error_tag: u16,
        pub start_ticks: u64,
        pub end_ticks: u64,
    }
    #[derive(Clone, Copy, Debug)]
    pub struct Report {
        pub id: u32,
        pub op: Op,
        pub reason: &'static str,
        pub job: u32,
        pub start_ticks: u64,
        pub end_ticks: u64,
        pub us: u64,
        pub poll_us: u64,
        pub max_poll_us: u64,
        pub polls: u32,
        pub result: &'static str,
        pub driver_error: Option<super::super::Error>,
        pub io: [Transfers; 3],
    }
    #[derive(Clone, Copy, Debug)]
    pub enum Record {
        Operation(Report),
        State {
            ticks: u64,
            mode: &'static str,
            job: u32,
        },
        Request {
            ticks: u64,
            enabled: bool,
        },
    }
    #[derive(Clone, Copy, Debug, Default)]
    pub struct Totals {
        pub operations: [u32; 7],
        pub cancelled: u32,
        pub lost: u32,
    }
    pub(super) struct State {
        next_id: u32,
        pub(super) active: Option<Report>,
        queue: heapless::Deque<Record, 32>,
        totals: Totals,
        mode: Option<(&'static str, u32)>,
    }
    static STATE: Mutex<CriticalSectionRawMutex, RefCell<State>> =
        Mutex::new(RefCell::new(State {
            next_id: 0,
            active: None,
            queue: heapless::Deque::new(),
            totals: Totals {
                operations: [0; 7],
                cancelled: 0,
                lost: 0,
            },
            mode: None,
        }));
    pub(super) fn with<T>(f: impl FnOnce(&mut State) -> T) -> T {
        STATE.lock(|s| f(&mut s.borrow_mut()))
    }
    fn push(s: &mut State, record: Record) {
        if s.queue.push_back(record).is_err() {
            s.totals.lost = s.totals.lost.saturating_add(1);
        }
    }
    pub fn pop() -> Option<Record> {
        with(|s| s.queue.pop_front())
    }
    pub fn totals() -> Totals {
        with(|s| s.totals)
    }
    pub(super) fn state(mode: &'static str, job: u32) {
        with(|s| {
            if s.mode != Some((mode, job)) {
                s.mode = Some((mode, job));
                push(
                    s,
                    Record::State {
                        ticks: Instant::now().as_ticks(),
                        mode,
                        job,
                    },
                );
            }
        });
    }
    pub(super) fn request(enabled: bool) {
        with(|s| {
            push(
                s,
                Record::Request {
                    ticks: Instant::now().as_ticks(),
                    enabled,
                },
            )
        });
    }
    pub(super) async fn poll_measured<F: Future>(future: F, stats: &mut Polls) -> F::Output {
        let mut future = core::pin::pin!(future);
        core::future::poll_fn(|cx| {
            let start = Instant::now();
            let result = future.as_mut().poll(cx);
            let elapsed = start.elapsed().as_micros();
            stats.us = stats.us.saturating_add(elapsed);
            stats.max_us = stats.max_us.max(elapsed);
            stats.count = stats.count.saturating_add(1);
            result
        })
        .await
    }
    pub(super) struct Operation {
        id: Option<u32>,
        pub poll: Polls,
    }
    impl Operation {
        pub fn begin(op: Op, reason: &'static str, job: u32) -> Self {
            let id = with(|s| {
                s.totals.operations[op as usize] =
                    s.totals.operations[op as usize].saturating_add(1);
                if s.active.is_some() {
                    s.totals.lost = s.totals.lost.saturating_add(1);
                    return None;
                }
                s.next_id = s.next_id.wrapping_add(1);
                s.active = Some(Report {
                    id: s.next_id,
                    op,
                    reason,
                    job,
                    start_ticks: Instant::now().as_ticks(),
                    end_ticks: 0,
                    us: 0,
                    poll_us: 0,
                    max_poll_us: 0,
                    polls: 0,
                    result: "active",
                    driver_error: None,
                    io: [Transfers::default(); 3],
                });
                Some(s.next_id)
            });
            Self {
                id,
                poll: Polls::default(),
            }
        }
        pub fn finish(&mut self, result: &'static str) {
            let Some(id) = self.id.take() else {
                return;
            };
            with(|s| {
                if s.active.as_ref().map(|r| r.id) != Some(id) {
                    return;
                }
                let mut report = s.active.take().unwrap();
                let end = Instant::now();
                report.end_ticks = end.as_ticks();
                report.us = end
                    .saturating_duration_since(Instant::from_ticks(report.start_ticks))
                    .as_micros();
                report.poll_us = self.poll.us;
                report.max_poll_us = self.poll.max_us;
                report.polls = self.poll.count;
                report.result = result;
                if result == "cancelled" {
                    s.totals.cancelled = s.totals.cancelled.saturating_add(1);
                }
                push(s, Record::Operation(report));
            });
        }
    }
    impl Drop for Operation {
        fn drop(&mut self) {
            self.finish("cancelled");
        }
    }

    pub(super) struct Stage {
        id: Option<u32>,
        kind: Kind,
        tag: u16,
        start: Instant,
        pub poll: Polls,
    }
    impl Stage {
        pub fn begin(kind: Kind, tag: u16) -> Self {
            Self {
                id: with(|s| s.active.as_ref().map(|r| r.id)),
                kind,
                tag,
                start: Instant::now(),
                poll: Polls::default(),
            }
        }
        pub fn finish(&mut self, failed: bool) {
            self.record(failed, false);
        }
        fn record(&mut self, failed: bool, cancelled: bool) {
            let Some(id) = self.id.take() else {
                return;
            };
            let end = Instant::now();
            let elapsed = end.saturating_duration_since(self.start).as_micros();
            with(|s| {
                let Some(active) = &mut s.active else {
                    return;
                };
                if active.id != id {
                    return;
                }
                let t = &mut active.io[self.kind as usize];
                t.calls = t.calls.saturating_add(1);
                t.errors = t.errors.saturating_add(u32::from(failed));
                t.cancelled = t.cancelled.saturating_add(u32::from(cancelled));
                if failed || cancelled {
                    t.error_tag = self.tag;
                }
                t.us = t.us.saturating_add(elapsed);
                t.poll_us = t.poll_us.saturating_add(self.poll.us);
                t.max_poll_us = t.max_poll_us.max(self.poll.max_us);
                if t.calls == 1 || elapsed > t.max_us {
                    t.max_us = elapsed;
                    t.tag = self.tag;
                    t.start_ticks = self.start.as_ticks();
                    t.end_ticks = end.as_ticks();
                }
            });
        }
    }
    impl Drop for Stage {
        fn drop(&mut self) {
            self.record(false, true);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use core::task::{Context, Poll, Waker};
        #[test]
        fn records_ready_errors_cancellation_stale_stages_and_overflow() {
            with(|s| {
                s.queue.clear();
                s.active = None;
                s.totals = Totals::default();
                s.mode = None;
            });
            let mut cx = Context::from_waker(Waker::noop());
            {
                let mut future = core::pin::pin!(operation(
                    Op::Initialize,
                    "startup",
                    0,
                    async {
                        driver_error(super::super::super::Error::BusyTimeout);
                        stage(
                            Kind::Command,
                            0x010f,
                            core::future::ready(Err::<(), ()>(())),
                        )
                        .await
                    },
                    |_| "driver"
                ));
                assert_eq!(future.as_mut().poll(&mut cx), Poll::Ready(Err(())));
            }
            let Some(Record::Operation(r)) = pop() else {
                panic!()
            };
            assert_eq!(r.result, "driver");
            assert_eq!(r.io[0].tag, 0x010f);
            assert_eq!(r.io[0].errors, 1);
            assert_eq!(r.io[0].error_tag, 0x010f);
            assert_eq!(
                r.driver_error,
                Some(super::super::super::Error::BusyTimeout)
            );
            assert_eq!(r.polls, 1);
            {
                let mut future = core::pin::pin!(operation(
                    Op::Sleep,
                    "disabled",
                    0,
                    stage(
                        Kind::Command,
                        0x011b,
                        stage(Kind::Busy, 0, core::future::pending::<Result<(), ()>>())
                    ),
                    |_| "driver"
                ));
                assert!(future.as_mut().poll(&mut cx).is_pending());
            }
            let Some(Record::Operation(r)) = pop() else {
                panic!()
            };
            assert_eq!(r.result, "cancelled");
            assert_eq!(r.io[2].cancelled, 1);
            assert_eq!(r.io[0].cancelled, 1);
            assert_eq!(r.io[0].error_tag, 0x011b);
            let mut first = Operation::begin(Op::Prepare, "job", 42);
            let mut stale = Stage::begin(Kind::Spi, 3);
            first.finish("ok");
            let mut next = Operation::begin(Op::Prepare, "job", 43);
            stale.finish(false);
            next.finish("ok");
            let _ = pop();
            let Some(Record::Operation(r)) = pop() else {
                panic!()
            };
            assert_eq!(r.io[1].calls, 0);
            state("Idle", 0);
            state("Idle", 0);
            state("Preparing", 42);
            request(false);
            assert!(matches!(pop(), Some(Record::State { mode: "Idle", .. })));
            assert!(matches!(
                pop(),
                Some(Record::State {
                    mode: "Preparing",
                    job: 42,
                    ..
                })
            ));
            assert!(matches!(
                pop(),
                Some(Record::Request { enabled: false, .. })
            ));
            assert!(pop().is_none());
            for _ in 0..40 {
                let mut op = Operation::begin(Op::Initialize, "retry", 0);
                op.finish("ok");
            }
            assert_eq!(totals().lost, 8);
            assert_eq!(totals().operations[Op::Initialize as usize], 41);
            assert_eq!(totals().cancelled, 1);
            while pop().is_some() {}
        }
    }
}
