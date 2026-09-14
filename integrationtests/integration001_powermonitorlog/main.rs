#![no_std]
#![no_main]

use core::sync::atomic::{AtomicBool, Ordering};
use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::Peri;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Output, Pull};
use embassy_stm32::i2c::{Config as I2cConfig, I2c, mode::Master};
use embassy_stm32::mode::Blocking;
use embassy_stm32::peripherals::{PA0, PA1, PB1};
use embassy_stm32::rcc::mux::Sdmmcsel;
use embassy_stm32::rcc::*;
use embassy_stm32::sdmmc::sd::{CmdBlock, StorageDevice};
use embassy_stm32::sdmmc::{Config as SdmmcConfig, Sdmmc};
use embassy_stm32::time::{Hertz, mhz};
use embassy_stm32::usart::{BufferedUart, Config as UartConfig, DataBits, Parity, StopBits};
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Instant, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{AdcVoltages, Board, Gps, Irqs, Leds, SdCard, SensI2C, UsbCdc};
use raylar_drivers::batterycharger::{ChargerConfig, ChargerDriver, ChargerResources};
use raylar_drivers::gps::stm32::{ExtiPps, Stm32GpsPower};
use raylar_drivers::gps::{GpsCommand, GpsConfig, GpsDriver, GpsResources};
use raylar_drivers::storage::stm32::Stm32SdBlockDevice;
use raylar_drivers::storage::{PartitionedBlockDevice, StorageDriver, detect_exfat_volume};
use raylar_drivers::voltagemonitor::stm32::Stm32VoltageMonitor;
use raylar_drivers::voltagemonitor::{VoltageConfig, VoltageMonitorDriver, VoltageResources};
use raylar_logging_service::{
    LogOutcome, LoggerHandle, LoggingResources, LoggingService, ProcessOutcome, StorageLogSink,
    info as log_info,
};
use raylar_power_management_service::{PowerConfig, PowerManagementService, PowerResources};
use raylar_storage_service::StorageService;
use raylar_time_service::gps::run_gps_time_source;
use raylar_time_service::{TimeConfig, TimeResources, TimeService};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 64 * 1024;
const SD_TARGET_FREQ: Hertz = mhz(24);
const MESSAGE_LENGTH: usize = 256;
const QUEUE_DEPTH: usize = 8;
const LINE_LENGTH: usize = 384;
const FLUSH_INTERVAL: Duration = Duration::from_secs(10);
// Change this value to adjust when SD logging is allowed.
const SD_SOC_THRESHOLD_PERCENT: u8 = 40;
const SOC_CHECK_INTERVAL: Duration = Duration::from_secs(10);
const LOW_SOC_LED_PULSE_TIME: Duration = Duration::from_millis(100);
const SD_POWER_SETTLE_TIME: Duration = Duration::from_secs(1);

static GPS_RESOURCES: GpsResources = GpsResources::new();
static TIME_RESOURCES: TimeResources<4, 8> = TimeResources::new();
static VOLTAGES: VoltageResources = VoltageResources::new();
static CHARGER: ChargerResources = ChargerResources::new();
static POWER: PowerResources = PowerResources::new();
static LOGGING_RESOURCES: LoggingResources<MESSAGE_LENGTH, QUEUE_DEPTH> = LoggingResources::new();
static ERROR_SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();
static SD_LOGGING_ACTIVE: AtomicBool = AtomicBool::new(false);

type TestLogger = LoggerHandle<'static, MESSAGE_LENGTH, QUEUE_DEPTH>;
type BoardVoltageMonitor =
    Stm32VoltageMonitor<Peri<'static, PA0>, Peri<'static, PA1>, Peri<'static, PB1>, Input<'static>>;
type BoardVoltageDriver = VoltageMonitorDriver<BoardVoltageMonitor>;
type BoardChargerDriver = ChargerDriver<I2c<'static, Blocking, Master>>;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }

    let peripherals = embassy_stm32::init(mcu_config());
    let Board {
        leds,
        gps,
        sd,
        adc_voltages,
        sens_i2c,
        usb_cdc,
        ..
    } = Board::new(peripherals);
    let Leds {
        sys_main_green,
        sys_main_red,
        sys_sd_blue,
        ..
    } = leds;

    spawner.spawn(unwrap!(error_latch_task(sys_main_red)));
    run_integration(
        spawner,
        gps,
        sd,
        adc_voltages,
        sens_i2c,
        usb_cdc,
        sys_main_green,
        sys_sd_blue,
    )
    .await
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
        mul: PllMul::MUL18,
        divp: Some(PllDiv::DIV6),
        divq: Some(PllDiv::DIV2),
        divr: Some(PllDiv::DIV2),
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.hsi48 = Some(Hsi48Config::new());
    config.rcc.mux.sdmmcsel = Sdmmcsel::PLL1_P;
    config
}

