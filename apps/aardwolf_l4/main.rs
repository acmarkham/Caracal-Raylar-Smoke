//! Aardwolf Evo L4: shared services, mono FLAC and M10 PPS timing.
#![no_std]
#![no_main]
mod audio;
mod diagnostics;
mod fault;
mod gnss;
mod setup;
mod streams;
mod ui;
mod verify;
use aardwolf_l4::*;
use caracal_evo::{capture, dfsdm, sd, stack_probe};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering::Relaxed};
use defmt_rtt as _;
use embassy_executor::Spawner;
use embassy_stm32::{
    gpio::{Input, Level, Output, Pull, Speed},
    pac,
    rcc::{self, *},
    sdmmc::{Config as SdConfig, Sdmmc},
    time::Hertz,
    usart::{self, BufferedUart},
};
use embassy_time::{Duration, Instant, Timer};
use raylar_audio_recorder_service::{AudioRecorder, AudioRecorderConfig, AudioRecorderError};
use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_drivers::gps::GpsResources;
use raylar_location_service::LocationResources;
use raylar_logging_service::{LoggingResources, LoggingService};
use raylar_storage_service::StorageLayout;
use raylar_time_service::{TimeResources, UtcTimestamp};
use static_cell::{ConstStaticCell, StaticCell};

const AUDIO_CAPACITY: usize = 4096;
static AUDIO: AudioSource<AUDIO_CAPACITY, 1> =
    AudioSource::new(AudioFormat::new(SAMPLE_RATE, 1, 1_000_000));
static TIME: TimeResources<2, 2> = TimeResources::new();
static GPS: GpsResources<128, 4, 8, 2> = GpsResources::new();
static LOCATION: LocationResources<2> = LocationResources::new();
static LOGGING: LoggingResources<192, 8> = LoggingResources::new();
static SDMMC: StaticCell<Sdmmc<'static>> = StaticCell::new();
static UART_RX: ConstStaticCell<[u8; 2048]> = ConstStaticCell::new([0; 2048]);
static UART_TX: ConstStaticCell<[u8; 128]> = ConstStaticCell::new([0; 128]);
static CONSUMED_SAMPLES: AtomicU32 = AtomicU32::new(0);
static MAX_CODEC_US: AtomicU32 = AtomicU32::new(0);
#[cfg(not(feature = "wav"))]
static ENCODED: ConstStaticCell<[u8; FLAC_ENCODE_BYTES]> =
    ConstStaticCell::new([0; FLAC_ENCODE_BYTES]);
#[cfg(not(feature = "wav"))]
static PCM: ConstStaticCell<[i32; FLAC_BLOCK]> = ConstStaticCell::new([0; FLAC_BLOCK]);
static MAX_BACKLOG: AtomicU32 = AtomicU32::new(0);
static ARMED: AtomicBool = AtomicBool::new(false);
#[global_allocator]
static HEAP: embedded_alloc::LlffHeap = embedded_alloc::LlffHeap::empty();

