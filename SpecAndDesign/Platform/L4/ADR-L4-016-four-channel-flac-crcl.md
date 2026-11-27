# ADR-L4-016: Four-channel DFSDM recording with FLAC and CRCL

Date: 2026-11-26. Status: implemented; target qualification is recorded in
[the validation record](../../../apps/aardwolf_l4/VALIDATION-QUAD-20261126.md).
This supersedes the normal mono recording profile in
[L4-014](ADR-L4-014-mono-flac-and-cpu.md).

## Decision

Capture four synchronous microphones at 16000 frames/s. Feed the same shared
paired recorder and CRCL/FLAC encoders that U5 Aardwolf uses. Lane 0 is the
lossless mono, signed 24-bit FLAC reference. Lanes 1–3 use U5's CRCL v2 temporal
shift profile: 10 retained bits, minimum shift 6, independent temporal Rice
coding. These three channels are **lossy**, with the existing bounded rounding
error. All four inputs retain the full signed DFSDM PCM value before encoding.

U5 retains six channels, its existing settings, entry points, tests and wire
output. Add const-generic `ChannelRecorder`, `ChannelEncoder` and
`ChannelTransform` implementations for four or six total channels. Existing
`HexaphonicRecorder`, `Encoder` and `Transform` are six-channel compatibility
aliases. Four-channel tests supplement the original suite. Compare the existing
six-channel reference/sidecar fixtures byte for byte against the pre-change
baseline, and build U5 separately from L4.

The shared FLAC encoder additionally accepts 256- and 512-sample blocks, using the
existing standard FLAC block code. L4 uses 256-sample FLAC blocks (16 ms),
256-frame CRCL blocks (16 ms), and caller-owned static workspaces. U5 block
sizes, reference precision and codec policy are unchanged.

## Hardware and acquisition

Retain the verified 16 MHz HSE / 80 MHz system clock and 3.2 MHz PDM clock,
Sinc4, FOSR 200, IOSR 1, channel right shift 8. Extract signed PCM as
`(RDATAR as i32) >> 8`; never include status bits or apply WAV's left alignment
to the 24-bit FLAC input.

| Recorded lane | DFSDM input/filter | Data pin, AF6 | DMA, request 0 | Schematic microphone pair |
| --- | --- | --- | --- | --- |
| 0, lossless reference | CH1 / FLT0 | PB12 | DMA1_CH4 | M1 / M5 |
| 1, CRCL channel 2 | CH3 / FLT1 | PC7 | DMA1_CH5 | M2 / M6 |
| 2, CRCL channel 3 | CH5 / FLT2 | PB6 | DMA1_CH6 | M4 / M8 |
| 3, CRCL channel 4 | CH7 / FLT3 | PB10 | DMA1_CH7 | M3 / M7 |

PC2 supplies the common clock. All lanes select the rising edge, matching the
verified mono path. The legacy logical MIC3/MIC4 labels do not follow schematic
pair numbering; preserve explicit pin/lane metadata. Electrical verification
does not establish physical microphone positions or the selected member of
each pair. Acoustic localization/calibration needs a separate stimulus check.

Configure and arm all DMA rings before starting conversions. Enable filters
1–3 with regular synchronization (`RSYNC`), then start only filter 0 in
continuous mode. Each lane has a 256-word circular ring and 128-word staging
buffer. A bank represents 8 ms. Interleave only corresponding, successfully
read banks into the shared AudioSource; retain 5120 scalar values, or 80 ms
across four channels. SRAM2 remains the 32 KiB CPU stack. DMA stays in SRAM1.

Drain DMA on a dedicated Embassy interrupt executor using the otherwise unused
TIM6_DAC vector at priority P1. DMA completion vectors retain HAL priority P0.
This allows capture to preempt thread-mode encoding and storage polls. No TIM6
hardware counter is used; TIM3/TIM2 PPS capture and TIM5 time remain available.
The producer performs bounded copy/statistics/publication only. It never encodes
or accesses the filesystem from interrupt context.

L4 explicitly selects AudioSource's `interrupt-safe` mutex feature. Its reader
callbacks execute in a bounded critical section so a priority producer cannot
alias an active borrowed PCM view. U5 retains the existing thread-mode mutex
and its unmasked callbacks. Notifications already use a critical-section mutex.
Compute time-model arithmetic outside the borrowed PCM callback to shorten
interrupt masking; the same recording cutoff and wire output are preserved.

Keep HAL pre/post-copy overrun checks, DMA transfer-error/coalesced-flag checks,
DFSDM status checks, channel bank counters, sample-clock checks, continuity
and retention limits. Stop on a real error; never reset a failed ring or use
`read_latest` to conceal missing audio. Keep separate master IRQ timestamps for
both retained banks so a queued bank cannot inherit its successor's timestamp.
TIM5 and DFSDM share the system PLL and have an exact 8000-local-microsecond
bank interval. Establish the sample epoch at the first completion, then advance
timestamps by sample count. Record IRQ latency/jitter separately; a bounded
critical section must not jitter the PCM timeline. An epoch discrepancy of a
whole bank remains a fault, alongside all hardware overwrite/error checks.