async fn run_integration(
    spawner: Spawner,
    gps: Gps<'static>,
    sd: SdCard<'static>,
    adc_voltages: AdcVoltages<'static>,
    sens_i2c: SensI2C<'static>,
    usb_cdc: UsbCdc<'static>,
    heartbeat_led: Output<'static>,
    mut low_soc_led: Output<'static>,
) -> ! {
    start_time(spawner, gps).await;
    start_power(spawner, adc_voltages, sens_i2c, usb_cdc).await;

    let SdCard {
        sdmmc,
        clk,
        cmd,
        d0,
        d1,
        d2,
        d3,
        switch,
        mut power,
    } = sd;

    power.set_high();
    if switch.is_high() {
        error!("integration001 requires an SD card");
        ERROR_SIGNAL.signal(());
        pending_forever().await;
    }

    let mut sd_config = SdmmcConfig::default();
    sd_config.data_transfer_timeout = 120_000_000;
    let mut sdmmc = Sdmmc::new_4bit(sdmmc, Irqs, clk, cmd, d0, d1, d2, d3, sd_config);
    let mut command = CmdBlock::new();

    wait_for_safe_sd_soc(&mut low_soc_led).await;
    power.set_low();
    Timer::after(SD_POWER_SETTLE_TIME).await;

    let mut card = match StorageDevice::new_sd_card(&mut sdmmc, &mut command, SD_TARGET_FREQ).await
    {
        Ok(card) => card,
        Err(error) => fail_forever("SD card initialization failed", error).await,
    };

    let system_log = LOGGING_RESOURCES.register("System");
    let power_log = LOGGING_RESOURCES.register("Power");
    let time_log = LOGGING_RESOURCES.register("Time");
    spawner.spawn(unwrap!(power_time_logger_task(
        power_log,
        time_log,
        heartbeat_led
    )));

    let mut first_mount = true;
    loop {
        let mut device = Stm32SdBlockDevice::new(card);
        let volume = match detect_exfat_volume(&mut device).await {
            Ok(volume) => volume,
            Err(error) => fail_forever("exFAT volume detection failed", error).await,
        };
        let driver = StorageDriver::<_>::new(PartitionedBlockDevice::new(device, volume));
        let mut storage = match StorageService::<_, _>::new(driver, &TIME_RESOURCES) {
            Ok(storage) => storage,
            Err(error) => fail_forever("storage service construction failed", error).await,
        };
        if let Err(error) = storage.mount().await {
            fail_forever("storage mount failed", error).await;
        }
        let sink = match StorageLogSink::open(&mut storage).await {
            Ok(sink) => sink,
            Err(error) => fail_forever("logging stream open failed", error).await,
        };
        let mut logging = LoggingService::<_, MESSAGE_LENGTH, QUEUE_DEPTH, LINE_LENGTH>::new(
            &LOGGING_RESOURCES,
            sink,
        );

        if first_mount {
            record_outcome(log_info!(
                system_log,
                "integration001 powermonitorlog started; SD SOC threshold={}",
                SD_SOC_THRESHOLD_PERCENT
            ));
            first_mount = false;
        } else {
            let state = POWER.state();
            record_outcome(log_info!(
                system_log,
                "battery recovered: batt={}mV percent={:?}; SD logging resumed",
                state.battery_mv,
                state.battery_percent
            ));
        }
        SD_LOGGING_ACTIVE.store(true, Ordering::Relaxed);
        info!("integration001 logging to /syslog.txt at 0.1 Hz");

        let low_power_state = {
            let mut next_flush = Instant::now() + FLUSH_INTERVAL;
            loop {
                let state = POWER.state();
                if !sd_soc_is_safe(state.battery_percent) {
                    break state;
                }

                let now = Instant::now();
                if now >= next_flush {
                    if let Err(error) = logging.flush().await {
                        error!("log flush failed: {}", error);
                        ERROR_SIGNAL.signal(());
                    }
                    let stats = logging.stats();
                    info!(
                        "logging stats: total={} dropped={} depth={} max_depth={} bytes={} truncated={} write_failures={}",
                        stats.total_messages,
                        stats.dropped_messages,
                        stats.queue_depth,
                        stats.maximum_queue_depth,
                        stats.bytes_written,
                        stats.truncated_messages,
                        stats.write_failures,
                    );
                    if stats.dropped_messages != 0 || stats.write_failures != 0 {
                        ERROR_SIGNAL.signal(());
                    }
                    next_flush = now + FLUSH_INTERVAL;
                }

                match logging.process_one().await {
                    Ok(ProcessOutcome::Written) => {}
                    Ok(ProcessOutcome::Empty) => Timer::after_millis(10).await,
                    Err(error) => {
                        error!("log append failed: {}", error);
                        ERROR_SIGNAL.signal(());
                        Timer::after_millis(100).await;
                    }
                }
            }
        };

        SD_LOGGING_ACTIVE.store(false, Ordering::Relaxed);
        drain_logging_queue(&mut logging).await;
        record_outcome(log_info!(
            system_log,
            "battery below SD threshold: batt={}mV percent={:?} threshold={}; shutting down SD card",
            low_power_state.battery_mv,
            low_power_state.battery_percent,
            SD_SOC_THRESHOLD_PERCENT
        ));
        drain_logging_queue(&mut logging).await;
        if let Err(error) = logging.flush().await {
            error!("final log flush failed before SD shutdown: {}", error);
            ERROR_SIGNAL.signal(());
        }

        let sink = logging.into_sink();
        if let Err(error) = sink.close().await {
            error!("log close failed before SD shutdown: {}", error);
            ERROR_SIGNAL.signal(());
        }

        let driver = storage.into_inner();
        let partition = driver.into_inner();
        let device = partition.into_inner();
        card = device.into_inner();

        power.set_high();
        info!(
            "SD card powered off: batt={}mV percent={:?}",
            low_power_state.battery_mv, low_power_state.battery_percent
        );
        wait_for_safe_sd_soc(&mut low_soc_led).await;

        power.set_low();
        Timer::after(SD_POWER_SETTLE_TIME).await;
        if let Err(error) = card.reacquire(&mut command, SD_TARGET_FREQ).await {
            fail_forever("SD card reinitialization failed", error).await;
        }
    }
}

