# Aardwolf on Caracal Evo L4

Explicit STM32L476RGT6 application composition using the upstream Aardwolf
policy and shared services. See [ADR-L4-012](../../SpecAndDesign/Platform/L4/ADR-L4-012-aardwolf-evo-port.md)
and the [validation record](evidence/README.md).

The normal image records mono 16 kHz WAV in one-minute files with concurrent
syslog on the existing exFAT SD card. It waits for real GPS UTC, uses captured
PPS for the unchanged Time Service calibration, and continues recording through
GPS standby/holdover. No radio, inertial sensors, battery services or button.
The WAV holds sign-extended 24-bit samples in the existing 32-bit PCM format.

Build each chip separately from the workspace root:

```powershell
rtk cargo build -p aardwolf --release --target thumbv8m.main-none-eabihf --locked --offline
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --locked --offline
rtk proxy probe-rs run --chip STM32L476RGTx --probe 0483:3748 --speed 100 --verify --preverify --non-interactive --no-catch-reset target/thumbv7em-none-eabihf/release/aardwolf-l4
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

Files use exclusive `/hour/aud_<utc>.wav` creation. `/syslog.txt` is appended.
Nothing is formatted or deleted. Existing-file collisions, insufficient reserve
or I/O/acquisition errors stop the run with diagnostics. Blue indicates an open
audio file; green indicates search or an admitted GPS anchor. Full fault UI and
buzzer remain deferred. Keep RTT attached for initial qualification; the L4
transport is nonblocking so losing the host does not block interrupts.

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

The capture helper saves a live RTT log and ELF hash. `--mode continuous`
checks two normal-image minute closures and then disconnects the monitor while
the normal firmware keeps recording. Build the selected image first; the helper
does not rebuild it. Frozen local build copies and their hashes are listed in
the evidence build records.

Portable checks:

```powershell
rtk cargo test --lib -p caracal-evo -p aardwolf-l4 --no-default-features --target x86_64-pc-windows-msvc --locked --offline
rtk cargo test --lib -p raylar-drivers --no-default-features --features ublox-m10 --target x86_64-pc-windows-msvc gps::ublox --locked --offline
```
