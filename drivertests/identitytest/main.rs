#![no_std]
#![no_main]

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_time::Timer;
use embedded_alloc::LlffHeap as Heap;
use raylar_drivers::identity::{self, IdentityError};
use {defmt_rtt as _, panic_probe as _};

const HEAP_BYTES: usize = 8 * 1024;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    unsafe {
        embedded_alloc::init!(HEAP, HEAP_BYTES);
    }

    let _p = embassy_stm32::init(Default::default());

    info!("Identity driver test started");

    let identity = identity::init();
    let uid = identity.uid();
    let uid_again = identity::read_device_uid();
    assert!(uid == uid_again);

    let serials = identity.serials();
    assert!(serials == identity::device_serials());
    assert!(serials.serial_48 == (serials.serial_64 & 0x0000_FFFF_FFFF_FFFF));
    assert!(serials.serial_16 == serials.serial_32 as u16);

    info!(
        "UID words: word0={} word1={} word2={}",
        uid.word0, uid.word1, uid.word2
    );
    info!(
        "Serials: serial64={} serial48={} serial32={} serial16={}",
        serials.serial_64, serials.serial_48, serials.serial_32, serials.serial_16
    );

    match identity::calculate_firmware_crc32() {
        Ok(crc) => info!("Runtime firmware CRC32={}", crc),
        Err(IdentityError::FirmwareRangeUnavailable) => {
            info!("Runtime firmware CRC32 unavailable")
        }
        Err(err) => error!("Runtime firmware CRC32 failed: {}", err),
    }

    let firmware = identity.firmware_identity();
    log_optional_u32("Build CRC32", firmware.build_crc32);
    log_optional_str("Git hash", firmware.git_hash);
    log_optional_str("Build timestamp", firmware.build_timestamp);
    log_optional_str("Firmware version", firmware.version);

    loop {
        Timer::after_secs(5).await;
    }
}

fn log_optional_u32(label: &str, value: Option<u32>) {
    if let Some(value) = value {
        info!("{}={}", label, value);
    } else {
        info!("{} unavailable", label);
    }
}

fn log_optional_str(label: &str, value: Option<&str>) {
    if let Some(value) = value {
        info!("{}={}", label, value);
    } else {
        info!("{} unavailable", label);
    }
}
