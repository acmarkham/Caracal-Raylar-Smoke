# Aardwolf on Caracal Evo L4

Explicit STM32L476RGT6 application composition using the upstream Aardwolf
policy and shared services. See [ADR-L4-012](../../SpecAndDesign/Platform/L4/ADR-L4-012-aardwolf-evo-port.md)
and [ADR-L4-016](../../SpecAndDesign/Platform/L4/ADR-L4-016-four-channel-flac-crcl.md).
Validation records: [initial port](evidence/README.md), [fault UI and WAV](VALIDATION-20261126.md),
[mono FLAC](VALIDATION-FLAC-20261126.md), [four-channel FLAC/CRCL and CPU](VALIDATION-QUAD-20261126.md).

The normal image captures **four synchronous 16 kHz DFSDM channels**. Lane 0
records lossless mono **24-bit FLAC**, and lanes 1–3 record in the paired **CRCL
v2** container using U5's 10-bit temporal Rice / minimum-shift-6 profile. The
three side channels are lossy. Both files rotate together each minute, alongside
concurrent syslog. It waits for real GPS UTC, uses captured PPS for the shared
Time Service calibration and continues through GPS standby/holdover. No radio,
inertial sensors, battery services or button.

DFSDM uses Sinc4/FOSR200/IOSR1 at 3.2 MHz, right shift 8. Input is signed
24-bit PCM right-justified in `i32`. Logical lane order is PB12/CH1, PC7/CH3,
PB6/CH5, PB10/CH7 (schematic microphone pairs M1/M5, M2/M6, M4/M8, M3/M7).
All lanes use the rising edge; physical microphone positions need acoustic
calibration. Filters 1–3 synchronize to filter 0. DMA1_CH4..CH7 drain on a
priority Embassy executor using the free TIM6_DAC vector. Four DMA rings have
128-frame halves (8 ms); AudioSource retains 5120 scalars / 80 ms.

The continuous `local-time` laboratory profile uses larger 256-frame DMA halves
(16 ms) and 6144 retained scalars (96 ms), using RAM otherwise reserved for GPS.
The fixture-export bench retains the smaller buffers to fit its export workspace.
SDMMC and SD DMA interrupts run below the audio executor. See the
[runtime fault investigation](VALIDATION-FAULTS-20261127.md) for the extended test.

The four-channel recorder now tolerates source-buffer overruns and recoverable
DMA ring overwrites. Missing whole frames become zero PCM in both the mono FLAC
and all CRCL channels, preserving file durations and channel alignment. Syslog
records substituted frame ranges and recovery counters; the paired reference
FLAC metadata marks the silence policy. U5 retains its strict default policy. See
[ADR-L4-017](../../SpecAndDesign/Platform/L4/ADR-L4-017-overrun-silence-recovery.md)
and the [glitch validation](VALIDATION-GAPS-20261127.md).

The reference uses fixed order-1 prediction and 256-sample FLAC blocks; CRCL uses
256-frame blocks. Static workspaces fit the L476's 96 KiB SRAM1, with the separate
32 KiB SRAM2 stack and existing guard. `mono` retains the previous 1024-sample
mono FLAC reference profile. `wav` implies `mono` and retains PCM32 WAV with
the 24 significant bits left aligned and the low byte zero.

Build each chip separately from the workspace root:

```powershell
rtk cargo build -p aardwolf --release --target thumbv8m.main-none-eabihf --locked --offline
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --locked --offline
rtk proxy probe-rs run --chip STM32L476RGTx --probe 0483:3748 --speed 100 --verify --preverify --non-interactive --no-catch-reset --no-catch-hardfault target/thumbv7em-none-eabihf/release/aardwolf-l4
```

Do not build the entire mixed-chip workspace at once. The root default target
and package selection remain U5.

One image supports u-blox M7/M8/M10 through USART2 PA2/PA3 and PB5 PPS.
At each boot it sweeps 38400, 9600, 57600, 115200, 19200, 4800 and 230400 baud,
wakes and polls MON-VER, and detects the generation from hardware/PROTVER.
Two bounded sweeps accommodate a receiver still booting. Unsupported identities
or failed profile readback enter the existing red LED/buzzer fault path.
M7/M8 use CFG-PRT and the legacy 8-byte RXM-PMREQ payload; M10 uses
VALSET/VALGET and the 16-byte RXM-PMREQ payload. Both verify UART operation at
38400 with UBX/NMEA enabled. M7/M8 UART settings are volatile; M10 retains its
UART profile in RAM/BBR. Neither writes receiver flash or resets navigation.
PB4 stays input. Software standby retains the supply and UART wakes the receiver. The
default GPS policy is 60 seconds minimum tracking / 1800 seconds standby after
the full initial calibration; clean-PPS phase qualification may extend tracking
to 180 seconds. Current consumption is not inferred from UART silence.

Files use exclusive `/hour/aud_<utc>.flac` and `.CRCL` creation. `/syslog.txt` is appended.
Nothing is formatted or deleted. Existing-file collisions, insufficient reserve
or hard I/O/peripheral errors stop the run with diagnostics. Blue indicates an open
audio file; green indicates search or an admitted GPS anchor. Red flashes on a
latched error, accompanied by a descending buzzer melody every 10 seconds.
PB14/TIM8_CH2N also plays the policy startup melody. PC6 card detection runs
continuously; removal or an unrecoverable storage/acquisition error stops recording and writes
until reset. Reinsertion alone does not resume an interrupted filesystem session.
An unplugged card cannot receive its own fault log; inspect RTT and the indicators.
The interrupted file and filesystem metadata are not guaranteed complete after
physical removal. Nothing attempts to format, repair or automatically remount it.

