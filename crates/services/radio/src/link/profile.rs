use raylar_drivers::radio::{
    ChannelConfig, LoRaBandwidth, LoRaChannel, LoRaCodingRate, LoRaHeaderMode, LoRaSpreadingFactor,
    LowDataRateOptimization, ModulationConfig, TxConfig, TxRampTime,
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
        let band = match frequency_hz {
            150_000_000..=960_000_000 => ProfileBand::SubGhz,
            2_400_000_000..=2_500_000_000 => ProfileBand::Ghz2_4,
            _ => return Err(LinkError::InvalidProfile),
        };
        let bandwidth = match bandwidth_hz {
            62_500 => LoRaBandwidth::Khz62_5,
            125_000 => LoRaBandwidth::Khz125,
            250_000 => LoRaBandwidth::Khz250,
            500_000 => LoRaBandwidth::Khz500,
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
        let validated = channel.validate().map_err(|_| LinkError::InvalidProfile)?;
        tx.validate(validated.band)
            .map_err(|_| LinkError::InvalidProfile)?;
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
    }
}
