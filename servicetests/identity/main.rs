#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use raylar_versioning_service::{
    IdentityConfig, IdentityField, IdentityResources, IdentityVersioningService,
};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 8 * 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();

static IDENTITY: IdentityResources = IdentityResources::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }
    let _p = embassy_stm32::init(Default::default());

    let service = IdentityVersioningService::new(&IDENTITY, IdentityConfig::default());
    spawner.spawn(unwrap!(identity_task(service)));

    let mut state_rx = unwrap!(IDENTITY.state_receiver());
    let state = state_rx.changed().await;

    match state.device.stm32_uid_96 {
        IdentityField::Known(uid) => {
            info!("UID {:08X}-{:08X}-{:08X}", uid.word0, uid.word1, uid.word2);
        }
        _ => panic!("STM32 UID was not populated"),
    }
    assert!(matches!(state.device.serial_64, IdentityField::Known(_)));
    assert!(matches!(state.device.serial_32, IdentityField::Known(_)));
    assert!(matches!(
        state.device.stm32_device_code,
        IdentityField::Known(_)
    ));

    match state.firmware.runtime_crc32 {
        IdentityField::Known(crc) => info!("Runtime firmware CRC32 {:08X}", crc),
        IdentityField::Unavailable => info!("Runtime firmware CRC32 unavailable"),
        IdentityField::Unknown => info!("Runtime firmware CRC32 unknown"),
    }
    assert!(matches!(
        state.hardware.board_revision,
        IdentityField::Known(_) | IdentityField::Unavailable
    ));
    assert!(matches!(state.hardware.sd_card, IdentityField::Unknown));
    assert!(matches!(state.hardware.gps_module, IdentityField::Unknown));
    assert!(matches!(
        state.hardware.radio_module,
        IdentityField::Unknown
    ));
    info!(
        "Optional module identities: sd_known={} gps_known={} radio_known={}",
        state.hardware.sd_card.is_known(),
        state.hardware.gps_module.is_known(),
        state.hardware.radio_module.is_known()
    );
    info!("Identity and versioning service test passed");

    loop {
        Timer::after_secs(5).await;
    }
}

#[embassy_executor::task]
async fn identity_task(service: IdentityVersioningService) -> ! {
    service.run().await
}
