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
    declared_file_seconds: u32,
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
        bytes_per_second
            .checked_mul(declared_file_seconds)
            .ok_or(WavError::DataSizeOverflow)?;
        Ok(Self {
            format,
            declared_file_seconds,
        })
    }

    pub fn header(&self, metadata: &RecordingMetadata) -> [u8; WAV_HEADER_BYTES] {
        let mut header = [0; WAV_HEADER_BYTES];
        let byte_rate = self.format.sample_rate_hz * u32::from(self.format.channels) * 4;
        let data_bytes = byte_rate * self.declared_file_seconds;

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
    let _ = write!(
        comment,
        "start_utc={}.{:06};device={};firmware={}",
        metadata.started_utc.seconds,
        metadata.started_utc.microseconds,
        metadata.device_id,
        metadata.firmware_version
    );
    if let (Some(latitude), Some(longitude)) = (metadata.latitude_e7, metadata.longitude_e7) {
        let _ = write!(comment, ";latitude_e7={latitude};longitude_e7={longitude}");
    }
    let bytes = comment.as_bytes();
    header[COMMENT_OFFSET..COMMENT_OFFSET + bytes.len()].copy_from_slice(bytes);
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
