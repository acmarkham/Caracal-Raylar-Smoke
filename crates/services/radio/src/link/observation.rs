use embassy_time::Instant;
use raylar_drivers::radio::GfskPacketStatus;

use crate::NodeId;

use super::ProfileId;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LinkOutcome {
    Received,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinkObservation {
    pub peer: NodeId,
    pub profile: ProfileId,
    pub rssi_dbm_x2: Option<i16>,
    pub snr_db_x4: Option<i16>,
    pub gfsk_status: Option<GfskPacketStatus>,
    pub outcome: LinkOutcome,
    pub observed_at: Instant,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PassiveLinkState {
    pub last_observation: Option<LinkObservation>,
    pub received_packets: u32,
    pub failed_packets: u32,
}

impl PassiveLinkState {
    pub fn observe(&mut self, observation: LinkObservation) {
        match observation.outcome {
            LinkOutcome::Received => {
                self.received_packets = self.received_packets.saturating_add(1)
            }
            LinkOutcome::Failed => self.failed_packets = self.failed_packets.saturating_add(1),
        }
        self.last_observation = Some(observation);
    }
}
