//! Bounded mixed NMEA/UBX framing and M10 protocol. See ADR-L4-012.

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

/// UBX framing, with an explicit output bound usable for static commands.
pub const fn packet<const N: usize>(class: u8, id: u8, payload: &[u8]) -> [u8; N] {
    assert!(N == payload.len() + 8 && payload.len() <= u16::MAX as usize);
    let mut out = [0; N];
    out[0] = 0xb5;
    out[1] = 0x62;
    out[2] = class;
    out[3] = id;
    out[4] = payload.len() as u8;
    out[5] = (payload.len() >> 8) as u8;
    let mut i = 0;
    while i < payload.len() {
        out[6 + i] = payload[i];
        i += 1;
    }
    let (mut a, mut b) = (0u8, 0u8);
    i = 2;
    while i < N - 2 {
        a = a.wrapping_add(out[i]);
        b = b.wrapping_add(a);
        i += 1;
    }
    out[N - 2] = a;
    out[N - 1] = b;
    out
}

/// UBX-21035062 R03, RXM-PMREQ v0: indefinite forced backup, UART RX wake.
/// VCC remains supplied. No CFG-RST and no navigation database clearing.
pub const STANDBY: [u8; 24] = packet(
    0x02,
    0x41,
    &[0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 8, 0, 0, 0],
);
pub const WAKE: [u8; 1] = [0xff];
pub const MON_VER: [u8; 8] = packet(0x0a, 0x04, &[]);
pub const NAV_PVT: [u8; 8] = packet(0x01, 0x07, &[]);
pub const UART_KEYS: [u32; 6] = [
    0x40520001, 0x10520005, 0x10730001, 0x10730002, 0x10740001, 0x10740002,
];
/// Save only the UART transport to RAM + BBR; never write receiver flash.
pub const UART_PROFILE: [u8; 45] = packet(
    6,
    0x8a,
    &[
        0, 3, 0, 0, 1, 0, 0x52, 0x40, 0, 0x96, 0, 0, 5, 0, 0x52, 0x10, 1, 1, 0, 0x73, 0x10, 1, 2,
        0, 0x73, 0x10, 1, 1, 0, 0x74, 0x10, 1, 2, 0, 0x74, 0x10, 1,
    ],
);
pub fn uart_profile_poll(layer: u8) -> [u8; 36] {
    let mut payload = [0; 28];
    payload[1] = layer;
    for (i, key) in UART_KEYS.iter().enumerate() {
        payload[4 + i * 4..8 + i * 4].copy_from_slice(&key.to_le_bytes());
    }
    packet(6, 0x8b, &payload)
}
/// Require the complete requested profile, correct types and no trailing data.
pub fn uart_profile_matches(p: &[u8], layer: u8) -> bool {
    if p.len() != 37 || p[..4] != [1, layer, 0, 0] {
        return false;
    }
    let mut offset = 4;
    let mut seen = 0u8;
    while offset + 4 <= p.len() {
        let key = u32::from_le_bytes(p[offset..offset + 4].try_into().unwrap());
        let Some(i) = UART_KEYS.iter().position(|expected| *expected == key) else {
            return false;
        };
        if seen & (1 << i) != 0 {
            return false;
        }
        seen |= 1 << i;
        offset += 4;
        if i == 0 {
            if p.get(offset..offset + 4) != Some(38400u32.to_le_bytes().as_slice()) {
                return false;
            }
            offset += 4;
        } else {
            if p.get(offset) != Some(&1) {
                return false;
            }
            offset += 1;
        }
    }
    offset == p.len() && seen == 0x3f
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
    fn standby_is_forced_backup_with_uart_wake_and_retains_aiding() {
        let mut f = Framer::default();
        for byte in STANDBY {
            if let Some(Frame::Ubx { class, id, payload }) = f.push(byte) {
                assert_eq!((class, id), (2, 0x41));
                assert_eq!(payload.len(), 16);
                assert_eq!(&payload[..8], &[0; 8]);
                assert_eq!(&payload[8..12], &6u32.to_le_bytes());
                assert_eq!(&payload[12..], &8u32.to_le_bytes());
                return;
            }
        }
        panic!("invalid standby packet");
    }
    #[test]
    fn uart_profile_uses_ram_bbr_without_flash_and_requires_exact_readback() {
        assert_eq!(&UART_PROFILE[6..10], &[0, 3, 0, 0]);
        let mut response = UART_PROFILE[6..UART_PROFILE.len() - 2].to_vec();
        response[0] = 1;
        response[1] = 0;
        assert!(uart_profile_matches(&response, 0));
        assert!(!uart_profile_matches(&response, 1));
        response[1] = 1;
        assert!(uart_profile_matches(&response, 1));
        let mut reordered = response[..4].to_vec();
        for (start, end) in [(32, 37), (27, 32), (22, 27), (17, 22), (12, 17), (4, 12)] {
            reordered.extend_from_slice(&response[start..end]);
        }
        assert!(uart_profile_matches(&reordered, 1));
        let mut duplicate = response.clone();
        duplicate[12..16].copy_from_slice(&UART_KEYS[2].to_le_bytes());
        assert!(!uart_profile_matches(&duplicate, 1));
        for i in [4, 8, 12, 16, 21, 26, 31, 36] {
            let mut bad = response.clone();
            bad[i] ^= 1;
            assert!(!uart_profile_matches(&bad, 1));
        }
        assert!(!uart_profile_matches(&response[..36], 1));
        for layer in [0, 1] {
            let poll = uart_profile_poll(layer);
            assert_eq!(&poll[..8], &[0xb5, 0x62, 6, 0x8b, 28, 0, 0, layer]);
            assert_eq!(ubx_checksum(&poll[2..34]), (poll[34], poll[35]));
        }
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
}
