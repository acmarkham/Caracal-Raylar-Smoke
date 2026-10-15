use super::{MICROPHONES, MIC_CONFIG};
use core::sync::atomic::{AtomicBool, Ordering};
use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
use raylar_audio_recorder_service::{
    AudioRecorderConfig, CaptureParameters, HexaphonicBuffers, HexaphonicConfig,
    HexaphonicRecorder, ReferenceFormat, TimeMetadataSource,
};
use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_board_v1p0::{Gps, SdCard};
use raylar_storage_service::{StorageLayout, StorageService};
use raylar_time_service::UtcTimestamp;
use static_cell::ConstStaticCell;

const SOURCE_CAPACITY: usize = if cfg!(any(
    feature = "stage-profiling",
    feature = "profile-control"
)) {
    127998
} else {
    96000
};
static AUDIO: AudioSource<SOURCE_CAPACITY, 1> =
    AudioSource::new(AudioFormat::new(16000, 6, 1000000));
static REF_PCM: ConstStaticCell<[i32; 2048]> = ConstStaticCell::new([0; 2048]);
static REF_OUTPUT: ConstStaticCell<[u8; 8224]> = ConstStaticCell::new([0; 8224]);
static FORWARDING: AtomicBool = AtomicBool::new(false);

#[cfg(feature = "laboratory-time")]
fn laboratory_seconds() -> i64 {
    // This is a named laboratory anchor, deliberately not a claim of GPS time.
    // Vary its folder/file names with each recorded firmware build to avoid collisions.
    let stamp = option_env!("RAYLAR_BUILD_TIMESTAMP")
        .unwrap_or("2026-10-14T00:00:00Z")
        .as_bytes();
    let pair = |i: usize| i64::from((stamp[i] - b'0') * 10 + stamp[i + 1] - b'0');
    1_792_000_000 + pair(11) * 3600 + pair(14) * 60 + pair(17)
}

