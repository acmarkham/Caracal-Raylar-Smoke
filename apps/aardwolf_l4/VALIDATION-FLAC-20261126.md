# Evo mono FLAC validation and CPU comparison

Date: 2026-11-26. [Decision](../../SpecAndDesign/Platform/L4/ADR-L4-014-mono-flac-and-cpu.md).
The clean baseline for this change is `dd935e83ad9d` (full hash in local
`target/l4-flac/baseline.json`). Raw logs and build records are retained under
`apps/aardwolf_l4/evidence/20261126-flac/`; exact local images are under
`target/l4-flac/`. These directories follow the repository's existing ignore
rules. Earlier evidence is preserved.

The measured WAV reference is the preceding
[normal-image validation](VALIDATION-20261126.md): 15.4% executor load during a
complete recording window, 80 MHz L476, mono 16 kHz, real GPS and concurrent
syslog. Do not compare the FLAC image's idle GPS-search windows to that value.

The new profile uses the unchanged upstream encoder's fixed order-1 prediction,
1024-sample blocks and 24-bit signed input. PDM clocks, DFSDM filtering and
production GPS settings are unchanged. U5 keeps its own profile and code.

Initial build and portable checks passed: nineteen L4/board tests, eight L4
tests in WAV reference mode, and both explicit firmware profiles compile.
All 61 original U5 firmware packages build, all 66 flash images and eleven
feature trees match the clean baseline, and 212 earlier L4 files are preserved.
The portable baseline remains 203 passes and the same five existing failures,
with no changed expectations (`regression.json`).

## Development observations

`flac-01` acquired real UTC at 127.771 seconds, after a GPS search. Recording
started at 156.664 seconds. The image stopped on a latched audio fault after
about 27 seconds of capture. The CPU record at 180 seconds includes substantial
pre-recording idle time and is **not** a valid steady recording comparison.
The failed run and original ELF are preserved. The next diagnostic build adds
the exact DMA error and maximum executor-poll duration before considering a
buffer or scheduling change; it retains the loss-detection checks.

`flac-02` completed two recording minutes with zero DMA/logging errors. Its
steady CPU window was 28.4%. The exported 1024-sample block passed Xiph CRC
testing and both Xiph/FFmpeg decoded PCM matched hardware checksum `8657bd78`.
Readback then stopped on a harness assumption that a 1024-byte read would return
all 1024 bytes. The filesystem correctly returned a short read. The verifier
now accumulates the header across reads. This failure was after acquisition
and both writers had stopped; it was not a recording fault.

## Accepted finite bench

`flac-03` passed at firmware time 182.154 s with real GPS/PPS, concurrent syslog,
and no generated tone. Its exact ELF SHA-256 is
`53ecfc12a86d1cabdf9aea415024ea7202f22f637b5f6f98aa1894d579fd4cb4`.
The loaded image is 268304 bytes. After all writers and DMA stopped, fresh
filesystem readback checked every file byte using the normalized content and
final STREAMINFO checksums described in the ADR.

| Check | Result |
| --- | --- |
| First file | `/1795683600/aud_1795687080.flac`, 1238637 bytes, 960000 samples; normalized hash `7e133b95`, STREAMINFO hash `ac3acc1c` |
| Second file | `/1795683600/aud_1795687140.flac`, 1238549 bytes, 960000 samples; normalized hash `e24cdbb8`, STREAMINFO hash `2ddd724d` |
| Format | Mono, 16000 Hz, 24 bits, 1024-sample blocks, fixed predictor order 1 |
| Syslog | 31811 appended bytes after 1252545 existing bytes; hash `589896d3`, 245 consecutive records, 176 heartbeats, two closed markers and CPU records |
| Recording CPU | 28.2% executor load over a complete active 60-second window |
| Frame wall time | Maximum 8965 microseconds; includes analysis yields; frame budget 64000 microseconds |
| Executor poll | Maximum 7282 microseconds during capture; DMA half interval remains 32000 microseconds |
| Errors | Zero DMA errors, dropped audio samples, log drops, truncations or write failures |
| Backlog / append | 1024 of 4096 samples; maximum append including lock wait 17295 microseconds |
| Syslog concurrency | Two simultaneous streams; seventeen checkpoints while audio was open |
| Stack | High-water bound 28884/32768 bytes; 3884 unused, including intact 1 KiB guard |

