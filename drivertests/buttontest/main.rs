#![no_std]
#![no_main]

use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_stm32::gpio::Output;
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Buttons, Leds};
use raylar_drivers::button::{init, ButtonResources, ButtonName};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 8 * 1024;
#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe { embedded_alloc::init!(ALLOCATOR, HEAP_BYTES); }
    let p = embassy_stm32::init(Default::default());
    let Board { leds, buttons, .. } = Board::new(p);
    let Leds { sys_main_red, sys_main_green, .. } = leds;
    let Buttons { user } = buttons;
    let buttons = init(ButtonResources { user });
    spawner.spawn(unwrap!(heartbeat_task(sys_main_green)));
    user_button_task(buttons, sys_main_red).await
}

#[embassy_executor::task]
async fn heartbeat_task(mut led: Output<'static>) -> ! {
    loop { led.set_high(); Timer::after_millis(100).await; led.set_low(); Timer::after_millis(900).await; }
}

async fn user_button_task(
    mut buttons: raylar_drivers::button::ButtonDriver<'static>,
    mut led: Output<'static>,
) -> ! {
    loop {
        buttons.wait_for_press(ButtonName::User).await;
        led.toggle();
        buttons.wait_for_release(ButtonName::User).await;
    }
}
