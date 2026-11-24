//! ADR-L4-007: DMA-backed exFAT inspection, exclusive file I/O and persistence.
#![no_std]
#![no_main]

mod sd;
mod stack_probe;

use core::fmt::Write as _;
use core::sync::atomic::Ordering::Relaxed;
use defmt::{assert, assert_eq, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Level, Output, Pull, Speed};
use embassy_stm32::rcc::{self, *};
use embassy_stm32::sdmmc::sd::{CmdBlock, StorageDevice};
use embassy_stm32::sdmmc::{Config as SdConfig, Sdmmc};
use embassy_stm32::{pac, time::Hertz};
use embassy_time::{with_timeout, Duration, Instant, Timer};
use embedded_alloc::LlffHeap;
#[cfg(feature = "write-test")]
use exfat_slim::asynchronous::error::ExFatError;
use exfat_slim::asynchronous::file::OpenOptions;
use exfat_slim::asynchronous::file_system::FileSystem;
use heapless::String;
use raylar_drivers::storage::{detect_exfat_volume, PartitionedBlockDevice, StorageBlockDevice};
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static HEAP: LlffHeap = LlffHeap::empty();
static IO_CHUNK: static_cell::ConstStaticCell<aligned::Aligned<aligned::A4, [u8; CHUNK]>> =
    static_cell::ConstStaticCell::new(aligned::Aligned([0; CHUNK]));
const CACHE: usize = 1;
const CHUNK: usize = 4096;
const MAX_TEST_BYTES: u64 = 2 * 1024 * 1024;
const MAGIC: &[u8; 16] = b"CARACAL_L4_EXFAT";

// Keep different phase poll frames out of the caller's common stack frame.
struct Phase<F>(F);

impl<F: core::future::Future> core::future::Future for Phase<F> {
    type Output = F::Output;

    #[inline(never)]
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        // This private wrapper never moves or replaces its pinned inner future.
        unsafe { self.map_unchecked_mut(|phase| &mut phase.0) }.poll(cx)
    }
}

