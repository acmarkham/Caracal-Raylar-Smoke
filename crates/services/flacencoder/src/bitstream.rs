use crate::Error;

pub(crate) struct Bits<'a> {
    bytes: &'a mut [u8],
    position: usize,
}

impl<'a> Bits<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Self { bytes, position: 0 }
    }

    pub fn write(&mut self, value: u64, width: u8) -> Result<(), Error> {
        if width > 64
            || self
                .position
                .checked_add(width as usize)
                .filter(|end| *end <= self.bytes.len().saturating_mul(8))
                .is_none()
        {
            return Err(Error::OutputTooSmall);
        }
        let mut remaining = width as usize;
        while remaining != 0 {
            let index = self.position / 8;
            let offset = self.position % 8;
            if offset == 0 {
                self.bytes[index] = 0;
            }
            let take = remaining.min(8 - offset);
            let fragment = ((value >> (remaining - take)) & ((1 << take) - 1)) as u8;
            self.bytes[index] |= fragment << (8 - offset - take);
            remaining -= take;
            self.position += take;
        }
        Ok(())
    }

    pub fn unary(&mut self, mut zeros: u64) -> Result<(), Error> {
        // Check before emitting, even for a malicious or miscomputed cost.
        let available = self.bytes.len().saturating_mul(8) - self.position;
        if zeros >= available as u64 {
            return Err(Error::OutputTooSmall);
        }
        while zeros != 0 {
            let take = zeros.min(64) as u8;
            self.write(0, take)?;
            zeros -= u64::from(take);
        }
        self.write(1, 1)
    }

    pub fn align(&mut self) -> Result<(), Error> {
        self.write(0, ((8 - self.position % 8) % 8) as u8)
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..self.position.div_ceil(8)]
    }

    pub fn number(&mut self, value: u32) -> Result<(), Error> {
        if value > 0x7fff_ffff {
            return Err(Error::CounterOverflow);
        }
        if value < 128 {
            return self.write(value.into(), 8);
        }
        let bits = 32 - value.leading_zeros();
        let count = if bits <= 11 {
            2
        } else if bits <= 16 {
            3
        } else if bits <= 21 {
            4
        } else if bits <= 26 {
            5
        } else {
            6
        };
        let first_width = 7 - count;
        let prefix = (0xffu8 << (8 - count)) as u32;
        self.write(
            (prefix | (value >> (6 * (count - 1))) & ((1 << first_width) - 1)).into(),
            8,
        )?;
        for index in (0..count - 1).rev() {
            self.write((0x80 | ((value >> (6 * index)) & 0x3f)).into(), 8)?;
        }
        Ok(())
    }
}

pub(crate) fn crc8(bytes: &[u8]) -> u8 {
    let mut crc = 0u8;
    for byte in bytes {
        crc ^= byte;
        for _ in 0..8 {
            crc = (crc << 1) ^ if crc & 0x80 != 0 { 0x07 } else { 0 };
        }
    }
    crc
}

pub(crate) fn crc16(bytes: &[u8]) -> u16 {
    let mut crc = 0u16;
    for byte in bytes {
        crc ^= u16::from(*byte) << 8;
        for _ in 0..8 {
            crc = (crc << 1) ^ if crc & 0x8000 != 0 { 0x8005 } else { 0 };
        }
    }
    crc
}
