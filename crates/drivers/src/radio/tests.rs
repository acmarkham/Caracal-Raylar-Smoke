use super::*;

fn lora() -> ChannelConfig {
    ChannelConfig {
        frequency_hz: 868_000_000,
        modulation: ModulationConfig::LoRa(LoRaChannel::default()),
        rx_boosted: false,
    }
}

fn gfsk() -> GfskChannel {
    let mut sync_word = heapless::Vec::new();
    sync_word.extend_from_slice(&[0x2D, 0xD4]).unwrap();
    GfskChannel {
        bit_rate_bps: 50_000,
        frequency_deviation_hz: 25_000,
        receiver_bandwidth: GfskBandwidth::Hz117_300,
        pulse_shape: GfskPulseShape::GaussianBt0_5,
        preamble_bits: 32,
        preamble_detector: GfskPreambleDetector::Bits16,
        sync_word,
        address_filtering: GfskAddressFiltering::Disabled,
        packet_length: GfskPacketLength::Variable { maximum: 64 },
        crc: GfskCrc::TwoBytes {
            initial: 0xFFFF,
            polynomial: 0x1021,
            inverted: false,
        },
        whitening: Some(GfskWhitening { initial: 0x01FF }),
    }
}

#[test]
fn frequency_band_boundaries_are_validated() {
    assert_eq!(band_for_frequency(150_000_000), Ok(RadioBand::SubGhz));
    assert_eq!(band_for_frequency(960_000_000), Ok(RadioBand::SubGhz));
    assert_eq!(band_for_frequency(2_400_000_000), Ok(RadioBand::Ghz2_4));
    assert_eq!(band_for_frequency(2_500_000_000), Ok(RadioBand::Ghz2_4));
    assert_eq!(
        band_for_frequency(1_000_000_000),
        Err(ConfigError::UnsupportedFrequency)
    );
}

#[test]
fn implicit_lora_requires_a_payload_length() {
    let mut channel = lora();
    let ModulationConfig::LoRa(ref mut config) = channel.modulation else {
        unreachable!()
    };
    config.header = LoRaHeaderMode::Implicit;
    assert_eq!(
        channel.validate(),
        Err(ConfigError::ImplicitHeaderNeedsPayloadLength)
    );
    let ModulationConfig::LoRa(ref mut config) = channel.modulation else {
        unreachable!()
    };
    config.payload_length = Some(8);
    assert!(channel.validate().is_ok());
}

#[test]
fn automatic_ldro_uses_sixteen_millisecond_symbol_threshold() {
    let mut config = LoRaChannel::default();
    config.spreading_factor = LoRaSpreadingFactor::Sf12;
    config.bandwidth = LoRaBandwidth::Khz125;
    assert!(config.low_data_rate_optimization_enabled());
    config.spreading_factor = LoRaSpreadingFactor::Sf7;
    assert!(!config.low_data_rate_optimization_enabled());
}

#[test]
fn lora_bandwidths_match_the_selected_rf_path() {
    let mut channel = lora();
    for bandwidth in [
        LoRaBandwidth::Khz203,
        LoRaBandwidth::Khz406,
        LoRaBandwidth::Khz812,
    ] {
        let ModulationConfig::LoRa(ref mut config) = channel.modulation else {
            unreachable!()
        };
        config.bandwidth = bandwidth;
        assert_eq!(
            channel.validate(),
            Err(ConfigError::UnsupportedLoRaBandwidth)
        );
        channel.frequency_hz = 2_445_000_000;
        assert!(channel.validate().is_ok());
        channel.frequency_hz = 868_000_000;
    }
    let ModulationConfig::LoRa(ref mut config) = channel.modulation else {
        unreachable!()
    };
    config.bandwidth = LoRaBandwidth::Khz125;
    channel.frequency_hz = 2_445_000_000;
    assert_eq!(
        channel.validate(),
        Err(ConfigError::UnsupportedLoRaBandwidth)
    );
}

#[test]
fn long_interleaver_enforces_documented_payload_bounds() {
    let mut config = LoRaChannel::default();
    config.coding_rate = LoRaCodingRate::LongInterleaver4_5;
    assert_eq!(
        config.validate(),
        Err(ConfigError::LongInterleaverNeedsPayloadLength)
    );
    config.payload_length = Some(7);
    assert_eq!(
        config.validate(),
        Err(ConfigError::InvalidLongInterleaverPayloadLength)
    );
    config.payload_length = Some(8);
    assert!(config.validate().is_ok());
}

#[test]
fn gfsk_rejects_an_impossible_receiver_bandwidth() {
    let mut config = gfsk();
    config.receiver_bandwidth = GfskBandwidth::Hz93_800;
    assert_eq!(
        config.validate(),
        Err(ConfigError::GfskReceiverBandwidthTooNarrow)
    );
}

#[test]
fn gfsk_table_3_9_filter_settings_are_accepted_without_broadening_the_exception() {
    let mut config = gfsk();
    for (bitrate, deviation, bandwidth) in [
        (1_200, 5_000, GfskBandwidth::Hz19_500),
        (4_800, 5_000, GfskBandwidth::Hz19_500),
        (38_400, 40_000, GfskBandwidth::Hz156_200),
        (250_000, 125_000, GfskBandwidth::Hz467_000),
    ] {
        config.bit_rate_bps = bitrate;
        config.frequency_deviation_hz = deviation;
        config.receiver_bandwidth = bandwidth;
        assert!(config.validate().is_ok());
    }
    config.frequency_deviation_hz = 126_000;
    assert_eq!(
        config.validate(),
        Err(ConfigError::GfskReceiverBandwidthTooNarrow)
    );
}

#[test]
fn tx_power_limits_depend_on_the_rf_path() {
    assert!(TxConfig {
        power_dbm: 22,
        ramp_time: TxRampTime::Us48
    }
    .validate(RadioBand::SubGhz)
    .is_ok());
    assert_eq!(
        TxConfig {
            power_dbm: 22,
            ramp_time: TxRampTime::Us48
        }
        .validate(RadioBand::Ghz2_4),
        Err(ConfigError::UnsupportedTxPower)
    );
}

#[test]
fn rtc_conversion_never_uses_reserved_continuous_value() {
    assert_eq!(rtc_ticks(Duration::from_micros(1)), 1);
    assert_eq!(rtc_ticks(Duration::from_secs(1)), 32_768);
    assert_eq!(rtc_ticks(Duration::from_secs(1_000)), 0xFF_FFFE);
}

#[test]
fn counters_saturate() {
    let mut value = u32::MAX;
    RadioStats::increment(&mut value);
    assert_eq!(value, u32::MAX);
}
