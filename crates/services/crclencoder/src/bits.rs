use crate::Error;

#[cfg(any(test, feature = "loop-benchmark"))]
#[path = "bits_legacy.rs"]
pub(crate) mod legacy;
#[cfg(test)]
#[path = "bits_tests.rs"]
mod tests;

/// Capacity-checked MSB-first u32 reservoir. Payloads may start at any byte
/// alignment; flushes use safe byte-slice stores, never aligned pointer casts.
pub(crate) struct Bits<'a> {
    bytes: &'a mut [u8],
    /// Number of complete bytes flushed (a multiple of four).
    position: usize,
    word: u32,
    /// Pending bits occupy the high end of word; unused low bits are zero.
    used: u32,
    remaining: usize,
}
impl<'a> Bits<'a> {
    pub fn new(bytes: &'a mut [u8]) -> Self {
        let remaining = bytes.len().saturating_mul(8);
        Self {
            bytes,
            position: 0,
            word: 0,
            used: 0,
            remaining,
        }
    }
    #[inline]
    pub fn write(&mut self, value: u32, width: u8) -> Result<(), Error> {
        if width > 32 || width as usize > self.remaining {
            return Err(Error::OutputTooSmall);
        }
        self.remaining -= width as usize;
        self.append(value, u32::from(width));
        Ok(())
    }

    /// Emit the entire common Rice codeword in one append. Long unary runs use
    /// zero-word appends followed by the combined stop bit and remainder.
    #[inline]
    pub fn rice(&mut self, value: u32, k: u8) -> Result<(), Error> {
        if k > 31 {
            return Err(Error::InvalidConfig);
        }
        let k = u32::from(k);
        let mut zeros = value >> k;
        let width = zeros.checked_add(1 + k).ok_or(Error::OutputTooSmall)?;
        if width as usize > self.remaining {
            return Err(Error::OutputTooSmall);
        }
        self.remaining -= width as usize;
        let code = (1 << k) | (value & ((1 << k) - 1));
        if width <= 32 {
            self.append(code, width);
        } else {
            while zeros >= 32 {
                self.append(0, 32);
                zeros -= 32;
            }
            self.append(0, zeros);
            self.append(code, k + 1);
        }
        Ok(())
    }

    /// Caller has reserved capacity and proved width <= 32. One field can
    /// cross at most one reservoir boundary. No shift by 32 is evaluated.
    #[inline(always)]
    fn append(&mut self, value: u32, width: u32) {
        if width == 0 {
            return;
        }
        let free = 32 - self.used;
        self.word |= (value << (32 - width)) >> self.used;
        if width < free {
            self.used += width;
        } else {
            self.bytes[self.position..self.position + 4].copy_from_slice(&self.word.to_be_bytes());
            self.position += 4;
            self.used = width - free;
            self.word = if self.used == 0 {
                0
            } else {
                value << (32 - self.used)
            };
        }
    }

    pub fn finish(self) -> Result<usize, Error> {
        // Capacity checked on every append guarantees that this rounded byte
        // count fits. Pending low bits already contain the required zero pad.
        let tail = self.used.div_ceil(8) as usize;
        self.bytes[self.position..self.position + tail]
            .copy_from_slice(&self.word.to_be_bytes()[..tail]);
        Ok(self.position + tail)
    }
}

pub(crate) fn fold(value: i32) -> u32 {
    if value >= 0 {
        (value as u32) * 2
    } else {
        ((-value) as u32) * 2 - 1
    }
}
