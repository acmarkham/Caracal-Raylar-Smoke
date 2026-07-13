// Ebyte E80 LR1121 RF module RX/TX smoke test.
//
// Runs continuous 2.4 GHz LoRa RX and occasionally transmits a short packet.
// Use two boards with this same firmware; each board listens most of the time
// and sends a randomized ping. TX flashes sys_sd_blue. RX flashes
// sys_main_green. Messages are printed via defmt.
//
// SPI interface:
// - PE13 RF_SCK  -> SPI1_SCK
// - PE14 RF_MISO -> SPI1_MISO
// - PE15 RF_MOSI -> SPI1_MOSI
// - PE8  RF_CS   -> manual chip select
//
// Control/status:
// - PE10 RF_BUSY
// - PE11 RF_NRST
// - PE12 RF_IRQ, LR1121 DIO9 interrupt output

#![no_std]
#![no_main]

use core::cmp::min;

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{Input, Output};
use embassy_stm32::mode::{Async, Blocking};
use embassy_stm32::rcc::*;
use embassy_stm32::spi::mode::Master;
use embassy_stm32::spi::{Config as SpiConfig, Spi};
use embassy_stm32::time::mhz;
use embassy_time::{Duration, Instant, Timer};
use raylar_board_v1p0::{Board, EbyteRf, Leds};
use {defmt_rtt as _, panic_probe as _};

const BUSY_TIMEOUT: Duration = Duration::from_millis(500);
const RADIO_FREQUENCY_HZ: u32 = 2_445_000_000;
const MAX_PAYLOAD_LEN: u8 = 32;
const TX_PAYLOAD_LEN: usize = 16;
const RX_TIMEOUT_CONTINUOUS: [u8; 3] = [0xFF, 0xFF, 0xFF];
const TX_TIMEOUT_RTC: [u8; 3] = [0x00, 0x80, 0x00];
const LOOP_TICK_MS: u32 = 20;
const TCXO_TUNE_V1P8: u8 = 0x02;
const TCXO_STARTUP_DELAY_TICKS: u32 = 320;
const LFCLK_XTAL_WAIT: u8 = 0x05;
const HF_PA_CONFIG: [u8; 4] = [0x02, 0x00, 0x00, 0x00];
const HF_MAX_TX_PARAMS: [u8; 2] = [0x0D, 0x02];
const EBYTE_RFSW_ENABLE_MASK: u8 = 0x0F;
const EBYTE_RFSW_STANDBY: u8 = 0x00;
const EBYTE_RFSW_RX: u8 = 0x02;
const EBYTE_RFSW_SUBGHZ_TX_LP: u8 = 0x03;
const EBYTE_RFSW_SUBGHZ_TX_HP: u8 = 0x01;
const EBYTE_RFSW_TX_HF: u8 = 0x00;
const EBYTE_RFSW_GNSS: u8 = 0x04;
const EBYTE_RFSW_WIFI: u8 = 0x08;

const IRQ_TX_DONE: u32 = 1 << 2;
const IRQ_RX_DONE: u32 = 1 << 3;
const IRQ_HEADER_ERR: u32 = 1 << 6;
const IRQ_CRC_ERR: u32 = 1 << 7;
const IRQ_TIMEOUT: u32 = 1 << 10;
const IRQ_CMD_ERROR: u32 = 1 << 22;
const IRQ_ERROR: u32 = 1 << 23;
const IRQ_MASK: u32 = IRQ_TX_DONE
    | IRQ_RX_DONE
    | IRQ_HEADER_ERR
    | IRQ_CRC_ERR
    | IRQ_TIMEOUT
    | IRQ_CMD_ERROR
    | IRQ_ERROR;

