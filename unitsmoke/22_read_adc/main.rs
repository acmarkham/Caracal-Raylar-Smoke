// ADC voltage smoke test.
//
// Activity: reads VREFINT plus PA0/PB1/PA1 once per second and prints raw ADC
// counts, pin voltage, and divider-corrected source voltage.

#![no_std]
#![no_main]

use defmt::{info, unwrap, warn};
use embassy_executor::Spawner;
use embassy_stm32::adc::adc4::{
    Averaging as Adc4Averaging, Resolution as Adc4Resolution, SampleTime as Adc4SampleTime,
};
use embassy_stm32::adc::{Adc, AdcChannel, AdcConfig, Averaging, Resolution, SampleTime, VrefInt};
use embassy_stm32::gpio::Output;
use embassy_stm32::pac::{self, vrefbuf};
use embassy_stm32::peripherals::{ADC1, ADC4};
use embassy_stm32::rcc::*;
use embassy_stm32::time::mhz;
use embassy_time::Timer;
use raylar_board_v1p0::{AdcVoltages, Board, Leds};
use {defmt_rtt as _, panic_probe as _};

const ADC_RESOLUTION: Resolution = Resolution::BITS14;
const ADC_MAX_COUNTS: u32 = (1 << 14) - 1;
const ADC4_MAX_COUNTS: u32 = (1 << 12) - 1;
const ADC_REFERENCE_MV: u32 = 2_500;
const VREFINT_MIN_PLAUSIBLE_RAW: u32 = 1_000;
const VREFBUF_READY_TIMEOUT_SPINS: u32 = 1_000_000;

const DIVIDER_180K_33K: Divider = Divider::new(180 + 33, 33);
const DIVIDER_33K_33K: Divider = Divider::new(33 + 33, 33);
const DIVIDER_180K_33K_FULL_SCALE_MV: u32 =
    (ADC_REFERENCE_MV * DIVIDER_180K_33K.numerator) / DIVIDER_180K_33K.denominator;

#[derive(Copy, Clone)]
struct Divider {
    numerator: u32,
    denominator: u32,
}

impl Divider {
    const fn new(numerator: u32, denominator: u32) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    fn scale_mv(self, pin_mv: u32) -> u32 {
        ((pin_mv * self.numerator) + (self.denominator / 2)) / self.denominator
    }
}

struct VoltageReading {
    raw: u32,
    pin_mv: u32,
    source_mv: u32,
}

struct VrefReading {
    raw: u32,
    mv: u32,
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
    let Board {
        leds, adc_voltages, ..
    } = Board::new(p);
    let Leds {
        sys_main_red,
        sys_main_green,
        ..
    } = leds;

    info!("ADC voltage smoke test started");

    if enable_vrefbuf() {
        info!("ADC VREFBUF enabled, VREFBUF={}mV", ADC_REFERENCE_MV);
        info!(
            "180k/33k ADC channels full-scale at {}.{:03}V source",
            DIVIDER_180K_33K_FULL_SCALE_MV / 1000,
            DIVIDER_180K_33K_FULL_SCALE_MV % 1000
        );
    } else {
        warn!("ADC VREFBUF did not report ready; ADC voltages may be invalid");
    }

    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    spawner.spawn(unwrap!(adc_task(adc_voltages, sys_main_red)));

    core::future::pending().await
}

#[embassy_executor::task]
async fn heartbeat_task(mut led: Output<'static>) -> ! {
    loop {
        led.set_high();
        Timer::after_millis(100).await;
        led.set_low();
        Timer::after_millis(900).await;
    }
}

