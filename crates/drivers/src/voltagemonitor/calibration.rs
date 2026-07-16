#![cfg_attr(not(any(feature = "stm32", test)), allow(dead_code))]

pub(super) const NOMINAL_ADC_REFERENCE_MV: u32 = 2_500;

const ADC_CALIB_RESOLUTION_BITS: u32 = 14;
const ADC_CALIB_VREF_MV: u32 = 3_000;
const VREFINT_CAL_ADDR: usize = 0x0BFA_07A5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Stm32AdcCalibration {
    vrefint_cal: u16,
}

impl Stm32AdcCalibration {
    pub(super) fn read() -> Self {
        Self {
            vrefint_cal: read_u16_system_memory(VREFINT_CAL_ADDR),
        }
    }

    pub(super) fn adc_reference_mv(self, vref_raw_12: u32) -> u32 {
        let raw_calib = adc12_to_calib14(vref_raw_12);
        if raw_calib == 0 || self.vrefint_cal == 0 {
            return NOMINAL_ADC_REFERENCE_MV;
        }

        (self.vrefint_cal as u32 * ADC_CALIB_VREF_MV) / raw_calib
    }
}

fn adc12_to_calib14(raw: u32) -> u32 {
    raw << (ADC_CALIB_RESOLUTION_BITS - 12)
}

fn read_u16_system_memory(addr: usize) -> u16 {
    let lo = unsafe { core::ptr::read_volatile(addr as *const u8) };
    let hi = unsafe { core::ptr::read_volatile((addr + 1) as *const u8) };
    u16::from_le_bytes([lo, hi])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factory_vref_calibration_converts_adc4_raw_to_adc_reference() {
        let calibration = Stm32AdcCalibration { vrefint_cal: 6_600 };

        assert_eq!(calibration.adc_reference_mv(1_981), 2_498);
    }

    #[test]
    fn factory_vref_calibration_falls_back_when_raw_is_zero() {
        let calibration = Stm32AdcCalibration { vrefint_cal: 6_600 };

        assert_eq!(calibration.adc_reference_mv(0), NOMINAL_ADC_REFERENCE_MV);
    }
}