#[derive(Clone, Copy)]
#[repr(u16)]
enum Opcode {
    GetStatus = 0x0100,
    GetVersion = 0x0101,
    GetErrors = 0x010D,
    ClearErrors = 0x010E,
    Calibrate = 0x010F,
    SetRegMode = 0x0110,
    SetDioAsRfSwitch = 0x0112,
    SetDioIrqParams = 0x0113,
    ClearIrq = 0x0114,
    ConfigLfClock = 0x0116,
    SetTcxoMode = 0x0117,
    SetStandby = 0x011C,
    WriteBuffer8 = 0x0109,
    ReadBuffer8 = 0x010A,
    ClearRxBuffer = 0x010B,
    GetRxBufferStatus = 0x0203,
    GetPacketStatus = 0x0204,
    SetRx = 0x0209,
    SetTx = 0x020A,
    SetRfFrequency = 0x020B,
    SetPacketType = 0x020E,
    SetModulationParams = 0x020F,
    SetPacketParams = 0x0210,
    SetTxParams = 0x0211,
    SetPaConfig = 0x0215,
    SetRxBoosted = 0x0227,
    SetLoRaSyncWord = 0x022B,
}

type RadioSpi = Spi<'static, Blocking, Master>;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
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

    info!("Ebyte E80 LR1121 RX/TX smoke test started");
    run_rf_rx_tx(ebyte_rf, leds).await
}

async fn run_rf_rx_tx(rf: EbyteRf<'static>, leds: Leds<'static>) -> ! {
    let EbyteRf {
        spi,
        sck,
        miso,
        mosi,
        mut cs,
        busy,
        mut nrst,
        mut irq,
    } = rf;

    let Leds {
        mut sys_main_green,
        mut sys_sd_blue,
        ..
    } = leds;

    cs.set_high();
    sys_main_green.set_low();
    sys_sd_blue.set_low();

    info!("Resetting LR1121");
    nrst.set_low();
    Timer::after_millis(10).await;
    nrst.set_high();
    Timer::after_millis(25).await;

    if !wait_busy_low(&busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY stayed high after reset");
        pending_forever().await;
    }

    let mut spi_config = SpiConfig::default();
    spi_config.frequency = mhz(1);
    let mut spi = Spi::new_blocking(spi, sck, mosi, miso, spi_config);

    let version = lr11xx_version(&mut spi, &mut cs, &busy).await;
    info!(
        "LR1121 version: hw={} type={} fw={}.{}",
        version[0], version[1], version[2], version[3],
    );

    enable_tcxo(&mut spi, &mut cs, &busy).await;
    configure_radio(&mut spi, &mut cs, &busy).await;
    start_rx_continuous(&mut spi, &mut cs, &busy).await;
    info!("RX continuous at {} Hz", RADIO_FREQUENCY_HZ);

    let mut rx_buf = [0u8; MAX_PAYLOAD_LEN as usize];
    let mut seq = 0u32;
    let mut rng = 0x4c52_3131u32
        ^ ((version[0] as u32) << 24)
        ^ ((version[1] as u32) << 16)
        ^ ((version[2] as u32) << 8)
        ^ version[3] as u32;
    let mut tx_due_ms = next_tx_delay_ms(&mut rng);

    loop {
        if irq.is_high() {
            handle_radio_irq(
                &mut spi,
                &mut cs,
                &busy,
                &mut irq,
                &mut sys_main_green,
                &mut rx_buf,
            )
            .await;
        }

        if tx_due_ms == 0 {
            seq = seq.wrapping_add(1);
            send_ping(&mut spi, &mut cs, &busy, &mut irq, &mut sys_sd_blue, seq).await;
            start_rx_continuous(&mut spi, &mut cs, &busy).await;
            tx_due_ms = next_tx_delay_ms(&mut rng);
            info!("Next TX in {} ms", tx_due_ms);
        }

        Timer::after_millis(LOOP_TICK_MS as u64).await;
        tx_due_ms = tx_due_ms.saturating_sub(LOOP_TICK_MS);
    }
}

