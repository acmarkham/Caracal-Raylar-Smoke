# App Aardwolf: GPS-Calibrated Audio Logger and Link Probe

- **Status:** Requirements
- **Date:** 2026-10-07
- **Decision owners:** Firmware team

## Objective

Define a long-running application that combines the GPS-calibrated audio and durable logging workflow of Integration Test 002 with the Radio Messaging Service heartbeat and link-observation workflow of Integration Test 004. It is intended to run continuously for approximately one week per deployment, exercising solar-powered operation, storage, timekeeping, audio capture, radio operation, and recovery from ordinary faults.

The first implementation is a measurement logger and scheduled link probe. It is not a general-purpose mesh network or a reliable data transport.

## Context

Integration Test 002 exercises GPS/PPS-calibrated time, location, minute WAV recording, system logging, power monitoring, and sensors. Integration Test 004 exercises the Radio Messaging Service, compact heartbeat frames, reception, RSSI/SNR observations, and persistent radio diagnostics. Aardwolf needs these capabilities together in an application-oriented, low-duty-cycle workload.

The radio campaign varies LoRa spreading factor and band over time so long-term link observations can be compared under changing conditions. Every node listens during each active minute as well as participating in its scheduled heartbeat exchange. The Time Service mapping provides the schedule reference; each received packet's radio metadata and timestamps are retained for later analysis.

## Goals

- Run unattended for a target duration of seven days.
- Capture GPS-calibrated, timestamped audio into bounded WAV segments.
- Persist useful system, location, power, time, audio, and radio diagnostics.
- Exercise both 868 MHz and 2.4 GHz LR1121 operation across SF7, SF8, SF10, and SF12 profiles.
- Let nodes hear other nodes' scheduled heartbeats and retain passive link measurements over time.
- Use sleep for the inactive portion of the probe epoch and battery hysteresis to support solar and battery-life evaluation.
- Remain heapless, statically allocated, and non-blocking across service boundaries.

## Non-goals

The first version does not provide acknowledgements, retries, guaranteed delivery, mesh routing, adaptive profile selection, a claimed RF range, audio classification, or an assumption that every transmitted heartbeat is received.

## Decision

Implement a continuously running Aardwolf application under `apps/aardwolf/`.

The application composes existing services and drivers. It shall not access radio, GPS, microphone, storage, or power hardware directly when a driver or service boundary exists.

### Service ownership

```text
Aardwolf application
  - continuous run policy and 30-minute probe epoch
  - audio segment policy and application-level diagnostics
  - radio profile schedule and heartbeat orchestration
  - sleep policy and run summaries
          |                 |                  |
          v                 v                  v
 Audio / Location /    Radio Messaging      Logging / Storage /
 Time / Power Services    Service           Versioning Services
          |                 |                  |
          v                 v                  v
      GPS, audio       LR1121 Driver         SD / filesystem
      and power
      drivers
```

- The Radio Messaging Service is the only owner of the LR1121 Driver. Aardwolf submits radio work through its bounded API.
- The Time Service is the sole UTC authority. Aardwolf uses its UTC estimate, validity, and uncertainty for epoch alignment and records UTC status alongside monotonic timestamps.
- GPS/PPS acquisition and oscillator calibration follow the Time Service and GPS Driver contracts established by Integration Test 002. NMEA arrival time is not a time anchor.
- Audio is consumed through the Audio Service/AudioSource and recorded through the recorder and Storage Services. Aardwolf does not operate microphone DMA or write the filesystem directly.
- Location comes from the Location Service's filtered estimate.
- Power state comes from the Power Management Service. The application observes battery/solar state, reports it, and applies the battery hysteresis below. It does not change the radio profile order based on link quality.
- Shared latest state uses watches. Events that must be accounted for, such as radio TX/RX results and diagnostic records, use bounded channels.

## Probe schedule

Use a repeating 30-minute epoch aligned to a UTC half-hour boundary. The first 12 minutes select one radio profile each, in this order:

