// Board temperature sensor smoke test.
//
// Activity: reads the LIS2HH12, LIS2MDL, LR1121, and STM32U595 internal
// temperature sensors once per second and prints raw plus converted values.

#![no_std]
#![no_main]

use core::convert::Infallible;

use arbitrary_int::u24;
use defmt::{error, info, unwrap, warn};
use embassy_executor::Spawner;
use embassy_stm32::adc::adc4::{
    Averaging as Adc4Averaging, Resolution as Adc4Resolution, SampleTime as Adc4SampleTime,
};
use embassy_stm32::adc::{Adc, VrefInt};
use embassy_stm32::gpio::{Input, Output};
use embassy_stm32::i2c::{Config as I2cConfig, I2c, Master as I2cMaster};
use embassy_stm32::mode::Blocking;
use embassy_stm32::pac::{self, vrefbuf};
use embassy_stm32::peripherals::ADC4;
use embassy_stm32::rcc::*;
use embassy_stm32::spi::mode::Master;
use embassy_stm32::spi::{Config as SpiConfig, Error as SpiError, Spi};
use embassy_stm32::time::{mhz, Hertz};
use embassy_time::{Duration, Instant, Timer};
use embedded_hal::digital::InputPin;
use embedded_hal_async::{digital::Wait, spi::Operation};
use lr11xx::{ops, Lr11xx};
use raylar_board_v1p0::{AdcVoltages, Board, EbyteRf, Leds, SensI2C};
use {defmt_rtt as _, panic_probe as _};

const I2C_FREQUENCY: Hertz = Hertz(100_000);
const BUSY_STARTUP_TIMEOUT: Duration = Duration::from_millis(500);
const TCXO_STARTUP_DELAY_TICKS: u24 = u24::new(320);

const LIS2HH12_ADDR: u8 = 0x1D;
const LIS2HH12_REG_WHO_AM_I: u8 = 0x0F;
const LIS2HH12_REG_TEMP_L: u8 = 0x0B;
const LIS2HH12_REG_CTRL1: u8 = 0x20;

const LIS2MDL_ADDR: u8 = 0x1E;
const LIS2MDL_REG_WHO_AM_I: u8 = 0x4F;
const LIS2MDL_REG_CFG_A: u8 = 0x60;
const LIS2MDL_REG_CFG_C: u8 = 0x62;
const LIS2MDL_REG_TEMP_OUT_L: u8 = 0x6E;
const LIS2MDL_CFG_A_TEMP_COMP_10HZ_CONTINUOUS: u8 = 0x80;

const ADC_REFERENCE_MV: u32 = 2_500;
const ADC_CALIB_RESOLUTION_BITS: u32 = 14;
const ADC_CALIB_VREF_MV: u32 = 3_000;
const ADC_TEMP_NOMINAL_ZERO_MC: i32 = 25_000;
const TEMPSENSOR_CAL1_ADDR: usize = 0x0BFA_0710;
const TEMPSENSOR_CAL2_ADDR: usize = 0x0BFA_0742;
const VREFINT_CAL_ADDR: usize = 0x0BFA_07A5;
const TEMPSENSOR_CAL1_TEMP_C: i32 = 30;
const TEMPSENSOR_CAL2_TEMP_C: i32 = 130;
const VREFBUF_READY_TIMEOUT_SPINS: u32 = 1_000_000;