The 1979-byte first-block fixture also passed Xiph's frame CRC test and decoded
identically with Xiph FLAC 1.5.0 and FFmpeg 7.1.1. Both matched the producer's
original signed PCM checksum `2fc4e7ba` across 1024 samples. Only STREAMINFO
counts/sizes were adjusted to finalize this standalone fixture; encoded frame
bytes were unchanged. The absent optional MD5 produces the expected decoder
warning. See `flac-03.decode.json` and `tools/verify_flac_fixture.py`.

| Mode | Steady executor load | Bytes per minute |
| --- | ---: | ---: |
| Previous PCM32 WAV reference | 15.4% | 3840512 |
| FLAC bench | 28.2% | about 1238600 |

This is an observed increase of 12.8 percentage points (about 1.8x load), with
about 68% less SD space for this ambient recording. Compression varies with
the signal. The WAV reference is from the preceding run on the same board,
clocks, filtering, real GPS and logging configuration, not simultaneous replay
of identical PCM. The added poll timing hooks have a small instrumentation
cost. Both numbers use executor-idle accounting and exclude IRQ attribution.

The initial `flac-01` DMA fault has not recurred in the subsequent recording
benchmarks; its exact cause was not captured by that first build. It is retained
as an unresolved development observation, not silently reclassified as a pass.
The precise DMA error/poll diagnostics and loss checks remain enabled. These
bounded checks do not establish unattended endurance.

## Normal image

The normal image's SHA-256 is
`2193cc2661eb3c7a9e4aa6c36edf0af8b085d08a2f792ac58b3e5c0c6e820785`.
It occupies 267440 flash bytes and 96676 SRAM1 bytes, leaving 1628 of 98304
physical SRAM1 bytes. The CPU stack remains in the separate 32768-byte SRAM2
bank. The heap reservation and codec workspaces are included in those totals;
the encoder itself does not allocate.

`continuous-flac-01` passed two normal-mode rotations with real GPS and
concurrent logging:

- `/1795687200/aud_1795687440.flac`: 1068205 bytes, 960000 samples,
  normalized content hash `b776db52`, closed at 87.260 s.
- `/1795687200/aud_1795687500.flac`: 1077369 bytes, 960000 samples,
  normalized content hash `17ef2a1e`, closed at 147.247 s.

The complete active CPU window was **28.3% executor load**, compared with the
previous **15.4% WAV** reference: **12.9 percentage points higher**, about 1.84x
load. These two normal files average **1072787 bytes/minute**, about **72% less
storage** than the 3840512-byte PCM32 WAV minute. Maximum frame wall time was
8515 microseconds; maximum executor poll was 11696 microseconds. No DMA errors,
sample drops, log drops, truncations or write failures were reported.

The monitor disconnected after the third file opened, leaving the normal
**mono 16 kHz / 24-bit FLAC image recording**. Normal files were observed through
closure; full fresh-mount readback belongs to the stopped finite bench. No
target memory inspection or reset followed the continuous check.

Across `flac-02`, `flac-03` and the normal check, six complete recording minutes
have now run without repeating the first DMA fault. The earlier unresolved
observation and exact diagnostics remain documented above; this is not an
unattended endurance qualification.

Final build/source identity, rustfmt, link and archive checks are retained in
the local evidence directory. `regression-final.json` confirms the 66 unchanged
U5 images, eleven unchanged feature trees and 212 preserved earlier L4 files
after the target work. The accepted ELF/BIN images, source overlay, fixture and
validation evidence are preserved in local `firmware.zip` with its SHA-256.
