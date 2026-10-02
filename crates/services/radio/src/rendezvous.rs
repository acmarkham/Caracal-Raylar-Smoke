use crate::{Epoch, NodeId, ScheduleError};

pub const RENDEZVOUS_ALGORITHM_VERSION: u8 = 1;
const FNV_OFFSET_BASIS: u64 = 0xCBF2_9CE4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01B3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum RendezvousPurpose {
    Heartbeat = 1,
    Presence = 2,
    PeerListen = 3,
    BroadcastListen = 4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rendezvous {
    pub network_id: u32,
    pub schedule_version: u8,
}

impl Rendezvous {
    pub const fn new(network_id: u32, schedule_version: u8) -> Self {
        Self {
            network_id,
            schedule_version,
        }
    }

    /// Return a stable slot using FNV-1a over an explicitly ordered byte
    /// sequence. Changing this function requires a new schedule version.
    pub fn slot(
        &self,
        purpose: RendezvousPurpose,
        node: NodeId,
        epoch: Epoch,
        occurrence: u8,
        slots_per_window: u32,
    ) -> Result<u32, ScheduleError> {
        if slots_per_window == 0 {
            return Err(ScheduleError::InvalidSchedule);
        }

        let mut hash = FNV_OFFSET_BASIS;
        hash_bytes(&mut hash, &self.network_id.to_be_bytes());
        hash_byte(&mut hash, RENDEZVOUS_ALGORITHM_VERSION);
        hash_byte(&mut hash, self.schedule_version);
        hash_byte(&mut hash, purpose as u8);
        hash_bytes(&mut hash, &node.0.to_be_bytes());
        hash_bytes(&mut hash, &epoch.0.to_be_bytes());
        hash_byte(&mut hash, occurrence);
        Ok((hash % u64::from(slots_per_window)) as u32)
    }
}

fn hash_byte(hash: &mut u64, byte: u8) {
    *hash ^= u64::from(byte);
    *hash = hash.wrapping_mul(FNV_PRIME);
}

fn hash_bytes(hash: &mut u64, bytes: &[u8]) {
    for byte in bytes {
        hash_byte(hash, *byte);
    }
}
