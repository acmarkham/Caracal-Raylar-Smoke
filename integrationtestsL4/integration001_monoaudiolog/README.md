# Evo L4 mono WAV and syslog integration

Implements [ADR-L4-011](../../SpecAndDesign/Platform/L4/ADR-L4-011-mono-wav-and-syslog.md).
Records the verified PB12 microphone lane at nominal 16 kHz into one-minute WAV
files while appending heartbeat and diagnostic records to `/syslog.txt` on the
same exFAT SD card. Real GPS hardware is unused; simulated time is visibly
labelled `FAKE-GPS` in syslog and WAV metadata. Green stays off.

The normal build records continuously:

```text
rtk cargo build -p integration-test-l4-001-monoaudiolog --target thumbv7em-none-eabihf --release --locked
rtk probe-rs run --chip STM32L476RGTx --probe 0483:3748 --speed 100 --verify --preverify --non-interactive --no-catch-reset target/thumbv7em-none-eabihf/release/integration-test-l4-001-monoaudiolog
```

Blue is on during recording. Look for `L4 RECORD closed` once per minute and
progress every ten seconds. Each complete file contains 960000 mono samples,
3840000 payload bytes and a 512-byte header: **3840512 bytes total**. It uses
the unchanged upstream 32-bit integer PCM WAV convention, carrying sign-extended
24-bit microphone samples. No gain normalization or DC removal is applied.

Files use Storage Service paths `/<simulated-hour>/aud_<simulated-start>.wav`.
Startup chooses an unused simulated hour beginning at 2026-01-01, preserving
earlier WAVs. `/syslog.txt` is reopened for append. No card formatting or file
deletion occurs. The normal image continues until stopped, a name collision,
insufficient free space or an I/O failure. Interrupting power during recording
can leave the active segment unfinished; earlier closed files are retained.

For finite bench qualification, enable the three-minute stop:

```text
rtk cargo build -p integration-test-l4-001-monoaudiolog --target thumbv7em-none-eabihf --release --locked --features bench-three-minutes
rtk probe-rs run --chip STM32L476RGTx --probe 0483:3748 --speed 100 --verify --preverify --non-interactive --no-catch-reset target/thumbv7em-none-eabihf/release/integration-test-l4-001-monoaudiolog
```

Allow initial flashing, card setup, 2.048 seconds of microphone warm-up, three
minutes of audio and complete card readback. The bench opens no fourth WAV.
Only final `L4 RECORD PASS` means all three WAVs and the appended syslog suffix
were read back and matched their written hashes. The producer is then stopped,
both streams are closed and blue blinks at 1 Hz. Reset chooses a new simulated
hour and repeats the test. Do not halt or attach for debug-memory inspection
while recording; CPU pauses can cause a real DMA overwrite.

The L4 profile uses 256 ms AudioSource retention, one-sector partial buffers for
two Storage Service slots and opt-in 2 KiB filesystem staging. U5 retains its
existing 64 KiB staging/default features. Build the platforms separately;
do not use a combined workspace build.

```text
rtk cargo test -p integration-test-l4-001-monoaudiolog --no-default-features --lib --target x86_64-pc-windows-msvc --locked
rtk cargo test --manifest-path third_party/exfat-slim/Cargo.toml --lib --target x86_64-pc-windows-msvc --locked --features small-write-batches
```

[Evidence](evidence/README.md) records the actual images tested, file locations,
loss counters, memory/stack measurements and U5 regression comparisons.
