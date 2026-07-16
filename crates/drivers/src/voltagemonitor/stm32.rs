use embassy_stm32::adc::adc4::{
    Averaging as Adc4Averaging, Resolution as Adc4Resolution, SampleTime as Adc4SampleTime,
};
use embassy_stm32::adc::{Adc, AdcChannel, AdcConfig, Averaging, Resolution, SampleTime, VrefInt};
use embassy_stm32::peripherals::{ADC1, ADC4};
use embassy_stm32::Peri;

use super::calibration::{Stm32AdcCalibration, NOMINAL_ADC_REFERENCE_MV};
use super::stm32_vrefbuf::enable_vrefbuf;
use crate::voltagemonitor::{raw_to_mv, Divider, VoltageSampler, VoltageState};

const ADC_RESOLUTION: Resolution = Resolution::BITS14;
const ADC_MAX_COUNTS: u32 = (1 << 14) - 1;
const ADC4_MAX_COUNTS: u32 = (1 << 12) - 1;
const VREFINT_MIN_PLAUSIBLE_RAW: u32 = 1_000;

const DIVIDER_180K_33K: Divider = Divider::new(180 + 33, 33);
const DIVIDER_33K_33K: Divider = Divider::new(33 + 33, 33);
const DIVIDER_USB_10K_5K1: Divider = Divider::new(151, 51);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Stm32VoltageStatus {
    pub vrefbuf_ready: bool,
    pub last_vrefint_raw: u32,
    pub last_adc_reference_mv: u32,
    pub vrefint_plausible: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct NoUsbVoltage;

pub trait UsbVoltageChannel {
    fn read_usb_mv(
        &mut self,
        adc: &mut Adc<'static, ADC1>,
        sample_time: SampleTime,
        adc_reference_mv: u32,
    ) -> u32;
}

impl UsbVoltageChannel for NoUsbVoltage {
    fn read_usb_mv(
        &mut self,
        _adc: &mut Adc<'static, ADC1>,
        _sample_time: SampleTime,
        _adc_reference_mv: u32,
    ) -> u32 {
        0
    }
}

impl<T> UsbVoltageChannel for T
where
    T: AdcChannel<ADC1>,
{
    fn read_usb_mv(
        &mut self,
        adc: &mut Adc<'static, ADC1>,
        sample_time: SampleTime,
        adc_reference_mv: u32,
    ) -> u32 {
        read_voltage(
            adc,
            self,
            sample_time,
            adc_reference_mv,
            DIVIDER_USB_10K_5K1,
        )
    }
}

pub struct Stm32VoltageMonitor<DC, BATT, SOLAR, USB = NoUsbVoltage> {
    adc: Adc<'static, ADC1>,
    adc4: Adc<'static, ADC4>,
    vrefint: VrefInt,
    v_dc: DC,
    v_batt: BATT,
    v_solar: SOLAR,
    usb: USB,
    sample_time: SampleTime,
    adc4_sample_time: Adc4SampleTime,
    calibration: Stm32AdcCalibration,
    status: Stm32VoltageStatus,
}

impl<DC, BATT, SOLAR> Stm32VoltageMonitor<DC, BATT, SOLAR, NoUsbVoltage>
where
    DC: AdcChannel<ADC1>,
    BATT: AdcChannel<ADC1>,
    SOLAR: AdcChannel<ADC1>,
{
    pub fn new_without_usb(
        adc: Peri<'static, ADC1>,
        adc4: Peri<'static, ADC4>,
        v_dc: DC,
        v_batt: BATT,
        v_solar: SOLAR,
    ) -> Self {
        Self::new(adc, adc4, v_dc, v_batt, v_solar, NoUsbVoltage)
    }
}

impl<DC, BATT, SOLAR, USB> Stm32VoltageMonitor<DC, BATT, SOLAR, USB>
where
    DC: AdcChannel<ADC1>,
    BATT: AdcChannel<ADC1>,
    SOLAR: AdcChannel<ADC1>,
    USB: UsbVoltageChannel,
{
    pub fn new(
        adc: Peri<'static, ADC1>,
        adc4: Peri<'static, ADC4>,
        v_dc: DC,
        v_batt: BATT,
        v_solar: SOLAR,
        usb: USB,
    ) -> Self {
        let vrefbuf_ready = enable_vrefbuf();
        let adc = Adc::new_with_config(
            adc,
            AdcConfig {
                resolution: Some(ADC_RESOLUTION),
                averaging: Some(Averaging::Samples256),
            },
        );
        let mut adc4 = Adc::new_adc4(adc4);
        adc4.set_resolution_adc4(Adc4Resolution::BITS12);
        adc4.set_averaging_adc4(Adc4Averaging::Samples256);
        let vrefint = adc4.enable_vrefint_adc4();

        Self {
            adc,
            adc4,
            vrefint,
            v_dc,
            v_batt,
            v_solar,
            usb,
            sample_time: SampleTime::CYCLES160_5,
            adc4_sample_time: Adc4SampleTime::CYCLES814_5,
            calibration: Stm32AdcCalibration::read(),
            status: Stm32VoltageStatus {
                vrefbuf_ready,
                last_vrefint_raw: 0,
                last_adc_reference_mv: NOMINAL_ADC_REFERENCE_MV,
                vrefint_plausible: false,
            },
        }
    }

    pub fn status(&self) -> Stm32VoltageStatus {
        self.status
    }
}

impl<DC, BATT, SOLAR, USB> VoltageSampler for Stm32VoltageMonitor<DC, BATT, SOLAR, USB>
where
    DC: AdcChannel<ADC1>,
    BATT: AdcChannel<ADC1>,
    SOLAR: AdcChannel<ADC1>,
    USB: UsbVoltageChannel,
{
    fn sample(&mut self) -> VoltageState {
        let vref_raw = self
            .adc4
            .blocking_read(&mut self.vrefint, self.adc4_sample_time) as u32;
        let adc_reference_mv = self.calibration.adc_reference_mv(vref_raw);
        self.status.last_vrefint_raw = vref_raw;
        self.status.last_adc_reference_mv = adc_reference_mv;
        self.status.vrefint_plausible = vref_raw >= VREFINT_MIN_PLAUSIBLE_RAW;

        VoltageState {
            battery_mv: read_voltage(
                &mut self.adc,
                &mut self.v_batt,
                self.sample_time,
                adc_reference_mv,
                DIVIDER_33K_33K,
            ),
            solar_mv: read_voltage(
                &mut self.adc,
                &mut self.v_solar,
                self.sample_time,
                adc_reference_mv,
                DIVIDER_180K_33K,
            ),
            ext_dc_mv: read_voltage(
                &mut self.adc,
                &mut self.v_dc,
                self.sample_time,
                adc_reference_mv,
                DIVIDER_180K_33K,
            ),
            usb_mv: self
                .usb
                .read_usb_mv(&mut self.adc, self.sample_time, adc_reference_mv),
            vref_mv: raw_to_mv(vref_raw, adc_reference_mv, ADC4_MAX_COUNTS),
        }
    }
}

fn read_voltage(
    adc: &mut Adc<'static, ADC1>,
    channel: &mut impl AdcChannel<ADC1>,
    sample_time: SampleTime,
    adc_reference_mv: u32,
    divider: Divider,
) -> u32 {
    let raw = adc.blocking_read(channel, sample_time) as u32;
    let pin_mv = raw_to_mv(raw, adc_reference_mv, ADC_MAX_COUNTS);
    divider.scale_mv(pin_mv)
}