pub async fn run(
    spawner: Spawner,
    sd: SdCard<'static>,
    gps: Gps<'static>,
    pcm: &'static mut [i32],
    out: &'static mut [u8],
) {
    MICROPHONES.set_enabled(false);
    let backend = super::common::storage_driver(sd).await;
    let mut storage = unwrap!(StorageService::<_, _, 512, 3, 16384>::new(
        backend,
        &super::common::TIME_RESOURCES
    ));
    unwrap!(storage.mount().await);
    #[cfg(feature = "laboratory-time")]
    super::common::start_fake_time(spawner, gps, laboratory_seconds()).await;
    #[cfg(not(feature = "laboratory-time"))]
    super::common::start_time(spawner, gps).await;
    let waiting = Instant::now();
    while super::common::TIME_RESOURCES.current_utc().is_err() {
        if waiting.elapsed() > Duration::from_secs(180) {
            info!("CRCL SD test blocked: GPS UTC unavailable after 180 seconds");
            return;
        }
        Timer::after_secs(1).await;
    }
    let now = unwrap!(super::common::TIME_RESOURCES.current_utc());
    let start = unwrap!(UtcTimestamp::new((now.seconds / 60 + 1) * 60, 0));
    let metadata = unwrap!(TimeMetadataSource::new(&super::common::TIME_RESOURCES)
        .with_boot_id(1)
        .with_device_id("CRCL-target-test"));
    let resolved = unwrap!(raylar_drivers::mic_array::stm32::resolve_config(MIC_CONFIG));
    let config = HexaphonicConfig {
        recorder: AudioRecorderConfig {
            recording_seconds: 60,
            storage_layout: StorageLayout::HourlyFolders,
        },
        spatial: if cfg!(feature = "cheap-crcl") {
            raylar_crcl_encoder_service::Config::cheap()
        } else {
            raylar_crcl_encoder_service::Config::default()
        },
        reference: ReferenceFormat::Flac(raylar_audio_recorder_service::FlacConfig {
            prediction: raylar_audio_recorder_service::FlacPrediction::Fixed(1),
            ..raylar_audio_recorder_service::FlacConfig::default()
        }),
        capture: CaptureParameters {
            pdm_clock_hz: resolved.microphone_clock_hz,
            sinc_order: MIC_CONFIG.sinc_filter.order(),
            decimation: resolved.decimation,
            cic_scale: MIC_CONFIG.cic_scale as u8,
            hpf_enabled: MIC_CONFIG.high_pass_filter,
            pcm_shift: 8,
            gain_tenths_db: MIC_CONFIG.cic_scale.gain_tenths_db(),
            origin_ticks: Some(|| MICROPHONES.state().started_at_ticks),
        },
        calibration_id: None,
    };
    let mut recorder = unwrap!(HexaphonicRecorder::new(
        &AUDIO,
        storage,
        metadata,
        config,
        HexaphonicBuffers {
            pcm,
            spatial_output: out,
            reference_pcm: REF_PCM.take(),
            reference_output: REF_OUTPUT.take()
        }
    ));
    spawner.spawn(unwrap!(forward_task()));
    MICROPHONES.set_enabled(true);
    FORWARDING.store(true, Ordering::Relaxed);
    let ticks = unwrap!(super::common::TIME_RESOURCES
        .time_state()
        .utc_to_system_holdover(start));
    Timer::at(ticks).await;
    unwrap!(recorder.start_at(start).await);
    info!(
        "CRCL SD recording started utc={} reference=flac sidecar=CRCL channels=6",
        start.seconds
    );
    let mut count = 0u64;
    let mut codec_max = 0;
    let mut write_max = 0;
    let mut backlog_max = 0;
    let mut reported = Instant::now();
    #[cfg(any(feature = "stage-profiling", feature = "profile-control"))]
    let mut window = 1;
    #[cfg(any(feature = "stage-profiling", feature = "profile-control"))]
    let mut window_frames = 0;
    #[cfg(any(feature = "stage-profiling", feature = "profile-control"))]
    let mut window_started = Instant::now();
    #[cfg(any(feature = "stage-profiling", feature = "profile-control"))]
    super::profiling::begin(cfg!(feature = "stage-profiling"));
    loop {
        #[cfg(not(any(feature = "stage-profiling", feature = "profile-control")))]
        let p = unwrap!(
            recorder
                .record_next_with_source_timeout(Duration::from_secs(2))
                .await
        );
        #[cfg(any(feature = "stage-profiling", feature = "profile-control"))]
        let p = unwrap!(
            raylar_stage_profiling::measure(
                raylar_stage_profiling::Stage::RecorderPoll,
                recorder.record_next_with_source_timeout(Duration::from_secs(2))
            )
            .await
        );
        count += p.pcm_samples as u64 / 6;
        #[cfg(any(feature = "stage-profiling", feature = "profile-control"))]
        {
            window_frames += p.pcm_samples as u64 / 6;
        }
        codec_max = codec_max.max(p.codec_elapsed_us);
        write_max = write_max.max(p.storage_elapsed_us);
        backlog_max = backlog_max.max(p.backlog_samples);
        assert_eq!(p.dropped_samples, 0);
        if reported.elapsed() >= Duration::from_secs(10) {
            info!(
                "CRCL SD progress frames={} codec_max_us={} write_max_us={} backlog={}",
                count, codec_max, write_max, backlog_max
            );
            reported = Instant::now();
        }
        if p.rotated {
            #[cfg(any(feature = "stage-profiling", feature = "profile-control"))]
            {
                super::profiling::report(
                    window,
                    window_frames,
                    window_started.elapsed().as_micros(),
                );
                if window == 1 && cfg!(feature = "stage-profiling") {
                    window = 2;
                    window_frames = 0;
                    window_started = Instant::now();
                    super::profiling::begin(false);
                    info!("PROFILE control started: detailed probes disabled; recording the next complete minute");
                    continue;
                }
            }
            break;
        }
    }
    unwrap!(recorder.stop().await);
    FORWARDING.store(false, Ordering::Relaxed);
    MICROPHONES.set_enabled(false);
    info!("CRCL SD TARGET PASS: finalized matching minute pair utc={} frames={} codec_max_us={} write_max_us={} backlog_max_scalar_samples={}",
        start.seconds,count,codec_max,write_max,backlog_max);
    #[cfg(feature = "stage-profiling")]
    info!(
        "CRCL PROFILE TARGET PASS: profiled and control minute pairs finalized without source loss"
    );
    #[cfg(feature = "profile-control")]
    info!("CRCL PROFILE CONTROL PASS: detailed probes compiled out; matching minute pair finalized without source loss");
}

#[embassy_executor::task]
async fn forward_task() -> ! {
    let mut receiver = unwrap!(MICROPHONES.frame_receiver());
    let mut previous = 0;
    loop {
        let state = receiver.changed().await;
        if !state.running || state.sequence == 0 {
            previous = 0;
            continue;
        }
        if !FORWARDING.load(Ordering::Relaxed) {
            previous = 0;
            continue;
        }
        assert!(state.error.is_none());
        assert!(previous == 0 || state.sequence == previous + 1);
        previous = state.sequence;
        let frame = MICROPHONES.frame(state);
        let channels = frame.active_channels();
        assert_eq!(channels.len(), 6);
        let ticks = state.started_at_ticks + (state.sequence - 1) * 100000;
        let _publish_profile =
            raylar_stage_profiling::Span::new(raylar_stage_profiling::Stage::CapturePublish);
        unwrap!(AUDIO.write_from_fn(1600 * 6, ticks, |i| (channels[i % 6][i / 6] as i32) >> 8));
    }
}
