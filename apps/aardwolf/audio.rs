use crate::{
    common, policy,
    power::ACTIVITY,
    storage::{RecorderStorage, SharedStorage, STORAGE_FLAGS},
    LOCATION,
};
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use defmt::{error, unwrap};
use embassy_executor::Spawner;
use embassy_time::{Duration, Instant, Timer};
#[cfg(not(feature = "hexaphonic"))]
use raylar_audio_recorder_service::AudioRecorder;
#[cfg(feature = "hexaphonic")]
use raylar_audio_recorder_service::{
    crcl, CaptureParameters, HexaphonicBuffers, HexaphonicConfig, HexaphonicRecorder,
    ReferenceFormat,
};
use raylar_audio_recorder_service::{
    flac_frame_capacity, AudioRecorderConfig, AudioRecorderError, TimeMetadataSource,
};
use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_board_v1p0::{PdmMicArray, PdmMicDma};
#[cfg(feature = "hexaphonic")]
use raylar_drivers::mic_array::stm32::{DmaChannels, Pins, Stm32MicrophoneDriver};
#[cfg(not(feature = "hexaphonic"))]
use raylar_drivers::mic_array::stm32::{MonoPins, Stm32MonoMicrophoneDriver};
use raylar_drivers::mic_array::{
    MicrophoneConfig, MicrophoneMode, MicrophonePreset, MicrophoneResources,
};
use raylar_drivers::storage::StorageError;
use raylar_logging_service::{info as log_info, LoggerHandle};
use raylar_storage_service::{StorageLayout, StorageServiceError};
use raylar_time_service::UtcTimestamp;
use static_cell::ConstStaticCell;

const HALF_SAMPLES: usize = 1_600;
const DMA_SAMPLES: usize = HALF_SAMPLES * 2;
const FLAC_SAMPLES: usize = crate::config::FLAC_BLOCK_SAMPLES;
const FLAC_BYTES: usize = flac_frame_capacity(FLAC_SAMPLES);
static FLAC_PCM: ConstStaticCell<[i32; FLAC_SAMPLES]> = ConstStaticCell::new([0; FLAC_SAMPLES]);
static FLAC_FRAME: ConstStaticCell<[u8; FLAC_BYTES]> = ConstStaticCell::new([0; FLAC_BYTES]);
pub const CAPACITY: usize = if cfg!(feature = "hexaphonic") {
    127_998
} else {
    16_000 * 8
};
#[cfg(feature = "hexaphonic")]
static SPATIAL_PCM: ConstStaticCell<[i32; crate::config::CRCL_BLOCK_FRAMES * 6]> =
    ConstStaticCell::new([0; crate::config::CRCL_BLOCK_FRAMES * 6]);
#[cfg(feature = "hexaphonic")]
static SPATIAL_OUTPUT: ConstStaticCell<
    [u8; crcl::workspace_capacity(crate::config::CRCL_BLOCK_FRAMES, crate::config::CRCL_BITS)],
> = ConstStaticCell::new(
    [0; crcl::workspace_capacity(crate::config::CRCL_BLOCK_FRAMES, crate::config::CRCL_BITS)],
);
const MIC_CONFIG: MicrophoneConfig = MicrophoneConfig {
    mode: if cfg!(feature = "hexaphonic") {
        MicrophoneMode::Hexaphonic
    } else {
        MicrophoneMode::Mono
    },
    ..MicrophoneConfig::from_preset(MicrophonePreset::ReferenceSinc5_16KhzHiperf)
};
pub static MICROPHONES: MicrophoneResources<DMA_SAMPLES> = MicrophoneResources::new();
static AUDIO: AudioSource<CAPACITY, 2> = AudioSource::new(AudioFormat::new(
    crate::config::FLAC_CONFIG.sample_rate_hz,
    crate::config::AUDIO_CHANNELS,
    1_000_000,
));
static RECORDING: AtomicBool = AtomicBool::new(false);
pub fn recording_active() -> bool {
    RECORDING.load(Ordering::Relaxed)
}
pub static AUDIO_LOSSES: AtomicU32 = AtomicU32::new(0);
pub static AUDIO_PACKETS: AtomicU32 = AtomicU32::new(0);
pub fn capture_allowed() -> bool {
    STORAGE_FLAGS.load(Ordering::Relaxed)
        & (raylar_radio_service::heartbeat_v4::STORAGE_FULL
            | raylar_radio_service::heartbeat_v4::STORAGE_UNAVAILABLE
            | raylar_radio_service::heartbeat_v4::AUDIO_FAULT)
        == 0
}
type Log = LoggerHandle<'static, 384, 32>;

