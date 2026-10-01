use raylar_time_service::UtcTimestamp;

use crate::radio_test_config::{CONFIGURATION_ID, MAX_LOCAL_OFFSET_METRES, PROTOCOL_VERSION};

pub const PACKET_LEN: usize = 29;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RangePacket {
    pub sender_id: u64,
    pub sequence: u16,
    pub tx_utc: UtcTimestamp,
    pub east_10m: i16,
    pub north_10m: i16,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecodeError {
    Length,
    Version,
    Configuration,
    Timestamp,
    Position,
}

impl RangePacket {
    pub fn encode(self) -> [u8; PACKET_LEN] {
        let mut output = [0u8; PACKET_LEN];
        output[0] = PROTOCOL_VERSION;
        output[1..3].copy_from_slice(&CONFIGURATION_ID.to_be_bytes());
        output[3..11].copy_from_slice(&self.sender_id.to_be_bytes());
        output[11..13].copy_from_slice(&self.sequence.to_be_bytes());
        output[13..21].copy_from_slice(&self.tx_utc.seconds.to_be_bytes());
        output[21..25].copy_from_slice(&self.tx_utc.microseconds.to_be_bytes());
        output[25..27].copy_from_slice(&self.east_10m.to_be_bytes());
        output[27..29].copy_from_slice(&self.north_10m.to_be_bytes());
        output
    }

    pub fn decode(input: &[u8]) -> Result<Self, DecodeError> {
        if input.len() != PACKET_LEN {
            return Err(DecodeError::Length);
        }
        if input[0] != PROTOCOL_VERSION {
            return Err(DecodeError::Version);
        }
        if u16::from_be_bytes([input[1], input[2]]) != CONFIGURATION_ID {
            return Err(DecodeError::Configuration);
        }
        let seconds = i64::from_be_bytes(input[13..21].try_into().unwrap());
        let microseconds = u32::from_be_bytes(input[21..25].try_into().unwrap());
        if seconds < 0 {
            return Err(DecodeError::Timestamp);
        }
        let tx_utc = UtcTimestamp::new(seconds, microseconds).ok_or(DecodeError::Timestamp)?;
        let east_10m = i16::from_be_bytes([input[25], input[26]]);
        let north_10m = i16::from_be_bytes([input[27], input[28]]);
        let maximum_units = MAX_LOCAL_OFFSET_METRES / 10;
        if i64::from(east_10m).abs() > maximum_units || i64::from(north_10m).abs() > maximum_units {
            return Err(DecodeError::Position);
        }
        Ok(Self {
            sender_id: u64::from_be_bytes(input[3..11].try_into().unwrap()),
            sequence: u16::from_be_bytes([input[11], input[12]]),
            tx_utc,
            east_10m,
            north_10m,
        })
    }
}