| Minute in epoch | Band | PHY / modulation | Activity |
| ---: | --- | --- | --- |
| 1 | 868 MHz | LoRa SF7, BW125k, CR4/5 | Heartbeat and receive/listen |
| 2 | 868 MHz | LoRa SF8, BW125k, CR4/5 | Heartbeat and receive/listen |
| 3 | 868 MHz | LoRa SF10, BW125k, CR4/5 | Heartbeat and receive/listen |
| 4 | 868 MHz | LoRa SF12, BW125k, CR4/5 | Heartbeat and receive/listen |
| 5 | 2.4 GHz | LoRa SF7, BW812k, CR4/5 | Heartbeat and receive/listen |
| 6 | 2.4 GHz | LoRa SF8, BW812k, CR4/5 | Heartbeat and receive/listen |
| 7 | 2.4 GHz | LoRa SF10, BW812k, CR4/5 | Heartbeat and receive/listen |
| 8 | 2.4 GHz | LoRa SF12, BW812k, CR4/5 | Heartbeat and receive/listen |
| 9 | 2.4 GHz | GFSK 250 kbps, Fdev 125k, RX BW467k (nominal 500k) | Heartbeat and receive/listen |
| 10 | 2.4 GHz | GFSK 38.4 kbps, Fdev 40k, RX BW156.2k (nominal 160k) | Heartbeat and receive/listen |
| 11 | 2.4 GHz | GFSK 4.8 kbps, Fdev 5k, RX BW19.5k (nominal 20k) | Heartbeat and receive/listen |
| 12 | 2.4 GHz | LoRa SF12, BW203k, CR4/5 | Heartbeat and receive/listen |
| 13–30 | — | — | Radio asleep |

Use +14 dBm conducted output power at 868 MHz and +13 dBm at 2.4 GHz. Keep these values fixed for the run and record antenna/path configuration. Fixed conducted power gives a repeatable comparison; antenna gain and propagation differ between bands.

The minute numbers are one-based relative to the epoch start. Minute 1 begins at second zero. While `Active`, each active minute uses exactly its listed band/profile. Each node sends one heartbeat and listens for peer broadcasts throughout the available receive time, except during its own TX, profile switching, and necessary radio recovery. Seconds 0–58 are available for scheduled radio activity; second 59 is unavailable for transmission and is reserved for profile switching and settling. At the end of minute 12, transition the radio to sleep. During minutes 13–30 the radio enters the service's supported low-power/off state; no background receive duty cycle is requested. Prepare the minute-1 profile before the next epoch boundary so it is ready at second zero.

All participating nodes share the same schedule, network parameters, and schedule version. Use Integration Test 004's deterministic per-node permutation, seeded with network ID, schedule version, purpose, NodeId, occurrence, and epoch block. Each profile minute has its own slot grid. The 16-byte heartbeat airtime below supports one-second slots except for minutes 4 and 12, which use two-second slots to leave practical scheduling guard after the long SF12 packet. One-second minutes have 59 candidate TX slots; two-second minutes have 29 candidate TX slots and a final RX-only second before the switch guard. The permutation visits every candidate slot once per block of that many 30-minute epochs. Slot length and algorithm version are shared configuration and are logged. Up to ten nodes are expected; collisions remain possible and are measured rather than hidden by unscheduled retries.

The node joins the next full epoch after its first usable GPS/PPS-derived UTC mapping. During a later GPS outage, continue the same heartbeat schedule using the last calibrated mapping and monotonic holdover, even as uncertainty grows. Keep listening during the active minutes. Mark transmitted heartbeats and local records as holdover or UTC-invalid as appropriate, with uncertainty and time since the last PPS. Receivers retain packet timestamps to measure holdover drift after the run; they widen receive coverage to the full available minute as needed rather than claiming a narrow rendezvous was met. If the Time Service cannot retain a mapping when UTC becomes invalid, extend its interface and the Radio Service scheduler to support this explicit holdover policy. After a reset with no retained mapping, acquire UTC before joining a schedule; do not guess the epoch phase.

### Common radio configuration

Use a dedicated Aardwolf network ID, schedule version, and configuration ID shared by all nodes. The initial LoRa packet configuration follows Integration Test 004: 12-symbol preamble, explicit header, CRC enabled, and sync word `0x12`. Use 868.1 MHz as the proposed UK sub-GHz center frequency for the 125 kHz profiles; 868.000 MHz from Integration Test 004 would place part of a 125 kHz signal below the 868.0 MHz band edge. Use one common 2.4 GHz center frequency that supports the widest selected profile, provisionally 2441 MHz. Freeze both exact frequencies in the checked-in run configuration before deployment, together with regional duty-cycle/EIRP limits and antenna details.

