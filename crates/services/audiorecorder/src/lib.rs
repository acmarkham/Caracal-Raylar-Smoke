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
    CaptureParameters, HexaphonicBuffers, HexaphonicConfig, HexaphonicRecorder, ReferenceFormat,
};
pub use raylar_crcl_encoder_service as crcl;
pub use raylar_flac_encoder_service::{
    frame_capacity as flac_frame_capacity, Config as FlacConfig, Prediction as FlacPrediction,
    DEFAULT_BLOCK_SAMPLES as FLAC_BLOCK_SAMPLES,
};
pub use service::{
    AudioRecorder, AudioRecorderConfig, AudioRecorderError, RecorderProgress,
    DEFAULT_ENCODE_BUFFER_BYTES,
};
pub use storage::RecordingStorage;
pub use wav::{WavContainer, WAV_HEADER_BYTES};
