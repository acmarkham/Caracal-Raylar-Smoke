//! Isolated paired-file lifecycle benchmark. Writes TEST DATA, not audio.
//! No microphone, encoder, GPS, logger, or other application task is started.
#![no_std]
#![no_main]

#[path = "../storage/common.rs"]
#[allow(dead_code)]
mod common;

use aligned::Aligned;
use core::cell::Cell;
use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::sdmmc::{
    sd::{CmdBlock, StorageDevice},
    Config, Sdmmc,
};
use embassy_time::{Duration, Instant, Timer};
use raylar_board_v1p0::{Board, Irqs, SdCard};
use raylar_drivers::storage::stm32::Stm32SdBlockDevice;
use raylar_drivers::storage::{
    detect_exfat_volume, PartitionedBlockDevice, StorageBlockDevice, StorageDriver,
};
use raylar_storage_service::{StorageLayout, StorageService, StreamHandle, StreamKind, UtcClock};
use raylar_time_service::UtcTimestamp;
use static_cell::{ConstStaticCell, StaticCell};
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static HEAP: embedded_alloc::LlffHeap = embedded_alloc::LlffHeap::empty();
static SDMMC: StaticCell<Sdmmc<'static>> = StaticCell::new();
static STATS: ConstStaticCell<Cell<Io>> = ConstStaticCell::new(Cell::new(Io::ZERO));
static DATA: ConstStaticCell<[u8; 4096]> = ConstStaticCell::new([0; 4096]);
// Compile-time settings make accelerated, populated-folder and paced runs use
// exactly the same benchmark. Exclusive creation rejects existing test files.
const fn setting(value: Option<&str>, default: u32) -> u32 {
    let Some(value) = value else {
        return default;
    };
    let bytes = value.as_bytes();
    assert!(!bytes.is_empty());
    let mut result = 0u32;
    let mut i = 0;
    while i < bytes.len() {
        assert!(bytes[i] >= b'0' && bytes[i] <= b'9');
        result = match result.checked_mul(10) {
            Some(v) => v,
            None => panic!("setting overflow"),
        };
        result = match result.checked_add((bytes[i] - b'0') as u32) {
            Some(v) => v,
            None => panic!("setting overflow"),
        };
        i += 1;
    }
    result
}
const START_UTC: i64 = setting(option_env!("SD_STRESS_START_UTC"), 1_900_000_800) as i64;
const PAIRS: u32 = setting(option_env!("SD_STRESS_PAIRS"), 256);
const SMALL_PAIRS: u32 = setting(option_env!("SD_STRESS_SMALL_PAIRS"), 32);
const FULL_BLOCKS: u32 = setting(option_env!("SD_STRESS_FULL_BLOCKS"), 937);
const REWRITE_UNTIL: u32 = setting(option_env!("SD_STRESS_REWRITE_UNTIL"), 224);
const ALTERNATE_REWRITE: bool = setting(option_env!("SD_STRESS_ALTERNATE_REWRITE"), 0) != 0;
const PACE_MS: u32 = setting(option_env!("SD_STRESS_PACE_MS"), 0);
const AUDIO_RETENTION_US: u64 = 1_333_312;

#[cfg(feature = "deep-sd-diagnostics")]
static DEEP_DIRECTORY_WRITES: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

#[cfg(feature = "deep-sd-diagnostics")]
fn verify_deep(scope: raylar_drivers::storage::diagnostics::Scope, observed: Io) {
    let snapshot = scope.finish();
    assert_eq!(
        snapshot.file.reads.calls + snapshot.directory.reads.calls,
        observed.reads
    );
    assert_eq!(
        snapshot.file.reads.sectors + snapshot.directory.reads.sectors,
        observed.read_sectors
    );
    assert_eq!(
        snapshot.file.writes.calls + snapshot.directory.writes.calls,
        observed.writes
    );
    assert_eq!(
        snapshot.file.writes.sectors + snapshot.directory.writes.sectors,
        observed.write_sectors
    );
    DEEP_DIRECTORY_WRITES.fetch_max(
        snapshot.directory.writes.calls,
        core::sync::atomic::Ordering::Relaxed,
    );
}

