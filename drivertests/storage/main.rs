// Storage driver proof-of-concept.
//
// This test mounts the microSD exFAT volume, opens four write handles at once,
// writes human-readable text files, closes two files at non-block boundaries,
// and reads one file back through the driver's single read handle.

#![no_std]
#![no_main]

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_stm32::rcc::mux::Sdmmcsel;
use embassy_stm32::rcc::*;
use embassy_stm32::sdmmc::sd::{CmdBlock, StorageDevice};
use embassy_stm32::sdmmc::{Config as SdmmcConfig, Sdmmc};
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Irqs, SdCard};
use raylar_drivers::storage::stm32::Stm32SdBlockDevice;
use raylar_drivers::storage::{
    detect_exfat_volume, PartitionedBlockDevice, StorageBlockDevice, StorageDriver, StorageError,
    BLOCK_BYTES,
};
use {defmt_rtt as _, panic_probe as _};

const SD_TARGET_FREQ: Hertz = mhz(24);
const HEAP_BYTES: usize = 64 * 1024;
const BULK_BYTES: usize = 8 * BLOCK_BYTES;
const LOG_FINAL: &[u8] = b"log: final partial line is valid; bytes after this are ignored\n";
const BULK_FINAL: &[u8] = b"bulk-c: partial final block\n";

#[global_allocator]
static HEAP: Heap = Heap::empty();

static mut LOG_BLOCK: [u8; BLOCK_BYTES] = [0; BLOCK_BYTES];
static mut LOG_TAIL: [u8; BLOCK_BYTES] = [0; BLOCK_BYTES];
static mut BULK_A: [u8; BULK_BYTES] = [0; BULK_BYTES];
static mut BULK_B: [u8; BULK_BYTES] = [0; BULK_BYTES];
static mut BULK_C: [u8; BULK_BYTES] = [0; BULK_BYTES];
static mut BULK_TAIL: [u8; BLOCK_BYTES] = [0; BLOCK_BYTES];
static mut READ_BUF: [u8; 256] = [0; 256];

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
        divp: Some(PllDiv::DIV6),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2),
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.hsi48 = Some(Hsi48Config::new());
    config.rcc.mux.sdmmcsel = Sdmmcsel::PLL1_P;

    let p = embassy_stm32::init(config);
    let Board { sd, .. } = Board::new(p);

    info!("Storage driver test started");
    run_storage_test(sd).await
}

async fn run_storage_test(mut sd: SdCard<'static>) -> ! {
    sd.power.set_high();

    if sd.switch.is_high() {
        let err: StorageError<()> = StorageError::CardNotPresent;
        error!("storage error path exercised: {}", err);
        pending_forever().await;
    }

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

    info!("Powering SD card");
    sd.power.set_low();
    Timer::after_secs(1).await;

    let card = match StorageDevice::new_sd_card(&mut sdmmc, &mut cmd_block, SD_TARGET_FREQ).await {
        Ok(card) => card,
        Err(e) => {
            error!("SD card init failed: {}", e);
            pending_forever().await;
        }
    };

    let mut block_device = Stm32SdBlockDevice::new(card);
    let volume = match detect_exfat_volume(&mut block_device).await {
        Ok(volume) => volume,
        Err(e) => {
            let err = StorageError::<()>::NoFilesystem;
            error!("storage error path exercised: {}; detect={}", err, e);
            pending_forever().await;
        }
    };

    info!(
        "Detected exFAT volume start_lba={} blocks={}",
        volume.start_lba, volume.block_count
    );

    let partition = PartitionedBlockDevice::new(block_device, volume);
    let mut storage = StorageDriver::new(partition);

    if let Err(e) = storage.mount().await {
        error!("mount failed: {}", e);
        pending_forever().await;
    }
    info!("mount ok");

    if let Err(e) = exercise_storage(&mut storage).await {
        error!("storage test failed: {}", e);
        pending_forever().await;
    }

    info!("storage test complete");
    pending_forever().await
}