// Polling frames, including filesystem temporaries, must fit the SRAM2 stack.
pub struct Phase<F>(pub F);
impl<F: core::future::Future> core::future::Future for Phase<F> {
    type Output = F::Output;
    #[inline(never)]
    fn poll(
        self: core::pin::Pin<&mut Self>,
        cx: &mut core::task::Context<'_>,
    ) -> core::task::Poll<Self::Output> {
        // SAFETY: the pinned inner future is never moved or replaced.
        unsafe { self.map_unchecked_mut(|s| &mut s.0) }.poll(cx)
    }
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
    let red = Output::new(p.PB11, Level::High, Speed::Low);
    let green = Output::new(p.PC3, Level::High, Speed::Low);
    let blue = Output::new(p.PC1, Level::High, Speed::Low);
    let detect = Input::new(p.PC6, Pull::Up);
    let enable = Input::new(p.PB4, Pull::None);
    let buzzer = caracal_evo::buzzer::Buzzer::new(p.TIM8, p.PB14);
    spawner.spawn(ui::task(red, blue, green, detect, buzzer).unwrap());
    Timer::after_millis(100).await;
    if fault::active() {
        fault::park(fault::Fault::SdRemoved).await;
    }
    defmt::assert!(pac::TIM5.psc().read() == 79);
    defmt::assert!(rcc::clocks(&p.RCC).sys.to_hertz().unwrap().0 == 80_000_000);
    defmt::assert!(rcc::clocks(&p.RCC).pllsai1_q.to_hertz().unwrap().0 == 48_000_000);
    defmt::info!(
        "AARDWOLF L4 boot uid={} git={} codec={} bits={} block={} predictor=Fixed1 gps_off_s={}",
        embassy_stm32::uid::uid_hex(),
        env!("L4_GIT"),
        CODEC,
        if cfg!(feature = "wav") { 32 } else { 24 },
        FLAC_BLOCK,
        config::GPS_OFF_SECONDS
    );
    let chain = capture::start(p.TIM3, p.TIM2, p.PB5);
    defmt::info!("AARDWOLF L4 PPS uncertainty_us={}", chain.uncertainty_us);
    let mut uart_config = usart::Config::default();
    uart_config.baudrate = 38400;
    uart_config.eager_reads = Some(1);
    let uart = BufferedUart::new(
        p.USART2,
        p.PA3,
        p.PA2,
        UART_TX.take(),
        UART_RX.take(),
        gnss::Irqs,
        uart_config,
    )
    .unwrap();
    spawner.spawn(gnss::time_task().unwrap());
    spawner.spawn(gnss::anchors_task().unwrap());
    spawner.spawn(gnss::location_task().unwrap());
    spawner.spawn(gnss::quality_task().unwrap());
    spawner.spawn(
        gnss::driver_task(gnss::Serial::new(uart), gnss::Pps { _chain: chain }, enable).unwrap(),
    );
    GPS.command_sender()
        .send(raylar_drivers::gps::GpsCommand::Start)
        .await;
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
    let setup::Setup {
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
        "startup target=L4Evo mode=REAL-GPS firmware={} git={} build_unix={}",
        env!("CARGO_PKG_VERSION"),
        env!("L4_GIT"),
        env!("L4_BUILD_UNIX")
    ));
    streams::CARD_SERIAL.store(serial, Relaxed);
    streams::log(format_args!("version profile=release git_state={} board_revision=Unknown runtime_crc32=Unavailable build_crc32=Unavailable",env!("L4_GIT_STATE")));
    streams::log(format_args!(
        "device node_id={:08x}",
        caracal_evo::identity::uid().serials().serial_32
    ));
    streams::log(format_args!("identity uid={} sd_serial={:08x} gps=Quescan_G10A-F30 radio=absent battery=absent button=absent",embassy_stm32::uid::uid_hex(),serial));
    let version = gnss::VERSION.lock(|s| s.borrow().clone());
    streams::log(format_args!(
        "gps_version {}",
        if version.is_empty() {
            "Unavailable (discovery pending)"
        } else {
            version.as_str()
        }
    ));
    streams::log(format_args!("policy mono=16000 codec={} source_bits=24 block={} predictor=Fixed1 segment_s=60 gps_off_s={}",CODEC,FLAC_BLOCK,config::GPS_OFF_SECONDS));
    if Phase(logging.drain()).await.is_err() || Phase(logging.flush()).await.is_err() {
        fault::park(fault::Fault::Storage).await;
    }
    #[cfg(feature = "wav")]
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
    #[cfg(not(feature = "wav"))]
    let mut recorder = AudioRecorder::<_, _, AUDIO_CAPACITY, 1, FLAC_ENCODE_BYTES, FLAC_BLOCK>::new_flac_with_buffers(
        &AUDIO, streams::Recording::new(), streams::metadata(),
        AudioRecorderConfig { recording_seconds: SEGMENT_SECONDS, storage_layout: StorageLayout::HourlyFolders },
        raylar_audio_recorder_service::FlacConfig { sample_rate_hz: SAMPLE_RATE, bits_per_sample: SOURCE_BITS,
            block_samples: FLAC_BLOCK, prediction: raylar_audio_recorder_service::FlacPrediction::Fixed(1) },
        ENCODED.take(), PCM.take(),
    ).unwrap();
    spawner.spawn(diagnostics::cpu_task().unwrap());
    spawner.spawn(audio::capture(p.PC2, p.PB12, p.DMA1_CH4).unwrap());
    let (_, syslog) = Phase(embassy_futures::join::join(
        recording_session(&mut recorder),
        logging_session(logging),
    ))
    .await;
    audio::DONE.wait().await;
    defmt::assert!(
        cfg!(any(feature = "bench-audio-ui", feature = "bench-codec"))
            || (cfg!(feature = "bench-gps-cycle") && gnss::BENCH_FINISH.load(Relaxed))
    );
    defmt::assert!(streams::OPEN.load(Relaxed) == 0 && streams::MAX_OPEN.load(Relaxed) == 2);
    defmt::assert!(audio::DMA_ERRORS.load(Relaxed) == 0 && capture::FAULTS.load(Relaxed) == 0);
    defmt::assert!(
        streams::CLOSED.load(Relaxed) == recorder.storage().opened
            && streams::CLOSED.load(Relaxed)
                >= if cfg!(any(feature = "bench-audio-ui", feature = "bench-codec")) {
                    2
                } else {
                    10
                }
    );
    defmt::assert!(
        cfg!(any(feature = "bench-audio-ui", feature = "bench-codec"))
            || (TIME.time_state().frequency_calibration_locked
                && GPS.stats().num_phase_qualified_shutdowns > 0
                && gnss::SLEEPS.load(Relaxed) >= 2
                && gnss::VERSIONS.load(Relaxed) >= 3)
    );
    recorder.storage_mut().export_fixture().await;
    let device = Phase(setup::unmount()).await;
    Phase(verify::files(device, recorder.storage_mut(), &syslog)).await;
    stack_probe::check("verified");
    defmt::info!("AARDWOLF L4 PASS segments={} dma_errors={} backlog_max={} copy_max_us={} append_max_us={} syslog_bytes={} log_checkpoints={} gps_sleeps={} gps_wakes={} calibrated_ppb={}",streams::CLOSED.load(Relaxed),audio::DMA_ERRORS.load(Relaxed),MAX_BACKLOG.load(Relaxed),audio::MAX_COPY_US.load(Relaxed),streams::MAX_APPEND_US.load(Relaxed),syslog.bytes,streams::LOG_CHECKPOINTS_DURING_AUDIO.load(Relaxed),gnss::SLEEPS.load(Relaxed),gnss::WAKES.load(Relaxed),TIME.time_state().calibrated_frequency_error_ppb);
    loop {
        Timer::after_secs(60).await;
    }
}

