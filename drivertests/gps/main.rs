// GPS driver proof-of-concept.
//
// This test owns the Raylar GPS peripherals through raylar-drivers, sends a
// Start command, and logs updates from the driver's fix, PPS, and stats
// publication channels. This test selects TIM4_CH4 input capture through
// GpsConfig and logs both the coarse Embassy timestamp and 1 MHz edge timing.

#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::usart::{BufferedUart, Config, DataBits, Parity, StopBits};
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Gps, Irqs};
use raylar_drivers::gps::stm32::{Stm32GpsPower, Stm32Pps};
use raylar_drivers::gps::{GpsCommand, GpsConfig, GpsDriver, GpsResources, PpsTimingSource};
use {defmt_rtt as _, panic_probe as _};

static GPS_RESOURCES: GpsResources = GpsResources::new();
const HEAP_BYTES: usize = 8 * 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();

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

    let p = embassy_stm32::init(config);
    let Board { gps, .. } = Board::new(p);

    info!("GPS driver PoC started");
    start_gps_driver(spawner, gps).await
}

async fn start_gps_driver(spawner: Spawner, gps: Gps<'static>) -> ! {
    let Gps {
        usart,
        tx,
        rx,
        pps,
        pps_capture_pin,
        pps_capture_timer,
        rst,
        en,
    } = gps;

    let mut config = Config::default();
    config.baudrate = 9_600;
    config.data_bits = DataBits::DataBits8;
    config.parity = Parity::ParityNone;
    config.stop_bits = StopBits::STOP1;

    static mut TX_BUFFER: [u8; 64] = [0; 64];
    static mut RX_BUFFER: [u8; 512] = [0; 512];

    // This test constructs one UART for the whole driver lifetime.
    let tx_buffer = unsafe { &mut *core::ptr::addr_of_mut!(TX_BUFFER) };
    let rx_buffer = unsafe { &mut *core::ptr::addr_of_mut!(RX_BUFFER) };

    let uart = unwrap!(BufferedUart::new(
        usart, rx, tx, tx_buffer, rx_buffer, Irqs, config
    ));

    let gps_config = GpsConfig {
        pps_timing_source: PpsTimingSource::Tim4Capture,
        ..GpsConfig::default()
    };
    let pps = Stm32Pps::from_config(&gps_config, pps, pps_capture_timer, pps_capture_pin, Irqs);
    let driver = GpsDriver::new(
        uart,
        pps,
        Stm32GpsPower::new(en, rst),
        &GPS_RESOURCES,
        gps_config,
    );

    spawner.spawn(unwrap!(gps_driver_task(driver)));
    spawner.spawn(unwrap!(gps_observer_task()));

    GPS_RESOURCES.command_sender().send(GpsCommand::Start).await;

    core::future::pending().await
}

#[embassy_executor::task]
async fn gps_driver_task(driver: GpsDriver<BufferedUart<'static>, Stm32Pps, Stm32GpsPower>) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn gps_observer_task() -> ! {
    let mut stats = unwrap!(GPS_RESOURCES.stats_receiver());
    let mut fixes = unwrap!(GPS_RESOURCES.fix_receiver());
    let mut pps = unwrap!(GPS_RESOURCES.pps_receiver());

    loop {
        while let Some(next_stats) = stats.try_changed() {
            info!(
            "GPS stats: powered={} operating state={} fixes={} pps={} checksum_errors={} uart_errors={}",
            next_stats.powered,
            next_stats.operating_state,
            next_stats.num_fixes,
            next_stats.num_pps_events,
            next_stats.num_checksum_errors,
            next_stats.num_uart_errors
        );
        }

        while let Some(fix) = fixes.try_changed() {
            info!(
                "GPS fix: lat_e7={} lon_e7={} sats={} utc_time={} systime={}",
                fix.latitude.degrees_e7,
                fix.longitude.degrees_e7,
                fix.satellites,
                fix.utc_time,
                fix.system_timestamp
            );
        }

        while let Some(info) = pps.try_changed() {
            info!(
                "GPS PPS: count={} source={} timestamp={} capture_ticks={} capture_delta_ticks={} capture_hz={} system_delta_us={}",
                info.pps_count,
                info.timing_source,
                info.timestamp,
                info.capture_ticks.unwrap_or(0),
                info.capture_delta_ticks.unwrap_or(0),
                info.capture_frequency_hz.unwrap_or(0),
                info.delta_time.map(|d| d.as_micros()).unwrap_or(0)
            );
        }
        // short pause
        Timer::after_millis(50).await;
    }
}
