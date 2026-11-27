# Four-channel L4 DFSDM, FLAC/CRCL and CPU validation

Date: 2026-11-26. See
[ADR-L4-016](../../SpecAndDesign/Platform/L4/ADR-L4-016-four-channel-flac-crcl.md).
The source baseline is `0345601458aa`. Evidence, frozen ELF images and JSON
records are kept locally in `apps/aardwolf_l4/evidence/20261126-quad/`, following
the repository's existing evidence ignore rules.

The profile is four synchronous 16 kHz, signed 24-bit DFSDM inputs. Lane 0
is lossless mono FLAC, fixed predictor 1, 256-sample blocks. Lanes 1–3 are
CRCL v2, 256-frame blocks, 10 retained bits and minimum shift 6, matching U5's
coding policy with fewer channels. CRCL side channels are lossy. The electrical
lane order is PB12/CH1, PC7/CH3, PB6/CH5, PB10/CH7; all use the rising edge.
The SD card's serial is `804e6392`, with 262144-byte clusters. No formatting,
file deletion or overwrite was performed.

GPS is deliberately excluded from audio bench images. Metadata and syslog
say `LOCAL-TIME-BENCH`; file labels are boot-specific synthetic timestamps,
with unknown UTC. The default firmware still uses real GPS/PPS and its
unchanged qualification/calibration/power policy.

## Development observations

- `quad-01` caught the existing FLAC encoder's minimum block-size restriction
  at construction. The shared encoder now accepts standard 512-sample blocks;
  independent decoding verifies that path.
- `quad-02` refused an existing filename from a cached lab epoch, entering the
  storage fault path. Laboratory timestamps now include a hardware-random
  boot namespace. Earlier files were preserved.
- `quad-03` exhausted the 64 ms source retention during the first shared
  admission check. The global filesystem's free-space bitmap cache was cold.
  Admission/cache warming now occurs before capture, alongside preparation of
  both file handles.
- `quad-04` stopped at the original strict bank counter check near an 8 ms
  boundary, with a 7331 us executor poll. `quad-05` allowed a retained bank but
  still stopped on a HAL read failure, with a 7365 us poll. Thread-mode capture
  did not leave enough scheduling margin. The final architecture preempts
  encoding/storage with a dedicated priority capture executor; overwrite checks
  and loss/error detection remain active.
- `quad-06` exposed AudioSource's default Cortex-M thread-only mutex assertion
  from the new executor. L4 now explicitly opts into `interrupt-safe` access.
  U5 retains its original mutex and callbacks. PCM timestamps advance from the
  common PLL/sample clock; IRQ jitter is measured separately.
- `quad-07` captured and closed the first complete minute on all four channels
  with 960000 frames per lane. The rotation guard still used completed-recorder
  progress, counting already-copied codec PCM as unread. It now checks the
  actual AudioSource reader cursor. The 64 ms retention limit and all hardware
  overwrite/loss checks remain active.
- `quad-08` measured the actual rotation backlog at 4160 scalar samples, above
  the 4096-sample limit. The final profile retains 5120 scalars / 80 ms, uses
  256-sample reference blocks, a 3 KiB heap, and two history entries (twenty only
  for the GPS-cycle bench). This fits the physical SRAM budget and keeps the
  same strict retention/loss checks. The first 60-second CPU record includes
  pre-recording idle time and is not a steady-load result.

These failed runs are retained as development evidence, not accepted CPU or
recording measurements.

## Portable and U5 checks

All 61 original U5 firmware packages build separately for
`thumbv8m.main-none-eabihf`. The portable suite adds the four-channel
rotation/tail test: 204 passes and the same five pre-existing failures as the
203-pass baseline. Those failures are the four Time Service estimator cases
and GPS framer case listed in `regression.json`; no expectations were changed.
The original 24 paired six-channel fixture files remain byte-identical to the
pre-change golden files. Existing U5 application configuration and source are
unchanged; shared services gain the four-channel specialization.

The codec/paired-recorder tests and six independent desktop decoder tests pass,
including four-channel WAV/24-bit FLAC, v1/v2, rotation, short tails, original
reference samples, side-channel error bounds, truncation and invalid profiles.
L4/board portable checks and explicit mono/WAV reference-mode checks also pass.

U5 hardware was not connected.

## Accepted stopped-writer bench

`quad-09` passed at firmware time 135.653 s. Its frozen ELF SHA-256 is
`153c1bf724b846783adfc3227a9ad4243d43045dc4ed4b144b59b3ba9ad5b7d6`.
Acquisition ran from approximately 8.104 s through the second complete minute.
All three writers and all four DMA channels stopped before fixture export and
fresh read-only mounting. No target memory reads were made during acquisition.

