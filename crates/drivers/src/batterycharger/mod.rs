#[cfg(feature = "stm32")]
pub mod stm32;

use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::{Receiver, Watch};
use embassy_time::{Duration, Timer};

pub const BQ25186_ADDRESS: u8 = 0x6A;
pub const DEFAULT_WATCHERS: usize = 4;
pub const DEFAULT_CHARGE_CURRENT_MA: u16 = 200;
pub const DEFAULT_INPUT_CURRENT_LIMIT_MA: u16 = 500;

const REG_STAT0: u8 = 0x00;
const REG_STAT1: u8 = 0x01;
const REG_ICHG_CTRL: u8 = 0x04;
const REG_TMR_ILIM: u8 = 0x08;

const STAT0_CHG_STAT_MASK: u8 = 0b0110_0000;
const STAT0_CHG_STAT_SHIFT: u8 = 5;
const STAT0_THERMREG_ACTIVE: u8 = 0b0000_0010;

const STAT1_VIN_OVP: u8 = 0b1000_0000;
const STAT1_BUVLO: u8 = 0b0100_0000;
const STAT1_TS_STAT_MASK: u8 = 0b0001_1000;
const STAT1_TS_STAT_SHIFT: u8 = 3;
const STAT1_SAFETY_TMR_FAULT: u8 = 0b0000_0100;

const ICHG_CTRL_CHG_DIS: u8 = 0b1000_0000;
const ICHG_CTRL_ICHG_MASK: u8 = 0b0111_1111;

const TMR_ILIM_ILIM_MASK: u8 = 0b0000_0111;

pub type ChargerMutex = CriticalSectionRawMutex;
pub type ChargerStateReceiver<'a, const WATCHERS: usize> =
    Receiver<'a, ChargerMutex, ChargerState, WATCHERS>;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeState {
    #[default]
    NotCharging,
    TrickleCharge,
    PreCharge,
    FastCharge,
    TopOff,
    ChargeComplete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ChargeFault {
    Thermal,
    InputFault,
    SafetyTimer,
    BatteryFault,
    ChargerFault,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ChargerState {
    pub charging: bool,
    pub state: ChargeState,
    pub fault: Option<ChargeFault>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChargerConfig {
    pub poll_interval: Duration,
    pub default_charge_current_ma: u16,
    pub default_input_current_limit_ma: u16,
}

impl Default for ChargerConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(1),
            default_charge_current_ma: DEFAULT_CHARGE_CURRENT_MA,
            default_input_current_limit_ma: DEFAULT_INPUT_CURRENT_LIMIT_MA,
        }
    }
}

pub struct ChargerResources<const WATCHERS: usize = DEFAULT_WATCHERS> {
    state: Watch<ChargerMutex, ChargerState, WATCHERS>,
}

impl<const WATCHERS: usize> ChargerResources<WATCHERS> {
    pub const fn new() -> Self {
        Self {
            state: Watch::new_with(ChargerState {
                charging: false,
                state: ChargeState::NotCharging,
                fault: None,
            }),
        }
    }

    pub fn state_receiver(&self) -> Option<ChargerStateReceiver<'_, WATCHERS>> {
        self.state.receiver()
    }

    pub fn state(&self) -> ChargerState {
        self.state.try_get().unwrap_or_default()
    }
}

impl<const WATCHERS: usize> Default for ChargerResources<WATCHERS> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ChargerBus {
    type Error;

    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Self::Error>;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChargerError<E> {
    Bus(E),
    InvalidChargeCurrent,
    InvalidInputCurrentLimit,
}

pub struct ChargerDriver<BUS, const WATCHERS: usize = DEFAULT_WATCHERS> {
    bus: BUS,
    resources: &'static ChargerResources<WATCHERS>,
    config: ChargerConfig,
}

impl<BUS, const WATCHERS: usize> ChargerDriver<BUS, WATCHERS> {
    pub const fn new(
        bus: BUS,
        resources: &'static ChargerResources<WATCHERS>,
        config: ChargerConfig,
    ) -> Self {
        Self {
            bus,
            resources,
            config,
        }
    }

    pub fn watch(&self) -> Option<ChargerStateReceiver<'_, WATCHERS>> {
        self.resources.state_receiver()
    }

    pub fn state(&self) -> ChargerState {
        self.resources.state()
    }
}