async fn configure_radio(spi: &mut RadioSpi, cs: &mut Output<'static>, busy: &Input<'static>) {
    info!("Configuring LR1121 for 2.4 GHz LoRa packet test");

    clear_errors(spi, cs, busy).await;
    clear_irqs(spi, cs, busy, IRQ_MASK).await;
    lr11xx_write(spi, cs, busy, Opcode::SetStandby, &[0x00]).await;
    lr11xx_write(spi, cs, busy, Opcode::SetRegMode, &[0x01]).await;
    configure_rf_switches(spi, cs, busy).await;
    lr11xx_write(spi, cs, busy, Opcode::SetPacketType, &[0x02]).await;
    lr11xx_write(
        spi,
        cs,
        busy,
        Opcode::SetRfFrequency,
        &RADIO_FREQUENCY_HZ.to_be_bytes(),
    )
    .await;

    // LoRa: SF7, 500 kHz bandwidth, CR 4/5, low data rate optimize off.
    lr11xx_write(
        spi,
        cs,
        busy,
        Opcode::SetModulationParams,
        // SF7 BW500k CR4/5 LDRO off
        //&[0x07, 0x06, 0x01, 0x00],
        // SF7 BW125k CR4/5 LDRO off
        &[0x07, 0x04, 0x01, 0x00],
        // SF12 BW125k CR4/5 LDRO on
        //&[0x0C, 0x04, 0x01, 0x01],
    )
    .await;
    lr11xx_write(spi, cs, busy, Opcode::SetLoRaSyncWord, &[0x12]).await;

    lr11xx_write(spi, cs, busy, Opcode::SetPaConfig, &HF_PA_CONFIG).await;
    lr11xx_write(spi, cs, busy, Opcode::SetTxParams, &HF_MAX_TX_PARAMS).await;
    lr11xx_write(spi, cs, busy, Opcode::SetRxBoosted, &[0x01]).await;

    let irq1 = IRQ_MASK.to_be_bytes();
    let irq2 = 0u32.to_be_bytes();
    lr11xx_write_data(spi, cs, busy, Opcode::SetDioIrqParams, &irq1, &irq2).await;
    configure_lora_packet(spi, cs, busy, MAX_PAYLOAD_LEN).await;
    lr11xx_write(spi, cs, busy, Opcode::ClearRxBuffer, &[]).await;
    clear_irqs(spi, cs, busy, IRQ_MASK).await;
}

async fn configure_rf_switches(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
) {
    info!("Configuring LR1121 RF switch mapping for Ebyte module");

    // Ebyte routes RFSW0/RFSW1 differently from Semtech reference designs.
    // RX = RFSW1, sub-GHz TX = RFSW0|RFSW1, sub-GHz TX HP = RFSW0, TX HF = 0.
    lr11xx_write(
        spi,
        cs,
        busy,
        Opcode::SetDioAsRfSwitch,
        &rf_switch_config_bytes(
            EBYTE_RFSW_ENABLE_MASK,
            EBYTE_RFSW_STANDBY,
            EBYTE_RFSW_RX,
            EBYTE_RFSW_SUBGHZ_TX_LP,
            EBYTE_RFSW_SUBGHZ_TX_HP,
            EBYTE_RFSW_TX_HF,
            EBYTE_RFSW_GNSS,
            EBYTE_RFSW_WIFI,
        ),
    )
    .await;
}

async fn enable_tcxo(spi: &mut RadioSpi, cs: &mut Output<'static>, busy: &Input<'static>) {
    info!("Enabling LR1121 TCXO supply");

    lr11xx_write(spi, cs, busy, Opcode::SetStandby, &[0x00]).await;
    clear_errors(spi, cs, busy).await;
    lr11xx_write(
        spi,
        cs,
        busy,
        Opcode::SetTcxoMode,
        &tcxo_mode_bytes(TCXO_TUNE_V1P8, TCXO_STARTUP_DELAY_TICKS),
    )
    .await;
    lr11xx_write(spi, cs, busy, Opcode::ConfigLfClock, &[LFCLK_XTAL_WAIT]).await;
    lr11xx_write(spi, cs, busy, Opcode::Calibrate, &[0x3F]).await;
    clear_errors(spi, cs, busy).await;
}

