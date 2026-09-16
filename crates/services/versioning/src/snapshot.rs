use raylar_drivers::identity as traceability;

use crate::{
    DeviceIdentity, FirmwareIdentity, HardwareIdentity, IdentityConfig, IdentityField,
    IdentityState,
};

pub fn collect_identity_state(config: &IdentityConfig) -> IdentityState {
    IdentityState {
        device: collect_device_identity(config),
        firmware: collect_firmware_identity(config),
        hardware: HardwareIdentity {
            board_revision: config.board_revision,
            sd_card: IdentityField::Unknown,
            gps_module: IdentityField::Unknown,
            radio_module: IdentityField::Unknown,
        },
    }
}

fn collect_device_identity(config: &IdentityConfig) -> DeviceIdentity {
    #[cfg(feature = "stm32")]
    {
        let identity = traceability::init();
        let uid = identity.uid();
        let serials = identity.serials();
        DeviceIdentity {
            stm32_uid_96: IdentityField::Known(uid),
            serial_64: IdentityField::Known(serials.serial_64),
            serial_48: IdentityField::Known(serials.serial_48),
            serial_32: IdentityField::Known(serials.serial_32),
            serial_16: IdentityField::Known(serials.serial_16),
            stm32_device_code: config.stm32_device_code,
        }
    }

    #[cfg(not(feature = "stm32"))]
    {
        DeviceIdentity {
            stm32_uid_96: IdentityField::Unavailable,
            serial_64: IdentityField::Unavailable,
            serial_48: IdentityField::Unavailable,
            serial_32: IdentityField::Unavailable,
            serial_16: IdentityField::Unavailable,
            stm32_device_code: config.stm32_device_code,
        }
    }
}

fn collect_firmware_identity(config: &IdentityConfig) -> FirmwareIdentity {
    let firmware = traceability::firmware_identity();
    FirmwareIdentity {
        version: field_from_option(firmware.version),
        git_hash: field_from_option(firmware.git_hash),
        build_timestamp: field_from_option(firmware.build_timestamp),
        build_profile: field_from_option(
            option_env!("RAYLAR_BUILD_PROFILE").or(option_env!("PROFILE")),
        ),
        runtime_crc32: if config.calculate_runtime_crc32 {
            traceability::calculate_firmware_crc32()
                .map(IdentityField::Known)
                .unwrap_or(IdentityField::Unavailable)
        } else {
            IdentityField::Unknown
        },
        build_crc32: field_from_option(firmware.build_crc32),
    }
}

fn field_from_option<T>(value: Option<T>) -> IdentityField<T> {
    value
        .map(IdentityField::Known)
        .unwrap_or(IdentityField::Unknown)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Stm32DeviceCode;

    #[test]
    fn non_stm32_snapshot_marks_uid_unavailable() {
        let config = IdentityConfig {
            stm32_device_code: IdentityField::Unavailable,
            board_revision: IdentityField::Known("test-board"),
            calculate_runtime_crc32: false,
        };

        let state = collect_identity_state(&config);

        assert_eq!(state.device.stm32_uid_96, IdentityField::Unavailable);
        assert_eq!(
            state.hardware.board_revision,
            IdentityField::Known("test-board")
        );
        assert_eq!(state.firmware.runtime_crc32, IdentityField::Unknown);
    }

    #[test]
    fn config_can_supply_stm32_device_code() {
        let config = IdentityConfig {
            stm32_device_code: IdentityField::Known(Stm32DeviceCode {
                family: "STM32U5",
                device: "STM32U595xx",
                package: Some("VJ"),
            }),
            board_revision: IdentityField::Unavailable,
            calculate_runtime_crc32: false,
        };

        let state = collect_identity_state(&config);

        assert_eq!(
            state.device.stm32_device_code,
            IdentityField::Known(Stm32DeviceCode {
                family: "STM32U5",
                device: "STM32U595xx",
                package: Some("VJ")
            })
        );
    }
}
