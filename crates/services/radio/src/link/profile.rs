use raylar_drivers::radio::{
    ChannelConfig, GfskAddressFiltering, GfskBandwidth, GfskChannel, GfskCrc, GfskPacketLength,
    GfskPreambleDetector, GfskPulseShape, GfskWhitening, LoRaBandwidth, LoRaChannel,
    LoRaCodingRate, LoRaHeaderMode, LoRaSpreadingFactor, LowDataRateOptimization, ModulationConfig,
    RadioBand, TxConfig, TxRampTime,
};

use crate::LinkError;

pub const PHASE_ONE_BOOTSTRAP_PROFILE_ID: ProfileId = ProfileId(1);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct ProfileId(pub u8);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ProfileBand {
    SubGhz,
    Ghz2_4,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpreadingFactor {
    Sf5,
    Sf6,
    Sf7,
    Sf8,
    Sf9,
    Sf10,
    Sf11,
    Sf12,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodingRate {
    Cr4_5,
    Cr4_6,
    Cr4_7,
    Cr4_8,
    LongInterleaver4_5,
    LongInterleaver4_6,
    LongInterleaver4_8,
}

/// Bitrate and deviation pairs from LR1121 datasheet Table 3-9 (2-FSK RX
/// conditions). GFSK uses Gaussian shaping, so the table's sensitivities do
/// not apply to these profiles without measurement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GfskReferenceRate {
    Bps1_200,
    Bps4_800,
    Bps38_400,
    Bps250_000,
}

impl GfskReferenceRate {
    const fn parameters(self) -> (u32, u32, GfskBandwidth) {
        match self {
            Self::Bps1_200 => (1_200, 5_000, GfskBandwidth::Hz19_500),
            Self::Bps4_800 => (4_800, 5_000, GfskBandwidth::Hz19_500),
            Self::Bps38_400 => (38_400, 40_000, GfskBandwidth::Hz156_200),
            // Table 3-9 calls this nominally 500 kHz; 467 kHz is the
            // largest actual SetModulationParams RX filter setting.
            Self::Bps250_000 => (250_000, 125_000, GfskBandwidth::Hz467_000),
        }
    }
}

/// A complete, policy-approved Phase I PHY profile.
///
/// Construction validates hardware-safe values. Product code is still
/// responsible for supplying only regionally approved profiles to the static
/// estimator.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelProfile {
    id: ProfileId,
    band: ProfileBand,
    channel: ChannelConfig,
    tx: TxConfig,
}

impl ChannelProfile {
    /// Interoperable Phase I bootstrap profile for the repository's default
    /// EU868 build. Product radio policy must confirm the fitted module and
    /// deployment region before enabling it in shipped firmware.
    pub fn phase_one_eu868_bootstrap() -> Self {
        Self {
            id: PHASE_ONE_BOOTSTRAP_PROFILE_ID,
            band: ProfileBand::SubGhz,
            channel: ChannelConfig {
                frequency_hz: 868_000_000,
                modulation: ModulationConfig::LoRa(LoRaChannel {
                    spreading_factor: LoRaSpreadingFactor::Sf9,
                    bandwidth: LoRaBandwidth::Khz125,
                    coding_rate: LoRaCodingRate::Cr4_5,
                    low_data_rate_optimization: LowDataRateOptimization::Auto,
                    preamble_symbols: 12,
                    header: LoRaHeaderMode::Explicit,
                    payload_length: None,
                    crc: true,
                    invert_iq: false,
                    sync_word: 0x12,
                }),
                rx_boosted: false,
            },
            tx: TxConfig {
                power_dbm: 14,
                ramp_time: TxRampTime::Us48,
            },
        }
    }

    /// Standard explicit-header LoRa profile. Long-interleaver coding rates
    /// need a known payload length; use `lora_config` for those profiles.
    #[allow(clippy::too_many_arguments)]
    pub fn lora(
        id: ProfileId,
        frequency_hz: u32,
        spreading_factor: SpreadingFactor,
        bandwidth_hz: u32,
        coding_rate: CodingRate,
        tx_power_dbm: i8,
        sync_word: u8,
    ) -> Result<Self, LinkError> {
        let bandwidth = match bandwidth_hz {
            62_500 => LoRaBandwidth::Khz62_5,
            125_000 => LoRaBandwidth::Khz125,
            250_000 => LoRaBandwidth::Khz250,
            500_000 => LoRaBandwidth::Khz500,
            203_000 => LoRaBandwidth::Khz203,
            406_000 => LoRaBandwidth::Khz406,
            812_000 => LoRaBandwidth::Khz812,
            _ => return Err(LinkError::InvalidProfile),
        };
        let channel = ChannelConfig {
            frequency_hz,
            modulation: ModulationConfig::LoRa(LoRaChannel {
                spreading_factor: map_sf(spreading_factor),
                bandwidth,
                coding_rate: map_cr(coding_rate),
                low_data_rate_optimization: LowDataRateOptimization::Auto,
                preamble_symbols: 12,
                header: LoRaHeaderMode::Explicit,
                payload_length: None,
                crc: true,
                invert_iq: false,
                sync_word,
            }),
            rx_boosted: false,
        };
        let tx = TxConfig {
            power_dbm: tx_power_dbm,
            ramp_time: TxRampTime::Us48,
        };
        Self::from_channel(id, channel, tx)
    }

