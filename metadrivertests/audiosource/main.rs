//! Hardware test: one microphone array feeds two independent AudioSource readers.

#![no_std]
#![no_main]

use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::dma::Channel;
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::{bind_interrupts, peripherals};
use embedded_alloc::LlffHeap as Heap;
use raylar_audiosource::{AudioFormat, AudioSource, ReaderStart};
use raylar_board_v1p0::{Board, PdmMicArray, PdmMicDma};
use raylar_drivers::mic_array::stm32::{
    Dma0TimestampHandler, Dma5TimestampHandler, DmaChannels, Pins, Stm32MicrophoneDriver,
};
use raylar_drivers::mic_array::{
    MicrophoneConfig, MicrophoneMode, MicrophonePreset, MicrophoneResources,
};
use {defmt_rtt as _, panic_probe as _};

const CONFIG: MicrophoneConfig = MicrophoneConfig {
    mode: MicrophoneMode::Mono,
    ..MicrophoneConfig::from_preset(MicrophonePreset::Hse16MhzHclk80Exact16Khz)
};
// Prevent the source format from diverging from the microphone mode.
const CHANNELS: usize = CONFIG.mode.channel_count();
const HALF_SAMPLES: usize = 1_600;
const DMA_SAMPLES: usize = HALF_SAMPLES * 2;
const AUDIO_CAPACITY: usize = HALF_SAMPLES * CHANNELS * 4;

static MICROPHONES: MicrophoneResources<DMA_SAMPLES> = MicrophoneResources::new();
static AUDIO: AudioSource<AUDIO_CAPACITY, 2> =
    AudioSource::new(AudioFormat::new(16_000, CHANNELS as u8, 1_000_000));
const TASK_HEAP_BYTES: usize = 1_024;

#[global_allocator]
static TASK_HEAP: Heap = Heap::empty();

type Driver = Stm32MicrophoneDriver<'static, DMA_SAMPLES>;

bind_interrupts!(struct MicIrqs {
    GPDMA1_CHANNEL0 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH0>, Dma0TimestampHandler;
    GPDMA1_CHANNEL1 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH1>;
    GPDMA1_CHANNEL2 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH2>;
    GPDMA1_CHANNEL3 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH3>;
    GPDMA1_CHANNEL4 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH4>;
    GPDMA1_CHANNEL5 => embassy_stm32::dma::InterruptHandler<peripherals::GPDMA1_CH5>, Dma5TimestampHandler;
});

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(TASK_HEAP, TASK_HEAP_BYTES);
    }
    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: mhz(16),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL10,
        divp: Some(PllDiv::DIV1),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2),
    });
    config.rcc.sys = Sysclk::PLL1_R;

    let board = Board::new(embassy_stm32::init(config));
    let PdmMicArray {
        cck0,
        sd0,
        cck1,
        sd1,
        sd2,
        sd3,
        dma,
    } = board.pdm_mic_array;
    let PdmMicDma {
        ch0,
        ch1,
        ch2,
        ch3,
        ch4,
        ch5,
    } = dma;
    let driver = unwrap!(Stm32MicrophoneDriver::new(
        Pins {
            cck0,
            sd0,
            cck1,
            sd1,
            sd2,
            sd3,
        },
        DmaChannels {
            ch0: Channel::new(ch0, MicIrqs),
            ch1: Channel::new(ch1, MicIrqs),
            ch2: Channel::new(ch2, MicIrqs),
            ch3: Channel::new(ch3, MicIrqs),
            ch4: Channel::new(ch4, MicIrqs),
            ch5: Channel::new(ch5, MicIrqs),
        },
        &MICROPHONES,
        CONFIG,
    ));

    spawner.spawn(unwrap!(audio_forwarder()));
    spawner.spawn(unwrap!(recorder_consumer()));
    spawner.spawn(unwrap!(detector_consumer()));
    spawner.spawn(unwrap!(capture(driver)));
    info!("AudioSource test started with two independent readers");
    core::future::pending().await
}

#[embassy_executor::task]
async fn capture(driver: Driver) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn audio_forwarder() -> ! {
    let mut frames = unwrap!(MICROPHONES.frame_receiver());
    loop {
        let state = frames.changed().await;
        if let Some(code) = state.error {
            error!("microphone error: {}", code);
            continue;
        }
        if !state.running || state.sequence == 0 {
            continue;
        }
        let frame = MICROPHONES.frame(state);
        let channels = frame.active_channels();
        let count = channels[0].len() * channels.len();
        unwrap!(
            AUDIO.write_from_fn(count, state.completed_at_ticks, |index| {
                let channel = index % channels.len();
                let sample = index / channels.len();
                // The selected 24-bit result is held in the upper 24 bits.
                (channels[channel][sample] as i32) >> 8
            })
        );
    }
}

#[embassy_executor::task]
async fn recorder_consumer() -> ! {
    let mut reader = unwrap!(AUDIO.register(ReaderStart::Latest));
    loop {
        unwrap!(reader.wait_for_samples(HALF_SAMPLES * CHANNELS).await);
        let status = unwrap!(reader.read(HALF_SAMPLES * CHANNELS, |chunk| {
            info!(
                "recorder: position={} samples={} timestamp={:?}",
                chunk.position,
                chunk.len(),
                chunk.timestamp_ticks
            );
            chunk.len()
        }));
        if status.overruns_since_last_read != 0 {
            error!(
                "recorder dropped {} samples",
                status.dropped_since_last_read
            );
        }
    }
}

#[embassy_executor::task]
async fn detector_consumer() -> ! {
    let mut reader = unwrap!(AUDIO.register(ReaderStart::Latest));
    loop {
        unwrap!(reader.wait_for_samples(HALF_SAMPLES * CHANNELS).await);
        let status = unwrap!(reader.read(HALF_SAMPLES * CHANNELS, |chunk| {
            let peak = chunk
                .first
                .iter()
                .chain(chunk.second)
                .map(|sample| sample.unsigned_abs())
                .max()
                .unwrap_or(0);
            info!(
                "detector: position={} samples={} peak={}",
                chunk.position,
                chunk.len(),
                peak
            );
            chunk.len()
        }));
        if status.overruns_since_last_read != 0 {
            error!(
                "detector dropped {} samples",
                status.dropped_since_last_read
            );
        }
    }
}
