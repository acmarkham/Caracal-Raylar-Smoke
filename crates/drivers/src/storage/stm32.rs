use aligned::{Aligned, A4};
use embassy_stm32::sdmmc::sd::{Card, DataBlock, StorageDevice};
use embassy_stm32::sdmmc::Error as SdError;
use exfat_slim::asynchronous::BlockDevice;

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub enum Stm32SdError {
    Sd(SdError),
}

pub struct Stm32SdBlockDevice<'a, 'b> {
    card: StorageDevice<'a, 'b, Card>,
}

impl<'a, 'b> Stm32SdBlockDevice<'a, 'b> {
    pub const fn new(card: StorageDevice<'a, 'b, Card>) -> Self {
        Self { card }
    }

    pub fn into_inner(self) -> StorageDevice<'a, 'b, Card> {
        self.card
    }
}

impl BlockDevice<512> for Stm32SdBlockDevice<'_, '_> {
    type Error = Stm32SdError;
    type Align = A4;

    async fn read(
        &mut self,
        block_address: u32,
        data: &mut [Aligned<Self::Align, [u8; 512]>],
    ) -> Result<(), Self::Error> {
        let blocks = unsafe {
            core::slice::from_raw_parts_mut(data.as_mut_ptr().cast::<DataBlock>(), data.len())
        };
        self.card
            .read_blocks(block_address, blocks)
            .await
            .map_err(Stm32SdError::Sd)
    }

    async fn write(
        &mut self,
        block_address: u32,
        data: &[Aligned<Self::Align, [u8; 512]>],
    ) -> Result<(), Self::Error> {
        let blocks =
            unsafe { core::slice::from_raw_parts(data.as_ptr().cast::<DataBlock>(), data.len()) };
        self.card
            .write_blocks(block_address, blocks)
            .await
            .map_err(Stm32SdError::Sd)
    }

    async fn size(&mut self) -> Result<u64, Self::Error> {
        Ok(self.card.card().csd.block_count() as u64 * 512)
    }
}