#[cfg(feature = "wav")]
type Recorder = AudioRecorder<
    'static,
    streams::Recording,
    raylar_audio_recorder_service::TimeMetadataSource<'static, 2, 2>,
    AUDIO_CAPACITY,
    1,
    4096,
>;
#[cfg(not(feature = "wav"))]
type Recorder = AudioRecorder<
    'static,
    streams::Recording,
    raylar_audio_recorder_service::TimeMetadataSource<'static, 2, 2>,
    AUDIO_CAPACITY,
    1,
    FLAC_ENCODE_BYTES,
    FLAC_BLOCK,
    raylar_audio_recorder_service::BorrowedBuffers<'static, FLAC_ENCODE_BYTES, FLAC_BLOCK>,
>;
async fn recording_session(recorder: &mut Recorder) {
    while TIME.current_utc().is_err() {
        Timer::after_millis(100).await;
    }
    let time = TIME.time_state();
    let now = time.system_to_utc_holdover(Instant::now()).unwrap();
    let mut next = policy::next_minute(now.as_micros());
    if next - now.as_micros() < 4_000_000 {
        next += 60_000_000;
    }
    let start = time
        .utc_to_system_holdover(UtcTimestamp::from_micros(next))
        .unwrap();
    CONSUMED_SAMPLES.store(0, Relaxed);
    ARMED.store(true, Relaxed);
    audio::START.signal(start);
    if !cfg!(feature = "wav") {
        if let Err(error) = Phase(
            recorder
                .storage_mut()
                .prepare_at(UtcTimestamp::from_micros(next)),
        )
        .await
        {
            defmt::error!("AARDWOLF L4 preallocate {}", error);
            fault::park(fault::Fault::Storage).await;
        }
    }
    // Prepare the filesystem/cache and header before any PCM is published.
    // Large-card bitmap scans and first-directory allocation have no DMA deadline.
    if let Err(error) = Phase(recorder.start_at(UtcTimestamp::from_micros(next))).await {
        defmt::error!("AARDWOLF L4 prepare {}", error);
        fault::park(fault::Fault::Storage).await;
    }
    defmt::assert!(
        Instant::now() < start,
        "AARDWOLF L4 storage preparation missed UTC start"
    );
    Timer::at(start).await;
    streams::log(format_args!(
        "audio_capture_started utc_us={} ticks={}",
        next,
        start.as_ticks()
    ));
    stack_probe::check("recording");
    loop {
        match recorder
            .record_next_with_source_timeout(Duration::from_secs(3))
            .await
        {
            Ok(progress) => {
                defmt::assert!(progress.dropped_samples == 0, "AARDWOLF L4 dropped samples");
                CONSUMED_SAMPLES.fetch_add(progress.pcm_samples as u32, Relaxed);
                MAX_BACKLOG.fetch_max(progress.backlog_samples as u32, Relaxed);
                MAX_CODEC_US.fetch_max(progress.codec_elapsed_us as u32, Relaxed);
                if cfg!(any(feature = "bench-codec", feature = "bench-audio-ui"))
                    && progress.rotated
                    && streams::CLOSED.load(Relaxed) >= 2
                {
                    recorder.stop().await.unwrap();
                    finish_bench();
                    return;
                }
            }
            Err(AudioRecorderError::Storage(streams::RecordError::BenchComplete))
                if cfg!(any(
                    feature = "bench-gps-cycle",
                    feature = "bench-audio-ui",
                    feature = "bench-codec"
                )) =>
            {
                finish_bench();
                return;
            }
            Err(error) => {
                defmt::error!("AARDWOLF L4 recorder {}", error);
                fault::park(match error {
                    AudioRecorderError::Storage(_) => fault::Fault::Storage,
                    _ => fault::Fault::Audio,
                })
                .await;
            }
        }
    }
}

