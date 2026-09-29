#![no_std]

#[cfg(test)]
extern crate std;

mod backend;
mod policy;
mod service;
mod types;

pub use backend::StorageBackend;
pub use service::{StorageService, UtcClock, DEFAULT_MAX_STREAMS, DEFAULT_WRITE_BUFFER_BYTES};
pub use types::{StorageLayout, StorageServiceError, StreamHandle, StreamKind};
