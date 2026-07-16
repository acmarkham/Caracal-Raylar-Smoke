#![no_std]
//! Heapless, ordered diagnostic logging for Raylar firmware components.
//!
//! Components keep a [`LoggerHandle`] and enqueue with [`info!`], [`warn!`],
//! and the other level macros. A [`LoggingService`] drains the bounded queue
//! into a [`LogSink`]; [`StorageLogSink`] connects it to the storage service's
//! single log stream.

#[cfg(test)]
extern crate std;

mod format;
mod resources;
mod service;
mod sink;
mod types;

pub use resources::{LoggerHandle, LoggingResources};
pub use service::{
    LoggingService, DEFAULT_LINE_LENGTH, DEFAULT_MESSAGE_LENGTH, DEFAULT_QUEUE_DEPTH,
};
pub use sink::{LogSink, StorageLogSink, StorageLogSinkError};
pub use types::{LogLevel, LogOutcome, LoggingStats, ProcessOutcome};

#[macro_export]
macro_rules! trace {
    ($logger:expr, $($arg:tt)*) => {
        ($logger).log($crate::LogLevel::Trace, core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! debug {
    ($logger:expr, $($arg:tt)*) => {
        ($logger).log($crate::LogLevel::Debug, core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($logger:expr, $($arg:tt)*) => {
        ($logger).log($crate::LogLevel::Info, core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! warn {
    ($logger:expr, $($arg:tt)*) => {
        ($logger).log($crate::LogLevel::Warn, core::format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! error {
    ($logger:expr, $($arg:tt)*) => {
        ($logger).log($crate::LogLevel::Error, core::format_args!($($arg)*))
    };
}