fn sd_soc_is_safe(percent: Option<u8>) -> bool {
    matches!(percent, Some(percent) if percent >= SD_SOC_THRESHOLD_PERCENT)
}

async fn wait_for_safe_sd_soc(low_soc_led: &mut Output<'_>) {
    low_soc_led.set_low();
    loop {
        let state = POWER.state();
        if sd_soc_is_safe(state.battery_percent) {
            info!(
                "battery permits SD startup: batt={}mV percent={:?} threshold={}",
                state.battery_mv, state.battery_percent, SD_SOC_THRESHOLD_PERCENT
            );
            return;
        }

        info!(
            "waiting with SD power off: batt={}mV percent={:?} threshold={}",
            state.battery_mv, state.battery_percent, SD_SOC_THRESHOLD_PERCENT
        );

        let next_check = Instant::now() + SOC_CHECK_INTERVAL;
        low_soc_led.set_high();
        Timer::after(LOW_SOC_LED_PULSE_TIME).await;
        low_soc_led.set_low();
        Timer::at(next_check).await;
    }
}

async fn drain_logging_queue<S>(
    logging: &mut LoggingService<'_, S, MESSAGE_LENGTH, QUEUE_DEPTH, LINE_LENGTH>,
) where
    S: raylar_logging_service::LogSink,
    S::Error: defmt::Format,
{
    loop {
        match logging.process_one().await {
            Ok(ProcessOutcome::Written) => {}
            Ok(ProcessOutcome::Empty) => return,
            Err(error) => {
                error!(
                    "log append failed while draining before SD shutdown: {}",
                    error
                );
                ERROR_SIGNAL.signal(());
                return;
            }
        }
    }
}

async fn start_time(spawner: Spawner, gps: Gps<'static>) {
    let Gps {
        usart,
        tx,
        rx,
        pps,
        pps_exti,
        rst,
        en,
        ..
    } = gps;
    let mut uart_config = UartConfig::default();
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

    let driver = GpsDriver::new(
        uart,
        ExtiPps::new(ExtiInput::new(pps, pps_exti, Pull::None, Irqs)),
        Stm32GpsPower::new(en, rst),
        &GPS_RESOURCES,
        GpsConfig::default(),
    );
    let time_service = TimeService::new(&TIME_RESOURCES, TimeConfig::default());
    let correlations = unwrap!(GPS_RESOURCES.time_receiver()).as_dyn();

    spawner.spawn(unwrap!(gps_driver_task(driver)));
    spawner.spawn(unwrap!(time_service_task(time_service)));
    spawner.spawn(unwrap!(gps_time_source_task(correlations)));
    GPS_RESOURCES.command_sender().send(GpsCommand::Start).await;
}

