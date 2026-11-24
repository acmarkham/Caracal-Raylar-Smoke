//! Bounded, allocation-free mixed UART framing for this diagnostic only.
#![no_std]

pub enum Frame<'a> {
    Nmea(&'a [u8]),
    Ubx {
        class: u8,
        id: u8,
        payload: &'a [u8],
    },
}

pub struct Framer {
    buffer: [u8; 768],
    len: usize,
    state: u8,
    pub rejected: u32,
}

impl Default for Framer {
    fn default() -> Self {
        Self {
            buffer: [0; 768],
            len: 0,
            state: 0,
            rejected: 0,
        }
    }
}

impl Framer {
    pub fn reset(&mut self) {
        self.len = 0;
        self.state = 0;
    }

    pub fn push(&mut self, byte: u8) -> Option<Frame<'_>> {
        if self.state == 0 {
            match byte {
                b'$' => {
                    self.state = 1;
                    self.buffer[0] = byte;
                    self.len = 1;
                }
                0xb5 => self.state = 2,
                _ => {}
            }
            return None;
        }
        if self.state == 2 {
            self.state = if byte == 0x62 {
                3
            } else if byte == 0xb5 {
                2
            } else {
                0
            };
            self.len = 0;
            if byte == b'$' {
                self.state = 1;
                self.buffer[0] = byte;
                self.len = 1;
            }
            return None;
        }
        if self.state == 1 && byte == b'$' {
            self.rejected += 1;
            self.len = 0;
        }
        if self.len == self.buffer.len() {
            self.rejected += 1;
            self.reset();
            return None;
        }
        self.buffer[self.len] = byte;
        self.len += 1;
        if self.state == 1 && byte == b'\n' {
            let len = self.len;
            self.reset();
            if nmea_valid(&self.buffer[..len]) {
                return Some(Frame::Nmea(&self.buffer[..len]));
            }
            self.rejected += 1;
        } else if self.state == 3 && self.len >= 4 {
            let payload_len = u16::from_le_bytes([self.buffer[2], self.buffer[3]]) as usize;
            if payload_len + 6 > self.buffer.len() {
                self.rejected += 1;
                self.reset();
            } else if self.len == payload_len + 6 {
                let (a, b) = ubx_checksum(&self.buffer[..payload_len + 4]);
                self.reset();
                if self.buffer[payload_len + 4] == a && self.buffer[payload_len + 5] == b {
                    return Some(Frame::Ubx {
                        class: self.buffer[0],
                        id: self.buffer[1],
                        payload: &self.buffer[4..payload_len + 4],
                    });
                }
                self.rejected += 1;
            }
        }
        None
    }
}

pub fn ubx_checksum(bytes: &[u8]) -> (u8, u8) {
    bytes.iter().fold((0u8, 0u8), |(a, b), byte| {
        let a = a.wrapping_add(*byte);
        (a, b.wrapping_add(a))
    })
}

pub fn poll(class: u8, id: u8) -> [u8; 8] {
    let (a, b) = ubx_checksum(&[class, id, 0, 0]);
    [0xb5, 0x62, class, id, 0, 0, a, b]
}

pub fn nmea_valid(sentence: &[u8]) -> bool {
    let Some(body) = sentence
        .strip_prefix(b"$")
        .and_then(|s| s.strip_suffix(b"\r\n"))
    else {
        return false;
    };
    let Some(star) = body.iter().position(|b| *b == b'*') else {
        return false;
    };
    if star + 3 != body.len() || !body[..star].iter().all(|b| (32..=126).contains(b)) {
        return false;
    }
    let hex = |b: u8| match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'A'..=b'F' => Some(b - b'A' + 10),
        b'a'..=b'f' => Some(b - b'a' + 10),
        _ => None,
    };
    match (hex(body[star + 1]), hex(body[star + 2])) {
        (Some(a), Some(b)) => body[..star].iter().fold(0, |sum, b| sum ^ b) == a * 16 + b,
        _ => false,
    }
}

pub fn ascii_field(field: &[u8]) -> Option<&str> {
    let end = field.iter().position(|b| *b == 0).unwrap_or(field.len());
    if end == 0 || !field[..end].iter().all(|b| (32..=126).contains(b)) {
        return None;
    }
    core::str::from_utf8(&field[..end]).ok()
}

pub struct Version<'a> {
    pub software: &'a str,
    pub hardware: &'a str,
    pub extensions: &'a [u8],
}

