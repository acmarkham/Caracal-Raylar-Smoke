use embassy_time::Instant;

#[derive(Debug, PartialEq, Eq)]
pub struct ReceivedPacket<'a> {
    pub payload: &'a [u8],
    pub metadata: RxMetadata,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RxMetadata {
    /// Local monotonic time captured immediately after the RX-done IRQ edge.
    pub packet_complete_at: Instant,
    pub frequency_hz: u32,
    pub metrics: RxMetrics,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxMetrics {
    LoRa {
        /// Packet RSSI in half-dBm units. Divide by two for dBm.
        rssi_dbm_x2: i16,
        /// Despread signal RSSI in half-dBm units.
        signal_rssi_dbm_x2: i16,
        /// Signed SNR in quarter-dB units. Divide by four for dB.
        snr_db_x4: i16,
    },
    Gfsk {
        /// Average packet RSSI in half-dBm units.
        rssi_dbm_x2: i16,
        status: GfskPacketStatus,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GfskPacketStatus {
    pub sync_rssi_dbm_x2: i16,
    pub length_error: bool,
    pub crc_error: bool,
    pub abort_error: bool,
    pub address_error: bool,
    pub sync_error: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TxReport {
    pub requested_start: Instant,
    /// Time immediately before issuing SetTx.
    pub command_started_at: Instant,
    /// Time immediately after SetTx completed.
    pub command_completed_at: Instant,
    /// Local monotonic time captured immediately after the TX-done IRQ edge.
    pub tx_done_at: Instant,
}
