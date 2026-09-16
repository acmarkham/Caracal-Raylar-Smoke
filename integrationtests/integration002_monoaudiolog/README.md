# Integration test 002: mono audio logging

This firmware implements
[`integrationtest002.md`](../../ADR/prompts/tests/integration/integrationtest002.md).
It runs the time, storage, logging, power-management, and audio services against
the Raylar v1.0 board drivers. Audio is captured as mono 16 kHz signed 32-bit
PCM and rotated into 60-second WAV files in hourly directories.

## Running it

The default build requires a GPS PPS-correlated UTC anchor before it creates an
audio file. NMEA arrival timestamps are never used as time anchors:

```powershell
rtk powershell -NoProfile -ExecutionPolicy Bypass -File scripts/firmware.ps1 `
  -Package integration-test-002-monoaudiolog -MonitorSeconds 180 `
  -QuietTargetOutput -ProbeArgs --preverify
```

For indoor audio development, enable the explicit test-only time source:

```powershell
rtk cargo build -p integration-test-002-monoaudiolog --release --features fake-gps-time
rtk powershell -NoProfile -ExecutionPolicy Bypass -File scripts/firmware.ps1 `
  -Package integration-test-002-monoaudiolog -NoBuild -MonitorSeconds 135 `
  -QuietTargetOutput -ProbeArgs --preverify
```

`fake-gps-time` leaves the GPS hardware disabled and injects one `Laboratory`
time anchor. It is not a default feature, and both RTT and the system log label
the source as synthetic. The build time supplies the default fake epoch. Set
`INTEGRATION002_FAKE_UTC_SECONDS` before rebuilding when a deterministic or
fresh epoch is needed; using a fresh value avoids reopening a same-named file
after repeated runs of an unchanged binary.

After a run, produce a read-only GPT-ready card report with:

```powershell
rtk powershell -NoProfile -ExecutionPolicy Bypass -File scripts/sd-inspect.ps1
```

The report is written to `.probe-rs-logs/sd-card-report.md`; raw RTT logs for
each flash are retained under `.probe-rs-logs/`.

## Implementation notes

- The microphone uses the high-performance SINC5 preset with a 96 MHz PLL3_Q
  kernel clock. Each DMA half is 1,600 samples (100 ms, 6,400 bytes).
- Mono mode owns only MDF filter 0, its two pins, and GPDMA channel 0. The five
  unused microphone filters and DMA channels are not started.
- DMA completion is distributed as latest-state data through an
  `embassy_sync::watch`. The audio source retains eight seconds so filesystem
  latency during rotation does not lose samples.
- Before UTC is valid, DMA cadence and errors are still monitored, but samples
  are deliberately not inserted into the recorder ring. This prevents an
  expected GPS wait from appearing as an audio overrun.
- The first real GPS fix plays a short alternating success trill followed by a
  high resolving note once its first PPS anchor has been accepted. The
  synthetic-time feature does not play this signal.
- After the first fix, GPS remains continuously powered for ten minutes so the
  Time Service can calibrate its oscillator frequency from PPS. Only after this
  one-time calibration period does the normal 30-second on/30-second off duty
  cycle begin.
- PPS edges use TIM4 channel 4 hardware input capture at 1 MHz. After the first
  cross-clock epoch is established, edge timestamps are reconstructed from the
  capture counter rather than interrupt wake-up time. TIM4 is 16-bit, so its
  multiple wraps between 1 Hz edges are resolved using coarse monotonic elapsed
  time while the captured sub-wrap phase retains 1 us resolution.
- Oscillator calibration uses an allocation-free 11-point, ten-minute
  Theil-Sen regression over minute-spaced PPS samples. Pairwise slopes outside
  100 ppm are discarded, so an isolated timing or UTC-label outlier cannot
  dominate the calibration.
- A PPS label approximately one second from the current mapping is corrected to
  the adjacent UTC second when that leaves a residual within 100 ms; larger
  discontinuities are rejected. Accepted phase error is removed by a bounded
  60-second rate slew, without stepping the existing UTC mapping.
- Published uncertainty includes the full latest PPS residual plus capture
  uncertainty, then grows according to the holdover stability bound while GPS
  is off.
- The ten-second Time records include first/current anchor source, latest PPS
  residual, calibrated and slew frequency components, accepted/rejected anchor
  counts, UTC-second corrections, uncertainty, and holdover duration. Separate
  GPS records expose `Searching`, `Calibrating`, and `Reacquiring` state, PPS
  capture backend, and calibration/search/reacquisition counters.
- Severe failures that make recording unsafe or impossible—including missing
  SD media, card/filesystem initialization failures, stream write failures,
  and unrecoverable recorder failures—latch recording off. Both red LEDs then
  flash together at 1 Hz and a three-note descending alarm repeats every ten
  seconds. Green heartbeats stop until the board is reset.
- `MDF_DFLTISR.DOVRF` (bit 1) is the data-overrun flag. The observed sticky
  `CKABF` bit (bit 10) is reported separately as `clock_absent`; it is not a DMA
  or data overrun.
- SDMMC and shared-storage owners use static cells. Required audio buffers,
  service queues, and DMA buffers are statically bounded. The exFAT library
  itself still uses the configured fixed 64 KiB embedded heap for filesystem
  metadata operations; application steady-state buffers do not allocate.
- Log records are drained and checkpointed every ten seconds even while
  waiting for UTC. A checkpoint commits all complete 512-byte sectors without
  closing and reopening the file; at most the final 511 bytes remain buffered
  in RAM. Startup still performs a full flush so the initial record is durable.
  Queue drops or storage write failures latch the red system LED.

## Hardware validation (2026-09-14)

The fake-time build was flashed and observed through one complete rotation:

| Check | Result |
| --- | --- |
| Time gate | Recording started with `source=Laboratory`; GPS ignored |
| Capture rate | 16,000 Hz measured in 100 ms DMA halves |
| DMA overrun | `DOVRF=false` through IRQ count 871 |
| DMA errors/gaps | None observed |
| Audio drops | None observed |
| Logging | 825 records, 0 dropped, 0 truncated, 0 write failures |
| CPU while recording | Approximately 21–26% |
| Rotation | Completed at 60 seconds of PCM |
| Final WAV | 3,840,512 bytes (512-byte header + 3,840,000 PCM bytes) |
| WAV format fields | PCM, mono, 16,000 Hz, 64,000 byte/s, 32 bit |

The read-only card report found the finalized file at
`/1789380000/aud_1789380248_2.wav`. The following zero-length file is the open
successor interrupted when probe-rs reset the board to flash the inspector; it
is expected for this forced test termination, not a rotation failure.
