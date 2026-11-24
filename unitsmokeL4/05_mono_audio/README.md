# Evo L4 mono audio

Implements [ADR-L4-010](../../SpecAndDesign/Platform/L4/ADR-L4-010-dfsdm-mono-audio.md):
16 kHz mono PDM acquisition through DFSDM filter 0 and circular DMA1 channel 4.
PC2 AF6 supplies a nominal 3.2 MHz clock; PB12 AF6 supplies channel 1 data on
the rising edge. Only one phase of the M1/M5 lane is captured.

```text
rtk cargo build -p unit-smoke-l4-05-mono-audio --target thumbv7em-none-eabihf --release --locked
rtk probe-rs run --chip STM32L476RGTx --probe 0483:3748 --speed 100 --verify --preverify --non-interactive --no-catch-reset target/thumbv7em-none-eabihf/release/unit-smoke-l4-05-mono-audio
```

Allow about 63 seconds after boot: 100 ms initial settling, 2.048 s warm-up
and sixty seconds of qualified audio. Final `L4 AUDIO PASS` means all target
checks passed. Acquisition then stops and blue blinks at 1 Hz. Reset repeats
the test. Red and green remain off. SD and GPS are unused.

Use `--no-catch-reset` as shown: probe-rs otherwise enables reset vector catch,
which can remain set if the monitor is terminated and leave a later reset halted
before Rust initializes RAM. A successful reset command alone does not prove a
fresh run. The evidence includes a new completed capture after reset without
an RTT monitor. Avoid debug-memory attachment or halting during acquisition:
DFSDM/DMA can continue while the CPU is stopped, triggering an overwrite.

DMA retains full 32-bit register words; signed PCM occupies bits 31:8. Arithmetic
right shift by eight gives signed 24-bit samples in `i32`, as used by the U5
AudioSource producer. Every DMA block is passed through the unchanged portable
AudioSource and checked by a reader. The format has 24 bits of precision; this
does not claim 24 effective acoustic bits. Raw microphone DC is preserved.

The L476's RDATACH status bits are unimplemented (ES0250 2.12.1). Their observed
zero value is expected. Fixed RCH=1 readback selects the channel; pending,
reserved, overrun, clock absence and DMA error checks remain active.

After PASS, close the RTT session before exporting RAM. Read `snapshot` and
`meta` addresses from that run's log; they can change after a rebuild:

```text
rtk proxy python unitsmokeL4/05_mono_audio/export_audio.py --audio-address 0x200010f8 --meta-address 0x200090f8
```

The Python standard-library tool reads the completed snapshot without resetting
the board, verifies its firmware hash and writes `evidence/snapshot-raw.bin`,
`snapshot-pcm24.wav` and `snapshot.json`. The WAV is 0.512 seconds, mono, 16 kHz,
packed 24-bit PCM, with original amplitude/DC and no normalization. It is a host
file; the SD card is untouched. Wait for PASS again if the board was reset.

```text
rtk cargo test -p unit-smoke-l4-05-mono-audio --no-default-features --lib --target x86_64-pc-windows-msvc --locked
```

See [evidence](evidence/README.md) for target results and U5 regression coverage.
Keep L4 and U5 builds in separate Cargo invocations; root defaults stay U5.
