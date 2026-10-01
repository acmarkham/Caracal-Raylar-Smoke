//! Generic manual-chip-select adapter for the board's blocking SPI peripheral.

use core::convert::Infallible;

use embassy_time::Timer;
use embedded_hal::{
    digital::OutputPin,
    spi::{ErrorType as SpiErrorType, SpiBus},
};
use embedded_hal_async::spi::{Operation, SpiDevice};

/// Adapts a blocking SPI bus and infallible CS pin to async `SpiDevice`.
///
/// Construction performs no I/O. The board must create the CS output inactive
/// (high), as `raylar-board-v1p0` does for the Ebyte resource.
pub struct ManualCsSpiDevice<SPI, CS> {
    spi: SPI,
    cs: CS,
}

impl<SPI, CS> ManualCsSpiDevice<SPI, CS> {
    pub const fn new(spi: SPI, cs: CS) -> Self {
        Self { spi, cs }
    }

    pub fn into_inner(self) -> (SPI, CS) {
        (self.spi, self.cs)
    }
}

impl<SPI, CS> SpiErrorType for ManualCsSpiDevice<SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: OutputPin<Error = Infallible>,
{
    type Error = SPI::Error;
}

impl<SPI, CS> SpiDevice<u8> for ManualCsSpiDevice<SPI, CS>
where
    SPI: SpiBus<u8>,
    CS: OutputPin<Error = Infallible>,
{
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        let _ = self.cs.set_low();
        let mut result = Ok(());

        for operation in operations {
            result = match operation {
                Operation::Read(words) => self.spi.read(words),
                Operation::Write(words) => self.spi.write(words),
                Operation::Transfer(read, write) => self.spi.transfer(read, write),
                Operation::TransferInPlace(words) => self.spi.transfer_in_place(words),
                Operation::DelayNs(ns) => {
                    Timer::after_micros((u64::from(*ns) + 999) / 1_000).await;
                    Ok(())
                }
            };
            if result.is_err() {
                break;
            }
        }

        let _ = self.cs.set_high();
        result
    }
}
