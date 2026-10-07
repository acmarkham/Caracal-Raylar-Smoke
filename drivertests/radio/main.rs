//! Continuous RX / randomly jittered TX hardware test for the Ebyte LR1121.

#![no_std]
#![no_main]

#[cfg(any(
    all(feature = "lora", feature = "gfsk"),
    not(any(feature = "lora", feature = "gfsk"))
))]
compile_error!("select exactly one radio modulation feature: `lora` or `gfsk`");

#[cfg(any(
    not(any(
        feature = "channel-868",
        feature = "channel-915",
        feature = "channel-2445"
    )),
    all(feature = "channel-868", feature = "channel-915"),
    all(feature = "channel-868", feature = "channel-2445"),
    all(feature = "channel-915", feature = "channel-2445")
))]
compile_error!(
    "select exactly one channel feature: `channel-868`, `channel-915`, or `channel-2445`"
);

use defmt::{error, info, warn};
use embassy_executor::Spawner;
use embassy_stm32::rcc::*;
use embassy_stm32::spi::{Config as SpiConfig, Spi};
use embassy_stm32::time::mhz;
use embassy_time::{Duration, Instant, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, EbyteRf, Leds};
use raylar_drivers::identity;
use raylar_drivers::radio::{
    ChannelConfig, DriverTiming, Error as RadioError, ManualCsSpiDevice, ModulationConfig,
    RadioDriver, RxMetrics, TxConfig, TxRampTime,
};
#[cfg(feature = "gfsk")]
use raylar_drivers::radio::{
    GfskAddressFiltering, GfskBandwidth, GfskChannel, GfskCrc, GfskPacketLength,
    GfskPreambleDetector, GfskPulseShape, GfskWhitening,
};
#[cfg(feature = "lora")]
use raylar_drivers::radio::{
    LoRaBandwidth, LoRaChannel, LoRaCodingRate, LoRaHeaderMode, LoRaSpreadingFactor,
    LowDataRateOptimization,
};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 8 * 1024;
const PACKET_LEN: usize = 12;
const RX_START_GUARD: Duration = Duration::from_millis(20);
const MIN_TX_INTERVAL_SECS: u64 = 3;
const TX_INTERVAL_SPAN_SECS: u32 = 15; // Inclusive range 3..=17; mean 10 seconds.

#[cfg(feature = "channel-868")]
const FREQUENCY_HZ: u32 = 868_000_000;
#[cfg(feature = "channel-915")]
const FREQUENCY_HZ: u32 = 915_000_000;
#[cfg(feature = "channel-2445")]
const FREQUENCY_HZ: u32 = 2_445_000_000;

#[cfg(any(feature = "channel-868", feature = "channel-915"))]
const TX_POWER_DBM: i8 = 14;
#[cfg(feature = "channel-2445")]
const TX_POWER_DBM: i8 = 10;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
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

    let peripherals = embassy_stm32::init(config);
    let Board { ebyte_rf, leds, .. } = Board::new(peripherals);

    let identity = identity::init();
    let device_id = identity.serial_64();
    let channel = selected_channel();
    let mode_name = selected_mode_name();

    info!(
        "Radio driver test: mode={} frequency_hz={} device_id={=u64:#x}",
        mode_name, FREQUENCY_HZ, device_id
    );

    run_radio_test(ebyte_rf, leds, channel, device_id).await
}

