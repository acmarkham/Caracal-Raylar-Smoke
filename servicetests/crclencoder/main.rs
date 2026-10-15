//! Live six-microphone codec matrix, capture restart, and optional paired SD recording.
#![no_std]
#![no_main]

#[path = "../storage/common.rs"]
#[allow(dead_code)]
mod common;
#[cfg(feature = "noise-floor")]
mod noise_floor;
#[cfg(any(feature = "stage-profiling", feature = "profile-control"))]
mod profiling;
#[cfg(feature = "sd-recording")]
mod recording;

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::{bind_interrupts, dma::Channel, peripherals};
use embassy_time::{with_timeout, Duration, Instant, Timer};
use raylar_board_v1p0::{Board, PdmMicArray, PdmMicDma};
use raylar_crcl_encoder_service::{crc32c, Config, Encoder, Prediction, Sha256, Transform};
use raylar_drivers::mic_array::stm32::{
    Dma0TimestampHandler, Dma5TimestampHandler, DmaChannels, Pins, Stm32MicrophoneDriver,
};
use raylar_drivers::mic_array::{
    MicrophoneConfig, MicrophoneMode, MicrophonePreset, MicrophoneResources,
};
use static_cell::ConstStaticCell;
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static HEAP: embedded_alloc::LlffHeap = embedded_alloc::LlffHeap::empty();
pub static MICROPHONES: MicrophoneResources<3200> = MicrophoneResources::new();
static PCM: ConstStaticCell<[i32; 6144]> = ConstStaticCell::new([0; 6144]);
const OUTPUT_CAPACITY: usize = if cfg!(feature = "cheap-crcl") {
    raylar_crcl_encoder_service::cheap_record_capacity(1024, 12)
} else {
    7776
};
static OUTPUT: ConstStaticCell<[u8; OUTPUT_CAPACITY]> = ConstStaticCell::new([0; OUTPUT_CAPACITY]);
pub const MIC_CONFIG: MicrophoneConfig = MicrophoneConfig {
    mode: MicrophoneMode::Hexaphonic,
    ..MicrophoneConfig::from_preset(MicrophonePreset::ReferenceSinc5_16KhzHiperf)
};

bind_interrupts!(struct MicIrqs {
    GPDMA1_CHANNEL0 => Dma0TimestampHandler, embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH0>;
    GPDMA1_CHANNEL1 => raylar_drivers::mic_array::stm32::Dma1TimestampHandler, embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH1>;
    GPDMA1_CHANNEL2 => raylar_drivers::mic_array::stm32::Dma2TimestampHandler, embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH2>;
    GPDMA1_CHANNEL3 => raylar_drivers::mic_array::stm32::Dma3TimestampHandler, embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH3>;
    GPDMA1_CHANNEL4 => raylar_drivers::mic_array::stm32::Dma4TimestampHandler, embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH4>;
    GPDMA1_CHANNEL5 => Dma5TimestampHandler, embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH5>;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let board = Board::new(embassy_stm32::init(common::mcu_config()));
    #[cfg(any(feature = "stage-profiling", feature = "profile-control"))]
    profiling::init();
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
    } = board.pdm_mic_array;
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
            ch0: Channel::new(ch0, MicIrqs),
            ch1: Channel::new(ch1, MicIrqs),
            ch2: Channel::new(ch2, MicIrqs),
            ch3: Channel::new(ch3, MicIrqs),
            ch4: Channel::new(ch4, MicIrqs),
            ch5: Channel::new(ch5, MicIrqs)
        },
        &MICROPHONES,
        MIC_CONFIG
    ));
    spawner.spawn(unwrap!(capture_task(driver)));
    #[cfg(feature = "noise-floor")]
    noise_floor::run().await;
    let pcm = PCM.take();
    let output = OUTPUT.take();
    let mut receiver = unwrap!(MICROPHONES.frame_receiver());
    let mut previous = 0;
    let mut worst_us = 0;
    for bits in [6, 8, 10, 12] {
        for prediction in [
            Prediction::DirectRice,
            Prediction::TemporalRice,
            Prediction::Auto,
        ] {
            if cfg!(feature = "cheap-crcl") && prediction != Prediction::TemporalRice {
                continue;
            }
            let state = loop {
                let state = unwrap!(with_timeout(Duration::from_secs(3), receiver.changed()).await);
                assert!(state.error.is_none());
                if state.running && state.sequence != 0 {
                    break state;
                }
            };
            assert!(previous == 0 || state.sequence == previous + 1);
            previous = state.sequence;
            let frame = MICROPHONES.frame(state);
            assert_eq!(frame.active_channels().len(), 6);
            let channels = frame.active_channels();
            let mut before = Sha256::new();
            for n in 0..1024 {
                for c in 0..6 {
                    pcm[n * 6 + c] = (channels[c][n] as i32) >> 8;
                }
                if !cfg!(feature = "cheap-crcl") {
                    before.update(&pcm[n * 6].to_le_bytes());
                }
            }
            let config = Config {
                bits,
                prediction,
                ..if cfg!(feature = "cheap-crcl") {
                    Config::cheap()
                } else {
                    Config::default()
                }
            };
            let encoder = unwrap!(Encoder::new(config));
            let started = Instant::now();
            let mut transform = unwrap!(Transform::new(pcm, config));
            while !unwrap!(transform.advance(256)) {
                embassy_futures::yield_now().await;
            }
            let len = unwrap!(encoder.data(transform, output));
            let elapsed = started.elapsed().as_micros();
            worst_us = worst_us.max(elapsed);
            assert!(elapsed < 64000);
            let mut after = Sha256::new();
            for n in 0..1024 {
                if cfg!(feature = "cheap-crcl") {
                    assert_eq!(pcm[n * 6], (channels[0][n] as i32) >> 8);
                } else {
                    after.update(&pcm[n * 6].to_le_bytes());
                }
            }
            if !cfg!(feature = "cheap-crcl") {
                assert_eq!(before.finish(), after.finish());
                assert_eq!(
                    u32::from_le_bytes(output[len - 4..len].try_into().unwrap()),
                    crc32c(&output[..len - 4])
                );
            }
            info!(
                "CRCL live bits={} prediction={:?} bytes={} codec_us={} budget_us=64000 seq={}",
                bits, prediction, len, elapsed, state.sequence
            );
        }
    }
    MICROPHONES.set_enabled(false);
    Timer::after_millis(150).await;
    assert!(!MICROPHONES.state().running);
    MICROPHONES.set_enabled(true);
    loop {
        let state = unwrap!(with_timeout(Duration::from_secs(3), receiver.changed()).await);
        assert!(state.error.is_none());
        if state.running && state.sequence != 0 {
            assert_eq!(state.sequence, 1);
            break;
        }
    }
    info!("CRCL TARGET PASS: live_profiles={} unchanged MIC1, bounded i32 in-place work, coherent six-channel capture and restart; worst_codec_us={}",
        if cfg!(feature = "cheap-crcl") { 4 } else { 12 }, worst_us);
    #[cfg(feature = "sd-recording")]
    recording::run(spawner, board.sd, board.gps, pcm, output).await;
    MICROPHONES.set_enabled(false);
    loop {
        Timer::after_secs(60).await;
    }
}

#[embassy_executor::task]
async fn capture_task(driver: Stm32MicrophoneDriver<'static, 3200>) -> ! {
    driver.run().await
}
