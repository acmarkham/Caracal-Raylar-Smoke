//! LR1121 command transport and Ebyte E80 module-specific wiring constants.

use arbitrary_int::{u24, u31};
use embedded_hal::digital::InputPin;
use embedded_hal_async::{
    digital::Wait,
    spi::{Operation, SpiDevice},
};
use lr11xx::ops;

use super::{
    config::{
        ChannelConfig, GfskAddressFiltering, GfskBandwidth, GfskChannel, GfskCrc, GfskPacketLength,
        GfskPulseShape, LoRaBandwidth, LoRaChannel, LoRaCodingRate, LoRaHeaderMode,
        LoRaSpreadingFactor, ModulationConfig, RadioBand, TxConfig, TxRampTime,
    },
    error::TransportError,
    packet::{GfskPacketStatus, RxMetrics},
};

// Values proven by unitsmoke/15_ebyte_crate and unitsmoke/16_ebyte_crate_rx.
const RF_SWITCH_BYTES: [u8; 8] = [0x0F, 0x00, 0x02, 0x03, 0x01, 0x00, 0x04, 0x08];
const TCXO_1V8_AND_9P8_MS: [u8; 4] = [0x02, 0x00, 0x01, 0x40];

const GET_STATUS: u16 = 0x0100;
const GET_VERSION: u16 = 0x0101;
const CLEAR_ERRORS: u16 = 0x010E;
const CALIBRATE: u16 = 0x010F;
const SET_REG_MODE: u16 = 0x0110;
const CALIB_IMAGE: u16 = 0x0111;
const SET_RF_SWITCH: u16 = 0x0112;
const SET_DIO_IRQ: u16 = 0x0113;
const CLEAR_IRQ: u16 = 0x0114;
const CONFIG_LF_CLOCK: u16 = 0x0116;
const SET_TCXO_MODE: u16 = 0x0117;
const SET_SLEEP: u16 = 0x011B;
const SET_STANDBY: u16 = 0x011C;
const WRITE_BUFFER: u16 = 0x0109;
const READ_BUFFER: u16 = 0x010A;
const GET_RX_BUFFER_STATUS: u16 = 0x0203;
const GET_PACKET_STATUS: u16 = 0x0204;
const SET_GFSK_SYNC_WORD: u16 = 0x0206;
const SET_RX: u16 = 0x0209;
const SET_TX: u16 = 0x020A;
const SET_RF_FREQUENCY: u16 = 0x020B;
const SET_PACKET_TYPE: u16 = 0x020E;
const SET_MODULATION: u16 = 0x020F;
const SET_PACKET: u16 = 0x0210;
const SET_TX_PARAMS: u16 = 0x0211;
const SET_PACKET_ADDRESS: u16 = 0x0212;
const SET_FALLBACK_MODE: u16 = 0x0213;
const SET_PA_CONFIG: u16 = 0x0215;
const SET_GFSK_CRC: u16 = 0x0224;
const SET_GFSK_WHITENING: u16 = 0x0225;
const SET_RX_BOOSTED: u16 = 0x0227;
const SET_LORA_SYNC_WORD: u16 = 0x022B;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BackendError {
    Transport(TransportError),
    Command,
    Device,
    Calibration,
}

pub(crate) struct EbyteE80<SPI, BUSY> {
    spi: SPI,
    busy: BUSY,
}

