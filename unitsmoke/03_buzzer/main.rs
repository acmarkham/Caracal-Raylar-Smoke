#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_stm32::mode::Async;
use embassy_time::{Duration, Timer};
use raylar_board_v1p0::{Board, Buzzer, Leds};
use {defmt_rtt as _, panic_probe as _};
// pwm imports
use embassy_stm32::gpio::OutputType;
use embassy_stm32::time::hz;
use embassy_stm32::timer::complementary_pwm::{
    ComplementaryPwm,
    ComplementaryPwmPin,
};
use embassy_stm32::timer::low_level::CountingMode;
use embassy_stm32::timer::Channel;

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let p = embassy_stm32::init(Default::default());
    let Board { leds, buzzer ,..} = Board::new(p);
    let Leds {
        sys_main_red,
        sys_main_green,
        ..
    } = leds;
    

    info!("Buzzer smoke test started");

    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    spawner.spawn(unwrap!(buzzer_task(buzzer, sys_main_red)));

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
async fn buzzer_task(
    buzzer: Buzzer<'static>,
    mut led: Output<'static>,
) -> ! {
    let Buzzer { tim, pin } = buzzer;

    let ch1n = ComplementaryPwmPin::new(pin, OutputType::PushPull);

    let mut pwm = ComplementaryPwm::new(
        tim,
        None,
        Some(ch1n),
        None,
        None,
        None,
        None,
        None,
        None,
        hz(1000),
        CountingMode::EdgeAlignedUp,
    );

    loop {
        led.set_high();

        let max = pwm.get_max_duty();
        pwm.set_duty(Channel::Ch1, max / 2);
        pwm.enable(Channel::Ch1);

        Timer::after_secs(1).await;

        led.set_low();

        pwm.disable(Channel::Ch1);
        

        Timer::after_secs(1).await;
    }
}
