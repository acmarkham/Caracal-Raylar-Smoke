#![no_std]

#[cfg(test)]
extern crate std;

mod service;
mod snapshot;
mod types;

pub use service::{
    IdentityResources, IdentityStateReceiver, IdentityVersioningService, DEFAULT_IDENTITY_WATCHERS,
};
pub use snapshot::collect_identity_state;
pub use types::{
    DeviceIdentity, FirmwareIdentity, FixedIdentityString, GpsModuleIdentity, HardwareIdentity,
    IdentityConfig, IdentityField, IdentityState, RadioModuleIdentity, SdCardIdentity,
    Stm32DeviceCode,
};