async fn start_power(
    spawner: Spawner,
    adc_voltages: AdcVoltages<'static>,
    sens_i2c: SensI2C<'static>,
    usb_cdc: UsbCdc<'static>,
) {
    let power_service = PowerManagementService::new(
        &POWER,
        unwrap!(VOLTAGES.state_receiver()).as_dyn(),
        unwrap!(CHARGER.state_receiver()).as_dyn(),
        PowerConfig::default(),
    );

    spawner.spawn(unwrap!(voltage_driver_task(build_voltage_driver(
        adc_voltages,
        usb_cdc
    ))));
    spawner.spawn(unwrap!(charger_driver_task(build_charger_driver(sens_i2c))));
    spawner.spawn(unwrap!(power_service_task(power_service)));
}

fn build_voltage_driver(
    adc_voltages: AdcVoltages<'static>,
    usb_cdc: UsbCdc<'static>,
) -> BoardVoltageDriver {
    let AdcVoltages {
        adc,
        adc4,
        v_dc,
        v_batt,
        v_solar,
    } = adc_voltages;
    let UsbCdc { vbus, .. } = usb_cdc;
    let usb_present = Input::new(vbus, Pull::None);
    let sampler = Stm32VoltageMonitor::new(adc, adc4, v_dc, v_batt, v_solar, usb_present);

    VoltageMonitorDriver::new(sampler, &VOLTAGES, VoltageConfig::default())
}

fn build_charger_driver(sens_i2c: SensI2C<'static>) -> BoardChargerDriver {
    let SensI2C { i2c, scl, sda } = sens_i2c;
    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);
    let i2c = I2c::new_blocking(i2c, scl, sda, i2c_config);

    ChargerDriver::new(i2c, &CHARGER, ChargerConfig::default())
}

#[embassy_executor::task]
async fn power_time_logger_task(
    power_log: TestLogger,
    time_log: TestLogger,
    mut heartbeat_led: Output<'static>,
) -> ! {
    loop {
        heartbeat_led.set_high();
        if SD_LOGGING_ACTIVE.load(Ordering::Relaxed) {
            let power = POWER.state();
            record_outcome(log_info!(
                power_log,
                "source={:?} batt={}mV solar={}mV ext_dc={}mV charging={} percent={:?} health={:?} charger_state={:?} charger_fault={:?}",
                power.source,
                power.battery_mv,
                power.solar_mv,
                power.ext_dc_mv,
                power.charging,
                power.battery_percent,
                power.health,
                power.charger.state,
                power.charger.fault
            ));

            let time = TIME_RESOURCES.time_state();
            match TIME_RESOURCES.current_utc() {
                Ok(utc) => record_outcome(log_info!(
                    time_log,
                    "UTC {} GPS ON valid={} source={:?} uncertainty_us={}",
                    utc.seconds,
                    time.utc_valid,
                    time.active_time_source,
                    time.uncertainty_us
                )),
                Err(_) => record_outcome(log_info!(
                    time_log,
                    "UTC unavailable GPS ON valid=false source={:?} uncertainty_us={}",
                    time.active_time_source,
                    time.uncertainty_us
                )),
            }
        }
        Timer::after_millis(100).await;
        heartbeat_led.set_low();
        Timer::after_secs(10).await;
    }
}

fn record_outcome(outcome: LogOutcome) {
    if matches!(outcome, LogOutcome::DroppedQueueFull) {
        ERROR_SIGNAL.signal(());
    }
}

async fn fail_forever<E: defmt::Format>(message: &str, value: E) -> ! {
    error!("{}: {}", message, value);
    ERROR_SIGNAL.signal(());
    pending_forever().await
}

async fn pending_forever() -> ! {
    core::future::pending::<()>().await;
    unreachable!()
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
async fn voltage_driver_task(driver: BoardVoltageDriver) -> ! {
    driver.run().await
}

#[embassy_executor::task]
async fn charger_driver_task(mut driver: BoardChargerDriver) -> ! {
    if driver.initialize().is_err() {
        ERROR_SIGNAL.signal(());
    }

    loop {
        if driver.refresh_state().is_err() {
            ERROR_SIGNAL.signal(());
        }
        Timer::after_secs(1).await;
    }
}

#[embassy_executor::task]
async fn power_service_task(service: PowerManagementService) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn error_latch_task(mut red_led: Output<'static>) -> ! {
    ERROR_SIGNAL.wait().await;
    red_led.set_high();
    pending_forever().await
}
