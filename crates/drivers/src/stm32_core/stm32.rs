//! STM32U5 implementation of core-supply selection.

use embassy_stm32::pac::pwr::vals::Regsel;
use embassy_stm32::pac::PWR;

use super::{CoreConfig, CoreError, CoreSupply, CoreSupplyControl};

// A regulator transition normally completes in far fewer polls. Keeping the
// wait bounded prevents a non-SMPS package or faulty supply from hanging boot.
const TRANSITION_STATUS_POLLS: usize = 1_000_000;

/// Driver for STM32 core facilities that do not own a peripheral token.
///
/// Construct this after `embassy_stm32::init`, which resets the PWR peripheral,
/// and before initializing drivers that place load on VCORE.
pub struct Stm32CoreDriver;

impl Stm32CoreDriver {
    pub fn init(config: CoreConfig) -> Result<Self, CoreError> {
        let mut driver = Self;
        driver.select_supply(config.supply)?;
        Ok(driver)
    }

    fn wait_for_supply(&self, requested: CoreSupply) -> Result<(), CoreError> {
        for _ in 0..TRANSITION_STATUS_POLLS {
            if self.selected_supply() == requested {
                return Ok(());
            }
            core::hint::spin_loop();
        }

        Err(CoreError::TransitionTimeout {
            requested,
            observed: self.selected_supply(),
        })
    }
}

impl CoreSupplyControl for Stm32CoreDriver {
    fn select_supply(&mut self, supply: CoreSupply) -> Result<(), CoreError> {
        let regsel = match supply {
            CoreSupply::Ldo => Regsel::LDO,
            CoreSupply::Smps => Regsel::SMPS,
        };

        PWR.cr3().modify(|w| w.set_regsel(regsel));
        self.wait_for_supply(supply)
    }

    fn selected_supply(&self) -> CoreSupply {
        match PWR.svmsr().read().regs() {
            Regsel::LDO => CoreSupply::Ldo,
            Regsel::SMPS => CoreSupply::Smps,
        }
    }
}