impl<SPI, BUSY> EbyteE80<SPI, BUSY>
where
    SPI: SpiDevice<u8>,
    BUSY: InputPin + Wait,
{
    pub(crate) fn new(spi: SPI, busy: BUSY) -> Self {
        Self { spi, busy }
    }

    pub(crate) async fn wait_ready(&mut self) -> Result<(), BackendError> {
        self.wait_busy().await
    }

    pub(crate) async fn initialize(&mut self) -> Result<(), BackendError> {
        let mut status = [0u8; 6];
        self.transfer_command(GET_STATUS, &mut status).await?;

        let mut version = [0u8; 4];
        self.read_command(GET_VERSION, &mut version).await?;
        // Response layout is HW, use-case, FW major, FW minor.
        if version[1] != 0x03 {
            return Err(BackendError::Device);
        }

        self.write_command(CLEAR_ERRORS, &[]).await?;
        self.standby(false).await?;
        self.write_command(SET_REG_MODE, &[1]).await?;
        self.write_command(SET_RF_SWITCH, &RF_SWITCH_BYTES).await?;
        self.write_command(SET_TCXO_MODE, &TCXO_1V8_AND_9P8_MS)
            .await?;
        // Crystal LF clock, wait for it before completing the command.
        self.write_command(CONFIG_LF_CLOCK, &[0x05]).await?;
        self.write_command(CALIBRATE, &[0x3F])
            .await
            .map_err(calibration_error)?;
        self.write_command(CLEAR_ERRORS, &[]).await?;
        self.clear_irq(u32::MAX).await?;
        self.write_command(SET_FALLBACK_MODE, &[0x02]).await?;
        self.standby(true).await
    }

    pub(crate) async fn standby(&mut self, xosc: bool) -> Result<(), BackendError> {
        self.write_command(SET_STANDBY, &[u8::from(xosc)]).await
    }

    pub(crate) async fn sleep_retained(&mut self) -> Result<(), BackendError> {
        // Retention enabled, automatic wake disabled, no RTC wake timeout.
        self.write_command(SET_SLEEP, &[0x01, 0, 0, 0, 0]).await
    }

    pub(crate) async fn apply_channel(
        &mut self,
        channel: &ChannelConfig,
        band: RadioBand,
    ) -> Result<(), BackendError> {
        self.standby(true).await?;
        self.clear_irq(u32::MAX).await?;

        if band == RadioBand::SubGhz {
            self.write_command(CALIB_IMAGE, &image_calibration_bytes(channel.frequency_hz))
                .await
                .map_err(calibration_error)?;
        }
        self.write_command(SET_RF_FREQUENCY, &channel.frequency_hz.to_be_bytes())
            .await?;
        self.write_command(SET_RX_BOOSTED, &[u8::from(channel.rx_boosted)])
            .await?;

        match &channel.modulation {
            ModulationConfig::LoRa(config) => self.apply_lora(config).await?,
            ModulationConfig::Gfsk(config) => self.apply_gfsk(config).await?,
        }
        self.write_command(SET_FALLBACK_MODE, &[0x02]).await?;
        self.standby(true).await
    }

    async fn apply_lora(&mut self, config: &LoRaChannel) -> Result<(), BackendError> {
        self.write_command(SET_PACKET_TYPE, &[ops::PacketType::LoRa.raw_value()])
            .await?;
        let modulation = ops::LoRaModulation::builder()
            .with_sf(map_lora_sf(config.spreading_factor))
            .with_bwl(map_lora_bw(config.bandwidth))
            .with_cr(map_lora_cr(config.coding_rate))
            .with_low_data_rate_optimize(config.low_data_rate_optimization_enabled())
            .build();
        self.write_command(SET_MODULATION, &modulation.raw_value().to_be_bytes())
            .await?;
        self.write_command(SET_LORA_SYNC_WORD, &[config.sync_word])
            .await?;
        self.write_lora_packet(config, config.payload_length.unwrap_or(u8::MAX))
            .await
    }

    async fn apply_gfsk(&mut self, config: &GfskChannel) -> Result<(), BackendError> {
        self.write_command(SET_PACKET_TYPE, &[ops::PacketType::Gfsk.raw_value()])
            .await?;
        let modulation = ops::GfskModulation::builder()
            .with_fdev(config.frequency_deviation_hz)
            .with_bandwidth(map_gfsk_bw(config.receiver_bandwidth))
            .with_shape(map_gfsk_shape(config.pulse_shape))
            .with_bitrate(u31::new(config.bit_rate_bps))
            .with_fractional(false)
            .build();
        self.write_command(SET_MODULATION, &modulation.raw_value().to_be_bytes())
            .await?;

        let mut sync_word = [0u8; 8];
        sync_word[..config.sync_word.len()].copy_from_slice(&config.sync_word);
        self.write_command(SET_GFSK_SYNC_WORD, &sync_word).await?;

        let (node, broadcast) = match config.address_filtering {
            GfskAddressFiltering::Disabled => (0, 0),
            GfskAddressFiltering::Node { node } => (node, 0),
            GfskAddressFiltering::NodeAndBroadcast { node, broadcast } => (node, broadcast),
        };
        self.write_command(SET_PACKET_ADDRESS, &[node, broadcast])
            .await?;

        if let GfskCrc::OneByte {
            initial,
            polynomial,
            ..
        }
        | GfskCrc::TwoBytes {
            initial,
            polynomial,
            ..
        } = config.crc
        {
            let mut bytes = [0u8; 8];
            bytes[..4].copy_from_slice(&initial.to_be_bytes());
            bytes[4..].copy_from_slice(&polynomial.to_be_bytes());
            self.write_command(SET_GFSK_CRC, &bytes).await?;
        }
        if let Some(whitening) = config.whitening {
            self.write_command(SET_GFSK_WHITENING, &whitening.initial.to_be_bytes())
                .await?;
        }
        self.write_gfsk_packet(config, config.packet_length.maximum())
            .await
    }

    pub(crate) async fn configure_payload(
        &mut self,
        channel: &ChannelConfig,
        payload_len: u8,
    ) -> Result<(), BackendError> {
        match &channel.modulation {
            ModulationConfig::LoRa(config) => self.write_lora_packet(config, payload_len).await,
            ModulationConfig::Gfsk(config) => self.write_gfsk_packet(config, payload_len).await,
        }
    }

    async fn write_lora_packet(
        &mut self,
        config: &LoRaChannel,
        payload_len: u8,
    ) -> Result<(), BackendError> {
        let packet = ops::LoRaPacket::builder()
            .with_preamble_length(config.preamble_symbols)
            .with_header_implicit(config.header == LoRaHeaderMode::Implicit)
            .with_payload_length(payload_len)
            .with_crc(config.crc)
            .with_invert_iq(config.invert_iq)
            .build();
        self.write_command(SET_PACKET, &packet.raw_value().to_be_bytes())
            .await
    }

    async fn write_gfsk_packet(
        &mut self,
        config: &GfskChannel,
        payload_len: u8,
    ) -> Result<(), BackendError> {
        let packet = ops::GfskPacket::builder()
            .with_preamble_length_tx(config.preamble_bits)
            .with_preamble_detect(match config.preamble_detector {
                super::config::GfskPreambleDetector::Off => 0,
                super::config::GfskPreambleDetector::Bits8 => 4,
                super::config::GfskPreambleDetector::Bits16 => 5,
                super::config::GfskPreambleDetector::Bits24 => 6,
                super::config::GfskPreambleDetector::Bits32 => 7,
            })
            .with_sync_word_len((config.sync_word.len() * 8) as u8)
            .with_addr_filter(match config.address_filtering {
                GfskAddressFiltering::Disabled => ops::AddressFilter::None,
                GfskAddressFiltering::Node { .. } => ops::AddressFilter::RxTx,
                GfskAddressFiltering::NodeAndBroadcast { .. } => ops::AddressFilter::RxTxBroadcast,
            })
            .with_packet_type(match config.packet_length {
                GfskPacketLength::Fixed(_) => ops::GfskPacketType::Known,
                GfskPacketLength::Variable { .. } => ops::GfskPacketType::Variable,
                GfskPacketLength::VariableSx128x { .. } => ops::GfskPacketType::VariableSx128x,
            })
            .with_payload_len(payload_len)
            .with_crc_type(map_gfsk_crc(config.crc))
            .with_whitening(config.whitening.is_some())
            .build();
        self.write_command(SET_PACKET, &packet.raw_value().to_be_bytes())
            .await
    }

    pub(crate) async fn configure_tx(
        &mut self,
        tx: TxConfig,
        band: RadioBand,
    ) -> Result<(), BackendError> {
        let pa = match band {
            RadioBand::SubGhz if tx.power_dbm > 14 => [0x01, 0x01, 0x04, 0x07],
            RadioBand::SubGhz => [0x00, 0x00, 0x04, 0x00],
            // Value proven by unitsmoke/13_ebyte_rx_tx at 2.445 GHz.
            RadioBand::Ghz2_4 => [0x02, 0x00, 0x00, 0x00],
        };
        self.write_command(SET_PA_CONFIG, &pa).await?;
        self.write_command(SET_TX_PARAMS, &[tx.power_dbm as u8, map_ramp(tx.ramp_time)])
            .await
    }

    pub(crate) async fn set_irq_mask(&mut self, mask: u32) -> Result<(), BackendError> {
        let mut bytes = [0u8; 8];
        bytes[..4].copy_from_slice(&mask.to_be_bytes());
        self.write_command(SET_DIO_IRQ, &bytes).await
    }

    pub(crate) async fn clear_irq(&mut self, mask: u32) -> Result<(), BackendError> {
        self.write_command(CLEAR_IRQ, &mask.to_be_bytes()).await
    }

    pub(crate) async fn irq_status(&mut self) -> Result<u32, BackendError> {
        let mut response = [0u8; 6];
        self.transfer_command(GET_STATUS, &mut response).await?;
        Ok(u32::from_be_bytes(response[2..6].try_into().unwrap()))
    }

    pub(crate) async fn write_payload(&mut self, payload: &[u8]) -> Result<(), BackendError> {
        self.write_command(WRITE_BUFFER, payload).await
    }

    pub(crate) async fn start_rx(&mut self, ticks: u32) -> Result<(), BackendError> {
        self.write_command(SET_RX, &u24::new(ticks).to_be_bytes())
            .await
    }

    pub(crate) async fn start_tx(&mut self, ticks: u32) -> Result<(), BackendError> {
        self.write_command(SET_TX, &u24::new(ticks).to_be_bytes())
            .await
    }

    pub(crate) async fn read_packet<'a>(
        &mut self,
        buffer: &'a mut [u8],
        lora: bool,
    ) -> Result<(&'a [u8], RxMetrics), BackendError> {
        let mut status = [0u8; 2];
        self.read_command(GET_RX_BUFFER_STATUS, &mut status).await?;
        let length = usize::from(status[0]);
        if length > buffer.len() {
            return Err(BackendError::Device);
        }
        let args = [status[1], status[0]];
        self.write_command(READ_BUFFER, &args).await?;
        self.read_response(&mut buffer[..length]).await?;

        let metrics = if lora {
            let mut raw = [0u8; 3];
            self.read_command(GET_PACKET_STATUS, &mut raw).await?;
            RxMetrics::LoRa {
                rssi_dbm_x2: -i16::from(raw[0]),
                snr_db_x4: i16::from(raw[1] as i8),
                signal_rssi_dbm_x2: -i16::from(raw[2]),
            }
        } else {
            let mut raw = [0u8; 3];
            self.read_command(GET_PACKET_STATUS, &mut raw).await?;
            RxMetrics::Gfsk {
                rssi_dbm_x2: -i16::from(raw[1]),
                status: GfskPacketStatus {
                    sync_rssi_dbm_x2: -i16::from(raw[0]),
                    length_error: raw[2] & 0x10 != 0,
                    crc_error: raw[2] & 0x08 != 0,
                    abort_error: raw[2] & 0x04 != 0,
                    address_error: raw[2] & 0x02 != 0,
                    sync_error: raw[2] & 0x01 != 0,
                },
            }
        };
        Ok((&buffer[..length], metrics))
    }

    async fn write_command(&mut self, opcode: u16, args: &[u8]) -> Result<(), BackendError> {
        let command = opcode.to_be_bytes();
        let mut status = [0u8; 2];
        self.spi
            .transaction(&mut [
                Operation::Transfer(&mut status, &command),
                Operation::Write(args),
            ])
            .await
            .map_err(|_| BackendError::Transport(TransportError::Spi))?;
        self.wait_busy().await?;
        check_command_status(status[0])
    }

    async fn read_command(&mut self, opcode: u16, response: &mut [u8]) -> Result<(), BackendError> {
        self.write_command(opcode, &[]).await?;
        self.read_response(response).await
    }

    async fn read_response(&mut self, response: &mut [u8]) -> Result<(), BackendError> {
        let mut status = [0u8; 1];
        self.spi
            .transaction(&mut [
                Operation::Transfer(&mut status, &[0]),
                Operation::Read(response),
            ])
            .await
            .map_err(|_| BackendError::Transport(TransportError::Spi))?;
        self.wait_busy().await?;
        check_command_status(status[0])
    }

    async fn transfer_command(
        &mut self,
        opcode: u16,
        response: &mut [u8],
    ) -> Result<(), BackendError> {
        let command = opcode.to_be_bytes();
        self.spi
            .transaction(&mut [Operation::Transfer(response, &command)])
            .await
            .map_err(|_| BackendError::Transport(TransportError::Spi))?;
        self.wait_busy().await
    }

    async fn wait_busy(&mut self) -> Result<(), BackendError> {
        self.busy
            .wait_for_low()
            .await
            .map_err(|_| BackendError::Transport(TransportError::Busy))
    }
}

