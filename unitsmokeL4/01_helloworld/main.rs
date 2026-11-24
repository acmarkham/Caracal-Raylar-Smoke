//! ADR-L4-006: external HSE -> PLL, Embassy TIM5 and active-low LED readback.
#![no_std]
#![no_main]

use defmt::{assert, assert_eq, info};
use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_stm32::rcc::{
    self, AHBPrescaler, APBPrescaler, Hse, HseMode, Pll, PllMul, PllPreDiv, PllRDiv, PllSource,
    Sysclk,
};
use embassy_stm32::{pac, time::Hertz};
use embassy_time::{Duration, Instant, Ticker};
use {defmt_rtt as _, panic_probe as _};

const HSE_HZ: u32 = 16_000_000;
const SYS_HZ: u32 = 80_000_000;
const VERIFIED_EDGES: u32 = 20;

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let mut config = embassy_stm32::Config::default();
    config.rcc.hse = Some(Hse {
        freq: Hertz(HSE_HZ),
        mode: HseMode::Oscillator,
    });
    config.rcc.pll = Some(Pll {
        source: PllSource::HSE,
        prediv: PllPreDiv::DIV2,
        mul: PllMul::MUL20,
        divp: None,
        divq: None,
        divr: Some(PllRDiv::DIV2),
    });
    config.rcc.sys = Sysclk::PLL1_R;
    config.rcc.ahb_pre = AHBPrescaler::DIV1;
    config.rcc.apb1_pre = APBPrescaler::DIV1;
    config.rcc.apb2_pre = APBPrescaler::DIV1;
    config.rcc.ls = rcc::LsConfig::off();

    let p = embassy_stm32::init(config);
    // All Evo LEDs are active low. Keep red and GPS green off in this test.
    let _red = Output::new(p.PB11, Level::High, Speed::Low);
    let _green = Output::new(p.PC3, Level::High, Speed::Low);
    let mut blue = Output::new(p.PC1, Level::High, Speed::Low);

    let device_id = pac::DBGMCU.idcode().read().0;
    // DS10198: read-only factory flash-size signature, in KiB.
    let flash_kib = unsafe { core::ptr::read_volatile(0x1fff_75e0 as *const u16) };
    info!(
        "L4 HELLOWORLD boot board=CaracalEvo chip=STM32L476RG device_id={:08x} flash_kib={}",
        device_id, flash_kib
    );
    assert_eq!(device_id & 0xfff, 0x415);
    assert_eq!(flash_kib, 1024);

    let clocks = rcc::clocks(&p.RCC);
    assert_eq!(clocks.hse.to_hertz().unwrap().0, HSE_HZ);
    assert_eq!(clocks.sys.to_hertz().unwrap().0, SYS_HZ);
    assert_eq!(clocks.hclk1.to_hertz().unwrap().0, SYS_HZ);
    assert_eq!(clocks.pclk1.to_hertz().unwrap().0, SYS_HZ);
    assert_eq!(clocks.pclk2.to_hertz().unwrap().0, SYS_HZ);
    let cr = pac::RCC.cr().read();
    let cfgr = pac::RCC.cfgr().read();
    let pll = pac::RCC.pllcfgr().read();
    info!(
        "L4 clocks cr={:08x} cfgr={:08x} pllcfgr={:08x} sys_hz={} tim5_psc={}",
        cr.0,
        cfgr.0,
        pll.0,
        SYS_HZ,
        pac::TIM5.psc().read()
    );
    assert!(cr.hserdy() && cr.pllrdy() && !cr.hsebyp());
    assert!(cfgr.sws() == Sysclk::PLL1_R);
    assert!(cfgr.hpre() == AHBPrescaler::DIV1);
    assert!(cfgr.ppre1() == APBPrescaler::DIV1);
    assert!(cfgr.ppre2() == APBPrescaler::DIV1);
    assert!(pll.pllsrc() == PllSource::HSE);
    assert!(pll.pllm() == PllPreDiv::DIV2);
    assert!(pll.plln() == PllMul::MUL20);
    assert!(pll.pllr() == PllRDiv::DIV2 && pll.pllren());
    assert!(pac::PWR.cr1().read().vos() == pac::pwr::vals::Vos::RANGE1);
    assert_eq!(pac::FLASH.acr().read().latency(), 4);
    assert_eq!(pac::TIM5.psc().read(), 79);
    info!(
        "L4 CLOCK PASS hse_hz={} sys_hz={} tick_hz=1000000",
        HSE_HZ, SYS_HZ
    );

    let mut edges = 0u32;
    let mut on = false;
    let mut previous = Instant::now();
    let started = previous;
    let mut ticker = Ticker::every(Duration::from_millis(500));
    loop {
        ticker.next().await;
        let now = Instant::now();
        let elapsed_us = (now - previous).as_micros();
        previous = now;
        assert!((490_000..=510_000).contains(&elapsed_us));

        on = !on;
        if on {
            blue.set_low();
        } else {
            blue.set_high();
        }
        // Allow the output pad to settle, then read hardware rather than cached state.
        cortex_m::asm::delay(80);
        let odr_high = pac::GPIOC.odr().read().0 & (1 << 1) != 0;
        let idr_high = pac::GPIOC.idr().read().0 & (1 << 1) != 0;
        assert_eq!(odr_high, !on);
        assert_eq!(idr_high, !on);
        assert!(pac::GPIOB.idr().read().0 & (1 << 11) != 0);
        assert!(pac::GPIOC.idr().read().0 & (1 << 3) != 0);
        edges = edges.wrapping_add(1);
        info!(
            "L4 LED edge={} on={} elapsed_us={} odr_high={} idr_high={}",
            edges, on, elapsed_us, odr_high, idr_high
        );
        if edges == VERIFIED_EDGES {
            info!(
                "L4 HELLOWORLD PASS edges={} elapsed_ms={} (GPIO readback; optical check separate)",
                edges,
                (now - started).as_millis()
            );
        }
    }
}
