//! STM32 core facilities that are configured once during platform startup.
//!
//! Core-supply selection is intentionally separate from the runtime power
//! management service. It changes how VCORE is generated and is therefore a
//! board/startup decision rather than an energy-policy decision.

#[cfg(feature = "stm32")]
pub mod stm32;

/// Regulator used to generate the MCU core supply.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoreSupply {
    /// Use the internal linear regulator. This is safe for packages and boards
    /// that do not provide the external SMPS components.
    #[default]
    Ldo,
    /// Use the internal switched-mode regulator. The MCU package must support
    /// SMPS and the board must fit the required external components.
    Smps,
}

/// Startup configuration for the MCU core driver.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct CoreConfig {
    pub supply: CoreSupply,
}

/// Failure to complete a requested regulator transition.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CoreError {
    TransitionTimeout {
        requested: CoreSupply,
        observed: CoreSupply,
    },
}

/// Hardware-independent contract implemented by an MCU core driver.
pub trait CoreSupplyControl {
    fn select_supply(&mut self, supply: CoreSupply) -> Result<(), CoreError>;
    fn selected_supply(&self) -> CoreSupply;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ldo_is_the_safe_library_default() {
        assert_eq!(CoreConfig::default().supply, CoreSupply::Ldo);
    }
}