fn check_command_status(stat1: u8) -> Result<(), BackendError> {
    match (stat1 >> 1) & 0x07 {
        2 | 3 => Ok(()),
        0 | 1 => Err(BackendError::Command),
        _ => Err(BackendError::Device),
    }
}

fn calibration_error(error: BackendError) -> BackendError {
    match error {
        BackendError::Transport(error) => BackendError::Transport(error),
        _ => BackendError::Calibration,
    }
}

fn image_calibration_bytes(frequency_hz: u32) -> [u8; 2] {
    let (start, end) = match frequency_hz {
        430_000_000..=440_000_000 => (430_000_000, 440_000_000),
        470_000_000..=510_000_000 => (470_000_000, 510_000_000),
        779_000_000..=787_000_000 => (779_000_000, 787_000_000),
        863_000_000..=870_000_000 => (863_000_000, 870_000_000),
        902_000_000..=928_000_000 => (902_000_000, 928_000_000),
        value => (
            value.saturating_sub(2_000_000),
            value.saturating_add(2_000_000),
        ),
    };
    [
        ((start - 1_000_000) / 4_000_000) as u8,
        ((end / 4_000_000) + 1) as u8,
    ]
}

fn map_lora_sf(value: LoRaSpreadingFactor) -> ops::SpreadingFactor {
    match value {
        LoRaSpreadingFactor::Sf5 => ops::SpreadingFactor::SF5,
        LoRaSpreadingFactor::Sf6 => ops::SpreadingFactor::SF6,
        LoRaSpreadingFactor::Sf7 => ops::SpreadingFactor::SF7,
        LoRaSpreadingFactor::Sf8 => ops::SpreadingFactor::SF8,
        LoRaSpreadingFactor::Sf9 => ops::SpreadingFactor::SF9,
        LoRaSpreadingFactor::Sf10 => ops::SpreadingFactor::SF10,
        LoRaSpreadingFactor::Sf11 => ops::SpreadingFactor::SF11,
        LoRaSpreadingFactor::Sf12 => ops::SpreadingFactor::SF12,
    }
}