macro_rules! io {
    ($phase:literal, $future:expr) => {
        match with_timeout(Duration::from_secs(15), Phase($future)).await {
            Ok(Ok(value)) => value,
            Ok(Err(error)) => defmt::panic!("L4 EXFAT FAIL phase={} error={}", $phase, error),
            Err(_) => defmt::panic!("L4 EXFAT FAIL phase={} software_timeout", $phase),
        }
    };
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    stack_probe::initialize();
    let chunk: &mut [u8; CHUNK] = IO_CHUNK.take();
    // Only directory-name inspection uses allocating String convenience methods.
    // File I/O uses the upstream exFAT engine with one file and fixed chunks.
    unsafe {
        embedded_alloc::init!(HEAP, 2048);
    }
    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz(16_000_000),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV2,
        mul: PllMul::MUL20,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2),
    });
    config.rcc.pllsai1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV2,
        mul: PllMul::MUL12,
        divp: None,
        divq: Some(PllQDiv::DIV2),
        divr: None,
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV1;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.mux.clk48sel = rcc::mux::Clk48sel::PLLSAI1_Q;
    config.rcc.ls = LsConfig::off();
    let p = embassy_stm32::init(config);
    let _red = Output::new(p.PB11, Level::High, Speed::Low);
    let _green = Output::new(p.PC3, Level::High, Speed::Low);
    let mut blue = Output::new(p.PC1, Level::High, Speed::Low);
    let detect = Input::new(p.PC6, Pull::Up);
    Timer::after_millis(100).await;
    info!(
        "L4 EXFAT boot write_test={} sd_sw_high={}",
        cfg!(feature = "write-test"),
        detect.is_high()
    );
    assert!(
        detect.is_low(),
        "Card detect reports no SD card; no writes attempted"
    );
    let clocks = rcc::clocks(&p.RCC);
    assert_eq!(clocks.sys.to_hertz().unwrap().0, 80_000_000);
    assert_eq!(clocks.pllsai1_q.to_hertz().unwrap().0, 48_000_000);
    assert!(pac::RCC.cr().read().hserdy() && pac::RCC.cr().read().pllsai1rdy());
    info!("L4 SD clocks sys_hz=80000000 kernel_hz=48000000 dma=DMA2_CH5 request=7");

    let mut sd_config = SdConfig::default();
    sd_config.data_transfer_timeout = 24_000_000;
    let mut sdmmc = Sdmmc::new_4bit(
        p.SDMMC1,
        p.DMA2_CH5,
        sd::Irqs,
        p.PC12,
        p.PD2,
        p.PC8,
        p.PC9,
        p.PC10,
        p.PC11,
        sd_config,
    );
    let mut command = CmdBlock::new();
    let card = io!(
        "card_init",
        StorageDevice::new_sd_card(&mut sdmmc, &mut command, Hertz(24_000_000))
    );
    let cid = card.card().cid;
    info!(
        "L4 SD card sectors={} serial={:08x} manufacturer={} clkcr={:08x}",
        card.card().csd.block_count(),
        cid.serial(),
        cid.manufacturer_id(),
        pac::SDMMC1.clkcr().read().0
    );
    assert_eq!(pac::SDMMC1.clkcr().read().widbus(), 1);
    let mut raw = sd::L4Sd::new(card);
    let volume = io!("detect_volume", detect_exfat_volume(&mut raw));
    info!(
        "L4 exFAT volume lba={} sectors={}",
        volume.start_lba, volume.block_count
    );
    let mut fs: FileSystem<_, 512, CACHE> =
        FileSystem::new(PartitionedBlockDevice::new(raw, volume));
    let (before, _previous, cluster_bytes) =
        unwrap!(with_timeout(Duration::from_secs(60), Phase(inspect(&mut fs, None))).await);
    stack_probe::check("inspected");
    assert_eq!(sd::WRITES.load(Relaxed), 0);
    info!(
        "L4 EXFAT READ PASS root_entries={} fingerprint={:016x} cluster_bytes={}",
        before.count, before.hash, cluster_bytes
    );

    #[cfg(feature = "write-test")]
    {
        let length = (256 * 1024 + 173).max(u64::from(cluster_bytes) + CHUNK as u64 + 173);
        assert!(
            length <= MAX_TEST_BYTES,
            "Cluster size exceeds this smoke profile"
        );
        sd::WRITE_ALLOWED.store(true, Relaxed);
        let path = Phase(write_file(&mut fs, chunk, length)).await;
        stack_probe::check("written");
        sd::WRITE_ALLOWED.store(false, Relaxed);
        fs = FileSystem::new(fs.unmount());
        Phase(verify_file(&mut fs, chunk, path.as_str())).await;
        let (after, _, _) = unwrap!(
            with_timeout(
                Duration::from_secs(60),
                Phase(inspect(&mut fs, Some(&path[1..])))
            )
            .await
        );
        assert_eq!(before.count, after.count);
        assert_eq!(before.hash, after.hash);
        assert!(sd::WRITES.load(Relaxed) > 0 && sd::MAX_WRITE_BLOCKS.load(Relaxed) > 1);
        info!(
            "L4 EXFAT WRITE PASS path={} bytes={} existing_root_unchanged=true",
            path.as_str(),
            length
        );
    }
    #[cfg(not(feature = "write-test"))]
    if let Some(path) = _previous {
        Phase(verify_file(&mut fs, chunk, path.as_str())).await;
        assert_eq!(sd::WRITES.load(Relaxed), 0);
        info!("L4 EXFAT PERSISTENCE PASS path={} writes=0", path.as_str());
    }
    assert!(sd::DMA_IRQS.load(Relaxed) > 0);
    stack_probe::check("complete");
    info!("L4 SD stats reads={} read_sectors={} max_read={} writes={} write_sectors={} max_write={} dma_irqs={} sd_irqs={}",
        sd::READS.load(Relaxed), sd::READ_BLOCKS.load(Relaxed), sd::MAX_READ_BLOCKS.load(Relaxed),
        sd::WRITES.load(Relaxed), sd::WRITE_BLOCKS.load(Relaxed), sd::MAX_WRITE_BLOCKS.load(Relaxed),
        sd::DMA_IRQS.load(Relaxed), sd::SD_IRQS.load(Relaxed));
    info!("L4 EXFAT PASS write_test={}", cfg!(feature = "write-test"));
    loop {
        blue.set_low();
        Timer::after_millis(500).await;
        blue.set_high();
        Timer::after_millis(500).await;
    }
}

#[derive(Clone, Copy, Default)]
struct Snapshot {
    count: u32,
    hash: u64,
}

async fn inspect<D>(
    fs: &mut FileSystem<D, 512, CACHE>,
    exclude: Option<&str>,
) -> (Snapshot, Option<String<32>>, u32)
where
    D: StorageBlockDevice<512>,
    D::Error: defmt::Format,
{
    io!("mount_inspect", fs.mount());
    let (_, free, cluster_bytes) = io!("space_info", fs.space_info());
    info!(
        "L4 exFAT free_bytes={} cluster_bytes={}",
        free, cluster_bytes
    );
    let mut directory = io!("root_open", fs.read_dir(""));
    let mut snapshot = Snapshot::default();
    let mut previous: Option<String<32>> = None;
    let mut scanned = 0;
    while let Some(entry) = io!("root_next", directory.next_entry(fs)) {
        scanned += 1;
        assert!(
            scanned <= 131_072,
            "Root listing exceeds bounded smoke limit; no further writes"
        );
        if scanned % 4096 == 0 {
            info!("L4 root scan entries={}", scanned);
        }
        let name = entry.file_name();
        let metadata = entry.metadata();
        if exclude == Some(name.as_str()) {
            continue;
        }
        let mut hash = 0xcbf29ce484222325u64;
        for b in name
            .as_bytes()
            .iter()
            .copied()
            .chain(metadata.len().to_le_bytes())
            .chain([u8::from(metadata.is_dir())])
        {
            hash = (hash ^ u64::from(b)).wrapping_mul(0x100000001b3);
        }
        snapshot.count += 1;
        snapshot.hash ^= hash;
        // The dedicated test namespace is read back only; never reopened for writing.
        if metadata.is_file() && test_name(&name) {
            let mut path = String::new();
            unwrap!(write!(path, "/{}", name.as_str()));
            if previous
                .as_ref()
                .map_or(true, |old| path.as_str() > old.as_str())
            {
                previous = Some(path);
            }
        }
    }
    (snapshot, previous, cluster_bytes)
}

