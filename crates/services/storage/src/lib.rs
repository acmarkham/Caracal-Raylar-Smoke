#![no_std]

#[cfg(test)]
extern crate std;

mod backend;
mod policy;
mod service;
mod types;

pub use backend::StorageBackend;
pub use service::{StorageService, UtcClock, DEFAULT_MAX_STREAMS};
pub use types::{
    AppendOutcome, RollingPolicy, StorageConfig, StorageServiceError, StreamHandle, StreamType,
};

/// Placeholder for the recording service's future WAV metadata implementation.
///
/// Audio remains opaque to storage; the recording service can populate and
/// append a header through this stable hook later.
pub const fn make_wavfile_header() -> &'static [u8] {
    &[]
}
