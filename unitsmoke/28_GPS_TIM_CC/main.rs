// GPS PPS EXTI + TIM4 capture/compare smoke test.
//
// Activity: GPS_PPS on PB9 is configured both as EXTI line 9 and as TIM4_CH4
// input capture through AF2. Each PPS rising edge is timestamped with Embassy
// system time from the EXTI path, then the latest TIM4_CH4 capture value is
// read and logged with deltas from the previous PPS edge.
//
// Clock check: HSE=16 MHz, PLL1_R = 16 MHz / 1 * 10 / 2 = 80 MHz SYSCLK.
// APB1 defaults to DIV1, so TIM4 is clocked at 80 MHz. The input-capture
// driver sets TIM4 to 1 MHz, so each TIM4 tick is 1 us. TIM4 is a 32-bit
// general-purpose timer on STM32U595, so the capture value is a 32-bit counter.

#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::{AfType, Flex, Level, Output, Pull, Speed};
use embassy_stm32::interrupt;
use embassy_stm32::mode::Async;
use embassy_stm32::peripherals::TIM4;
use embassy_stm32::rcc::*;
use embassy_stm32::time::{hz, mhz};
use embassy_stm32::timer::input_capture::{CapturePin, Ch1, Ch2, Ch3, Ch4, InputCapture};
use embassy_stm32::timer::low_level::{CountingMode, InputCaptureMode, InputTISelection};
use embassy_stm32::timer::{self, Channel, CoreInstance};
use embassy_stm32::{bind_interrupts, exti, Peripherals};
use embassy_time::{Duration, Instant, Timer};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    EXTI9 => exti::InterruptHandler<interrupt::typelevel::EXTI9>;
    TIM4 => timer::CaptureCompareInterruptHandler<TIM4>;
});

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
    let Peripherals {
        PB9,
        EXTI9,
        TIM4,
        PC13,
        PE3,
        PD7,
        ..
    } = p;

    let mut gps_en = Output::new(PC13, Level::Low, Speed::Medium);
    let mut gps_rst = Output::new(PE3, Level::Low, Speed::Medium);
    let led = Output::new(PD7, Level::Low, Speed::Medium);

    info!("GPS PPS TIM4 capture smoke test started");
    info!("TIM4_CH4 input capture on PB9 AF2 at 1000000 Hz");

    gps_en.set_high();
    gps_rst.set_high();
    info!("GPS_EN=HIGH GPS_RST=HIGH");

    Timer::after(Duration::from_millis(250)).await;

    let pps_af_pin = unsafe { PB9.clone_unchecked() };
    let pps = ExtiInput::new(PB9, EXTI9, Pull::None, Irqs);

    let mut pps_af = Flex::new(pps_af_pin);
    pps_af.set_as_af_unchecked(2, AfType::input(Pull::None));

    let mut tim4_capture: InputCapture<'static, TIM4> = InputCapture::new(
        TIM4,
        None::<CapturePin<'static, TIM4, Ch1>>,
        None::<CapturePin<'static, TIM4, Ch2>>,
        None::<CapturePin<'static, TIM4, Ch3>>,
        None::<CapturePin<'static, TIM4, Ch4>>,
        Irqs,
        hz(1_000_000),
        CountingMode::EdgeAlignedUp,
    );

    tim4_capture.set_input_ti_selection(Channel::Ch4, InputTISelection::Normal);
    tim4_capture.set_input_capture_mode(Channel::Ch4, InputCaptureMode::Rising);
    tim4_capture.enable(Channel::Ch4);
    clear_tim4_ch4_capture_flag();

    spawner.spawn(unwrap!(pps_capture_task(pps, pps_af, tim4_capture, led)));

    core::future::pending().await
}

#[embassy_executor::task]
async fn pps_capture_task(
    mut pps: ExtiInput<'static, Async>,
    _pps_af: Flex<'static>,
    tim4_capture: InputCapture<'static, TIM4>,
    mut led: Output<'static>,
) -> ! {
    let mut last_systime_us = None;
    let mut last_cc4 = None;

    loop {
        pps.wait_for_rising_edge().await;
        let systime_us = Instant::now().as_micros();
        let cc4if = tim4_capture.get_input_interrupt(Channel::Ch4);
        let cc4: u32 = tim4_capture.get_capture_value(Channel::Ch4);
        clear_tim4_ch4_capture_flag();

        if let (Some(prev_systime_us), Some(prev_cc4)) = (last_systime_us, last_cc4) {
            let systime_delta_us = systime_us.wrapping_sub(prev_systime_us);
            let tim4_delta_us = cc4.wrapping_sub(prev_cc4);

            info!(
                "PPS systime_us={} tim4_cc4={} systime_delta_us={} tim4_delta_us={} cc4if={}",
                systime_us, cc4, systime_delta_us, tim4_delta_us, cc4if
            );
        } else {
            info!(
                "PPS systime_us={} tim4_cc4={} systime_delta_us=first tim4_delta_us=first cc4if={}",
                systime_us, cc4, cc4if
            );
        }

        last_systime_us = Some(systime_us);
        last_cc4 = Some(cc4);

        led.set_high();
        Timer::after(Duration::from_millis(50)).await;
        led.set_low();
    }
}

fn clear_tim4_ch4_capture_flag() {
    let regs =
        unsafe { embassy_stm32::pac::timer::TimGp16::from_ptr(<TIM4 as CoreInstance>::regs()) };
    regs.sr()
        .modify(|w| w.set_ccif(Channel::Ch4.index(), false));
}
