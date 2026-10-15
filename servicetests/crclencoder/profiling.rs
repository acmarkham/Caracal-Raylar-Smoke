use defmt::{info, unwrap};
use raylar_drivers::stm32_core::{stm32::Stm32CoreDriver, CoreConfig, CoreSupply};
use raylar_stage_profiling::{self as profile, Span, Stage, STAGES};

// common::mcu_config: HSE=16 MHz, PLL1 N=18, R=2, AHB divider=1.
const CORE_HZ: u64 = 144_000_000;

pub fn init() {
    let _core = unwrap!(Stm32CoreDriver::init(CoreConfig {
        supply: CoreSupply::Smps
    }));
    // Embassy initialized the core already; only borrow the debug facilities here.
    let mut core = unsafe { cortex_m::Peripherals::steal() };
    assert!(cortex_m::peripheral::DWT::has_cycle_counter());
    core.DCB.enable_trace();
    core.DWT.enable_cycle_counter();
    let before = cortex_m::peripheral::DWT::cycle_count();
    cortex_m::asm::delay(1024);
    assert_ne!(before, cortex_m::peripheral::DWT::cycle_count());
    profile::set_running(true);
    let started = cortex_m::peripheral::DWT::cycle_count();
    for _ in 0..4096 {
        drop(Span::new(Stage::Probe));
    }
    let outer = cortex_m::peripheral::DWT::cycle_count().wrapping_sub(started);
    profile::set_running(false);
    let empty = profile::snapshot(Stage::Probe);
    info!("PROFILE config core_hz={} supply=smps gps=off sample_stride=32 inclusive=true empty_span_cycles={} probe_overhead_cycles={} codec={}", CORE_HZ, empty.cycles / 4096, outer / 4096,
        if cfg!(feature = "cheap-crcl") { "temporal_shift_v2" } else { "spatial_v1" });
    profile::reset();
}

pub fn begin(detailed: bool) {
    profile::set_running(false);
    profile::reset();
    profile::set_detailed(detailed);
    profile::set_running(true);
}

pub fn report(window: u32, frames: u64, elapsed_us: u64) {
    profile::set_running(false);
    if cfg!(feature = "cheap-crcl") {
        assert_eq!(profile::snapshot(Stage::CrclCrc).calls, 0);
        assert_eq!(profile::snapshot(Stage::ReferenceShaCompress).calls, 0);
    }
    let budget = CORE_HZ * elapsed_us / 1_000_000;
    let recorder = profile::snapshot(Stage::RecorderPoll).cycles;
    let capture = profile::snapshot(Stage::CaptureCheck).cycles
        + profile::snapshot(Stage::CapturePublish).cycles;
    info!("PROFILE window={} frames={} elapsed_us={} cycle_budget={} recorder_cycles={} capture_cycles={} accounted_cpu_permille={}", window, frames, elapsed_us, budget, recorder, capture, (recorder + capture) * 1000 / budget.max(1));
    for stage in STAGES {
        let s = profile::snapshot(stage);
        info!(
            "PROFILE stage={} calls={} cycles={} max_cycles={} wall_us={} bytes={} sampled={}",
            stage.name(),
            s.calls,
            s.cycles,
            s.max_cycles,
            s.wall_us,
            s.bytes,
            stage.sampled()
        );
    }
    let representations = profile::representations();
    info!(
        "PROFILE representation zero={} direct={} temporal={} packed={}",
        representations[0], representations[1], representations[2], representations[3]
    );
}
