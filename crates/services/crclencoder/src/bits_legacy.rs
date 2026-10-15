//! Frozen byte writer for compatibility tests and benchmark comparisons only.
use crate::Error;

pub(crate) struct Bits<'a> {
    bytes: &'a mut [u8],
    position: usize,
}
impl<'a> Bits<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        Self { bytes, position: 0 }
    }
    pub fn write(&mut self, value: u32, width: u8) -> Result<(), Error> {
        if width > 32
            || self
                .position
                .checked_add(width as usize)
                .is_none_or(|end| end > self.bytes.len().saturating_mul(8))
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
            let part = (value >> (remaining - take)) & ((1 << take) - 1);
            self.bytes[index] |= (part as u8) << (8 - offset - take);
            self.position += take;
            remaining -= take;
        }
        Ok(())
    }
    pub fn unary(&mut self, mut zeros: u32) -> Result<(), Error> {
        if zeros as usize >= self.bytes.len().saturating_mul(8) - self.position {
            return Err(Error::OutputTooSmall);
        }
        while zeros != 0 {
            let take = zeros.min(32) as u8;
            self.write(0, take)?;
            zeros -= u32::from(take);
        }
        self.write(1, 1)
    }
    pub fn finish(mut self) -> Result<usize, Error> {
        self.write(0, ((8 - self.position % 8) % 8) as u8)?;
        Ok(self.position / 8)
    }
}
