// Read-only, bounded SD-card diagnostics intended for automated GPT debugging.

#![no_std]
#![no_main]

extern crate alloc;

use alloc::{format, string::String, vec::Vec};
use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_stm32::rcc::mux::Sdmmcsel;
use embassy_stm32::rcc::*;
use embassy_stm32::sdmmc::sd::{CmdBlock, StorageDevice};
use embassy_stm32::sdmmc::{Config as SdmmcConfig, Sdmmc};
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use exfat_slim::asynchronous::file::OpenOptions;
use exfat_slim::asynchronous::file_system::FileSystem;
use exfat_slim::asynchronous::BlockDevice;
use raylar_board_v1p0::{Board, Irqs, SdCard};
use raylar_drivers::storage::stm32::Stm32SdBlockDevice;
use raylar_drivers::storage::{detect_exfat_volume, PartitionedBlockDevice, BLOCK_BYTES};
use {defmt_rtt as _, panic_probe as _};

const SD_TARGET_FREQ: Hertz = mhz(24);
const HEAP_BYTES: usize = 96 * 1024;
const CACHE_BLOCKS: usize = 8;
const MAX_ENTRIES: usize = 128;
const MAX_DIRECTORIES: usize = 32;
const MAX_DEPTH: u8 = 4;
// The card commonly contains more than 16 rotated recordings. Cover the full
// bounded directory listing so newly created files are not hidden behind old
// entries during automated validation.
const MAX_SNIPPET_FILES: usize = 64;
const MAX_SNIPPET_BYTES: usize = 256;
const SNIPPET_CHUNK_BYTES: usize = 32;

#[global_allocator]
static HEAP: Heap = Heap::empty();

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
    inspect_card(sd).await
}

async fn inspect_card(mut sd: SdCard<'static>) -> ! {
    info!(
        "SDGPT|BEGIN|version=1|max_entries={}|max_directories={}|max_depth={}|max_snippet_files={}|max_snippet_bytes={}|#",
        MAX_ENTRIES,
        MAX_DIRECTORIES,
        MAX_DEPTH,
        MAX_SNIPPET_FILES,
        MAX_SNIPPET_BYTES
    );

    // The Raylar SD power switch is active-low. Keep power off while checking
    // card presence, then allow the card a full second to settle.
    sd.power.set_high();
    if sd.switch.is_high() {
        finish_with_error("card_absent").await;
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

    sd.power.set_low();
    Timer::after_secs(1).await;

    let card = match StorageDevice::new_sd_card(&mut sdmmc, &mut cmd_block, SD_TARGET_FREQ).await {
        Ok(card) => card,
        Err(e) => {
            error!("SDGPT|ERROR|stage=card_init|detail={}|#", e);
            finish_with_error("card_init").await;
        }
    };

    let card_blocks = card.card().csd.block_count();
    info!(
        "SDGPT|CARD|blocks={}|bytes={=u64}|#",
        card_blocks,
        card_blocks as u64 * BLOCK_BYTES as u64
    );

    let mut raw_device = Stm32SdBlockDevice::new(card);
    let volume = match detect_exfat_volume(&mut raw_device).await {
        Ok(volume) => volume,
        Err(e) => {
            error!("SDGPT|ERROR|stage=detect_exfat|detail={}|#", e);
            finish_with_error("detect_exfat").await;
        }
    };
    info!(
        "SDGPT|VOLUME|start_lba={}|blocks={}|#",
        volume.start_lba, volume.block_count
    );

    let partition = PartitionedBlockDevice::new(raw_device, volume);
    let read_only = ReadOnlyDevice(partition);
    let mut fs: FileSystem<_, BLOCK_BYTES, CACHE_BLOCKS> = FileSystem::new(read_only);
    if let Err(e) = fs.mount().await {
        error!("SDGPT|ERROR|stage=mount|detail={}|#", e);
        finish_with_error("mount").await;
    }

    let stats = inspect_filesystem(&mut fs).await;
    info!(
        "SDGPT|END|status=ok|entries={}|directories={}|files_snippeted={}|snippet_bytes={}|errors={}|truncated={}|#",
        stats.entries,
        stats.directories,
        stats.files_snippeted,
        stats.snippet_bytes,
        stats.errors,
        stats.truncated
    );
    defmt::flush();
    pending_forever().await
}

#[derive(Default)]
struct InspectionStats {
    entries: usize,
    directories: usize,
    files_snippeted: usize,
    snippet_bytes: usize,
    errors: usize,
    truncated: bool,
}

async fn inspect_filesystem<D>(
    fs: &mut FileSystem<ReadOnlyDevice<D>, BLOCK_BYTES, CACHE_BLOCKS>,
) -> InspectionStats
where
    D: BlockDevice<BLOCK_BYTES>,
    D::Error: defmt::Format,
{
    let mut stats = InspectionStats::default();
    let mut directories: Vec<(String, u8)> = Vec::new();
    directories.push((String::from("/"), 0));
    let mut directory_index = 0usize;

    'directories: while directory_index < directories.len() {
        let (directory_path, depth) = directories[directory_index].clone();
        directory_index += 1;
        stats.directories += 1;

        let mut directory = match fs.read_dir(directory_path.as_str()).await {
            Ok(directory) => directory,
            Err(e) => {
                error!(
                    "SDGPT|ERROR|stage=read_dir|path={}|detail={}|#",
                    directory_path.as_str(),
                    e
                );
                stats.errors += 1;
                continue;
            }
        };

        loop {
            if stats.entries >= MAX_ENTRIES {
                info!("SDGPT|TRUNCATED|reason=max_entries|limit={}|#", MAX_ENTRIES);
                stats.truncated = true;
                break 'directories;
            }

            let entry = match directory.next_entry(fs).await {
                Ok(Some(entry)) => entry,
                Ok(None) => break,
                Err(e) => {
                    error!(
                        "SDGPT|ERROR|stage=next_entry|path={}|detail={}|#",
                        directory_path.as_str(),
                        e
                    );
                    stats.errors += 1;
                    break;
                }
            };

            let name = entry.file_name();
            let path = if directory_path == "/" {
                format!("/{}", name)
            } else {
                format!("{}/{}", directory_path, name)
            };
            let metadata = entry.metadata();
            stats.entries += 1;

            if metadata.is_dir() {
                info!(
                    "SDGPT|ENTRY|kind=dir|size={=u64}|depth={}|path={}|#",
                    metadata.len(),
                    depth,
                    path.as_str()
                );
                if depth < MAX_DEPTH {
                    if directories.len() < MAX_DIRECTORIES {
                        directories.push((path, depth + 1));
                    } else if !stats.truncated {
                        info!(
                            "SDGPT|TRUNCATED|reason=max_directories|limit={}|#",
                            MAX_DIRECTORIES
                        );
                        stats.truncated = true;
                    }
                } else {
                    info!(
                        "SDGPT|TRUNCATED|reason=max_depth|limit={}|path={}|#",
                        MAX_DEPTH,
                        path.as_str()
                    );
                    stats.truncated = true;
                }
                continue;
            }

            info!(
                "SDGPT|ENTRY|kind=file|size={=u64}|depth={}|path={}|#",
                metadata.len(),
                depth,
                path.as_str()
            );
            if stats.files_snippeted >= MAX_SNIPPET_FILES || metadata.is_empty() {
                continue;
            }

            match emit_file_snippet(fs, path.as_str()).await {
                Ok(bytes) => {
                    stats.files_snippeted += 1;
                    stats.snippet_bytes += bytes;
                }
                Err(e) => {
                    error!(
                        "SDGPT|ERROR|stage=read_file|path={}|detail={}|#",
                        path.as_str(),
                        e
                    );
                    stats.errors += 1;
                }
            }
        }
    }

    if stats.files_snippeted >= MAX_SNIPPET_FILES && stats.entries > stats.files_snippeted {
        info!(
            "SDGPT|TRUNCATED|reason=max_snippet_files|limit={}|#",
            MAX_SNIPPET_FILES
        );
        stats.truncated = true;
    }

    stats
}