async fn configure_lora_packet(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
    payload_len: u8,
) {
    let args = [0x00, 0x0C, 0x00, payload_len, 0x01, 0x00];
    lr11xx_write(spi, cs, busy, Opcode::SetPacketParams, &args).await;
}

async fn start_rx_continuous(spi: &mut RadioSpi, cs: &mut Output<'static>, busy: &Input<'static>) {
    configure_lora_packet(spi, cs, busy, MAX_PAYLOAD_LEN).await;
    clear_irqs(spi, cs, busy, IRQ_MASK).await;
    lr11xx_write(spi, cs, busy, Opcode::SetRx, &RX_TIMEOUT_CONTINUOUS).await;
}

async fn send_ping(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
    irq: &mut ExtiInput<'static, Async>,
    tx_led: &mut Output<'static>,
    seq: u32,
) {
    let mut payload = *b"raylar-ping:0000";
    let mut n = seq % 10_000;
    payload[15] = b'0' + (n % 10) as u8;
    n /= 10;
    payload[14] = b'0' + (n % 10) as u8;
    n /= 10;
    payload[13] = b'0' + (n % 10) as u8;
    n /= 10;
    payload[12] = b'0' + (n % 10) as u8;

    info!("TX seq={} bytes={=[u8]}", seq, &payload[..]);
    tx_led.set_high();

    lr11xx_write(spi, cs, busy, Opcode::SetStandby, &[0x00]).await;
    clear_irqs(spi, cs, busy, IRQ_MASK).await;
    configure_lora_packet(spi, cs, busy, TX_PAYLOAD_LEN as u8).await;
    lr11xx_write(spi, cs, busy, Opcode::WriteBuffer8, &payload).await;
    lr11xx_write(spi, cs, busy, Opcode::SetTx, &TX_TIMEOUT_RTC).await;

    let started = Instant::now();
    loop {
        if irq.is_high() || Instant::now().duration_since(started) >= Duration::from_millis(5000) {
            let irq_status = lr11xx_status(spi, cs, busy).await;
            if irq_status & IRQ_TX_DONE != 0 {
                info!("TX done seq={}", seq);
            } else if irq_status & IRQ_TIMEOUT != 0 {
                error!("TX timeout seq={} irq=0x{:08x}", seq, irq_status);
            } else {
                error!("TX wait ended seq={} irq=0x{:08x}", seq, irq_status);
            }
            clear_irqs(spi, cs, busy, irq_status | IRQ_MASK).await;
            break;
        }
        Timer::after_millis(10).await;
    }

    tx_led.set_low();
}

async fn handle_radio_irq(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
    _irq: &mut ExtiInput<'static, Async>,
    rx_led: &mut Output<'static>,
    rx_buf: &mut [u8; MAX_PAYLOAD_LEN as usize],
) {
    let irq_status = lr11xx_status(spi, cs, busy).await;

    if irq_status & (IRQ_CMD_ERROR | IRQ_ERROR) != 0 {
        let errors = lr11xx_errors(spi, cs, busy).await;
        error!(
            "LR1121 error irq=0x{:08x} errors=0x{:04x}",
            irq_status, errors
        );
        clear_errors(spi, cs, busy).await;
    }

    if irq_status & IRQ_RX_DONE != 0 {
        if irq_status & (IRQ_HEADER_ERR | IRQ_CRC_ERR) != 0 {
            error!("RX packet error irq=0x{:08x}", irq_status);
        } else {
            let (len, offset) = rx_buffer_status(spi, cs, busy).await;
            let len = min(len as usize, rx_buf.len());
            read_buffer(spi, cs, busy, offset, &mut rx_buf[..len]).await;
            let pkt_status = lora_packet_status(spi, cs, busy).await;

            rx_led.set_high();
            info!(
                "RX len={} offset={} rssi_raw={} snr_raw={} signal_rssi_raw={} bytes={=[u8]}",
                len,
                offset,
                pkt_status[0],
                pkt_status[1],
                pkt_status[2],
                &rx_buf[..len],
            );
            Timer::after_millis(60).await;
            rx_led.set_low();
        }
    }

    if irq_status & IRQ_TIMEOUT != 0 {
        info!("RX/TX timeout irq=0x{:08x}", irq_status);
    }

    clear_irqs(spi, cs, busy, irq_status | IRQ_MASK).await;
    start_rx_continuous(spi, cs, busy).await;
}