impl<BUS, const WATCHERS: usize> ChargerDriver<BUS, WATCHERS>
where
    BUS: ChargerBus,
{
    pub fn initialize(&mut self) -> Result<ChargerState, ChargerError<BUS::Error>> {
        self.set_charge_current(self.config.default_charge_current_ma)?;
        self.set_input_current_limit(self.config.default_input_current_limit_ma)?;
        self.refresh_state()
    }

    pub fn enable(&mut self) -> Result<ChargerState, ChargerError<BUS::Error>> {
        self.update_register(REG_ICHG_CTRL, |value| value & !ICHG_CTRL_CHG_DIS)?;
        self.refresh_state()
    }

    pub fn disable(&mut self) -> Result<ChargerState, ChargerError<BUS::Error>> {
        self.update_register(REG_ICHG_CTRL, |value| value | ICHG_CTRL_CHG_DIS)?;
        self.refresh_state()
    }

    pub fn set_charge_current(&mut self, milliamps: u16) -> Result<u16, ChargerError<BUS::Error>> {
        let code =
            encode_charge_current_ma(milliamps).map_err(|_| ChargerError::InvalidChargeCurrent)?;
        self.update_register(REG_ICHG_CTRL, |value| {
            (value & ICHG_CTRL_CHG_DIS) | (code & ICHG_CTRL_ICHG_MASK)
        })?;
        Ok(decode_charge_current_code(code))
    }

    pub fn set_input_current_limit(
        &mut self,
        milliamps: u16,
    ) -> Result<u16, ChargerError<BUS::Error>> {
        let code = encode_input_current_limit_ma(milliamps)
            .map_err(|_| ChargerError::InvalidInputCurrentLimit)?;
        self.update_register(REG_TMR_ILIM, |value| {
            (value & !TMR_ILIM_ILIM_MASK) | (code & TMR_ILIM_ILIM_MASK)
        })?;
        Ok(decode_input_current_limit_code(code))
    }

    pub fn refresh_state(&mut self) -> Result<ChargerState, ChargerError<BUS::Error>> {
        let stat0 = self.read_register(REG_STAT0)?;
        let stat1 = self.read_register(REG_STAT1)?;
        let ichg_ctrl = self.read_register(REG_ICHG_CTRL)?;
        let state = decode_state(stat0, stat1, ichg_ctrl);
        self.resources.state.sender().send(state);
        Ok(state)
    }

    pub async fn run(mut self) -> ! {
        if self.initialize().is_err() {
            self.publish_charger_fault();
        }

        loop {
            if self.refresh_state().is_err() {
                self.publish_charger_fault();
            }
            Timer::after(self.config.poll_interval).await;
        }
    }

    fn read_register(&mut self, register: u8) -> Result<u8, ChargerError<BUS::Error>> {
        self.bus.read_register(register).map_err(ChargerError::Bus)
    }

    fn write_register(&mut self, register: u8, value: u8) -> Result<(), ChargerError<BUS::Error>> {
        self.bus
            .write_register(register, value)
            .map_err(ChargerError::Bus)
    }

    fn update_register<F>(&mut self, register: u8, f: F) -> Result<(), ChargerError<BUS::Error>>
    where
        F: FnOnce(u8) -> u8,
    {
        let value = self.read_register(register)?;
        self.write_register(register, f(value))
    }

    fn publish_charger_fault(&self) {
        self.resources.state.sender().send(ChargerState {
            charging: false,
            state: ChargeState::NotCharging,
            fault: Some(ChargeFault::ChargerFault),
        });
    }
}

fn decode_state(stat0: u8, stat1: u8, ichg_ctrl: u8) -> ChargerState {
    let disabled = (ichg_ctrl & ICHG_CTRL_CHG_DIS) != 0;
    let charge_stat = (stat0 & STAT0_CHG_STAT_MASK) >> STAT0_CHG_STAT_SHIFT;
    let state = match (disabled, charge_stat) {
        (true, _) => ChargeState::NotCharging,
        (false, 0b00) => ChargeState::NotCharging,
        (false, 0b01) => ChargeState::FastCharge,
        (false, 0b10) => ChargeState::TopOff,
        (false, 0b11) => ChargeState::ChargeComplete,
        _ => ChargeState::NotCharging,
    };

    ChargerState {
        charging: matches!(
            state,
            ChargeState::TrickleCharge
                | ChargeState::PreCharge
                | ChargeState::FastCharge
                | ChargeState::TopOff
        ),
        state,
        fault: decode_fault(stat0, stat1),
    }
}

fn decode_fault(stat0: u8, stat1: u8) -> Option<ChargeFault> {
    let ts_stat = (stat1 & STAT1_TS_STAT_MASK) >> STAT1_TS_STAT_SHIFT;

    if (stat1 & STAT1_SAFETY_TMR_FAULT) != 0 {
        Some(ChargeFault::SafetyTimer)
    } else if (stat1 & STAT1_BUVLO) != 0 {
        Some(ChargeFault::BatteryFault)
    } else if (stat1 & STAT1_VIN_OVP) != 0 {
        Some(ChargeFault::InputFault)
    } else if ts_stat != 0 || (stat0 & STAT0_THERMREG_ACTIVE) != 0 {
        Some(ChargeFault::Thermal)
    } else {
        None
    }
}