For GFSK use the Radio Service reference packet settings: Gaussian BT 0.5 shaping, 32-bit preamble with 16-bit detection, four-byte sync word, address filtering disabled, variable packet length, two-byte CRC (`init=0xFFFF`, `poly=0x1021`, not inverted), and whitening with seed `0x01FF`. Fix the four sync bytes in the shared run configuration. The chosen sync word changes neither the 16-byte frame length nor the airtime calculation as long as it remains four bytes.

## Receive and link observations

During each profile's active minute, every node opens receive windows according to the Radio Messaging Service scheduler, including time to hear peers transmitting under the shared rendezvous policy. The radio service re-arms receive promptly after TX and between windows as required by its API.

For each relevant received frame, retain:

- monotonic packet-complete timestamp and UTC mapping/status/uncertainty when available;
- epoch number, active minute, band, modulation, SF or GFSK bitrate/deviation, and configured radio profile;
- decoded frame type, source NodeId, BootId, sequence, and validation result;
- RSSI and LoRa SNR for LoRa, and supported RSSI/quality metrics for GFSK;
- observed TX/rendezvous slot and predicted slot where available; and
- receive-window, scheduler, and radio recovery/error context.

For each scheduled local heartbeat, record intended epoch/minute/profile, derived rendezvous, enqueue result, TX completion/rejection, airtime where available, and sequence. Use the fixed 16-byte V4 frame below. The current Radio Service still uses wire version 1 with a 32-bit BootId, so Aardwolf needs a versioned V4 encoder/decoder and corresponding session handling. Do not truncate a V1 frame's BootId without changing its declared wire version. Generate each 16-bit on-air BootId randomly at boot and log it with the local reset context; detect and report a reused BootId for the same NodeId when possible.

Profile comparisons must be interpretable across the week. Every record and summary identifies exact band, frequency/configuration, modulation and its complete parameters, transmit power, antenna/configuration identifier, and radio firmware when available. The common profile configuration is fixed for a run and persisted at startup; a change requires a schedule/configuration version change.

The Radio Service and LR1121 Driver now represent 2.4 GHz LoRa BW203/BW406/BW812 and the requested GFSK reference rates. The reference builder uses the actual programmable GFSK RX filters of 467, 156.2, and 19.5 kHz; the larger round numbers in the original profile request are nominal datasheet labels. Aardwolf must configure and log the actual values. The V4 frame remains application work and is not yet implemented by the service.

### Proposed V4 heartbeat frame

Use a fixed 16-byte broadcast frame at every profile so packet length and airtime are stable even when location or storage state changes. All multi-byte fields use network byte order. The 16 bytes are the radio payload; LoRa preamble/header/CRC and GFSK preamble/sync/length/CRC are additional on-air bits.

| Offset | Size | Field | Encoding |
| ---: | ---: | --- | --- |
| 0 | 1 | Version and type | High nibble `4`, low nibble `1` for heartbeat (`0x41`). |
| 1 | 1 | Frame flags | Zero for broadcast heartbeat; reject unknown bits. |
| 2 | 4 | NodeId | Stable 32-bit identity. |
| 6 | 2 | BootId | Random 16-bit ID generated each boot. |
| 8 | 2 | Sequence | 16-bit counter, wrapping within one BootId. |
| 10 | 1 | Battery SOC | `0..100` percent; `0xFF` means unavailable. |
| 11 | 1 | Charging/source | Existing `ChargingState` codes: none 0, solar 1, USB 2, external 3, unknown 4. |
| 12 | 2 | Error flags | Bit 0: storage full; 1: storage unavailable; 2: recovering from low-SOC pause; 3: audio/storage write fault; 4: logging impaired. Remaining bits zero until assigned in V4. |
| 14 | 1 | Storage use | `0..100` percent; `0xFF` means unavailable. |
| 15 | 1 | GPS/time status | Existing `GpsStatus` bit layout: UTC valid, fix-quality class, uncertainty class, and holdover flag; reserved bits zero. |

