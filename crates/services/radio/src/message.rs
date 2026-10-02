use heapless::Vec;
use raylar_time_service::UtcTimestamp;

use crate::{GroupId, NodeId};

pub const MAX_MESSAGE_LEN: usize = 192;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageError {
    PayloadTooLarge,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MessageId(pub u32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    pub kind: u16,
    pub payload: Vec<u8, MAX_MESSAGE_LEN>,
}

impl Message {
    pub fn new(kind: u16, payload: &[u8]) -> Result<Self, MessageError> {
        let mut bytes = Vec::new();
        bytes
            .extend_from_slice(payload)
            .map_err(|_| MessageError::PayloadTooLarge)?;
        Ok(Self {
            kind,
            payload: bytes,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Destination {
    Gateway,
    Node(NodeId),
    Broadcast,
    Group(GroupId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MessageClass {
    Control,
    Presence,
    Telemetry,
    ReliableData,
    Collaborative,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum MessagePriority {
    BestEffort,
    ReliableData,
    Control,
    CriticalControl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Reliability {
    Unreliable,
    HopByHop { maximum_attempts: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MessageOptions {
    pub destination: Destination,
    pub class: MessageClass,
    pub priority: MessagePriority,
    pub reliability: Reliability,
    pub deadline: Option<UtcTimestamp>,
    pub expiry: Option<UtcTimestamp>,
}