#[derive(Clone, Copy)]
struct Io {
    reads: u32,
    read_sectors: u32,
    read_us: u64,
    read_max: u64,
    read_lba: u32,
    writes: u32,
    write_sectors: u32,
    write_us: u64,
    write_max: u64,
    write_lba: u32,
}
impl Io {
    const ZERO: Self = Self {
        reads: 0,
        read_sectors: 0,
        read_us: 0,
        read_max: 0,
        read_lba: 0,
        writes: 0,
        write_sectors: 0,
        write_us: 0,
        write_max: 0,
        write_lba: 0,
    };
}
struct TimedDevice<D> {
    inner: D,
    stats: &'static Cell<Io>,
}
impl<D: StorageBlockDevice<512>> StorageBlockDevice<512> for TimedDevice<D> {
    type Error = D::Error;
    type Align = D::Align;
    async fn read(
        &mut self,
        lba: u32,
        data: &mut [Aligned<Self::Align, [u8; 512]>],
    ) -> Result<(), Self::Error> {
        let start = Instant::now();
        let result = self.inner.read(lba, data).await;
        let elapsed = start.elapsed().as_micros();
        let mut s = self.stats.get();
        s.reads += 1;
        s.read_sectors += data.len() as u32;
        s.read_us += elapsed;
        if elapsed > s.read_max {
            s.read_max = elapsed;
            s.read_lba = lba;
        }
        self.stats.set(s);
        result
    }
    async fn write(
        &mut self,
        lba: u32,
        data: &[Aligned<Self::Align, [u8; 512]>],
    ) -> Result<(), Self::Error> {
        let start = Instant::now();
        let result = self.inner.write(lba, data).await;
        let elapsed = start.elapsed().as_micros();
        let mut s = self.stats.get();
        s.writes += 1;
        s.write_sectors += data.len() as u32;
        s.write_us += elapsed;
        if elapsed > s.write_max {
            s.write_max = elapsed;
            s.write_lba = lba;
        }
        self.stats.set(s);
        result
    }
    async fn size(&mut self) -> Result<u64, Self::Error> {
        self.inner.size().await
    }
}

struct Clock;
impl UtcClock for Clock {
    fn current_utc(&self) -> Option<UtcTimestamp> {
        UtcTimestamp::new(START_UTC, 0)
    }
}
type Backend =
    StorageDriver<TimedDevice<PartitionedBlockDevice<Stm32SdBlockDevice<'static, 'static>>>>;
type Storage = StorageService<Backend, Clock, 512, 3, 16_384>;
static STORAGE: StaticCell<Storage> = StaticCell::new();

#[derive(Clone, Copy)]
struct Measurement {
    us: u64,
    io: Io,
}
impl Measurement {
    fn start(stats: &Cell<Io>) -> Instant {
        stats.set(Io::ZERO);
        Instant::now()
    }
    fn stop(start: Instant, stats: &Cell<Io>) -> Self {
        Self {
            us: start.elapsed().as_micros(),
            io: stats.get(),
        }
    }
    fn log(self, pair: u32, phase: &str) {
        let s = self.io;
        info!(
            "SDOP pair={} phase={} us={} reads={} rsectors={} read_us={} read_max={} read_lba={} writes={} wsectors={} write_us={} write_max={} write_lba={}",
            pair,
            phase,
            self.us,
            s.reads,
            s.read_sectors,
            s.read_us,
            s.read_max,
            s.read_lba,
            s.writes,
            s.write_sectors,
            s.write_us,
            s.write_max,
            s.write_lba
        );
    }
}

