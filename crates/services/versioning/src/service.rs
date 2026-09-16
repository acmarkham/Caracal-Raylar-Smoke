use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::{Receiver, Watch};
use embassy_time::Timer;

use crate::{
    collect_identity_state, GpsModuleIdentity, HardwareIdentity, IdentityConfig, IdentityField,
    IdentityState, RadioModuleIdentity, SdCardIdentity,
};

pub const DEFAULT_IDENTITY_WATCHERS: usize = 4;

pub type IdentityMutex = CriticalSectionRawMutex;
pub type IdentityStateReceiver<'a, const WATCHERS: usize> =
    Receiver<'a, IdentityMutex, IdentityState, WATCHERS>;

pub struct IdentityResources<const WATCHERS: usize = DEFAULT_IDENTITY_WATCHERS> {
    state: Watch<IdentityMutex, IdentityState, WATCHERS>,
}

impl<const WATCHERS: usize> IdentityResources<WATCHERS> {
    pub const fn new() -> Self {
        Self {
            state: Watch::new_with(IdentityState::unknown()),
        }
    }

    pub fn state_receiver(&self) -> Option<IdentityStateReceiver<'_, WATCHERS>> {
        self.state.receiver()
    }

    pub fn state(&self) -> IdentityState {
        self.state.try_get().unwrap_or_else(IdentityState::unknown)
    }

    fn publish(&self, state: IdentityState) {
        self.state.sender().send(state);
    }
}

impl<const WATCHERS: usize> Default for IdentityResources<WATCHERS> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct IdentityVersioningService<const WATCHERS: usize = DEFAULT_IDENTITY_WATCHERS> {
    resources: &'static IdentityResources<WATCHERS>,
    state: IdentityState,
}

impl<const WATCHERS: usize> IdentityVersioningService<WATCHERS> {
    pub fn new(resources: &'static IdentityResources<WATCHERS>, config: IdentityConfig) -> Self {
        Self {
            resources,
            state: collect_identity_state(&config),
        }
    }

    pub fn state(&self) -> IdentityState {
        self.state.clone()
    }

    pub fn watch(&self) -> Option<IdentityStateReceiver<'_, WATCHERS>> {
        self.resources.state_receiver()
    }

    pub fn publish(&self) {
        self.resources.publish(self.state.clone());
    }

    pub fn set_sd_card_identity(&mut self, identity: IdentityField<SdCardIdentity>) {
        self.state.hardware.sd_card = identity;
        self.publish();
    }

    pub fn set_gps_module_identity(&mut self, identity: IdentityField<GpsModuleIdentity>) {
        self.state.hardware.gps_module = identity;
        self.publish();
    }

    pub fn set_radio_module_identity(&mut self, identity: IdentityField<RadioModuleIdentity>) {
        self.state.hardware.radio_module = identity;
        self.publish();
    }

    pub fn set_hardware_identity(&mut self, identity: HardwareIdentity) {
        self.state.hardware = identity;
        self.publish();
    }

    pub async fn run(self) -> ! {
        self.publish();
        loop {
            Timer::after_secs(60).await;
        }
    }
}
