//! L4 SDMMC v1 adapter. Filesystem and partition behavior stays upstream.
use aligned::{Aligned, A4};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use embassy_stm32::sdmmc::sd::{Card, DataBlock, StorageDevice};
use embassy_stm32::{bind_interrupts, dma, interrupt, peripherals, sdmmc};
use raylar_drivers::storage::StorageBlockDevice;

pub static DMA_IRQS: AtomicU32 = AtomicU32::new(0);
pub static WRITE_ALLOWED: AtomicBool = AtomicBool::new(false);
pub static SD_IRQS: AtomicU32 = AtomicU32::new(0);
pub static READS: AtomicU32 = AtomicU32::new(0);
pub static WRITES: AtomicU32 = AtomicU32::new(0);
pub static READ_BLOCKS: AtomicU32 = AtomicU32::new(0);
pub static WRITE_BLOCKS: AtomicU32 = AtomicU32::new(0);
pub static MAX_READ_BLOCKS: AtomicU32 = AtomicU32::new(0);
pub static MAX_WRITE_BLOCKS: AtomicU32 = AtomicU32::new(0);

pub struct DmaCounter;
impl interrupt::typelevel::Handler<interrupt::typelevel::DMA2_CHANNEL5> for DmaCounter {
    unsafe fn on_interrupt() {
        DMA_IRQS.fetch_add(1, Ordering::Relaxed);
    }
}

pub struct SdCounter;
impl interrupt::typelevel::Handler<interrupt::typelevel::SDMMC1> for SdCounter {
    unsafe fn on_interrupt() {
        SD_IRQS.fetch_add(1, Ordering::Relaxed);
    }
}

bind_interrupts!(pub struct Irqs {
    SDMMC1 => SdCounter, sdmmc::InterruptHandler<peripherals::SDMMC1>;
    DMA2_CHANNEL5 => DmaCounter, dma::InterruptHandler<peripherals::DMA2_CH5>;
});

#[derive(Debug, defmt::Format)]
pub enum Error {
    Sd(sdmmc::Error),
    WriteProhibited,
    StackGuard,
    BufferOutsideSram1,
    OutOfRange,
}

pub struct L4Sd<'a, 'b> {
    card: StorageDevice<'a, 'b, Card>,
}

impl<'a, 'b> L4Sd<'a, 'b> {
    pub fn new(card: StorageDevice<'a, 'b, Card>) -> Self {
        Self { card }
    }

    fn check_range(&self, lba: u32, blocks: usize) -> Result<(), Error> {
        if u64::from(lba) + blocks as u64 > self.card.card().csd.block_count() {
            return Err(Error::OutOfRange);
        }
        Ok(())
    }

    fn check_buffer(address: usize, bytes: usize) -> Result<(), Error> {
        if address < 0x2000_0000 || address.saturating_add(bytes) > 0x2001_8000 {
            return Err(Error::BufferOutsideSram1);
        }
        Ok(())
    }
}

impl StorageBlockDevice<512> for L4Sd<'_, '_> {
    type Error = Error;
    type Align = A4;

    async fn read(&mut self, lba: u32, data: &mut [Aligned<A4, [u8; 512]>]) -> Result<(), Error> {
        self.check_range(lba, data.len())?;
        Self::check_buffer(data.as_ptr() as usize, core::mem::size_of_val(data))?;
        // Same representation/alignment conversion as the upstream STM32 adapter.
        let blocks = unsafe {
            core::slice::from_raw_parts_mut(data.as_mut_ptr().cast::<DataBlock>(), data.len())
        };
        self.card
            .read_blocks(lba, blocks)
            .await
            .map_err(Error::Sd)?;
        READS.fetch_add(1, Ordering::Relaxed);
        READ_BLOCKS.fetch_add(data.len() as u32, Ordering::Relaxed);
        MAX_READ_BLOCKS.fetch_max(data.len() as u32, Ordering::Relaxed);
        Ok(())
    }

    async fn write(&mut self, lba: u32, data: &[Aligned<A4, [u8; 512]>]) -> Result<(), Error> {
        if !crate::stack_probe::guard_intact() {
            return Err(Error::StackGuard);
        }
        if !WRITE_ALLOWED.load(Ordering::Relaxed) {
            return Err(Error::WriteProhibited);
        }
        self.check_range(lba, data.len())?;
        Self::check_buffer(data.as_ptr() as usize, core::mem::size_of_val(data))?;
        let blocks =
            unsafe { core::slice::from_raw_parts(data.as_ptr().cast::<DataBlock>(), data.len()) };
        self.card
            .write_blocks(lba, blocks)
            .await
            .map_err(Error::Sd)?;
        WRITES.fetch_add(1, Ordering::Relaxed);
        WRITE_BLOCKS.fetch_add(data.len() as u32, Ordering::Relaxed);
        MAX_WRITE_BLOCKS.fetch_max(data.len() as u32, Ordering::Relaxed);
        Ok(())
    }

    async fn size(&mut self) -> Result<u64, Error> {
        Ok(self.card.card().csd.block_count() * 512)
    }
}
