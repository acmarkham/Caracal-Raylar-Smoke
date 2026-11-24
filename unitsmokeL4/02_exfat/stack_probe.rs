//! Conservative high-water observation of this smoke image's MSP stack.
use core::sync::atomic::{AtomicUsize, Ordering};

const WORD: u32 = 0xa55a_3cc3;
static TOP: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" {
    static _stack_end: u32;
    static _stack_start: u32;
}

fn floor() -> usize {
    core::ptr::addr_of!(_stack_end) as usize
}

pub fn initialize() {
    let top = (cortex_m::register::msp::read() as usize - 256) & !3;
    defmt::assert_eq!(floor(), 0x1000_0000);
    defmt::assert!(top < 0x1000_8000);
    defmt::assert!(top > floor() + 1024);
    // Only the unused portion of the dedicated SRAM2 stack bank is painted;
    // current frames are excluded. SRAM1 statics/DMA buffers are untouched.
    for address in (floor()..top).step_by(4) {
        unsafe { core::ptr::write_volatile(address as *mut u32, WORD) };
    }
    for address in (floor()..top).step_by(4) {
        defmt::assert_eq!(
            unsafe { core::ptr::read_volatile(address as *const u32) },
            WORD
        );
    }
    TOP.store(top, Ordering::Relaxed);
}

pub fn guard_intact() -> bool {
    (floor()..floor() + 1024)
        .step_by(4)
        .all(|address| unsafe { core::ptr::read_volatile(address as *const u32) == WORD })
}

pub fn check(phase: &str) {
    defmt::assert!(guard_intact(), "L4 stack red zone was overwritten");
    let mut first_used = floor();
    let top = TOP.load(Ordering::Relaxed);
    while first_used < top && unsafe { core::ptr::read_volatile(first_used as *const u32) == WORD }
    {
        first_used += 4;
    }
    let start = core::ptr::addr_of!(_stack_start) as usize;
    defmt::info!(
        "L4 stack phase={} headroom={} high_water_upper_bound={} unused={} guard=ok",
        phase,
        start - floor(),
        start - first_used,
        first_used - floor()
    );
}
