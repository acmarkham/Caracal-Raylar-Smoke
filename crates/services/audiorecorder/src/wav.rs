use core::fmt::Write;

use heapless::String;
use raylar_audiosource::AudioFormat;

use crate::RecordingMetadata;

pub const WAV_HEADER_BYTES: usize = 512;
const COMMENT_OFFSET: usize = 56;
const COMMENT_BYTES: usize = 448;
const DATA_HEADER_OFFSET: usize = 504;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum WavError {
    InvalidFormat,
    DataSizeOverflow,
}

/// PCM WAV header generator owned by the recorder service.
pub struct WavContainer {
    format: AudioFormat,
    declared_data_bytes: u32,
}

impl WavContainer {
    pub fn new(format: AudioFormat, declared_file_seconds: u32) -> Result<Self, WavError> {
        if format.sample_rate_hz == 0 || format.channels == 0 || declared_file_seconds == 0 {
            return Err(WavError::InvalidFormat);
        }
        let bytes_per_second = format
            .sample_rate_hz
            .checked_mul(u32::from(format.channels))
            .and_then(|value| value.checked_mul(4))
            .ok_or(WavError::DataSizeOverflow)?;
        let declared_data_bytes = bytes_per_second
            .checked_mul(declared_file_seconds)
            .ok_or(WavError::DataSizeOverflow)?;
        Ok(Self {
            format,
            declared_data_bytes,
        })
    }

    pub fn header(&self, metadata: &RecordingMetadata) -> [u8; WAV_HEADER_BYTES] {
        self.header_with_data_bytes(metadata, self.declared_data_bytes)
    }

    pub(crate) const fn declared_file_seconds(&self) -> u32 {
        let bytes_per_second = self.format.sample_rate_hz * self.format.channels as u32 * 4;
        self.declared_data_bytes / bytes_per_second
    }

    pub fn header_for_samples(
        &self,
        metadata: &RecordingMetadata,
        interleaved_samples: usize,
    ) -> Result<[u8; WAV_HEADER_BYTES], WavError> {
        if !interleaved_samples.is_multiple_of(usize::from(self.format.channels)) {
            return Err(WavError::InvalidFormat);
        }
        let data_bytes = u32::try_from(interleaved_samples)
            .ok()
            .and_then(|samples| samples.checked_mul(4))
            .filter(|bytes| *bytes <= self.declared_data_bytes)
            .ok_or(WavError::DataSizeOverflow)?;
        Ok(self.header_with_data_bytes(metadata, data_bytes))
    }

    fn header_with_data_bytes(
        &self,
        metadata: &RecordingMetadata,
        data_bytes: u32,
    ) -> [u8; WAV_HEADER_BYTES] {
        let mut header = [0; WAV_HEADER_BYTES];
        let byte_rate = self.format.sample_rate_hz * u32::from(self.format.channels) * 4;

        put_id(&mut header, 0, b"RIFF");
        put_u32(
            &mut header,
            4,
            (WAV_HEADER_BYTES as u32 - 8).saturating_add(data_bytes),
        );
        put_id(&mut header, 8, b"WAVE");
        put_id(&mut header, 12, b"fmt ");
        put_u32(&mut header, 16, 16);
        put_u16(&mut header, 20, 1);
        put_u16(&mut header, 22, u16::from(self.format.channels));
        put_u32(&mut header, 24, self.format.sample_rate_hz);
        put_u32(&mut header, 28, byte_rate);
        put_u16(&mut header, 32, u16::from(self.format.channels) * 4);
        put_u16(&mut header, 34, 32);

        put_id(&mut header, 36, b"LIST");
        put_u32(&mut header, 40, 460);
        put_id(&mut header, 44, b"INFO");
        put_id(&mut header, 48, b"ICMT");
        put_u32(&mut header, 52, COMMENT_BYTES as u32);
        write_comment(&mut header, metadata);

        put_id(&mut header, DATA_HEADER_OFFSET, b"data");
        put_u32(&mut header, DATA_HEADER_OFFSET + 4, data_bytes);
        header
    }
}