This is a 10-byte header and 6-byte status payload, without an optional location field. Keep full filtered location and location quality in the SD log. Use one 16-byte heartbeat per node per active minute while `Active`; there is no per-profile padding, repetition, or retry. A V1 decoder must reject V4 unless it explicitly implements that version, and the V4 decoder must reject malformed lengths and reserved bits. The shorter BootId increases collision risk across reboots of one NodeId, so sequence and receiver state must be reset on a newly observed boot session.

### Heartbeat time on air and slots

For LoRa, calculate symbol time as `2^SF / BW` and packet time as `(12 + 4.25 + payload_symbols) × symbol_time`. With explicit header, CRC enabled, CR4/5, 16 PHY payload bytes, and low-data-rate optimization enabled when symbol time is at least 16 ms, use `payload_symbols = 8 + 5 × ceil((8×16 − 4×SF + 28 + 16) / (4×(SF − 2×DE)))`, where `DE` is 1 when low-data-rate optimization is enabled and 0 otherwise. These are calculated radio packet durations, excluding profile setup, TX ramp, scheduler latency, and receive re-arming.

For GFSK, use the Radio Service's reference packet configuration: 32-bit preamble, a four-byte sync word fixed in the common run configuration, variable-length mode with one on-air length byte, a 16-byte frame, and a two-byte CRC. Total on-air size is `32 + 32 + 8 + 128 + 16 = 216 bits`; packet time is `216 / bitrate`. Gaussian shaping, whitening, deviation, and RX bandwidth do not add packet bits.

| Minute | Profile | Calculated packet airtime | TX slot |
| ---: | --- | ---: | ---: |
| 1 | 868 MHz LoRa SF7/BW125k | 55.552 ms | 1 s |
| 2 | 868 MHz LoRa SF8/BW125k | 100.864 ms | 1 s |
| 3 | 868 MHz LoRa SF10/BW125k | 362.496 ms | 1 s |
| 4 | 868 MHz LoRa SF12/BW125k, DE=1 | 1,449.984 ms | 2 s |
| 5 | 2.4 GHz LoRa SF7/BW812k | 8.552 ms | 1 s |
| 6 | 2.4 GHz LoRa SF8/BW812k | 15.527 ms | 1 s |
| 7 | 2.4 GHz LoRa SF10/BW812k | 55.803 ms | 1 s |
| 8 | 2.4 GHz LoRa SF12/BW812k | 197.990 ms | 1 s |
| 9 | 2.4 GHz GFSK 250 kbps | 0.864 ms | 1 s |
| 10 | 2.4 GHz GFSK 38.4 kbps | 5.625 ms | 1 s |
| 11 | 2.4 GHz GFSK 4.8 kbps | 45.000 ms | 1 s |
| 12 | 2.4 GHz LoRa SF12/BW203k, DE=1 | 892.847 ms | 2 s |

One node transmits about 1.969 s at 868 MHz and 1.222 s at 2.4 GHz per 30-minute epoch, or about 3.191 s total, before radio setup overhead. Use the calculated airtime to set the TX deadlines and pre-switch guard; confirm packet-complete timing and slot margins on hardware before the week-long run. Any change to frame length, preamble, sync length, CRC, or PHY parameters requires recalculating this table and changing the schedule/configuration version.

## Audio capture and time correlation

Use the Audio Service and Audio Recorder Service to record mono 16 kHz audio as 32-bit integer WAV data, using the agreed high-quality decimation/filter configuration. Segment files on UTC minute boundaries after GPS/PPS-derived time is valid. Retain every completed file without overwrite, using minute-long WAV files in hour-long folders through the Storage Service. Each file's metadata/log record includes UTC file start time, monotonic system start, time validity/uncertainty, a hash of the Location Service estimate, firmware/device identity, calibration status, calibration correction in ppb, and the last PPS time relative to monotonic system time. Record the hash algorithm and input representation so files from the same location can be compared. During GPS holdover, continue recording on the calibrated minute boundaries and mark the increased time uncertainty.

