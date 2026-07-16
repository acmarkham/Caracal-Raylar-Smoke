mod calibration;
#[cfg(feature = "stm32")]
pub mod stm32;
#[cfg(feature = "stm32")]
mod stm32_vrefbuf;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::{Receiver, Watch};
use embassy_time::{Duration, Timer};

pub const DEFAULT_WATCHERS: usize = 4;
pub type VoltageMutex = CriticalSectionRawMutex;
pub type VoltageStateReceiver<'a, const WATCHERS: usize> =
    Receiver<'a, VoltageMutex, VoltageState, WATCHERS>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct VoltageState {
    pub battery_mv: u32,
    pub solar_mv: u32,
    pub ext_dc_mv: u32,
    pub usb_present: bool,
    pub vref_mv: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VoltageConfig {
    pub sample_interval: Duration,
}

impl Default for VoltageConfig {
    fn default() -> Self {
        Self {
            sample_interval: Duration::from_secs(1),
        }
    }
}

pub struct VoltageResources<const WATCHERS: usize = DEFAULT_WATCHERS> {
    state: Watch<VoltageMutex, VoltageState, WATCHERS>,
}

impl<const WATCHERS: usize> VoltageResources<WATCHERS> {
    pub const fn new() -> Self {
        Self {
            state: Watch::new_with(VoltageState {
                battery_mv: 0,
                solar_mv: 0,
                ext_dc_mv: 0,
                usb_present: false,
                vref_mv: 0,
            }),
        }
    }

    pub fn state_receiver(&self) -> Option<VoltageStateReceiver<'_, WATCHERS>> {
        self.state.receiver()
    }

    pub fn state(&self) -> VoltageState {
        self.state.try_get().unwrap_or_default()
    }
}

impl<const WATCHERS: usize> Default for VoltageResources<WATCHERS> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait VoltageSampler {
    fn sample(&mut self) -> VoltageState;
}

pub struct VoltageMonitorDriver<S, const WATCHERS: usize = DEFAULT_WATCHERS> {
    sampler: S,
    resources: &'static VoltageResources<WATCHERS>,
    config: VoltageConfig,
}

impl<S, const WATCHERS: usize> VoltageMonitorDriver<S, WATCHERS> {
    pub const fn new(
        sampler: S,
        resources: &'static VoltageResources<WATCHERS>,
        config: VoltageConfig,
    ) -> Self {
        Self {
            sampler,
            resources,
            config,
        }
    }
}

impl<S, const WATCHERS: usize> VoltageMonitorDriver<S, WATCHERS>
where
    S: VoltageSampler,
{
    pub async fn run(mut self) -> ! {
        let state = self.resources.state.sender();

        loop {
            state.send(self.sampler.sample());
            Timer::after(self.config.sample_interval).await;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Divider {
    numerator: u32,
    denominator: u32,
}

impl Divider {
    pub const fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    pub fn scale_mv(self, pin_mv: u32) -> u32 {
        ((pin_mv * self.numerator) + (self.denominator / 2)) / self.denominator
    }
}

#[cfg_attr(not(feature = "stm32"), allow(dead_code))]
pub(crate) fn raw_to_mv(raw: u32, vref_mv: u32, max_counts: u32) -> u32 {
    ((raw * vref_mv) + (max_counts / 2)) / max_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_to_mv_maps_full_scale_to_reference() {
        assert_eq!(raw_to_mv(16_383, 2_500, 16_383), 2_500);
        assert_eq!(raw_to_mv(0, 2_500, 16_383), 0);
    }

    #[test]
    fn divider_scales_source_voltage_with_rounding() {
        let divider_180k_33k = Divider::new(180 + 33, 33);

        assert_eq!(divider_180k_33k.scale_mv(1_000), 6_455);
    }
}
