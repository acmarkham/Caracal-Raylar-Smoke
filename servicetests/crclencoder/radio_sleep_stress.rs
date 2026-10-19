//! Isolated LR1121 retained sleep/wake regression. No GPS/SD/audio tasks or TX.
#![no_std]
#![no_main]
use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::{
    rcc::*,
    spi::{Config, Spi},
    time::mhz,
};
use embassy_time::{with_timeout, Duration, Instant, Timer};
use raylar_board_v1p0::{Board, EbyteRf};
use raylar_drivers::radio::{
    ChannelConfig, LoRaChannel, ManualCsSpiDevice, ModulationConfig, RadioDriver, RadioState,
};
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static HEAP: embedded_alloc::LlffHeap = embedded_alloc::LlffHeap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, 8192);
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
    let EbyteRf {
        spi,
        sck,
        miso,
        mosi,
        cs,
        busy,
        nrst,
        irq,
    } = board.ebyte_rf;
    let mut spi_config = Config::default();
    spi_config.frequency = mhz(1);
    let spi = Spi::new_blocking(spi, sck, mosi, miso, spi_config);
    let mut radio = RadioDriver::new(ManualCsSpiDevice::new(spi, cs), busy, nrst, irq);
    info!("RADIO_SLEEP_TEST start: 200 cycles, retained sleep, no TX/GPS/SD/audio");
    unwrap!(radio.initialize().await);
    let channel = ChannelConfig {
        frequency_hz: 868_100_000,
        modulation: ModulationConfig::LoRa(LoRaChannel::default()),
        rx_boosted: false,
    };
    unwrap!(radio.prepare_channel(&channel).await);
    let mut sleep_max = 0u64;
    let mut wake_max = 0u64;
    let mut buffer = [0u8; 255];
    for cycle in 1..=200u32 {
        // Drop RX while it waits for IRQ; next sleep must reconcile it.
        if cycle % 2 == 0 {
            let start = Instant::now() + Duration::from_millis(30);
            assert!(with_timeout(
                Duration::from_millis(60),
                radio.receive_at(start, start + Duration::from_secs(1), &mut buffer)
            )
            .await
            .is_err());
            assert_eq!(radio.state(), RadioState::Rx);
        }
        let start = Instant::now();
        unwrap!(radio.sleep().await);
        let sleep_us = start.elapsed().as_micros();
        sleep_max = sleep_max.max(sleep_us);
        assert_eq!(radio.state(), RadioState::Sleep);
        // Board v1p0: PE8=NSS, PE10=BUSY. Observe hardware without issuing
        // another SPI command, which would itself wake the sleeping chip.
        assert_ne!(embassy_stm32::pac::GPIOE.idr().read().idr(10) as u8, 0);
        assert_ne!(embassy_stm32::pac::GPIOE.idr().read().idr(8) as u8, 0);
        // Alternate immediate wake and a quiet interval.
        if cycle % 2 == 0 {
            Timer::after_millis(100).await;
        }
        let start = Instant::now();
        unwrap!(radio.wake().await);
        let wake_us = start.elapsed().as_micros();
        wake_max = wake_max.max(wake_us);
        assert_eq!(radio.state(), RadioState::Standby);
        assert_eq!(embassy_stm32::pac::GPIOE.idr().read().idr(10) as u8, 0);
        assert_eq!(radio.stats().resets, 1);
        if cycle % 20 == 0 {
            info!(
                "RADIO_SLEEP_TEST progress cycle={} sleep_us={} wake_us={} resets={}",
                cycle,
                sleep_us,
                wake_us,
                radio.stats().resets
            );
        }
    }
    info!("RADIO_SLEEP_TEST PASS cycles=200 cancelled_rx=100 sleep_max_us={} wake_max_us={} resets={} command_errors={} transport_errors={}",
        sleep_max, wake_max, radio.stats().resets, radio.stats().command_errors, radio.stats().transport_errors);
    unwrap!(radio.sleep().await);
    loop {
        Timer::after_secs(60).await;
    }
}
