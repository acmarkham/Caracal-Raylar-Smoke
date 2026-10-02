#![no_std]
#![allow(async_fn_in_trait)]
//! Heapless Phase I radio messaging primitives and the sole-owner radio task.
//!
//! Applications exchange semantic messages. Protocol modules turn those
//! messages into compact frames and submit bounded, timed jobs to
//! [`RadioService`]; only that service owns the physical LR1121 driver.

#[cfg(test)]
extern crate std;

mod address;
mod config;
mod epoch;
mod error;
mod frame;
mod heartbeat;
pub mod link;
mod message;
mod neighbour;
mod presence;
mod rendezvous;
mod scheduler;
mod service;
mod stats;

pub use address::{BootId, DuplicateKey, GroupId, NodeId, Sequence, SequenceState};
pub use config::{ConfigError, RadioServiceConfig};
pub use epoch::{Epoch, EpochConfig, EpochPosition};
pub use error::{FrameError, LinkError, RadioServiceError, ScheduleError};
pub use frame::{DecodedFrame, FrameHeader, FrameType, FRAME_HEADER_LEN, WIRE_VERSION};
pub use heartbeat::{
    BatterySoc, ChargingState, CompactLocation, ErrorFlags, FixQuality, GpsStatus, Heartbeat,
    HeartbeatProtocol, StorageUsage, TimeUncertaintyClass,
};
pub use message::{
    Destination, Message, MessageClass, MessageError, MessageId, MessageOptions, MessagePriority,
    Reliability, MAX_MESSAGE_LEN,
};
pub use neighbour::{NeighbourEntry, NeighbourTable};
pub use presence::{CapabilityFlags, PresenceAdvert, PresenceProtocol, ScheduleVersion};
pub use rendezvous::{Rendezvous, RendezvousPurpose, RENDEZVOUS_ALGORITHM_VERSION};
pub use scheduler::{guard_interval, RadioPriority, Reservation, ReservationOutcome, Scheduler};
pub use service::{
    DriverPacketMetadata, FrameBuffer, JobId, RadioDevice, RadioDeviceError, RadioEvent,
    RadioEventReceiver, RadioHandle, RadioJob, RadioMode, RadioResources, RadioRxJob, RadioService,
    RadioServiceState, RadioStateReceiver, RadioTxJob, RxPurpose, DEFAULT_EVENT_DEPTH,
    DEFAULT_JOB_DEPTH, DEFAULT_STATE_WATCHERS, MAX_FRAME_LEN,
};
pub use stats::RadioServiceStats;

#[cfg(test)]
mod tests;
