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

        let hash = self.seed(purpose, node, epoch, occurrence);
        Ok((hash % u64::from(slots_per_window)) as u32)
    }

    /// Return one element of a deterministic per-node permutation.
    ///
    /// A block contains `SLOT_COUNT` epochs. Within each block every slot is
    /// selected exactly once, and the permutation is independently derived
    /// for each network, schedule version, purpose, node, occurrence, and
    /// block. This avoids the short low-bit cycles produced by hashing every
    /// epoch independently and then reducing modulo a small slot count.
    pub fn permuted_slot<const SLOT_COUNT: usize>(
        &self,
        purpose: RendezvousPurpose,
        node: NodeId,
        epoch: Epoch,
        occurrence: u8,
    ) -> Result<u32, ScheduleError> {
        let slot_count = u64::try_from(SLOT_COUNT).map_err(|_| ScheduleError::InvalidSchedule)?;
        if slot_count == 0 || slot_count > u64::from(u32::MAX) {
            return Err(ScheduleError::InvalidSchedule);
        }

        let block = Epoch(epoch.0 / slot_count);
        let index = (epoch.0 % slot_count) as usize;
        let mut permutation = [0u32; SLOT_COUNT];
        for (slot, value) in permutation.iter_mut().enumerate() {
            *value = slot as u32;
        }

        let mut random_state = self.seed(purpose, node, block, occurrence);
        for upper in (1..SLOT_COUNT).rev() {
            let selected = uniform_below(&mut random_state, (upper + 1) as u64) as usize;
            permutation.swap(upper, selected);
        }
        Ok(permutation[index])
    }

    fn seed(&self, purpose: RendezvousPurpose, node: NodeId, epoch: Epoch, occurrence: u8) -> u64 {
        let mut hash = FNV_OFFSET_BASIS;
        hash_bytes(&mut hash, &self.network_id.to_be_bytes());
        hash_byte(&mut hash, RENDEZVOUS_ALGORITHM_VERSION);
        hash_byte(&mut hash, self.schedule_version);
        hash_byte(&mut hash, purpose as u8);
        hash_bytes(&mut hash, &node.0.to_be_bytes());
        hash_bytes(&mut hash, &epoch.0.to_be_bytes());
        hash_byte(&mut hash, occurrence);
        hash
    }
}

fn uniform_below(state: &mut u64, bound: u64) -> u64 {
    let rejection_threshold = bound.wrapping_neg() % bound;
    loop {
        let candidate = splitmix64(state);
        if candidate >= rejection_threshold {
            return candidate % bound;
        }
    }
}

fn splitmix64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
    let mut value = *state;
    value = (value ^ (value >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    value = (value ^ (value >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    value ^ (value >> 31)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permutation_visits_every_slot_once_per_block() {
        let rendezvous = Rendezvous::new(0x4954_0004, 3);
        for purpose in [RendezvousPurpose::Presence, RendezvousPurpose::Heartbeat] {
            let mut seen = [false; 20];
            for epoch in 40..60 {
                let slot = rendezvous
                    .permuted_slot::<20>(purpose, NodeId(0xAABB_CCDD), Epoch(epoch), 0)
                    .unwrap() as usize;
                assert!(!seen[slot]);
                seen[slot] = true;
            }
            assert!(seen.into_iter().all(|value| value));
        }
    }

    #[test]
    fn permutation_is_deterministic_and_changes_between_blocks() {
        let rendezvous = Rendezvous::new(0x4954_0004, 3);
        let block = |first_epoch| {
            core::array::from_fn::<_, 20, _>(|offset| {
                rendezvous
                    .permuted_slot::<20>(
                        RendezvousPurpose::Presence,
                        NodeId(0xAABB_CCDD),
                        Epoch(first_epoch + offset as u64),
                        0,
                    )
                    .unwrap()
            })
        };
        assert_eq!(block(0), block(0));
        assert_ne!(block(0), block(20));
    }
}
