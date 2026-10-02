/// Bounded latest-state counters. Every counter saturates at `u32::MAX`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RadioServiceStats {
    pub frames_tx: u32,
    pub frames_rx: u32,
    pub heartbeat_tx: u32,
    pub presence_tx: u32,
    pub presence_rx: u32,
    pub malformed_frames: u32,
    pub unsupported_frames: u32,
    pub schedule_misses: u32,
    pub scheduler_conflicts: u32,
    pub queue_drops: u32,
    pub neighbour_count: u16,
    pub radio_errors: u32,
}

impl RadioServiceStats {
    pub(crate) fn increment(counter: &mut u32) {
        *counter = counter.saturating_add(1);
    }
}