pub fn version(payload: &[u8]) -> Option<Version<'_>> {
    if payload.len() < 40 || (payload.len() - 40) % 30 != 0 {
        return None;
    }
    Some(Version {
        software: ascii_field(&payload[..30])?,
        hardware: ascii_field(&payload[30..40])?,
        extensions: &payload[40..],
    })
}

#[derive(Clone, Copy, Debug)]
pub struct Pvt {
    pub tow_ms: u32,
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub valid: u8,
    pub fix_type: u8,
    pub flags: u8,
    pub satellites: u8,
    pub longitude_e7: i32,
    pub latitude_e7: i32,
    pub horizontal_accuracy_mm: u32,
    pub position_dop_centi: u16,
    pub invalid_llh: bool,
}

impl Pvt {
    pub fn qualified(&self) -> bool {
        self.fix_type == 3
            && self.flags & 1 != 0
            && self.valid & 7 == 7
            && self.satellites >= 4
            && !self.invalid_llh
            && (-900_000_000..=900_000_000).contains(&self.latitude_e7)
            && (-1_800_000_000..=1_800_000_000).contains(&self.longitude_e7)
            && self.year >= 2020
            && (1..=12).contains(&self.month)
            && (1..=31).contains(&self.day)
            && self.hour < 24
            && self.minute < 60
            && self.second <= 60
    }
}

pub fn nav_pvt(p: &[u8]) -> Option<Pvt> {
    if p.len() != 92 {
        return None;
    }
    let u32_at = |i| u32::from_le_bytes(p[i..i + 4].try_into().unwrap());
    Some(Pvt {
        tow_ms: u32_at(0),
        year: u16::from_le_bytes([p[4], p[5]]),
        month: p[6],
        day: p[7],
        hour: p[8],
        minute: p[9],
        second: p[10],
        valid: p[11],
        fix_type: p[20],
        flags: p[21],
        satellites: p[23],
        longitude_e7: u32_at(24) as i32,
        latitude_e7: u32_at(28) as i32,
        horizontal_accuracy_mm: u32_at(40),
        position_dop_centi: u16::from_le_bytes([p[76], p[77]]),
        invalid_llh: p[78] & 1 != 0,
    })
}

/// Diagnostic software timestamps, never UTC anchors or oscillator calibration.
#[derive(Clone, Copy, Default)]
pub struct PpsCheck {
    pub edges: u32,
    pub streak: u32,
    pub bad_intervals: u32,
    pub last_us: Option<u64>,
    pub minimum_us: Option<u64>,
    pub maximum_us: Option<u64>,
}

impl PpsCheck {
    pub fn edge(&mut self, now_us: u64) -> Option<u64> {
        let delta = self.last_us.map(|last| now_us.saturating_sub(last));
        if let Some(delta) = delta {
            self.minimum_us = Some(self.minimum_us.map_or(delta, |old| old.min(delta)));
            self.maximum_us = Some(self.maximum_us.map_or(delta, |old| old.max(delta)));
            if (990_000..=1_010_000).contains(&delta) {
                self.streak += 1;
            } else {
                self.streak = 0;
                self.bad_intervals += 1;
            }
        }
        self.last_us = Some(now_us);
        self.edges += 1;
        delta
    }