type RadioSpi = Spi<'static, Blocking, Master>;
type SensorI2c = I2c<'static, Blocking, I2cMaster>;

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
        sens_i2c,
        ebyte_rf,
        adc_voltages,
        ..
    } = Board::new(p);
    let Leds {
        sys_main_red,
        sys_main_green,
        ..
    } = leds;

    info!("Temperature sensors smoke test started");

    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    spawner.spawn(unwrap!(temps_task(
        sens_i2c,
        ebyte_rf,
        adc_voltages,
        sys_main_red
    )));

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
async fn temps_task(
    sens_i2c: SensI2C<'static>,
    ebyte_rf: EbyteRf<'static>,
    adc_voltages: AdcVoltages<'static>,
    mut led: Output<'static>,
) -> ! {
    let mut i2c = init_sens_i2c(sens_i2c);
    init_lis2hh12(&mut i2c);
    init_lis2mdl(&mut i2c);

    if enable_vrefbuf() {
        info!("ADC VREFBUF enabled, VREFBUF={}mV", ADC_REFERENCE_MV);
    } else {
        warn!("ADC VREFBUF did not report ready; STM32 ADC temperature may be invalid");
    }
    let mut adc_temp = init_stm32_temp_adc(adc_voltages);
    adc_temp.log_calibration();

    let mut radio = init_lr1121(ebyte_rf).await;

    loop {
        led.set_high();

        log_lis2hh12_temp(&mut i2c);
        log_lis2mdl_temp(&mut i2c);
        log_stm32_temp(&mut adc_temp);
        log_lr1121_temp(&mut radio).await;

        led.set_low();
        Timer::after_secs(1).await;
    }
}

fn init_sens_i2c(sens_i2c: SensI2C<'static>) -> SensorI2c {
    let SensI2C { i2c, scl, sda } = sens_i2c;
    let mut config = I2cConfig::default();
    config.frequency = I2C_FREQUENCY;
    I2c::new_blocking(i2c, scl, sda, config)
}

fn init_lis2hh12(i2c: &mut SensorI2c) {
    let mut whoami = [0u8; 1];
    match i2c.blocking_write_read(LIS2HH12_ADDR, &[LIS2HH12_REG_WHO_AM_I], &mut whoami) {
        Ok(_) => info!("LIS2HH12 WHO_AM_I = {=u8:#x}", whoami[0]),
        Err(e) => warn!("LIS2HH12 WHO_AM_I error: {:?}", e),
    }

    let ctrl1 = 0b0101_0111;
    match i2c.blocking_write(LIS2HH12_ADDR, &[LIS2HH12_REG_CTRL1, ctrl1]) {
        Ok(_) => info!("LIS2HH12 started"),
        Err(e) => warn!("LIS2HH12 config error: {:?}", e),
    }
}

fn init_lis2mdl(i2c: &mut SensorI2c) {
    let mut whoami = [0u8; 1];
    match i2c.blocking_write_read(LIS2MDL_ADDR, &[LIS2MDL_REG_WHO_AM_I], &mut whoami) {
        Ok(_) => info!("LIS2MDL WHO_AM_I = {=u8:#x}", whoami[0]),
        Err(e) => warn!("LIS2MDL WHO_AM_I error: {:?}", e),
    }

    match i2c.blocking_write(
        LIS2MDL_ADDR,
        &[LIS2MDL_REG_CFG_A, LIS2MDL_CFG_A_TEMP_COMP_10HZ_CONTINUOUS],
    ) {
        Ok(_) => info!("LIS2MDL configured"),
        Err(e) => warn!("LIS2MDL config A error: {:?}", e),
    }
    match i2c.blocking_write(LIS2MDL_ADDR, &[LIS2MDL_REG_CFG_C, 0x00]) {
        Ok(_) => info!("LIS2MDL continuous mode"),
        Err(e) => warn!("LIS2MDL config C error: {:?}", e),
    }
}

fn log_lis2hh12_temp(i2c: &mut SensorI2c) {
    let mut raw = [0u8; 2];
    match i2c.blocking_write_read(LIS2HH12_ADDR, &[LIS2HH12_REG_TEMP_L | 0x80], &mut raw) {
        Ok(_) => {
            let raw16 = i16::from_le_bytes(raw);
            let raw11 = raw16 >> 5;
            let temp_mc = nominal_temp_millicelsius_from_lsb8(raw11);
            info!(
                "LIS2HH12 temp raw16={} raw11={} approx={}.{:03}C",
                raw16,
                raw11,
                temp_mc / 1000,
                (temp_mc % 1000).abs()
            );
        }
        Err(e) => warn!("LIS2HH12 temp error: {:?}", e),
    }
}

