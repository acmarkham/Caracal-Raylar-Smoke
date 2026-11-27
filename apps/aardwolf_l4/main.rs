//! Aardwolf Evo L4: four synchronous microphones, paired FLAC/CRCL and shared time.
#![no_std]
#![no_main]
#[cfg(feature = "mono")]
mod audio;
#[cfg(not(feature = "mono"))]
#[path = "quad_audio.rs"]
mod audio;
mod diagnostics;
mod fault;
mod gnss;
mod gnss_diagnostics;
#[cfg(feature = "local-time")]
mod local_time;
mod quad;
#[cfg(all(not(feature = "mono"), feature = "bench-codec"))]
mod quad_fixture;
mod setup;
mod streams;
mod ui;
#[cfg(feature = "mono")]
mod verify;
#[cfg(not(feature = "mono"))]
#[path = "quad_verify.rs"]
mod verify;
use aardwolf_l4::*;
use caracal_evo::{capture, dfsdm, sd, stack_probe};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering::Relaxed};
use defmt_rtt as _;
use embassy_executor::Spawner;
#[cfg(not(feature = "local-time"))]
use embassy_stm32::usart::{self, BufferedUart};
use embassy_stm32::{
    gpio::{Input, Level, Output, Pull, Speed},
    pac,
    rcc::{self, *},
    sdmmc::{Config as SdConfig, Sdmmc},
    time::Hertz,
};
use embassy_time::{Duration, Instant, Timer};
use raylar_audio_recorder_service::AudioRecorderError;
#[cfg(feature = "mono")]
use raylar_audio_recorder_service::{AudioRecorder, AudioRecorderConfig};
use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_drivers::gps::GpsResources;
use raylar_location_service::LocationResources;
use raylar_logging_service::{LoggingResources, LoggingService};
#[cfg(feature = "mono")]
use raylar_storage_service::StorageLayout;
use raylar_time_service::{TimeResources, UtcTimestamp};
#[cfg(any(feature = "mono", not(feature = "local-time")))]
use static_cell::ConstStaticCell;
use static_cell::StaticCell;

const AUDIO_CAPACITY: usize = if cfg!(feature = "mono") {
    4096
} else if cfg!(all(feature = "local-time", not(feature = "bench-codec"))) {
    6144
} else {
    5120
};
const STREAM_COUNT: usize = if cfg!(feature = "mono") { 2 } else { 3 };
static AUDIO: AudioSource<AUDIO_CAPACITY, 1> =
    AudioSource::new(AudioFormat::new(SAMPLE_RATE, CHANNELS as u8, 1_000_000));
static TIME: TimeResources<2, 2> = TimeResources::new();
static GPS: GpsResources<128, 4, 8, 2> = GpsResources::new();
static LOCATION: LocationResources<2> = LocationResources::new();
static LOGGING: LoggingResources<192, 8> = LoggingResources::new();
static SDMMC: StaticCell<Sdmmc<'static>> = StaticCell::new();
#[cfg(not(feature = "local-time"))]
static UART_RX: ConstStaticCell<[u8; 2048]> = ConstStaticCell::new([0; 2048]);
#[cfg(not(feature = "local-time"))]
static UART_TX: ConstStaticCell<[u8; 128]> = ConstStaticCell::new([0; 128]);
static CONSUMED_SAMPLES: AtomicU32 = AtomicU32::new(0);
static MAX_CODEC_US: AtomicU32 = AtomicU32::new(0);
#[cfg(all(feature = "mono", not(feature = "wav")))]
static ENCODED: ConstStaticCell<[u8; FLAC_ENCODE_BYTES]> =
    ConstStaticCell::new([0; FLAC_ENCODE_BYTES]);
