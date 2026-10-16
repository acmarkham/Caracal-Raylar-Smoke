#![no_std]

#[cfg(test)]
extern crate embassy_executor;
#[cfg(test)]
extern crate std;

mod buffers;
mod container;
mod metadata;
mod multichannel;
mod service;
mod storage;
mod wav;

pub use buffers::{BorrowedBuffers, OwnedBuffers, RecorderBuffers};
pub use metadata::{MetadataSource, RecordingMetadata, TimeMetadataSource};
pub use multichannel::{
    CaptureParameters, HexaphonicBuffers, HexaphonicConfig, HexaphonicRecorder, LifecycleMetrics,
    ReferenceFormat,
};
pub use raylar_crcl_encoder_service as crcl;
pub use raylar_flac_encoder_service::{
    Config as FlacConfig, DEFAULT_BLOCK_SAMPLES as FLAC_BLOCK_SAMPLES,
    Prediction as FlacPrediction, frame_capacity as flac_frame_capacity,
};
pub use service::{
    AudioRecorder, AudioRecorderConfig, AudioRecorderError, DEFAULT_ENCODE_BUFFER_BYTES,
    RecorderProgress,
};
pub use storage::RecordingStorage;
pub use wav::{WAV_HEADER_BYTES, WavContainer};