fn storage_is_full<E>(error: &AudioRecorderError<StorageServiceError<StorageError<E>>>) -> bool {
    matches!(
        error,
        AudioRecorderError::Storage(StorageServiceError::OutOfSpace)
            | AudioRecorderError::Storage(StorageServiceError::Backend(StorageError::OutOfSpace))
    )
}

#[cfg(not(feature = "hexaphonic"))]
pub fn start(
    spawner: Spawner,
    mic: PdmMicArray<'static>,
    storage: &'static SharedStorage,
    node: u32,
    boot: u16,
    log: Log,
) {
    let PdmMicArray {
        cck0,
        sd0,
        dma: PdmMicDma { ch0, .. },
        ..
    } = mic;
    let driver = unwrap!(Stm32MonoMicrophoneDriver::new(
        MonoPins { cck0, sd0 },
        embassy_stm32::dma::Channel::new(ch0, crate::MicIrqs),
        &MICROPHONES,
        MIC_CONFIG
    ));
    MICROPHONES.set_enabled(ACTIVITY.try_get().unwrap_or_default().active() && capture_allowed());
    spawner.spawn(unwrap!(capture_task(driver)));
    spawner.spawn(unwrap!(forward_task(log)));
    spawner.spawn(unwrap!(recorder_task(storage, node, boot, log)));
}

#[cfg(feature = "hexaphonic")]
pub fn start(
    spawner: Spawner,
    mic: PdmMicArray<'static>,
    storage: &'static SharedStorage,
    node: u32,
    boot: u16,
    log: Log,
) {
    let PdmMicArray {
        cck0,
        sd0,
        cck1,
        sd1,
        sd2,
        sd3,
        dma:
            PdmMicDma {
                ch0,
                ch1,
                ch2,
                ch3,
                ch4,
                ch5,
            },
        ..
    } = mic;
    let driver = unwrap!(Stm32MicrophoneDriver::new(
        Pins {
            cck0,
            sd0,
            cck1,
            sd1,
            sd2,
            sd3
        },
        DmaChannels {
            ch0: embassy_stm32::dma::Channel::new(ch0, crate::MicIrqs),
            ch1: embassy_stm32::dma::Channel::new(ch1, crate::MicIrqs),
            ch2: embassy_stm32::dma::Channel::new(ch2, crate::MicIrqs),
            ch3: embassy_stm32::dma::Channel::new(ch3, crate::MicIrqs),
            ch4: embassy_stm32::dma::Channel::new(ch4, crate::MicIrqs),
            ch5: embassy_stm32::dma::Channel::new(ch5, crate::MicIrqs),
        },
        &MICROPHONES,
        MIC_CONFIG
    ));
    MICROPHONES.set_enabled(ACTIVITY.try_get().unwrap_or_default().active() && capture_allowed());
    spawner.spawn(unwrap!(capture_array_task(driver)));
    spawner.spawn(unwrap!(forward_task(log)));
    spawner.spawn(unwrap!(recorder_task(storage, node, boot, log)));
}

#[cfg(feature = "hexaphonic")]
#[embassy_executor::task]
async fn capture_array_task(driver: Stm32MicrophoneDriver<'static, DMA_SAMPLES>) -> ! {
    driver.run().await
}

