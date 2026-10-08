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
use raylar_audio_recorder_service::{
    AudioRecorder, AudioRecorderConfig, AudioRecorderError, TimeMetadataSource,
};
use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_board_v1p0::{PdmMicArray, PdmMicDma};
use raylar_drivers::mic_array::stm32::{MonoPins, Stm32MonoMicrophoneDriver};
use raylar_drivers::mic_array::{
    MicrophoneConfig, MicrophoneMode, MicrophonePreset, MicrophoneResources,
};
use raylar_drivers::storage::StorageError;
use raylar_logging_service::{info as log_info, LoggerHandle};
use raylar_storage_service::{StorageLayout, StorageServiceError};
use raylar_time_service::UtcTimestamp;

const HALF_SAMPLES: usize = 1_600;
const DMA_SAMPLES: usize = HALF_SAMPLES * 2;
pub const CAPACITY: usize = 16_000 * 8;
const MIC_CONFIG: MicrophoneConfig = MicrophoneConfig {
    mode: MicrophoneMode::Mono,
    ..MicrophoneConfig::from_preset(MicrophonePreset::ReferenceSinc5_16KhzHiperf)
};
pub static MICROPHONES: MicrophoneResources<DMA_SAMPLES> = MicrophoneResources::new();
static AUDIO: AudioSource<CAPACITY, 2> = AudioSource::new(AudioFormat::new(16_000, 1, 1_000_000));
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

#[embassy_executor::task]
async fn capture_task(driver: Stm32MonoMicrophoneDriver<'static, DMA_SAMPLES>) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn forward_task(log: Log) -> ! {
    let mut frames = unwrap!(MICROPHONES.frame_receiver());
    let mut previous = 0u64;
    loop {
        let state = frames.changed().await;
        if !state.running || state.sequence == 0 {
            previous = 0;
            continue;
        }
        if previous != 0 && state.sequence != previous + 1 {
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
            AUDIO_LOSSES.fetch_add(1, Ordering::Relaxed);
            continue;
        }
        if !RECORDING.load(Ordering::Relaxed) {
            continue;
        }
        let frame = MICROPHONES.frame(state);
        let channel = frame.active_channels()[0];
        if AUDIO
            .write_from_fn(channel.len(), state.completed_at_ticks, |index| {
                (channel[index] as i32) >> 8
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

#[embassy_executor::task]
async fn recorder_task(storage: &'static SharedStorage, node: u32, boot: u16, log: Log) -> ! {
    let metadata = TimeMetadataSource::new(&common::TIME_RESOURCES)
        .with_node_id(node)
        .with_boot_id(u32::from(boot))
        .with_location_source(location_e7);
    let mut recorder = unwrap!(AudioRecorder::<_, _, CAPACITY, 2>::new(
        &AUDIO,
        RecorderStorage { shared: storage },
        metadata,
        AudioRecorderConfig {
            recording_seconds: 60,
            storage_layout: StorageLayout::HourlyFolders
        }
    ));
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
        loop {
            match recorder
                .record_next_with_source_timeout(Duration::from_secs(2))
                .await
            {
                Ok(progress) => {
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
