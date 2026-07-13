// SDMMC1 exFAT root directory smoke test.
//
// Reads an exFAT volume from the microSD card and logs the root directory
// entries. Supports cards formatted as a raw exFAT volume or with an MBR
// partition whose first sector is exFAT.

#![no_std]
#![no_main]

extern crate alloc;

use aligned::{Aligned, A4};
use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Input;
use embassy_stm32::rcc::mux::Sdmmcsel;
use embassy_stm32::rcc::*;
use embassy_stm32::sdmmc::sd::{Card, CmdBlock, DataBlock, StorageDevice};
use embassy_stm32::sdmmc::{Config as SdmmcConfig, Error as SdError, Sdmmc};
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use exfat_slim::asynchronous::file::OpenOptions;
use exfat_slim::asynchronous::file_system::FileSystem;
use exfat_slim::asynchronous::BlockDevice;
use raylar_board_v1p0::{Board, Irqs, SdCard};
use {defmt_rtt as _, panic_probe as _};

const SD_TARGET_FREQ: Hertz = mhz(24);
const BLOCK_BYTES: usize = 512;
const EXFAT_CACHE_BLOCKS: usize = 8;
const HEAP_BYTES: usize = 64 * 1024;
const FILE_BYTES: usize = 8 * 1024;
const WRITE_INTERVAL_SECS: u64 = 5;
const EXFAT_JUMP_BOOT: [u8; 3] = [0xeb, 0x76, 0x90];
const EXFAT_OEM_NAME: &[u8; 8] = b"EXFAT   ";
const MBR_SIGNATURE: [u8; 2] = [0x55, 0xaa];

#[global_allocator]
static HEAP: Heap = Heap::empty();

static mut FILE_DATA: [u8; FILE_BYTES] = [0; FILE_BYTES];

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }

    let mut config = embassy_stm32::Config::default();

    config.rcc.hse = Some(Hse {
        freq: mhz(16),
        mode: HseMode::Oscillator,
    });

    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL18,
        divp: Some(PllDiv::DIV6), // 48 MHz SDMMC kernel clock through PLL1_P.
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2), // 144 MHz SYSCLK.
    });

    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.hsi48 = Some(Hsi48Config::new());
    config.rcc.mux.sdmmcsel = Sdmmcsel::PLL1_P;

    let p = embassy_stm32::init(config);
    let Board { sd, .. } = Board::new(p);

    info!("SDMMC1 exFAT write test started");
    run_exfat_write(sd).await
}

async fn run_exfat_write(mut sd: SdCard<'static>) -> ! {
    sd.power.set_high();

    if card_absent(&sd.switch) {
        info!("Card absent: SD_SW is high, leaving SD power off");
        pending_forever().await;
    }

    info!("Card present: SD_SW is low");
    let mut sdmmc_config = SdmmcConfig::default();
    sdmmc_config.data_transfer_timeout = 120_000_000;
    let mut sdmmc = Sdmmc::new_4bit(
        sd.sdmmc,
        Irqs,
        sd.clk,
        sd.cmd,
        sd.d0,
        sd.d1,
        sd.d2,
        sd.d3,
        sdmmc_config,
    );
    let mut cmd_block = CmdBlock::new();

    info!("Turning SD power on");
    sd.power.set_low();
    Timer::after_secs(1).await;

    let mut card =
        match StorageDevice::new_sd_card(&mut sdmmc, &mut cmd_block, SD_TARGET_FREQ).await {
            Ok(card) => card,
            Err(e) => {
                error!("SD card init failed: {}", e);
                pending_forever().await;
            }
        };

    let total_blocks = card.card().csd.block_count();
    info!(
        "SD card initialized: blocks={} bytes={=u64}",
        total_blocks,
        total_blocks as u64 * 512
    );

    let volume = match detect_exfat_volume(&mut card, total_blocks).await {
        Ok(volume) => volume,
        Err(e) => {
            error!("exFAT volume detection failed: {}", e);
            pending_forever().await;
        }
    };

    info!(
        "Mounting exFAT volume: start_lba={} blocks={}",
        volume.start_lba, volume.block_count
    );

    let block_device = PartitionedSd::new(card, volume.start_lba, volume.block_count);
    let mut fs: FileSystem<_, BLOCK_BYTES, EXFAT_CACHE_BLOCKS> = FileSystem::new(block_device);

    if let Err(e) = fs.mount().await {
        error!("exFAT mount failed: {}", e);
        pending_forever().await;
    }

    info!(
        "Starting periodic exFAT writes: {} bytes every {} seconds",
        FILE_BYTES, WRITE_INTERVAL_SECS
    );
    let mut file_index = 1u32;
    loop {
        write_one_file(&mut fs, file_index).await;
        file_index = file_index.wrapping_add(1);
        Timer::after_secs(WRITE_INTERVAL_SECS).await;
    }
}

