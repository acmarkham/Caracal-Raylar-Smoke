//! STM32 TRNG backend.

use core::sync::atomic::{AtomicU32, Ordering};

use embassy_stm32::interrupt::typelevel::Binding;
use embassy_stm32::rng::{Error as HalError, Instance, InterruptHandler, Rng};
use embassy_stm32::Peri;
use embassy_time::with_timeout;

use super::{TrngConfig, TrngError};

// The HAL rejects a zero RNG word, so zero is an unambiguous empty sentinel.
// This lives in ordinary zero-initialized RAM and is cleared by MCU reset.
static BOOT_ID: AtomicU32 = AtomicU32::new(0);

/// Single-owner STM32 true random number generator.
///
/// Construct this after `embassy_stm32::init`. The boot ID is retained in
/// static RAM until the MCU resets, including after the driver is dropped.
pub struct Stm32Trng<'d, T: Instance> {
    rng: Rng<'d, T>,
    config: TrngConfig,
}

impl<'d, T: Instance> Stm32Trng<'d, T> {
    /// Initialize the STM32 RNG and its interrupt-driven HAL backend.
    pub fn new(
        peripheral: Peri<'d, T>,
        irq: impl Binding<T::Interrupt, InterruptHandler<T>> + 'd,
        config: TrngConfig,
    ) -> Self {
        Self {
            rng: Rng::new(peripheral, irq),
            config,
        }
    }

    /// Obtain a fresh random word from the STM32 hardware TRNG.
    ///
    /// Hardware health and clock errors are propagated. A failed read never
    /// substitutes an earlier word.
    pub async fn latest_trng(&mut self) -> Result<u32, TrngError> {
        let mut bytes = [0u8; core::mem::size_of::<u32>()];

        match with_timeout(
            self.config.read_timeout,
            self.rng.async_fill_bytes(&mut bytes),
        )
        .await
        {
            Ok(Ok(())) => Ok(u32::from_ne_bytes(bytes)),
            Ok(Err(HalError::SeedError)) => Err(TrngError::SeedError),
            Ok(Err(HalError::ClockError)) => Err(TrngError::ClockError),
            Err(_) => Err(TrngError::DataReadyTimeout),
        }
    }

    /// Return the ID for this boot session.
    ///
    /// The first successful call obtains and stores a fresh TRNG word. Later
    /// calls return the stored word without touching the peripheral. A failed
    /// first attempt leaves the cache empty so that the caller can retry.
    pub async fn boot_id(&mut self) -> Result<u32, TrngError> {
        if let Some(boot_id) = self.cached_boot_id() {
            return Ok(boot_id);
        }

        let candidate = self.latest_trng().await?;
        match BOOT_ID.compare_exchange(0, candidate, Ordering::AcqRel, Ordering::Acquire) {
            Ok(_) => Ok(candidate),
            Err(boot_id) => Ok(boot_id),
        }
    }

    /// Return the boot ID if it has already been generated.
    pub fn cached_boot_id(&self) -> Option<u32> {
        match BOOT_ID.load(Ordering::Acquire) {
            0 => None,
            boot_id => Some(boot_id),
        }
    }
}
