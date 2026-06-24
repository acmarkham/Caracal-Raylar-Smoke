// Ebyte E80 LR1121 crate-driven RX smoke test.
//
// Receives packets from 15_ebyte_crate at 868 MHz LoRa SF7/BW125.
// Blue LED blinks rapidly as a heartbeat. Green LED flashes on packet RX.

#![no_std]
#![no_main]

use core::cmp::min;
use core::convert::Infallible;

use arbitrary_int::u24;
use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Input, Output};
use embassy_stm32::mode::Blocking;
use embassy_stm32::rcc::*;
use embassy_stm32::spi::mode::Master;
use embassy_stm32::spi::{Config as SpiConfig, Error as SpiError, Spi};
use embassy_stm32::time::mhz;
use embassy_time::{Duration, Instant, Timer};
use embedded_hal::digital::InputPin;
use embedded_hal_async::{digital::Wait, spi::Operation};
use lr11xx::{ops, Lr11xx};
use raylar_board_v1p0::{Board, EbyteRf, Leds};
use {defmt_rtt as _, panic_probe as _};

const RADIO_FREQUENCY_HZ: u32 = 868_000_000;
const BUSY_STARTUP_TIMEOUT: Duration = Duration::from_millis(500);
const TCXO_STARTUP_DELAY_TICKS: u24 = u24::new(320);
const RX_TIMEOUT_CONTINUOUS: u24 = u24::new(0xFFFFFF);
const MAX_PAYLOAD_LEN: usize = 8;
const IRQ_RX_DONE: u32 = 1 << 3;
const IRQ_HEADER_ERR: u32 = 1 << 6;
const IRQ_CRC_ERR: u32 = 1 << 7;
const IRQ_TIMEOUT: u32 = 1 << 10;
const IRQ_CMD_ERROR: u32 = 1 << 22;
const IRQ_ERROR: u32 = 1 << 23;

type RadioSpi = Spi<'static, Blocking, Master>;

