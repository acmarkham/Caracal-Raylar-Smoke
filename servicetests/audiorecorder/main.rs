//! Hardware test for GPS-gated, minute-long mono WAV recording.

#![no_std]
#![no_main]

extern crate alloc;

#[path = "../storage/common.rs"]
mod common;

use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::dma::Channel;
use embassy_stm32::gpio::Output;
use embassy_stm32::rcc::mux::Sdmmcsel;
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::{bind_interrupts, peripherals};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel as SyncChannel;
use embassy_time::{Timer, TICK_HZ};
use embedded_alloc::LlffHeap as Heap;
use raylar_audio_recorder_service::{AudioRecorder, AudioRecorderConfig, TimeMetadataSource};
use raylar_audiosource::{AudioFormat, AudioSource};
use raylar_board_v1p0::{Board, Leds, PdmMicArray, PdmMicDma};
use raylar_drivers::mic_array::stm32::{
    Dma0TimestampHandler, Dma5TimestampHandler, DmaChannels, Pins, Stm32MicrophoneDriver,
};
use raylar_drivers::mic_array::{
    MicrophoneConfig, MicrophoneMode, MicrophonePreset, MicrophoneResources,
};
use raylar_storage_service::{StorageLayout, StorageService};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 64 * 1024;
const MIC_CONFIG: MicrophoneConfig = MicrophoneConfig {
    mode: MicrophoneMode::Mono,
    ..MicrophoneConfig::from_preset(MicrophonePreset::ReferenceSinc5_16KhzHiperf)
};
const CHANNELS: usize = MIC_CONFIG.mode.channel_count();
const SAMPLE_RATE_HZ: usize = 16_000;
const HALF_SAMPLES: usize = 1_600;
const DMA_SAMPLES: usize = HALF_SAMPLES * 2;
/// Four seconds covers the measured SD close/open/header latency at rotation.
const AUDIO_CAPACITY: usize = SAMPLE_RATE_HZ * CHANNELS * 4;

#[global_allocator]
static HEAP: Heap = Heap::empty();

static MICROPHONES: MicrophoneResources<DMA_SAMPLES> = MicrophoneResources::new();
static AUDIO: AudioSource<AUDIO_CAPACITY, 2> =
    AudioSource::new(AudioFormat::new(16_000, CHANNELS as u8, 1_000_000));
static SD_WRITE_FLASHES: SyncChannel<CriticalSectionRawMutex, (), 4> = SyncChannel::new();

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
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }
    let board = Board::new(embassy_stm32::init(mcu_config()));
    let Board {
        gps,
        sd,
        pdm_mic_array,
        leds,
        ..
    } = board;
    let Leds {
        sys_main_green,
        sys_sd_blue,
        mut sys_main_red,
        ..
    } = leds;
    let storage_driver = common::storage_driver(sd).await;
    let mut storage = unwrap!(StorageService::<_, _>::new(
        storage_driver,
        &common::TIME_RESOURCES
    ));
    unwrap!(storage.mount().await);

    common::start_time(spawner, gps).await;
    info!("waiting for valid GPS UTC before recording");
    while common::TIME_RESOURCES.current_utc().is_err() {
        Timer::after_secs(1).await;
    }

    let driver = microphone_driver(pdm_mic_array);
    let resolved = driver.resolved_config();
    info!(
        "microphone configured: requested={}Hz calculated={}Hz clock={}Hz kernel={:?} decimation={} sinc={:?} reshape={:?} hpf={}",
        resolved.requested.sample_rate.hz(),
        resolved.actual_sample_rate_hz,
        resolved.microphone_clock_hz,
        resolved.requested.kernel_clock,
        resolved.total_decimation,
        resolved.requested.sinc_filter,
        resolved.requested.reshape_filter,
        resolved.requested.high_pass_filter,
    );
    spawner.spawn(unwrap!(sd_write_led(sys_sd_blue)));
    spawner.spawn(unwrap!(audio_forwarder(sys_main_green)));
    spawner.spawn(unwrap!(capture(driver)));

    let metadata = TimeMetadataSource::new(&common::TIME_RESOURCES);
    let mut recorder = unwrap!(AudioRecorder::<_, _, AUDIO_CAPACITY, 2>::new(
        &AUDIO,
        storage,
        metadata,
        AudioRecorderConfig {
            recording_seconds: 60,
            storage_layout: StorageLayout::IntervalFolders {
                interval_seconds: 600,
            },
        },
    ));
    unwrap!(recorder.start().await);
    info!("recording mono 16 kHz WAV: 60-second files, 600-second folders");
    loop {
        match recorder.record_next().await {
            Ok(progress) => {
                SD_WRITE_FLASHES.send(()).await;
                if progress.rotated {
                    sys_main_red.toggle();
                    info!("audio file rotated");
                }
                if progress.dropped_samples != 0 {
                    error!("audio source dropped {} samples", progress.dropped_samples);
                }
            }
            Err(error) => {
                error!("audio recorder failed: {:?}", error);
                Timer::after_secs(1).await;
            }
        }
    }
}

