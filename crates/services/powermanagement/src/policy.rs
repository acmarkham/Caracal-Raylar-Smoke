use raylar_drivers::batterycharger::ChargerState;
use raylar_drivers::voltagemonitor::VoltageState;

use crate::{BatteryHealth, PowerConfig, PowerSource, PowerState};

pub fn derive_power_state(
    voltage: VoltageState,
    charger: ChargerState,
    config: PowerConfig,
) -> PowerState {
    let battery_mv = clamp_mv(voltage.battery_mv);
    let solar_mv = clamp_mv(voltage.solar_mv);
    let ext_dc_mv = clamp_mv(voltage.ext_dc_mv);

    PowerState {
        source: source_for(voltage.usb_present, battery_mv, solar_mv, ext_dc_mv, config),
        battery_mv,
        solar_mv,
        ext_dc_mv,
        charging: charger.charging,
        battery_percent: if battery_mv >= config.battery_present_mv {
            Some(estimate_battery_percent(battery_mv))
        } else {
            None
        },
        health: health_for(battery_mv, config),
        charger,
    }
}

pub fn estimate_battery_percent(battery_mv: u16) -> u8 {
    const TABLE: &[(u16, u8)] = &[
        (3_300, 0),
        (3_500, 10),
        (3_600, 20),
        (3_700, 40),
        (3_800, 60),
        (3_950, 80),
        (4_100, 95),
        (4_200, 100),
    ];

    if battery_mv <= TABLE[0].0 {
        return TABLE[0].1;
    }

    let mut index = 1;
    while index < TABLE.len() {
        let (upper_mv, upper_percent) = TABLE[index];
        let (lower_mv, lower_percent) = TABLE[index - 1];
        if battery_mv <= upper_mv {
            let span_mv = upper_mv - lower_mv;
            let span_percent = upper_percent - lower_percent;
            let offset_mv = battery_mv - lower_mv;
            return lower_percent
                + (((offset_mv as u32 * span_percent as u32) / span_mv as u32) as u8);
        }
        index += 1;
    }

    100
}

fn source_for(
    usb_present: bool,
    battery_mv: u16,
    solar_mv: u16,
    ext_dc_mv: u16,
    config: PowerConfig,
) -> PowerSource {
    if usb_present {
        PowerSource::Usb
    } else if ext_dc_mv >= config.ext_dc_present_mv {
        PowerSource::ExternalDc
    } else if solar_mv >= config.solar_present_mv {
        PowerSource::Solar
    } else if battery_mv >= config.battery_present_mv {
        PowerSource::Battery
    } else {
        PowerSource::Unknown
    }
}

fn health_for(battery_mv: u16, config: PowerConfig) -> BatteryHealth {
    if battery_mv <= config.critical_battery_mv {
        BatteryHealth::Critical
    } else if battery_mv <= config.low_battery_mv {
        BatteryHealth::Low
    } else {
        BatteryHealth::Normal
    }
}

fn clamp_mv(value: u32) -> u16 {
    value.min(u16::MAX as u32) as u16
}

#[cfg(test)]
mod tests {
    use raylar_drivers::batterycharger::{ChargeState, ChargerState};

    use super::*;

    fn charger(charging: bool) -> ChargerState {
        ChargerState {
            charging,
            state: if charging {
                ChargeState::FastCharge
            } else {
                ChargeState::NotCharging
            },
            fault: None,
        }
    }

    fn voltage(battery_mv: u32, solar_mv: u32, ext_dc_mv: u32, usb_present: bool) -> VoltageState {
        VoltageState {
            battery_mv,
            solar_mv,
            ext_dc_mv,
            usb_present,
            vref_mv: 2_500,
        }
    }

    #[test]
    fn usb_takes_priority_over_other_sources() {
        let state = derive_power_state(
            voltage(3_900, 8_000, 12_000, true),
            charger(true),
            PowerConfig::default(),
        );

        assert_eq!(state.source, PowerSource::Usb);
        assert!(state.charging);
        assert_eq!(state.health, BatteryHealth::Normal);
    }

    #[test]
    fn external_dc_and_solar_are_detected_from_voltage() {
        let config = PowerConfig::default();

        assert_eq!(
            derive_power_state(voltage(3_900, 0, 4_600, false), charger(false), config).source,
            PowerSource::ExternalDc
        );
        assert_eq!(
            derive_power_state(voltage(3_900, 4_600, 0, false), charger(false), config).source,
            PowerSource::Solar
        );
    }

    #[test]
    fn low_and_critical_thresholds_follow_config() {
        let config = PowerConfig::default();

        assert_eq!(
            derive_power_state(voltage(3_550, 0, 0, false), charger(false), config).health,
            BatteryHealth::Low
        );
        assert_eq!(
            derive_power_state(voltage(3_350, 0, 0, false), charger(false), config).health,
            BatteryHealth::Critical
        );
    }

    #[test]
    fn missing_battery_has_no_percent_and_unknown_source() {
        let state = derive_power_state(
            voltage(100, 0, 0, false),
            charger(false),
            PowerConfig::default(),
        );

        assert_eq!(state.source, PowerSource::Unknown);
        assert_eq!(state.battery_percent, None);
    }

    #[test]
    fn battery_percent_interpolates_table() {
        assert_eq!(estimate_battery_percent(3_300), 0);
        assert_eq!(estimate_battery_percent(3_550), 15);
        assert_eq!(estimate_battery_percent(3_750), 50);
        assert_eq!(estimate_battery_percent(4_200), 100);
    }
}