#[cfg(not(feature = "hexaphonic"))]
#[embassy_executor::task]
async fn capture_task(driver: Stm32MonoMicrophoneDriver<'static, DMA_SAMPLES>) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn forward_task(log: Log) -> ! {
    let mut frames = unwrap!(MICROPHONES.frame_receiver());
    let mut previous = 0u64;
    let mut previous_origin = 0u64;
    loop {
        let state = frames.changed().await;
        if state.running && state.started_at_ticks != 0 {
            if previous_origin != 0
                && previous_origin != state.started_at_ticks
                && RECORDING.load(Ordering::Relaxed)
            {
                AUDIO.report_discontinuity(
                    HALF_SAMPLES as u64 * u64::from(crate::config::AUDIO_CHANNELS),
                );
                AUDIO_LOSSES.fetch_add(1, Ordering::Relaxed);
                let _ = log_info!(
                    log,
                    "capture restarted; missing sample count is a lower bound"
                );
            }
            previous_origin = state.started_at_ticks;
        }
        if !state.running || state.sequence == 0 {
            previous = 0;
            continue;
        }
        if previous != 0 && state.sequence > previous + 1 {
            if RECORDING.load(Ordering::Relaxed) {
                AUDIO.report_discontinuity((state.sequence - previous - 1).saturating_mul(
                    HALF_SAMPLES as u64 * u64::from(crate::config::AUDIO_CHANNELS),
                ));
            }
            AUDIO_LOSSES.fetch_add(
                (state.sequence - previous - 1).min(u32::MAX as u64) as u32,
                Ordering::Relaxed,
            );
            let _ = log_info!(
                log,
                "microphone gap previous={} current={}",
                previous,
                state.sequence
            );
        }
        previous = state.sequence;
        if state.error.is_some() {
            if RECORDING.load(Ordering::Relaxed) {
                AUDIO.report_discontinuity(
                    HALF_SAMPLES as u64 * u64::from(crate::config::AUDIO_CHANNELS),
                );
            }
            AUDIO_LOSSES.fetch_add(1, Ordering::Relaxed);
            continue;
        }
        if !RECORDING.load(Ordering::Relaxed) {
            continue;
        }
        let frame = MICROPHONES.frame(state);
        let channels = frame.active_channels();
        if channels.len() != crate::config::AUDIO_CHANNELS as usize {
            AUDIO.report_discontinuity(
                HALF_SAMPLES as u64 * u64::from(crate::config::AUDIO_CHANNELS),
            );
            AUDIO_LOSSES.fetch_add(1, Ordering::Relaxed);
            continue;
        }
        let channel_count = channels.len();
        // Hexaphonic timestamps refer to the first acquired sample, not the DMA completion.
        let ticks = if cfg!(feature = "hexaphonic") {
            state.started_at_ticks + (state.sequence - 1) * HALF_SAMPLES as u64 * 1_000_000 / 16_000
        } else {
            state.completed_at_ticks
        };
        if AUDIO
            .write_from_fn(channels[0].len() * channel_count, ticks, |index| {
                (channels[index % channel_count][index / channel_count] as i32) >> 8
            })
            .is_err()
        {
            AUDIO_LOSSES.fetch_add(1, Ordering::Relaxed);
        } else {
            AUDIO_PACKETS.fetch_add(1, Ordering::Relaxed);
        }
    }
}

fn location_e7() -> Option<(i32, i32)> {
    let state = LOCATION.state();
    state
        .valid
        .then_some((state.latitude.degrees_e7, state.longitude.degrees_e7))
}