async fn run_radio_test(
    rf: EbyteRf<'static>,
    leds: Leds<'static>,
    channel: ChannelConfig,
    device_id: u64,
) -> ! {
    let EbyteRf {
        spi,
        sck,
        miso,
        mosi,
        cs,
        busy,
        nrst,
        irq,
    } = rf;
    let Leds {
        mut sys_main_green,
        mut sys_sd_blue,
        ..
    } = leds;

    let mut spi_config = SpiConfig::default();
    spi_config.frequency = mhz(1);
    let spi = Spi::new_blocking(spi, sck, mosi, miso, spi_config);
    let spi_device = ManualCsSpiDevice::new(spi, cs);
    let timing = DriverTiming {
        preparation_guard: Duration::from_millis(10),
        ..DriverTiming::default()
    };
    let mut radio = RadioDriver::with_timing(spi_device, busy, nrst, irq, timing);

    if let Err(error) = radio.initialize().await {
        error!("Radio initialization failed: {:?}", error);
        fail_forever().await;
    }
    if let Err(error) = radio.prepare_channel(&channel).await {
        error!("Radio channel preparation failed: {:?}", error);
        fail_forever().await;
    }

    info!("Radio initialized in STBY_XOSC; entering continuous receive loop");

    let mut random = XorShift32::new((device_id as u32) ^ (device_id >> 32) as u32);
    let mut counter = 0u32;
    let mut next_tx = Instant::now() + random_tx_interval(&mut random);
    let mut receive_buffer = [0u8; PACKET_LEN];

    loop {
        let now = Instant::now();
        if now + RX_START_GUARD < next_tx {
            let rx_start = now + RX_START_GUARD;
            match radio
                .receive_at(rx_start, next_tx, &mut receive_buffer)
                .await
            {
                Ok(packet) => {
                    sys_main_green.set_high();
                    report_received_packet(packet.payload, packet.metadata);
                    Timer::after_millis(30).await;
                    sys_main_green.set_low();
                }
                Err(RadioError::RxTimeout { .. }) => {}
                Err(error) => {
                    warn!("RX failed: {:?}; attempting recovery", error);
                    recover_and_prepare(&mut radio, &channel).await;
                }
            }
            continue;
        }

        let payload = make_payload(device_id, counter);
        let tx = TxConfig {
            power_dbm: TX_POWER_DBM,
            ramp_time: TxRampTime::Us48,
        };
        let tx_start = Instant::now() + RX_START_GUARD;
        sys_sd_blue.set_high();
        match radio.transmit_at(tx_start, &payload, &tx).await {
            Ok(report) => {
                info!(
                    "TX device={=u64:#x} counter={} requested_tick={} command_tick={} done_tick={} command_latency_us={}",
                    device_id,
                    counter,
                    report.requested_start.as_ticks(),
                    report.command_started_at.as_ticks(),
                    report.tx_done_at.as_ticks(),
                    report
                        .command_completed_at
                        .duration_since(report.command_started_at)
                        .as_micros()
                );
                counter = counter.wrapping_add(1);
            }
            Err(error) => {
                warn!("TX failed for counter={}: {:?}", counter, error);
                recover_and_prepare(&mut radio, &channel).await;
            }
        }
        sys_sd_blue.set_low();
        next_tx = Instant::now() + random_tx_interval(&mut random);
        info!("Next TX scheduled at system_tick={}", next_tx.as_ticks());
    }
}

fn report_received_packet(payload: &[u8], metadata: raylar_drivers::radio::RxMetadata) {
    let observed_at = Instant::now();
    if payload.len() == PACKET_LEN {
        let sender = u64::from_be_bytes(payload[..8].try_into().unwrap());
        let counter = u32::from_be_bytes(payload[8..].try_into().unwrap());
        info!(
            "RX application packet sender={=u64:#x} counter={}",
            sender, counter
        );
    }
    match metadata.metrics {
        RxMetrics::LoRa {
            rssi_dbm_x2,
            signal_rssi_dbm_x2,
            snr_db_x4,
        } => info!(
            "RX LoRa bytes={=[u8]} frequency_hz={} complete_tick={} observed_tick={} report_latency_us={} rssi_x2={} signal_rssi_x2={} snr_x4={}",
            payload,
            metadata.frequency_hz,
            metadata.packet_complete_at.as_ticks(),
            observed_at.as_ticks(),
            observed_at.duration_since(metadata.packet_complete_at).as_micros(),
            rssi_dbm_x2,
            signal_rssi_dbm_x2,
            snr_db_x4
        ),
        RxMetrics::Gfsk {
            rssi_dbm_x2,
            status,
        } => info!(
            "RX GFSK bytes={=[u8]} frequency_hz={} complete_tick={} observed_tick={} report_latency_us={} rssi_x2={} sync_rssi_x2={} len_err={} crc_err={} abort_err={} address_err={} sync_err={}",
            payload,
            metadata.frequency_hz,
            metadata.packet_complete_at.as_ticks(),
            observed_at.as_ticks(),
            observed_at.duration_since(metadata.packet_complete_at).as_micros(),
            rssi_dbm_x2,
            status.sync_rssi_dbm_x2,
            status.length_error,
            status.crc_error,
            status.abort_error,
            status.address_error,
            status.sync_error
        ),
    }
}