#[embassy_executor::task]
async fn heartbeat_task(mut led: Output<'static>) {
    loop {
        led.set_high();
        Timer::after_millis(30).await;
        led.set_low();
        Timer::after_millis(150).await;
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
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
    let Board { ebyte_rf, leds, .. } = Board::new(p);

    info!("Ebyte E80 LR1121 crate RX smoke test started");
    run_ebyte_crate_rx(spawner, ebyte_rf, leds).await
}

async fn run_ebyte_crate_rx(spawner: Spawner, rf: EbyteRf<'static>, leds: Leds<'static>) -> ! {
    let EbyteRf {
        spi,
        sck,
        miso,
        mosi,
        mut cs,
        busy,
        mut nrst,
        irq,
    } = rf;

    let Leds {
        mut sys_main_green,
        sys_sd_blue,
        ..
    } = leds;

    cs.set_high();
    sys_main_green.set_low();
    spawner.spawn(unwrap!(heartbeat_task(sys_sd_blue)));

    info!("Resetting LR1121");
    nrst.set_low();
    Timer::after_millis(10).await;
    nrst.set_high();
    Timer::after_millis(25).await;

    let mut spi_config = SpiConfig::default();
    spi_config.frequency = mhz(1);
    let spi = Spi::new_blocking(spi, sck, mosi, miso, spi_config);
    let spi_device = ManualSpiDevice::new(spi, cs);
    let mut busy = BusyPoll::new(busy);

    info!(
        "RF_BUSY after reset: high={}",
        busy.is_high().unwrap_or(false)
    );
    if !wait_busy_low(&mut busy, BUSY_STARTUP_TIMEOUT).await {
        error!("RF_BUSY stayed high after reset");
        pending_forever().await;
    }
    info!("RF_BUSY low before lr11xx driver init");

    info!("Creating lr11xx driver");
    let mut radio = match Lr11xx::new(spi_device, busy).await {
        Ok(radio) => radio,
        Err(e) => {
            error!("Lr11xx::new failed: {}", e);
            pending_forever().await;
        }
    };

    configure_radio(&mut radio).await;
    start_rx(&mut radio).await;

    let mut rx_buf = [0u8; MAX_PAYLOAD_LEN];
    loop {
        if irq.is_high() {
            handle_rx_irq(&mut radio, &mut sys_main_green, &mut rx_buf).await;
            start_rx(&mut radio).await;
        }
        Timer::after_millis(5).await;
    }
}

async fn configure_radio<S, B>(radio: &mut Lr11xx<S, B>)
where
    S: embedded_hal_async::spi::SpiDevice<u8>,
    B: InputPin + Wait,
{
    info!("Configuring LR1121 RX through lr11xx crate");
    log_result("clear_errors", radio.clear_errors().await);
    log_result("standby_rc", radio.standby(false).await);
    log_result("set_reg_mode_dcdc", radio.set_reg_mode(true).await);
    log_result(
        "set_rf_switch",
        radio.set_dio_as_rf_switch(ebyte_rf_switch_config()).await,
    );
    log_result(
        "set_tcxo_mode_1v8",
        radio
            .set_tcxo_mode(
                ops::TcxoMode::builder()
                    .with_delay(TCXO_STARTUP_DELAY_TICKS)
                    .with_tune(ops::TcxoTune::V1p8)
                    .build(),
            )
            .await,
    );
    log_result(
        "config_lf_clock_xtal_wait",
        radio
            .config_lf_clock(
                ops::LfClkConfig::builder()
                    .with_sel(ops::LfClkSel::Crystal)
                    .with_wait(true)
                    .build(),
            )
            .await,
    );
    log_result("calibrate_all", radio.calibrate(ops::Calibrate::ALL).await);
    log_result("clear_errors_after_cal", radio.clear_errors().await);
    log_result("clear_irq_all", radio.clear_irq(ops::Interrupt::ALL).await);
    log_result(
        "packet_type_lora",
        radio.set_packet_type(ops::PacketType::LoRa).await,
    );
    log_result(
        "rf_frequency",
        radio.set_rf_frequency(RADIO_FREQUENCY_HZ).await,
    );
    log_result(
        "lora_mod_sf12_bw125",
        radio
            .set_lora_modulation(
                ops::LoRaModulation::builder()
                    .with_sf(ops::SpreadingFactor::SF12)
                    .with_bwl(ops::LoRaBandwidth::KHz125)
                    .with_cr(ops::CodingRate::Short45)
                    .with_low_data_rate_optimize(false)
                    .build(),
            )
            .await,
    );
    log_result("lora_sync_private", radio.set_lora_sync_word(0x12).await);
    log_result("rx_boosted", radio.set_rx_boosted(true).await);
    log_result(
        "fallback_standby_rc",
        radio
            .set_rx_tx_fallback_mode(ops::FallbackMode::StandbyRc)
            .await,
    );
    log_result(
        "lora_packet_rx_8b",
        radio
            .set_lora_packet(
                ops::LoRaPacket::builder()
                    .with_preamble_length(12)
                    .with_header_implicit(false)
                    .with_payload_length(MAX_PAYLOAD_LEN as u8)
                    .with_crc(true)
                    .with_invert_iq(false)
                    .build(),
            )
            .await,
    );
    log_status(radio, "configured").await;
}

async fn start_rx<S, B>(radio: &mut Lr11xx<S, B>)
where
    S: embedded_hal_async::spi::SpiDevice<u8>,
    B: InputPin + Wait,
{
    info!("Starting continuous RX");
    log_result(
        "clear_irq_before_rx",
        radio.clear_irq(ops::Interrupt::ALL).await,
    );
    log_result(
        "set_irq_rx",
        radio
            .set_dio_irq(
                ops::Interrupt::new_with_raw_value(
                    IRQ_RX_DONE
                        | IRQ_HEADER_ERR
                        | IRQ_CRC_ERR
                        | IRQ_TIMEOUT
                        | IRQ_CMD_ERROR
                        | IRQ_ERROR,
                ),
                ops::Interrupt::default(),
            )
            .await,
    );
    log_result(
        "set_rx_continuous",
        radio.set_rx(RX_TIMEOUT_CONTINUOUS).await,
    );
}

async fn handle_rx_irq<S, B>(
    radio: &mut Lr11xx<S, B>,
    rx_led: &mut Output<'static>,
    rx_buf: &mut [u8; MAX_PAYLOAD_LEN],
) where
    S: embedded_hal_async::spi::SpiDevice<u8>,
    B: InputPin + Wait,
{
    match radio.status().await {
        Ok((status, irq)) => {
            info!("RX IRQ status={} irq={}", status, irq);
            if irq.raw_value() & IRQ_RX_DONE != 0 {
                match radio.rx_buffer_status().await {
                    Ok(buffer_status) => {
                        let len = min(buffer_status.payload_length() as usize, rx_buf.len());
                        match radio
                            .read_buffer8(buffer_status.offset(), &mut rx_buf[..len])
                            .await
                        {
                            Ok(read_status) => {
                                let packet_status = radio.lora_packet_status().await;
                                rx_led.set_high();
                                match packet_status {
                                    Ok(packet_status) => info!(
                                        "RX len={} offset={} bytes={=[u8]} read_status={} rssi_raw={} snr_raw={} signal_rssi_raw={}",
                                        len,
                                        buffer_status.offset(),
                                        &rx_buf[..len],
                                        read_status,
                                        packet_status.rssi(),
                                        packet_status.snr(),
                                        packet_status.signal_rssi(),
                                    ),
                                    Err(e) => error!(
                                        "lora_packet_status failed after RX len={} offset={} bytes={=[u8]} read_status={}: {}",
                                        len,
                                        buffer_status.offset(),
                                        &rx_buf[..len],
                                        read_status,
                                        e,
                                    ),
                                }
                                Timer::after_millis(80).await;
                                rx_led.set_low();
                            }
                            Err(e) => error!("read_buffer8 failed: {}", e),
                        }
                    }
                    Err(e) => error!("rx_buffer_status failed: {}", e),
                }
            }
        }
        Err(e) => error!("status read after RX IRQ failed: {}", e),
    }

    log_status(radio, "after_rx_irq").await;
    log_result(
        "clear_irq_after_rx",
        radio.clear_irq(ops::Interrupt::ALL).await,
    );
}

fn ebyte_rf_switch_config() -> ops::RfSwitchConfig {
    ops::RfSwitchConfig::new_with_raw_value(u64::from_be_bytes([
        0x0F, 0x00, 0x02, 0x03, 0x01, 0x00, 0x04, 0x08,
    ]))
}

fn log_result<T: defmt::Format>(label: &str, result: lr11xx::Result<T>) {
    match result {
        Ok(value) => info!("{} ok: {}", label, value),
        Err(e) => error!("{} failed: {}", label, e),
    }
}

async fn log_status<S, B>(radio: &mut Lr11xx<S, B>, label: &str)
where
    S: embedded_hal_async::spi::SpiDevice<u8>,
    B: InputPin + Wait,
{
    match radio.status().await {
        Ok((status, irq)) => info!("{} status={} irq={}", label, status, irq),
        Err(e) => error!("{} status read failed: {}", label, e),
    }
    match radio.errors().await {
        Ok(errors) => info!("{} errors={}", label, errors),
        Err(e) => error!("{} error read failed: {}", label, e),
    }
}

struct BusyPoll {
    pin: Input<'static>,
}

impl BusyPoll {
    fn new(pin: Input<'static>) -> Self {
        Self { pin }
    }
}

impl embedded_hal::digital::ErrorType for BusyPoll {
    type Error = Infallible;
}

impl InputPin for BusyPoll {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_low())
    }
}