fn finish_bench() {
    streams::log(format_args!(
        "bench_complete segments={} dma_errors=0 dropped_samples=0 real_gps=true",
        streams::CLOSED.load(Relaxed)
    ));
    ARMED.store(false, Relaxed);
    audio::STOP.store(true, Relaxed);
    streams::STOP_LOG.store(true, Relaxed);
}

async fn logging_session(
    mut logging: LoggingService<'static, streams::Syslog, 192, 8, 256>,
) -> streams::Syslog {
    let mut next_heartbeat = Instant::now() + Duration::from_secs(1);
    let mut next_flush = Instant::now() + Duration::from_secs(10);
    let mut heartbeat = 0u32;
    let mut cycle = gnss::CycleCheck::new();
    let mut versions = 0;
    loop {
        if fault::active() {
            core::future::pending::<()>().await;
        }
        let now = Instant::now();
        if now >= next_heartbeat {
            heartbeat += 1;
            cycle.update();
            let current_versions = gnss::VERSIONS.load(Relaxed);
            if current_versions != versions {
                versions = current_versions;
                let identity = gnss::VERSION.lock(|s| s.borrow().clone());
                streams::log(format_args!(
                    "gps_identity_verified count={} {} uart_profile_ram_bbr={}",
                    versions,
                    identity.as_str(),
                    gnss::PROFILE_VALID.load(Relaxed)
                ));
            }
            streams::log(format_args!(
                "heartbeat={} published_samples={} consumed_samples={} closed={} backlog_max={} dma_errors={}",
                heartbeat,
                audio::PUBLISHED_SAMPLES.load(Relaxed),
                CONSUMED_SAMPLES.load(Relaxed),
                streams::CLOSED.load(Relaxed),
                MAX_BACKLOG.load(Relaxed),
                audio::DMA_ERRORS.load(Relaxed)
            ));
            if heartbeat % 10 == 0 {
                let gps = GPS.stats();
                let time = TIME.time_state();
                let location = LOCATION.state();
                streams::log(format_args!(
                    "gps state={:?} pps={} fixes={} rx={} uart_errors={} frame_errors={}",
                    gps.operating_state,
                    gps.num_pps_events,
                    gps.num_fixes,
                    gnss::RX_BYTES.load(Relaxed),
                    gnss::UART_ERRORS.load(Relaxed),
                    gnss::FRAME_ERRORS.load(Relaxed)
                ));
                streams::log(format_args!("time accepted={} rejected={} cal_samples={} locked={} ppb={} residual={:?} uncertainty={} gate={}",time.accepted_anchors,time.rejected_anchors,time.frequency_calibration_samples,time.frequency_calibration_locked,time.calibrated_frequency_error_ppb,time.last_anchor_residual_us,time.uncertainty_us,time.pps_reacquisition_active));
                streams::log(format_args!(
                    "location valid={} lat_e7={} lon_e7={} satellites={:?}",
                    location.valid,
                    location.latitude.degrees_e7,
                    location.longitude.degrees_e7,
                    location.satellites
                ));
                defmt::info!("AARDWOLF L4 health seconds={} closed={} gps={} pps={} anchors={} rejected={} cal_samples={} locked={} ppb={} gate={} backlog={} append_us={} log_bytes={}",heartbeat,streams::CLOSED.load(Relaxed),gps.operating_state,gps.num_pps_events,time.accepted_anchors,time.rejected_anchors,time.frequency_calibration_samples,time.frequency_calibration_locked,time.calibrated_frequency_error_ppb,time.pps_reacquisition_active,MAX_BACKLOG.load(Relaxed),streams::MAX_APPEND_US.load(Relaxed),logging.stats().bytes_written);
                stack_probe::check("running");
            }
            next_heartbeat += Duration::from_secs(1);
        }
        if logging.drain().await.is_err() {
            fault::park(fault::Fault::Storage).await;
        }
        if now >= next_flush {
            if logging.checkpoint().await.is_err() {
                fault::park(fault::Fault::Storage).await;
            }
            next_flush += Duration::from_secs(10);
        }
        if streams::STOP_LOG.load(Relaxed) {
            break;
        }
        Timer::after_millis(20).await;
    }
    if logging.drain().await.is_err() || logging.flush().await.is_err() {
        fault::park(fault::Fault::Storage).await;
    }
    let stats = logging.stats();
    defmt::assert!(
        stats.dropped_messages == 0
            && stats.truncated_messages == 0
            && stats.write_failures == 0
            && stats.queue_depth == 0
    );
    let syslog = logging.into_sink();
    if streams::STORAGE
        .lock()
        .await
        .as_mut()
        .unwrap()
        .finish(syslog.stream)
        .await
        .is_err()
    {
        fault::park(fault::Fault::Storage).await;
    }
    streams::LOG_OPEN.store(false, Relaxed);
    streams::closed();
    syslog
}
