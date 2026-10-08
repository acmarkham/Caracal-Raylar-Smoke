//! Fixed-size deployment heartbeat. V1 remains separately decodable.
use crate::{FrameError, NodeId};

pub const V4_FRAME_LEN: usize = 16;
pub const STORAGE_FULL: u16 = 1;
pub const STORAGE_UNAVAILABLE: u16 = 2;
pub const ENERGY_RECOVERED: u16 = 4;
pub const AUDIO_FAULT: u16 = 8;
pub const LOGGING_IMPAIRED: u16 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HeartbeatV4 {
    pub node: NodeId,
    pub boot: u16,
    pub sequence: u16,
    pub soc: Option<u8>,
    pub charging: u8,
    pub errors: u16,
    pub storage_percent: Option<u8>,
    pub gps_status: u8,
}

impl HeartbeatV4 {
    pub fn encode(self) -> Result<[u8; V4_FRAME_LEN], FrameError> {
        let mut out = [0u8; V4_FRAME_LEN];
        out[0] = 0x41;
        out[2..6].copy_from_slice(&self.node.0.to_be_bytes());
        out[6..8].copy_from_slice(&self.boot.to_be_bytes());
        out[8..10].copy_from_slice(&self.sequence.to_be_bytes());
        out[10] = self.soc.unwrap_or(255);
        out[11] = self.charging;
        out[12..14].copy_from_slice(&self.errors.to_be_bytes());
        out[14] = self.storage_percent.unwrap_or(255);
        out[15] = self.gps_status;
        Self::decode(&out)?;
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, FrameError> {
        if bytes.len() < V4_FRAME_LEN {
            return Err(FrameError::Truncated);
        }
        if bytes.len() != V4_FRAME_LEN
            || bytes[0] != 0x41
            || bytes[1] != 0
            || (bytes[10] > 100 && bytes[10] != 255)
            || (bytes[14] > 100 && bytes[14] != 255)
            || bytes[11] > 4
            || bytes[12] != 0
            || bytes[13] & !31 != 0
            || bytes[15] & 0xc0 != 0
        {
            return Err(FrameError::Malformed);
        }
        Ok(Self {
            node: NodeId(u32::from_be_bytes(bytes[2..6].try_into().unwrap())),
            boot: u16::from_be_bytes(bytes[6..8].try_into().unwrap()),
            sequence: u16::from_be_bytes(bytes[8..10].try_into().unwrap()),
            soc: (bytes[10] != 255).then_some(bytes[10]),
            charging: bytes[11],
            errors: u16::from_be_bytes(bytes[12..14].try_into().unwrap()),
            storage_percent: (bytes[14] != 255).then_some(bytes[14]),
            gps_status: bytes[15],
        })
    }
}

/// Service envelope validation supports both versions without making V1
/// protocol consumers accidentally interpret the shorter V4 header.
pub(crate) fn frame_type(bytes: &[u8]) -> Result<crate::FrameType, FrameError> {
    if bytes.first().map(|b| b >> 4) == Some(4) {
        HeartbeatV4::decode(bytes).map(|_| crate::FrameType::Heartbeat)
    } else {
        crate::FrameHeader::decode(bytes).map(|f| f.header.frame_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wire_vector_and_rejection_boundaries() {
        let h = HeartbeatV4 {
            node: NodeId(0x12345678),
            boot: 0x9abc,
            sequence: 0xdef0,
            soc: Some(42),
            charging: 1,
            errors: 5,
            storage_percent: None,
            gps_status: 0x29,
        };
        let bytes = [
            0x41, 0, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 42, 1, 0, 5, 255, 0x29,
        ];
        assert_eq!(h.encode().unwrap(), bytes);
        assert_eq!(HeartbeatV4::decode(&bytes).unwrap(), h);
        assert!(crate::FrameHeader::decode(&bytes).is_err());
        for length in 0..16 {
            assert!(HeartbeatV4::decode(&bytes[..length]).is_err());
        }
        for (index, value) in [
            (1, 1),
            (10, 101),
            (11, 5),
            (12, 1),
            (13, 32),
            (14, 101),
            (15, 64),
        ] {
            let mut invalid = bytes;
            invalid[index] = value;
            assert!(HeartbeatV4::decode(&invalid).is_err());
        }
    }
}
