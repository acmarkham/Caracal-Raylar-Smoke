//! Device and firmware identity helpers.
//!
//! The raw STM32 factory UID is the authoritative device identity. Shorter
//! serial IDs are deterministic derivatives intended for logs, filenames and
//! human support workflows, not for security decisions.

#[cfg(feature = "stm32")]
pub mod stm32;

const SERIAL_48_MASK: u64 = 0x0000_FFFF_FFFF_FFFF;
const UID_HASH64_SEED: u64 = 0x9E37_79B9_7F4A_7C15;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeviceUid {
    pub word0: u32,
    pub word1: u32,
    pub word2: u32,
}

impl DeviceUid {
    pub const fn new(word0: u32, word1: u32, word2: u32) -> Self {
        Self {
            word0,
            word1,
            word2,
        }
    }

    pub const fn words(self) -> [u32; 3] {
        [self.word0, self.word1, self.word2]
    }

    pub fn serials(self) -> DeviceSerials {
        DeviceSerials::from_uid(self)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DeviceSerials {
    /// 64-bit compact identifier derived from the 96-bit UID.
    pub serial_64: u64,
    /// 48-bit compact identifier stored in a `u64` with upper bits cleared.
    pub serial_48: u64,
    /// 32-bit support/log identifier matching the legacy C avalanche.
    pub serial_32: u32,
    /// 16-bit local display/debug identifier. Collisions are expected.
    pub serial_16: u16,
}

impl DeviceSerials {
    pub fn from_uid(uid: DeviceUid) -> Self {
        let serial_64 = uid_hash64(uid);
        let serial_32 = uid_hash32(uid);

        Self {
            serial_64,
            serial_48: serial_64 & SERIAL_48_MASK,
            serial_32,
            serial_16: serial_32 as u16,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FirmwareIdentity {
    pub runtime_crc32: Option<u32>,
    pub build_crc32: Option<u32>,
    pub git_hash: Option<&'static str>,
    pub build_timestamp: Option<&'static str>,
    pub version: Option<&'static str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum IdentityError {
    FirmwareRangeUnavailable,
    InvalidFirmwareRange,
    UnsupportedFlashLayout,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FirmwareImageRange {
    start: usize,
    end: usize,
}

impl FirmwareImageRange {
    pub const fn new(start: usize, end: usize) -> Result<Self, IdentityError> {
        if start >= end {
            Err(IdentityError::InvalidFirmwareRange)
        } else {
            Ok(Self { start, end })
        }
    }

    /// Create a range from linker-provided addresses.
    ///
    /// The caller must ensure the range is readable flash for the whole
    /// firmware image. Prefer explicit linker/build metadata over literals.
    pub const unsafe fn new_unchecked(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub const fn start(self) -> usize {
        self.start
    }

    pub const fn end(self) -> usize {
        self.end
    }

    pub const fn len(self) -> usize {
        self.end - self.start
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct IdentityDriver {
    uid: DeviceUid,
    serials: DeviceSerials,
    firmware: FirmwareIdentity,
}

impl IdentityDriver {
    pub fn from_uid(uid: DeviceUid) -> Self {
        Self {
            uid,
            serials: uid.serials(),
            firmware: firmware_identity(),
        }
    }

    pub const fn uid(&self) -> DeviceUid {
        self.uid
    }

    pub const fn serials(&self) -> DeviceSerials {
        self.serials
    }

    pub const fn serial_64(&self) -> u64 {
        self.serials.serial_64
    }

    pub const fn serial_48(&self) -> u64 {
        self.serials.serial_48
    }

    pub const fn serial_32(&self) -> u32 {
        self.serials.serial_32
    }

    pub const fn serial_16(&self) -> u16 {
        self.serials.serial_16
    }

    pub const fn firmware_identity(&self) -> FirmwareIdentity {
        self.firmware
    }
}

#[cfg(feature = "stm32")]
pub fn init() -> IdentityDriver {
    IdentityDriver::from_uid(read_device_uid())
}

#[cfg(feature = "stm32")]
pub fn read_device_uid() -> DeviceUid {
    stm32::read_device_uid()
}

#[cfg(feature = "stm32")]
pub fn device_serials() -> DeviceSerials {
    read_device_uid().serials()
}

pub fn firmware_identity() -> FirmwareIdentity {
    FirmwareIdentity {
        runtime_crc32: None,
        build_crc32: option_env!("RAYLAR_FIRMWARE_CRC32").and_then(parse_u32),
        git_hash: first_some(&[
            option_env!("RAYLAR_GIT_HASH"),
            option_env!("GIT_HASH"),
            option_env!("VERGEN_GIT_SHA"),
        ]),
        build_timestamp: first_some(&[
            option_env!("RAYLAR_BUILD_TIMESTAMP"),
            option_env!("BUILD_TIMESTAMP"),
            option_env!("VERGEN_BUILD_TIMESTAMP"),
        ]),
        version: option_env!("RAYLAR_FIRMWARE_VERSION"),
    }
}

pub fn calculate_firmware_crc32() -> Result<u32, IdentityError> {
    let range = default_firmware_image_range()?;
    unsafe { calculate_firmware_crc32_for_range(range) }
}

/// Calculate CRC32 over a caller-provided firmware image range.
///
/// # Safety
///
/// The range must point to readable memory for its full length and must remain
/// stable while the CRC is calculated.
pub unsafe fn calculate_firmware_crc32_for_range(
    range: FirmwareImageRange,
) -> Result<u32, IdentityError> {
    if range.start >= range.end {
        return Err(IdentityError::InvalidFirmwareRange);
    }

    let bytes = unsafe { core::slice::from_raw_parts(range.start as *const u8, range.len()) };
    Ok(crc32(bytes))
}

pub fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;

    for byte in bytes {
        crc ^= *byte as u32;
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }

    !crc
}

fn uid_hash32(uid: DeviceUid) -> u32 {
    avalanche32(uid.word0 ^ uid.word1 ^ uid.word2)
}

fn uid_hash64(uid: DeviceUid) -> u64 {
    let mut h = UID_HASH64_SEED ^ uid.word0 as u64;
    h = avalanche64(h);
    h ^= ((uid.word1 as u64) << 32) | uid.word2 as u64;
    avalanche64(h)
}

fn avalanche32(mut h: u32) -> u32 {
    h ^= h >> 16;
    h = h.wrapping_mul(0x85EB_CA6B);
    h ^= h >> 13;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^= h >> 16;
    h
}

fn avalanche64(mut h: u64) -> u64 {
    h ^= h >> 33;
    h = h.wrapping_mul(0xFF51_AFD7_ED55_8CCD);
    h ^= h >> 33;
    h = h.wrapping_mul(0xC4CE_B9FE_1A85_EC53);
    h ^= h >> 33;
    h
}

fn parse_u32(value: &'static str) -> Option<u32> {
    let value = strip_hex_prefix(value);
    let mut parsed = 0u32;

    for byte in value.as_bytes() {
        let digit = match *byte {
            b'0'..=b'9' => *byte - b'0',
            b'a'..=b'f' => *byte - b'a' + 10,
            b'A'..=b'F' => *byte - b'A' + 10,
            b'_' => continue,
            _ => return None,
        };

        parsed = parsed.checked_mul(16)?;
        parsed = parsed.checked_add(digit as u32)?;
    }

    Some(parsed)
}

fn strip_hex_prefix(value: &'static str) -> &'static str {
    if let Some(stripped) = value.strip_prefix("0x") {
        stripped
    } else if let Some(stripped) = value.strip_prefix("0X") {
        stripped
    } else {
        value
    }
}

fn first_some(values: &[Option<&'static str>]) -> Option<&'static str> {
    values.iter().copied().flatten().next()
}

#[cfg(all(feature = "stm32", target_arch = "arm"))]
fn default_firmware_image_range() -> Result<FirmwareImageRange, IdentityError> {
    extern "C" {
        static __vector_table: u8;
        static __sidata: u8;
        static __sdata: u8;
        static __edata: u8;
    }

    let start = core::ptr::addr_of!(__vector_table) as usize;
    let data_load_start = core::ptr::addr_of!(__sidata) as usize;
    let data_len =
        (core::ptr::addr_of!(__edata) as usize).wrapping_sub(core::ptr::addr_of!(__sdata) as usize);
    let end = data_load_start
        .checked_add(data_len)
        .ok_or(IdentityError::InvalidFirmwareRange)?;

    FirmwareImageRange::new(start, end)
}

#[cfg(not(all(feature = "stm32", target_arch = "arm")))]
fn default_firmware_image_range() -> Result<FirmwareImageRange, IdentityError> {
    Err(IdentityError::FirmwareRangeUnavailable)
}

#[cfg(test)]
mod tests {
    use super::*;

    const UID: DeviceUid = DeviceUid::new(0x0011_2233, 0x4455_6677, 0x8899_AABB);

    #[test]
    fn uid_words_keep_canonical_order() {
        assert_eq!(UID.words(), [0x0011_2233, 0x4455_6677, 0x8899_AABB]);
    }

    #[test]
    fn serials_are_deterministic_and_nested() {
        let first = UID.serials();
        let second = UID.serials();

        assert_eq!(first, second);
        assert_eq!(first.serial_48 >> 48, 0);
        assert_eq!(first.serial_48, first.serial_64 & SERIAL_48_MASK);
        assert_eq!(first.serial_16, first.serial_32 as u16);
    }

    #[test]
    fn serial32_matches_legacy_avalanche_construction() {
        let mut h = UID.word0 ^ UID.word1 ^ UID.word2;
        h ^= h >> 16;
        h = h.wrapping_mul(0x85EB_CA6B);
        h ^= h >> 13;
        h = h.wrapping_mul(0xC2B2_AE35);
        h ^= h >> 16;

        assert_eq!(UID.serials().serial_32, h);
    }

    #[test]
    fn crc32_matches_standard_check_value() {
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
    }

    #[test]
    fn invalid_firmware_range_is_rejected() {
        assert_eq!(
            FirmwareImageRange::new(0x0800_1000, 0x0800_1000),
            Err(IdentityError::InvalidFirmwareRange)
        );
    }

    #[test]
    fn decimal_free_build_crc_parser_accepts_hex() {
        assert_eq!(parse_u32("0x7C91_D42E"), Some(0x7C91_D42E));
        assert_eq!(parse_u32("7c91d42e"), Some(0x7C91_D42E));
        assert_eq!(parse_u32("not-a-crc"), None);
    }
}
