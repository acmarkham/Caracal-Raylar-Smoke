use crate::gps::types::RawNmeaSentence;
use heapless::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FramerEvent<const N: usize> {
    Sentence(RawNmeaSentence<N>),
    ChecksumError,
    Overflow,
}

pub struct NmeaFramer<const N: usize> {
    buf: Vec<u8, N>,
    in_sentence: bool,
}

impl<const N: usize> NmeaFramer<N> {
    pub const fn new() -> Self {
        Self {
            buf: Vec::new(),
            in_sentence: false,
        }
    }

    pub fn push(&mut self, byte: u8) -> Option<FramerEvent<N>> {
        match byte {
            b'$' => {
                self.buf.clear();
                self.in_sentence = true;
                self.buf.push(byte).ok()?;
                None
            }
            b'\r' => None,
            b'\n' if self.in_sentence => self.finish(),
            _ if self.in_sentence => {
                if self.buf.push(byte).is_err() {
                    self.buf.clear();
                    self.in_sentence = false;
                    Some(FramerEvent::Overflow)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn finish(&mut self) -> Option<FramerEvent<N>> {
        self.in_sentence = false;

        let event = if checksum_valid(&self.buf) {
            RawNmeaSentence::new(&self.buf)
                .map(FramerEvent::Sentence)
                .ok()
        } else {
            Some(FramerEvent::ChecksumError)
        };

        self.buf.clear();
        event
    }
}

pub fn checksum_valid(sentence: &[u8]) -> bool {
    if sentence.first() != Some(&b'$') {
        return false;
    }

    let Some(star) = sentence.iter().position(|b| *b == b'*') else {
        return false;
    };

    if sentence.len() < star + 3 {
        return false;
    }

    let mut checksum = 0u8;
    for b in &sentence[1..star] {
        checksum ^= *b;
    }

    let Some(expected) = parse_hex_byte(sentence[star + 1], sentence[star + 2]) else {
        return false;
    };

    checksum == expected
}

fn parse_hex_byte(high: u8, low: u8) -> Option<u8> {
    Some((hex_nibble(high)? << 4) | hex_nibble(low)?)
}

fn hex_nibble(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_checksum() {
        assert!(checksum_valid(
            b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*47"
        ));
        assert!(!checksum_valid(
            b"$GPGGA,123519,4807.038,N,01131.000,E,1,08,0.9,545.4,M,46.9,M,,*00"
        ));
    }

    #[test]
    fn frames_sentence() {
        let mut framer = NmeaFramer::<128>::new();
        let mut event = None;
        for b in b"$GPRMC,092751.000,A,5321.6802,N,00630.3372,W,0.06,31.66,280511,,,A*43\r\n" {
            event = framer.push(*b).or(event);
        }
        assert!(matches!(event, Some(FramerEvent::Sentence(_))));
    }
}