At 16,000 samples/s × 4 bytes/sample, audio uses 3.84 MB per minute, 230.4 MB per hour, and 38.7 GB for seven days before WAV headers and filesystem overhead. A seven-day run creates 10,080 minute files in 168 hour folders. Plan for a card with sufficient usable capacity for those files, a system log below 1 GB, and a protected 100 MB log reserve; a nominal 64 GB card has sufficient nominal capacity if its usable free space is verified at startup.

Audio recording continues through radio probe windows unless power or a recording/storage fault requires a documented degraded mode. Radio scheduling, receive processing, and log draining do not block audio acquisition. Audio packet loss, timestamp gaps, overruns, file-finalization failures, and storage errors are surfaced in durable diagnostics when possible.

## UI

Use LEDs and the buzzer to indicate device health and status. Indications must be short, asynchronous, and must not delay audio or radio work.

| Device | Indication |
| --- | --- |
| Buzzer | Distinct startup, first GPS fix, UTC calibration locked, and severe-error patterns. Optional packet-RX beep is a bench configuration and is disabled in field builds. |
| `SysGpsGreen` | Solid while GPS is powered and waiting for PPS; brief PPS indication when edges arrive; off when GPS is powered down. |
| `SysMainRed` | Severe error. |
| `SysSdBlue` | Brief pulse when an audio packet is ready for recording; pulse rate follows the actual packet rate. |
| `SysMainGreen` | Brief pulse on valid radio RX. |
| `SysGpsRed` | Brief pulse on radio TX. |

When `SysGpsGreen` is already solid, make PPS visible as a brief off/on blink rather than an indistinguishable additional on pulse. Preserve `SysMainRed` for severe errors; recoverable GPS unavailability is reported in logs and heartbeat status.

## Long-run logging and power

Create a standard system log through the Storage and Logging Services. Persist startup traceability from the Versioning Service, including explicit unknown or unavailable states. Record at minimum:

- startup configuration, firmware/build identity, node/boot identity, and start/end/reset reason;
- Time Service readiness, PPS calibration/lock, holdover, degradation, and recovery transitions;
- location estimates and fix quality periodically and on validity changes;
- periodic PowerState snapshots including battery, solar input, charging, and power-source/status fields exposed by the service;
- audio segment start/end, packet/sample counts, timestamp bounds, and gaps;
- every heartbeat attempt and relevant reception with link metrics and the V4 frame length;
- profile changes, skipped windows, sleep/wake transitions, scheduler conflicts, queue pressure, radio errors/recoveries, and periodic summaries; and
- logging/storage errors, enqueue failures, dropped/truncated records, and media availability transitions.

Emit a bounded health/power/time/radio summary every 10 seconds, plus per-event records for heartbeat TX/RX, audio segment boundaries, and significant state changes. Keep the seven-day system log below 1 GB; the expected volume from comparable runs is about 200 MB, but actual event volume and queue capacity must be checked against the chosen node count and packet rate.

The event path is bounded and sized for sustained traffic. Radio and audio timing do not wait on SD writes. Check every enqueue/write result and account for loss in a later summary if storage recovers. If persistent logging is unavailable, retain bounded loss counters and mark the run incomplete. A deployment run cannot pass data-integrity acceptance with unaccounted log or audio loss.

Before starting each new WAV file, check free space and stop audio cleanly when starting the next segment would cross a protected 100 MB free-space reserve. Never overwrite an earlier file. Continue system logging and heartbeats while `Active`, with the storage-full flag in each heartbeat. If the card is unavailable or logging itself fails, continue heartbeats with a storage-unavailable flag and bounded in-memory loss counters. Do not claim durable records during that interval. If the log consumes the reserve too, report its failure over heartbeat and retain loss counters until storage recovers or reset occurs. The low-SOC `EnergyRecovery` state takes precedence over this storage-fault radio policy.

The application exposes enough periodic power information to assess solar charging and energy balance over the week. Sleep the radio for minutes 13–30 of each epoch and use service-supported low-power behavior elsewhere. The application must not claim that the entire device sleeps while audio recording is active. Any deeper system sleep requires compatible audio, time, storage, and wakeup behavior to be specified and validated separately.

### Battery hysteresis

