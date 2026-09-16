use aligned::{A4, Aligned};
use embassy_stm32::sdmmc::Error as SdError;
use embassy_stm32::sdmmc::sd::{Card, DataBlock, StorageDevice};
use exfat_slim::asynchronous::BlockDevice;

use super::StorageDeviceIdentity;

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

    pub fn device_identity(&self) -> StorageDeviceIdentity {
        let card = self.card.card();
        let (manufacture_month, manufacture_year) = card.cid.manufacturing_date();
        StorageDeviceIdentity {
            manufacturer_id: card.cid.manufacturer_id(),
            oem_id: fixed_ascii(card.cid.oem_id()),
            product_name: fixed_ascii(card.cid.product_name()),
            product_revision: card.cid.product_revision(),
            serial_number: card.cid.serial(),
            manufacture_year,
            manufacture_month,
            capacity_bytes: card.csd.card_size(),
        }
    }
}

fn fixed_ascii<const N: usize>(value: &str) -> Option<[u8; N]> {
    let bytes = value.as_bytes();
    if bytes.len() != N {
        return None;
    }
    let mut result = [0; N];
    result.copy_from_slice(bytes);
    Some(result)
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
