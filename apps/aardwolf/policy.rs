use crate::config::{AUDIO_FILE_BYTES, EPOCH_US, LOG_RESERVE_BYTES};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Activity {
    #[default]
    EnergyRecovery,
    Active,
}

impl Activity {
    pub fn update(&mut self, soc: Option<u8>) -> bool {
        let previous = *self;
        match (previous, soc) {
            (Self::EnergyRecovery, Some(21..=100)) => *self = Self::Active,
            (Self::Active, Some(0..=9)) => *self = Self::EnergyRecovery,
            _ => {}
        }
        previous != *self
    }
    pub const fn active(self) -> bool {
        matches!(self, Self::Active)
    }
}

pub fn next_epoch(utc_us: i64) -> i64 {
    (utc_us.div_euclid(EPOCH_US) + 1) * EPOCH_US
}
pub fn next_minute(utc_us: i64) -> i64 {
    (utc_us.div_euclid(60_000_000) + 1) * 60_000_000
}

/// Include pending buffered writes and cluster rounding, not only PCM bytes.
pub fn can_start_audio(free: u64, cluster: u32, pending: u64) -> bool {
    let cluster = u64::from(cluster.max(1));
    let allocation = AUDIO_FILE_BYTES.div_ceil(cluster).saturating_mul(cluster);
    free >= LOG_RESERVE_BYTES
        .saturating_add(allocation)
        .saturating_add(pending)
}
