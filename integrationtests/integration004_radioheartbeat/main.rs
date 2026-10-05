#![no_std]
#![no_main]

#[cfg(not(all(feature = "lora", not(feature = "gfsk"))))]
compile_error!("Integration Test 004 requires exactly the `lora` modulation feature");

#[cfg(any(
    not(any(
        feature = "channel-868",
        feature = "channel-915",
        feature = "channel-2445"
    )),
    all(feature = "channel-868", feature = "channel-915"),
    all(feature = "channel-868", feature = "channel-2445"),
    all(feature = "channel-915", feature = "channel-2445")
))]
compile_error!(
    "select exactly one channel feature: `channel-868`, `channel-915`, or `channel-2445`"
);

extern crate alloc;

#[path = "../../servicetests/storage/common.rs"]
#[allow(dead_code)]
mod common;
mod diagnostics;
mod indication;
mod runtime;

use defmt::{error, info, unwrap};
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_stm32::gpio::Output;
use embassy_time::{Duration, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Leds};
use raylar_drivers::button::{self, ButtonName, ButtonResources};
use raylar_drivers::buzzer;
use raylar_drivers::identity;
use raylar_drivers::trng::{stm32::Stm32Trng, TrngConfig};
use raylar_location_service::{LocationConfig, LocationResources, LocationService, LocationState};
use raylar_radio_service::{BootId, NodeId};
use raylar_time_service::TimeState;
use {defmt_rtt as _, panic_probe as _};

use integration004_radioheartbeat::config;
use integration004_radioheartbeat::policy::RoleLatch;

const HEAP_BYTES: usize = 64 * 1024;
const LOCATION_HISTORY: usize = 9;
const TEST_NAME: &str = "Integration Test 004: radio heartbeat and rendezvous";
const FIRMWARE_VERSION: &str = env!("CARGO_PKG_VERSION");

#[global_allocator]
static HEAP: Heap = Heap::empty();

pub(crate) static LOCATION: LocationResources<4> = LocationResources::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }

    let peripherals = embassy_stm32::init(common::mcu_config());
    let Board {
        leds,
        buttons,
        gps,
        sd,
        ebyte_rf,
        trng,
        buzzer: board_buzzer,
        ..
    } = Board::new(peripherals);
    let Leds {
        sys_gps_red,
        sys_gps_green,
        sys_main_red,
        sys_main_green,
        sys_sd_blue,
        ..
    } = leds;

    let button = button::init(ButtonResources { user: buttons.user });
    Timer::after(Duration::from_millis(50)).await;
    let mut role_latch = RoleLatch::new();
    let role = role_latch.sample(button.is_pressed(ButtonName::User));
    drop(button);

    spawner.spawn(unwrap!(indication::role_led_task(sys_gps_red, role)));
    let buzzer_driver = buzzer::init(buzzer::BuzzerResources {
        timer: board_buzzer.tim,
        pin: board_buzzer.pin,
    });
    spawner.spawn(unwrap!(indication::activity_task(
        sys_sd_blue,
        sys_main_green,
        sys_main_red,
        buzzer_driver,
    )));
    indication::startup();

    if config::validate().is_err() {
        error!("Integration004 invalid configuration");
        fatal_forever("invalid integration configuration").await;
    }
    let profile = match config::profile() {
        Ok(profile) => profile,
        Err(value) => {
            error!("Integration004 invalid radio profile: {:?}", value);
            fatal_forever("invalid radio profile").await
        }
    };

    let identity = identity::init();
    let node_id = NodeId::from(identity.serials());
    let firmware = identity.firmware_identity();
    let mut trng = Stm32Trng::new(trng, raylar_board_v1p0::Irqs, TrngConfig::default());
    let boot_id = match trng.boot_id().await {
        Ok(value) => BootId(value),
        Err(value) => {
            error!("Integration004 TRNG boot ID failed: {:?}", value);
            fatal_forever("TRNG boot ID failed").await
        }
    };
    diagnostics::initialize_identity(node_id, boot_id);

    common::start_time(spawner, gps).await;
    let location = LocationService::<4, LOCATION_HISTORY>::new(
        &LOCATION,
        unwrap!(common::GPS_RESOURCES.fix_receiver()).as_dyn(),
        LocationConfig::default(),
    );
    spawner.spawn(unwrap!(location_service_task(location)));
    spawner.spawn(unwrap!(pps_led_task(sys_gps_green)));
    spawner.spawn(unwrap!(system_ui_task()));
    spawner.spawn(unwrap!(diagnostics::logging_task(sd)));

    let driver = runtime::make_radio(ebyte_rf);
    let service =
        raylar_radio_service::RadioService::new(driver, &runtime::RADIO, config::PREPARATION_GUARD);
    spawner.spawn(unwrap!(runtime::radio_service_task(service)));

    diagnostics::emit(diagnostics::DiagnosticKind::Boot {
        test_name: TEST_NAME,
        firmware_version: FIRMWARE_VERSION,
        firmware_hash: firmware.git_hash,
        role,
        network_id: config::NETWORK_ID,
        schedule_version: config::SCHEDULE_VERSION.0,
        configuration_id: config::CONFIGURATION_ID,
        frequency_hz: config::FREQUENCY_HZ,
        tx_power_dbm: config::TX_POWER_DBM,
    });
    info!(
        "{} started firmware_version={} firmware_hash={} node={:#010x} boot={:#010x} base={} network={:#010x} schedule={} profile={} frequency_hz={} tx_power_dbm={}",
        TEST_NAME,
        FIRMWARE_VERSION,
        firmware.git_hash.unwrap_or("unknown"),
        node_id.0,
        boot_id.0,
        role.is_base_station(),
        config::NETWORK_ID,
        config::SCHEDULE_VERSION.0,
        config::CONFIGURATION_ID,
        config::FREQUENCY_HZ,
        config::TX_POWER_DBM,
    );

    runtime::run(role, node_id, boot_id, profile).await
}

