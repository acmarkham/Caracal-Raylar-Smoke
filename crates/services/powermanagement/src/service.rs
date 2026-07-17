use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::{DynReceiver, Receiver, Watch};
use embassy_time::Timer;
use raylar_drivers::batterycharger::ChargerState;
use raylar_drivers::voltagemonitor::VoltageState;

use crate::{derive_power_state, PowerConfig, PowerState};

pub const DEFAULT_POWER_WATCHERS: usize = 4;
pub type PowerMutex = CriticalSectionRawMutex;
pub type PowerStateReceiver<'a, const WATCHERS: usize> =
    Receiver<'a, PowerMutex, PowerState, WATCHERS>;

pub struct PowerResources<const WATCHERS: usize = DEFAULT_POWER_WATCHERS> {
    state: Watch<PowerMutex, PowerState, WATCHERS>,
}

impl<const WATCHERS: usize> PowerResources<WATCHERS> {
    pub const fn new() -> Self {
        Self {
            state: Watch::new_with(PowerState::unknown()),
        }
    }

    pub fn state_receiver(&self) -> Option<PowerStateReceiver<'_, WATCHERS>> {
        self.state.receiver()
    }

    pub fn state(&self) -> PowerState {
        self.state.try_get().unwrap_or_else(PowerState::unknown)
    }
}

impl<const WATCHERS: usize> Default for PowerResources<WATCHERS> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PowerManagementService<const WATCHERS: usize = DEFAULT_POWER_WATCHERS> {
    resources: &'static PowerResources<WATCHERS>,
    voltage: DynReceiver<'static, VoltageState>,
    charger: DynReceiver<'static, ChargerState>,
    config: PowerConfig,
}

impl<const WATCHERS: usize> PowerManagementService<WATCHERS> {
    pub fn new(
        resources: &'static PowerResources<WATCHERS>,
        voltage: DynReceiver<'static, VoltageState>,
        charger: DynReceiver<'static, ChargerState>,
        config: PowerConfig,
    ) -> Self {
        Self {
            resources,
            voltage,
            charger,
            config,
        }
    }

    pub async fn run(mut self) -> ! {
        let state = self.resources.state.sender();
        let mut voltage = self.voltage.get().await;
        let mut charger = self.charger.get().await;

        loop {
            while let Some(next) = self.voltage.try_changed() {
                voltage = next;
            }
            while let Some(next) = self.charger.try_changed() {
                charger = next;
            }

            state.send(derive_power_state(voltage, charger, self.config));
            Timer::after(self.config.publish_interval).await;
        }
    }
}