fn log_lis2mdl_temp(i2c: &mut SensorI2c) {
    let mut raw = [0u8; 2];
    match i2c.blocking_write_read(LIS2MDL_ADDR, &[LIS2MDL_REG_TEMP_OUT_L | 0x80], &mut raw) {
        Ok(_) => {
            let raw16 = i16::from_le_bytes(raw);
            let raw12 = sign_extend_12(raw16);
            let temp_mc = nominal_temp_millicelsius_from_lsb8(raw12);
            info!(
                "LIS2MDL temp raw16={} raw12={} approx={}.{:03}C",
                raw16,
                raw12,
                temp_mc / 1000,
                (temp_mc % 1000).abs()
            );
        }
        Err(e) => warn!("LIS2MDL temp error: {:?}", e),
    }
}

fn nominal_temp_millicelsius_from_lsb8(raw: i16) -> i32 {
    ADC_TEMP_NOMINAL_ZERO_MC + ((raw as i32 * 1000) / 8)
}

fn sign_extend_12(raw: i16) -> i16 {
    (raw << 4) >> 4
}

struct Stm32TempAdc {
    adc4: Adc<'static, ADC4>,
    vrefint: VrefInt,
    temperature: embassy_stm32::adc::Temperature,
    sample_time: Adc4SampleTime,
    calibration: Stm32TempCalibration,
}

#[derive(Copy, Clone)]
struct Stm32TempCalibration {
    ts_cal1: u16,
    ts_cal2: u16,
    vrefint_cal: u16,
}

impl Stm32TempAdc {
    fn log_calibration(&self) {
        info!(
            "STM32 temp calibration ts_cal1={}@{}C ts_cal2={}@{}C vrefint_cal={}@{}mV",
            self.calibration.ts_cal1,
            TEMPSENSOR_CAL1_TEMP_C,
            self.calibration.ts_cal2,
            TEMPSENSOR_CAL2_TEMP_C,
            self.calibration.vrefint_cal,
            ADC_CALIB_VREF_MV
        );
    }
}

fn init_stm32_temp_adc(adc_voltages: AdcVoltages<'static>) -> Stm32TempAdc {
    let AdcVoltages { adc4, .. } = adc_voltages;

    let mut adc4 = Adc::new_adc4(adc4);
    disable_adc4_for_internal_channel_selection();
    adc4.set_resolution_adc4(Adc4Resolution::BITS12);
    adc4.set_averaging_adc4(Adc4Averaging::Samples256);
    let vrefint = adc4.enable_vrefint_adc4();
    let temperature = adc4.enable_temperature_adc4();
    info!(
        "ADC4 internal channels: vrefen={} vsensesel={}",
        pac::ADC4.ccr().read().vrefen(),
        pac::ADC4.ccr().read().vsensesel()
    );

    Stm32TempAdc {
        adc4,
        vrefint,
        temperature,
        sample_time: Adc4SampleTime::CYCLES814_5,
        calibration: Stm32TempCalibration::read(),
    }
}

