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
use embassy_time::{Duration, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_board_v1p0::{Board, Leds};
use raylar_drivers::button::{self, ButtonName, ButtonResources};
use raylar_drivers::identity;
use raylar_drivers::trng::{stm32::Stm32Trng, TrngConfig};
use raylar_location_service::{LocationConfig, LocationResources, LocationService};
use raylar_radio_service::{BootId, NodeId};
use {defmt_rtt as _, panic_probe as _};

use integration004_radioheartbeat::config;
use integration004_radioheartbeat::policy::RoleLatch;

const HEAP_BYTES: usize = 64 * 1024;
const LOCATION_HISTORY: usize = 9;

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
        ..
    } = Board::new(peripherals);
    let Leds {
        sys_gps_red,
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
    spawner.spawn(unwrap!(indication::activity_led_task(
        sys_sd_blue,
        sys_main_green,
        sys_main_red,
    )));

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

    let node_id = NodeId::from(identity::init().serials());
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
    spawner.spawn(unwrap!(diagnostics::logging_task(sd)));

    let driver = runtime::make_radio(ebyte_rf);
    let service =
        raylar_radio_service::RadioService::new(driver, &runtime::RADIO, config::PREPARATION_GUARD);
    spawner.spawn(unwrap!(runtime::radio_service_task(service)));

    diagnostics::emit(diagnostics::DiagnosticKind::Boot {
        role,
        network_id: config::NETWORK_ID,
        schedule_version: config::SCHEDULE_VERSION.0,
        configuration_id: config::CONFIGURATION_ID,
        frequency_hz: config::FREQUENCY_HZ,
        tx_power_dbm: config::TX_POWER_DBM,
    });
    info!(
        "Integration004 started node={:#010x} boot={:#010x} base={} network={:#010x} schedule={} profile={} frequency_hz={} tx_power_dbm={}",
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

async fn fatal_forever(message: &str) -> ! {
    error!("Integration004 fatal: {}", message);
    indication::fatal();
    loop {
        Timer::after_secs(60).await;
    }
}
