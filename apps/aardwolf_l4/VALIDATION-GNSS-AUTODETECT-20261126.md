# L4 runtime GNSS autodetection validation — 2026-11-26

Implementation: [ADR-L4-015](../../SpecAndDesign/Platform/L4/ADR-L4-015-gnss-autodetect.md).
The normal image contains M7/M8/M10 selection at runtime; there is no GPS
generation build feature. U5 and shared driver sources are unchanged.

## Automated checks

- L4 and Evo board host tests: 25 passed (normal codec, revised common image).
- L4 WAV-reference host tests: 14 passed (revised common image).
- Existing shared u-blox tests: 7 passed, 66 unrelated tests filtered out.
- Existing U5 Aardwolf host tests: 6 passed.
- L4 FLAC and WAV firmware and default U5 Aardwolf firmware: release builds passed
  with their separate explicit targets, `--locked --offline`.
- Rustfmt on changed Rust files and `git diff --check`: passed.
- L4 build emits the existing shared `resolve_periodic_capture_delta` dead-code
  warning. No shared source was changed to suppress it.

New tests exercise M7/M8/M10 hardware and protocol identities, older ROMs
without protocol extensions, protocol fallback, conflicting and malformed
identity rejection, framed MON-VER decoding, generation-specific standby
checksums, legacy UART profile readback and legacy/modern fix validity checks.
These are protocol/policy tests; they do not simulate a physical UART baud
transition or qualify receiver power consumption.

## Original frozen normal image

Local image: `target/l4-gnss-auto/aardwolf-l4-autodetect.elf`.

SHA-256: `15099eadc6d07d6951a7ada7c8a986e79374e9817c7d71da73852e637861bdec`.

Flash sections total 269656 bytes. SRAM1 static sections total 96676 of 98304
bytes, leaving 1628 bytes; the existing separate SRAM2 stack arrangement is
unchanged. All recorded L4 firmware source hashes match the frozen build.
Local source/ELF hashes and section sizes are retained in
`evidence/20261126-gnss-auto/build.json` and `memory.txt` (ignored raw evidence).

## M10 hardware run: passed

After reconnecting the ST-Link and fitting the M10, `m10-auto-01` flashed and
verified the exact frozen ELF above. No rebuild or receiver feature selection
was used. Flashing took 135.22 seconds at 100 kHz SWD; the capture completed
successfully in 326.56 seconds including flashing and debugger shutdown.

- MCU UID: `3F001F001050525857323120`; SD serial `804e6392`.
- Detected M10 at 38400 baud, first sweep, at firmware time 0.657187 seconds.
- MON-VER: `ROM SPG 5.10 (7b202e)`, hardware `000A0000`.
- Extensions: `FWVER=SPG 5.10`, `PROTVER=34.10`, `GPS;GLO;GAL;BDS`,
  `SBAS;QZSS`. This receiver supplied no separate MOD extension.
- UART RAM and BBR readback passed at 0.826566 seconds, without receiver flash
  writes. Subsequent software UART wake returned the same identity.
- Real UTC synchronized at 2.152837 seconds. At 180 seconds, 181 PPS events and
  179 accepted anchors were reported, with zero rejected anchors. Drift
  regression reached four samples spanning 182 seconds and +8783 ppb near the
  end of capture. The normal initial calibration had not yet locked.
- Mono 16 kHz/24-bit FLAC started at the next UTC minute. Two files closed with
  960000 samples each: 1178607 bytes at 126.119496 seconds (normalized checksum
  `c15b4623`) and 1234502 bytes at 186.122991 seconds (`d6f409ae`). The third
  file opened normally. These are successful recorder close/write results;
  this run did not independently read back or externally decode the SD files.
- Concurrent syslog reached 31927 appended bytes by 180 seconds. All observed
  CPU windows reported zero dropped/truncated log messages and zero log write
  errors; no audio, storage or panic fault occurred.
- Full recording window (120–180 seconds): CPU 27.5% using the existing executor
  accounting, peak codec time 8821 us. Interrupt work before executor wake is
  counted as idle by this measurement, as on U5. Peak observed backlog was 1024
  samples, peak SD append 54967 us; stack guard stayed intact with 5580 bytes
  remaining below the measured high-water bound.

The monitor detached through its normal console cleanup path, with
`console_stop_completed=true`. Its Windows Ctrl-Break exit code is expected;
the capture and explicit identity/UTC/recording assertions passed. The normal
image remains running and recording. Raw evidence is in
`evidence/20261126-gnss-auto/m10-auto-01.{log,json}` and
`m10-auto-01-summary.json`.

This run qualifies M10 detection at a retained 38400 baud, profile selection,
fix/PPS/time admission and two recording rotations. Factory-default baud
discovery, completed initial calibration and subsequent backup/silence/wake
cycles remain to be exercised physically. M7/M8 were not exercised during this
M10 run; subsequent M8 results follow below.

## Same-image M8 test: original image failed verification

`m8-auto-01` verified and booted exactly the M10-tested ELF (SHA-256 above),
without rebuilding. The same Evo UID and SD serial were present. It detected
M8 at 38400 baud at 0.641100 seconds; MON-VER reported software `2.01 (75331)`,
hardware `00080000`, extensions `PROTVER 15.00` and
`GPS;SBAS;GLO;BDS;QZSS`. Hardware identification selected M8 correctly despite
the older extension format. Startup then panicked at 5.934022 seconds with
`GNSS UART profile readback timeout`; no audio recording started.

