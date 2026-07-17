use embassy_time::Duration;
use raylar_drivers::batterycharger::{ChargeState, ChargerState};

pub const DEFAULT_LOW_BATTERY_MV: u16 = 3_550;
pub const DEFAULT_CRITICAL_BATTERY_MV: u16 = 3_350;
pub const DEFAULT_BATTERY_PRESENT_MV: u16 = 2_500;
pub const DEFAULT_SOLAR_PRESENT_MV: u16 = 4_500;
pub const DEFAULT_EXT_DC_PRESENT_MV: u16 = 4_500;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerSource {
    Battery,
    Solar,
    ExternalDc,
    Usb,
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BatteryHealth {
    Normal,
    Low,
    Critical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PowerConfig {
    pub publish_interval: Duration,
    pub low_battery_mv: u16,
    pub critical_battery_mv: u16,
    pub battery_present_mv: u16,
    pub solar_present_mv: u16,
    pub ext_dc_present_mv: u16,
}

impl Default for PowerConfig {
    fn default() -> Self {
        Self {
            publish_interval: Duration::from_secs(1),
            low_battery_mv: DEFAULT_LOW_BATTERY_MV,
            critical_battery_mv: DEFAULT_CRITICAL_BATTERY_MV,
            battery_present_mv: DEFAULT_BATTERY_PRESENT_MV,
            solar_present_mv: DEFAULT_SOLAR_PRESENT_MV,
            ext_dc_present_mv: DEFAULT_EXT_DC_PRESENT_MV,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerState {
    pub source: PowerSource,
    pub battery_mv: u16,
    pub solar_mv: u16,
    pub ext_dc_mv: u16,
    pub charging: bool,
    pub battery_percent: Option<u8>,
    pub health: BatteryHealth,
    pub charger: ChargerState,
}

impl PowerState {
    pub const fn unknown() -> Self {
        Self {
            source: PowerSource::Unknown,
            battery_mv: 0,
            solar_mv: 0,
            ext_dc_mv: 0,
            charging: false,
            battery_percent: None,
            health: BatteryHealth::Critical,
            charger: ChargerState {
                charging: false,
                state: ChargeState::NotCharging,
                fault: None,
            },
        }
    }
}
