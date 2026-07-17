#![no_std]

#[cfg(test)]
extern crate std;

mod policy;
mod service;
mod types;

pub use policy::{derive_power_state, estimate_battery_percent};
pub use service::{
    PowerManagementService, PowerResources, PowerStateReceiver, DEFAULT_POWER_WATCHERS,
};
pub use types::{BatteryHealth, PowerConfig, PowerSource, PowerState};