    pub fn qualified(&self, now_us: u64) -> bool {
        self.streak >= 20
            && self
                .last_us
                .is_some_and(|last| now_us.saturating_sub(last) < 1_500_000)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;
    use super::*;
    use std::vec::Vec;

    fn packet(class: u8, id: u8, payload: &[u8]) -> Vec<u8> {
        let mut data = std::vec![
            0xb5,
            0x62,
            class,
            id,
            payload.len() as u8,
            (payload.len() >> 8) as u8
        ];
        data.extend_from_slice(payload);
        let (a, b) = ubx_checksum(&data[2..]);
        data.extend_from_slice(&[a, b]);
        data
    }

    #[test]
    fn published_poll_and_ascii_checksums() {
        assert_eq!(poll(0x0a, 0x04), [0xb5, 0x62, 0x0a, 4, 0, 0, 0x0e, 0x34]);
        assert!(nmea_valid(b"$PUBX,00*33\r\n"));
        assert!(nmea_valid(b"$PUBX,04*37\r\n"));
        assert!(!nmea_valid(b"$PUBX,04*33\r\n"));
        assert!(!nmea_valid(b"$PUBX,04*37junk\r\n"));
        assert!(!nmea_valid(b"$PUBX,04*ZZ\r\n"));
    }

    #[test]
    fn mixed_stream_embedded_delimiters_and_noise() {
        let mut stream = std::vec![0, 0xff, 0xb5];
        stream.extend(packet(10, 4, b"$\r\n\xb5\x62*"));
        stream.extend_from_slice(b"noise$PUBX,00*33\r\n");
        let mut framer = Framer::default();
        let mut kinds = Vec::new();
        for byte in stream {
            if let Some(frame) = framer.push(byte) {
                kinds.push(match frame {
                    Frame::Ubx { payload, .. } => {
                        assert_eq!(payload, b"$\r\n\xb5\x62*");
                        1
                    }
                    Frame::Nmea(_) => 2,
                });
            }
        }
        assert_eq!(kinds, [1, 2]);
    }

    #[test]
    fn corruption_oversize_truncation_and_resynchronization() {
        let mut stream = packet(10, 4, b"bad");
        *stream.last_mut().unwrap() ^= 1;
        stream.extend_from_slice(b"$garbage\r\n\xb5\x62\x0a\x04\xff\xff");
        stream.extend_from_slice(b"$truncated$PUBX,04*37\r\n");
        let mut f = Framer::default();
        let mut received = 0;
        for b in stream {
            if f.push(b).is_some() {
                received += 1;
            }
        }
        assert_eq!(received, 1);
        assert_eq!(f.rejected, 4);
        for b in b"\xb5\x62\x0a\x04\x50\x00partial" {
            f.push(*b);
        }
        f.reset(); // UART idle timeout releases a truncated binary frame.
        for b in poll(10, 4) {
            if f.push(b).is_some() {
                received += 1;
            }
        }
        assert_eq!(received, 2);
    }

    #[test]
    fn version_fields_and_extension_bounds() {
        let mut p = [0; 70];
        p[..8].copy_from_slice(b"ROM TEST");
        p[30..40].copy_from_slice(b"000A0000xx");
        p[40..53].copy_from_slice(b"PROTVER=34.00");
        let v = version(&p).unwrap();
        assert_eq!(v.software, "ROM TEST");
        assert_eq!(v.hardware, "000A0000xx");
        assert_eq!(ascii_field(v.extensions), Some("PROTVER=34.00"));
        assert!(version(&p[..39]).is_none());
        assert!(version(&p[..69]).is_none());
        p[0] = 0xff;
        assert!(version(&p).is_none());
    }

    fn pvt_payload() -> [u8; 92] {
        let mut p = [0; 92];
        p[4..6].copy_from_slice(&2026u16.to_le_bytes());
        p[6] = 11;
        p[7] = 24;
        p[11] = 7;
        p[20] = 3;
        p[21] = 1;
        p[23] = 8;
        p[24..28].copy_from_slice(&(-1_234_567i32).to_le_bytes());
        p[28..32].copy_from_slice(&512_345_678i32.to_le_bytes());
        p
    }

    #[test]
    fn fix_requires_flags_time_coordinates_and_satellites() {
        let p = pvt_payload();
        let nav = nav_pvt(&p).unwrap();
        assert!(nav.qualified());
        assert_eq!(nav.longitude_e7, -1_234_567);
        assert!(nav_pvt(&p[..91]).is_none());
        for (index, value) in [(11, 3), (20, 2), (21, 0), (23, 3), (78, 1), (6, 0), (8, 24)] {
            let mut bad = p;
            bad[index] = value;
            assert!(!nav_pvt(&bad).unwrap().qualified());
        }
        let mut bad = p;
        bad[28..32].copy_from_slice(&900_000_001i32.to_le_bytes());
        assert!(!nav_pvt(&bad).unwrap().qualified());
    }

    #[test]
    fn pps_streak_resets_on_missing_edges_and_expires() {
        let mut p = PpsCheck::default();
        for i in 0..=20 {
            p.edge(i * 1_000_000);
        }
        assert!(p.qualified(20_100_000));
        assert!(!p.qualified(21_500_000));
        p.edge(22_000_000);
        assert_eq!(p.streak, 0);
        assert_eq!(p.bad_intervals, 1);
        for i in 23..=42 {
            p.edge(i * 1_000_000);
        }
        assert!(p.qualified(42_000_000));
    }
}
