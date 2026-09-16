#![no_std]

#[cfg(test)]
extern crate std;

mod filter;
mod service;
mod types;

pub use filter::{LocationFilter, LocationSample};
pub use service::{
    LocationResources, LocationService, LocationStateReceiver, DEFAULT_LOCATION_HISTORY,
    DEFAULT_LOCATION_WATCHERS,
};
pub use types::{LocationConfig, LocationSource, LocationState};
