#![no_std]

pub const SAMPLE_RATE: u32 = 16_000;
pub const SEGMENT_SECONDS: u32 = 60;
pub const SEGMENT_SAMPLES: u32 = SAMPLE_RATE * SEGMENT_SECONDS;
pub const PAYLOAD_BYTES: u64 = SEGMENT_SAMPLES as u64 * 4;
pub const FILE_BYTES: u64 = 512 + PAYLOAD_BYTES;
pub const BENCH_SEGMENTS: usize = 3;
pub const FNV_INITIAL: u32 = 0x811c9dc5;
// Deliberately simulated; never interpreted as a GPS fix or current wall time.
pub const FAKE_BASE: i64 = 1_767_225_600; // 2026-01-01 00:00:00 UTC.

pub fn hash_bytes(mut hash: u32, bytes: &[u8]) -> u32 {
    for byte in bytes {
        hash = (hash ^ u32::from(*byte)).wrapping_mul(0x01000193);
    }
    hash
}

pub fn valid_minute_header(h: &[u8]) -> bool {
    h.len() == 512
        && &h[..4] == b"RIFF"
        && &h[8..12] == b"WAVE"
        && &h[12..16] == b"fmt "
        && &h[504..508] == b"data"
        && u32::from_le_bytes(h[4..8].try_into().unwrap()) == (FILE_BYTES - 8) as u32
        && u16::from_le_bytes(h[20..22].try_into().unwrap()) == 1
        && u16::from_le_bytes(h[22..24].try_into().unwrap()) == 1
        && u32::from_le_bytes(h[24..28].try_into().unwrap()) == SAMPLE_RATE
        && u32::from_le_bytes(h[28..32].try_into().unwrap()) == SAMPLE_RATE * 4
        && u16::from_le_bytes(h[32..34].try_into().unwrap()) == 4
        && u16::from_le_bytes(h[34..36].try_into().unwrap()) == 32
        && u32::from_le_bytes(h[508..512].try_into().unwrap()) == PAYLOAD_BYTES as u32
        && h[56..504].windows(8).any(|v| v == b"FAKE-GPS")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn minute_sample_budget_is_exact_and_sector_aligned() {
        assert_eq!(SEGMENT_SAMPLES, 960_000);
        assert_eq!(FILE_BYTES, 3_840_512);
        assert_eq!(PAYLOAD_BYTES % 512, 0);
        assert_eq!(FAKE_BASE % 3600, 0);
    }
    #[test]
    fn streaming_hash_preserves_chunk_independence_and_detects_loss() {
        let data = b"microphone PCM and independent syslog bytes";
        assert_eq!(
            hash_bytes(hash_bytes(FNV_INITIAL, &data[..7]), &data[7..]),
            hash_bytes(FNV_INITIAL, data)
        );
        assert_ne!(
            hash_bytes(FNV_INITIAL, data),
            hash_bytes(FNV_INITIAL, &data[1..])
        );
    }
    #[test]
    fn rejects_truncated_or_foreign_headers() {
        assert!(!valid_minute_header(&[]));
        assert!(!valid_minute_header(&[0; 512]));
    }
}