Use `PowerState.battery_percent` from the Power Management Service as the SOC input. Maintain a latched `Active` or `EnergyRecovery` state; the thresholds are strict. In `Active`, enter `EnergyRecovery` when a valid SOC is **below 10%**. In `EnergyRecovery`, return to `Active` only when a valid SOC is **above 20%**. Values of exactly 10% or 20% retain the current state. A missing SOC value does not imply either threshold: retain the current state and log that SOC is unavailable. At boot, start in `EnergyRecovery` until a valid SOC above 20% is observed. An explicit externally powered bench configuration may bypass this startup gate, but must be recorded and must not be used for the solar campaign.

On entry to `EnergyRecovery`, finalize the current WAV, record the transition and audio gap, stop scheduled radio TX/RX, put the LR1121 into its service-supported sleep state, and power down GPS through its service/driver policy. Keep charging, low-rate Power Management Service sampling, and enough timekeeping alive to decide when to resume. Avoid claiming the whole MCU is off unless a wake path from charger/RTC/power monitor is implemented. Log the low-power interval when storage is available. No heartbeat is sent while radio is asleep; this is an intentional, separately counted gap rather than a packet-loss event.

On return to `Active`, restart GPS/time acquisition as needed, resume WAV recording at the next valid UTC minute boundary, and resume the radio at the next complete 30-minute epoch using the available calibrated mapping. Set error flag bit 2 on the first successful post-recovery heartbeat to tell peers why the node was absent, then clear it. Log SOC, raw battery voltage, power source, and both transition times so solar recharge and hysteresis behavior can be reconstructed. The Power Management Service's SOC is currently estimated from battery voltage; the threshold policy uses that published estimate and records the voltage for review.

## Error behavior

- GPS unavailability is a service degradation while `Active`. Continue holdover scheduling after the first valid anchor, retain the last location estimate with age, and flag degraded timing in heartbeat and logs. GPS shutdown in `EnergyRecovery` is intentional.
- The Radio Messaging Service attempts its documented bounded recovery. An unrecoverable radio failure triggers a self-reset through the supported platform reset path, with the reset cause recorded before reset where possible.
- An unusable or full SD card stops new WAV files without overwrite. Heartbeats continue, carrying distinct full/unavailable status. The severe-error indication may report storage impairment, but it must not stop the radio service.
- On each startup, read and persist the hardware reset reason and start a new BootId/run segment. If the previous shutdown was a brownout, watchdog, or deliberate radio-fault reset, identify that explicitly when available. Persistent sequence continuity across reset is not assumed.
- Schedule/configuration mismatch, invalid UTC for a rendezvous, and profile setup failures are visible and are not reported as successful link observations.

## Test procedure

1. Use at least two nodes with identical Aardwolf firmware and common configuration.
2. Confirm the startup log captures device, firmware, storage-card, GPS, and radio identity/version states through the Versioning Service.
3. Confirm PPS-derived time reaches the defined valid state and recording begins on the next UTC minute boundary.
4. Observe a complete 30-minute probe epoch and verify all 12 active profiles occur in order, the two-second slots are used for minutes 4 and 12, second 59 of each active minute is unavailable for TX, and radio sleep occupies minutes 13–30.
5. Confirm both nodes transmit 16-byte V4 heartbeats and receive peer broadcasts during each profile's active minute, with timestamps and link metrics persisted. Compare measured packet-complete timing with the airtime table and account for profile setup and scheduling overhead.
6. Confirm WAV files remain correctly segmented and timestamped while radio windows execute.
7. Remove GPS reception after initial calibration; confirm heartbeats continue on monotonic holdover, are marked degraded, and can be compared against peers' receive timestamps.
8. Fill the card toward the protected reserve or simulate SD unavailability; confirm audio stops, prior files remain intact, and heartbeats continue with the correct storage status.
9. Drive reported SOC below 10%, through the 10–20% band, and above 20%. Confirm one transition into recovery, no oscillation in the band, clean WAV finalization, intentional radio silence, and aligned resumption with a recovery flag in the first heartbeat.
10. Exercise a recoverable radio fault and an unrecoverable radio fault; confirm recovery or a reasoned self-reset and new BootId.
11. Run a short soak first, then the planned seven-day solar/battery campaign.
12. Inspect summaries for audio continuity, profile-specific reception statistics, power trends, queue pressure, recoveries, reset reasons, and complete logs.

