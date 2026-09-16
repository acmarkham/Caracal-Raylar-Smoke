use heapless::String;
use raylar_drivers::identity::DeviceUid;

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