#[cfg(feature = "hexaphonic")]
fn hex_recorder(
    storage: &'static SharedStorage,
    node: u32,
    metadata: TimeMetadataSource<'static, 4, 8>,
) -> HexaphonicRecorder<'static, RecorderStorage, TimeMetadataSource<'static, 4, 8>, CAPACITY, 2> {
    use core::fmt::Write;
    let mut device = heapless::String::<32>::new();
    unwrap!(write!(device, "{node:08x}"));
    let metadata = unwrap!(metadata.with_device_id(&device));
    let resolved = unwrap!(raylar_drivers::mic_array::stm32::resolve_config(MIC_CONFIG));
    unwrap!(HexaphonicRecorder::new(
        &AUDIO,
        RecorderStorage { shared: storage },
        metadata,
        HexaphonicConfig {
            recorder: AudioRecorderConfig {
                recording_seconds: 60,
                storage_layout: StorageLayout::HourlyFolders
            },
            spatial: crcl::Config {
                block_frames: crate::config::CRCL_BLOCK_FRAMES,
                bits: crate::config::CRCL_BITS,
                prediction: crcl::Prediction::DirectRice,
                max_abs_pcm: 1 << 23
            },
            reference: ReferenceFormat::Flac(crate::config::FLAC_CONFIG),
            capture: CaptureParameters {
                pdm_clock_hz: resolved.microphone_clock_hz,
                sinc_order: MIC_CONFIG.sinc_filter.order(),
                decimation: resolved.decimation,
                cic_scale: MIC_CONFIG.cic_scale as u8,
                hpf_enabled: MIC_CONFIG.high_pass_filter,
                pcm_shift: 8,
                gain_tenths_db: MIC_CONFIG.cic_scale.gain_tenths_db(),
                origin_ticks: Some(|| MICROPHONES.state().started_at_ticks)
            },
            calibration_id: None,
        },
        HexaphonicBuffers {
            pcm: SPATIAL_PCM.take(),
            spatial_output: SPATIAL_OUTPUT.take(),
            reference_pcm: FLAC_PCM.take(),
            reference_output: FLAC_FRAME.take()
        }
    ))
}