async fn write_one_file<D>(fs: &mut FileSystem<D, BLOCK_BYTES, EXFAT_CACHE_BLOCKS>, file_index: u32)
where
    D: BlockDevice<BLOCK_BYTES>,
    D::Error: defmt::Format,
{
    let mut file_name = [0u8; 15];
    make_file_name(file_index, &mut file_name);
    let file_name = match core::str::from_utf8(&file_name) {
        Ok(name) => name,
        Err(_) => {
            error!("Generated invalid UTF-8 filename for index={}", file_index);
            pending_forever().await;
        }
    };

    let fill_byte = b'0'.wrapping_add((file_index % 256) as u8);
    let data = unsafe { &mut *core::ptr::addr_of_mut!(FILE_DATA) };
    data.fill(fill_byte);

    info!(
        "Creating {} index={} fill_byte={=u8:#04x} bytes={}",
        file_name, file_index, fill_byte, FILE_BYTES
    );

    let options = OpenOptions::new().create_new(true).write(true);
    let mut file = match fs.open(file_name, options).await {
        Ok(file) => file,
        Err(e) => {
            error!("open {} failed: {}", file_name, e);
            pending_forever().await;
        }
    };

    if let Err(e) = file.write(fs, data).await {
        error!("write {} failed: {}", file_name, e);
        pending_forever().await;
    }

    if let Err(e) = file.close(fs).await {
        error!("close {} failed: {}", file_name, e);
        pending_forever().await;
    }

    info!("Closed {}", file_name);
}

fn make_file_name(index: u32, out: &mut [u8; 15]) {
    out.copy_from_slice(b"file_000000.txt");
    let mut value = index % 1_000_000;
    for pos in (5..11).rev() {
        out[pos] = b'0' + (value % 10) as u8;
        value /= 10;
    }
}

#[derive(Clone, Copy, defmt::Format)]
struct ExfatVolume {
    start_lba: u32,
    block_count: u32,
}

#[derive(Debug, defmt::Format)]
enum VolumeDetectError {
    Sd(SdError),
    NoExfatVolume,
    CardTooLarge,
    InvalidExfatVbr(VbrValidationError),
}

async fn detect_exfat_volume(
    card: &mut StorageDevice<'_, '_, Card>,
    total_blocks: u64,
) -> Result<ExfatVolume, VolumeDetectError> {
    let mut block0 = DataBlock::new();
    card.read_block(0, &mut block0)
        .await
        .map_err(VolumeDetectError::Sd)?;
    let bytes = data_block_bytes(&block0);

    log_lba0_summary(bytes, total_blocks);
    if looks_like_exfat_vbr(bytes) {
        log_vbr_summary("LBA0", 0, bytes, total_blocks);
        match validate_exfat_vbr(bytes, total_blocks) {
            Ok(()) => {
                info!("Found raw exFAT boot sector at LBA 0");
                return Ok(ExfatVolume {
                    start_lba: 0,
                    block_count: u32::try_from(total_blocks)
                        .map_err(|_| VolumeDetectError::CardTooLarge)?,
                });
            }
            Err(e) => {
                error!("LBA0 is an exFAT VBR but it cannot be mounted: {}", e);
                return Err(VolumeDetectError::InvalidExfatVbr(e));
            }
        }
    }

    if bytes[510..512] != MBR_SIGNATURE {
        return Err(VolumeDetectError::NoExfatVolume);
    }

    info!("LBA 0 is not a valid exFAT VBR; scanning MBR partitions");
    for index in 0..4 {
        let offset = 446 + index * 16;
        let status = bytes[offset];
        let partition_type = bytes[offset + 4];
        let start_lba = read_u32_le(&bytes[offset + 8..offset + 12]);
        let block_count = read_u32_le(&bytes[offset + 12..offset + 16]);
        let partition_end = u64::from(start_lba) + u64::from(block_count);

        info!(
            "MBR partition {}: status={=u8:#04x} type={=u8:#04x} start_lba={} blocks={} end_lba={} card_blocks={}",
            index, status, partition_type, start_lba, block_count, partition_end, total_blocks
        );
        if partition_type == 0 || start_lba == 0 || block_count == 0 {
            info!("MBR partition {} ignored: empty entry", index);
            continue;
        }

        if partition_end > total_blocks {
            error!(
                "MBR partition {} out of range: end_lba={} card_blocks={} over_by={}",
                index,
                partition_end,
                total_blocks,
                partition_end - total_blocks
            );
            continue;
        }

        let mut boot = DataBlock::new();
        card.read_block(start_lba, &mut boot)
            .await
            .map_err(VolumeDetectError::Sd)?;
        let boot_bytes = data_block_bytes(&boot);
        if looks_like_exfat_vbr(boot_bytes) {
            log_vbr_summary("MBR", start_lba, boot_bytes, u64::from(block_count));
            match validate_exfat_vbr(boot_bytes, u64::from(block_count)) {
                Ok(()) => {
                    info!(
                        "Found exFAT MBR partition: index={} type={=u8:#04x} start_lba={} blocks={}",
                        index, partition_type, start_lba, block_count
                    );
                    return Ok(ExfatVolume {
                        start_lba,
                        block_count,
                    });
                }
                Err(e) => {
                    error!(
                        "MBR partition {} has exFAT VBR but it cannot be mounted: {}",
                        index, e
                    );
                }
            }
        } else {
            log_vbr_summary("MBR", start_lba, boot_bytes, u64::from(block_count));
            info!("MBR partition {} is not an exFAT VBR", index);
        }
    }

    Err(VolumeDetectError::NoExfatVolume)
}

