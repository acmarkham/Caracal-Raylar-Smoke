# Integration test 002: mono audio logging

This firmware implements
[`integrationtest002.md`](../../ADR/prompts/tests/integration/integrationtest002.md).
It runs the time, storage, logging, power-management, location, sensor,
versioning, and audio services against the Raylar v1.0 board drivers. Audio is
captured as mono 16 kHz signed 32-bit PCM and rotated into 60-second WAV files
in hourly directories.

Startup traceability is supplied by the Identity and Versioning Service rather
than direct calls to the low-level identity driver. Separate syslog records
capture device IDs, firmware/build metadata, board revision, SD-card identity,
and GPS/radio module identity and firmware fields. Sources that are not yet
wired are retained explicitly as `Unknown` or `Unavailable`.

SD-card identity is captured once from CID/CSD by the STM32 storage driver and
passed through the Storage Service to Versioning. The syslog snapshot includes
manufacturer/OEM/product identifiers, revision, serial number, manufacture
date, and physical capacity without integration code accessing SDMMC directly.

## Running it

The default build requires a GPS PPS-correlated UTC anchor before it creates an
audio file. It also selects the STM32U595's internal SMPS, matching the Q-package
Raylar v1.0 hardware and its fitted inductor. NMEA arrival timestamps are never
used as time anchors:

```powershell
rtk powershell -NoProfile -ExecutionPolicy Bypass -File scripts/firmware.ps1 `
  -Package integration-test-002-monoaudiolog -MonitorSeconds 180 `
  -QuietTargetOutput -ProbeArgs --preverify
```

For a diagnostic build that retains the reset-default LDO, disable default
features (and explicitly restore any other desired features):