async fn open_pair(
    storage: &mut Storage,
    stats: &Cell<Io>,
    pair: u32,
) -> ((StreamHandle, StreamHandle), [Measurement; 3]) {
    let utc = unwrap!(UtcTimestamp::new(START_UTC + i64::from(pair) * 60, 0));
    #[cfg(feature = "deep-sd-diagnostics")]
    let deep = raylar_drivers::storage::diagnostics::Scope::begin().unwrap();
    let t = Measurement::start(stats);
    let reference = unwrap!(
        storage
            .begin_new_audio_with_suffix_at(StorageLayout::HourlyFolders, utc, "sdref")
            .await
    );
    let reference_open = Measurement::stop(t, stats);
    #[cfg(feature = "deep-sd-diagnostics")]
    verify_deep(deep, reference_open.io);
    #[cfg(feature = "deep-sd-diagnostics")]
    let deep = raylar_drivers::storage::diagnostics::Scope::begin().unwrap();
    let t = Measurement::start(stats);
    let extra = unwrap!(
        storage
            .begin_new_audio_with_suffix_at(StorageLayout::HourlyFolders, utc, "sdextra")
            .await
    );
    let extra_open = Measurement::stop(t, stats);
    #[cfg(feature = "deep-sd-diagnostics")]
    verify_deep(deep, extra_open.io);
    let t = Measurement::start(stats);
    unwrap!(storage.write(reference, &[0x52; 128]).await);
    unwrap!(storage.write(extra, &[0x58; 256]).await);
    let headers = Measurement::stop(t, stats);
    ((reference, extra), [reference_open, extra_open, headers])
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let (config, correction) = unwrap!(common::mcu_config_with_hse_error_ppm(-9));
    let p = embassy_stm32::init(config);
    correction.apply();
    let Board { sd, .. } = Board::new(p);
    let SdCard {
        sdmmc,
        clk,
        cmd,
        d0,
        d1,
        d2,
        d3,
        switch,
        mut power,
    } = sd;
    info!(
        "SD LIFECYCLE STRESS START pairs={} start_utc={} small_pairs={} full_blocks={} rewrite_until={} alternate_rewrite={} pace_ms={} buffers=16384 streams=3 retention_us={} TEST DATA ONLY",
        PAIRS,
        START_UTC,
        SMALL_PAIRS,
        FULL_BLOCKS,
        REWRITE_UNTIL,
        ALTERNATE_REWRITE,
        PACE_MS,
        AUDIO_RETENTION_US
    );
    power.set_high();
    assert!(switch.is_low());
    let mut config = Config::default();
    config.data_transfer_timeout = 120_000_000;
    let sdmmc = SDMMC.init(Sdmmc::new_4bit(
        sdmmc, Irqs, clk, cmd, d0, d1, d2, d3, config,
    ));
    power.set_low();
    Timer::after_secs(1).await;
    let mut cmd_block = CmdBlock::new();
    let card = unwrap!(
        StorageDevice::new_sd_card(sdmmc, &mut cmd_block, embassy_stm32::time::mhz(24)).await
    );
    // Embassy 0.6 disables PWRSAV. Enable idle clock gating after card initialization.
    embassy_stm32::pac::SDMMC1.clkcr().modify(|w| w.set_pwrsav(true));
    let mut device = Stm32SdBlockDevice::new(card);
    let identity = device.device_identity();
    info!(
        "SDIDENT mid={} serial={} capacity_bytes={}",
        identity.manufacturer_id, identity.serial_number, identity.capacity_bytes
    );
    let volume = unwrap!(detect_exfat_volume(&mut device).await);
    let stats: &'static Cell<Io> = STATS.take();
    let driver = StorageDriver::new_with_device_identity(
        TimedDevice {
            inner: PartitionedBlockDevice::new(device, volume),
            stats,
        },
        Some(identity),
    );
    let storage = STORAGE.init(unwrap!(Storage::new(driver, Clock)));
    unwrap!(storage.mount().await);
    info!("SDSPACE {:?}", unwrap!(storage.space_info().await));
    // A persistent third stream creates production-like interleaved allocation.
    // It lives in the synthetic test folder, leaving the real syslog untouched.
    let log = unwrap!(
        storage
            .begin_stream(StreamKind::Log, StorageLayout::HourlyFolders)
            .await
    );
    let data = DATA.take();
    let mut random = 0x5a17_2026u32;
    for byte in data.iter_mut() {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        *byte = random as u8;
    }
    let ((mut reference, mut extra), initial) = open_pair(storage, stats, 0).await;
    for (phase, m) in ["open_ref", "open_extra", "headers"].iter().zip(initial) {
        m.log(0, phase);
    }
    let run_start = Instant::now();
    let mut overruns = 0;
    let mut worst = 0;
    for pair in 0..PAIRS {
        // Controls: 32 tiny pairs, 192 representative pairs with STREAMINFO
        // rewrite, then 32 representative pairs without the header rewrite.
        let blocks = if pair < SMALL_PAIRS { 8 } else { FULL_BLOCKS };
        let rewrite = if ALTERNATE_REWRITE {
            pair % 2 == 0
        } else {
            pair < REWRITE_UNTIL
        };
        let mut append_max = 0;
        let mut log_max = 0;
        let t = Measurement::start(stats);
        let mut reference_bytes = 128u64;
        let mut extra_bytes = 256u64;
        for block in 0..blocks {
            let block_start = Instant::now();
            let reference_len = 1280 + (block % 17) as usize;
            let extra_len = 3840 + (block % 23) as usize;
            let start = Instant::now();
            unwrap!(storage.write(reference, &data[..reference_len]).await);
            append_max = append_max.max(start.elapsed().as_micros());
            let start = Instant::now();
            unwrap!(storage.write(extra, &data[..extra_len]).await);
            append_max = append_max.max(start.elapsed().as_micros());
            reference_bytes += reference_len as u64;
            extra_bytes += extra_len as u64;
            if block % 16 == 0 {
                unwrap!(storage.write(extra, &data[..48]).await);
                extra_bytes += 48;
                let start = Instant::now();
                unwrap!(storage.write(log, &data[..701]).await);
                unwrap!(storage.checkpoint(log).await);
                log_max = log_max.max(start.elapsed().as_micros());
            }
            if PACE_MS != 0 {
                Timer::at(block_start + Duration::from_millis(u64::from(PACE_MS))).await;
            }
        }
        let payload = Measurement::stop(t, stats);
        let transition = Instant::now();
        #[cfg(feature = "deep-sd-diagnostics")]
        let deep = raylar_drivers::storage::diagnostics::Scope::begin().unwrap();
        let t = Measurement::start(stats);
        if rewrite {
            unwrap!(storage.rewrite(reference, 8, &[0x49; 34]).await);
        }
        let patch = Measurement::stop(t, stats);
        #[cfg(feature = "deep-sd-diagnostics")]
        verify_deep(deep, patch.io);
        #[cfg(feature = "deep-sd-diagnostics")]
        let deep = raylar_drivers::storage::diagnostics::Scope::begin().unwrap();
        let t = Measurement::start(stats);
        unwrap!(storage.finish(reference).await);
        let close_ref = Measurement::stop(t, stats);
        #[cfg(feature = "deep-sd-diagnostics")]
        verify_deep(deep, close_ref.io);
        #[cfg(feature = "deep-sd-diagnostics")]
        let deep = raylar_drivers::storage::diagnostics::Scope::begin().unwrap();
        let t = Measurement::start(stats);
        unwrap!(storage.write(extra, &[0x45; 52]).await);
        unwrap!(storage.finish(extra).await);
        let close_extra = Measurement::stop(t, stats);
        #[cfg(feature = "deep-sd-diagnostics")]
        verify_deep(deep, close_extra.io);
        let next = if pair + 1 < PAIRS {
            Some(open_pair(storage, stats, pair + 1).await)
        } else {
            None
        };
        let transition_us = transition.elapsed().as_micros();
        if next.is_some() {
            worst = worst.max(transition_us);
            if transition_us > AUDIO_RETENTION_US {
                overruns += 1;
            }
        }
        // All RTT output is after the uninterrupted finalization/open interval.
        info!(
            "SDPAIR pair={} blocks={} rewrite={} ref_bytes={} extra_bytes={} payload_us={} append_max={} log_max={} transition_us={} has_next={} overrun={}",
            pair,
            blocks,
            rewrite,
            reference_bytes,
            extra_bytes + 52,
            payload.us,
            append_max,
            log_max,
            transition_us,
            next.is_some(),
            next.is_some() && transition_us > AUDIO_RETENTION_US
        );
        payload.log(pair, "payload");
        patch.log(pair, "rewrite");
        close_ref.log(pair, "close_ref");
        close_extra.log(pair, "close_extra");
        if let Some(((r, e), measurements)) = next {
            reference = r;
            extra = e;
            // Attribute new opens to the preceding transition for easy grouping.
            for (phase, m) in ["next_open_ref", "next_open_extra", "next_headers"]
                .iter()
                .zip(measurements)
            {
                m.log(pair, phase);
            }
        }
    }
    unwrap!(storage.finish(log).await);
    #[cfg(feature = "deep-sd-diagnostics")]
    {
        let directory_writes = DEEP_DIRECTORY_WRITES.load(core::sync::atomic::Ordering::Relaxed);
        if START_UTC / 3600 != (START_UTC + i64::from(PAIRS - 1) * 60) / 3600 {
            assert!(directory_writes >= 256);
        }
        info!(
            "SD DEEP DIAGNOSTICS PASS independent read/write counts and sectors agree across opens, rewrites and closes; directory_writes_max={}",
            directory_writes
        );
    }
    info!(
        "SD LIFECYCLE STRESS PASS pairs={} overruns={} worst_transition_us={} elapsed_us={}",
        PAIRS,
        overruns,
        worst,
        run_start.elapsed().as_micros()
    );
    // Keep the active-low power pin alive throughout and after the test.
    loop {
        power.set_low();
        Timer::after_secs(30).await;
    }
}
