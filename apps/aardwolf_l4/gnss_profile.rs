//! Runtime receiver capabilities; deliberately local to the Evo composition.
use raylar_drivers::gps::ublox;

pub const BAUD_RATES: [u32; 7] = [38_400, 9_600, 57_600, 115_200, 19_200, 4_800, 230_400];
pub const LEGACY_UART: [u8; 28] = ublox::packet(
    6,
    0,
    &[
        1, 0, 0, 0, 0xd0, 8, 0, 0, 0, 0x96, 0, 0, 3, 0, 3, 0, 0, 0, 0, 0,
    ],
);
pub const LEGACY_UART_POLL: [u8; 9] = ublox::packet(6, 0, &[1]);
const LEGACY_STANDBY: [u8; 16] = ublox::packet(2, 0x41, &[0, 0, 0, 0, 2, 0, 0, 0]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Receiver {
    M7,
    M8,
    M10,
}

impl Receiver {
    /// MON-VER is already checksum validated by the mixed stream framer.
    /// Hardware identifies older ROMs without PROTVER. A protocol fallback is
    /// limited to the supported families, and conflicting identities fail closed.
    pub fn detect(payload: &[u8]) -> Option<Self> {
        let version = ublox::version(payload)?;
        let hardware = match version.hardware {
            "00070000" => Some(Self::M7),
            "00080000" => Some(Self::M8),
            "000A0000" => Some(Self::M10),
            _ => None,
        };
        let mut protocol = None;
        for ext in version.extensions.chunks_exact(30) {
            let ext = ublox::ascii_field(ext)?;
            if let Some(value) = ext
                .strip_prefix("PROTVER=")
                .or_else(|| ext.strip_prefix("PROTVER "))
            {
                let (major, minor) = value.split_once('.')?;
                if minor.len() != 2 || !minor.bytes().all(|b| b.is_ascii_digit()) {
                    return None;
                }
                protocol = Some(match major.parse::<u8>().ok()? {
                    14 => Self::M7,
                    15..=23 => Self::M8,
                    34 => Self::M10,
                    _ => return None,
                });
            }
        }
        match (hardware, protocol) {
            (Some(a), Some(b)) if a != b => None,
            (Some(a), _) | (_, Some(a)) => Some(a),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::M7 => "M7",
            Self::M8 => "M8",
            Self::M10 => "M10",
        }
    }

    pub const fn standby(self) -> &'static [u8] {
        match self {
            Self::M7 | Self::M8 => &LEGACY_STANDBY,
            Self::M10 => &ublox::STANDBY,
        }
    }

    pub fn nav_pvt(self, payload: &[u8]) -> Option<ublox::Pvt> {
        if self == Self::M10 {
            return ublox::nav_pvt(payload);
        }
        if !matches!(payload.len(), 84 | 92) {
            return None;
        }
        let mut padded = [0; 92];
        padded[..payload.len()].copy_from_slice(payload);
        // This byte is reserved on M7/M8, not the M10 invalidLlh flag.
        padded[78] = 0;
        ublox::nav_pvt(&padded)
    }
}

