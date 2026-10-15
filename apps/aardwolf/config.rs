use raylar_flac_encoder_service::{Config as FlacConfig, Prediction as FlacPrediction};
use raylar_radio_service::link::{
    ChannelProfile, CodingRate, GfskReferenceRate, ProfileId, SpreadingFactor,
};
use raylar_radio_service::{
    Epoch, LinkError, NodeId, Rendezvous, RendezvousPurpose, ScheduleError,
};

pub const NETWORK_ID: u32 = 0x4141_5244;
pub const SCHEDULE_VERSION: u8 = 4;
pub const CONFIG_ID: &str = "aardwolf-uk-v4-20261013-flac1";
/// Board measurement used by Integration 002; recalibrate per hardware lot.
pub const HSE_MEASURED_ERROR_PPM: i32 = -9;
pub const EPOCH_US: i64 = 1_800_000_000;
pub const MINUTE_US: i64 = 60_000_000;
pub const ACTIVE_MINUTES: usize = 12;
pub const LOG_RESERVE_BYTES: u64 = 100_000_000;
pub const FLAC_BLOCK_SAMPLES: usize = 2048;
pub const AUDIO_CHANNELS: u8 = if cfg!(feature = "hexaphonic") { 6 } else { 1 };
pub const CRCL_BLOCK_FRAMES: usize = 1024;
pub const CRCL_BITS: u8 = 10;
/// Absolute discard in unpacked microphone PCM units; MIC1 is untouched.
/// See docs/audio/multichannel/analysis001.md for the measured noise margin.
pub const CRCL_MIN_SHIFT: u8 = 6;
#[cfg(feature = "firmware")]
pub const CRCL_CONFIG: raylar_audio_recorder_service::crcl::Config = {
    use raylar_audio_recorder_service::crcl::{Algorithm, Config, Prediction};
    Config {
        algorithm: if cfg!(feature = "cheap-crcl") {
            Algorithm::TemporalShiftV2
        } else {
            Algorithm::SpatialV1
        },
        prediction: if cfg!(feature = "cheap-crcl") {
            Prediction::TemporalRice
        } else {
            Prediction::DirectRice
        },
        block_frames: CRCL_BLOCK_FRAMES,
        bits: CRCL_BITS,
        min_shift: if cfg!(feature = "cheap-crcl") {
            CRCL_MIN_SHIFT
        } else {
            0
        },
        max_abs_pcm: 1 << 23,
    }
};
/// Measured low-CPU profile: exact Rice search, one fixed predictor order.
pub const FLAC_CONFIG: FlacConfig = FlacConfig {
    sample_rate_hz: 16_000,
    bits_per_sample: 32,
    block_samples: FLAC_BLOCK_SAMPLES,
    prediction: FlacPrediction::Fixed(1),
};
/// Worst-case verbatim frames plus the bounded native metadata header.
pub const AUDIO_FILE_BYTES: u64 = 1024
    + 16_000 * 4 * 61
    + 32 * (16_000u64 * 61).div_ceil(FLAC_BLOCK_SAMPLES as u64)
    + if cfg!(feature = "hexaphonic") {
        // Conservative packed-v1 or bounded estimated-Rice-v2 payload plus metadata.
        4096 + 16_000
            * 61
            * 5
            * (CRCL_BITS as u64 + if cfg!(feature = "cheap-crcl") { 2 } else { 0 })
            / 8
            + 96 * (16_000u64 * 61).div_ceil(CRCL_BLOCK_FRAMES as u64)
            + 548 * 60
            + 92
    } else {
        0
    };
pub const LOG_BUDGET_BYTES: u64 = 1_000_000_000;
pub const SYNC_WORD: [u8; 4] = NETWORK_ID.to_be_bytes();

pub fn profile(index: usize) -> Result<ChannelProfile, LinkError> {
    use SpreadingFactor::*;
    let id = ProfileId((index + 1) as u8);
    if (8..=10).contains(&index) {
        let rate = [
            GfskReferenceRate::Bps250_000,
            GfskReferenceRate::Bps38_400,
            GfskReferenceRate::Bps4_800,
        ][index - 8];
        return ChannelProfile::gfsk_reference_2_4(id, 2_441_000_000, rate, 13, &SYNC_WORD);
    }
    let sf = match index {
        0 | 4 => Sf7,
        1 | 5 => Sf8,
        2 | 6 => Sf10,
        3 | 7 | 11 => Sf12,
        _ => return Err(LinkError::InvalidProfile),
    };
    let (hz, bw, dbm) = if index < 4 {
        (868_100_000, 125_000, 14)
    } else {
        (
            2_441_000_000,
            if index == 11 { 203_000 } else { 812_000 },
            13,
        )
    };
    ChannelProfile::lora(id, hz, sf, bw, CodingRate::Cr4_5, dbm, 0x12)
}

pub const fn slot_seconds(index: usize) -> u64 {
    if index == 3 || index == 11 {
        2
    } else {
        1
    }
}

pub fn slot(node: NodeId, epoch: u64, index: usize) -> Result<u32, ScheduleError> {
    let rendezvous = Rendezvous::new(NETWORK_ID, SCHEDULE_VERSION);
    if slot_seconds(index) == 2 {
        rendezvous.permuted_slot::<29>(
            RendezvousPurpose::Heartbeat,
            node,
            Epoch(epoch),
            index as u8,
        )
    } else {
        rendezvous.permuted_slot::<59>(
            RendezvousPurpose::Heartbeat,
            node,
            Epoch(epoch),
            index as u8,
        )
    }
}

/// Rounded upward; includes the complete PHY packet but not setup/ramp.
pub fn airtime_us(index: usize) -> u64 {
    if (8..=10).contains(&index) {
        return 216_000_000u64.div_ceil([250_000, 38_400, 4_800][index - 8]);
    }
    let sf = match index {
        0 | 4 => 7,
        1 | 5 => 8,
        2 | 6 => 10,
        _ => 12,
    };
    let bw = if index < 4 {
        125_000u64
    } else if index == 11 {
        203_000
    } else {
        812_000
    };
    let de = u64::from((1u64 << sf) * 1_000_000 >= 16_000 * bw);
    let symbols = 8 + 5 * (128u64 - 4 * sf + 44).div_ceil(4 * (sf - 2 * de));
    ((12 * 4 + 17 + symbols * 4) * (1 << sf) * 1_000_000).div_ceil(4 * bw)
}
