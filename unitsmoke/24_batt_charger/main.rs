// Battery charger and voltage monitor smoke test.
//
// Activity: reads VREFINT plus PA0/PB1/PA1 once per second and prints raw ADC
// counts, pin voltage, and divider-corrected source voltage. In the same loop it
// reads the BQ25186 charger status registers on SENS_I2C and prints the charger
// state flags that are relevant for battery attachment, power path, charge mode,
// and input current limiting.

#![no_std]
#![no_main]

use defmt::{info, unwrap, warn};
use embassy_executor::Spawner;
use embassy_stm32::adc::adc4::{
    Averaging as Adc4Averaging, Resolution as Adc4Resolution, SampleTime as Adc4SampleTime,
};
use embassy_stm32::adc::{Adc, AdcChannel, AdcConfig, Averaging, Resolution, SampleTime, VrefInt};
use embassy_stm32::gpio::Output;
use embassy_stm32::i2c::{Config as I2cConfig, I2c};
use embassy_stm32::pac::{self, vrefbuf};
use embassy_stm32::peripherals::{ADC1, ADC4};
use embassy_stm32::rcc::*;
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::Timer;
use raylar_board_v1p0::{AdcVoltages, Board, Leds, SensI2C};
use {defmt_rtt as _, panic_probe as _};

const ADC_RESOLUTION: Resolution = Resolution::BITS14;
const ADC_MAX_COUNTS: u32 = (1 << 14) - 1;
const ADC4_MAX_COUNTS: u32 = (1 << 12) - 1;
const ADC_REFERENCE_MV: u32 = 2_500;
const VREFINT_MIN_PLAUSIBLE_RAW: u32 = 1_000;
const VREFBUF_READY_TIMEOUT_SPINS: u32 = 1_000_000;
const BATTERY_CONNECTED_MIN_MV: u32 = 2_000;

const DIVIDER_180K_33K: Divider = Divider::new(180 + 33, 33);
const DIVIDER_33K_33K: Divider = Divider::new(33 + 33, 33);
const DIVIDER_180K_33K_FULL_SCALE_MV: u32 =
    (ADC_REFERENCE_MV * DIVIDER_180K_33K.numerator) / DIVIDER_180K_33K.denominator;

const BQ25186_ADDR: u8 = 0x6A;
const REG_STAT0: u8 = 0x00;
const REG_STAT1: u8 = 0x01;
const REG_ICHG_CTRL: u8 = 0x04;
const REG_TMR_ILIM: u8 = 0x08;
const REG_MASK_ID: u8 = 0x0C;
const BQ_DUMP_FIRST_REG: u8 = 0x00;
const BQ_DUMP_LAST_REG: u8 = 0x0D;
const BQ_DUMP_REG_COUNT: usize = (BQ_DUMP_LAST_REG - BQ_DUMP_FIRST_REG + 1) as usize;

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

#[derive(Copy, Clone)]
struct VoltageReading {
    raw: u32,
    pin_mv: u32,
    source_mv: u32,
}

#[derive(Copy, Clone)]
struct VrefReading {
    raw: u32,
    mv: u32,
}

struct ChargerStatus {
    stat0: u8,
    stat1: u8,
    tmr_ilim: u8,
}

