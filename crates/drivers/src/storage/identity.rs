/// Stable SD-card identity captured during card initialization.
///
/// This deliberately contains no STM32 or SDMMC types, so higher layers can
/// expose the metadata without taking ownership of the hardware driver.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct StorageDeviceIdentity {
    pub manufacturer_id: u8,
    pub oem_id: Option<[u8; 2]>,
    pub product_name: Option<[u8; 5]>,
    pub product_revision: u8,
    pub serial_number: u32,
    pub manufacture_year: u16,
    pub manufacture_month: u8,
    pub capacity_bytes: u64,
}