fn log_stm32_temp(adc: &mut Stm32TempAdc) {
    let vref_raw = adc.adc4.blocking_read(&mut adc.vrefint, adc.sample_time) as u32;
    let temp_raw = adc
        .adc4
        .blocking_read(&mut adc.temperature, adc.sample_time) as u32;
    let vdda_mv = adc.calibration.vdda_mv(vref_raw);
    let temp_mc = adc.calibration.temp_millicelsius(temp_raw, vdda_mv);

    if vref_raw == 0 || vref_raw == 4095 || temp_raw == 4095 {
        warn!(
            "STM32 ADC4 suspicious raw values temp_raw={} vref_raw={}; check VREFBUF and ADC4 internal channel enable",
            temp_raw,
            vref_raw
        );
    }

    info!(
        "STM32U595 temp raw={} vref_raw={} vdda={}.{:03}V temp={}.{:03}C",
        temp_raw,
        vref_raw,
        vdda_mv / 1000,
        vdda_mv % 1000,
        temp_mc / 1000,
        (temp_mc % 1000).abs()
    );
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

fn disable_adc4_for_internal_channel_selection() {
    let cr = pac::ADC4.cr().read();
    if cr.adstart() {
        pac::ADC4.cr().modify(|w| w.set_adstp(true));
        while pac::ADC4.cr().read().adstart() {}
    }

    if cr.aden() || cr.adstart() {
        pac::ADC4.cr().modify(|w| w.set_addis(true));
        while pac::ADC4.cr().read().aden() {}
    }
}

impl Stm32TempCalibration {
    fn read() -> Self {
        Self {
            ts_cal1: read_u16_system_memory(TEMPSENSOR_CAL1_ADDR),
            ts_cal2: read_u16_system_memory(TEMPSENSOR_CAL2_ADDR),
            vrefint_cal: read_u16_system_memory(VREFINT_CAL_ADDR),
        }
    }

    fn vdda_mv(self, vref_raw_12: u32) -> u32 {
        if vref_raw_12 == 0 {
            return ADC_CALIB_VREF_MV;
        }

        (self.vrefint_cal as u32 * ADC_CALIB_VREF_MV) / adc12_to_calib14(vref_raw_12)
    }

    fn temp_millicelsius(self, temp_raw_12: u32, vdda_mv: u32) -> i32 {
        let temp_raw_calib = ((adc12_to_calib14(temp_raw_12) * vdda_mv) / ADC_CALIB_VREF_MV) as i32;
        let ts_cal1 = self.ts_cal1 as i32;
        let ts_cal2 = self.ts_cal2 as i32;
        let delta_cal = ts_cal2 - ts_cal1;

        if delta_cal == 0 {
            return temp_raw_calib * 10;
        }

        TEMPSENSOR_CAL1_TEMP_C * 1000
            + ((TEMPSENSOR_CAL2_TEMP_C - TEMPSENSOR_CAL1_TEMP_C)
                * 1000
                * (temp_raw_calib - ts_cal1))
                / delta_cal
    }
}

fn adc12_to_calib14(raw: u32) -> u32 {
    raw << (ADC_CALIB_RESOLUTION_BITS - 12)
}

fn read_u16_system_memory(addr: usize) -> u16 {
    // Some STM32U5 calibration words are at odd byte addresses in ST's headers.
    let lo = unsafe { core::ptr::read_volatile(addr as *const u8) };
    let hi = unsafe { core::ptr::read_volatile((addr + 1) as *const u8) };
    u16::from_le_bytes([lo, hi])
}

async fn init_lr1121(rf: EbyteRf<'static>) -> Lr11xx<ManualSpiDevice, BusyPoll> {
    let EbyteRf {
        spi,
        sck,
        miso,
        mosi,
        mut cs,
        busy,
        mut nrst,
        ..
    } = rf;

    cs.set_high();

    info!("Resetting LR1121");
    nrst.set_low();
    Timer::after_millis(10).await;
    nrst.set_high();
    Timer::after_millis(25).await;

    let mut spi_config = SpiConfig::default();
    spi_config.frequency = mhz(1);
    let spi = Spi::new_blocking(spi, sck, mosi, miso, spi_config);
    let spi_device = ManualSpiDevice::new(spi, cs);
    let mut busy = BusyPoll::new(busy);

    info!(
        "RF_BUSY after reset: high={}",
        busy.is_high().unwrap_or(false)
    );
    if !wait_busy_low(&mut busy, BUSY_STARTUP_TIMEOUT).await {
        error!("RF_BUSY stayed high after reset");
        pending_forever().await;
    }

    let mut radio = match Lr11xx::new(spi_device, busy).await {
        Ok(radio) => radio,
        Err(e) => {
            error!("Lr11xx::new failed: {}", e);
            pending_forever().await;
        }
    };

    log_result("lr1121_clear_errors", radio.clear_errors().await);
    log_result("lr1121_standby_rc", radio.standby(false).await);
    log_result(
        "lr1121_set_tcxo_mode_1v8",
        radio
            .set_tcxo_mode(
                ops::TcxoMode::builder()
                    .with_delay(TCXO_STARTUP_DELAY_TICKS)
                    .with_tune(ops::TcxoTune::V1p8)
                    .build(),
            )
            .await,
    );
    log_result("lr1121_standby_xosc", radio.standby(true).await);

    radio
}

async fn log_lr1121_temp<S, B>(radio: &mut Lr11xx<S, B>)
where
    S: embedded_hal_async::spi::SpiDevice<u8>,
    B: InputPin + Wait,
{
    match radio.temp().await {
        Ok(temp_c) => info!("LR1121 temp raw=n/a temp={}C", temp_c),
        Err(e) => warn!("LR1121 temp error: {}", e),
    }
}

fn log_result<T: defmt::Format>(label: &str, result: lr11xx::Result<T>) {
    match result {
        Ok(value) => info!("{} ok: {}", label, value),
        Err(e) => warn!("{} failed: {}", label, e),
    }
}

struct BusyPoll {
    pin: Input<'static>,
}

impl BusyPoll {
    fn new(pin: Input<'static>) -> Self {
        Self { pin }
    }
}

impl embedded_hal::digital::ErrorType for BusyPoll {
    type Error = Infallible;
}

impl InputPin for BusyPoll {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_high())
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(self.pin.is_low())
    }
}

