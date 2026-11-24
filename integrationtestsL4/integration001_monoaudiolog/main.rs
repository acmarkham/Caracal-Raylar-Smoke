//! ADR-L4-011: minute WAV chunks and syslog sharing the L4 SD card.
#![no_std]
#![no_main]
mod audio;
#[path = "../../unitsmokeL4/05_mono_audio/dfsdm.rs"]
mod dfsdm;
#[path = "../../unitsmokeL4/02_exfat/sd.rs"]
mod sd;
mod setup;
#[path = "../../unitsmokeL4/02_exfat/stack_probe.rs"]
mod stack_probe;
mod streams;
mod verify;

use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
use defmt::{assert, info};
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Input, Level, Output, Pull, Speed},
    pac,
    rcc::{self, *},
    sdmmc::{Config as SdConfig, Sdmmc},
    time::Hertz,
};
use embassy_time::{Duration, Instant, Timer};
use integration_test_l4_001_monoaudiolog::*;
use raylar_audio_recorder_service::{AudioRecorder, AudioRecorderConfig, AudioRecorderError};
use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_logging_service::{LoggingResources, LoggingService};
use raylar_storage_service::StorageLayout;
use raylar_time_service::{TimeConfig, TimeResources, TimeService, UtcTimestamp};
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

const AUDIO_CAPACITY: usize = 4096; // 256 ms, independent of the DMA ring.
static AUDIO: AudioSource<AUDIO_CAPACITY, 1> =
    AudioSource::new(AudioFormat::new(SAMPLE_RATE, 1, 1_000_000));
static TIME: TimeResources<2, 2> = TimeResources::new();
static LOGGING: LoggingResources<192, 8> = LoggingResources::new();
static SDMMC: StaticCell<Sdmmc<'static>> = StaticCell::new();
static CONSUMED_HALVES: AtomicU32 = AtomicU32::new(0);
static MAX_BACKLOG: AtomicU32 = AtomicU32::new(0);
#[global_allocator]
static HEAP: embedded_alloc::LlffHeap = embedded_alloc::LlffHeap::empty();

// Bound stack usage by keeping phase poll temporaries out of the caller.
struct Phase<F>(F);
impl<F: core::future::Future> core::future::Future for Phase<F> {
    type Output = F::Output;
    #[inline(never)]
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        // This wrapper never moves or replaces its pinned inner future.
        unsafe { self.map_unchecked_mut(|phase| &mut phase.0) }.poll(cx)
    }
}

