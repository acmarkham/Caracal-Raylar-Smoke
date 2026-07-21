//! Standalone six-channel microphone-driver test.

#![no_std]
#![no_main]

use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::dma::Channel;
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::{bind_interrupts, peripherals};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, PdmMicArray, PdmMicDma};
use raylar_drivers::mic_array::stm32::{
    Dma0TimestampHandler, Dma5TimestampHandler, DmaChannels, Pins, Stm32MicrophoneDriver,
};
use raylar_drivers::mic_array::{MicrophoneConfig, MicrophoneResources};
use {defmt_rtt as _, panic_probe as _};

const SAMPLE_RATE: usize = 16_000;
const HALF_MS: usize = 100;
const HALF_SAMPLES: usize = SAMPLE_RATE * HALF_MS / 1_000;
const DMA_BUFFER_SAMPLES: usize = HALF_SAMPLES * 2;

static MICROPHONES: MicrophoneResources<DMA_BUFFER_SAMPLES> = MicrophoneResources::new();
const HEAP_BYTES: usize = 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();

type Driver = Stm32MicrophoneDriver<'static, DMA_BUFFER_SAMPLES>;

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
        MicrophoneConfig::default(),
    ));
    let resolved = driver.resolved_config();
    info!(
        "Microphone driver test: clock={}Hz decimation={} buffer={} samples",
        resolved.microphone_clock_hz, resolved.decimation, DMA_BUFFER_SAMPLES
    );

    spawner.spawn(unwrap!(capture_task(driver)));
    report_stats().await
}

#[embassy_executor::task]
async fn capture_task(driver: Driver) -> ! {
    driver.run().await
}

async fn report_stats() -> ! {
    let mut receiver = unwrap!(MICROPHONES.frame_receiver());
    loop {
        let state = receiver.changed().await;
        if let Some(code) = state.error {
            error!("Microphone capture error: {}", code);
            continue;
        }
        if !state.running || state.sequence == 0 || state.sequence % 10 != 0 {
            continue;
        }

        let frame = MICROPHONES.frame(state);
        for (channel, samples) in frame.active_channels().iter().enumerate() {
            let stats = statistics(samples);
            info!(
                "MIC{} min={} max={} rms={} approx_dbfs={} seq={} dma_ticks={} start_ticks={}",
                channel + 1,
                stats.min,
                stats.max,
                stats.rms,
                stats.dbfs,
                state.sequence,
                state.completed_at_ticks,
                state.started_at_ticks,
            );
        }
    }
}

struct Statistics {
    min: i32,
    max: i32,
    rms: u32,
    dbfs: i16,
}

fn statistics(samples: &[u32]) -> Statistics {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    let mut squares = 0u64;
    for &raw in samples {
        let value = ((raw as i32) >> 8).clamp(-8_388_608, 8_388_607);
        min = min.min(value);
        max = max.max(value);
        let magnitude = value.unsigned_abs() as u64;
        squares = squares.saturating_add(magnitude * magnitude);
    }
    let mean_square = squares / samples.len() as u64;
    let rms = integer_sqrt(mean_square) as u32;
    // Integer log2 gives a stable, allocation-free ~6 dB/octave estimate.
    let dbfs = if rms == 0 {
        -144
    } else {
        ((31 - rms.leading_zeros()) as i16 - 23) * 6
    };
    Statistics {
        min,
        max,
        rms,
        dbfs,
    }
}

fn integer_sqrt(value: u64) -> u64 {
    if value == 0 {
        return 0;
    }
    let mut x = value;
    let mut next = (x + value / x) / 2;
    while next < x {
        x = next;
        next = (x + value / x) / 2;
    }
    x
}
