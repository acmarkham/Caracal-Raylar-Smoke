#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Leds};
use raylar_drivers::leds::{init, LedName, LedResources};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 8 * 1024;
#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

const ALL: [LedName; 5] = [LedName::SysGpsGreen, LedName::SysGpsRed, LedName::SysMainRed, LedName::SysMainGreen, LedName::SysSdBlue];

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    unsafe { embedded_alloc::init!(ALLOCATOR, HEAP_BYTES); }
    let p = embassy_stm32::init(Default::default());
    let Board { leds, .. } = Board::new(p);
    let Leds { sys_gps_green, sys_gps_red, sys_main_red, sys_main_green, sys_sd_blue } = leds;
    let mut leds = init(LedResources { sys_gps_green, sys_gps_red, sys_main_red, sys_main_green, sys_sd_blue });
    loop {
        for led in ALL {
            leds.on(led); Timer::after(Duration::from_millis(200)).await;
            leds.off(led); Timer::after(Duration::from_millis(100)).await;
        }
        for led in ALL { leds.toggle(led); }
        Timer::after(Duration::from_millis(200)).await;
        for led in ALL { leds.toggle(led); }
    }
}