async fn recover_and_prepare<SPI, BUSY, RESET, IRQ>(
    radio: &mut RadioDriver<SPI, BUSY, RESET, IRQ>,
    channel: &ChannelConfig,
) where
    SPI: embedded_hal_async::spi::SpiDevice<u8>,
    BUSY: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
    RESET: embedded_hal::digital::OutputPin,
    IRQ: embedded_hal::digital::InputPin + embedded_hal_async::digital::Wait,
{
    if let Err(error) = radio.recover().await {
        error!("Radio recovery failed: {:?}", error);
        fail_forever().await;
    }
    if let Err(error) = radio.prepare_channel(channel).await {
        error!("Channel preparation after recovery failed: {:?}", error);
        fail_forever().await;
    }
}

fn make_payload(device_id: u64, counter: u32) -> [u8; PACKET_LEN] {
    let mut payload = [0u8; PACKET_LEN];
    payload[..8].copy_from_slice(&device_id.to_be_bytes());
    payload[8..].copy_from_slice(&counter.to_be_bytes());
    payload
}

fn random_tx_interval(random: &mut XorShift32) -> Duration {
    let seconds = MIN_TX_INTERVAL_SECS + u64::from(random.next() % TX_INTERVAL_SPAN_SECS);
    Duration::from_secs(seconds)
}

struct XorShift32 {
    state: u32,
}

impl XorShift32 {
    fn new(seed: u32) -> Self {
        Self {
            state: if seed == 0 { 0xA341_316C } else { seed },
        }
    }

    fn next(&mut self) -> u32 {
        let mut value = self.state;
        value ^= value << 13;
        value ^= value >> 17;
        value ^= value << 5;
        self.state = value;
        value
    }
}

#[cfg(feature = "lora")]
fn selected_channel() -> ChannelConfig {
    ChannelConfig {
        frequency_hz: FREQUENCY_HZ,
        modulation: ModulationConfig::LoRa(LoRaChannel {
            spreading_factor: LoRaSpreadingFactor::Sf7,
            bandwidth: if FREQUENCY_HZ >= 2_400_000_000 {
                LoRaBandwidth::Khz406
            } else {
                LoRaBandwidth::Khz125
            },
            coding_rate: LoRaCodingRate::Cr4_5,
            low_data_rate_optimization: LowDataRateOptimization::Auto,
            preamble_symbols: 12,
            header: LoRaHeaderMode::Explicit,
            payload_length: Some(PACKET_LEN as u8),
            crc: true,
            invert_iq: false,
            sync_word: 0x12,
        }),
        rx_boosted: false,
    }
}

#[cfg(feature = "gfsk")]
fn selected_channel() -> ChannelConfig {
    let mut sync_word = heapless::Vec::new();
    sync_word.extend_from_slice(&[0x2D, 0xD4]).unwrap();
    ChannelConfig {
        frequency_hz: FREQUENCY_HZ,
        modulation: ModulationConfig::Gfsk(GfskChannel {
            bit_rate_bps: if FREQUENCY_HZ >= 2_400_000_000 {
                38_400
            } else {
                50_000
            },
            frequency_deviation_hz: if FREQUENCY_HZ >= 2_400_000_000 {
                40_000
            } else {
                25_000
            },
            receiver_bandwidth: if FREQUENCY_HZ >= 2_400_000_000 {
                GfskBandwidth::Hz156_200
            } else {
                GfskBandwidth::Hz117_300
            },
            pulse_shape: GfskPulseShape::GaussianBt0_5,
            preamble_bits: 32,
            preamble_detector: GfskPreambleDetector::Bits16,
            sync_word,
            address_filtering: GfskAddressFiltering::Disabled,
            packet_length: GfskPacketLength::Fixed(PACKET_LEN as u8),
            crc: GfskCrc::TwoBytes {
                initial: 0xFFFF,
                polynomial: 0x1021,
                inverted: false,
            },
            whitening: Some(GfskWhitening { initial: 0x01FF }),
        }),
        rx_boosted: false,
    }
}

#[cfg(feature = "lora")]
fn selected_mode_name() -> &'static str {
    "LoRa"
}

#[cfg(feature = "gfsk")]
fn selected_mode_name() -> &'static str {
    "GFSK"
}

async fn fail_forever() -> ! {
    loop {
        Timer::after_secs(60).await;
    }
}
