use crate::{fault, streams, GPS, TIME};
use aardwolf_l4::{policy, ui_policy as sound};
use caracal_evo::buzzer::Buzzer;
use core::sync::atomic::Ordering::Relaxed;
use embassy_stm32::gpio::{Input, Level, Output};
use embassy_time::{Instant, Timer};

#[embassy_executor::task]
pub async fn task(
    mut red: Output<'static>,
    mut blue: Output<'static>,
    mut green: Output<'static>,
    detect: Input<'static>,
    mut buzzer: Buzzer,
) -> ! {
    let boot = Instant::now();
    let mut fault_at = None;
    let mut observed = 0;
    let mut green_until = boot;
    let mut card = sound::CardDebounce::default();
    let mut previous = (u32::MAX, 0);
    loop {
        if card.removed(detect.is_high()) {
            fault::latch(fault::Fault::SdRemoved);
        }
        let now = Instant::now();
        let failed = fault::active();
        let (hz, volume) = if failed {
            let start = *fault_at.get_or_insert(now);
            let ms = (now - start).as_millis();
            red.set_level(if sound::red_on(ms) {
                Level::Low
            } else {
                Level::High
            });
            (sound::error_hz(ms), sound::ERROR_VOLUME)
        } else if cfg!(feature = "bench-audio-ui")
            && (220..282).contains(&crate::audio::PUBLISHED_BLOCKS.load(Relaxed))
        {
            // Finite acoustic loopback: verification checks the 1 kHz peak in
            // the first WAV. Production only plays startup/fault indications.
            (1000, sound::STARTUP_VOLUME)
        } else {
            (
                sound::startup_hz((now - boot).as_millis()),
                sound::STARTUP_VOLUME,
            )
        };
        if previous != (hz, volume) {
            buzzer.set(hz, volume);
            if cfg!(feature = "bench-audio-ui") && hz == 1000 {
                let tim = embassy_stm32::pac::TIM8;
                defmt::info!(
                    "AARDWOLF L4 test tone blocks={} ticks={} tim8_psc={} arr={} ccer={:08x}",
                    crate::audio::PUBLISHED_BLOCKS.load(Relaxed),
                    now.as_ticks(),
                    tim.psc().read(),
                    tim.arr().read(),
                    tim.ccer().read().0
                );
            }
            previous = (hz, volume);
        }
        let gps = GPS.stats();
        let searching = matches!(
            gps.operating_state,
            raylar_drivers::gps::OperatingState::Searching
                | raylar_drivers::gps::OperatingState::Reacquiring
        );
        let active = gps.operating_state.is_tracking() || searching;
        if policy::gps_green_on(active, searching, &TIME.time_state(), &mut observed) {
            green_until = now + embassy_time::Duration::from_millis(50);
        }
        green.set_level(if active && (searching || now < green_until) {
            Level::Low
        } else {
            Level::High
        });
        blue.set_level(if !failed && streams::AUDIO_OPEN.load(Relaxed) {
            Level::Low
        } else {
            Level::High
        });
        Timer::after_millis(10).await;
    }
}
