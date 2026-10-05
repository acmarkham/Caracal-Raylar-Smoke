use defmt::warn;
use embassy_stm32::gpio::Output;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::channel::Channel;
use embassy_time::{Duration, Timer};
use raylar_drivers::buzzer::{BuzzerDriver, PitchHz, Volume};
use raylar_radio_service::FrameType;

use integration004_radioheartbeat::policy::Role;

static COMMANDS: Channel<CriticalSectionRawMutex, Command, 32> = Channel::new();

const ACTIVITY_DURATION: Duration = Duration::from_millis(40);
const PACKET_BEEP_DURATION: Duration = Duration::from_millis(35);
const BEEP_VOLUME: Volume = Volume(96);

#[derive(Clone, Copy)]
enum Command {
    Startup,
    GpsLock,
    UtcCalibrated,
    Transmit,
    Packet(FrameType),
    Fatal,
}

pub fn startup() {
    let _ = COMMANDS.try_send(Command::Startup);
}

pub fn gps_lock() {
    let _ = COMMANDS.try_send(Command::GpsLock);
}

pub fn utc_calibrated() {
    let _ = COMMANDS.try_send(Command::UtcCalibrated);
}

pub fn transmit() {
    let _ = COMMANDS.try_send(Command::Transmit);
}

pub fn packet_received(frame_type: FrameType) {
    let _ = COMMANDS.try_send(Command::Packet(frame_type));
}

pub fn fatal() {
    let _ = COMMANDS.try_send(Command::Fatal);
}

#[embassy_executor::task]
pub async fn role_led_task(mut role_led: Output<'static>, role: Role) -> ! {
    if role.is_base_station() {
        role_led.set_high();
    } else {
        role_led.set_low();
    }
    loop {
        // Keep ownership and periodically restore the latched indication.
        Timer::after_secs(60).await;
        if role.is_base_station() {
            role_led.set_high();
        } else {
            role_led.set_low();
        }
    }
}

#[embassy_executor::task]
pub async fn activity_task(
    mut tx_blue_led: Output<'static>,
    mut rx_led: Output<'static>,
    mut tx_fatal_red_led: Output<'static>,
    mut buzzer: BuzzerDriver<'static>,
) -> ! {
    tx_blue_led.set_low();
    rx_led.set_low();
    tx_fatal_red_led.set_low();
    let mut fatal_latched = false;
    loop {
        match COMMANDS.receive().await {
            Command::Startup => play_startup(&mut buzzer).await,
            Command::GpsLock => {
                play_tone(&mut buzzer, 880, Duration::from_millis(140)).await;
            }
            Command::UtcCalibrated => {
                play_tone(&mut buzzer, 1_047, Duration::from_millis(90)).await;
                Timer::after_millis(45).await;
                play_tone(&mut buzzer, 1_568, Duration::from_millis(140)).await;
            }
            Command::Transmit => {
                tx_blue_led.set_high();
                tx_fatal_red_led.set_high();
                Timer::after(ACTIVITY_DURATION).await;
                tx_blue_led.set_low();
                if !fatal_latched {
                    tx_fatal_red_led.set_low();
                }
            }
            Command::Packet(frame_type) => {
                rx_led.set_high();
                let pitch_hz = match frame_type {
                    FrameType::Heartbeat => 2_200,
                    FrameType::Presence => 1_200,
                    FrameType::Data => 1_600,
                };
                play_tone(&mut buzzer, pitch_hz, PACKET_BEEP_DURATION).await;
                rx_led.set_low();
            }
            Command::Fatal => {
                fatal_latched = true;
                tx_fatal_red_led.set_high();
            }
        }
        if fatal_latched {
            tx_fatal_red_led.set_high();
        }
    }
}

async fn play_startup(buzzer: &mut BuzzerDriver<'static>) {
    for pitch_hz in [1_047, 1_319, 1_568] {
        play_tone(buzzer, pitch_hz, Duration::from_millis(70)).await;
        Timer::after_millis(35).await;
    }
}

async fn play_tone(buzzer: &mut BuzzerDriver<'static>, pitch_hz: u32, duration: Duration) {
    if let Err(value) = buzzer
        .play_tone(PitchHz(pitch_hz), duration, BEEP_VOLUME)
        .await
    {
        warn!("Integration004 indication beep failed: {:?}", value);
    }
}