fn encode_charge_current_ma(milliamps: u16) -> Result<u8, ChargerError<core::convert::Infallible>> {
    if !(5..=1_000).contains(&milliamps) {
        return Err(ChargerError::InvalidChargeCurrent);
    }

    let code = if milliamps <= 35 {
        milliamps - 5
    } else if milliamps < 40 {
        30
    } else {
        ((milliamps - 40) / 10) + 31
    };

    Ok(code as u8)
}

fn decode_charge_current_code(code: u8) -> u16 {
    let code = code & ICHG_CTRL_ICHG_MASK;
    if code <= 30 {
        (code as u16) + 5
    } else {
        40 + (((code as u16) - 31) * 10)
    }
}

fn encode_input_current_limit_ma(
    milliamps: u16,
) -> Result<u8, ChargerError<core::convert::Infallible>> {
    match milliamps {
        50 => Ok(0b000),
        100 => Ok(0b001),
        200 => Ok(0b010),
        300 => Ok(0b011),
        400 => Ok(0b100),
        500 => Ok(0b101),
        665 => Ok(0b110),
        1050 => Ok(0b111),
        _ => Err(ChargerError::InvalidInputCurrentLimit),
    }
}

fn decode_input_current_limit_code(code: u8) -> u16 {
    match code & TMR_ILIM_ILIM_MASK {
        0b000 => 50,
        0b001 => 100,
        0b010 => 200,
        0b011 => 300,
        0b100 => 400,
        0b101 => 500,
        0b110 => 665,
        0b111 => 1050,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn charge_current_encoding_matches_datasheet_formula() {
        assert_eq!(encode_charge_current_ma(5), Ok(0));
        assert_eq!(encode_charge_current_ma(35), Ok(30));
        assert_eq!(encode_charge_current_ma(36), Ok(30));
        assert_eq!(encode_charge_current_ma(40), Ok(31));
        assert_eq!(encode_charge_current_ma(200), Ok(47));
        assert_eq!(decode_charge_current_code(47), 200);
        assert_eq!(encode_charge_current_ma(1_000), Ok(127));
    }

    #[test]
    fn invalid_charge_current_is_rejected() {
        assert_eq!(
            encode_charge_current_ma(4),
            Err(ChargerError::InvalidChargeCurrent)
        );
        assert_eq!(
            encode_charge_current_ma(1_001),
            Err(ChargerError::InvalidChargeCurrent)
        );
    }

    #[test]
    fn input_current_limit_uses_supported_steps() {
        assert_eq!(encode_input_current_limit_ma(50), Ok(0));
        assert_eq!(encode_input_current_limit_ma(500), Ok(5));
        assert_eq!(encode_input_current_limit_ma(1_050), Ok(7));
        assert_eq!(
            encode_input_current_limit_ma(600),
            Err(ChargerError::InvalidInputCurrentLimit)
        );
    }

    #[test]
    fn status_decoding_maps_charging_states() {
        assert_eq!(
            decode_state(0b0010_0000, 0, 0),
            ChargerState {
                charging: true,
                state: ChargeState::FastCharge,
                fault: None,
            }
        );
        assert_eq!(
            decode_state(0b0100_0000, 0, 0),
            ChargerState {
                charging: true,
                state: ChargeState::TopOff,
                fault: None,
            }
        );
        assert_eq!(
            decode_state(0b0110_0000, 0, 0),
            ChargerState {
                charging: false,
                state: ChargeState::ChargeComplete,
                fault: None,
            }
        );
        assert_eq!(
            decode_state(0b0110_0000, 0, ICHG_CTRL_CHG_DIS).state,
            ChargeState::NotCharging
        );
    }

    #[test]
    fn status_decoding_prioritizes_faults() {
        assert_eq!(
            decode_state(0, STAT1_SAFETY_TMR_FAULT, 0).fault,
            Some(ChargeFault::SafetyTimer)
        );
        assert_eq!(
            decode_state(0, STAT1_BUVLO, 0).fault,
            Some(ChargeFault::BatteryFault)
        );
        assert_eq!(
            decode_state(0, STAT1_VIN_OVP, 0).fault,
            Some(ChargeFault::InputFault)
        );
        assert_eq!(
            decode_state(STAT0_THERMREG_ACTIVE, 0, 0).fault,
            Some(ChargeFault::Thermal)
        );
    }
}
