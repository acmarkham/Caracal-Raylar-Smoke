use crate::{BootId, FrameError, NodeId, Sequence};

pub const WIRE_VERSION: u8 = 1;
pub const FRAME_HEADER_LEN: usize = 12;
const DESTINATION_LEN: usize = 4;
const FLAG_DESTINATION: u8 = 1 << 0;
const KNOWN_FLAGS: u8 = FLAG_DESTINATION;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum FrameType {
    Heartbeat = 1,
    Presence = 2,
    Data = 3,
}

impl FrameType {
    fn from_wire(value: u8) -> Result<Self, FrameError> {
        match value {
            1 => Ok(Self::Heartbeat),
            2 => Ok(Self::Presence),
            3 => Ok(Self::Data),
            value => Err(FrameError::UnknownFrameType(value)),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct FrameHeader {
    pub frame_type: FrameType,
    pub source: NodeId,
    pub boot_id: BootId,
    pub sequence: Sequence,
    pub destination: Option<NodeId>,
}

impl FrameHeader {
    pub const fn encoded_len(&self) -> usize {
        FRAME_HEADER_LEN
            + if self.destination.is_some() {
                DESTINATION_LEN
            } else {
                0
            }
    }

    pub fn encode(&self, payload: &[u8], output: &mut [u8]) -> Result<usize, FrameError> {
        if matches!(self.frame_type, FrameType::Heartbeat | FrameType::Presence)
            && self.destination.is_some()
        {
            return Err(FrameError::InvalidDestination);
        }
        let length = self
            .encoded_len()
            .checked_add(payload.len())
            .ok_or(FrameError::FrameTooLarge)?;
        if length > u8::MAX as usize {
            return Err(FrameError::FrameTooLarge);
        }
        if output.len() < length {
            return Err(FrameError::BufferTooSmall);
        }

        output[0] = (WIRE_VERSION << 4) | self.frame_type as u8;
        output[1] = if self.destination.is_some() {
            FLAG_DESTINATION
        } else {
            0
        };
        output[2..6].copy_from_slice(&self.source.0.to_be_bytes());
        output[6..10].copy_from_slice(&self.boot_id.0.to_be_bytes());
        output[10..12].copy_from_slice(&self.sequence.0.to_be_bytes());
        let mut cursor = FRAME_HEADER_LEN;
        if let Some(destination) = self.destination {
            output[cursor..cursor + DESTINATION_LEN].copy_from_slice(&destination.0.to_be_bytes());
            cursor += DESTINATION_LEN;
        }
        output[cursor..length].copy_from_slice(payload);
        Ok(length)
    }

    pub fn decode(input: &[u8]) -> Result<DecodedFrame<'_>, FrameError> {
        if input.len() < FRAME_HEADER_LEN {
            return Err(FrameError::Truncated);
        }
        let version = input[0] >> 4;
        if version != WIRE_VERSION {
            return Err(FrameError::UnsupportedVersion(version));
        }
        let frame_type = FrameType::from_wire(input[0] & 0x0F)?;
        let flags = input[1];
        if flags & !KNOWN_FLAGS != 0 {
            return Err(FrameError::Malformed);
        }
        let has_destination = flags & FLAG_DESTINATION != 0;
        if has_destination && matches!(frame_type, FrameType::Heartbeat | FrameType::Presence) {
            return Err(FrameError::InvalidDestination);
        }
        let header_len = FRAME_HEADER_LEN + usize::from(has_destination) * DESTINATION_LEN;
        if input.len() < header_len {
            return Err(FrameError::Truncated);
        }
        let source = NodeId(u32::from_be_bytes([input[2], input[3], input[4], input[5]]));
        let boot_id = BootId(u32::from_be_bytes([input[6], input[7], input[8], input[9]]));
        let sequence = Sequence(u16::from_be_bytes([input[10], input[11]]));
        let destination = has_destination.then(|| {
            NodeId(u32::from_be_bytes([
                input[FRAME_HEADER_LEN],
                input[FRAME_HEADER_LEN + 1],
                input[FRAME_HEADER_LEN + 2],
                input[FRAME_HEADER_LEN + 3],
            ]))
        });
        Ok(DecodedFrame {
            header: Self {
                frame_type,
                source,
                boot_id,
                sequence,
                destination,
            },
            payload: &input[header_len..],
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DecodedFrame<'a> {
    pub header: FrameHeader,
    pub payload: &'a [u8],
}