fn map_lora_bw(value: LoRaBandwidth) -> ops::LoRaBandwidth {
    match value {
        LoRaBandwidth::Khz62_5 => ops::LoRaBandwidth::KHz62,
        LoRaBandwidth::Khz125 => ops::LoRaBandwidth::KHz125,
        LoRaBandwidth::Khz250 => ops::LoRaBandwidth::KHz250,
        LoRaBandwidth::Khz500 => ops::LoRaBandwidth::KHz500,
    }
}

fn map_lora_cr(value: LoRaCodingRate) -> ops::CodingRate {
    match value {
        LoRaCodingRate::Cr4_5 => ops::CodingRate::Short45,
        LoRaCodingRate::Cr4_6 => ops::CodingRate::Short46,
        LoRaCodingRate::Cr4_7 => ops::CodingRate::Short47,
        LoRaCodingRate::Cr4_8 => ops::CodingRate::Short48,
        LoRaCodingRate::LongInterleaver4_5 => ops::CodingRate::Long45,
        LoRaCodingRate::LongInterleaver4_6 => ops::CodingRate::Long46,
        LoRaCodingRate::LongInterleaver4_8 => ops::CodingRate::Long48,
    }
}

fn map_gfsk_bw(value: GfskBandwidth) -> ops::GfskBandwidth {
    use GfskBandwidth::*;
    match value {
        Hz4_800 => ops::GfskBandwidth::Hz4800,
        Hz5_800 => ops::GfskBandwidth::Hz5800,
        Hz7_300 => ops::GfskBandwidth::Hz7300,
        Hz9_700 => ops::GfskBandwidth::Hz9700,
        Hz11_700 => ops::GfskBandwidth::Hz11700,
        Hz14_600 => ops::GfskBandwidth::Hz14600,
        Hz19_500 => ops::GfskBandwidth::Hz19500,
        Hz23_400 => ops::GfskBandwidth::Hz23400,
        Hz29_300 => ops::GfskBandwidth::Hz29300,
        Hz39_000 => ops::GfskBandwidth::Hz39000,
        Hz46_900 => ops::GfskBandwidth::Hz46900,
        Hz58_600 => ops::GfskBandwidth::Hz58600,
        Hz78_200 => ops::GfskBandwidth::Hz78200,
        Hz93_800 => ops::GfskBandwidth::Hz93800,
        Hz117_300 => ops::GfskBandwidth::Hz117300,
        Hz156_200 => ops::GfskBandwidth::Hz156200,
        Hz187_200 => ops::GfskBandwidth::Hz187200,
        Hz234_300 => ops::GfskBandwidth::Hz234300,
        Hz312_000 => ops::GfskBandwidth::Hz312000,
        Hz373_600 => ops::GfskBandwidth::Hz373600,
        Hz467_000 => ops::GfskBandwidth::Hz467000,
    }
}

