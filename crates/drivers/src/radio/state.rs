#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RadioState {
    Sleep,
    #[default]
    Standby,
    Rx,
    Tx,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RadioStats {
    pub state: RadioState,
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub crc_errors: u32,
    pub header_errors: u32,
    pub gfsk_length_errors: u32,
    pub gfsk_address_errors: u32,
    pub rx_timeouts: u32,
    pub tx_timeouts: u32,
    pub command_errors: u32,
    pub transport_errors: u32,
    pub deadline_misses: u32,
    pub resets: u32,
}

impl RadioStats {
    pub(crate) fn set_state(&mut self, state: RadioState) {
        self.state = state;
    }

    pub(crate) fn increment(value: &mut u32) {
        *value = value.saturating_add(1);
    }
}