#[derive(Copy, Clone)]
struct RegisterDump {
    values: [Option<u8>; BQ_DUMP_REG_COUNT],
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
        leds,
        adc_voltages,
        sens_i2c,
        ..
    } = Board::new(p);
    let Leds {
        sys_main_red,
        sys_main_green,
        ..
    } = leds;

    info!("Battery charger smoke test started");

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
    spawner.spawn(unwrap!(monitor_task(adc_voltages, sens_i2c, sys_main_red)));

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
async fn monitor_task(
    adc_voltages: AdcVoltages<'static>,
    sens_i2c: SensI2C<'static>,
    mut led: Output<'static>,
) -> ! {
    let AdcVoltages {
        adc,
        adc4,
        mut v_dc,
        mut v_batt,
        mut v_solar,
    } = adc_voltages;

    let SensI2C { i2c, scl, sda } = sens_i2c;

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

    let mut i2c_config = I2cConfig::default();
    i2c_config.frequency = Hertz(100_000);
    let mut i2c = I2c::new_blocking(i2c, scl, sda, i2c_config);

    info!(
        "ADC config: ADC1 14-bit, ADC4 12-bit, 256x hardware averaging, VREFBUF={}mV",
        ADC_REFERENCE_MV
    );

    let mut whoami = [0u8; 1];
    match i2c.blocking_write_read(BQ25186_ADDR, &[REG_MASK_ID], &mut whoami) {
        Ok(_) => info!("BQ25186 WHO_AM_I = {=u8:#x}", whoami[0]),
        Err(e) => info!("BQ25186 WHO_AM_I read failed: {=?}", e),
    }

    configure_charge_current(&mut i2c);

    loop {
        led.set_high();

        let vrefint = read_vrefint(&mut adc4, &mut vrefint, adc4_sample_time);
        let v_dc = read_voltage(&mut adc, &mut v_dc, sample_time, DIVIDER_180K_33K);
        let v_solar = read_voltage(&mut adc, &mut v_solar, sample_time, DIVIDER_180K_33K);
        let v_batt = read_voltage(&mut adc, &mut v_batt, sample_time, DIVIDER_33K_33K);
        let charger = read_charger_status(&mut i2c);
        let register_dump = read_bq_register_dump(&mut i2c);

        led.set_low();

        log_vrefint(vrefint);
        log_reading("V_ADC_DC", v_dc);
        log_reading("V_ADC_SOLAR", v_solar);
        log_reading("V_ADC_BATT", v_batt);
        log_bq_register_dump(register_dump);

        match charger {
            Some(status) => log_charger_status(status, v_batt),
            None => warn!("BQ25186 status unavailable"),
        }

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

fn read_charger_status(
    i2c: &mut I2c<'static, embassy_stm32::mode::Blocking, embassy_stm32::i2c::mode::Master>,
) -> Option<ChargerStatus> {
    let stat0 = read_bq_register(i2c, REG_STAT0)?;
    let stat1 = read_bq_register(i2c, REG_STAT1)?;
    let tmr_ilim = read_bq_register(i2c, REG_TMR_ILIM)?;

    Some(ChargerStatus {
        stat0,
        stat1,
        tmr_ilim,
    })
}

fn read_bq_register(
    i2c: &mut I2c<'static, embassy_stm32::mode::Blocking, embassy_stm32::i2c::mode::Master>,
    register: u8,
) -> Option<u8> {
    let mut value = [0u8; 1];
    match i2c.blocking_write_read(BQ25186_ADDR, &[register], &mut value) {
        Ok(_) => Some(value[0]),
        Err(e) => {
            warn!(
                "BQ25186 register 0x{=u8:02x} read failed: {=?}",
                register, e
            );
            None
        }
    }
}

fn configure_charge_current(
    i2c: &mut I2c<'static, embassy_stm32::mode::Blocking, embassy_stm32::i2c::mode::Master>,
) {
    const ICHG_CTRL_100MA: u8 = 0x25;
    const ICHG_CTRL_200MA: u8 = 0x35;

    match i2c.blocking_write(BQ25186_ADDR, &[REG_ICHG_CTRL, ICHG_CTRL_200MA]) {
        Ok(_) => info!("BQ25186 ICHG_CTRL set to 0x35 (200mA)"),
        Err(e) => {
            warn!("BQ25186 ICHG_CTRL write failed: {=?}", e);
            return;
        }
    }

    match read_bq_register(i2c, REG_ICHG_CTRL) {
        Some(value) => info!("BQ25186 ICHG_CTRL readback = 0x{=u8:02x}", value),
        None => warn!("BQ25186 ICHG_CTRL readback unavailable"),
    }
}

fn read_bq_register_dump(
    i2c: &mut I2c<'static, embassy_stm32::mode::Blocking, embassy_stm32::i2c::mode::Master>,
) -> RegisterDump {
    let mut values = [None; BQ_DUMP_REG_COUNT];

    let mut register = BQ_DUMP_FIRST_REG;
    while register <= BQ_DUMP_LAST_REG {
        values[(register - BQ_DUMP_FIRST_REG) as usize] = read_bq_register_quiet(i2c, register);
        register += 1;
    }

    RegisterDump { values }
}

fn read_bq_register_quiet(
    i2c: &mut I2c<'static, embassy_stm32::mode::Blocking, embassy_stm32::i2c::mode::Master>,
    register: u8,
) -> Option<u8> {
    let mut value = [0u8; 1];
    match i2c.blocking_write_read(BQ25186_ADDR, &[register], &mut value) {
        Ok(_) => Some(value[0]),
        Err(_) => None,
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

fn log_bq_register_dump(dump: RegisterDump) {
    info!(
        "BQ25186 dump 00-0D: 00={} 01={} 02={} 03={} 04={} 05={} 06={} 07={} 08={} 09={} 0A={} 0B={} 0C={} 0D={}",
        HexByte(dump.values[0]),
        HexByte(dump.values[1]),
        HexByte(dump.values[2]),
        HexByte(dump.values[3]),
        HexByte(dump.values[4]),
        HexByte(dump.values[5]),
        HexByte(dump.values[6]),
        HexByte(dump.values[7]),
        HexByte(dump.values[8]),
        HexByte(dump.values[9]),
        HexByte(dump.values[10]),
        HexByte(dump.values[11]),
        HexByte(dump.values[12]),
        HexByte(dump.values[13])
    );
}

struct HexByte(Option<u8>);

impl defmt::Format for HexByte {
    fn format(&self, fmt: defmt::Formatter) {
        match self.0 {
            Some(value) => defmt::write!(fmt, "0x{=u8:02x}", value),
            None => defmt::write!(fmt, "ERR"),
        }
    }
}

fn log_charger_status(status: ChargerStatus, v_batt: VoltageReading) {
    let ts_open = (status.stat0 & 0x80) != 0;
    let chg_stat = (status.stat0 >> 5) & 0x03;
    let ilim_active = (status.stat0 & 0x10) != 0;
    let vdppm_active = (status.stat0 & 0x08) != 0;
    let vindpm_active = (status.stat0 & 0x04) != 0;
    let thermreg_active = (status.stat0 & 0x02) != 0;
    let vin_pgood = (status.stat0 & 0x01) != 0;

    let vin_ovp = (status.stat1 & 0x80) != 0;
    let buvlo = (status.stat1 & 0x40) != 0;
    let ts_stat = (status.stat1 >> 3) & 0x03;
    let safety_timer_fault = (status.stat1 & 0x04) != 0;
    let wake1 = (status.stat1 & 0x02) != 0;
    let wake2 = (status.stat1 & 0x01) != 0;

    let chg_state = match chg_stat {
        0b00 => "NotCharging",
        0b01 => "ConstantCurrent",
        0b10 => "ConstantVoltage",
        0b11 => "ChargeDone",
        _ => "Invalid",
    };
    let ts_state = match ts_stat {
        0b00 => "Normal",
        0b01 => "HotOrColdSuspend",
        0b10 => "CoolCurrentReduced",
        0b11 => "WarmVoltageReduced",
        _ => "Invalid",
    };
    let power_path = match (vin_pgood, buvlo) {
        (true, false) => "InputPowerGood",
        (true, true) => "InputPowerGoodBatteryUvlo",
        (false, false) => "BatteryOnlyOrNoInput",
        (false, true) => "BatteryUvloNoInput",
    };
    let battery_connected_inferred = v_batt.source_mv >= BATTERY_CONNECTED_MIN_MV;
    let charging = matches!(chg_stat, 0b01 | 0b10);
    let charge_mode = match chg_stat {
        0b01 => "ConstantCurrent",
        0b10 => "ConstantVoltage",
        _ => "None",
    };
    let input_current_limit_ma = input_current_limit_ma(status.tmr_ilim);

    info!(
        "BQ25186 regs: STAT0=0x{=u8:02x} STAT1=0x{=u8:02x} TMR_ILIM=0x{=u8:02x}",
        status.stat0, status.stat1, status.tmr_ilim
    );
    info!(
        "BQ25186 battery_connected_inferred={} v_batt={}.{:03}V power_path={} charging={} mode={} chg_state={}",
        battery_connected_inferred,
        v_batt.source_mv / 1000,
        v_batt.source_mv % 1000,
        power_path,
        charging,
        charge_mode,
        chg_state
    );
    info!(
        "BQ25186 limits: ILIM_ACTIVE={} VDPPM={} VINDPM={} THERM={} input_current_limit={}mA TMR_ILIM=0x{=u8:02x}",
        ilim_active,
        vdppm_active,
        vindpm_active,
        thermreg_active,
        input_current_limit_ma,
        status.tmr_ilim
    );
    info!(
        "BQ25186 detail: VIN_PGOOD={} VIN_OVP={} BUVLO={} TS_OPEN={} TS={} SAFETY_TMR={} WAKE1={} WAKE2={}",
        vin_pgood,
        vin_ovp,
        buvlo,
        ts_open,
        ts_state,
        safety_timer_fault,
        wake1,
        wake2
    );
}

fn input_current_limit_ma(tmr_ilim: u8) -> u16 {
    match tmr_ilim & 0x07 {
        0b000 => 50,
        0b001 => 100,
        0b010 => 200,
        0b011 => 300,
        0b100 => 400,
        0b101 => 500,
        0b110 => 665,
        0b111 => 1050,
        _ => unreachable!(),
    }
}
