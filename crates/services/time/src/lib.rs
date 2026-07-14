#![no_std]

#[cfg(test)]
extern crate std;

mod estimator;
#[cfg(feature = "gps")]
pub mod gps;
mod service;
mod types;

pub use estimator::TimeEstimator;
pub use service::{AnchorSender, TimeResources, TimeService, TimeStateReceiver};
pub use types::{
    Anchor, AnchorQuality, TimeConfig, TimeError, TimeSource, TimeState, UtcTimestamp,
    DEFAULT_ANCHOR_DEPTH, DEFAULT_WATCHERS,
};