## File and metadata contract

Create `/<hour>/aud_<timestamp>.flac` and `.CRCL` for each common minute boundary,
with the existing segment identity, first source-frame index, device/boot
metadata and TIME anchors. A hardware RNG boot ID supplies pairing identity.
Use a 3 KiB filesystem heap reservation and retain two segment-history entries
(the GPS-cycle bench explicitly retains twenty). This funds the larger source
ring without consuming stack/DMA headroom. Append `/syslog.txt` concurrently,
using three stream handles under the same serialized StorageService/exFAT owner.

The CRCL header declares `[32,1,4,3]` at offsets 36..40 and channel-order TLV 6
contains four IDs. The source-width value 32 describes unpacked `i32` samples;
FLAC STREAMINFO declares the actual 24-bit reference precision. V2 DATA has
three descriptors and a 64-byte header. TIME and the 52-byte END retain the
existing layout. V1 is also generalized to three descriptors and a 68-byte
DATA header, without changing its coding or integrity rules. The desktop
decoder accepts both channel profiles and reverses FFmpeg's 24-bit alignment.

Before capture, admit the combined worst-case byte budget and warm the global
filesystem's free-space cache, then open both prepared handles. In particular,
do not leave a 256 GB allocation-bitmap scan until the first PCM arrives. On
rotation, finalize FLAC STREAMINFO, close the reference, write CRCL END and close
the sidecar before opening the next pair. Count a completed segment only when
the boundary END was written successfully. Partial cleanup must remain valid.

Preserve the 100 MB log reserve, allow for both companions' cluster rounding,
and use exclusive file creation. Do not assume a compression ratio, format the
card, overwrite old recordings, or delete failed bench artifacts. Existing
latched SD/audio errors retain the red LED and repeating buzzer indication.

## Qualification and CPU accounting

`bench-local-time` selects the finite two-minute `bench-codec` test with GPS
tasks disabled. `local-time` alone is its continuous laboratory counterpart.
Both explicitly mark syslog and recording metadata `LOCAL-TIME-BENCH`, use
`TimeSource::Laboratory` with unknown UTC, and assign boot-specific synthetic
time labels to avoid collisions when reflashing a cached ELF. These timestamps
are not measured UTC. The default build retains real GPS and its unchanged
acquisition, calibration, standby and recording gates.

The finite bench closes all streams, stops DMA, exports original four-lane PCM
and the first encoded blocks, then remounts read-only. Read back every byte of
both file pairs and the appended syslog; check final STREAMINFO, companion END
counts and running content checksums. Normalize only rewritten STREAMINFO in
the reference content checksum and verify that region separately.

Independently CRC-test and decode the real FLAC frame with Xiph and FFmpeg.
Verify that its reference samples match the original producer PCM exactly.
Decode the real CRCL DATA prefix and compare each side lane with the specified
rounding of its original PCM. A deliberately short prefix has no END and must
be reported incomplete; it is not substituted for full-file readback. Also
check that four varying, distinct inputs were acquired.

CPU diagnostics count the union of thread- and interrupt-executor polling
intervals, using nesting depth so preemption neither creates false idle time
nor counts work twice. Bare interrupt overhead before executor entry remains
unattributed, as in U5's original executor-idle method. Report complete active
60-second windows, error counts, maximum codec/poll/copy/append latency, source
and DMA backlog, logging health, and the SRAM2 stack guard. Waiting for capture,
flashing, stopped verification and fixture export are excluded from comparisons.

## Policy overrides

[ADR-L4-017](ADR-L4-017-overrun-silence-recovery.md) supersedes the original
fatal-on-overrun policy for L4: recoverable loss is replaced with aligned silence
in both files. Hard storage/peripheral failures still latch the error UI.

The [2026-11-27 runtime follow-up](../../../apps/aardwolf_l4/VALIDATION-FAULTS-20261127.md)
lowers SDMMC/SD-DMA interrupts to P2, below audio capture P1. Continuous local-time
operation uses 16 ms DMA banks and 96 ms source retention, spending the unused
GPS allocation on service margin. GPS-enabled and fixture-export builds retain
the compact allocation described above. Loss checks and error indication remain
mandatory in every profile.

Replace L4's mono-only default with four channels, 256-sample/24-bit mono FLAC
and three-channel CRCL v2 with 256-frame blocks. Keep `mono` and `wav` as explicit
reference modes. The optional laboratory clock is a marked test exception to
real GPS gating, never the default deployment clock. All other L4 capability,
GPS, UI, storage-reserve and failure-handling policies remain in effect.