#[cfg(all(feature = "mono", not(feature = "wav")))]
static PCM: ConstStaticCell<[i32; FLAC_BLOCK]> = ConstStaticCell::new([0; FLAC_BLOCK]);
static MAX_BACKLOG: AtomicU32 = AtomicU32::new(0);
static RETENTION_SILENCE_FRAMES: AtomicU32 = AtomicU32::new(0);
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
        embedded_alloc::init!(HEAP, 3072);
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
    // Bulk SD traffic must not preempt the microphone drain executor (P1).
    // DMA1 audio completion and the system timer retain their P0 priority.
    {
        use embassy_stm32::interrupt::{InterruptExt, Priority};
        embassy_stm32::interrupt::SDMMC1.set_priority(Priority::P2);
        embassy_stm32::interrupt::DMA2_CHANNEL5.set_priority(Priority::P2);
    }
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
    // A fresh hardware random boot ID binds companion files across resets.
    streams::BOOT_ID.store(quad::boot_id(p.RNG), Relaxed);
    #[cfg(not(feature = "local-time"))]
    {
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
            gnss::driver_task(gnss::Serial::new(uart), gnss::Pps { _chain: chain }, enable)
                .unwrap(),
        );
        GPS.command_sender()
            .send(raylar_drivers::gps::GpsCommand::Start)
            .await;
    }
    #[cfg(feature = "local-time")]
    {
        let _ = enable;
        spawner.spawn(gnss::time_task().unwrap());
        spawner.spawn(local_time::task().unwrap());
    }
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
        "startup target=L4Evo mode={} firmware={} git={} build_unix={}",
        if cfg!(feature = "local-time") {
            "LOCAL-TIME-BENCH"
        } else {
            "REAL-GPS"
        },
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
    streams::log(format_args!("identity uid={} sd_serial={:08x} gps=ublox_autodetect radio=absent battery=absent button=absent",embassy_stm32::uid::uid_hex(),serial));
    let version = gnss::VERSION.lock(|s| s.borrow().clone());
    streams::log(format_args!(
        "gps_version {}",
        if version.is_empty() {
            "Unavailable (discovery pending)"
        } else {
            version.as_str()
        }
    ));
    streams::log(format_args!("policy channels={} rate=16000 reference={} bits=24 flac_block={} crcl_block={} crcl_bits=10 min_shift=6 segment_s=60",CHANNELS,CODEC,FLAC_BLOCK,SPATIAL_BLOCK));
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
    #[cfg(all(feature="mono",not(feature = "wav")))]
    let mut recorder = AudioRecorder::<_, _, AUDIO_CAPACITY, 1, FLAC_ENCODE_BYTES, FLAC_BLOCK>::new_flac_with_buffers(
        &AUDIO, streams::Recording::new(), streams::metadata(),
        AudioRecorderConfig { recording_seconds: SEGMENT_SECONDS, storage_layout: StorageLayout::HourlyFolders },
        raylar_audio_recorder_service::FlacConfig { sample_rate_hz: SAMPLE_RATE, bits_per_sample: SOURCE_BITS,
            block_samples: FLAC_BLOCK, prediction: raylar_audio_recorder_service::FlacPrediction::Fixed(1) },
        ENCODED.take(), PCM.take(),
    ).unwrap();
    #[cfg(not(feature = "mono"))]
    let mut recorder = quad::recorder();
    spawner.spawn(diagnostics::cpu_task().unwrap());
    #[cfg(feature = "mono")]
    spawner.spawn(audio::capture(p.PC2, p.PB12, p.DMA1_CH4).unwrap());
    #[cfg(not(feature = "mono"))]
    audio::spawner().spawn(
        audio::capture(
            p.PC2, p.PB12, p.PC7, p.PB6, p.PB10, p.DMA1_CH4, p.DMA1_CH5, p.DMA1_CH6, p.DMA1_CH7,
        )
        .unwrap(),
    );
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
    defmt::assert!(
        streams::OPEN.load(Relaxed) == 0 && streams::MAX_OPEN.load(Relaxed) == STREAM_COUNT as u32
    );
    defmt::assert!(audio::DMA_ERRORS.load(Relaxed) == 0 && capture::FAULTS.load(Relaxed) == 0);
    #[cfg(all(feature = "bench-glitches", not(feature = "mono")))]
    defmt::assert!(
        audio::DMA_RECOVERIES.load(Relaxed) > 0
            && audio::DMA_SILENCE_FRAMES.load(Relaxed) > 0
            && RETENTION_SILENCE_FRAMES.load(Relaxed) > 0
    );
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
#[cfg(all(feature = "mono", not(feature = "wav")))]
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
#[cfg(not(feature = "mono"))]
type Recorder = quad::Recorder;
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
    // Prepare the filesystem/cache before any PCM is published. Paired headers
    // retain the actual first source-frame index and timestamp on first read.
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
    let mut timeline_frames = 0u64;
    let mut gap_start = 0u64;
    let mut gap_frames = 0u64;
    loop {
        match recorder
            .record_next_with_source_timeout(Duration::from_secs(3))
            .await
        {
            Ok(progress) => {
                #[cfg(feature = "mono")]
                defmt::assert!(progress.dropped_samples == 0, "AARDWOLF L4 dropped samples");
                RETENTION_SILENCE_FRAMES
                    .fetch_add((progress.dropped_samples / CHANNELS as u64) as u32, Relaxed);
                if progress.dropped_samples != 0 {
                    if gap_frames == 0 {
                        gap_start = timeline_frames;
                    }
                    gap_frames += progress.dropped_samples / CHANNELS as u64;
                }
                if gap_frames != 0
                    && (progress.rotated
                        || (progress.dropped_samples == 0 && progress.pcm_samples != 0))
                {
                    streams::log(format_args!("audio_gap reason=retention first_source_frame={} frames={} fill=zero_all_channels",gap_start,gap_frames));
                    gap_frames = 0;
                }
                timeline_frames += (progress.pcm_samples / CHANNELS) as u64;
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
        "bench_complete segments={} retention_silence_frames={} local_time={}",
        streams::CLOSED.load(Relaxed),
        RETENTION_SILENCE_FRAMES.load(Relaxed),
        cfg!(feature = "local-time")
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
                    "gps_identity_verified count={} {} detected_baud={} uart_profile_verified={}",
                    versions,
                    identity.as_str(),
                    gnss::DETECTED_BAUD.load(Relaxed),
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
                #[cfg(not(feature = "mono"))]
                streams::log(format_args!("audio_recovery dma_rejoins={} dma_silence_frames={} retention_silence_frames={} policy=silence_all_channels",
                    audio::DMA_RECOVERIES.load(Relaxed),audio::DMA_SILENCE_FRAMES.load(Relaxed),RETENTION_SILENCE_FRAMES.load(Relaxed)));
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