```powershell
rtk cargo build -p integration-test-002-monoaudiolog --release --no-default-features
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

- Core-supply selection runs immediately after `embassy_stm32::init`, because
  Embassy resets the PWR block during MCU initialization. The default
  `core-smps` feature selects SMPS and waits for the hardware status to confirm
  the transition before any board peripherals are constructed.
- The microphone uses the high-performance SINC5 preset with a 96 MHz PLL3_Q
  kernel clock. Each DMA half is 1,600 samples (100 ms, 6,400 bytes).
- Mono mode owns only MDF filter 0, its two pins, and GPDMA channel 0. The five
  unused microphone filters and DMA channels are not started.
- Mono capture pins its circular GPDMA descriptor table for the lifetime of the
  task and publishes completed halves directly from the DMA buffer. The DMA
  interrupt signal replaces the former `read_exact` call that copied every
  1,600-sample half into an unused synchronization buffer.
- DMA completion is distributed as latest-state data through an
  `embassy_sync::watch`. The audio source retains eight seconds so filesystem
  latency during rotation does not lose samples.
- Audio packet timing is aggregated into a 1 Hz system-log record containing
  the first and last DMA timestamps and the packet/sample totals for that
  interval. A full diagnostic INFO queue increments a reported drop counter
  but does not stop recording; filesystem write failures remain fatal.
- RTT emits start, completion/failure, and elapsed-time markers while closing
  each WAV, opening its successor, and appending the new WAV header. These
  markers do not depend on the SD-backed system-log queue.
- RTT reports total CPU use once per second and an attributed CPU profile every
  five seconds. `mic_dma`, `audio_forward`, `audio_recorder`, and `logging` are
  disjoint executor work; `other` is the active time not covered by those
  points. `nested_audio_storage` and `nested_log_storage` are subsets of their
  callers and show whether filesystem/SD polling, rather than PCM conversion or
  log formatting, accounts for the time. Each entry is `%/calls/polls`.
- The async profile measures time inside each future's `poll` calls and excludes
  time returned as `Pending`. It therefore measures MCU work rather than
  charging the CPU for time spent asleep while DMA or SDMMC hardware runs.
- Before UTC is valid, DMA cadence and errors are still monitored, but samples
  are deliberately not inserted into the recorder ring. This prevents an
  expected GPS wait from appearing as an audio overrun.
- The first real GPS fix plays a short alternating success trill followed by a
  high resolving note once its first PPS anchor has been accepted. The
  synthetic-time feature does not play this signal.
- GPS fixes also feed the Location Service's nine-sample median filter. The
  first valid estimate (after three accepted fixes) is written immediately to
  the `Location` syslog component; the latest retained estimate is then logged
  every 60 seconds with coordinates in signed degrees times 10^7, fix age,
  source quality, and uncertainty metadata. Synthetic-time mode has no GPS
  location and therefore emits no location records.
- The battery charger, LIS2HH12, and LIS2MDL share the blocking sensor I2C bus
  through a single-executor adapter. Its transactions never yield and do not
  mask interrupts, so the audio DMA interrupt remains responsive.
- The Sensor Service polls raw acceleration and magnetic field every 10
  seconds, and the LIS2HH12 and LIS2MDL die temperatures every 30 seconds.
  Each new sample is written to the `Sensor` component of `/syslog.txt` with
  explicit fixed-point units and source identity. This integration deliberately
  registers no composite sensors or threshold rules.
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
  dominate the calibration. The result is locked after the eleventh sample;
  later duty-cycle reacquisitions cannot move the learned oscillator rate.
- A PPS label approximately one second from the current mapping is corrected to
  the adjacent UTC second when that leaves a residual within 100 ms; larger
  discontinuities are rejected. Accepted phase error is removed by a bounded
  60-second rate slew, without stepping the existing UTC mapping.
- Published uncertainty includes the full latest PPS residual plus capture
  uncertainty, then grows according to the holdover stability bound while GPS
  is off. After 1.5 seconds without PPS, the temporary phase slew is removed
  with a continuity-preserving rebase, so holdover runs only at the calibrated
  oscillator rate. The gap edge and the next three qualifying intervals are
  excluded; anchors resume only after three consecutive PPS intervals within
  50 ms of one second.
- The ten-second Time records include first/current anchor source, latest PPS
  residual, calibrated and slew frequency components, accepted/rejected anchor
  counts, UTC-second corrections, uncertainty, and holdover duration. Separate
  GPS records expose `Searching`, `Calibrating`, and `Reacquiring` state, PPS
  capture backend, and calibration/search/reacquisition counters.
- `Pps: EDGE` records preserve every raw PPS monotonic/capture stamp and
  interval. `GpsCorr: PAIR` records preserve every emitted NMEA/PPS pairing,
  including the PPS sequence number and unmatched NMEA records, so UTC can be
  reconstructed or corrected post-hoc. The streams are statically bounded and
  emit explicit `LOSS` records if a logger ever falls behind.
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
- exFAT stages up to eight aligned, physically contiguous sectors and submits
  them as one block-device write. This preserves cache coherence and bounded
  memory while avoiding a separate SDMMC command/readiness cycle per sector.
- Log records are drained and checkpointed every ten seconds even while
  waiting for UTC. A checkpoint commits all complete 512-byte sectors without
  closing and reopening the file; at most the final 511 bytes remain buffered
  in RAM. Startup still performs a full flush so the initial record is durable.
  Storage write failures latch the red system LED. Best-effort diagnostic INFO
  queue drops are counted and reported at the next checkpoint but are not
  fatal.

## Hardware validation (2026-09-17)

### Synthetic-time release variant

The fake-time build was flashed and observed through one complete rotation:

| Check | Result |
| --- | --- |
| Time gate | Recording started with `source=Laboratory`; GPS ignored |
| Capture rate | 16,000 Hz measured in 100 ms DMA halves |
| DMA overrun | `DOVRF=false` through IRQ count 601 at rotation |
| DMA errors/gaps | None observed |
| Audio drops | None observed |
| Logging | 100 records at last pre-rotation checkpoint; 0 dropped, 0 truncated, 0 write failures |
| CPU while recording | 7.7–8.5%; 8.0% mean across eleven steady five-second windows |
| Storage polling | 114–120 polls/5 s, down from 642 |
| Rotation | 60 seconds; close 7.3 ms, successor open 6.7 ms, header append 44 us |
| Final WAV | 3,840,512 bytes |
| WAV format | PCM, mono, 16,000 Hz, 64,000 byte/s, 32 bit |

The read-only card report found the finalized file at
`/1789635600/aud_1789638801_2.wav`, containing exactly a 512-byte header and
60 x 16,000 x 4 bytes of PCM. The following zero-length file,
`/1789635600/aud_1789638861_3.wav`, is the open successor interrupted when
probe-rs reset the board to flash the inspector; it is expected for this forced
test termination, not a rotation failure.

### GPS-enabled release variant

The default-feature release build was then flashed and observed with the GPS
hardware, PPS capture, NMEA processing, Time Service, and Location Service all
enabled. The first GPS/PPS anchor was accepted after 3.7 seconds and recording
started with `source=GpsPps` after 3.9 seconds.

| Check | Result |
| --- | --- |
| CPU while recording | 8.3–9.4%; 8.84% mean across 25 steady five-second windows |
| Mean attribution | `mic_dma` 0.00%, `audio_forward` 0.20%, `audio_recorder` 5.29%, `logging` 0.34%, `other` 2.81% |
| Mean nested storage | Audio 3.32%; logging 0.31% |
| Capture rate | 16,000 Hz through IRQ count 1,281 |
| DMA errors/gaps | None observed; `DOVRF=false` |
| Audio drops/write failures | None observed |
| Rotations | Two completed at 60-second intervals |
| First rotation | Close 15.7 ms, successor open 22.6 ms, header append 43 us |
| Second rotation | Close 14.5 ms, successor open 25.0 ms, header append 44 us |

The first profile window included startup and filesystem initialization and was
21.1%, with 19.3% attributed to `other`; it is excluded from the steady-state
mean. This GPS-enabled result was produced by the release profile. Unoptimized
development builds must not be compared directly with it.

One run exposed an approximately 590 ms residual after accepting the initial
GPS/PPS epoch, causing subsequent anchors to be rejected. A clean release
reflash after reconnecting the ST-Link did not reproduce it: GPS/PPS anchors
were accepted continuously with sub-millisecond residuals and zero rejected
anchors throughout the 56-second verification capture. The fault was traced to
TIM4 wrap extension using the delayed executor observation time to choose the
number of 65.536 ms counter wraps. A storage or logging stall could therefore
add several false wraps permanently even though the captured PPS cadence was
still one second. Wrap extension now chooses the candidate consistent with the
periodic PPS cadence while retaining the hardware-measured oscillator drift.
Synthetic 343 ms and 590 ms delayed-wake regressions pass, and the patched
release accepted 65 consecutive hardware anchors with zero rejections and
sub-millisecond residuals through an audio rotation and the first 60-second
frequency-calibration sample.
