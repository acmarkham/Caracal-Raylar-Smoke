use super::{MIC_CONFIG, MICROPHONES};
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
    feature = "profile-control",
    feature = "recovery-test"
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
    #[cfg(feature = "recovery-test")]
    let storage = SlowOpen {
        inner: storage,
        delay: true,
    };
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
    let start = unwrap!(UtcTimestamp::new(
        if cfg!(feature = "recovery-test") {
            now.seconds + 2
        } else {
            (now.seconds / 60 + 1) * 60
        },
        0
    ));
    let metadata = unwrap!(
        TimeMetadataSource::new(&super::common::TIME_RESOURCES)
            .with_boot_id(1)
            .with_device_id("CRCL-target-test")
    );
    let resolved = unwrap!(raylar_drivers::mic_array::stm32::resolve_config(MIC_CONFIG));
    let config = HexaphonicConfig {
        recorder: AudioRecorderConfig {
            recording_seconds: if cfg!(feature = "recovery-test") {
                5
            } else {
                60
            },
            storage_layout: StorageLayout::HourlyFolders,
        },
        spatial: if cfg!(feature = "cheap-crcl") {
            raylar_crcl_encoder_service::Config {
                min_shift: 6,
                ..raylar_crcl_encoder_service::Config::cheap()
            }
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
    let ticks = unwrap!(
        super::common::TIME_RESOURCES
            .time_state()
            .utc_to_system_holdover(start)
    );
    Timer::at(ticks).await;
    unwrap!(recorder.start_at(start).await);
    #[cfg(feature = "recovery-test")]
    {
        run_recovery(&mut recorder).await;
        FORWARDING.store(false, Ordering::Relaxed);
        MICROPHONES.set_enabled(false);
        return;
    }
    #[cfg(not(feature = "recovery-test"))]
    {
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
                        info!(
                            "PROFILE control started: detailed probes disabled; recording the next complete minute"
                        );
                        continue;
                    }
                }
                break;
            }
        }
        unwrap!(recorder.stop().await);
        FORWARDING.store(false, Ordering::Relaxed);
        MICROPHONES.set_enabled(false);
        info!(
            "CRCL SD TARGET PASS: finalized matching minute pair utc={} frames={} codec_max_us={} write_max_us={} backlog_max_scalar_samples={}",
            start.seconds, count, codec_max, write_max, backlog_max
        );
        #[cfg(feature = "stage-profiling")]
        info!(
            "CRCL PROFILE TARGET PASS: profiled and control minute pairs finalized without source loss"
        );
        #[cfg(feature = "profile-control")]
        info!(
            "CRCL PROFILE CONTROL PASS: detailed probes compiled out; matching minute pair finalized without source loss"
        );
    }
}

#[cfg(feature = "recovery-test")]
struct SlowOpen<S> {
    inner: S,
    delay: bool,
}
#[cfg(feature = "recovery-test")]
impl<S: raylar_audio_recorder_service::RecordingStorage>
    raylar_audio_recorder_service::RecordingStorage for SlowOpen<S>
{
    type Error = S::Error;
    type Handle = S::Handle;
    async fn admit_audio_bytes(&mut self, n: u64) -> Result<(), Self::Error> {
        self.inner.admit_audio_bytes(n).await
    }
    async fn begin_audio_stream(
        &mut self,
        layout: StorageLayout,
        utc: UtcTimestamp,
        suffix: &str,
    ) -> Result<Self::Handle, Self::Error> {
        let handle = self.inner.begin_audio_stream(layout, utc, suffix).await?;
        if self.delay {
            self.delay = false;
            info!("RECOVERY injecting 2s open stall, exceeding the 1.333s source retention");
            Timer::after_secs(2).await;
        }
        Ok(handle)
    }
    async fn append_audio(&mut self, h: Self::Handle, bytes: &[u8]) -> Result<(), Self::Error> {
        self.inner.append_audio(h, bytes).await
    }
    async fn rewrite_audio(
        &mut self,
        h: Self::Handle,
        at: u64,
        bytes: &[u8],
    ) -> Result<(), Self::Error> {
        self.inner.rewrite_audio(h, at, bytes).await
    }
    async fn finish_audio(&mut self, h: Self::Handle) -> Result<(), Self::Error> {
        self.inner.finish_audio(h).await
    }
}

#[cfg(feature = "recovery-test")]
async fn run_recovery<S, M>(recorder: &mut HexaphonicRecorder<'_, S, M, SOURCE_CAPACITY, 1>)
where
    S: raylar_audio_recorder_service::RecordingStorage,
    S::Error: defmt::Format,
    M: raylar_audio_recorder_service::MetadataSource,
{
    use raylar_audio_recorder_service::AudioRecorderError;
    for phase in 0..3 {
        if phase != 0 {
            info!(
                "RECOVERY phase={} pausing recorder 8s while capture keeps publishing",
                phase
            );
            Timer::after_secs(8).await;
            let now = unwrap!(super::common::TIME_RESOURCES.current_utc());
            let start = unwrap!(UtcTimestamp::new(now.seconds + 1, 0));
            let at = unwrap!(
                super::common::TIME_RESOURCES
                    .time_state()
                    .utc_to_system_holdover(start)
            );
            Timer::at(at).await;
            unwrap!(recorder.start_at(start).await);
        }
        let began = Instant::now();
        let mut frames = 0usize;
        loop {
            match recorder
                .record_next_with_source_timeout(Duration::from_secs(2))
                .await
            {
                Err(AudioRecorderError::Discontinuity { dropped_samples }) if phase == 0 => {
                    assert!(dropped_samples > 0);
                    info!(
                        "RECOVERY expected injected overrun dropped_scalars={}",
                        dropped_samples
                    );
                    break;
                }
                Ok(p) => {
                    frames += p.encoded_frames;
                    assert!(began.elapsed() < Duration::from_secs(8));
                    if p.rotated {
                        assert!(phase != 0);
                        assert!((78_400..=80_001).contains(&frames));
                        assert!(AUDIO.timestamp_error_ticks().unsigned_abs() < 1000);
                        info!(
                            "RECOVERY phase={} frames={} elapsed_us={} timestamp_error_ticks={}",
                            phase,
                            frames,
                            began.elapsed().as_micros(),
                            AUDIO.timestamp_error_ticks()
                        );
                        break;
                    }
                }
                other => {
                    let _ = unwrap!(other);
                }
            }
        }
        unwrap!(recorder.stop().await);
        let t = recorder.take_lifecycle_metrics();
        info!(
            "RECOVERY lifecycle open_us={} finish_us={} final_ref_us={} first_sample_ticks={} first_source_frame={} backlog_before={} backlog_after={}",
            t.open_max_us,
            t.finish_max_us,
            t.final_reference_max_us,
            t.first_sample_ticks,
            t.first_source_frame,
            t.backlog_before_max_samples,
            t.backlog_after_max_samples
        );
    }
    info!(
        "CRCL RECOVERY TARGET PASS: injected overrun, two recorder pauses, aligned UTC recovery, paired five-second recordings"
    );
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