async fn lr11xx_version(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
) -> [u8; 4] {
    let mut version = [0u8; 4];
    lr11xx_cmd_read(spi, cs, busy, Opcode::GetVersion, &mut version).await;
    version
}

async fn lr11xx_errors(spi: &mut RadioSpi, cs: &mut Output<'static>, busy: &Input<'static>) -> u16 {
    let mut errors = [0u8; 2];
    lr11xx_cmd_read(spi, cs, busy, Opcode::GetErrors, &mut errors).await;
    u16::from_be_bytes(errors)
}

async fn clear_errors(spi: &mut RadioSpi, cs: &mut Output<'static>, busy: &Input<'static>) {
    lr11xx_cmd(spi, cs, busy, Opcode::ClearErrors).await;
}

async fn lr11xx_status(spi: &mut RadioSpi, cs: &mut Output<'static>, busy: &Input<'static>) -> u32 {
    let op = opcode_bytes(Opcode::GetStatus);
    let mut read = [0u8; 6];

    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high before GetStatus");
        pending_forever().await;
    }

    cs.set_low();
    let result = spi.blocking_transfer(&mut read, &op);
    cs.set_high();

    if let Err(e) = result {
        error!("SPI GetStatus transfer failed: {}", e);
        pending_forever().await;
    }

    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high after GetStatus");
        pending_forever().await;
    }

    u32::from_be_bytes([read[2], read[3], read[4], read[5]])
}

async fn rx_buffer_status(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
) -> (u8, u8) {
    let mut status = [0u8; 2];
    lr11xx_cmd_read(spi, cs, busy, Opcode::GetRxBufferStatus, &mut status).await;
    (status[0], status[1])
}

async fn lora_packet_status(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
) -> [u8; 3] {
    let mut status = [0u8; 3];
    lr11xx_cmd_read(spi, cs, busy, Opcode::GetPacketStatus, &mut status).await;
    status
}

async fn read_buffer(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
    offset: u8,
    data: &mut [u8],
) {
    let args = [offset, data.len() as u8];
    lr11xx_write(spi, cs, busy, Opcode::ReadBuffer8, &args).await;
    lr11xx_read(spi, cs, busy, data).await;
}

async fn clear_irqs(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
    mask: u32,
) {
    lr11xx_write(spi, cs, busy, Opcode::ClearIrq, &mask.to_be_bytes()).await;
}

async fn lr11xx_cmd_read(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
    op: Opcode,
    data: &mut [u8],
) {
    lr11xx_cmd(spi, cs, busy, op).await;
    lr11xx_read(spi, cs, busy, data).await;
}

async fn lr11xx_cmd(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
    op: Opcode,
) {
    let mut read = [0u8; 2];
    let write = opcode_bytes(op);

    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high before command 0x{:04x}", op as u16);
        pending_forever().await;
    }

    cs.set_low();
    let result = spi.blocking_transfer(&mut read, &write);
    cs.set_high();

    if let Err(e) = result {
        error!("SPI command 0x{:04x} failed: {}", op as u16, e);
        pending_forever().await;
    }

    check_command_status(op, read);
    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high after command 0x{:04x}", op as u16);
        pending_forever().await;
    }
}

