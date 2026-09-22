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
- RTT reports total CPU use once per second. The attributed CPU profile is
  emitted to both RTT and the `Cpu` component of `/syslog.txt` every five
  seconds. `mic_dma`, `audio_forward`, `audio_recorder`, and `logging` are
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
  frequency-calibration lock later plays a distinct rising major-arpeggio
  jingle with an octave resolve. The synthetic-time feature plays neither
  signal.
- The green GPS LED is off while the receiver is off or in standby, solid while
  it is searching or reacquiring a navigation fix, and pulses for 50 ms for
  each GPS PPS anchor admitted by the Time Service. Raw PPS edges rejected by
  the settling/cadence gate do not flash the LED.
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
- After the first fix, GPS remains continuously active until the Time Service
  reports `frequency_calibration_locked`. This normally requires the complete
  eleven-sample, ten-minute PPS baseline, but PPS outages now extend continuous
  tracking instead of allowing a fixed timer to end calibration early. Only
  after the explicit lock handshake does the normal 30-second on/30-second off
  duty cycle begin.
- PPS edges use TIM4 channel 4 hardware input capture at 1 MHz. After the first
  cross-clock epoch is established, edge timestamps are reconstructed from the
  capture counter rather than interrupt wake-up time. STM32U59xxx TIM4 is
  32-bit, so it wraps every 2^32 us (about 71 minutes 35 seconds), not between
  1 Hz PPS edges. The driver preserves the full TIM4_CH4 value and uses coarse
  monotonic elapsed time only to extend a rare full 32-bit wrap. This is defined
  by DS13633 Rev 3 section 3.44 Table 19 (p. 80/385), section 3.44.2
  (p. 81/385), and the RM0456 TIM2-TIM5 general-purpose-timer chapter and
  `TIMx_ARR`/`TIMx_CCR4` register definitions.
  Capture initialization explicitly changes `TIM4_ARR` from its RM0456 reset
  value of `0x0000_FFFF` to `0xFFFF_FFFF`; setting the 1 MHz prescaler alone
  would otherwise retain a 65.536 ms counting period on the 32-bit peripheral.
  Embassy-stm32 0.6's asynchronous capture future also reads CCR4 through a
  16-bit register view. The GPS driver uses that future only as the edge wakeup
  and then re-reads the latched CCR4 with the library's 32-bit synchronous
  accessor before extending capture time.
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
  is off. UTC is `Invalid` only before the first accepted anchor. Above 1 ms it
  becomes `Degraded`, while the previously synchronized mapping remains
  available. A one-shot holdover warning is raised after 90 seconds.
- After 1.5 seconds without PPS, the temporary phase slew is removed with a
  continuity-preserving rebase. Reacquisition discards the first five PPS
  edges so the receiver can settle its PPS phase, then requires three clean
  raw PPS edge-pair intervals within an inclusive +/-20 ppm of one second
  (999,980 through 1,000,020 us). The settling window and cadence tolerance
  are controlled in one place by `TimeConfig::pps_reacquisition_discard_edges`
  and `TimeConfig::pps_interval_tolerance_ppm`. Raw edge intervals,
  rather than the spacing between NMEA-correlated anchors, qualify the gate,
  so a missed NMEA pairing cannot permanently block recovery. Any edge pair
  outside that tolerance is rejected and restarts both the five-edge settling
  window and the subsequent three-clean-pair gate.
- The ten-second Time records include UTC status, first/current anchor source, latest PPS
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
  seconds. The main green heartbeat and GPS status indication stop until the
  board is reset.
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
anchors throughout the 56-second verification capture. The earlier analysis
incorrectly treated STM32U595 TIM4 as a 16-bit timer and attempted to infer
65.536 ms wraps. DS13633 Rev 3 section 3.44 Table 19 instead specifies TIM4 as
32-bit, which is also represented by the RM0456 TIM2-TIM5 register definitions.
The driver now consumes the complete 32-bit TIM4_CH4 capture and has no wrap
ambiguity during ordinary PPS intervals or 30-second GPS standby cycles. It
also explicitly programs `TIM4_ARR = 0xFFFF_FFFF`; Embassy's input-capture
constructor configures the prescaler but otherwise leaves ARR at its
`0x0000_FFFF` reset value. A subsequent endurance run showed that the async
capture future in embassy-stm32 0.6 independently truncated the captured CCR4
value to 16 bits. The driver now re-reads the latched register through the
32-bit synchronous accessor after every edge. The next hardware endurance run
supersedes conclusions drawn from both former 16-bit paths.
