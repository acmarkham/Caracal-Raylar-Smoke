use raylar_radio_service::link::{
    ChannelProfile, CodingRate, GfskReferenceRate, ProfileId, SpreadingFactor,
};
use raylar_radio_service::{
    Epoch, LinkError, NodeId, Rendezvous, RendezvousPurpose, ScheduleError,
};

pub const NETWORK_ID: u32 = 0x4141_5244;
pub const SCHEDULE_VERSION: u8 = 4;
pub const CONFIG_ID: &str = "aardwolf-uk-v4-20261008";
/// Board measurement used by Integration 002; recalibrate per hardware lot.
pub const HSE_MEASURED_ERROR_PPM: i32 = -9;
pub const EPOCH_US: i64 = 1_800_000_000;
pub const MINUTE_US: i64 = 60_000_000;
pub const ACTIVE_MINUTES: usize = 12;
pub const LOG_RESERVE_BYTES: u64 = 100_000_000;
pub const AUDIO_FILE_BYTES: u64 = 512 + 16_000 * 4 * 60;
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
