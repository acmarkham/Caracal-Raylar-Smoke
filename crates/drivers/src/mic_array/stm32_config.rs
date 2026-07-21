//! STM32U5 MDF register and request mapping.

pub(super) const MDF1_BASE: usize = 0x4002_5000;
pub(super) const MDF_GCR: usize = 0x0000;
pub(super) const MDF_CKGCR: usize = 0x0004;
pub(super) const MDF_FILTER_STRIDE: usize = 0x0080;
pub(super) const MDF_SITFCR0: usize = 0x0080;
pub(super) const MDF_BSMXCR0: usize = 0x0084;
pub(super) const MDF_DFLTCR0: usize = 0x0088;
pub(super) const MDF_DFLTCICR0: usize = 0x008c;
pub(super) const MDF_DFLTRSFR0: usize = 0x0090;
pub(super) const MDF_DFLTDR0: usize = 0x00f0;

pub(super) const FILTERS: [usize; 6] = [0, 1, 2, 3, 4, 5];
pub(super) const DMA_REQUESTS: [u8; 6] = [92, 93, 94, 95, 96, 97];
// BS0_R, BS1_R, BS1_F, BS2_R, BS2_F, BS3_R.
pub(super) const BITSTREAM_SELECTS: [u32; 6] = [0, 2, 3, 4, 5, 6];
pub(super) const MDF_KERNEL_HZ: u32 = 80_000_000;

pub(super) const fn register(base: usize, filter: usize) -> usize {
    base + filter * MDF_FILTER_STRIDE
}