fn map_gfsk_shape(value: GfskPulseShape) -> ops::GfskShape {
    match value {
        GfskPulseShape::None => ops::GfskShape::None,
        GfskPulseShape::GaussianBt0_3 => ops::GfskShape::GaussianBt03,
        GfskPulseShape::GaussianBt0_5 => ops::GfskShape::GaussianBt05,
        GfskPulseShape::GaussianBt0_7 => ops::GfskShape::GaussianBt07,
        GfskPulseShape::GaussianBt1 => ops::GfskShape::GaussianBt1,
        GfskPulseShape::RaisedCosineBt0_7 => ops::GfskShape::RaisedCosineBt07,
    }
}

fn map_gfsk_crc(value: GfskCrc) -> ops::CrcType {
    match value {
        GfskCrc::Off => ops::CrcType::Off,
        GfskCrc::OneByte {
            inverted: false, ..
        } => ops::CrcType::OneByte,
        GfskCrc::OneByte { inverted: true, .. } => ops::CrcType::OneByteInverted,
        GfskCrc::TwoBytes {
            inverted: false, ..
        } => ops::CrcType::TwoBytes,
        GfskCrc::TwoBytes { inverted: true, .. } => ops::CrcType::TwoBytesInverted,
    }
}

fn map_ramp(value: TxRampTime) -> u8 {
    match value {
        TxRampTime::Us16 => 0,
        TxRampTime::Us32 => 1,
        TxRampTime::Us48 => 2,
        TxRampTime::Us64 => 3,
        TxRampTime::Us80 => 4,
        TxRampTime::Us96 => 5,
        TxRampTime::Us112 => 6,
        TxRampTime::Us128 => 7,
        TxRampTime::Us144 => 8,
        TxRampTime::Us160 => 9,
        TxRampTime::Us176 => 10,
        TxRampTime::Us192 => 11,
        TxRampTime::Us208 => 12,
        TxRampTime::Us240 => 13,
        TxRampTime::Us272 => 14,
        TxRampTime::Us304 => 15,
    }
}
