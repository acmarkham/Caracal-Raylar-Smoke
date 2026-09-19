//! Polling driver for the ST LIS2HH12 three-axis accelerometer.

use embedded_hal::i2c::I2c;

pub const DEVICE_ADDRESS: u8 = 0x1D;
pub const DEVICE_ID: u8 = 0x41;

const REG_TEMP_L: u8 = 0x0B;
const REG_WHO_AM_I: u8 = 0x0F;
const REG_CTRL1: u8 = 0x20;
const REG_CTRL4: u8 = 0x23;
const REG_STATUS: u8 = 0x27;
const REG_OUT_X_L: u8 = 0x28;

const AUTO_INCREMENT: u8 = 0x80;
const CTRL1_BDU_XYZ: u8 = 0x0F;
const CTRL1_ODR_SHIFT: u8 = 4;
const CTRL4_IF_ADD_INC: u8 = 1 << 2;
const CTRL4_FS_MASK: u8 = 0b0011_0000;
const CTRL4_FS_SHIFT: u8 = 4;
const STATUS_XYZ_NEW_DATA: u8 = 1 << 3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FullScale {
    #[default]
    G2,
    G4,
    G8,
}

impl FullScale {
    const fn register_value(self) -> u8 {
        match self {
            Self::G2 => 0,
            Self::G4 => 0b10,
            Self::G8 => 0b11,
        }
    }

    const fn sensitivity_ug(self) -> i32 {
        match self {
            Self::G2 => 61,
            Self::G4 => 122,
            Self::G8 => 244,
        }
    }

    /// Converts native sample counts to milli-g. Gravity is included.
    pub fn convert(self, raw: RawAcceleration) -> Acceleration {
        let sensitivity = self.sensitivity_ug();
        Acceleration {
            x_mg: micro_g_to_milli_g(raw.x, sensitivity),
            y_mg: micro_g_to_milli_g(raw.y, sensitivity),
            z_mg: micro_g_to_milli_g(raw.z, sensitivity),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum OutputDataRate {
    #[default]
    Hz10,
    Hz50,
    Hz100,
    Hz200,
    Hz400,
    Hz800,
}

impl OutputDataRate {
    const fn register_value(self) -> u8 {
        match self {
            Self::Hz10 => 1,
            Self::Hz50 => 2,
            Self::Hz100 => 3,
            Self::Hz200 => 4,
            Self::Hz400 => 5,
            Self::Hz800 => 6,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct AccelerometerConfig {
    pub full_scale: FullScale,
    pub odr: OutputDataRate,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct RawAcceleration {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Acceleration {
    pub x_mg: i32,
    pub y_mg: i32,
    pub z_mg: i32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct DieTemperature {
    /// Signed 11-bit temperature sample after removing the five padding bits.
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

pub struct Lis2hh12<I2C> {
    i2c: I2C,
    config: AccelerometerConfig,
    enabled: bool,
}

impl<I2C> Lis2hh12<I2C>
where
    I2C: I2c,
{
    /// Verifies the device, applies default settings, and leaves it powered down.
    pub fn new(i2c: I2C) -> Result<Self, Error<I2C::Error>> {
        let mut driver = Self {
            i2c,
            config: AccelerometerConfig::default(),
            enabled: false,
        };
        let observed = driver.read_register(REG_WHO_AM_I)?;
        if observed != DEVICE_ID {
            return Err(Error::DeviceIdMismatch { observed });
        }
        driver.write_ctrl1(false, driver.config.odr)?;
        driver.write_full_scale(driver.config.full_scale)?;
        Ok(driver)
    }

    pub fn configure(&mut self, config: AccelerometerConfig) -> Result<(), Error<I2C::Error>> {
        if self.enabled {
            self.write_full_scale(config.full_scale)?;
            // Keep conversion state aligned with hardware even if the following
            // ODR write fails; I2C register updates cannot be atomic.
            self.config.full_scale = config.full_scale;
            self.write_ctrl1(true, config.odr)?;
            self.config.odr = config.odr;
        } else {
            self.config = config;
        }
        Ok(())
    }

    pub fn config(&self) -> AccelerometerConfig {
        self.config
    }

    pub fn turn_on(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_full_scale(self.config.full_scale)?;
        self.write_ctrl1(true, self.config.odr)?;
        self.enabled = true;
        Ok(())
    }

    pub fn turn_off(&mut self) -> Result<(), Error<I2C::Error>> {
        self.write_ctrl1(false, self.config.odr)?;
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

    pub fn read_raw_acceleration(&mut self) -> Result<RawAcceleration, Error<I2C::Error>> {
        self.require_enabled()?;
        let mut bytes = [0; 6];
        self.read_registers(REG_OUT_X_L, &mut bytes)?;
        Ok(RawAcceleration {
            x: i16::from_le_bytes([bytes[0], bytes[1]]),
            y: i16::from_le_bytes([bytes[2], bytes[3]]),
            z: i16::from_le_bytes([bytes[4], bytes[5]]),
        })
    }

    pub fn read_acceleration(&mut self) -> Result<Acceleration, Error<I2C::Error>> {
        let raw = self.read_raw_acceleration()?;
        Ok(self.config.full_scale.convert(raw))
    }

    pub fn read_die_temperature(&mut self) -> Result<DieTemperature, Error<I2C::Error>> {
        self.require_enabled()?;
        let mut bytes = [0; 2];
        self.read_registers(REG_TEMP_L, &mut bytes)?;
        let raw = i16::from_le_bytes(bytes) >> 5;
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

    fn write_ctrl1(&mut self, enabled: bool, odr: OutputDataRate) -> Result<(), Error<I2C::Error>> {
        let odr = if enabled { odr.register_value() } else { 0 };
        self.write_register(REG_CTRL1, (odr << CTRL1_ODR_SHIFT) | CTRL1_BDU_XYZ)
    }

    fn write_full_scale(&mut self, scale: FullScale) -> Result<(), Error<I2C::Error>> {
        let current = self.read_register(REG_CTRL4)?;
        let value = (current & !CTRL4_FS_MASK)
            | (scale.register_value() << CTRL4_FS_SHIFT)
            | CTRL4_IF_ADD_INC;
        self.write_register(REG_CTRL4, value)
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

fn micro_g_to_milli_g(raw: i16, sensitivity_ug: i32) -> i32 {
    let micro_g = i32::from(raw) * sensitivity_ug;
    if micro_g >= 0 {
        (micro_g + 500) / 1_000
    } else {
        (micro_g - 500) / 1_000
    }
}

#[cfg(test)]
mod tests;
