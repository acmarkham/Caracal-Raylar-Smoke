//! L4 composition; the portable Aardwolf policy remains owned by the U5 app.
#![no_std]
pub mod audio_recovery;
pub mod config;
pub mod gnss_profile;
#[path = "../aardwolf/policy.rs"]
pub mod policy;
pub mod tone;
pub mod ui_policy;
pub const SAMPLE_RATE: u32 = 16_000;
pub const SEGMENT_SECONDS: u32 = 60;
pub const SEGMENT_SAMPLES: usize = SAMPLE_RATE as usize * SEGMENT_SECONDS as usize;
pub const CHANNELS: usize = if cfg!(feature = "mono") { 1 } else { 4 };
pub const SPATIAL_BLOCK: usize = 256;
pub const SPATIAL_ENCODE_BYTES: usize = 1536;
pub const FLAC_BLOCK: usize = if cfg!(feature = "mono") { 1024 } else { 256 };
pub const FLAC_ENCODE_BYTES: usize = FLAC_BLOCK * 4 + 32;
pub const VERIFY_BYTES: usize = 1024 + FLAC_BLOCK * 3 + 32;
pub const CODEC: &str = if cfg!(feature = "wav") { "wav" } else { "flac" };
pub const SOURCE_BITS: u8 = 24;
pub const FILE_BUDGET: u64 = if cfg!(feature = "wav") {
    FILE_BYTES
} else {
    1024 + SEGMENT_SAMPLES as u64 * 4 + 32 * (SEGMENT_SAMPLES as u64).div_ceil(FLAC_BLOCK as u64)
};
pub const PAYLOAD_BYTES: u64 = SAMPLE_RATE as u64 * SEGMENT_SECONDS as u64 * 4;
pub const FILE_BYTES: u64 = 512 + PAYLOAD_BYTES;
pub const BENCH_SEGMENTS: usize = if cfg!(feature = "bench-gps-cycle") {
    20
} else {
    2
};
pub const FNV_INITIAL: u32 = 0x811c9dc5;
pub fn hash_bytes(mut hash: u32, bytes: &[u8]) -> u32 {
    for byte in bytes {
        hash = (hash ^ *byte as u32).wrapping_mul(0x01000193);
    }
    hash
}

/// STREAMINFO is rewritten on close. Hash all other bytes with this region
/// normalized to zero; its final bytes have a separate recorded checksum.
pub fn recording_hash(mut hash: u32, offset: u64, bytes: &[u8]) -> u32 {
    for (n, byte) in bytes.iter().enumerate() {
        let value = if !cfg!(feature = "wav") && (8..42).contains(&(offset + n as u64)) {
            0
        } else {
            *byte
        };
        hash = (hash ^ u32::from(value)).wrapping_mul(0x01000193);
    }
    hash
}

pub fn valid_streaminfo(bytes: &[u8], samples: u64) -> bool {
    if bytes.len() != 34 {
        return false;
    }
    let properties = u64::from_be_bytes(bytes[10..18].try_into().unwrap());
    u16::from_be_bytes(bytes[..2].try_into().unwrap()) == FLAC_BLOCK as u16
        && u16::from_be_bytes(bytes[2..4].try_into().unwrap()) == FLAC_BLOCK as u16
        && properties >> 44 == u64::from(SAMPLE_RATE)
        && (properties >> 41) & 7 == 0
        && (properties >> 36) & 31 == u64::from(SOURCE_BITS - 1)
        && properties & ((1 << 36) - 1) == samples
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
        && h[56..504].windows(6).any(|v| v == b"L4Evo:")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn gps_uses_full_calibration_phase_qualification_and_retained_software_wake() {
        use embassy_time::Duration;
        use raylar_drivers::gps::{ublox, StartMode};
        let gps = config::gps(gnss_profile::Receiver::M10);
        assert!(gps.wait_for_frequency_calibration_lock);
        assert_eq!(gps.initial_start_mode, StartMode::Hot);
        assert_eq!(gps.gps_on_time, Duration::from_secs(60));
        assert_eq!(
            gps.gps_off_time,
            Duration::from_secs(if cfg!(feature = "bench-gps-cycle") {
                30
            } else {
                1800
            })
        );
        let phase = gps.phase_qualified_shutdown.unwrap();
        assert_eq!(phase.minimum_clean_pps_time, Duration::from_secs(120));
        assert_eq!(phase.maximum_on_time, Duration::from_secs(180));
        assert_eq!(
            (
                phase.residual_threshold_us,
                phase.uncertainty_threshold_us,
                phase.consecutive_anchors
            ),
            (250, 500, 5)
        );
        assert_eq!(gps.module_commands.standby, Some(ublox::STANDBY.as_slice()));
        assert_eq!(gps.module_commands.wake, Some(ublox::WAKE.as_slice()));
        assert!(
            gps.module_commands.hot_start.is_none()
                && gps.module_commands.warm_start.is_none()
                && gps.module_commands.cold_start.is_none()
        );
        for receiver in [gnss_profile::Receiver::M7, gnss_profile::Receiver::M8] {
            let legacy = config::gps(receiver);
            assert_eq!(legacy.module_commands.standby, Some(receiver.standby()));
            assert_eq!(legacy.module_commands.wake, gps.module_commands.wake);
            assert_eq!(legacy.gps_off_time, gps.gps_off_time);
            assert!(legacy.module_commands.hot_start.is_none());
            assert!(legacy.module_commands.warm_start.is_none());
            assert!(legacy.module_commands.cold_start.is_none());
        }
    }
    #[test]
    fn storage_gate_preserves_aardwolf_reserve_and_cluster_rounding() {
        let cluster = 32768;
        let needed =
            config::LOG_RESERVE_BYTES + config::AUDIO_FILE_BYTES.div_ceil(cluster) * cluster + 1024;
        assert!(!policy::can_start_audio(needed - 1, cluster as u32, 1024));
        assert!(policy::can_start_audio(needed, cluster as u32, 1024));
        assert!(!policy::can_start_audio(u64::MAX - 1, 1, u64::MAX - 1));
    }
    #[test]
    fn flac_streaminfo_requires_mono_24bit_exact_minute_and_block_size() {
        let mut info = [0u8; 34];
        info[..2].copy_from_slice(&(FLAC_BLOCK as u16).to_be_bytes());
        info[2..4].copy_from_slice(&(FLAC_BLOCK as u16).to_be_bytes());
        let properties = ((SAMPLE_RATE as u64) << 44) | (23u64 << 36) | SEGMENT_SAMPLES as u64;
        info[10..18].copy_from_slice(&properties.to_be_bytes());
        assert!(valid_streaminfo(&info, 960000));
        info[17] ^= 1;
        assert!(!valid_streaminfo(&info, 960000));
        info[17] ^= 1;
        info[13] ^= 0x10;
        assert!(!valid_streaminfo(&info, 960000));
    }
    #[test]
    fn utc_gate_remains_next_minute_and_full_minute_pcm() {
        assert_eq!(policy::next_minute(60_000_000), 120_000_000);
        assert_eq!(policy::next_minute(119_999_999), 120_000_000);
        assert_eq!(FILE_BYTES, 3_840_512);
        assert_eq!(PAYLOAD_BYTES % 512, 0);
    }
}
