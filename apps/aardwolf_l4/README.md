# Aardwolf on Caracal Evo L4

Explicit STM32L476RGT6 application composition using the upstream Aardwolf
policy and shared services. See [ADR-L4-012](../../SpecAndDesign/Platform/L4/ADR-L4-012-aardwolf-evo-port.md)
and [ADR-L4-014](../../SpecAndDesign/Platform/L4/ADR-L4-014-mono-flac-and-cpu.md).
Validation records: [initial port](evidence/README.md), [fault UI and WAV](VALIDATION-20261126.md),
[FLAC and CPU comparison](VALIDATION-FLAC-20261126.md).

The normal image records mono 16 kHz **24-bit FLAC** in one-minute files with concurrent
syslog on the existing exFAT SD card. It waits for real GPS UTC, uses captured
PPS for the unchanged Time Service calibration, and continues recording through
GPS standby/holdover. No radio, inertial sensors, battery services or button.
It uses the upstream fixed order-1 predictor, 1024-sample blocks and static
workspaces. DFSDM uses Sinc4/FOSR200/IOSR1 at 3.2 MHz. FLAC input is signed
24-bit PCM right-justified in `i32`; the `wav` reference feature instead writes
PCM32 with those 24 significant bits left aligned and the low byte zero.

Build each chip separately from the workspace root:

```powershell
rtk cargo build -p aardwolf --release --target thumbv8m.main-none-eabihf --locked --offline
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --locked --offline
rtk proxy probe-rs run --chip STM32L476RGTx --probe 0483:3748 --speed 100 --verify --preverify --non-interactive --no-catch-reset --no-catch-hardfault target/thumbv7em-none-eabihf/release/aardwolf-l4
```

Do not build the entire mixed-chip workspace at once. The root default target
and package selection remain U5.

The fitted Quescan G10A-F30 M10 uses USART2 PA2/PA3 at 38400 baud and PB5 PPS.
PB4 stays input; there is no EN/reset pulse. UART setup is retained in RAM/BBR,
with no GPS flash write. RXM-PMREQ requests software standby with the supply
retained; a UART transition wakes the receiver. No cold reset is sent. The
default GPS policy is 60 seconds minimum tracking / 1800 seconds standby after
the full initial calibration; clean-PPS phase qualification may extend tracking
to 180 seconds. Current consumption is not inferred from UART silence.

Files use exclusive `/hour/aud_<utc>.flac` creation. `/syslog.txt` is appended.
Nothing is formatted or deleted. Existing-file collisions, insufficient reserve
or I/O/acquisition errors stop the run with diagnostics. Blue indicates an open
audio file; green indicates search or an admitted GPS anchor. Red flashes on a
latched error, accompanied by a descending buzzer melody every 10 seconds.
PB14/TIM8_CH2N also plays the policy startup melody. PC6 card detection runs
continuously; removal or a storage/acquisition error stops recording and writes
until reset. Reinsertion alone does not resume an interrupted filesystem session.
An unplugged card cannot receive its own fault log; inspect RTT and the indicators.
The interrupted file and filesystem metadata are not guaranteed complete after
physical removal. Nothing attempts to format, repair or automatically remount it.

Syslog includes a heartbeat each second, GPS/time/location status every 10 seconds,
and CPU executor load plus logging health and I/O maxima every 60 seconds. CPU
measurement matches U5's executor-idle method; it does not profile individual IRQ
cycles. Queue drops/truncation are counted and reported without stopping capture.
Keep RTT attached for initial qualification; the L4
transport is nonblocking so losing the host does not block interrupts.

The short codec test records two complete minutes, closes both streams and
checks fresh-filesystem readback and CPU records. After DMA stops it also
exports a real encoded block for independent Xiph/FFmpeg decoding:

```powershell
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --features bench-codec --locked --offline
rtk proxy python apps/aardwolf_l4/tools/capture.py flac --evidence-dir apps/aardwolf_l4/evidence/20261126-flac
rtk proxy python apps/aardwolf_l4/tools/verify_flac_fixture.py apps/aardwolf_l4/evidence/20261126-flac/flac.log --flac target/flac-tools/flac-1.5.0-win/Win64/flac.exe
```

Use `--features wav` for the reference recording format, or `wav,bench-codec`
for its finite check. Compare only complete CPU windows during recording;
GPS search and waiting for the first minute are idle/preparation time.

The finite hardware qualification build shortens only GPS standby to 30 seconds:

```powershell
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --features bench-gps-cycle --locked --offline
rtk proxy python apps/aardwolf_l4/tools/capture.py bench-03
```

It keeps the full estimator and post-wake tracking requirements. After a
successful sleep/wake/phase-qualified shutdown cycle it finishes the current
minute, closes both streams and reads back every recorded byte. Allow roughly
15–20 minutes including flashing. The default image records continuously.
Do not halt or attach memory reads during DMA capture.

A shorter audio/UI qualification records two minutes with real GPS UTC and
inserts a two-second 1 kHz buzzer tone in the first file. It reopens and hashes
both WAVs and syslog, checks PCM packing, reports the acoustic spectrum and
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
