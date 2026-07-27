#![no_std]

#[cfg(test)]
extern crate embassy_executor;
#[cfg(test)]
extern crate std;

mod metadata;
mod service;
mod storage;
mod wav;

pub use metadata::{MetadataSource, RecordingMetadata, TimeMetadataSource};
pub use service::{
    AudioRecorder, AudioRecorderConfig, AudioRecorderError, RecorderProgress,
    DEFAULT_ENCODE_BUFFER_BYTES,
};
pub use storage::RecordingStorage;
pub use wav::{WavContainer, WAV_HEADER_BYTES};
