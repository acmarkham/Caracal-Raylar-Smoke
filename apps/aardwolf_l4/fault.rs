//! First-fault latch. Storage and audio stop; UI remains runnable until reset.
use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
pub static CODE: AtomicU32 = AtomicU32::new(0);
#[derive(Clone, Copy, defmt::Format)]
#[repr(u32)]
pub enum Fault {
    SdRemoved = 1,
    Storage = 2,
    Audio = 3,
    Panic = 4,
}
pub fn active() -> bool {
    CODE.load(Relaxed) != 0
}
pub fn latch(fault: Fault) {
    if CODE
        .compare_exchange(0, fault as u32, Relaxed, Relaxed)
        .is_ok()
    {
        crate::audio::STOP.store(true, Relaxed);
        crate::ARMED.store(false, Relaxed);
        crate::sd::WRITE_ALLOWED.store(false, Relaxed);
        defmt::error!(
            "AARDWOLF L4 FAULT code={} reason={} latched_until_reset=true",
            fault as u32,
            fault
        );
    }
}
pub async fn park(fault: Fault) -> ! {
    latch(fault);
    core::future::pending().await
}

// Backstop for assertions/panics, including storage initialization failures.
// No executor, timer interrupt, allocation or filesystem is needed here.
#[panic_handler]
fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
    defmt::error!("AARDWOLF L4 panic {}", defmt::Display2Format(info));
    latch(Fault::Panic);
    emergency()
}

#[defmt::panic_handler]
fn defmt_panic() -> ! {
    latch(Fault::Panic);
    emergency()
}

pub fn emergency() -> ! {
    cortex_m::interrupt::disable();
    // Terminal ownership transfer: halt microphone/SD DMA before repurposing
    // the buzzer pin as GPIO. No more filesystem I/O or audio buffer access.
    unsafe {
        let modify = |address: usize, clear: u32, set: u32| {
            let p = address as *mut u32;
            p.write_volatile((p.read_volatile() & !clear) | set);
        };
        modify(0x4002_0044, 1, 0); // DMA1 channel 4 EN
        modify(0x4002_0458, 1, 0); // DMA2 channel 5 EN
        modify(0x4002_104c, 0, 1 << 1); // RCC AHB2ENR GPIOBEN
        modify(0x4001_3444, 1 << 15, 0); // TIM8 MOE off
        modify(0x4800_0400, (3 << 22) | (3 << 28), (1 << 22) | (1 << 28));
        modify(0xe000_edfc, 0, 1 << 24); // DEMCR TRCENA
        modify(0xe000_1000, 0, 1); // DWT CYCCNTENA
        let bsrr = 0x4800_0418 as *mut u32;
        // Switch to the internal 16 MHz oscillator, including for clock-init
        // failures. Flash latency may safely remain at its higher-clock value.
        modify(0x4002_1000, 0, 1 << 8); // HSION
        while (0x4002_1000 as *const u32).read_volatile() & (1 << 10) == 0 {
            core::hint::spin_loop();
        }
        modify(0x4002_1008, 3, 1); // SYSCLK = HSI16 (AHB stays DIV1)
        while (0x4002_1008 as *const u32).read_volatile() & 12 != 4 {
            core::hint::spin_loop();
        }
        let cycles_per_us = 16;
        let cycle = || (0xe000_1004 as *const u32).read_volatile();
        let mut elapsed_us = 0u64;
        loop {
            let hz = aardwolf_l4::ui_policy::error_hz(elapsed_us / 1000);
            let half_us = if hz == 0 {
                1000
            } else {
                500_000 / u64::from(hz)
            };
            let red = aardwolf_l4::ui_policy::red_on(elapsed_us / 1000);
            bsrr.write_volatile(if red { 1 << 27 } else { 1 << 11 });
            bsrr.write_volatile(if hz != 0 && (elapsed_us / half_us) % 2 == 0 {
                1 << 14
            } else {
                1 << 30
            });
            let start = cycle();
            while cycle().wrapping_sub(start) < (half_us * cycles_per_us) as u32 {
                core::hint::spin_loop();
            }
            elapsed_us += half_us;
        }
    }
}

// A failed/overflowed thread stack must not be used to signal the fault. The
// bottom 1 KiB SRAM2 guard is disposable only after this terminal transition.
#[unsafe(naked)]
#[unsafe(no_mangle)]
unsafe extern "C" fn HardFault() -> ! {
    core::arch::naked_asm!("cpsid i", "ldr r0, =0x10000400", "msr msp, r0", "b {handler}", handler = sym hard_fault);
}
extern "C" fn hard_fault() -> ! {
    // Avoid defmt/formatting on the emergency stack.
    CODE.store(Fault::Panic as u32, Relaxed);
    emergency()
}
