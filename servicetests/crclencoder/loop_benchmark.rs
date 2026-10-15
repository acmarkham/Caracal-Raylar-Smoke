//! Synchronous CRCL-only cycle bench. No executor, capture, GPS, SD or radio tasks.
#![no_std]
#![no_main]

#[path = "../storage/common.rs"]
#[allow(dead_code)]
mod common;

use cortex_m::peripheral::DWT;
use defmt::{info, unwrap};
use raylar_crcl_encoder_service::benchmark::{self, Memory};
use raylar_drivers::stm32_core::{stm32::Stm32CoreDriver, CoreConfig, CoreSupply};
use static_cell::ConstStaticCell;
use {defmt_rtt as _, panic_probe as _};

#[global_allocator]
static HEAP: embedded_alloc::LlffHeap = embedded_alloc::LlffHeap::empty();
static MEMORY: ConstStaticCell<Memory> = ConstStaticCell::new(Memory::new());

#[cortex_m_rt::entry]
fn main() -> ! {
    let _peripherals = embassy_stm32::init(common::mcu_config());
    let _core = unwrap!(Stm32CoreDriver::init(CoreConfig {
        supply: CoreSupply::Smps
    }));
    let mut core = unwrap!(cortex_m::Peripherals::take());
    assert!(DWT::has_cycle_counter());
    core.DCB.enable_trace();
    core.DWT.enable_cycle_counter();
    // No timers or async tasks are needed. Eliminate IRQ jitter throughout run.
    cortex_m::interrupt::disable();
    let before = DWT::cycle_count();
    cortex_m::asm::delay(1024);
    assert_ne!(before, DWT::cycle_count());
    info!("CRCL_LOOP_META core_hz=144000000 frames=1024 auxiliary_channels=5 bits=10 min_shift=6 repeats=129 warmups=8 interrupts=off supply=smps fixture=sine500_noise128_seed12345678 filter=optional_dcblock_shift8 writer=reservoir32_single");
    benchmark::run(
        MEMORY.take(),
        DWT::cycle_count,
        |row| {
            info!(
                "CRCL_LOOP_ROW stage={} batch={} min={} median={} p90={} max={} checksum={}",
                row.name, row.batch, row.min, row.median, row.p90, row.max, row.checksum
            );
        },
        |info| {
            info!("CRCL_LOOP_FIXTURE hash={} bytes={} shifts={:?} rice_k={:?} sums={:?} payload_bytes={:?}", info.input_hash, info.record_bytes, info.shifts, info.k, info.sums, info.payload_bytes);
        },
    );
    info!("CRCL LOOP BENCH PASS");
    loop {
        cortex_m::asm::wfi();
    }
}