fn log_lba0_summary(bytes: &[u8; BLOCK_BYTES], total_blocks: u64) {
    info!(
        "LBA0 summary: jump={=u8:#04x} {=u8:#04x} {=u8:#04x} oem={=u8:#04x} {=u8:#04x} {=u8:#04x} {=u8:#04x} {=u8:#04x} {=u8:#04x} {=u8:#04x} {=u8:#04x} sig={=u8:#04x}{=u8:#04x} card_blocks={}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[511],
        bytes[510],
        total_blocks
    );
}

fn log_vbr_summary(origin: &str, lba: u32, bytes: &[u8; BLOCK_BYTES], max_blocks: u64) {
    let partition_offset = read_u64_le(&bytes[64..72]);
    let volume_length = read_u64_le(&bytes[72..80]);
    let fat_offset = read_u32_le(&bytes[80..84]);
    let fat_length = read_u32_le(&bytes[84..88]);
    let cluster_heap_offset = read_u32_le(&bytes[88..92]);
    let cluster_count = read_u32_le(&bytes[92..96]);
    let first_cluster_of_root_dir = read_u32_le(&bytes[96..100]);
    let bytes_per_sector_shift = bytes[108];
    let sectors_per_cluster_shift = bytes[109];
    let number_of_fats = bytes[110];
    let heap_end = if sectors_per_cluster_shift <= 31 {
        u64::from(cluster_heap_offset)
            + u64::from(cluster_count) * (1u64 << sectors_per_cluster_shift)
    } else {
        u64::MAX
    };
    let fat_end = u64::from(fat_offset) + u64::from(fat_length) * u64::from(number_of_fats);

    info!(
        "VBR candidate {} @ LBA {}: jump={=u8:#04x} {=u8:#04x} {=u8:#04x} oem={=u8:#04x} {=u8:#04x} {=u8:#04x} {=u8:#04x} {=u8:#04x} {=u8:#04x} {=u8:#04x} {=u8:#04x} sig={=u8:#04x}{=u8:#04x}",
        origin,
        lba,
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[511],
        bytes[510]
    );
    info!(
        "VBR fields {}: partition_offset={} volume_length={} max_blocks={} fat_offset={} fat_length={} fat_end={} cluster_heap_offset={} cluster_count={} heap_end={} root_cluster={} sector_shift={} cluster_shift={} fats={}",
        origin,
        partition_offset,
        volume_length,
        max_blocks,
        fat_offset,
        fat_length,
        fat_end,
        cluster_heap_offset,
        cluster_count,
        heap_end,
        first_cluster_of_root_dir,
        bytes_per_sector_shift,
        sectors_per_cluster_shift,
        number_of_fats
    );
}

fn looks_like_exfat_vbr(bytes: &[u8; BLOCK_BYTES]) -> bool {
    bytes[0..3] == EXFAT_JUMP_BOOT
        && &bytes[3..11] == EXFAT_OEM_NAME
        && bytes[510..512] == MBR_SIGNATURE
}

#[derive(Clone, Copy, Debug, defmt::Format)]
enum VbrValidationError {
    BadSignature,
    UnsupportedSectorShift(u8),
    UnsupportedClusterShift(u8),
    ZeroFatCount,
    InvalidVolumeLength,
    ZeroLayoutField,
    InvalidRootCluster,
    PartitionOffsetOutOfRange,
    FatOutOfRange,
    HeapOutOfRange,
}

