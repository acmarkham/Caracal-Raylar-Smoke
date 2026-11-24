# ADR-L4-010: Evo mono DFSDM audio through circular DMA

## Status

Accepted for implementation, 2026-11-24. Target:
[`unitsmokeL4/05_mono_audio`](../../../unitsmokeL4/05_mono_audio/README.md).
The [evidence record](../../../unitsmokeL4/05_mono_audio/evidence/README.md)
records actual target results and their limits.

## Context

The earlier smokes established the L476's clock, GPIO, SDMMC and GPS/time
interfaces separately. This step brings up one microphone without adding SD
recording or concurrent GPS. The U5 platform remains the upstream source of
truth. Its MDF implementation, service code, tests and default build composition
remain unchanged.

The Evo schematic shows eight ICS-41350 PDM microphones sharing PC2's clock.
PB12 carries the M1/M5 data pair on opposite clock phases. The active Mbed
development point uses DFSDM channel 1 for that lane. References are the
[pinned schematic](https://github.com/OpenWild/CaracalEvo_HW/blob/28ac586353e4258b828d69277ab215bf99090ef9/pcb/caracal_evo.sch)
and [legacy microphone setup](https://github.com/acmarkham/CARACAL_EVO_COMBRETUM/blob/5c655351dbd9f85ec1f119ef6680f6f78fa30a25/EvoC_007_Mpala/multi_audio.cpp).
These establish wiring and peripheral routes; the new implementation uses
Embassy Rust and the existing portable AudioSource.

## Decision

Add an isolated leaf with the L476 HAL feature selected only by its firmware
feature. Root `default-members`, dependency versions and existing feature
selections stay fixed. Build each chip in a separate Cargo invocation. Portable
sample interpretation and statistics have a native-testable library; the
firmware build script rejects the U5 target triple.

### Hardware and clock allocation

| Resource | Configuration |
| --- | --- |
| HSE / PLL | Existing 16 MHz oscillator; PLL M=2, N=20, R=2; 80 MHz system/AHB/APB clocks. |
| PC2 AF6 | DFSDM1_CKOUT; system clock divided by 25 gives nominal 3.2 MHz PDM. |
| PB12 AF6 | DFSDM1_DATIN1; channel 1, internal CKOUT, rising-edge sampling. |
| DFSDM filter 0 | Regular channel 1, fast continuous conversion, Sinc5, FOSR=50, IOSR=4, channel right shift=8, offset=0. |
| DMA1 channel 4 | Request selection 0; fixed FLT0RDATAR source, 32-bit source/destination, incrementing circular SRAM1 destination. |
| DMA storage | 1024 words / 4096 bytes, processed as alternating 512-sample halves every 32 ms. |
| TIM5 | Existing Embassy 1 MHz monotonic; no time discipline or GPS dependency. |
| RAM export | 8192 raw words / 32768 bytes, plus a twelve-word completion descriptor. |
| LEDs | Red and green off; blue blinks at 1 Hz after final PASS. |

The microphone's [electrical specification](https://invensense.tdk.com/wp-content/uploads/2016/02/DS-000047-ICS-41350-v1.1.pdf)
allows a 1.0–3.3 MHz clock in standard operation. This choice gives the exact
nominal integer ratio `80_000_000 / 25 / 50 / 4 = 16_000` samples/s. All eight
microphones receive the clock, but only one phase of one data lane is acquired.
The particular physical microphone on that phase needs individual acoustic
stimulation before assigning it a verified M-number.

Use the DFSDM register layout and DMA route from
[RM0351, chapter 24 and the DMA request table](https://www.st.com/resource/en/reference_manual/rm0351-stm32l47xxx-stm32l48xxx-stm32l49xxx-and-stm32l4axxx-advanced-armbased-32bit-mcus-stmicroelectronics.pdf).
The chosen filter gain is `50^5 * 4 = 1_250_000_000`: it fits the signed internal
32-bit path, and shifting by eight fits signed 24-bit output. This configuration
has its own gain and frequency response. It does not establish equivalence to
the U5 MDF filter response or microphone sensitivity calibration.

The pinned Embassy STM32 0.6.0/PAC exposes the DFSDM base address without a typed
L476 driver or ownership token. Keep volatile accesses inside `dfsdm.rs`, with
one private owner protected by an atomic singleton claim and owned GPIO tokens.
Use the existing Embassy DMA ring and interrupt handler. No HAL fork, PAC
regeneration or shared U5 driver change is needed for this smoke.

### Sample representation and service compatibility

DMA reads the full 32-bit data register. Signed 24-bit PCM occupies bits 31:8;
the bottom byte is peripheral status/reserved space. Convert with arithmetic
`(raw_word as i32) >> 8` and supply that right-justified signed value to the
unchanged `raylar-audiosource::AudioSource`. This matches the upstream Aardwolf
producer's PCM representation. Verify each produced block through an AudioSource
reader, including exact samples, sequence positions and zero drops/overruns.
Block timestamps use DMA completion in nominal microseconds, as in the current
non-hexaphonic U5 producer. First-sample epoch and UTC alignment are not qualified
by this smoke.

**Do not use RDATACH to identify the microphone on L476.**
[ES0250 Rev 13, section 2.12.1](https://www.st.com/resource/en/errata_sheet/es0250-stm32l471xx475xx476xx486xx-device-errata-stmicroelectronics.pdf)
states that these bits are unimplemented. The connected unit returns zero even
with RCH=1. Channel identity comes from fixed configuration and readback. Still
reject RPEND and nonzero reserved bits, and monitor the selected channel's clock
absence plus filter overrun flags. Clock-absence flags for unused inputs are
not failures of the acquired lane.

Twenty-four bits describes the data path and container precision. Neither low
bit variation nor this bench test establishes 24 effective acoustic bits.
Preserve microphone DC offset in firmware and exported PCM. Quiet audio can
stay entirely above zero; require meaningful variation and varying low PCM
bits rather than zero crossings. Select an export burst using approximate
block AC power so DC bias does not determine the retained event.

### Finite target qualification

Start the PDM clock, allow 100 ms for initial settling, arm DMA, then start
regular conversions. Discard 64 halves (2.048 s) from qualification statistics.
Process a further 1875 halves: exactly 960000 samples / sixty nominal seconds.
No heap, per-sample interrupt or per-sample formatting is required.

The DMA IRQ records half/full/error counts and timestamps, then delegates to
the exact Embassy handler. The task must receive each half within one second,
service it before the next 32 ms half deadline and observe the expected IRQ
count. Reject transfer errors, simultaneous outstanding half/full flags,
overwrites, filter overruns, selected-channel clock absence, clipped PCM and
stuck/truncated data. Sparse progress logs expose sample values and counters.

Compare first-to-last DMA completion timing against 62016000 microseconds,
allowing less than one millisecond difference. Both DFSDM and TIM5 derive from
the same HSE: this verifies the divider ratio and cadence, **not independent
absolute sample frequency**. GPS-calibrated audio rate and concurrent SD/GPS
load remain later integration work.

After 992768 total samples, stop the filter, microphone clock and DMA. Publish
the snapshot completion descriptor and FNV-1a hash, then final `L4 AUDIO PASS`.
Only then blink blue. Reset repeats the qualification. Stopping acquisition
allows stable RAM export through ST-Link without halting an active DMA producer.
This is a finite bring-up diagnostic, not a continuous recorder application.

Run the monitor with `--no-catch-reset`. Its default reset vector catch can
remain enabled after monitor termination, causing a later reset to halt before
startup clears RAM. For a standalone check, require evidence of a new capture
after the qualification interval; command success or an unchanged old ready
flag is insufficient. Do not halt or attach to inspect RAM while DMA is active:
debug attachment can stop the CPU while DFSDM/DMA continue and correctly cause
an overwrite failure. Check completion only after the finite capture ends.

Retain a contiguous 8192-word / 0.512 s burst after warm-up. The twelve little
endian u32 descriptor words are: `L4AU` magic, schema=1, sample rate=16000,
word count=8192, channels=1, valid bits=24, raw word bits=32, ready=1,
starting sample index low/high, raw-byte FNV-1a32, reserved=0. Log both addresses.
The host export tool checks readiness and the raw hash before writing a standard
packed 24-bit mono WAV. It preserves amplitude and DC; no SD access is involved.

### Evidence and regression boundary

Retain target RTT, raw RAM, WAV and host statistics, exact tested ELF/BIN,
lockfile, source/tool hashes, feature graph and linker memory layout. Native
checks cover signed PCM boundaries, arithmetic widths, exact rate/gain,
pending/reserved flags, low bit variation and DC-biased statistics.

Build all existing U5 firmware packages individually. Compare their saved ELF
hashes, relevant effective feature trees and baseline host results without
changing shared sources, tests or expectations. The connected L4 does not
provide U5 hardware coverage. Preserve prior L4 leaves and their features.

The L476RG has 96 KiB SRAM1 plus 32 KiB SRAM2 and 1 MiB flash. DMA, statics and
the ordinary stack reside in SRAM1; reserve at least 24 KiB between statics and
the initial stack pointer at link time. The export buffer is diagnostic memory
and can be removed in a later recording composition. This mono test does not
qualify four simultaneous filters, all eight microphones, codec throughput,
storage recording, UTC metadata, acoustic bandwidth, SNR or sensitivity.

## Policy overrides

The Audio Recorder, Storage, GPS and Time services are absent. The host WAV is
a lab export rather than a recorder segment, so their recording/scheduling
policies do not introduce extra firmware capabilities. Green remains off.

```yaml
policy_overrides:
  - policy: ui
    key: startup.enabled
    value: false
    reason: This diagnostic initializes no buzzer.
  - policy: ui
    key: errors.enabled
    value: false
    reason: Panic-probe and RTT report diagnostic failures; application fault annunciation is absent.
```
