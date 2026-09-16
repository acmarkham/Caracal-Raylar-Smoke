use heapless::String;
use raylar_drivers::{identity::DeviceUid, storage::StorageDeviceIdentity};

pub const IDENTITY_STRING_CAPACITY: usize = 64;
pub type FixedIdentityString = String<IDENTITY_STRING_CAPACITY>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdentityField<T> {
    Known(T),
    #[default]
    Unknown,
    Unavailable,
}

impl<T> IdentityField<T> {
    pub const fn is_known(&self) -> bool {
        matches!(self, Self::Known(_))
    }

    pub const fn as_ref(&self) -> IdentityField<&T> {
        match self {
            Self::Known(value) => IdentityField::Known(value),
            Self::Unknown => IdentityField::Unknown,
            Self::Unavailable => IdentityField::Unavailable,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Stm32DeviceCode {
    pub family: &'static str,
    pub device: &'static str,
    pub package: Option<&'static str>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentityConfig {
    pub stm32_device_code: IdentityField<Stm32DeviceCode>,
    pub board_revision: IdentityField<&'static str>,
    pub calculate_runtime_crc32: bool,
}

impl Default for IdentityConfig {
    fn default() -> Self {
        Self {
            stm32_device_code: default_stm32_device_code(),
            board_revision: option_env!("RAYLAR_BOARD_REVISION")
                .map(IdentityField::Known)
                .unwrap_or(IdentityField::Unavailable),
            calculate_runtime_crc32: true,
        }
    }
}

const fn default_stm32_device_code() -> IdentityField<Stm32DeviceCode> {
    #[cfg(feature = "stm32")]
    {
        IdentityField::Known(Stm32DeviceCode {
            family: "STM32U5",
            device: "STM32U595xx",
            package: None,
        })
    }
    #[cfg(not(feature = "stm32"))]
    {
        IdentityField::Unavailable
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentityState {
    pub device: DeviceIdentity,
    pub firmware: FirmwareIdentity,
    pub hardware: HardwareIdentity,
}

impl IdentityState {
    pub const fn unknown() -> Self {
        Self {
            device: DeviceIdentity::unknown(),
            firmware: FirmwareIdentity::unknown(),
            hardware: HardwareIdentity::unknown(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeviceIdentity {
    pub stm32_uid_96: IdentityField<DeviceUid>,
    pub serial_64: IdentityField<u64>,
    pub serial_48: IdentityField<u64>,
    pub serial_32: IdentityField<u32>,
    pub serial_16: IdentityField<u16>,
    pub stm32_device_code: IdentityField<Stm32DeviceCode>,
}

impl DeviceIdentity {
    pub const fn unknown() -> Self {
        Self {
            stm32_uid_96: IdentityField::Unknown,
            serial_64: IdentityField::Unknown,
            serial_48: IdentityField::Unknown,
            serial_32: IdentityField::Unknown,
            serial_16: IdentityField::Unknown,
            stm32_device_code: IdentityField::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FirmwareIdentity {
    pub version: IdentityField<&'static str>,
    pub git_hash: IdentityField<&'static str>,
    pub build_timestamp: IdentityField<&'static str>,
    pub build_profile: IdentityField<&'static str>,
    pub runtime_crc32: IdentityField<u32>,
    pub build_crc32: IdentityField<u32>,
}

impl FirmwareIdentity {
    pub const fn unknown() -> Self {
        Self {
            version: IdentityField::Unknown,
            git_hash: IdentityField::Unknown,
            build_timestamp: IdentityField::Unknown,
            build_profile: IdentityField::Unknown,
            runtime_crc32: IdentityField::Unknown,
            build_crc32: IdentityField::Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HardwareIdentity {
    pub board_revision: IdentityField<&'static str>,
    pub sd_card: IdentityField<SdCardIdentity>,
    pub gps_module: IdentityField<GpsModuleIdentity>,
    pub radio_module: IdentityField<RadioModuleIdentity>,
}

impl HardwareIdentity {
    pub const fn unknown() -> Self {
        Self {
            board_revision: IdentityField::Unknown,
            sd_card: IdentityField::Unknown,
            gps_module: IdentityField::Unknown,
            radio_module: IdentityField::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct SdCardIdentity {
    pub manufacturer_id: IdentityField<u8>,
    pub oem_id: IdentityField<[u8; 2]>,
    pub product_name: IdentityField<[u8; 5]>,
    pub product_revision: IdentityField<u8>,
    pub serial_number: IdentityField<u32>,
    pub manufacture_year: IdentityField<u16>,
    pub manufacture_month: IdentityField<u8>,
    pub capacity_bytes: IdentityField<u64>,
}

impl SdCardIdentity {
    pub const fn unknown() -> Self {
        Self {
            manufacturer_id: IdentityField::Unknown,
            oem_id: IdentityField::Unknown,
            product_name: IdentityField::Unknown,
            product_revision: IdentityField::Unknown,
            serial_number: IdentityField::Unknown,
            manufacture_year: IdentityField::Unknown,
            manufacture_month: IdentityField::Unknown,
            capacity_bytes: IdentityField::Unknown,
        }
    }
}

impl From<StorageDeviceIdentity> for SdCardIdentity {
    fn from(identity: StorageDeviceIdentity) -> Self {
        Self {
            manufacturer_id: IdentityField::Known(identity.manufacturer_id),
            oem_id: identity
                .oem_id
                .map(IdentityField::Known)
                .unwrap_or(IdentityField::Unavailable),
            product_name: identity
                .product_name
                .map(IdentityField::Known)
                .unwrap_or(IdentityField::Unavailable),
            product_revision: IdentityField::Known(identity.product_revision),
            serial_number: IdentityField::Known(identity.serial_number),
            manufacture_year: IdentityField::Known(identity.manufacture_year),
            manufacture_month: IdentityField::Known(identity.manufacture_month),
            capacity_bytes: IdentityField::Known(identity.capacity_bytes),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GpsModuleIdentity {
    pub vendor: IdentityField<&'static str>,
    pub model: IdentityField<&'static str>,
    pub firmware_version: IdentityField<FixedIdentityString>,
    pub protocol_version: IdentityField<FixedIdentityString>,
    pub hardware_version: IdentityField<FixedIdentityString>,
}

impl GpsModuleIdentity {
    pub const fn unknown() -> Self {
        Self {
            vendor: IdentityField::Unknown,
            model: IdentityField::Unknown,
            firmware_version: IdentityField::Unknown,
            protocol_version: IdentityField::Unknown,
            hardware_version: IdentityField::Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RadioModuleIdentity {
    pub vendor: IdentityField<&'static str>,
    pub model: IdentityField<&'static str>,
    pub firmware_version: IdentityField<FixedIdentityString>,
    pub hardware_version: IdentityField<FixedIdentityString>,
    pub protocol_version: IdentityField<FixedIdentityString>,
}

impl RadioModuleIdentity {
    pub const fn unknown() -> Self {
        Self {
            vendor: IdentityField::Unknown,
            model: IdentityField::Unknown,
            firmware_version: IdentityField::Unknown,
            hardware_version: IdentityField::Unknown,
            protocol_version: IdentityField::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_identity_maps_into_versioning_fields() {
        let identity = StorageDeviceIdentity {
            manufacturer_id: 0x03,
            oem_id: Some(*b"SD"),
            product_name: Some(*b"TEST1"),
            product_revision: 0x21,
            serial_number: 0x1234_5678,
            manufacture_year: 2026,
            manufacture_month: 9,
            capacity_bytes: 128_000_000_000,
        };

        let card = SdCardIdentity::from(identity);

        assert_eq!(card.manufacturer_id, IdentityField::Known(0x03));
        assert_eq!(card.oem_id, IdentityField::Known(*b"SD"));
        assert_eq!(card.product_name, IdentityField::Known(*b"TEST1"));
        assert_eq!(card.product_revision, IdentityField::Known(0x21));
        assert_eq!(card.serial_number, IdentityField::Known(0x1234_5678));
        assert_eq!(card.manufacture_year, IdentityField::Known(2026));
        assert_eq!(card.manufacture_month, IdentityField::Known(9));
        assert_eq!(card.capacity_bytes, IdentityField::Known(128_000_000_000));
    }

    #[test]
    fn invalid_card_text_is_explicitly_unavailable() {
        let identity = StorageDeviceIdentity {
            manufacturer_id: 1,
            oem_id: None,
            product_name: None,
            product_revision: 0,
            serial_number: 0,
            manufacture_year: 2000,
            manufacture_month: 0,
            capacity_bytes: 0,
        };

        let card = SdCardIdentity::from(identity);

        assert_eq!(card.oem_id, IdentityField::Unavailable);
        assert_eq!(card.product_name, IdentityField::Unavailable);
    }
}