pub fn legacy_uart_matches(payload: &[u8]) -> bool {
    if payload.len() != 20 {
        return false;
    }
    let mode = u32::from_le_bytes(payload[4..8].try_into().unwrap());
    // CFG-PRT: charLen[7:6]=11, parity[11:9]=10X, nStopBits[13:12]=00.
    // Older M8 ROMs clear reserved bit 4 (0x08c0 instead of 0x08d0).
    payload[0] == 1
        && mode & 0x3cc0 == 0x08c0
        && payload[8..12] == 38_400u32.to_le_bytes()
        && u16::from_le_bytes([payload[12], payload[13]]) & 3 == 3
        && u16::from_le_bytes([payload[14], payload[15]]) & 3 == 3
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(hw: &[u8], protocol: Option<&[u8]>) -> ([u8; 70], usize) {
        let mut p = [0; 70];
        p[..8].copy_from_slice(b"ROM CORE");
        p[30..30 + hw.len()].copy_from_slice(hw);
        if let Some(ext) = protocol {
            p[40..40 + ext.len()].copy_from_slice(ext);
            (p, 70)
        } else {
            (p, 40)
        }
    }

    #[test]
    fn identifies_all_generations_with_and_without_protocol_extension() {
        for (hw, protocol, receiver) in [
            (b"00070000", b"PROTVER=14.00", Receiver::M7),
            (b"00080000", b"PROTVER=18.00", Receiver::M8),
            (b"000A0000", b"PROTVER=34.10", Receiver::M10),
        ] {
            for extension in [Some(protocol.as_slice()), None] {
                let (p, len) = identity(hw, extension);
                assert_eq!(Receiver::detect(&p[..len]), Some(receiver));
            }
            let (p, len) = identity(b"UNKNOWN", Some(protocol));
            assert_eq!(Receiver::detect(&p[..len]), Some(receiver));
        }
    }

    #[test]
    fn rejects_unsupported_conflicting_and_malformed_identities() {
        for (hw, ext) in [
            (b"UNKNOWN".as_slice(), None),
            (b"00070000", Some(b"PROTVER=34.10".as_slice())),
            (b"000A0000", Some(b"PROTVER=27.00".as_slice())),
            (b"00080000", Some(b"PROTVER=18.xx".as_slice())),
        ] {
            let (p, len) = identity(hw, ext);
            assert_eq!(Receiver::detect(&p[..len]), None);
        }
        assert_eq!(Receiver::detect(&[0; 39]), None);
        let (p, _) = identity(b"00080000", Some(b"PROTVER=18.00"));
        assert_eq!(Receiver::detect(&p[..69]), None);
        // Detection also operates on a real checksum-validated UBX stream.
        let packet = ublox::packet::<78>(10, 4, &p);
        let mut framer = ublox::Framer::default();
        let mut detected = None;
        for byte in packet {
            if let Some(ublox::Frame::Ubx { payload, .. }) = framer.push(byte) {
                detected = Receiver::detect(payload);
            }
        }
        assert_eq!(detected, Some(Receiver::M8));
    }

    #[test]
    fn older_m8_protocol_extension_uses_a_space_separator() {
        let (mut p, len) = identity(b"00080000", Some(b"PROTVER 15.00"));
        assert_eq!(Receiver::detect(&p[..len]), Some(Receiver::M8));
        p[30..40].fill(0);
        p[30..37].copy_from_slice(b"UNKNOWN");
        assert_eq!(Receiver::detect(&p[..len]), Some(Receiver::M8));
        p[30..38].copy_from_slice(b"000A0000");
        assert_eq!(Receiver::detect(&p[..len]), None);
    }

    #[test]
    fn runtime_commands_use_generation_specific_backup_and_verified_uart() {
        for receiver in [Receiver::M7, Receiver::M8, Receiver::M10] {
            let command = receiver.standby();
            assert_eq!(&command[2..4], &[2, 0x41]);
            assert_eq!(
                ublox::ubx_checksum(&command[2..command.len() - 2]),
                (command[command.len() - 2], command[command.len() - 1])
            );
            assert_eq!(
                command.len(),
                if receiver == Receiver::M10 { 24 } else { 16 }
            );
        }
        assert!(legacy_uart_matches(&LEGACY_UART[6..26]));
        for offset in [0, 8, 12, 14] {
            let mut bad = LEGACY_UART;
            bad[6 + offset] ^= 1;
            assert!(!legacy_uart_matches(&bad[6..26]));
        }
    }

    #[test]
    fn captured_m8_201_uart_readback_ignores_reserved_mode_bits_only() {
        // Captured from ROM 2.01 (75331), PROTVER 15.00. Reserved bit 4
        // differs from our request; all required UART settings are correct.
        let response = [
            0xb5, 0x62, 6, 0, 20, 0, 1, 0, 0, 0, 0xc0, 8, 0, 0, 0, 0x96, 0, 0, 3, 0, 3, 0, 0, 0, 0,
            0, 0x7f, 0x70,
        ];
        let mut framer = ublox::Framer::default();
        let mut accepted = false;
        for byte in response {
            if let Some(ublox::Frame::Ubx {
                class: 6,
                id: 0,
                payload,
            }) = framer.push(byte)
            {
                accepted = legacy_uart_matches(payload);
            }
        }
        assert!(accepted);
        for mode in [0x08c0u32, 0x08d0, 0x0ac0] {
            let mut payload = LEGACY_UART[6..26].try_into().unwrap_or([0u8; 20]);
            payload[4..8].copy_from_slice(&mode.to_le_bytes());
            assert!(legacy_uart_matches(&payload));
        }
        for mode in [0x0880u32, 0x00c0, 0x02c0, 0x18c0, 0x28c0, 0x38c0] {
            let mut payload = [0u8; 20];
            payload.copy_from_slice(&LEGACY_UART[6..26]);
            payload[4..8].copy_from_slice(&mode.to_le_bytes());
            assert!(!legacy_uart_matches(&payload));
        }
        assert!(!legacy_uart_matches(&response[6..25]));
    }

    #[test]
    fn legacy_pvt_layout_preserves_fix_checks_and_modern_invalid_llh() {
        let mut p = [0; 92];
        p[4..6].copy_from_slice(&2026u16.to_le_bytes());
        p[6] = 11;
        p[7] = 26;
        p[11] = 7;
        p[20] = 3;
        p[21] = 1;
        p[23] = 8;
        p[78] = 1;
        for receiver in [Receiver::M7, Receiver::M8] {
            for len in [84, 92] {
                assert!(receiver.nav_pvt(&p[..len]).unwrap().qualified());
            }
            assert!(receiver.nav_pvt(&p[..83]).is_none());
        }
        assert!(!Receiver::M10.nav_pvt(&p).unwrap().qualified());
        assert!(Receiver::M10.nav_pvt(&p[..84]).is_none());
        p[11] = 3;
        assert!(!Receiver::M7.nav_pvt(&p[..84]).unwrap().qualified());
        p[11] = 7;
        p[21] = 0;
        assert!(!Receiver::M8.nav_pvt(&p).unwrap().qualified());
    }
}
