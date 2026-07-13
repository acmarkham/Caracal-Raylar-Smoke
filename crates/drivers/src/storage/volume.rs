use aligned::Aligned;
use exfat_slim::asynchronous::BlockDevice;

use super::{PartitionedDeviceError, VolumeDetectError};

const EXFAT_JUMP_BOOT: [u8; 3] = [0xeb, 0x76, 0x90];
const EXFAT_OEM_NAME: &[u8; 8] = b"EXFAT   ";
const MBR_SIGNATURE: [u8; 2] = [0x55, 0xaa];

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExfatVolume {
    pub start_lba: u32,
    pub block_count: u32,
}

pub async fn detect_exfat_volume<D, const SIZE: usize>(
    device: &mut D,
) -> Result<ExfatVolume, VolumeDetectError<D::Error>>
where
    D: BlockDevice<SIZE>,
{
    if SIZE != 512 {
        return Err(VolumeDetectError::UnsupportedSectorSize);
    }

    let total_blocks = device.size().await.map_err(VolumeDetectError::Io)? / SIZE as u64;
    let total_blocks_u32 =
        u32::try_from(total_blocks).map_err(|_| VolumeDetectError::CardTooLarge)?;

    let mut block0 = [Aligned::<D::Align, [u8; SIZE]>([0; SIZE])];
    device
        .read(0, &mut block0)
        .await
        .map_err(VolumeDetectError::Io)?;
    let bytes = block0[0].as_slice();

    if looks_like_exfat_vbr(bytes) {
        validate_exfat_vbr(bytes, total_blocks).map_err(|_| VolumeDetectError::InvalidVolume)?;
        return Ok(ExfatVolume {
            start_lba: 0,
            block_count: total_blocks_u32,
        });
    }

    if bytes[510..512] != MBR_SIGNATURE {
        return Err(VolumeDetectError::NoFilesystem);
    }

    for index in 0..4 {
        let offset = 446 + index * 16;
        let start_lba = read_u32_le(&bytes[offset + 8..offset + 12]);
        let block_count = read_u32_le(&bytes[offset + 12..offset + 16]);
        if bytes[offset + 4] == 0 || start_lba == 0 || block_count == 0 {
            continue;
        }

        let end = u64::from(start_lba) + u64::from(block_count);
        if end > total_blocks {
            continue;
        }

        let mut boot = [Aligned::<D::Align, [u8; SIZE]>([0; SIZE])];
        device
            .read(start_lba, &mut boot)
            .await
            .map_err(VolumeDetectError::Io)?;
        let boot = boot[0].as_slice();
        if looks_like_exfat_vbr(boot) && validate_exfat_vbr(boot, u64::from(block_count)).is_ok() {
            return Ok(ExfatVolume {
                start_lba,
                block_count,
            });
        }
    }

    Err(VolumeDetectError::NoFilesystem)
}

pub struct PartitionedBlockDevice<D> {
    inner: D,
    volume: ExfatVolume,
}

impl<D> PartitionedBlockDevice<D> {
    pub const fn new(inner: D, volume: ExfatVolume) -> Self {
        Self { inner, volume }
    }

    pub fn into_inner(self) -> D {
        self.inner
    }

    pub const fn volume(&self) -> ExfatVolume {
        self.volume
    }
}

impl<D, const SIZE: usize> BlockDevice<SIZE> for PartitionedBlockDevice<D>
where
    D: BlockDevice<SIZE>,
{
    type Error = PartitionedDeviceError<D::Error>;
    type Align = D::Align;

    async fn read(
        &mut self,
        block_address: u32,
        data: &mut [Aligned<Self::Align, [u8; SIZE]>],
    ) -> Result<(), Self::Error> {
        let physical_lba = self.translate_sized(block_address, data.len())?;
        self.inner
            .read(physical_lba, data)
            .await
            .map_err(PartitionedDeviceError::Inner)
    }

    async fn write(
        &mut self,
        block_address: u32,
        data: &[Aligned<Self::Align, [u8; SIZE]>],
    ) -> Result<(), Self::Error> {
        let physical_lba = self.translate_sized(block_address, data.len())?;
        self.inner
            .write(physical_lba, data)
            .await
            .map_err(PartitionedDeviceError::Inner)
    }

    async fn size(&mut self) -> Result<u64, Self::Error> {
        Ok(self.volume.block_count as u64 * SIZE as u64)
    }
}

impl<D> PartitionedBlockDevice<D> {
    fn translate_sized<const SIZE: usize>(
        &self,
        block_address: u32,
        blocks: usize,
    ) -> Result<u32, PartitionedDeviceError<D::Error>>
    where
        D: BlockDevice<SIZE>,
    {
        let blocks = u32::try_from(blocks).map_err(|_| PartitionedDeviceError::OutOfRange)?;
        if block_address
            .checked_add(blocks)
            .map_or(true, |end| end > self.volume.block_count)
        {
            return Err(PartitionedDeviceError::OutOfRange);
        }

        self.volume
            .start_lba
            .checked_add(block_address)
            .ok_or(PartitionedDeviceError::OutOfRange)
    }
}

fn looks_like_exfat_vbr(bytes: &[u8]) -> bool {
    bytes[0..3] == EXFAT_JUMP_BOOT
        && &bytes[3..11] == EXFAT_OEM_NAME
        && bytes[510..512] == MBR_SIGNATURE
}

fn validate_exfat_vbr(bytes: &[u8], max_blocks: u64) -> Result<(), ()> {
    if !looks_like_exfat_vbr(bytes) {
        return Err(());
    }

    let volume_length = read_u64_le(&bytes[72..80]);
    let fat_offset = read_u32_le(&bytes[80..84]);
    let fat_length = read_u32_le(&bytes[84..88]);
    let cluster_heap_offset = read_u32_le(&bytes[88..92]);
    let cluster_count = read_u32_le(&bytes[92..96]);
    let first_cluster_of_root_dir = read_u32_le(&bytes[96..100]);
    let bytes_per_sector_shift = bytes[108];
    let sectors_per_cluster_shift = bytes[109];
    let number_of_fats = bytes[110];

    if bytes_per_sector_shift != 9 || sectors_per_cluster_shift > 16 || number_of_fats == 0 {
        return Err(());
    }
    if volume_length == 0 || volume_length > max_blocks {
        return Err(());
    }
    if fat_offset == 0 || fat_length == 0 || cluster_heap_offset == 0 || cluster_count == 0 {
        return Err(());
    }
    if first_cluster_of_root_dir < 2 || first_cluster_of_root_dir >= cluster_count + 2 {
        return Err(());
    }

    let sectors_per_cluster = 1u64 << sectors_per_cluster_shift;
    let heap_end = u64::from(cluster_heap_offset) + u64::from(cluster_count) * sectors_per_cluster;
    let fat_end = u64::from(fat_offset) + u64::from(fat_length) * u64::from(number_of_fats);

    if fat_end > volume_length || heap_end > volume_length {
        return Err(());
    }

    Ok(())
}

fn read_u64_le(bytes: &[u8]) -> u64 {
    u64::from_le_bytes([
        bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
    ])
}

fn read_u32_le(bytes: &[u8]) -> u32 {
    u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]])
}
