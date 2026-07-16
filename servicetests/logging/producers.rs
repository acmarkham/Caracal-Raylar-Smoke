use embassy_time::Timer;
use raylar_logging_service::{
    debug as log_debug, error as log_error, info as log_info, trace as log_trace, warn as log_warn,
};

use crate::TestLogger;

#[embassy_executor::task]
pub async fn gps(log: TestLogger) -> ! {
    let _ = log_info!(log, "GPS producer started");
    let mut fixes = 0u32;
    loop {
        Timer::after_secs(2).await;
        fixes = fixes.wrapping_add(1);
        let satellites = 6 + fixes % 7;
        let _ = log_debug!(log, "search cycle {} sees {} satellites", fixes, satellites);
        if fixes.is_multiple_of(3) {
            let _ = log_info!(log, "fix {} acquired with {} satellites", fixes, satellites);
        }
    }
}

#[embassy_executor::task]
pub async fn audio(log: TestLogger) -> ! {
    let _ = log_info!(log, "Audio producer started at {} Hz", 16_000u32);
    let mut blocks = 0u32;
    loop {
        Timer::after_secs(1).await;
        blocks = blocks.wrapping_add(1);
        let _ = log_trace!(log, "DMA block {} ready ({} samples)", blocks, 1_024u16);
        if blocks.is_multiple_of(8) {
            let _ = log_debug!(log, "recording has written {} blocks", blocks);
        }
    }
}

#[embassy_executor::task]
pub async fn battery(log: TestLogger) -> ! {
    let _ = log_info!(log, "Battery producer started");
    let mut samples = 0u32;
    loop {
        Timer::after_secs(3).await;
        samples = samples.wrapping_add(1);
        match samples % 6 {
            4 => {
                let _ = log_warn!(log, "battery voltage low: {} mV", 3_550u16);
            }
            5 => {
                let _ = log_error!(log, "battery sample {} failed: ADC timeout", samples);
            }
            _ => {
                let millivolts = 4_100u32.saturating_sub(samples % 20 * 10);
                let _ = log_info!(log, "battery voltage = {} mV", millivolts);
            }
        }
    }
}
