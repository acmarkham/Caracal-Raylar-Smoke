pub(crate) const TX_DONE: u32 = 1 << 2;
pub(crate) const RX_DONE: u32 = 1 << 3;
pub(crate) const HEADER_ERROR: u32 = 1 << 6;
pub(crate) const CRC_ERROR: u32 = 1 << 7;
pub(crate) const TIMEOUT: u32 = 1 << 10;
pub(crate) const COMMAND_ERROR: u32 = 1 << 22;
pub(crate) const DEVICE_ERROR: u32 = 1 << 23;
pub(crate) const GFSK_LENGTH_ERROR: u32 = 1 << 24;
pub(crate) const GFSK_ADDRESS_ERROR: u32 = 1 << 25;

pub(crate) const RX_MASK: u32 = RX_DONE
    | HEADER_ERROR
    | CRC_ERROR
    | TIMEOUT
    | COMMAND_ERROR
    | DEVICE_ERROR
    | GFSK_LENGTH_ERROR
    | GFSK_ADDRESS_ERROR;

pub(crate) const TX_MASK: u32 = TX_DONE | TIMEOUT | COMMAND_ERROR | DEVICE_ERROR;
pub(crate) const ALL: u32 = u32::MAX;
