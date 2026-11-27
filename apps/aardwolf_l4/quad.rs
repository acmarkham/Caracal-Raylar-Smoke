//! Bounded four-channel composition of the upstream paired recorder.
use crate::{streams, AUDIO, AUDIO_CAPACITY};
use aardwolf_l4::*;
use raylar_audio_recorder_service::{
    AudioRecorderConfig, CaptureParameters, ChannelRecorder, FlacConfig, FlacPrediction,
    HexaphonicBuffers, HexaphonicConfig, ReferenceFormat,
};
use static_cell::ConstStaticCell;
static PCM: ConstStaticCell<[i32; SPATIAL_BLOCK * 4]> =
    ConstStaticCell::new([0; SPATIAL_BLOCK * 4]);
static SPATIAL: ConstStaticCell<[u8; SPATIAL_ENCODE_BYTES]> =
    ConstStaticCell::new([0; SPATIAL_ENCODE_BYTES]);
static REFERENCE_PCM: ConstStaticCell<[i32; FLAC_BLOCK]> = ConstStaticCell::new([0; FLAC_BLOCK]);
static REFERENCE: ConstStaticCell<[u8; FLAC_ENCODE_BYTES]> =
    ConstStaticCell::new([0; FLAC_ENCODE_BYTES]);
pub type Recorder = ChannelRecorder<
    'static,
    streams::Recording,
    raylar_audio_recorder_service::TimeMetadataSource<'static, 2, 2>,
    AUDIO_CAPACITY,
    1,
    4,
>;
pub fn recorder() -> Recorder {
    let mut recorder = Recorder::new(
        &AUDIO,
        streams::Recording::new(),
        streams::metadata(),
        HexaphonicConfig {
            recorder: AudioRecorderConfig {
                recording_seconds: SEGMENT_SECONDS,
                storage_layout: raylar_storage_service::StorageLayout::HourlyFolders,
            },
            spatial: raylar_crcl_encoder_service::Config {
                block_frames: SPATIAL_BLOCK,
                min_shift: 6,
                ..raylar_crcl_encoder_service::Config::cheap()
            },
            reference: ReferenceFormat::Flac(FlacConfig {
                sample_rate_hz: SAMPLE_RATE,
                bits_per_sample: 24,
                block_samples: FLAC_BLOCK,
                prediction: FlacPrediction::Fixed(1),
            }),
            capture: CaptureParameters {
                pdm_clock_hz: 3_200_000,
                sinc_order: 4,
                decimation: 200,
                cic_scale: 8,
                hpf_enabled: false,
                pcm_shift: 0,
                gain_tenths_db: 0,
                origin_ticks: None,
            },
            calibration_id: Some("Evo:PB12,PC7,PB6,PB10;rising"),
        },
        HexaphonicBuffers {
            pcm: PCM.take(),
            spatial_output: SPATIAL.take(),
            reference_pcm: REFERENCE_PCM.take(),
            reference_output: REFERENCE.take(),
        },
    )
    .unwrap();
    recorder.set_silence_on_overrun(true).unwrap();
    recorder
}

/// RNG is supplied by the existing 48 MHz PLLSAI1 clock. Poll only at boot.
pub fn boot_id(_rng: embassy_stm32::Peri<'static, embassy_stm32::peripherals::RNG>) -> u32 {
    use embassy_stm32::pac;
    pac::RCC.ahb2enr().modify(|w| w.set_rngen(true));
    pac::RNG.cr().modify(|w| w.set_rngen(true));
    let deadline = embassy_time::Instant::now() + embassy_time::Duration::from_millis(100);
    while !pac::RNG.sr().read().drdy() {
        defmt::assert!(
            embassy_time::Instant::now() < deadline,
            "L4 RNG unavailable"
        );
    }
    defmt::assert!(!pac::RNG.sr().read().cecs() && !pac::RNG.sr().read().secs());
    let value = pac::RNG.dr().read();
    pac::RNG.cr().modify(|w| w.set_rngen(false));
    pac::RCC.ahb2enr().modify(|w| w.set_rngen(false));
    value
}
