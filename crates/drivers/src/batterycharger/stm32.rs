use embassy_stm32::i2c::{mode::Master, Error, I2c};
use embassy_stm32::mode::Blocking;

use crate::batterycharger::{ChargerBus, BQ25186_ADDRESS};

pub type BlockingMasterI2c = I2c<'static, Blocking, Master>;

impl ChargerBus for BlockingMasterI2c {
    type Error = Error;

    fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut value = [0u8; 1];
        self.blocking_write_read(BQ25186_ADDRESS, &[register], &mut value)?;
        Ok(value[0])
    }

    fn write_register(&mut self, register: u8, value: u8) -> Result<(), Self::Error> {
        self.blocking_write(BQ25186_ADDRESS, &[register, value])
    }
}