fn write_comment(header: &mut [u8; WAV_HEADER_BYTES], metadata: &RecordingMetadata) {
    let mut comment = String::<COMMENT_BYTES>::new();
    let pps_utc = metadata.last_gps_pps_utc.map(format_utc);
    let calibration_ppb = metadata.calibration_ppb.map(format_ppb);
    let _ = write!(
        comment,
        "start_utc={}.{:06};system_timestamp={};node_id={};card_id={};firmware_hash={};boot_id={};last_gps_pps_utc={};calibration_ppb={}",
        metadata.started_utc.seconds,
        metadata.started_utc.microseconds,
        optional_u64(metadata.started_system_ticks),
        optional_u64(metadata.node_id.map(u64::from)),
        metadata.card_id.as_ref().map(|v| v.as_str()).unwrap_or("unknown"),
        if metadata.firmware_hash.is_empty() { metadata.firmware_version.as_str() } else { metadata.firmware_hash.as_str() },
        optional_u64(metadata.boot_id.map(u64::from)),
        pps_utc.as_ref().map(|v| v.as_str()).unwrap_or("unknown"),
        calibration_ppb.as_ref().map(|v| v.as_str()).unwrap_or("unknown")
    );
    if let (Some(latitude), Some(longitude)) = (metadata.latitude_e7, metadata.longitude_e7) {
        let digest = location_digest(latitude, longitude, metadata.node_id.unwrap_or(0));
        let _ = write!(comment, ";location_hash={digest:016x}");
    }
    let _ = write!(comment, ";utc_status={};uncertainty_us={};calibration_locked={};last_pps_ticks={};location_hash_algo=fnv1a64-raylar-location-v1-node-e7-le",
        metadata.utc_status, metadata.uncertainty_us, metadata.calibration_locked as u8, optional_u64(metadata.last_pps_system_ticks));
    let bytes = comment.as_bytes();
    header[COMMENT_OFFSET..COMMENT_OFFSET + bytes.len()].copy_from_slice(bytes);
}

fn optional_u64(value: Option<u64>) -> heapless::String<20> {
    let mut output = heapless::String::new();
    if let Some(value) = value { let _ = write!(output, "{value}"); } else { let _ = output.push_str("unknown"); }
    output
}

fn format_utc(value: raylar_time_service::UtcTimestamp) -> heapless::String<32> {
    let mut output = heapless::String::new();
    let _ = write!(output, "{}.{:06}", value.seconds, value.microseconds);
    output
}

fn format_ppb(value: i64) -> heapless::String<24> {
    let mut output = heapless::String::new();
    let _ = write!(output, "{value}");
    output
}

// FNV-1a over a domain tag, NodeID and signed E7 coordinates. This obscures
// coordinates from casual inspection but is not cryptographic protection.
fn location_digest(latitude_e7: i32, longitude_e7: i32, node_id: u32) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    let build_salt = option_env!("RAYLAR_LOCATION_HASH_SALT").unwrap_or("raylar-public-salt");
    for byte in b"raylar-location-v1".iter()
        .chain(build_salt.as_bytes().iter())
        .chain(node_id.to_le_bytes().iter())
        .chain(latitude_e7.to_le_bytes().iter())
        .chain(longitude_e7.to_le_bytes().iter())
    {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn put_id(target: &mut [u8], offset: usize, id: &[u8; 4]) {
    target[offset..offset + 4].copy_from_slice(id);
}

fn put_u16(target: &mut [u8], offset: usize, value: u16) {
    target[offset..offset + 2].copy_from_slice(&value.to_le_bytes());
}

fn put_u32(target: &mut [u8], offset: usize, value: u32) {
    target[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use raylar_time_service::UtcTimestamp;

    #[test]
    fn header_is_one_block_and_describes_mono_16khz_pcm() {
        let container = WavContainer::new(AudioFormat::new(16_000, 1, 1_000_000), 60).unwrap();
        let metadata = RecordingMetadata::new(UtcTimestamp::new(1_700_000_000, 123_456).unwrap());
        let header = container.header(&metadata);

        assert_eq!(header.len(), 512);
        assert_eq!(&header[0..4], b"RIFF");
        assert_eq!(&header[8..12], b"WAVE");
        assert_eq!(&header[12..16], b"fmt ");
        assert_eq!(u16::from_le_bytes([header[22], header[23]]), 1);
        assert_eq!(
            u32::from_le_bytes(header[24..28].try_into().unwrap()),
            16_000
        );
        assert_eq!(
            u32::from_le_bytes(header[28..32].try_into().unwrap()),
            64_000
        );
        assert_eq!(u16::from_le_bytes([header[34], header[35]]), 32);
        assert_eq!(&header[504..508], b"data");
        assert_eq!(
            u32::from_le_bytes(header[508..512].try_into().unwrap()),
            3_840_000
        );
        assert!(header[56..].starts_with(b"start_utc=1700000000.123456"));
    }
}
