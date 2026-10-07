//! Complete, validated radio channel descriptions.

use heapless::Vec;

use super::error::ConfigError;

pub const MAX_GFSK_SYNC_WORD_LEN: usize = 8;
pub const MAX_PACKET_LEN: usize = u8::MAX as usize;

/// RF path selected by the fitted Ebyte E80 module.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadioBand {
    SubGhz,
    Ghz2_4,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelConfig {
    pub frequency_hz: u32,
    pub modulation: ModulationConfig,
    /// Enables the LR1121's approximately 2 mA higher-power RX boost.
    pub rx_boosted: bool,
}

impl ChannelConfig {
    pub fn validate(&self) -> Result<ValidatedChannel, ConfigError> {
        let band = band_for_frequency(self.frequency_hz)?;
        match &self.modulation {
            ModulationConfig::LoRa(config) => {
                config.validate()?;
                if !config.bandwidth.supports(band) {
                    return Err(ConfigError::UnsupportedLoRaBandwidth);
                }
            }
            ModulationConfig::Gfsk(config) => config.validate()?,
        }
        Ok(ValidatedChannel { band })
    }

    pub fn maximum_payload_len(&self) -> usize {
        match &self.modulation {
            ModulationConfig::LoRa(config) => usize::from(config.payload_length.unwrap_or(u8::MAX)),
            ModulationConfig::Gfsk(config) => usize::from(config.packet_length.maximum()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ValidatedChannel {
    pub band: RadioBand,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ModulationConfig {
    LoRa(LoRaChannel),
    Gfsk(GfskChannel),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoRaChannel {
    pub spreading_factor: LoRaSpreadingFactor,
    pub bandwidth: LoRaBandwidth,
    pub coding_rate: LoRaCodingRate,
    pub low_data_rate_optimization: LowDataRateOptimization,
    pub preamble_symbols: u16,
    pub header: LoRaHeaderMode,
    /// Required in implicit mode. In explicit mode this is an optional maximum.
    pub payload_length: Option<u8>,
    pub crc: bool,
    pub invert_iq: bool,
    pub sync_word: u8,
}

impl LoRaChannel {
    pub fn validate(&self) -> Result<(), ConfigError> {
        let minimum_preamble = if self.spreading_factor.value() <= 6 {
            12
        } else {
            8
        };
        if self.preamble_symbols < minimum_preamble {
            return Err(ConfigError::LoRaPreambleTooShort);
        }
        if self.header == LoRaHeaderMode::Implicit && self.payload_length.is_none() {
            return Err(ConfigError::ImplicitHeaderNeedsPayloadLength);
        }
        if self.payload_length == Some(0) {
            return Err(ConfigError::InvalidPayloadLength);
        }
        if matches!(
            self.coding_rate,
            LoRaCodingRate::LongInterleaver4_5
                | LoRaCodingRate::LongInterleaver4_6
                | LoRaCodingRate::LongInterleaver4_8
        ) {
            let payload_length = self
                .payload_length
                .ok_or(ConfigError::LongInterleaverNeedsPayloadLength)?;
            let maximum = if self.crc { 253 } else { 255 };
            if !(8..=maximum).contains(&payload_length) {
                return Err(ConfigError::InvalidLongInterleaverPayloadLength);
            }
        }
        Ok(())
    }

    pub fn low_data_rate_optimization_enabled(&self) -> bool {
        match self.low_data_rate_optimization {
            LowDataRateOptimization::Enabled => true,
            LowDataRateOptimization::Disabled => false,
            LowDataRateOptimization::Auto => {
                // Semtech recommends LDRO when a symbol lasts at least 16 ms.
                (1u64 << self.spreading_factor.value()) * 1_000_000
                    >= 16_000 * u64::from(self.bandwidth.hz())
            }
        }
    }
}

impl Default for LoRaChannel {
    fn default() -> Self {
        Self {
            spreading_factor: LoRaSpreadingFactor::Sf7,
            bandwidth: LoRaBandwidth::Khz125,
            coding_rate: LoRaCodingRate::Cr4_5,
            low_data_rate_optimization: LowDataRateOptimization::Auto,
            preamble_symbols: 12,
            header: LoRaHeaderMode::Explicit,
            payload_length: None,
            crc: true,
            invert_iq: false,
            sync_word: 0x12,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoRaSpreadingFactor {
    Sf5,
    Sf6,
    Sf7,
    Sf8,
    Sf9,
    Sf10,
    Sf11,
    Sf12,
}

impl LoRaSpreadingFactor {
    pub(crate) const fn value(self) -> u8 {
        match self {
            Self::Sf5 => 5,
            Self::Sf6 => 6,
            Self::Sf7 => 7,
            Self::Sf8 => 8,
            Self::Sf9 => 9,
            Self::Sf10 => 10,
            Self::Sf11 => 11,
            Self::Sf12 => 12,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoRaBandwidth {
    Khz62_5,
    Khz125,
    Khz250,
    Khz500,
    Khz203,
    Khz406,
    Khz812,
}

impl LoRaBandwidth {
    pub const fn hz(self) -> u32 {
        match self {
            Self::Khz62_5 => 62_500,
            Self::Khz125 => 125_000,
            Self::Khz250 => 250_000,
            Self::Khz500 => 500_000,
            Self::Khz203 => 203_000,
            Self::Khz406 => 406_000,
            Self::Khz812 => 812_000,
        }
    }

    pub const fn supports(self, band: RadioBand) -> bool {
        matches!(
            (band, self),
            (
                RadioBand::SubGhz,
                Self::Khz62_5 | Self::Khz125 | Self::Khz250 | Self::Khz500
            ) | (
                RadioBand::Ghz2_4,
                Self::Khz203 | Self::Khz406 | Self::Khz812
            )
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoRaCodingRate {
    Cr4_5,
    Cr4_6,
    Cr4_7,
    Cr4_8,
    LongInterleaver4_5,
    LongInterleaver4_6,
    LongInterleaver4_8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LowDataRateOptimization {
    #[default]
    Auto,
    Enabled,
    Disabled,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoRaHeaderMode {
    Explicit,
    Implicit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GfskChannel {
    pub bit_rate_bps: u32,
    pub frequency_deviation_hz: u32,
    pub receiver_bandwidth: GfskBandwidth,
    pub pulse_shape: GfskPulseShape,
    pub preamble_bits: u16,
    pub preamble_detector: GfskPreambleDetector,
    pub sync_word: Vec<u8, MAX_GFSK_SYNC_WORD_LEN>,
    pub address_filtering: GfskAddressFiltering,
    pub packet_length: GfskPacketLength,
    pub crc: GfskCrc,
    pub whitening: Option<GfskWhitening>,
}

impl GfskChannel {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if !(600..=300_000).contains(&self.bit_rate_bps) {
            return Err(ConfigError::UnsupportedGfskBitRate);
        }
        if self.frequency_deviation_hz == 0 || self.frequency_deviation_hz > 200_000 {
            return Err(ConfigError::UnsupportedGfskFrequencyDeviation);
        }
        let occupied =
            u64::from(self.bit_rate_bps).saturating_add(2 * u64::from(self.frequency_deviation_hz));
        // Table 3-9 characterizes 250 kb/s / 125 kHz deviation with a
        // nominal 500 kHz RX filter. The largest programmable LR1121 filter
        // is 467 kHz (user manual Table 8-14), so allow this exact reference
        // point while keeping the conservative check for all other settings.
        let documented_fast_fsk = self.bit_rate_bps == 250_000
            && self.frequency_deviation_hz == 125_000
            && self.receiver_bandwidth == GfskBandwidth::Hz467_000;
        if occupied > u64::from(self.receiver_bandwidth.hz()) && !documented_fast_fsk {
            return Err(ConfigError::GfskReceiverBandwidthTooNarrow);
        }
        if self.preamble_bits == 0 {
            return Err(ConfigError::InvalidPreamble);
        }
        if self.sync_word.len() > MAX_GFSK_SYNC_WORD_LEN {
            return Err(ConfigError::GfskSyncWordTooLong);
        }
        if self.preamble_detector != GfskPreambleDetector::Off && self.sync_word.is_empty() {
            return Err(ConfigError::GfskSyncWordRequired);
        }
        if self.packet_length.maximum() == 0 {
            return Err(ConfigError::InvalidPayloadLength);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GfskBandwidth {
    Hz4_800,
    Hz5_800,
    Hz7_300,
    Hz9_700,
    Hz11_700,
    Hz14_600,
    Hz19_500,
    Hz23_400,
    Hz29_300,
    Hz39_000,
    Hz46_900,
    Hz58_600,
    Hz78_200,
    Hz93_800,
    Hz117_300,
    Hz156_200,
    Hz187_200,
    Hz234_300,
    Hz312_000,
    Hz373_600,
    Hz467_000,
}

impl GfskBandwidth {
    pub const fn hz(self) -> u32 {
        match self {
            Self::Hz4_800 => 4_800,
            Self::Hz5_800 => 5_800,
            Self::Hz7_300 => 7_300,
            Self::Hz9_700 => 9_700,
            Self::Hz11_700 => 11_700,
            Self::Hz14_600 => 14_600,
            Self::Hz19_500 => 19_500,
            Self::Hz23_400 => 23_400,
            Self::Hz29_300 => 29_300,
            Self::Hz39_000 => 39_000,
            Self::Hz46_900 => 46_900,
            Self::Hz58_600 => 58_600,
            Self::Hz78_200 => 78_200,
            Self::Hz93_800 => 93_800,
            Self::Hz117_300 => 117_300,
            Self::Hz156_200 => 156_200,
            Self::Hz187_200 => 187_200,
            Self::Hz234_300 => 234_300,
            Self::Hz312_000 => 312_000,
            Self::Hz373_600 => 373_600,
            Self::Hz467_000 => 467_000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GfskPulseShape {
    None,
    GaussianBt0_3,
    GaussianBt0_5,
    GaussianBt0_7,
    GaussianBt1,
    RaisedCosineBt0_7,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GfskPreambleDetector {
    Off,
    Bits8,
    Bits16,
    Bits24,
    Bits32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GfskAddressFiltering {
    Disabled,
    Node { node: u8 },
    NodeAndBroadcast { node: u8, broadcast: u8 },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GfskPacketLength {
    Fixed(u8),
    Variable { maximum: u8 },
    VariableSx128x { maximum: u8 },
}

impl GfskPacketLength {
    pub const fn maximum(self) -> u8 {
        match self {
            Self::Fixed(value) => value,
            Self::Variable { maximum } | Self::VariableSx128x { maximum } => maximum,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GfskCrc {
    Off,
    OneByte {
        initial: u32,
        polynomial: u32,
        inverted: bool,
    },
    TwoBytes {
        initial: u32,
        polynomial: u32,
        inverted: bool,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GfskWhitening {
    pub initial: u16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TxConfig {
    pub power_dbm: i8,
    pub ramp_time: TxRampTime,
}

impl TxConfig {
    pub fn validate(self, band: RadioBand) -> Result<(), ConfigError> {
        let supported = match band {
            RadioBand::SubGhz => (-17..=22).contains(&self.power_dbm),
            RadioBand::Ghz2_4 => (-18..=13).contains(&self.power_dbm),
        };
        if supported {
            Ok(())
        } else {
            Err(ConfigError::UnsupportedTxPower)
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TxRampTime {
    Us16,
    Us32,
    #[default]
    Us48,
    Us64,
    Us80,
    Us96,
    Us112,
    Us128,
    Us144,
    Us160,
    Us176,
    Us192,
    Us208,
    Us240,
    Us272,
    Us304,
}

pub fn band_for_frequency(frequency_hz: u32) -> Result<RadioBand, ConfigError> {
    match frequency_hz {
        150_000_000..=960_000_000 => Ok(RadioBand::SubGhz),
        2_400_000_000..=2_500_000_000 => Ok(RadioBand::Ghz2_4),
        _ => Err(ConfigError::UnsupportedFrequency),
    }
}