Syslog includes a heartbeat each second, GPS/time/location status every 10 seconds,
and CPU executor load plus logging health and I/O maxima every 60 seconds. CPU
measurement counts the union of both executor polling intervals without overlap;
it does not profile bare IRQ cycles before executor entry. This extends U5's
executor-idle method to the priority capture executor. Queue drops/truncation are counted and reported without stopping capture.
Keep RTT attached for initial qualification; the L4
transport is nonblocking so losing the host does not block interrupts.

The short codec test records two complete minute pairs, closes all three streams,
stops DMA and checks full fresh-filesystem readback and CPU records. It exports
real four-lane PCM and encoded blocks for independent Xiph/FFmpeg/CRCL decoding.
Use the explicit laboratory clock to leave GPS out of this audio check:

```powershell
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --features bench-local-time --locked --offline
rtk proxy python apps/aardwolf_l4/tools/capture.py quad --evidence-dir apps/aardwolf_l4/evidence/20261126-quad
rtk proxy python apps/aardwolf_l4/tools/verify_quad_fixture.py apps/aardwolf_l4/evidence/20261126-quad/quad.log --flac target/flac-tools/flac-1.5.0-win/Win64/flac.exe
```

`local-time` alone records continuously with GPS tasks disabled; `bench-local-time`
also selects the finite `bench-codec` test. Syslog and file metadata explicitly
mark `LOCAL-TIME-BENCH`, with unknown UTC and boot-specific synthetic labels.
These labels are not present-day UTC. The default image retains real GPS.
Use `bench-codec` without `local-time` for the finite real-GPS check.

`bench-glitches` selects the finite fake-GPS bench and injects a 100 ms DMA
consumer pause plus a 1500 ms audio append delay across the first minute
boundary. It requires recovery, logs the substituted ranges, and performs
fresh-mount file/syslog readback. Build with `local-time` alone afterwards to
leave ordinary continuous recording without fault injection.

Use `--features mono` for the previous mono FLAC reference, `wav` for PCM32 WAV,
or `wav,bench-codec` for its finite check. Compare only complete CPU windows during recording;
GPS search and waiting for the first minute are idle/preparation time.

The finite hardware qualification build shortens only GPS standby to 30 seconds:

```powershell
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --features bench-gps-cycle --locked --offline
rtk proxy python apps/aardwolf_l4/tools/capture.py bench-03
```

It keeps the full estimator and post-wake tracking requirements. After a
successful sleep/wake/phase-qualified shutdown cycle it finishes the current
minute, closes all streams and reads back every recorded byte. Allow roughly
15–20 minutes including flashing. The default image records continuously.
Do not halt or attach memory reads during DMA capture.

A shorter audio/UI qualification records two minutes with real GPS UTC and
inserts a two-second 1 kHz buzzer tone in the first file. It reopens and hashes
the two WAV segments and syslog, checks PCM packing, reports the acoustic spectrum and
requires CPU telemetry and zero logging/acquisition errors. The uncalibrated
buzzer-to-microphone coupling has no acoustic pass/fail threshold; listen to
the recordings to assess their sound:

```powershell
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --features wav,bench-audio-ui --locked --offline
rtk proxy python apps/aardwolf_l4/tools/capture.py audio-ui --evidence-dir apps/aardwolf_l4/evidence/20261126
```

The normal image has no acoustic test tone. This bench does not wait for the
full ten-minute frequency calibration or change the production GPS policy.

The capture helper saves a live RTT log and ELF hash. `--mode continuous`
checks two normal-image minute closures and then disconnects the monitor while
the normal firmware keeps recording. Build the selected image first; the helper
does not rebuild it. Frozen local build copies and their hashes are listed in
the evidence build records.
`--mode fault` waits for a live removal/error latch and observes a complete
ten-second audible repeat interval before releasing the probe.

Portable checks:

```powershell
rtk cargo test --lib -p caracal-evo -p aardwolf-l4 --no-default-features --target x86_64-pc-windows-msvc --locked --offline
rtk cargo test --lib -p raylar-drivers --no-default-features --features ublox-m10 --target x86_64-pc-windows-msvc gps::ublox --locked --offline
```

Receiver selection requires no build flag. Replace a GPS while the equipment is
powered down, then boot/reset Evo to rerun detection using the same image.
RTT shows `GNSS DETECT receiver=... baud=...`, raw version/model extensions and
`GNSS PROFILE PASS`; syslog records the selected family and discovered baud.
See [ADR-L4-015](../../SpecAndDesign/Platform/L4/ADR-L4-015-gnss-autodetect.md)
for the detection contract and device-swap checks.

The normal image also emits read-only acquisition diagnostics: legacy receiver
configuration after wake, satellite/navigation/RF status every 30 seconds, and
raw GSA/GSV/PVT in short windows before fix qualification. These logs leave the
shared fix/time gates unchanged. For a longer cold-start capture, add
`--timeout-seconds 1200` to `tools/capture.py`; default observation windows are
unchanged. See the [V.KEL acquisition report](VALIDATION-GNSS-ACQUISITION-20261126.md)
for measured TTFF, configuration readback and signal history.