| Check | Result |
| --- | --- |
| First pair | `/4540834800/aud_4540835760.flac` (1091268 bytes), `.CRCL` (805541 bytes); 960000 frames on each lane |
| Second pair | `/4540834800/aud_4540835820.flac` (1088968 bytes), `.CRCL` (796549 bytes); 960000 frames on each lane |
| Reference checksums | Normalized content `3e6ae625`, `0c16ac42`; final STREAMINFO separately checked |
| Sidecar checksums | Whole-file `872b17c0`, `7fb21089`; final END totals checked |
| Syslog | 23907 appended bytes, 183 consecutive records, two CPU records, twelve checkpoints during audio |
| CPU | **56.9%**, complete active window 60.363–120.363 s; both executors counted without overlap |
| Errors | Zero DMA errors, dropped PCM, log drops, log truncations or write failures |
| Source backlog | 3976 of 5120 scalar samples (about 62.1 of 80 ms) |
| Timing | Maximum codec block wall time 7700 us; copy/statistics/publication 1208 us; append 13567 us |
| Thread poll | Maximum 12879 us in the measured window; capture preempts thread polls |
| DMA / IRQ | At the first CPU report, seven unread DMA words maximum, IRQ jitter 305 us; PCM timestamps use the common sample clock |
| Stack | High-water upper bound 29316/32768 bytes after verification, 3452 unused; 1 KiB guard intact |

The 50.5% CPU value in the first window includes eight seconds before recording
and must not be used as the steady measurement. The earlier mono reference
measured 28.3% with real GPS, 1024-sample FLAC blocks and the same 80 MHz clock.
The four-channel bench therefore adds about 28.6 percentage points, roughly
twice that observed load. This compares two ambient runs and different channel/
buffer profiles, not identical replayed PCM; the new bench excludes GPS work.
Bare IRQ work before executor entry remains unattributed in both measurements.

The paired files average 1891163 bytes per minute in this ambient recording.
Compression depends on the signal. All four full files were read back and
hashed, including separate validation of rewritten reference STREAMINFO. CRCL
v2 itself has no payload CRC/digest; the readback checksums compare SD bytes
against the exact bytes submitted by the recorder.

The hardware fixture passed independent decoding (`quad-09.decode.json`):
Xiph checked FLAC frame CRCs, and both Xiph and FFmpeg decoded all 256 reference
samples exactly to the original captured PCM. The three CRCL lanes decoded
exactly to their prescribed rounded source values. Their shifts were 8, 8, 9;
observed maximum errors were 127, 128, 256 PCM counts, within respective bounds
128, 128, 256. All four original microphone lanes varied and were distinct.
The CRCL fixture intentionally contains one complete DATA record and no END;
the decoder correctly reports an incomplete prefix. It does not represent a
claim to independent decoding of every sample in both complete minute pairs.
Xiph's absent-optional-MD5 warning is expected; frame CRC testing passed.

## Continuous image

The continuous audio image uses `local-time`, with no finite-stop flag or test
tone. GPS is bypassed explicitly for this round. The normal default build
retains GPS and also compiles within physical RAM. Combined GPS/audio hardware
qualification and acoustic microphone-position calibration are separate checks.

`continuous-quad-01` passed two rotations and was left recording after the third
pair opened. Its flashed ELF SHA-256 is
`047378a4565bd7dea1caae36dae86588ecbc051f79142ca68fae7875e231eebc`.
The monitor released USB cleanly without a reset or memory inspection.

| Check | Result |
| --- | --- |
| First pair | `aud_729220127820.flac` (1086820 bytes), `.CRCL` (772771 bytes), 960000 frames/lane, closed at 97.160 s |
| Second pair | `aud_729220127880.flac` (1092286 bytes), `.CRCL` (804138 bytes), 960000 frames/lane, closed at 157.144 s |
| Running pair on disconnect | `aud_729220127940.flac` and `.CRCL`, opened at 157.160–157.173 s |
| CPU | **56.6%** during the complete active window 60.350–120.350 s |
| Errors | Zero reported DMA errors, PCM drops, logging drops/truncations/write errors |
| Backlog / I/O | 4188 of 5120 scalars maximum; append 51008 us, thread poll 51315 us |
| DMA / IRQ | Seven unread DMA words maximum at reporting; IRQ jitter 327 us, without sample-clock discontinuity |
| Codec | 7940 us maximum block wall time |
| Stack | Bound 28664/32768 bytes, 4104 unused, guard intact |
| Heap | Reporting snapshots show 0 used / 3072 free bytes; these are not peak-allocation measurements |

The first 24.5% CPU report includes 37 seconds waiting for the first minute;
the **56.6%** second report is the relevant comparison. Against the previous
28.3% mono result, the observed increase is about 28.3 percentage points,
roughly twice the load. Both accepted runs therefore place this profile near
**57% executor load** for this ambient signal at 80 MHz. The continuous pair
closures were observed; full fresh-mount readback belongs to the stopped bench.
These bounded checks do not establish unattended endurance.

| Build | Flash bytes | SRAM1 bytes / 98304 | SRAM1 free |
| --- | ---: | ---: | ---: |
| Accepted two-minute lab bench | 247664 | 94140 | 4164 |
| Flashed continuous lab image | 246244 | 84876 | 13428 |
| Default GPS-enabled four-channel image | 293648 | 97228 | 1076 |

SRAM2 is the separate 32768-byte CPU stack. The totals include the reserved
heap, DMA rings, source/codec workspaces, task storage and RTT buffer. L4 mono,
WAV and GPS-cycle bench variants also compile. The default artifact is restored
in the standard target directory; the device intentionally retains the tested
continuous laboratory image. Exact images and their build records are frozen
in the evidence directory. Emergency fault handling now stops all four audio
DMA channels and all filters before changing the clock for the error buzzer.