    /// Use this form when the LoRa packet contract differs from the default,
    /// including the known payload length required by long-interleaver CRs.
    pub fn lora_config(
        id: ProfileId,
        frequency_hz: u32,
        modulation: LoRaChannel,
        rx_boosted: bool,
        tx_power_dbm: i8,
    ) -> Result<Self, LinkError> {
        Self::from_channel(
            id,
            ChannelConfig {
                frequency_hz,
                modulation: ModulationConfig::LoRa(modulation),
                rx_boosted,
            },
            TxConfig {
                power_dbm: tx_power_dbm,
                ramp_time: TxRampTime::Us48,
            },
        )
    }

    /// Construct a complete GFSK profile without changing its packet contract.
    pub fn gfsk(
        id: ProfileId,
        frequency_hz: u32,
        modulation: GfskChannel,
        rx_boosted: bool,
        tx_power_dbm: i8,
    ) -> Result<Self, LinkError> {
        Self::from_channel(
            id,
            ChannelConfig {
                frequency_hz,
                modulation: ModulationConfig::Gfsk(modulation),
                rx_boosted,
            },
            TxConfig {
                power_dbm: tx_power_dbm,
                ramp_time: TxRampTime::Us48,
            },
        )
    }

    /// Create a 2.4 GHz GFSK profile using a Table 3-9 bitrate/deviation pair.
    /// Packet fields are fixed here and must match on every peer. The receiver
    /// filter uses the programmable value, not the rounded datasheet label.
    pub fn gfsk_reference_2_4(
        id: ProfileId,
        frequency_hz: u32,
        rate: GfskReferenceRate,
        tx_power_dbm: i8,
        sync_word: &[u8],
    ) -> Result<Self, LinkError> {
        if !(2_400_000_000..=2_500_000_000).contains(&frequency_hz) {
            return Err(LinkError::InvalidProfile);
        }
        let mut sync = heapless::Vec::new();
        sync.extend_from_slice(sync_word)
            .map_err(|_| LinkError::InvalidProfile)?;
        let (bit_rate_bps, frequency_deviation_hz, receiver_bandwidth) = rate.parameters();
        Self::gfsk(
            id,
            frequency_hz,
            GfskChannel {
                bit_rate_bps,
                frequency_deviation_hz,
                receiver_bandwidth,
                pulse_shape: GfskPulseShape::GaussianBt0_5,
                preamble_bits: 32,
                preamble_detector: GfskPreambleDetector::Bits16,
                sync_word: sync,
                address_filtering: GfskAddressFiltering::Disabled,
                packet_length: GfskPacketLength::Variable { maximum: u8::MAX },
                crc: GfskCrc::TwoBytes {
                    initial: 0xFFFF,
                    polynomial: 0x1021,
                    inverted: false,
                },
                whitening: Some(GfskWhitening { initial: 0x01FF }),
            },
            true,
            tx_power_dbm,
        )
    }

    fn from_channel(
        id: ProfileId,
        channel: ChannelConfig,
        tx: TxConfig,
    ) -> Result<Self, LinkError> {
        let validated = channel.validate().map_err(|_| LinkError::InvalidProfile)?;
        tx.validate(validated.band)
            .map_err(|_| LinkError::InvalidProfile)?;
        let band = match validated.band {
            RadioBand::SubGhz => ProfileBand::SubGhz,
            RadioBand::Ghz2_4 => ProfileBand::Ghz2_4,
        };
        Ok(Self {
            id,
            band,
            channel,
            tx,
        })
    }

    pub const fn id(&self) -> ProfileId {
        self.id
    }

    pub const fn band(&self) -> ProfileBand {
        self.band
    }

    pub const fn frequency_hz(&self) -> u32 {
        self.channel.frequency_hz
    }

    pub(crate) fn driver_channel(&self) -> &ChannelConfig {
        &self.channel
    }

    pub(crate) const fn driver_tx(&self) -> &TxConfig {
        &self.tx
    }
}

fn map_sf(value: SpreadingFactor) -> LoRaSpreadingFactor {
    match value {
        SpreadingFactor::Sf5 => LoRaSpreadingFactor::Sf5,
        SpreadingFactor::Sf6 => LoRaSpreadingFactor::Sf6,
        SpreadingFactor::Sf7 => LoRaSpreadingFactor::Sf7,
        SpreadingFactor::Sf8 => LoRaSpreadingFactor::Sf8,
        SpreadingFactor::Sf9 => LoRaSpreadingFactor::Sf9,
        SpreadingFactor::Sf10 => LoRaSpreadingFactor::Sf10,
        SpreadingFactor::Sf11 => LoRaSpreadingFactor::Sf11,
        SpreadingFactor::Sf12 => LoRaSpreadingFactor::Sf12,
    }
}

fn map_cr(value: CodingRate) -> LoRaCodingRate {
    match value {
        CodingRate::Cr4_5 => LoRaCodingRate::Cr4_5,
        CodingRate::Cr4_6 => LoRaCodingRate::Cr4_6,
        CodingRate::Cr4_7 => LoRaCodingRate::Cr4_7,
        CodingRate::Cr4_8 => LoRaCodingRate::Cr4_8,
        CodingRate::LongInterleaver4_5 => LoRaCodingRate::LongInterleaver4_5,
        CodingRate::LongInterleaver4_6 => LoRaCodingRate::LongInterleaver4_6,
        CodingRate::LongInterleaver4_8 => LoRaCodingRate::LongInterleaver4_8,
    }
}