#[embassy_executor::task]
async fn recorder_task(storage: &'static SharedStorage, node: u32, boot: u16, log: Log) -> ! {
    let metadata = TimeMetadataSource::new(&common::TIME_RESOURCES)
        .with_node_id(node)
        .with_boot_id(u32::from(boot))
        .with_location_source(location_e7);
    #[cfg(not(feature = "hexaphonic"))]
    let mut recorder = unwrap!(
        AudioRecorder::<_, _, CAPACITY, 2, FLAC_BYTES, FLAC_SAMPLES>::new_flac_with_buffers(
            &AUDIO,
            RecorderStorage { shared: storage },
            metadata,
            AudioRecorderConfig {
                recording_seconds: 60,
                storage_layout: StorageLayout::HourlyFolders
            },
            crate::config::FLAC_CONFIG,
            FLAC_FRAME.take(),
            FLAC_PCM.take(),
        )
    );
    #[cfg(feature = "hexaphonic")]
    let mut recorder = hex_recorder(storage, node, metadata);
    let mut activity = unwrap!(ACTIVITY.receiver());
    loop {
        if !ACTIVITY.try_get().unwrap_or_default().active() {
            RECORDING.store(false, Ordering::Relaxed);
            activity.changed().await;
            continue;
        }
        if !capture_allowed() {
            MICROPHONES.set_enabled(false);
            Timer::after_secs(30).await;
            continue;
        }
        let time = common::TIME_RESOURCES.time_state();
        let Ok(now) = time.system_to_utc_holdover(Instant::now()) else {
            Timer::after_secs(1).await;
            continue;
        };
        let next = policy::next_minute(now.as_micros());
        let Ok(start) = time.utc_to_system_holdover(UtcTimestamp::from_micros(next)) else {
            Timer::after_secs(1).await;
            continue;
        };
        Timer::at(start).await;
        if !ACTIVITY.try_get().unwrap_or_default().active() {
            continue;
        }
        RECORDING.store(true, Ordering::Relaxed);
        match recorder.start_at(UtcTimestamp::from_micros(next)).await {
            Ok(()) => {
                let _ = log_info!(
                    log,
                    "audio started utc_us={} ticks={}",
                    next,
                    start.as_ticks()
                );
            }
            Err(error) => {
                RECORDING.store(false, Ordering::Relaxed);
                if !ACTIVITY.try_get().unwrap_or_default().active() {
                    MICROPHONES.set_enabled(false);
                }
                if !storage_is_full(&error) {
                    STORAGE_FLAGS.fetch_or(
                        raylar_radio_service::heartbeat_v4::AUDIO_FAULT,
                        Ordering::Relaxed,
                    );
                }
                let _ = log_info!(log, "audio start failed utc_us={} error={:?}", next, error);
                if !capture_allowed() {
                    MICROPHONES.set_enabled(false);
                }
                Timer::after_secs(1).await;
                continue;
            }
        }
        let mut metrics_started = Instant::now();
        let mut codec_max_us = 0;
        let mut write_max_us = 0;
        let mut backlog_max_samples = 0;
        loop {
            match recorder
                .record_next_with_source_timeout(Duration::from_secs(2))
                .await
            {
                Ok(progress) => {
                    codec_max_us = codec_max_us.max(progress.codec_elapsed_us);
                    write_max_us = write_max_us.max(progress.storage_elapsed_us);
                    backlog_max_samples = backlog_max_samples.max(progress.backlog_samples);
                    if metrics_started.elapsed() >= Duration::from_secs(10) {
                        let _ = log_info!(
                            log,
                            "audio timing codec_max_us={} write_max_us={} frame_budget_us={} backlog_max_samples={} source_capacity_samples={}",
                            codec_max_us,
                            write_max_us,
                            if cfg!(feature = "hexaphonic") { crate::config::CRCL_BLOCK_FRAMES as u64 } else { FLAC_SAMPLES as u64 } * 1_000_000 / 16_000,
                            backlog_max_samples,
                            CAPACITY
                        );
                        metrics_started = Instant::now();
                        codec_max_us = 0;
                        write_max_us = 0;
                        backlog_max_samples = 0;
                    }
                    if progress.dropped_samples != 0 {
                        AUDIO_LOSSES.fetch_add(
                            progress.dropped_samples.min(u32::MAX as u64) as u32,
                            Ordering::Relaxed,
                        );
                    }
                    if progress.rotated {
                        let _ = log_info!(
                            log,
                            "audio rotated samples={} dropped={}",
                            progress.pcm_samples,
                            progress.dropped_samples
                        );
                    }
                }
                Err(error) => {
                    if let AudioRecorderError::Discontinuity { dropped_samples } = error {
                        AUDIO_LOSSES.fetch_add(
                            dropped_samples.max(1).min(u32::MAX as u64) as u32,
                            Ordering::Relaxed,
                        );
                        let _ = log_info!(
                            log,
                            "audio discontinuity dropped={}; closed FLAC prefix",
                            dropped_samples
                        );
                        break;
                    }
                    let recovering = !ACTIVITY.try_get().unwrap_or_default().active();
                    if recovering && matches!(error, AudioRecorderError::SourceTimeout) {
                        let _ = log_info!(log, "audio source wait ended during energy recovery");
                    } else {
                        let _ = log_info!(log, "audio write failed error={:?}", error);
                    }
                    if !storage_is_full(&error)
                        && !(recovering && matches!(error, AudioRecorderError::SourceTimeout))
                    {
                        AUDIO_LOSSES.fetch_add(1, Ordering::Relaxed);
                        STORAGE_FLAGS.fetch_or(
                            raylar_radio_service::heartbeat_v4::AUDIO_FAULT,
                            Ordering::Relaxed,
                        );
                    }
                    break;
                }
            }
            if !ACTIVITY.try_get().unwrap_or_default().active() {
                break;
            }
        }
        RECORDING.store(false, Ordering::Relaxed);
        if let Err(error) = recorder.stop().await {
            error!("audio close failed: {:?}", error);
            STORAGE_FLAGS.fetch_or(
                raylar_radio_service::heartbeat_v4::AUDIO_FAULT,
                Ordering::Relaxed,
            );
        }
        if !ACTIVITY.try_get().unwrap_or_default().active() || !capture_allowed() {
            MICROPHONES.set_enabled(false);
        }
        let _ = log_info!(
            log,
            "audio stopped losses={}",
            AUDIO_LOSSES.load(Ordering::Relaxed)
        );
    }
}
