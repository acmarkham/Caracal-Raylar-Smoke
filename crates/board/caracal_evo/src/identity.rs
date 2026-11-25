//! Read the L4 UID through its HAL; keep upstream serial derivation unchanged.
pub fn uid() -> raylar_drivers::identity::DeviceUid {
    let bytes = embassy_stm32::uid::uid();
    raylar_drivers::identity::DeviceUid::new(
        u32::from_le_bytes(bytes[0..4].try_into().unwrap()),
        u32::from_le_bytes(bytes[4..8].try_into().unwrap()),
        u32::from_le_bytes(bytes[8..12].try_into().unwrap()),
    )
}