fn microphone_driver(pdm: PdmMicArray<'static>) -> Driver {
    let PdmMicArray {
        cck0,
        sd0,
        cck1,
        sd1,
        sd2,
        sd3,
        dma,
    } = pdm;
    let PdmMicDma {
        ch0,
        ch1,
        ch2,
        ch3,
        ch4,
        ch5,
    } = dma;
    unwrap!(Stm32MicrophoneDriver::new(
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
        MIC_CONFIG,
    ))
}

fn mcu_config() -> embassy_stm32::Config {
    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: mhz(16),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll1 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL10,
        divp: Some(PllDiv::DIV4),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2),
    });
    config.rcc.pll3 = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV1,
        mul: PllMul::MUL12,
        divp: None,
        divq: Some(PllDiv::DIV2),
        divr: None,
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.hsi48 = Some(Hsi48Config::new());
    config.rcc.mux.sdmmcsel = Sdmmcsel::PLL1_P;
    config
}

#[embassy_executor::task]
async fn capture(driver: Driver) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn audio_forwarder(mut dma_led: Output<'static>) -> ! {
    let mut frames = unwrap!(MICROPHONES.frame_receiver());
    let mut last_sequence = 0;
    let mut rate_sequence = 0;
    let mut rate_ticks = 0;
    loop {
        let state = frames.changed().await;
        if let Some(code) = state.error {
            error!("microphone error: {}", code);
            continue;
        }
        if !state.running || state.sequence == 0 || state.sequence <= last_sequence {
            continue;
        }
        last_sequence = state.sequence;
        dma_led.toggle();
        if rate_sequence == 0 {
            rate_sequence = state.sequence;
            rate_ticks = state.completed_at_ticks;
        } else if state.sequence - rate_sequence >= 10 {
            let elapsed = state.completed_at_ticks.saturating_sub(rate_ticks);
            if elapsed != 0 {
                let samples = (state.sequence - rate_sequence) * HALF_SAMPLES as u64;
                let measured_hz = samples.saturating_mul(TICK_HZ) / elapsed;
                info!(
                    "microphone measured rate={}Hz frames={} elapsed_ticks={}",
                    measured_hz,
                    state.sequence - rate_sequence,
                    elapsed,
                );
            }
            rate_sequence = state.sequence;
            rate_ticks = state.completed_at_ticks;
        }
        let frame = MICROPHONES.frame(state);
        let channel = frame.active_channels()[0];
        unwrap!(AUDIO.write_from_fn(
            channel.len(),
            state.completed_at_ticks,
            |index| (channel[index] as i32) >> 8,
        ));
    }
}

#[embassy_executor::task]
async fn sd_write_led(mut led: Output<'static>) -> ! {
    loop {
        SD_WRITE_FLASHES.receive().await;
        led.set_high();
        Timer::after_millis(25).await;
        led.set_low();
    }
}