#[embassy_executor::task]
async fn adc_task(adc_voltages: AdcVoltages<'static>, mut led: Output<'static>) -> ! {
    let AdcVoltages {
        adc,
        adc4,
        mut v_dc,
        mut v_batt,
        mut v_solar,
    } = adc_voltages;

    let mut adc = Adc::new_with_config(
        adc,
        AdcConfig {
            resolution: Some(ADC_RESOLUTION),
            averaging: Some(Averaging::Samples256),
        },
    );
    let mut adc4 = Adc::new_adc4(adc4);
    adc4.set_resolution_adc4(Adc4Resolution::BITS12);
    adc4.set_averaging_adc4(Adc4Averaging::Samples256);
    let mut vrefint = adc4.enable_vrefint_adc4();

    let sample_time = SampleTime::CYCLES160_5;
    let adc4_sample_time = Adc4SampleTime::CYCLES814_5;

    info!(
        "ADC config: ADC1 14-bit, ADC4 12-bit, 256x hardware averaging, VREFBUF={}mV",
        ADC_REFERENCE_MV
    );

    loop {
        led.set_high();

        let vrefint = read_vrefint(&mut adc4, &mut vrefint, adc4_sample_time);
        let v_dc = read_voltage(&mut adc, &mut v_dc, sample_time, DIVIDER_180K_33K);
        let v_solar = read_voltage(&mut adc, &mut v_solar, sample_time, DIVIDER_180K_33K);
        let v_batt = read_voltage(&mut adc, &mut v_batt, sample_time, DIVIDER_33K_33K);

        led.set_low();

        log_vrefint(vrefint);
        log_reading("V_ADC_DC", v_dc);
        log_reading("V_ADC_SOLAR", v_solar);
        log_reading("V_ADC_BATT", v_batt);

        Timer::after_secs(1).await;
    }
}

fn enable_vrefbuf() -> bool {
    pac::RCC.apb3enr().modify(|w| {
        w.set_vrefen(true);
    });
    let vref_clock_enabled = pac::RCC.apb3enr().read().vrefen();

    pac::VREFBUF.csr().modify(|w| {
        w.set_envr(false);
    });

    pac::VREFBUF.csr().modify(|w| {
        w.set_hiz(vrefbuf::vals::Hiz::CONNECTED);
        w.set_vrs(vrefbuf::vals::Vrs::VREF3);
        w.set_envr(true);
    });

    for _ in 0..VREFBUF_READY_TIMEOUT_SPINS {
        if pac::VREFBUF.csr().read().vrr() {
            return true;
        }
    }

    warn!(
        "VREFBUF not ready: rcc_vrefen={} envr={} hiz={:?} vrs={:?} vrr={}",
        vref_clock_enabled,
        pac::VREFBUF.csr().read().envr(),
        pac::VREFBUF.csr().read().hiz(),
        pac::VREFBUF.csr().read().vrs(),
        pac::VREFBUF.csr().read().vrr()
    );

    false
}

fn read_voltage(
    adc: &mut Adc<'static, ADC1>,
    channel: &mut impl AdcChannel<ADC1>,
    sample_time: SampleTime,
    divider: Divider,
) -> VoltageReading {
    let raw = adc.blocking_read(channel, sample_time) as u32;
    let pin_mv = raw_to_mv(raw, ADC_REFERENCE_MV, ADC_MAX_COUNTS);

    VoltageReading {
        raw,
        pin_mv,
        source_mv: divider.scale_mv(pin_mv),
    }
}

fn read_vrefint(
    adc4: &mut Adc<'static, ADC4>,
    vrefint: &mut VrefInt,
    sample_time: Adc4SampleTime,
) -> VrefReading {
    let raw = adc4.blocking_read(vrefint, sample_time) as u32;
    if raw < VREFINT_MIN_PLAUSIBLE_RAW {
        warn!(
            "ADC4 VREFINT raw={} is implausibly low; check VREFBUF/VREF+ and ADC4 VREFEN",
            raw
        );
    }

    VrefReading {
        raw,
        mv: raw_to_mv(raw, ADC_REFERENCE_MV, ADC4_MAX_COUNTS),
    }
}

fn raw_to_mv(raw: u32, vref_mv: u32, max_counts: u32) -> u32 {
    ((raw * vref_mv) + (max_counts / 2)) / max_counts
}

fn log_vrefint(reading: VrefReading) {
    info!(
        "ADC4 VREFINT raw={} approx={}.{:03}V VREFBUF={}.{:03}V",
        reading.raw,
        reading.mv / 1000,
        reading.mv % 1000,
        ADC_REFERENCE_MV / 1000,
        ADC_REFERENCE_MV % 1000
    );
}

fn log_reading(name: &str, reading: VoltageReading) {
    info!(
        "{} raw={} pin={}.{:03}V source={}.{:03}V",
        name,
        reading.raw,
        reading.pin_mv / 1000,
        reading.pin_mv % 1000,
        reading.source_mv / 1000,
        reading.source_mv % 1000
    );
}