#[embassy_executor::task]
async fn location_service_task(service: LocationService<4, LOCATION_HISTORY>) -> ! {
    service.run().await
}

#[embassy_executor::task]
async fn system_ui_task() -> ! {
    let mut locations = unwrap!(LOCATION.state_receiver());
    let mut times = unwrap!(common::TIME_RESOURCES.state_receiver());
    let mut last_location = LOCATION.state();
    let mut gps_locked = last_location.valid;
    let mut last_time = common::TIME_RESOURCES.time_state();
    let mut utc_calibrated = last_time.frequency_calibration_locked;
    log_location_status(last_location);
    log_time_calibration(last_time);
    if gps_locked {
        indication::gps_lock();
    }
    if utc_calibrated {
        indication::utc_calibrated();
    }

    loop {
        match select(locations.changed(), times.changed()).await {
            Either::First(location) => {
                if location.valid && !gps_locked {
                    indication::gps_lock();
                }
                let periodic_fix = location.total_fix_count_seen
                    != last_location.total_fix_count_seen
                    && (location.total_fix_count_seen == 1
                        || location.total_fix_count_seen.is_multiple_of(60));
                if location.valid != last_location.valid || periodic_fix {
                    log_location_status(location);
                }
                gps_locked = location.valid;
                last_location = location;
            }
            Either::Second(time) => {
                if time.frequency_calibration_locked && !utc_calibrated {
                    indication::utc_calibrated();
                }
                let accepted_minute = time.accepted_anchors / 60 != last_time.accepted_anchors / 60;
                if time.utc_status != last_time.utc_status
                    || time.active_time_source != last_time.active_time_source
                    || time.frequency_calibration_samples != last_time.frequency_calibration_samples
                    || time.frequency_calibration_locked != last_time.frequency_calibration_locked
                    || time.rejected_anchors != last_time.rejected_anchors
                    || accepted_minute
                {
                    log_time_calibration(time);
                }
                utc_calibrated = time.frequency_calibration_locked;
                last_time = time;
            }
        }
    }
}

fn log_location_status(location: LocationState) {
    diagnostics::emit(diagnostics::DiagnosticKind::LocationStatus {
        valid: location.valid,
        latitude_e7: location.latitude.degrees_e7,
        longitude_e7: location.longitude.degrees_e7,
        fixes_seen: location.total_fix_count_seen,
        fixes_used: location.fix_count_used,
        satellites: location.satellites,
        hdop_centi: location.hdop_centi,
        uncertainty_meters: location.uncertainty_meters,
    });
    info!(
        "Integration004 GPS valid={} fixes={} satellites={:?} hdop_centi={:?} uncertainty_m={:?}",
        location.valid,
        location.total_fix_count_seen,
        location.satellites,
        location.hdop_centi,
        location.uncertainty_meters,
    );
}

fn log_time_calibration(time: TimeState) {
    diagnostics::emit(diagnostics::DiagnosticKind::TimeCalibration {
        status: time.utc_status,
        source: time.active_time_source,
        uncertainty_us: time.uncertainty_us,
        accepted_anchors: time.accepted_anchors,
        rejected_anchors: time.rejected_anchors,
        calibration_samples: time.frequency_calibration_samples,
        calibration_locked: time.frequency_calibration_locked,
        calibrated_error_ppb: time.calibrated_frequency_error_ppb,
    });
    info!(
        "Integration004 UTC status={:?} source={:?} uncertainty_us={} anchors={}/{} calibration={}/{} error_ppb={}",
        time.utc_status,
        time.active_time_source,
        time.uncertainty_us,
        time.accepted_anchors,
        time.rejected_anchors,
        time.frequency_calibration_samples,
        time.frequency_calibration_locked,
        time.calibrated_frequency_error_ppb,
    );
}

#[embassy_executor::task]
async fn pps_led_task(mut gps_green: Output<'static>) -> ! {
    let mut pps = unwrap!(common::GPS_RESOURCES.pps_receiver());
    gps_green.set_low();
    loop {
        let _ = pps.changed().await;
        gps_green.set_high();
        Timer::after_millis(50).await;
        gps_green.set_low();
    }
}

async fn fatal_forever(message: &str) -> ! {
    error!("Integration004 fatal: {}", message);
    indication::fatal();
    loop {
        Timer::after_secs(60).await;
    }
}
