# L4 GPS hardware evidence

Validation date: 2026-11-24. Operator identifies the fitted GPS as Quescan
G10A-F30 / u-blox M10. Board revision, harness control polarity and electrical
levels have not been independently identified.

**Final hardware result: PASS.** See the complete
[RTT log](hardware-final.log), [capture identity](hardware-final.json) and
[parsed summary](hardware-summary.json).

| Check | Observed result |
| --- | --- |
| UART | USART2 PA2/PA3, 38400 baud, 8N1. Two valid frames establish rate; MON-VER and both PUBX responses establish both directions. |
| Receiver software | `ROM SPG 5.10 (7b202e)`; `FWVER=SPG 5.10`; `PROTVER=34.10`. |
| Receiver hardware | `000A0000`. No `MOD=` extension was returned; the Quescan product name remains operator identification. |
| Other extensions | `GPS;GLO;GAL;BDS` and `SBAS;QZSS`. |
| Navigation | 26 qualified UBX-NAV-PVT reports across the final capture, 22–24 satellites, 3D/gnssFixOK and valid/fully resolved UTC. GGA/RMC and NAV-PVT agree on the receiver UTC second at PASS. |
| PPS | PB5 rising edges received independently of UART. Last status: 55 edges, 54 consecutive valid intervals. Logged cumulative min/max at edge 50: 999991/999992 microseconds; no broken intervals. |
| Serial integrity | 990 expected error IRQs while deliberately listening at the wrong 9600 baud; zero after locking to 38400. One rejected partial frame at acquisition, zero parse errors. |
| Combined acceptance | PASS at 20.453 seconds after UART initialization: eight qualified NAV-PVT epochs, sixteen GGA epochs, 21 PPS edges / twenty valid intervals. Continued monitoring for more than 35 seconds after PASS. |
| Memory | 43,568 bytes flash; 7,332 bytes SRAM1 statics, leaving 90,972 bytes for stack/unused space. Initial MSP `0x20018000`; SRAM2 unused. No runtime heap calls. Stack watermark was not measured. |

The first qualified NAV-PVT arrived 5.567 seconds after firmware start. The
receiver was already acquiring/powered before that reset: **this is not a
cold-start TTFF measurement**. No receiver reset, power/standby command, baud
change, constellation change or saved-configuration write was sent. PB4 stayed
input/no pull and read high. Precise UTC/PPS association, waveform electrical
checks, timer capture and Time Service calibration remain unqualified.

The blue LED toggles per observed PPS edge; a complete blue blink cycle takes
two seconds. Green/red stay off. The MCU was reset after capture to leave this
GPS image running; [reset result](standalone-reset.json). The SD card is unused.

[build.json](build.json) records source hashes, base Git commit, tools, command,
lockfile hash and ELF layout. [firmware.zip](firmware.zip) contains the exact
final ELF/BIN, lockfile, changed build inputs, generated memory map, feature
graph and the initial development ELF. Verify its
[SHA-256](firmware.zip.sha256). Rebuilding needs the recorded base repository
plus the archived source changes; it is not a complete source checkout.
Use the explicit leaf `defmt.toml` when building; USART error accounting stays
in the leaf while per-byte HAL IRQ logging is suppressed.

The initial development image's 28-second listen-only sweep saw no UART bytes
or PPS and stopped before sending commands. The operator then reported PPS
activity. Several retries failed to open ST-Link through WinUSB, and one saw
no probe during reconnection. These are retained as `hardware-initial`,
`hardware-retry`, `hardware-baud-sweep`, `hardware-live` and `hardware-reconnect`.
After USB reconnection, `hardware-enabled` and `hardware-repeat` passed using
the initial ELF. Their noisy wrong-baud logs motivated bounded error counters
and explicit log filtering. `hardware-usart-filter` is an intermediate counter
image; its narrower log filter still admitted HAL warnings. Only
`hardware-final` corresponds to the final source and archived final ELF hash.

U5 regression: all 61 existing firmware packages build individually; six
relevant U5/previous-L4 feature graphs are identical, and 69 pre-existing U5
ELF files retain their pre-change hashes. Portable host suites have 203 passes
and the same five pre-existing failures as 02; integration004 adds 13 passes.
The new GPS protocol tests add six passes. The build guard rejects the default
U5 target. See [regression.json](regression.json), [U5 build logs](u5-build-logs.zip),
[portable host log](host-portable.log), [integration004](host-integration004.log),
[GPS decoder tests](host-gps.log) and [wrong-target check](wrong-target.log).
U5 hardware and a non-Windows build host were not tested in this round.