A read-only SRAM capture after the fault retained nine checksum-valid returned
CFG-PRT frames with UART1, baud 38400, UBX+NMEA masks `0x0003`, and mode
`0x000008c0`. The firmware had requested `0x000008d0` and compared the entire
mode word. The differing bit 4 is reserved; the defined settings were correct.
RX counters showed 1810 bytes and zero rejected frames. This was a verifier
compatibility defect, rather than absent UART responses. The failed run and
decoded response evidence are retained as `m8-auto-01.{log,json}` and
`m8-auto-01-uart-readback.json` in the raw evidence directory.

## Revised common image (revision 2)

The legacy verifier now checks only the documented 8N1 fields, including both
valid no-parity encodings, plus UART1, baud and protocol masks. Identity parsing
also accepts the older space separator in `PROTVER 15.00`. The captured full
CFG-PRT response is a regression fixture; tests reject wrong character length,
parity, stop bits, baud and disabled protocols. M10 uses its existing VALGET
verification path. No U5 or shared driver source changed.

Local image: `target/l4-gnss-auto/aardwolf-l4-autodetect-v2.elf`.

SHA-256: `a9be6c6b101649fdd8ac075f05b86760b90948aa42387ab9c9a18160fe27632c`.

Flash sections total 269732 bytes; SRAM1 usage remains 96676 bytes. Exact
image/source hashes and memory sections are in `v2-build.json` and
`v2-memory.txt`. Normal L4 firmware built successfully; the updated L4/board,
WAV-mode, shared u-blox and U5 Aardwolf host checks passed. The original M10
result applies to the original hash; physical M10 reconfirmation on revision 2
is pending. Revision 2 is a common autodetect image, with no M8 build flag.

## M8 on revision 2: UART passed; timing/recording not yet qualified

`m8-auto-02` flashed and verified revision 2 in 138.91 seconds. Detection passed
at 0.641118 seconds and CFG-PRT readback passed at 0.762925 seconds. The software
UART wake returned the same `2.01 (75331)` / `00080000` identity and
`PROTVER 15.00`. The run remained in Searching with no accepted anchors or
audio files. CPU/syslog windows reported no dropped/truncated messages or log
write errors. Monitoring was stopped cleanly after 200 seconds of firmware
runtime to inspect received navigation data; its capture result is incomplete
(`pass=false`), rather than an application panic.

A read-only 2048-byte UART receive-buffer snapshot contained checksum-valid
NAV-PVT responses and matching RMC/GGA/GSA sentences. The receiver reported
the correct 2026 date, valid UTC flags (`0xf7`, required low bits set),
`fixType=2` and `numSV=3`. GSA likewise reported a 2D fix using two GPS satellites
and one GLONASS satellite. GSV reported usable signal levels around 23–29
dB-Hz. The unchanged qualification gate requires a 3D fix and four satellites,
so the transport correctly withheld this navigation state from recording/time
admission. The receiver returned a supported 92-byte NAV-PVT payload, which the
L4 adapter decoded successfully. Decoded fields are retained in
`m8-auto-02-navigation.json`.

PPS event counts also showed intermittent bursts above one edge per second,
alternating with intervals at one per second. With no qualified fix/accepted
anchors, this run does not qualify PPS timing. The user checked PPS/common
ground wiring and confirmed good sky view; the signal/antenna and extra edges
remain under investigation. No fix-validity thresholds or PPS checks were
relaxed. A further run (`m8-auto-03`) uses the exact same revision 2 ELF and
retains the receiver's navigation state while allowing more acquisition time.

`m8-auto-03` again passed M8 identification/profile verification on the identical
revision 2 ELF. No anchor or audio file was admitted during the search. It ended
with a capture-integrity assertion at 360.936516 seconds, near the user's
transition to another receiver, after a UART framing error at 357.845277
seconds. The capture guard was not bypassed. There is insufficient evidence to
attribute this terminal fault to the swap alone; excess capture edges had also
been observed earlier. M8 PPS/fix/recording qualification remains incomplete.

## V.KEL VK2828U7G5LF: detected as M7

The user supplied this replacement as an M6, then provided the V.KEL model.
`vkel-auto-01` verified and ran the same revision 2 ELF, with no additional
firmware changes. MON-VER identified actual hardware `00070000`, software
`1.00 (59842)`, extension `PROTVER 14.00`, and `GPS;SBAS;GLO;QZSS`: the M7
profile applies. This is not evidence of M6 support, and no M6-specific path
was added.

The first scan at 38400 produced recoverable framing errors. At 3.322957 seconds
the receiver was detected at 9600 baud. The unchanged image configured UART1
to 38400 and passed CFG-PRT readback at 3.474964 seconds. A subsequent UART wake
returned the same identity. This exercises discovery at a different initial
baud as well as runtime M7 selection. Cold acquisition/PPS/recording monitoring
is in progress; no success is inferred from identification alone.

## Further receiver swaps

For each device, retain the same frozen normal ELF. Power down before swapping
the receiver, keep the SD card inserted, and boot/reset Evo. Record:

1. `GNSS DETECT receiver=M7|M8|M10 baud=...`, software/hardware strings and all
   MON-VER extensions (including model/PROTVER when supplied).
2. `GNSS PROFILE PASS` at 38400 and the corresponding syslog identity/profile.
3. Real UTC/PPS acquisition and accepted time anchors, followed by two complete
   minute-long mono FLAC rotations with concurrent syslog and zero audio faults.
4. In a separate longer check, calibration, software backup with UART/PPS
   silence, UART wake and reacquisition. Keep family-specific power-cycle
   qualification pending until that receiver has actually completed it.

Future M7/M8/M10 device swaps require a reboot/reset and no rebuild or receiver
configuration selection. Direct MCU resets interrupt recording, so allow the
current minute file to close where practical. Do not remove or reformat the SD
card to perform these checks.