fn validate_exfat_vbr(
    bytes: &[u8; BLOCK_BYTES],
    max_blocks: u64,
) -> Result<(), VbrValidationError> {
    if !looks_like_exfat_vbr(bytes) {
        return Err(VbrValidationError::BadSignature);
    }

    let partition_offset = read_u64_le(&bytes[64..72]);
    let volume_length = read_u64_le(&bytes[72..80]);
    let fat_offset = read_u32_le(&bytes[80..84]);
    let fat_length = read_u32_le(&bytes[84..88]);
    let cluster_heap_offset = read_u32_le(&bytes[88..92]);
    let cluster_count = read_u32_le(&bytes[92..96]);
    let first_cluster_of_root_dir = read_u32_le(&bytes[96..100]);
    let bytes_per_sector_shift = bytes[108];
    let sectors_per_cluster_shift = bytes[109];
    let number_of_fats = bytes[110];

    if bytes_per_sector_shift != 9 {
        return Err(VbrValidationError::UnsupportedSectorShift(
            bytes_per_sector_shift,
        ));
    }

    if sectors_per_cluster_shift > 16 {
        return Err(VbrValidationError::UnsupportedClusterShift(
            sectors_per_cluster_shift,
        ));
    }

    if number_of_fats == 0 {
        return Err(VbrValidationError::ZeroFatCount);
    }

    if volume_length == 0 || volume_length > max_blocks {
        return Err(VbrValidationError::InvalidVolumeLength);
    }

    if fat_offset == 0 || fat_length == 0 || cluster_heap_offset == 0 || cluster_count == 0 {
        return Err(VbrValidationError::ZeroLayoutField);
    }

    if first_cluster_of_root_dir < 2 || first_cluster_of_root_dir >= cluster_count + 2 {
        return Err(VbrValidationError::InvalidRootCluster);
    }

    let sectors_per_cluster = 1u64 << sectors_per_cluster_shift;
    let heap_sectors = u64::from(cluster_count) * sectors_per_cluster;
    let heap_end = u64::from(cluster_heap_offset) + heap_sectors;
    let fat_end = u64::from(fat_offset) + u64::from(fat_length) * u64::from(number_of_fats);

    if partition_offset >= max_blocks {
        return Err(VbrValidationError::PartitionOffsetOutOfRange);
    }

    if fat_end > volume_length {
        return Err(VbrValidationError::FatOutOfRange);
    }

    if heap_end > volume_length {
        return Err(VbrValidationError::HeapOutOfRange);
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

fn data_block_bytes(block: &DataBlock) -> &[u8; BLOCK_BYTES] {
    unsafe { &*(block.0.as_ptr().cast::<u8>() as *const [u8; BLOCK_BYTES]) }
}

struct PartitionedSd<'a, 'b> {
    card: StorageDevice<'a, 'b, Card>,
    start_lba: u32,
    block_count: u32,
}

impl<'a, 'b> PartitionedSd<'a, 'b> {
    fn new(card: StorageDevice<'a, 'b, Card>, start_lba: u32, block_count: u32) -> Self {
        Self {
            card,
            start_lba,
            block_count,
        }
    }

    fn translate(&self, block_address: u32, blocks: usize) -> Result<u32, ExfatSdError> {
        let blocks = u32::try_from(blocks).map_err(|_| ExfatSdError::OutOfRange)?;
        if block_address
            .checked_add(blocks)
            .map_or(true, |end| end > self.block_count)
        {
            return Err(ExfatSdError::OutOfRange);
        }

        self.start_lba
            .checked_add(block_address)
            .ok_or(ExfatSdError::OutOfRange)
    }
}

#[derive(Debug, defmt::Format)]
enum ExfatSdError {
    Sd(SdError),
    OutOfRange,
}

impl BlockDevice<BLOCK_BYTES> for PartitionedSd<'_, '_> {
    type Error = ExfatSdError;
    type Align = A4;

    async fn read(
        &mut self,
        block_address: u32,
        data: &mut [Aligned<Self::Align, [u8; BLOCK_BYTES]>],
    ) -> Result<(), Self::Error> {
        let physical_lba = self.translate(block_address, data.len())?;
        let blocks = unsafe {
            core::slice::from_raw_parts_mut(data.as_mut_ptr().cast::<DataBlock>(), data.len())
        };
        self.card
            .read_blocks(physical_lba, blocks)
            .await
            .map_err(ExfatSdError::Sd)
    }

    async fn write(
        &mut self,
        block_address: u32,
        data: &[Aligned<Self::Align, [u8; BLOCK_BYTES]>],
    ) -> Result<(), Self::Error> {
        let physical_lba = self.translate(block_address, data.len())?;
        let blocks =
            unsafe { core::slice::from_raw_parts(data.as_ptr().cast::<DataBlock>(), data.len()) };
        self.card
            .write_blocks(physical_lba, blocks)
            .await
            .map_err(ExfatSdError::Sd)
    }

    async fn size(&mut self) -> Result<u64, Self::Error> {
        Ok(self.block_count as u64 * BLOCK_BYTES as u64)
    }
}

fn card_absent(sd_switch: &Input<'_>) -> bool {
    sd_switch.is_high()
}

async fn pending_forever() -> ! {
    core::future::pending::<()>().await;
    unreachable!()
}
