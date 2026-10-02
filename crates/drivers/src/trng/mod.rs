//! True random number generation and boot-session identification.
//!
//! A fresh TRNG read consumes a new hardware word. The boot ID is generated
//! once, on demand, and remains stable until the MCU resets.

#[cfg(feature = "stm32")]
pub mod stm32;

use embassy_time::Duration;

/// Runtime configuration for TRNG reads.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct TrngConfig {
    /// Maximum time to wait for the peripheral to produce a fresh word.
    pub read_timeout: Duration,
}

impl Default for TrngConfig {
    fn default() -> Self {
        Self {
            read_timeout: Duration::from_millis(100),
        }
    }
}

/// Failure to obtain a valid word from the hardware TRNG.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum TrngError {
    /// The hardware health test reported invalid entropy.
    SeedError,
    /// The peripheral reported that its source clock is unsuitable.
    ClockError,
    /// No result or hardware error arrived before the configured deadline.
    DataReadyTimeout,
}
