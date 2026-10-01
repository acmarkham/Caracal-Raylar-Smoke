use embassy_time::Instant;

use super::state::RadioState;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    UnsupportedFrequency,
    UnsupportedTxPower,
    LoRaPreambleTooShort,
    ImplicitHeaderNeedsPayloadLength,
    LongInterleaverNeedsPayloadLength,
    InvalidLongInterleaverPayloadLength,
    InvalidPayloadLength,
    UnsupportedGfskBitRate,
    UnsupportedGfskFrequencyDeviation,
    GfskReceiverBandwidthTooNarrow,
    InvalidPreamble,
    GfskSyncWordTooLong,
    GfskSyncWordRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransportError {
    Spi,
    Busy,
    Reset,
    Irq,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    InvalidConfiguration(ConfigError),
    InvalidState {
        expected: RadioState,
        actual: RadioState,
    },
    NotInitialized,
    ChannelNotPrepared,
    InvalidTimeWindow,
    DeadlineMissed {
        requested: Instant,
        observed: Instant,
    },
    BufferTooSmall {
        required: usize,
        available: usize,
    },
    PayloadTooLarge,
    Transport(TransportError),
    BusyTimeout,
    RxTimeout {
        at: Instant,
    },
    TxTimeout {
        at: Instant,
    },
    CrcRejected {
        at: Instant,
    },
    HeaderRejected {
        at: Instant,
    },
    GfskLengthRejected {
        at: Instant,
    },
    GfskAddressRejected {
        at: Instant,
    },
    Command,
    Device,
    CommandIrq {
        at: Instant,
    },
    DeviceIrq {
        at: Instant,
    },
    WakeUp,
    Calibration,
}

impl From<ConfigError> for Error {
    fn from(value: ConfigError) -> Self {
        Self::InvalidConfiguration(value)
    }
}