async fn exercise_storage<D>(storage: &mut StorageDriver<D>) -> Result<(), StorageError<D::Error>>
where
    D: StorageBlockDevice<BLOCK_BYTES>,
{
    prepare_buffers();
    let log = storage.open_for_append("/st_log.txt").await?;
    let bulk_a = storage.open_for_append("/st_bulka.txt").await?;
    let bulk_b = storage.open_for_append("/st_bulkb.txt").await?;
    let bulk_c = storage.open_for_append("/st_bulkc.txt").await?;

    let log_block = unsafe { &*core::ptr::addr_of!(LOG_BLOCK) };
    let log_tail = unsafe { &*core::ptr::addr_of!(LOG_TAIL) };
    let bulk_a_data = unsafe { &*core::ptr::addr_of!(BULK_A) };
    let bulk_b_data = unsafe { &*core::ptr::addr_of!(BULK_B) };
    let bulk_c_data = unsafe { &*core::ptr::addr_of!(BULK_C) };
    let bulk_tail = unsafe { &*core::ptr::addr_of!(BULK_TAIL) };

    storage.append(log, log_block).await?;
    storage.append(bulk_a, bulk_a_data).await?;
    storage.append(bulk_b, bulk_b_data).await?;
    storage.append(bulk_c, bulk_c_data).await?;

    storage.flush(log).await?;
    storage.append(log, log_tail).await?;
    storage.append(bulk_c, bulk_tail).await?;

    storage.close(log, LOG_FINAL.len()).await?;
    storage.close(bulk_a, BLOCK_BYTES).await?;
    storage.close(bulk_b, BLOCK_BYTES).await?;
    storage.close(bulk_c, BULK_FINAL.len()).await?;

    let read = storage.open_for_read("/st_log.txt").await?;
    let read_buf = unsafe { &mut *core::ptr::addr_of_mut!(READ_BUF) };
    let mut total = 0usize;
    loop {
        let n = storage.read(read, read_buf).await?;
        if n == 0 {
            break;
        }
        total += n;
    }
    storage.close_read(read)?;

    let expected = BLOCK_BYTES + LOG_FINAL.len();
    if total != expected {
        error!(
            "readback length mismatch expected={} actual={}",
            expected, total
        );
        return Err(StorageError::InvalidState);
    }

    info!("readback ok bytes={}", total);
    Ok(())
}

fn prepare_buffers() {
    let log_block = unsafe { &mut *core::ptr::addr_of_mut!(LOG_BLOCK) };
    let log_tail = unsafe { &mut *core::ptr::addr_of_mut!(LOG_TAIL) };
    let bulk_a = unsafe { &mut *core::ptr::addr_of_mut!(BULK_A) };
    let bulk_b = unsafe { &mut *core::ptr::addr_of_mut!(BULK_B) };
    let bulk_c = unsafe { &mut *core::ptr::addr_of_mut!(BULK_C) };
    let bulk_tail = unsafe { &mut *core::ptr::addr_of_mut!(BULK_TAIL) };

    fill_repeating(
        log_block,
        b"log: mounted storage driver and opened four handles\n",
    );
    fill_prefix(log_tail, LOG_FINAL);
    fill_repeating(bulk_a, b"bulk-a: sequential append payload\n");
    fill_repeating(bulk_b, b"bulk-b: sequential append payload\n");
    fill_repeating(bulk_c, b"bulk-c: sequential append payload\n");
    fill_prefix(bulk_tail, BULK_FINAL);
}

fn fill_prefix(out: &mut [u8], prefix: &[u8]) {
    out.fill(b' ');
    out[..prefix.len()].copy_from_slice(prefix);
}

fn fill_repeating(out: &mut [u8], line: &[u8]) {
    let mut offset = 0;
    while offset < out.len() {
        let len = line.len().min(out.len() - offset);
        out[offset..offset + len].copy_from_slice(&line[..len]);
        offset += len;
    }
}

async fn pending_forever() -> ! {
    core::future::pending::<()>().await;
    unreachable!()
}
