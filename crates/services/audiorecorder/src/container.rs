use crate::{RecordingMetadata, WavContainer};
use core::fmt::{self, Write};
use heapless::String;
use raylar_flac_encoder_service::{Config as FlacConfig, Encoder};

pub(crate) enum Container {
    Wav(WavContainer),
    Flac(Encoder),
}

impl Container {
    pub fn suffix(&self) -> &'static str {
        match self {
            Self::Wav(_) => "wav",
            Self::Flac(_) => "flac",
        }
    }
}

struct Optional<T>(Option<T>);
impl<T: fmt::Display> fmt::Display for Optional<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            Some(value) => value.fmt(f),
            None => f.write_str("unknown"),
        }
    }
}

/// A bounded COMMENT value preserves the recorder's metadata without truncation.
pub(crate) fn flac_comment(
    metadata: &RecordingMetadata,
    config: FlacConfig,
) -> Result<String<900>, fmt::Error> {
    let mut text = String::new();
    write!(text, "COMMENT=start_utc={}.{:06};system_timestamp={};node_id={};card_id={};device_id={};firmware_hash={};firmware_version={};boot_id={};calibration_ppb={};utc_status={};uncertainty_us={};calibration_locked={};last_pps_ticks={};bits_per_sample={};block_samples={};sample_rate={};channels=1",
        metadata.started_utc.seconds, metadata.started_utc.microseconds,
        Optional(metadata.started_system_ticks), Optional(metadata.node_id),
        metadata.card_id.as_ref().map(|s| s.as_str()).unwrap_or("unknown"),
        metadata.device_id, metadata.firmware_hash, metadata.firmware_version,
        Optional(metadata.boot_id), Optional(metadata.calibration_ppb),
        metadata.utc_status, metadata.uncertainty_us, metadata.calibration_locked as u8,
        Optional(metadata.last_pps_system_ticks), config.bits_per_sample,
        config.block_samples, config.sample_rate_hz)?;
    if let Some(utc) = metadata.last_gps_pps_utc {
        write!(
            text,
            ";last_gps_pps_utc={}.{:06}",
            utc.seconds, utc.microseconds
        )?;
    } else {
        text.push_str(";last_gps_pps_utc=unknown")
            .map_err(|_| fmt::Error)?;
    }
    if let (Some(lat), Some(lon)) = (metadata.latitude_e7, metadata.longitude_e7) {
        let hash = crate::wav::location_digest(lat, lon, metadata.node_id.unwrap_or(0));
        write!(
            text,
            ";location_hash={hash:016x};location_hash_algo=fnv1a64-raylar-location-v1-node-e7-le"
        )?;
    }
    Ok(text)
}