async fn lr11xx_write(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
    op: Opcode,
    args: &[u8],
) {
    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high before write 0x{:04x}", op as u16);
        pending_forever().await;
    }

    let mut read = [0u8; 2];
    let write = opcode_bytes(op);
    cs.set_low();
    let result = spi.blocking_transfer(&mut read, &write);
    let result = if result.is_ok() && !args.is_empty() {
        spi.blocking_write(args)
    } else {
        result
    };
    cs.set_high();

    if let Err(e) = result {
        error!("SPI write opcode 0x{:04x} failed: {}", op as u16, e);
        pending_forever().await;
    }

    check_command_status(op, read);
    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high after write 0x{:04x}", op as u16);
        pending_forever().await;
    }
}

async fn lr11xx_write_data(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
    op: Opcode,
    args: &[u8],
    data: &[u8],
) {
    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high before write_data 0x{:04x}", op as u16);
        pending_forever().await;
    }

    let mut read = [0u8; 2];
    let write = opcode_bytes(op);
    cs.set_low();
    let op_result = spi.blocking_transfer(&mut read, &write);
    let args_result = if op_result.is_ok() {
        spi.blocking_write(args)
    } else {
        op_result
    };
    let data_result = if args_result.is_ok() {
        spi.blocking_write(data)
    } else {
        args_result
    };
    cs.set_high();

    if let Err(e) = data_result {
        error!("SPI write_data 0x{:04x} failed: {}", op as u16, e);
        pending_forever().await;
    }

    check_command_status(op, read);
    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high after write_data 0x{:04x}", op as u16);
        pending_forever().await;
    }
}

async fn lr11xx_read(
    spi: &mut RadioSpi,
    cs: &mut Output<'static>,
    busy: &Input<'static>,
    data: &mut [u8],
) {
    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high before read");
        pending_forever().await;
    }

    let mut stat = [0u8; 1];
    cs.set_low();
    let stat_result = spi.blocking_transfer(&mut stat, &[0x00]);
    let read_result = if stat_result.is_ok() {
        spi.blocking_read(data)
    } else {
        stat_result
    };
    cs.set_high();

    if let Err(e) = read_result {
        error!("SPI read failed: {}", e);
        pending_forever().await;
    }

    if !wait_busy_low(busy, BUSY_TIMEOUT).await {
        error!("RF_BUSY high after read");
        pending_forever().await;
    }
}

fn check_command_status(op: Opcode, status: [u8; 2]) {
    let command_status = (status[0] >> 1) & 0x07;
    if command_status != 0x02 && command_status != 0x03 {
        error!(
            "LR1121 command 0x{:04x} returned status {:02x} {:02x}",
            op as u16, status[0], status[1],
        );
    }
}

fn opcode_bytes(op: Opcode) -> [u8; 2] {
    (op as u16).to_be_bytes()
}

fn tcxo_mode_bytes(tune: u8, delay_ticks: u32) -> [u8; 4] {
    [
        tune,
        ((delay_ticks >> 16) & 0xFF) as u8,
        ((delay_ticks >> 8) & 0xFF) as u8,
        (delay_ticks & 0xFF) as u8,
    ]
}

fn rf_switch_config_bytes(
    enable: u8,
    standby: u8,
    rx: u8,
    tx: u8,
    tx_hp: u8,
    tx_hf: u8,
    gnss: u8,
    wifi: u8,
) -> [u8; 8] {
    [enable, standby, rx, tx, tx_hp, tx_hf, gnss, wifi]
}

fn next_tx_delay_ms(rng: &mut u32) -> u32 {
    *rng ^= *rng << 13;
    *rng ^= *rng >> 17;
    *rng ^= *rng << 5;
    3_000 + (*rng % 7_000)
}

async fn wait_busy_low(busy: &Input<'_>, timeout: Duration) -> bool {
    let start = Instant::now();
    while busy.is_high() {
        if Instant::now().duration_since(start) >= timeout {
            return false;
        }
        Timer::after_millis(1).await;
    }
    true
}

async fn pending_forever() -> ! {
    loop {
        Timer::after_secs(60).await;
    }
}