impl Wait for BusyPoll {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        while self.pin.is_low() {
            Timer::after_millis(1).await;
        }
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        while self.pin.is_high() {
            Timer::after_millis(1).await;
        }
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_low().await?;
        self.wait_for_high().await
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_high().await?;
        self.wait_for_low().await
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        let initial = self.pin.is_high();
        while self.pin.is_high() == initial {
            Timer::after_millis(1).await;
        }
        Ok(())
    }
}

async fn wait_busy_low(busy: &mut BusyPoll, timeout: Duration) -> bool {
    let start = Instant::now();
    while busy.is_high().unwrap_or(true) {
        if Instant::now().duration_since(start) >= timeout {
            return false;
        }
        Timer::after_millis(1).await;
    }
    true
}

struct ManualSpiDevice {
    spi: RadioSpi,
    cs: Output<'static>,
}

impl ManualSpiDevice {
    fn new(spi: RadioSpi, mut cs: Output<'static>) -> Self {
        cs.set_high();
        Self { spi, cs }
    }
}

impl embedded_hal_async::spi::ErrorType for ManualSpiDevice {
    type Error = SpiError;
}

impl embedded_hal_async::spi::SpiDevice<u8> for ManualSpiDevice {
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        self.cs.set_low();
        let mut result = Ok(());

        for operation in operations {
            result = match operation {
                Operation::Read(words) => self.spi.blocking_read(words),
                Operation::Write(words) => self.spi.blocking_write(words),
                Operation::Transfer(read, write) => self.spi.blocking_transfer(read, write),
                Operation::TransferInPlace(words) => self.spi.blocking_transfer_in_place(words),
                Operation::DelayNs(ns) => {
                    Timer::after_micros(((*ns as u64) + 999) / 1000).await;
                    Ok(())
                }
            };

            if result.is_err() {
                break;
            }
        }

        self.cs.set_high();
        result
    }
}

async fn pending_forever() -> ! {
    loop {
        Timer::after_secs(60).await;
    }
}
