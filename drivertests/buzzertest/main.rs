#![no_std]
#![no_main]

use defmt::unwrap;
use embassy_executor::Spawner;
use embassy_time::Duration;
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::Board;
use raylar_drivers::buzzer::{init, BuzzerResources, PitchHz, Volume};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 8 * 1024;

#[global_allocator]
static ALLOCATOR: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    unsafe { embedded_alloc::init!(ALLOCATOR, HEAP_BYTES); }
    let p = embassy_stm32::init(Default::default());
    let Board { buzzer, .. } = Board::new(p);
    let raylar_board_v1p0::Buzzer { tim, pin } = buzzer;
    let mut buzzer = init(BuzzerResources { timer: tim, pin });

    loop {
        for pitch_hz in (250..=2_500).step_by(250) {
            for duration_ms in (10..=500).step_by(100) {
                for volume in (0..=250).step_by(100).chain(core::iter::once(255)) {
                    unwrap!(buzzer
                        .play_tone(
                            PitchHz(pitch_hz),
                            Duration::from_millis(duration_ms),
                            Volume(volume),
                        )
                        .await);
                }
            }
        }
    }
}
