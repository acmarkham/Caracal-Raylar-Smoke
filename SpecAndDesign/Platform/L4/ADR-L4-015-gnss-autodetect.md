# ADR-L4-015: Detect u-blox M7/M8/M10 at runtime

Date: 2026-11-26

Status: Accepted; physical qualification across receiver variants is ongoing.

## Context

Evo boards have interchangeable external u-blox receivers. Selecting a GPS
generation with a Cargo feature produces board-specific firmware and is not a
viable deployment model. M7/M8 and M10 differ in configuration and backup
commands, initial baud, and NAV-PVT payload layout. Identity polling must work
before a fix, including when a separately powered receiver remains in backup
or retains the baud set by an earlier run.

## Decision

The normal `aardwolf-l4` image includes all three receiver profiles. Remove the
`gps-m8` feature. Keep this adaptation in the L4 composition; U5 code, shared
GPS policy, estimator and shared protocol helpers remain unchanged.

At every MCU boot:

1. Sweep 38400, 9600, 57600, 115200, 19200, 4800 and 230400 baud. At each rate,
   transmit the UART wake byte, allow 500 ms for wake, and retry UBX-MON-VER
   every 500 ms for up to two seconds. Reset framing state between rates and
   recover UART errors. Make at most two complete sweeps for booting modules.
2. Accept identity only from a complete, checksum-validated MON-VER response.
   Decode hardware `00070000`/`00080000`/`000A0000` as M7/M8/M10. PROTVER
   major 14, 15–23, or 34 respectively confirms that family, or identifies it
   when the hardware string is unknown. Older ROMs can omit PROTVER. Reject
   unsupported protocol families and conflicting hardware/protocol identities.
   Accept both the older `PROTVER 15.00` and newer `PROTVER=34.10` extension
   formats.
3. Configure UART1 at the discovered baud, then switch the MCU to 38400 after
   100 ms. Read back the profile at 38400, retrying queries every 500 ms for a
   maximum of five seconds per layer. A baud-change ACK may arrive at either
   baud, so verified readback is required instead of relying on that ACK.
   Legacy CFG-PRT verification masks the defined character-length, parity and
   stop-bit fields. Reserved mode bits can differ on readback: the captured M8
   ROM 2.01 returns `0x08c0` for the requested `0x08d0`, both 8N1. Accept either
   documented no-parity encoding (`10X`), while rejecting wrong data length,
   parity or stop bits. Baud and UBX/NMEA input/output remain verified.
4. Construct the shared GpsDriver with the detected receiver's static standby
   command. All generations retain the existing UART wake byte. No navigation
   reset, PB4 enable pulse, cold-start command, or receiver flash write occurs.

| Family | UART configuration/readback | Backup request | NAV-PVT |
| --- | --- | --- | --- |
| M7 | CFG-PRT UART1, 38400 8N1, UBX+NMEA input/output; volatile | RXM-PMREQ, 8-byte payload, indefinite backup | 84-byte legacy layout |
| M8 | Same legacy CFG-PRT profile | Same legacy backup request | 84/92-byte legacy layouts |
| M10 | Existing VALSET profile in RAM+BBR; VALGET verifies both layers | RXM-PMREQ version 0, 16-byte payload, forced backup, UART RX wake | 92-byte layout with invalidLlh checked |

The L4 parser normalizes legacy NAV-PVT before using the unchanged shared fix
checks. Legacy reserved byte 78 is not interpreted as M10 invalidLlh. UTC,
valid-fix flags, 3D fix, satellite count and coordinate checks remain required.
The transport recognizes the selected standby command for every family, so
sleep bookkeeping and UART/PPS silence verification also apply to M7/M8.

RTT records each scanned baud, hardware/software and model/protocol extensions,
selected family, discovered baud and successful profile readback. Syslog records
family, hardware/software identity, discovered baud and profile verification.
Failure to identify or configure a receiver uses the existing latched panic
fault indication (red LED and error buzzer), rather than starting with a guessed
profile. A different identity after wake requires restarting Evo.

## Scope and limits

Detection is a boot operation, independent of navigation acquisition. It does
not wait for PPS/fix before identifying the receiver. Real qualified UTC/PPS is
still required before recording. Existing initial calibration, phase-qualified
shutdown, holdover and audio/storage settings are unchanged.

Swap receivers with equipment powered down and boot/reset Evo afterwards;
live electrical hot swapping is outside this decision. This supports the
listed baud rates, 8N1, UART1 and receivers accepting UBX polls. Arbitrary
saved UART formats, disabled UBX input, unrelated GNSS chipsets and other
u-blox generations are outside the detection contract. A missing receiver
fails after the bounded scan; recovery requires a restart.

## Validation

Host tests cover generation detection with/without PROTVER, protocol fallback,
conflicting/unknown/truncated identities, mixed-stream UBX framing, backup
packet checksums, legacy UART readback rejection, legacy NAV-PVT layouts and
preservation of UTC/fix qualification. Test all L4/board policy checks and the
unchanged shared u-blox tests, build both FLAC and WAV L4 profiles, then build
and test U5 Aardwolf separately with its existing target and settings.

For each physical M7/M8/M10 variant, use the same normal ELF and record its
SHA-256, discovered baud, MON-VER extensions, selected family and profile
readback. Verify real UTC/PPS anchors, two full mono FLAC files with concurrent
syslog and no DMA/storage faults. Follow with standby/silence, UART wake and
reacquisition checks for each receiver family; current draw requires separate
bench measurement. Results belong in the application's validation record and
raw local RTT evidence. Host tests alone do not qualify a physical receiver.

The original normal ELF passed M10 identification at retained 38400
baud, RAM/BBR profile verification, real UTC/PPS admission and two complete
mono FLAC rotations with concurrent syslog. A subsequent same-image M8 test
identified an over-strict reserved-bit comparison in legacy UART readback.
The revised common image corrects that comparison and supports both PROTVER
extension formats; there is still no receiver-specific build selection. See the
[validation record](../../../apps/aardwolf_l4/VALIDATION-GNSS-AUTODETECT-20261126.md)
for the exact image hash, receiver identity and remaining physical checks.

Read-only acquisition diagnostics now accompany the common image: bounded
legacy configuration polls after wake, periodic per-satellite/navigation/RF
status, and brief raw GSA/GSV/PVT windows before fix admission. They do not
retune navigation, relax time/fix checks or change U5. Legacy-only queries are
skipped for M10. The capture tool supports an explicit longer timeout for cold
starts. The [V.KEL acquisition report](../../../apps/aardwolf_l4/VALIDATION-GNSS-ACQUISITION-20261126.md)
records the observed M7 identity, 13-minute-45-second native TTFF, later UTC
resolution, two recording passes and candidate stationary/retained-aiding
settings. Actual M6 support was not needed or implemented for this module.

## Protocol references

- [u-blox 7 protocol V14](https://content.u-blox.com/sites/default/files/products/documents/u-blox7-V14_ReceiverDescriptionProtocolSpec_%28GPS.G7-SW-12001%29_Public.pdf): CFG-PRT, MON-VER, NAV-PVT and RXM-PMREQ.
- [u-blox 8/M8 protocol specification](https://content.u-blox.com/sites/default/files/products/documents/u-blox8-M8_ReceiverDescrProtSpec_UBX-13003221.pdf): legacy UART, navigation and power-management messages.
- [u-blox M10 SPG 5.10 interface description](https://content.u-blox.com/sites/default/files/u-blox-M10-SPG-5.10_InterfaceDescription_UBX-21035062.pdf): hardware/PROTVER identity, VALSET/VALGET and RXM-PMREQ version 0.