#[embassy_executor::task]
async fn time_task() {
    TimeService::new(&TIME, TimeConfig::default()).run().await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    stack_probe::initialize();
    unsafe {
        embedded_alloc::init!(HEAP, 4096);
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
    assert!(detect.is_low(), "L4 INTEGRATION FAIL no SD card");
    assert!(rcc::clocks(&p.RCC).sys.to_hertz().unwrap().0 == 80_000_000);
    assert!(rcc::clocks(&p.RCC).pllsai1_q.to_hertz().unwrap().0 == 48_000_000);
    assert!(pac::TIM5.psc().read() == 79);
    info!("L4 RECORD boot fake_gps=true time_source=Laboratory bench={} sys_hz=80000000 sd_kernel_hz=48000000 audio_hz=16000 source_bits=24 wav_bits=32", cfg!(feature = "bench-three-minutes"));
    let mut sd_config = SdConfig::default();
    sd_config.data_transfer_timeout = 24_000_000;
    let sdmmc = SDMMC.init(Sdmmc::new_4bit(
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
    ));
    spawner.spawn(time_task().unwrap());
    let setup::Setup {
        epoch,
        old_log_length,
        serial,
        log_stream,
    } = Phase(setup::storage(sdmmc)).await;
    streams::LOG_OPEN.store(true, Relaxed);
    streams::opened();
    let mut logging = LoggingService::<_, 192, 8, 256>::new(
        &LOGGING,
        streams::Syslog {
            stream: log_stream,
            bytes: 0,
            hash: FNV_INITIAL,
            old_length: old_log_length,
        },
    );
    streams::log(format_args!(
        "startup target=L4Evo mode=FAKE-GPS source=Laboratory epoch={} minute_seconds=60",
        epoch
    ));
    streams::log(format_args!(
        "version firmware={} git={}-dirty build_unix={} profile=release",
        env!("CARGO_PKG_VERSION"),
        env!("L4_GIT"),
        env!("L4_BUILD_UNIX")
    ));
    streams::log(format_args!("identity device_uid=unknown board_revision=unknown runtime_crc32=unavailable build_crc32=unavailable"));
    streams::log(format_args!("identity sd_serial={:08x} gps_version=unavailable radio_version=unavailable utc_uncertainty_us=1000000", serial));
    Phase(logging.drain()).await.unwrap();
    Phase(logging.flush()).await.unwrap();
    let mut recorder = AudioRecorder::<_, _, AUDIO_CAPACITY, 1, 4096>::new(
        &AUDIO,
        streams::Recording::new(),
        streams::metadata(),
        AudioRecorderConfig {
            recording_seconds: SEGMENT_SECONDS,
            storage_layout: StorageLayout::HourlyFolders,
        },
    )
    .unwrap();
    Phase(recorder.start_at(UtcTimestamp::new(epoch, 0).unwrap()))
        .await
        .unwrap();
    stack_probe::check("streams_open");
    blue.set_low();
    spawner.spawn(audio::capture(p.PC2, p.PB12, p.DMA1_CH4).unwrap());
    let (_, syslog) = Phase(embassy_futures::join::join(
        recording_session(&mut recorder),
        logging_session(logging),
    ))
    .await;
    audio::DONE.wait().await;
    assert!(cfg!(feature = "bench-three-minutes"));
    assert!(streams::OPEN.load(Relaxed) == 0 && streams::MAX_OPEN.load(Relaxed) == 2);
    assert!(streams::CLOSED.load(Relaxed) == BENCH_SEGMENTS as u32);
    assert!(streams::LOG_CHECKPOINTS_DURING_AUDIO.load(Relaxed) >= 17);
    assert!(audio::DMA_ERRORS.load(Relaxed) == 0);
    let device = Phase(setup::unmount()).await;
    Phase(verify::files(device, recorder.storage(), &syslog)).await;
    stack_probe::check("verified");
    info!("L4 RECORD PASS segments=3 samples_per_segment=960000 wav_bytes={} open_streams_max={} dma_errors={} audio_backlog_max={} copy_max_us={} append_max_us={} syslog_bytes={} syslog_checkpoints_during_audio={} sd_writes={} sd_dma_irqs={} fake_gps=true", FILE_BYTES, streams::MAX_OPEN.load(Relaxed), audio::DMA_ERRORS.load(Relaxed), MAX_BACKLOG.load(Relaxed), audio::MAX_COPY_US.load(Relaxed), streams::MAX_APPEND_US.load(Relaxed), syslog.bytes, streams::LOG_CHECKPOINTS_DURING_AUDIO.load(Relaxed), sd::WRITES.load(Relaxed), sd::DMA_IRQS.load(Relaxed));
    loop {
        blue.toggle();
        Timer::after_millis(500).await;
    }
}

type Recorder = AudioRecorder<
    'static,
    streams::Recording,
    raylar_audio_recorder_service::TimeMetadataSource<'static, 2, 2>,
    AUDIO_CAPACITY,
    1,
    4096,
>;
async fn recording_session(recorder: &mut Recorder) {
    loop {
        match recorder
            .record_next_with_source_timeout(Duration::from_secs(3))
            .await
        {
            Ok(progress) => {
                assert!(
                    progress.dropped_samples == 0 && progress.pcm_samples % 512 == 0,
                    "L4 INTEGRATION FAIL recorder lost samples"
                );
                CONSUMED_HALVES.fetch_add((progress.pcm_samples / 512) as u32, Relaxed);
                MAX_BACKLOG.fetch_max(progress.backlog_samples as u32, Relaxed);
            }
            Err(AudioRecorderError::Storage(streams::RecordError::BenchComplete))
                if cfg!(feature = "bench-three-minutes") =>
            {
                assert!(streams::CLOSED.load(Relaxed) == BENCH_SEGMENTS as u32);
                streams::log(format_args!(
                    "bench_complete segments=3 dma_errors=0 dropped_samples=0"
                ));
                audio::STOP.store(true, Relaxed);
                streams::STOP_LOG.store(true, Relaxed);
                return;
            }
            Err(error) => defmt::panic!("L4 INTEGRATION FAIL recorder {}", error),
        }
    }
}

async fn logging_session(
    mut logging: LoggingService<'static, streams::Syslog, 192, 8, 256>,
) -> streams::Syslog {
    let mut next_heartbeat = Instant::now() + Duration::from_secs(1);
    let mut next_flush = Instant::now() + Duration::from_secs(10);
    let mut heartbeat = 0u32;
    loop {
        let now = Instant::now();
        if now >= next_heartbeat {
            heartbeat += 1;
            streams::log(format_args!("heartbeat={} published_halves={} consumed_halves={} closed={} backlog_max={} dma_errors={}", heartbeat, audio::PUBLISHED_BLOCKS.load(Relaxed), CONSUMED_HALVES.load(Relaxed), streams::CLOSED.load(Relaxed), MAX_BACKLOG.load(Relaxed), audio::DMA_ERRORS.load(Relaxed)));
            if heartbeat % 10 == 0 {
                info!("L4 RECORD progress seconds={} closed={} backlog_max={} append_max_us={} log_bytes={} open_streams={}", heartbeat, streams::CLOSED.load(Relaxed), MAX_BACKLOG.load(Relaxed), streams::MAX_APPEND_US.load(Relaxed), logging.stats().bytes_written, streams::OPEN.load(Relaxed));
            }
            next_heartbeat += Duration::from_secs(1);
        }
        logging.drain().await.unwrap();
        if now >= next_flush {
            logging.checkpoint().await.unwrap();
            next_flush += Duration::from_secs(10);
        }
        if streams::STOP_LOG.load(Relaxed) {
            break;
        }
        Timer::after_millis(20).await;
    }
    logging.drain().await.unwrap();
    logging.flush().await.unwrap();
    let stats = logging.stats();
    assert!(
        stats.dropped_messages == 0
            && stats.truncated_messages == 0
            && stats.write_failures == 0
            && stats.queue_depth == 0
    );
    let syslog = logging.into_sink();
    streams::STORAGE
        .lock()
        .await
        .as_mut()
        .unwrap()
        .finish(syslog.stream)
        .await
        .unwrap();
    streams::LOG_OPEN.store(false, Relaxed);
    streams::closed();
    syslog
}
