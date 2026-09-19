//! Polling driver for the ST LIS2MDL three-axis magnetometer.

use embedded_hal::i2c::I2c;

pub const DEVICE_ADDRESS: u8 = 0x1E;
pub const DEVICE_ID: u8 = 0x40;

const REG_WHO_AM_I: u8 = 0x4F;
const REG_CFG_A: u8 = 0x60;
const REG_CFG_B: u8 = 0x61;
const REG_CFG_C: u8 = 0x62;
const REG_STATUS: u8 = 0x67;
const REG_OUT_X_L: u8 = 0x68;
const REG_TEMP_OUT_L: u8 = 0x6E;

const AUTO_INCREMENT: u8 = 0x80;
const CFG_A_MODE_CONTINUOUS: u8 = 0b00;
const CFG_A_MODE_POWER_DOWN: u8 = 0b10;
const CFG_A_ODR_SHIFT: u8 = 2;
const CFG_A_TEMP_COMPENSATION: u8 = 1 << 7;
const CFG_B_OFFSET_CANCELLATION_EVERY_ODR: u8 = 0b01 << 1;
const CFG_C_BLOCK_DATA_UPDATE: u8 = 1 << 4;
const STATUS_XYZ_NEW_DATA: u8 = 1 << 3;
const NANOTESLA_PER_LSB: i32 = 150;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputDataRate {
    #[default]
    Hz10,
    Hz20,
    Hz50,
    Hz100,
}

impl OutputDataRate {
    const fn register_value(self) -> u8 {
        match self {
            Self::Hz10 => 0,
            Self::Hz20 => 1,
            Self::Hz50 => 2,
            Self::Hz100 => 3,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MagnetometerConfig {
    pub odr: OutputDataRate,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawMagneticField {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

impl RawMagneticField {
    /// Converts native sample counts to nanotesla without floating point.
    pub fn to_nanotesla(self) -> MagneticField {
        MagneticField {
            x_nanotesla: i32::from(self.x) * NANOTESLA_PER_LSB,
            y_nanotesla: i32::from(self.y) * NANOTESLA_PER_LSB,
            z_nanotesla: i32::from(self.z) * NANOTESLA_PER_LSB,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MagneticField {
    pub x_nanotesla: i32,
    pub y_nanotesla: i32,
    pub z_nanotesla: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DieTemperature {
    /// Signed 12-bit temperature sample.
    pub raw: i16,
    pub milli_celsius: i32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error<E> {
    Bus(E),
    DeviceIdMismatch { observed: u8 },
    NotEnabled,
}

pub struct Lis2mdl<I2C> {
    i2c: I2C,
    config: MagnetometerConfig,
    enabled: bool,
}

impl<I2C> Lis2mdl<I2C>
where
    I2C: I2c,
{
    /// Verifies the device, applies fixed defaults, and leaves it powered down.
    pub fn new(i2c: I2C) -> Result<Self, Error<I2C::Error>> {
        let mut driver = Self {
            i2c,
            config: MagnetometerConfig::default(),
            enabled: false,
        };
        let observed = driver.read_register(REG_WHO_AM_I)?;
        if observed != DEVICE_ID {
            return Err(Error::DeviceIdMismatch { observed });
        }

        driver.write_cfg_a(false, driver.config.odr)?;
        driver.write_fixed_configuration()?;
        Ok(driver)
    }

    pub fn configure(&mut self, config: MagnetometerConfig) -> Result<(), Error<I2C::Error>> {
        if self.enabled {
            self.write_cfg_a(true, config.odr)?;
        }
        self.config = config;
        Ok(())
    }

    pub fn config(&self) -> MagnetometerConfig {
        self.config
    }

    pub fn turn_on(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_fixed_configuration()?;
        self.write_cfg_a(true, self.config.odr)?;
        self.enabled = true;
        Ok(())
    }

    pub fn turn_off(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_cfg_a(false, self.config.odr)?;
        self.enabled = false;
        Ok(())
    }

    pub fn is_on(&self) -> bool {
        self.enabled
    }

    pub fn is_data_ready(&mut self) -> Result<bool, Error<I2C::Error>> {
        self.require_enabled()?;
        Ok((self.read_register(REG_STATUS)? & STATUS_XYZ_NEW_DATA) != 0)
    }

    pub fn read_raw_magnetic_field(&mut self) -> Result<RawMagneticField, Error<I2C::Error>> {
        self.require_enabled()?;
        let mut bytes = [0; 6];
        self.read_registers(REG_OUT_X_L, &mut bytes)?;
        Ok(RawMagneticField {
            x: i16::from_le_bytes([bytes[0], bytes[1]]),
            y: i16::from_le_bytes([bytes[2], bytes[3]]),
            z: i16::from_le_bytes([bytes[4], bytes[5]]),
        })
    }

    pub fn read_magnetic_field(&mut self) -> Result<MagneticField, Error<I2C::Error>> {
        Ok(self.read_raw_magnetic_field()?.to_nanotesla())
    }

    pub fn read_die_temperature(&mut self) -> Result<DieTemperature, Error<I2C::Error>> {
        self.require_enabled()?;
        let mut bytes = [0; 2];
        self.read_registers(REG_TEMP_OUT_L, &mut bytes)?;
        let raw = (i16::from_le_bytes(bytes) << 4) >> 4;
        Ok(DieTemperature {
            raw,
            milli_celsius: 25_000 + (i32::from(raw) * 125),
        })
    }

    pub fn into_inner(self) -> I2C {
        self.i2c
    }

    fn require_enabled(&self) -> Result<(), Error<I2C::Error>> {
        if self.enabled {
            Ok(())
        } else {
            Err(Error::NotEnabled)
        }
    }

    fn write_fixed_configuration(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_register(REG_CFG_B, CFG_B_OFFSET_CANCELLATION_EVERY_ODR)?;
        self.write_register(REG_CFG_C, CFG_C_BLOCK_DATA_UPDATE)
    }

    fn write_cfg_a(&mut self, enabled: bool, odr: OutputDataRate) -> Result<(), Error<I2C::Error>> {
        let mode = if enabled {
            CFG_A_MODE_CONTINUOUS
        } else {
            CFG_A_MODE_POWER_DOWN
        };
        let value = CFG_A_TEMP_COMPENSATION | (odr.register_value() << CFG_A_ODR_SHIFT) | mode;
        self.write_register(REG_CFG_A, value)
    }

    fn read_register(&mut self, register: u8) -> Result<u8, Error<I2C::Error>> {
        let mut value = [0];
        self.i2c
            .write_read(DEVICE_ADDRESS, &[register], &mut value)
            .map_err(Error::Bus)?;
        Ok(value[0])
    }

    fn read_registers(
        &mut self,
        first_register: u8,
        values: &mut [u8],
    ) -> Result<(), Error<I2C::Error>> {
        self.i2c
            .write_read(DEVICE_ADDRESS, &[first_register | AUTO_INCREMENT], values)
            .map_err(Error::Bus)
    }

    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Error<I2C::Error>> {
        self.i2c
            .write(DEVICE_ADDRESS, &[register, value])
            .map_err(Error::Bus)
    }
}

#[cfg(test)]
mod tests;