fn test_name(name: &str) -> bool {
    name.len() == 12
        && name.starts_with("L4E")
        && name.ends_with(".BIN")
        && name.as_bytes()[3..8].iter().all(u8::is_ascii_digit)
}

fn pattern(offset: u64, length: u64) -> u8 {
    if offset < 16 {
        return MAGIC[offset as usize];
    }
    if offset < 24 {
        return length.to_le_bytes()[offset as usize - 16];
    }
    let x = (offset as u32)
        .wrapping_mul(0x9e3779b1)
        .rotate_left(((offset >> 8) & 31) as u32)
        ^ (offset >> 13) as u32;
    (x ^ (x >> 8) ^ (x >> 16) ^ (x >> 24) ^ 0xa5) as u8
}

#[cfg(feature = "write-test")]
async fn write_file<D>(
    fs: &mut FileSystem<D, 512, CACHE>,
    chunk: &mut [u8; CHUNK],
    length: u64,
) -> String<32>
where
    D: StorageBlockDevice<512>,
    D::Error: defmt::Format,
{
    let (_, free, cluster) = io!("write_space", fs.space_info());
    assert!(
        free >= length + 2 * u64::from(cluster),
        "Insufficient free space; no file created"
    );
    let mut chosen = None;
    for sequence in 0..256 {
        let mut path = String::<32>::new();
        unwrap!(write!(path, "/L4E{:05}.BIN", sequence));
        if !io!("candidate_exists", fs.exists(&path)) {
            chosen = Some(path);
            break;
        }
    }
    let path = unwrap!(chosen);
    let mut file = io!(
        "create_exclusive",
        fs.open(&path, OpenOptions::new().create_new(true).write(true))
    );
    info!("L4 exFAT writing path={} bytes={}", path.as_str(), length);
    let start = Instant::now();
    let mut offset = 0;
    while offset < length {
        let count = (length - offset).min(CHUNK as u64) as usize;
        for (index, byte) in chunk[..count].iter_mut().enumerate() {
            *byte = pattern(offset + index as u64, length);
        }
        io!("append", file.write(fs, &chunk[..count]));
        offset += count as u64;
    }
    io!("flush", file.flush(fs));
    io!("close", file.close(fs));
    let collision = unwrap!(
        with_timeout(
            Duration::from_secs(15),
            Phase(fs.open(&path, OpenOptions::new().create_new(true).write(true)))
        )
        .await
    );
    assert!(matches!(collision, Err(ExFatError::AlreadyExists)));
    info!(
        "L4 exFAT flushed path={} bytes={} elapsed_ms={}",
        path.as_str(),
        length,
        (Instant::now() - start).as_millis()
    );
    path
}

async fn verify_file<D>(fs: &mut FileSystem<D, 512, CACHE>, chunk: &mut [u8; CHUNK], path: &str)
where
    D: StorageBlockDevice<512>,
    D::Error: defmt::Format,
{
    io!("mount_verify", fs.mount());
    let mut file = io!("open_verify", fs.open(path, OpenOptions::new().read(true)));
    assert_eq!(
        io!("header", file.read(fs, &mut chunk[..24])).unwrap_or(0),
        24
    );
    assert!(
        &chunk[..16] == MAGIC,
        "Test namespace contains a foreign file; preserved without writing"
    );
    let length = u64::from_le_bytes(chunk[16..24].try_into().unwrap());
    assert!((24..=MAX_TEST_BYTES).contains(&length));
    let start = Instant::now();
    let mut offset = 24;
    let mut hash = 0xcbf29ce484222325u64;
    for byte in &chunk[..24] {
        hash = (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3);
    }
    while offset < length {
        let count = io!("readback", file.read(fs, &mut chunk[..])).unwrap_or(0);
        assert!(count > 0 && offset + count as u64 <= length);
        for (index, byte) in chunk[..count].iter().enumerate() {
            assert_eq!(*byte, pattern(offset + index as u64, length));
            hash = (hash ^ u64::from(*byte)).wrapping_mul(0x100000001b3);
        }
        offset += count as u64;
    }
    assert_eq!(io!("eof", file.read(fs, &mut chunk[..1])).unwrap_or(0), 0);
    io!("close_read", file.close(fs));
    info!(
        "L4 EXFAT VERIFY PASS path={} bytes={} hash={:016x} elapsed_ms={}",
        path,
        length,
        hash,
        (Instant::now() - start).as_millis()
    );
}