impl Wait for BusyPoll {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        while self.pin.is_low() {
            Timer::after_millis(1).await;
        }
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        while self.pin.is_high() {
            Timer::after_millis(1).await;
        }
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_low().await?;
        self.wait_for_high().await
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        self.wait_for_high().await?;
        self.wait_for_low().await
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        let initial = self.pin.is_high();
        while self.pin.is_high() == initial {
            Timer::after_millis(1).await;
        }
        Ok(())
    }
}

async fn wait_busy_low(busy: &mut BusyPoll, timeout: Duration) -> bool {
    let start = Instant::now();
    while busy.is_high().unwrap_or(true) {
        if Instant::now().duration_since(start) >= timeout {
            return false;
        }
        Timer::after_millis(1).await;
    }
    true
}

struct ManualSpiDevice {
    spi: RadioSpi,
    cs: Output<'static>,
}

impl ManualSpiDevice {
    fn new(spi: RadioSpi, mut cs: Output<'static>) -> Self {
        cs.set_high();
        Self { spi, cs }
    }
}

impl embedded_hal_async::spi::ErrorType for ManualSpiDevice {
    type Error = SpiError;
}

impl embedded_hal_async::spi::SpiDevice<u8> for ManualSpiDevice {
    async fn transaction(
        &mut self,
        operations: &mut [Operation<'_, u8>],
    ) -> Result<(), Self::Error> {
        self.cs.set_low();
        let mut result = Ok(());

        for operation in operations {
            result = match operation {
                Operation::Read(words) => self.spi.blocking_read(words),
                Operation::Write(words) => self.spi.blocking_write(words),
                Operation::Transfer(read, write) => self.spi.blocking_transfer(read, write),
                Operation::TransferInPlace(words) => self.spi.blocking_transfer_in_place(words),
                Operation::DelayNs(ns) => {
                    Timer::after_micros(((*ns as u64) + 999) / 1000).await;
                    Ok(())
                }
            };

            if result.is_err() {
                break;
            }
        }

        self.cs.set_high();
        result
    }
}

async fn pending_forever() -> ! {
    loop {
        Timer::after_secs(60).await;
    }
}