## Acceptance criteria

- Aardwolf runs continuously for the configured target duration without unhandled panic or memory allocation during normal operation.
- Audio files are valid mono 16 kHz/32-bit WAV segments with GPS/PPS-calibrated minute boundaries and auditable time metadata.
- Every 30-minute epoch in `Active` follows the 12-profile order, then keeps the radio inactive for the remaining 18 minutes.
- Every transmitted and received heartbeat uses the 16-byte V4 frame and is associated with its profile, epoch/minute, monotonic timestamp, and available UTC/link metadata.
- Measured packet-complete timing is consistent with the calculated airtime plus bounded radio setup and scheduler overhead; minutes 4 and 12 fit their two-second TX slots.
- Link metrics are retained per LoRa band/SF and GFSK bitrate/deviation so variability can be analyzed after the run; missed packets and radio/scheduler errors have explicit counters.
- Radio receive activity causes no unexplained audio loss and does not block audio acquisition or persistent log draining.
- PowerState, solar/battery observations, radio sleep windows, and run summaries are available to evaluate week-long operation.
- Battery SOC below 10% latches `EnergyRecovery`; activity resumes only above 20%, with transitions and intentional audio/radio gaps recorded. Exact-threshold and unknown-SOC behavior follows the stated policy.
- Audio stops before consuming the protected 100 MB log reserve, earlier files are retained, and heartbeats continue with explicit full/unavailable card status while SOC permits `Active` operation.
- GPS outage after initial calibration does not stop the probe schedule; holdover timing and uncertainty remain visible to peers and in local records.
- SD/logging or audio loss is never silently represented as a complete run.
- All radio access is mediated by the Radio Messaging Service, audio by Audio/Recorder services, and filesystem access by Storage.

## Consequences

### Advantages

- Combines time-calibrated audio capture with repeatable, profile-specific link measurements in one realistic long-running workload.
- The long idle portion of each probe epoch exercises radio low-power behavior while audio, logging, and solar charging continue.
- Timestamped local TX and peer RX records support post-run comparison across bands and spreading factors.
- Existing service boundaries remain responsible for hardware ownership and domain-specific state.

### Costs and limitations

- Continuous audio recording and persistent logging may dominate energy and storage; the campaign measures these costs rather than attributing them to radio alone.
- 2.4 GHz and 868 MHz profile airtimes, antenna behavior, legal limits, and available link metrics differ, limiting direct comparisons without careful configuration metadata.
- Unacknowledged broadcasts can collide and packet reception is probabilistic.
- A one-week run requires storage-capacity, SD-card endurance, watchdog, brownout, and recovery planning.
- The radio sleeps for most of each epoch, so the test measures scheduled heartbeat links rather than continuous availability.

## Remaining implementation decisions

- Confirm measured TX setup, ramp, packet-complete timing, RX re-arming, and time-uncertainty guards fit the one- and two-second slots. The airtime table is a PHY calculation, not a measured scheduling bound.
- Confirm the 2.4 GHz profile switching and GFSK reference settings on the fitted LR1121 module during hardware bring-up. The Driver and Radio Service already represent the profiles, including the 467 kHz programmable RX filter behind the nominal 500 kHz label.
- Finalize the dedicated Aardwolf network/configuration identifiers, exact RF center frequencies, GFSK packet parameters, antenna configuration, and UK duty-cycle/EIRP policy in the shared run configuration. The listed center frequencies are proposals until that configuration is frozen.
- Implement the specified V4 wire layout and handling of the shorter 16-bit BootId across resets. The existing V1 codec and neighbour table assume a 32-bit BootId.
- Define the location-hash algorithm and byte representation, including whether the hash must allow comparisons across nodes and deployments.
- Set bounded queue capacities from measured SD write throughput and maximum ten-node RX/event rate; keep the system log below 1 GB over seven days.
- Finalize the watchdog timeout and how a radio-fault reset reason survives reset. Do not reset solely because GPS is unavailable or the card is full.

## Implementation location

Implement the application under `apps/aardwolf/`.
