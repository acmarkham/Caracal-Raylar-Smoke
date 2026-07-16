use core::cell::RefCell;
use core::fmt;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_time::Instant;
use heapless::{Deque, String};

use crate::format::TruncatingWriter;
use crate::service::{DEFAULT_MESSAGE_LENGTH, DEFAULT_QUEUE_DEPTH};
use crate::{LogLevel, LogOutcome, LoggingStats};

pub(crate) struct LogRecord<const MESSAGE_LENGTH: usize> {
    pub sequence: u64,
    pub timestamp: Instant,
    pub level: LogLevel,
    pub component: &'static str,
    pub message: String<MESSAGE_LENGTH>,
    pub message_truncated: bool,
}

struct LoggingState<const MESSAGE_LENGTH: usize, const QUEUE_DEPTH: usize> {
    queue: Deque<LogRecord<MESSAGE_LENGTH>, QUEUE_DEPTH>,
    next_sequence: u64,
    stats: LoggingStats,
}

impl<const MESSAGE_LENGTH: usize, const QUEUE_DEPTH: usize>
    LoggingState<MESSAGE_LENGTH, QUEUE_DEPTH>
{
    const fn new() -> Self {
        Self {
            queue: Deque::new(),
            next_sequence: 0,
            stats: LoggingStats {
                total_messages: 0,
                dropped_messages: 0,
                queue_depth: 0,
                maximum_queue_depth: 0,
                bytes_written: 0,
                truncated_messages: 0,
                write_failures: 0,
            },
        }
    }
}

pub struct LoggingResources<
    const MESSAGE_LENGTH: usize = DEFAULT_MESSAGE_LENGTH,
    const QUEUE_DEPTH: usize = DEFAULT_QUEUE_DEPTH,
> {
    state: Mutex<CriticalSectionRawMutex, RefCell<LoggingState<MESSAGE_LENGTH, QUEUE_DEPTH>>>,
    ready: Signal<CriticalSectionRawMutex, ()>,
}

impl<const MESSAGE_LENGTH: usize, const QUEUE_DEPTH: usize>
    LoggingResources<MESSAGE_LENGTH, QUEUE_DEPTH>
{
    pub const fn new() -> Self {
        Self {
            state: Mutex::new(RefCell::new(LoggingState::new())),
            ready: Signal::new(),
        }
    }

    pub const fn register(
        &self,
        component: &'static str,
    ) -> LoggerHandle<'_, MESSAGE_LENGTH, QUEUE_DEPTH> {
        LoggerHandle {
            component,
            resources: self,
        }
    }

    pub fn stats(&self) -> LoggingStats {
        self.with_state(|state| state.stats)
    }

    pub(crate) fn try_record(&self) -> Option<LogRecord<MESSAGE_LENGTH>> {
        self.with_state(|state| {
            let record = state.queue.pop_front();
            state.stats.queue_depth = state.queue.len();
            record
        })
    }

    pub(crate) async fn next_record(&self) -> LogRecord<MESSAGE_LENGTH> {
        loop {
            if let Some(record) = self.try_record() {
                return record;
            }
            self.ready.wait().await;
        }
    }

    pub(crate) fn record_write(&self, bytes: usize, line_truncated: bool, message_truncated: bool) {
        self.with_state(|state| {
            state.stats.bytes_written = state.stats.bytes_written.saturating_add(bytes as u64);
            if line_truncated && !message_truncated {
                state.stats.truncated_messages = state.stats.truncated_messages.saturating_add(1);
            }
        });
    }

    pub(crate) fn record_write_failure(&self) {
        self.with_state(|state| {
            state.stats.write_failures = state.stats.write_failures.saturating_add(1)
        });
    }

    fn enqueue(
        &self,
        component: &'static str,
        timestamp: Instant,
        level: LogLevel,
        message: String<MESSAGE_LENGTH>,
        message_truncated: bool,
    ) -> LogOutcome {
        let outcome = self.with_state(|state| {
            let sequence = state.next_sequence;
            state.next_sequence = state.next_sequence.wrapping_add(1);
            state.stats.total_messages = state.stats.total_messages.saturating_add(1);

            if state.queue.is_full() {
                state.stats.dropped_messages = state.stats.dropped_messages.saturating_add(1);
                return LogOutcome::DroppedQueueFull;
            }

            let _ = state.queue.push_back(LogRecord {
                sequence,
                timestamp,
                level,
                component,
                message,
                message_truncated,
            });
            state.stats.queue_depth = state.queue.len();
            state.stats.maximum_queue_depth =
                state.stats.maximum_queue_depth.max(state.stats.queue_depth);
            if message_truncated {
                state.stats.truncated_messages = state.stats.truncated_messages.saturating_add(1);
                LogOutcome::EnqueuedTruncated
            } else {
                LogOutcome::Enqueued
            }
        });
        if outcome != LogOutcome::DroppedQueueFull {
            self.ready.signal(());
        }
        outcome
    }

    fn with_state<R>(
        &self,
        f: impl FnOnce(&mut LoggingState<MESSAGE_LENGTH, QUEUE_DEPTH>) -> R,
    ) -> R {
        self.state.lock(|state| f(&mut state.borrow_mut()))
    }
}

impl<const MESSAGE_LENGTH: usize, const QUEUE_DEPTH: usize> Default
    for LoggingResources<MESSAGE_LENGTH, QUEUE_DEPTH>
{
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
pub struct LoggerHandle<'a, const MESSAGE_LENGTH: usize, const QUEUE_DEPTH: usize> {
    component: &'static str,
    resources: &'a LoggingResources<MESSAGE_LENGTH, QUEUE_DEPTH>,
}

impl<const MESSAGE_LENGTH: usize, const QUEUE_DEPTH: usize>
    LoggerHandle<'_, MESSAGE_LENGTH, QUEUE_DEPTH>
{
    pub fn component(&self) -> &'static str {
        self.component
    }

    pub fn log(&self, level: LogLevel, arguments: fmt::Arguments<'_>) -> LogOutcome {
        self.log_at(Instant::now(), level, arguments)
    }

    pub fn log_at(
        &self,
        timestamp: Instant,
        level: LogLevel,
        arguments: fmt::Arguments<'_>,
    ) -> LogOutcome {
        let mut message = String::new();
        let truncated = {
            let mut writer = TruncatingWriter::new(&mut message, MESSAGE_LENGTH);
            let _ = fmt::write(&mut writer, arguments);
            writer.truncated()
        };
        self.resources
            .enqueue(self.component, timestamp, level, message, truncated)
    }
}