async fn emit_file_snippet<D>(
    fs: &mut FileSystem<ReadOnlyDevice<D>, BLOCK_BYTES, CACHE_BLOCKS>,
    path: &str,
) -> Result<usize, exfat_slim::asynchronous::error::ExFatError<ReadOnlyError<D::Error>>>
where
    D: BlockDevice<BLOCK_BYTES>,
    D::Error: defmt::Format,
{
    let options = OpenOptions::new().read(true);
    let mut file = fs.open(path, options).await?;
    let mut total = 0usize;
    let mut buffer = [0u8; SNIPPET_CHUNK_BYTES];

    while total < MAX_SNIPPET_BYTES {
        let remaining = MAX_SNIPPET_BYTES - total;
        let requested = remaining.min(buffer.len());
        let read = match file.read(fs, &mut buffer[..requested]).await? {
            Some(0) | None => break,
            Some(read) => read,
        };
        info!(
            "SDGPT|DATA|offset={}|path={}|bytes={=[u8]}|#",
            total,
            path,
            &buffer[..read]
        );
        total += read;
    }

    Ok(total)
}

struct ReadOnlyDevice<D>(D);

#[derive(Debug, defmt::Format)]
enum ReadOnlyError<E: defmt::Format> {
    Inner(E),
    WriteAttempt,
}

impl<D, const SIZE: usize> BlockDevice<SIZE> for ReadOnlyDevice<D>
where
    D: BlockDevice<SIZE>,
    D::Error: defmt::Format,
{
    type Error = ReadOnlyError<D::Error>;
    type Align = D::Align;

    async fn read(
        &mut self,
        block_address: u32,
        data: &mut [aligned::Aligned<Self::Align, [u8; SIZE]>],
    ) -> Result<(), Self::Error> {
        self.0
            .read(block_address, data)
            .await
            .map_err(ReadOnlyError::Inner)
    }

    async fn write(
        &mut self,
        _block_address: u32,
        _data: &[aligned::Aligned<Self::Align, [u8; SIZE]>],
    ) -> Result<(), Self::Error> {
        Err(ReadOnlyError::WriteAttempt)
    }

    async fn size(&mut self) -> Result<u64, Self::Error> {
        self.0.size().await.map_err(ReadOnlyError::Inner)
    }
}

async fn finish_with_error(stage: &str) -> ! {
    error!("SDGPT|FATAL|stage={}|#", stage);
    info!("SDGPT|END|status=error|#");
    defmt::flush();
    pending_forever().await
}

async fn pending_forever() -> ! {
    core::future::pending::<()>().await;
    unreachable!()
}
