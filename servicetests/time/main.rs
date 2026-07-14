#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_stm32::usart::{BufferedUart, Config, DataBits, Parity, StopBits};
use embassy_time::{Instant, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Gps, Irqs};
use raylar_drivers::gps::stm32::{ExtiPps, Stm32GpsPower};
use raylar_drivers::gps::{GpsCommand, GpsConfig, GpsDriver, GpsResources};
use raylar_time_service::gps::run_gps_time_source;
use raylar_time_service::{TimeConfig, TimeResources, TimeService};
use {defmt_rtt as _, panic_probe as _};

static GPS_RESOURCES: GpsResources = GpsResources::new();
static TIME_RESOURCES: TimeResources<4, 8> = TimeResources::new();
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
    start_services(spawner, gps).await
}

async fn start_services(spawner: Spawner, gps: Gps<'static>) -> ! {
    let Gps {
        usart,
        tx,
        rx,
        pps,
        rst,
        en,
        ..
    } = gps;
    let mut uart_config = Config::default();
    uart_config.baudrate = 9_600;
    uart_config.data_bits = DataBits::DataBits8;
    uart_config.parity = Parity::ParityNone;
    uart_config.stop_bits = StopBits::STOP1;

    static mut TX_BUFFER: [u8; 64] = [0; 64];
    static mut RX_BUFFER: [u8; 512] = [0; 512];
    let tx_buffer = unsafe { &mut *core::ptr::addr_of_mut!(TX_BUFFER) };
    let rx_buffer = unsafe { &mut *core::ptr::addr_of_mut!(RX_BUFFER) };
    let uart = unwrap!(BufferedUart::new(
        usart,
        rx,
        tx,
        tx_buffer,
        rx_buffer,
        Irqs,
        uart_config
    ));

    let gps_driver = GpsDriver::new(
        uart,
        ExtiPps::new(pps),
        Stm32GpsPower::new(en, rst),
        &GPS_RESOURCES,
        GpsConfig::default(),
    );
    let time_service = TimeService::new(&TIME_RESOURCES, TimeConfig::default());
    let correlations = unwrap!(GPS_RESOURCES.time_receiver()).as_dyn();

    spawner.spawn(unwrap!(gps_driver_task(gps_driver)));
    spawner.spawn(unwrap!(time_service_task(time_service)));
    spawner.spawn(unwrap!(gps_time_source_task(correlations)));
    spawner.spawn(unwrap!(time_observer_task()));
    GPS_RESOURCES.command_sender().send(GpsCommand::Start).await;
    core::future::pending().await
}

#[embassy_executor::task]
async fn gps_driver_task(driver: GpsDriver<BufferedUart<'static>, ExtiPps, Stm32GpsPower>) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn time_service_task(service: TimeService<4, 8>) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn gps_time_source_task(
    correlations: embassy_sync::watch::DynReceiver<'static, raylar_drivers::gps::TimeCorrelation>,
) -> ! {
    run_gps_time_source(correlations, TIME_RESOURCES.anchor_sender()).await
}

#[embassy_executor::task]
async fn time_observer_task() -> ! {
    loop {
        Timer::after_secs(1).await;
        let now = Instant::now();
        let state = TIME_RESOURCES.time_state();
        match TIME_RESOURCES.system_to_utc(now) {
            Ok(utc) => info!(
                "time: system_us={} utc={}s+{}us valid={} drift_ppb={} uncertainty_us={} holdover_ms={} source={} accepted={} rejected={}",
                now.as_micros(), utc.seconds, utc.microseconds, state.utc_valid,
                state.estimated_frequency_error_ppb, state.uncertainty_us,
                state.holdover_duration.as_millis(), state.active_time_source,
                state.accepted_anchors, state.rejected_anchors,
            ),
            Err(_) => info!(
                "time: system_us={} utc=unavailable valid=false uncertainty_us={} accepted={} rejected={}",
                now.as_micros(), state.uncertainty_us,
                state.accepted_anchors, state.rejected_anchors,
            ),
        }
    }
}
