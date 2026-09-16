#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_sync::watch::Watch;
use embassy_time::{Instant, Timer};
use embedded_alloc::LlffHeap as Heap;
use raylar_drivers::gps::{Coordinate, GpsFix, GpsMutex, UtcDateTime, UtcTime};
use raylar_location_service::{LocationConfig, LocationResources, LocationService};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 8 * 1024;
const HISTORY: usize = 5;

#[global_allocator]
static HEAP: Heap = Heap::empty();

static LOCATION: LocationResources = LocationResources::new();
static GPS_FIXES: Watch<GpsMutex, GpsFix, 4> = Watch::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }
    let _p = embassy_stm32::init(Default::default());

    let service = LocationService::<4, HISTORY>::new(
        &LOCATION,
        unwrap!(GPS_FIXES.receiver()).as_dyn(),
        LocationConfig::default(),
    );
    spawner.spawn(unwrap!(location_task(service)));

    let fixes = GPS_FIXES.sender();
    let mut states = unwrap!(LOCATION.state_receiver());

    info!("Location service simulated test started");
    fixes.send(fix(520_000_010, -10_000_010, 8, Some(120), 1));
    Timer::after_millis(20).await;
    fixes.send(fix(520_000_020, -10_000_020, 8, Some(130), 2));
    Timer::after_millis(20).await;
    fixes.send(fix(800_000_000, 900_000_000, 8, Some(100), 3));
    Timer::after_millis(20).await;
    fixes.send(fix(520_000_000, -10_000_000, 8, Some(110), 4));

    let state = loop {
        let state = states.changed().await;
        info!(
            "Location state valid={} lat_e7={} lon_e7={} used={} seen={} uncertainty_m={:?}",
            state.valid,
            state.latitude.degrees_e7,
            state.longitude.degrees_e7,
            state.fix_count_used,
            state.total_fix_count_seen,
            state.uncertainty_meters
        );
        if state.valid {
            break state;
        }
    };

    assert!(state.valid);
    assert!(state.latitude.degrees_e7 < 521_000_000);
    assert!(state.longitude.degrees_e7 < 0);
    info!("Location service simulated test passed");

    loop {
        Timer::after_secs(5).await;
    }
}

#[embassy_executor::task]
async fn location_task(service: LocationService<4, HISTORY>) -> ! {
    service.run().await
}

fn fix(latitude: i32, longitude: i32, satellites: u8, hdop: Option<u16>, ticks: u64) -> GpsFix {
    GpsFix {
        latitude: Coordinate {
            degrees_e7: latitude,
        },
        longitude: Coordinate {
            degrees_e7: longitude,
        },
        utc_time: UtcDateTime {
            date: None,
            time: UtcTime {
                hour: 12,
                minute: 0,
                second: ticks as u8,
            },
        },
        satellites,
        hdop_centi: hdop,
        system_timestamp: Instant::from_ticks(ticks),
    }
}
