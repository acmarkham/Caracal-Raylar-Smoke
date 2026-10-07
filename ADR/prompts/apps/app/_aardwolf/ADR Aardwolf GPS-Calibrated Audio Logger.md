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
- Use sleep for the inactive portion of the probe epoch to support solar and battery-life evaluation.
- Remain heapless, statically allocated, and non-blocking across service boundaries.

## Non-goals

The first version does not provide acknowledgements, retries, guaranteed delivery, mesh routing, adaptive profile selection, a claimed RF range, audio classification, or an assumption that every transmitted heartbeat is received.

## Decision

Implement a continuously running Aardwolf application under `apps/aardwolf/`.

The application composes existing services and drivers. It shall not access radio, GPS, microphone, storage, or power hardware directly when a driver or service boundary exists.

### Service ownership

```text
Aardwolf application
  - seven-day run policy and 30-minute probe epoch
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
- Power state comes from the Power Management Service. The application observes battery/solar state and reports it; adaptive changes to the radio schedule are outside this first version.
- Shared latest state uses watches. Events that must be accounted for, such as radio TX/RX results and diagnostic records, use bounded channels.

## Probe schedule

Use a repeating 30-minute epoch aligned to a valid UTC half-hour boundary. The first 12 minutes select one radio profile each, in this order:

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
| 9 | 2.4 GHz | GFSK 250 kbps, Fdev 125k, RX BW500k | Heartbeat and receive/listen |
| 10 | 2.4 GHz | GFSK 38.4 kbps, Fdev 40k, RX BW160k | Heartbeat and receive/listen |
| 11 | 2.4 GHz | GFSK 4.8 kbps, Fdev 5k, RX BW20k | Heartbeat and receive/listen |
| 12 | 2.4 GHz | LoRa SF12, BW203k, CR4/5 | Heartbeat and receive/listen |
| 13–30 | — | — | Radio asleep |

Set output powers to: 

868 MHz  = +14 dBm
2.4 GHz  = +13 dBm

to enable fair comparison

The minute numbers are one-based relative to the epoch start. Each active minute uses exactly its listed band/profile for its radio window. The window permits each participating node to send its scheduled heartbeat and listen for peer broadcasts. During the other 17 minutes the radio enters the service's supported low-power/off state; no background receive duty cycle is requested.

All participating nodes share the same schedule, network parameters, and schedule version. The schedule and heartbeat rendezvous are deterministic from shared configuration and stable node identity so receivers can predict when to listen. Transmissions may collide; the protocol remains unacknowledged and collision outcomes are measured rather than hidden by unscheduled retries.

UTC may start or label an epoch only when the Time Service reports configured validity/uncertainty criteria. If time is not ready at startup, the node acquires/calibrates time while preserving the audio/logging startup policy, then joins at the next full epoch. On time degradation, it records the state and follows a documented safe radio behavior; it does not claim accurate UTC rendezvous from monotonic time alone.

## Receive and link observations

During each profile's active minute, every node opens receive windows according to the Radio Messaging Service scheduler, including time to hear peers transmitting under the shared rendezvous policy. The radio service re-arms receive promptly after TX and between windows as required by its API.

For each relevant received frame, retain:

- monotonic packet-complete timestamp and UTC mapping/status/uncertainty when available;
- epoch number, active minute, band, SF, and configured radio profile;
- decoded frame type, source NodeId, BootId, sequence, and validation result;
- RSSI and LoRa SNR, or corresponding radio-specific metrics for the 2.4 GHz mode;
- observed TX/rendezvous slot and predicted slot where available; and
- receive-window, scheduler, and radio recovery/error context.

For each scheduled local heartbeat, record intended epoch/minute/profile, derived rendezvous, enqueue result, TX completion/rejection, airtime where available, and sequence. Heartbeat payloads remain compact and identify the source node and boot session; do not add audio or large telemetry payloads.

Profile comparisons must be interpretable across the week. Every record and summary identifies exact band, frequency/configuration, SF, bandwidth, coding rate, transmit power, antenna/configuration identifier, and radio firmware when available. The common profile configuration is fixed for a run and persisted at startup; a change requires a schedule/configuration version change.

## Audio capture and time correlation

Use the Audio Service and Audio Recorder Service to record mono 16 kHz audio as 32-bit integer WAV data, using the agreed high-quality decimation/filter configuration. Segment files on UTC minute boundaries after GPS/PPS-derived time is valid. Place segments in time-organized folders through the Storage Service. Every file's metadata/log record includes UTC file start time, monotonic system start, time validity/uncertainty, hashed location snapshot, and relevant firmware/device identity, as well as any other useful fields such as calibration status, calibration_ppb, last PPS time (relative to system time).

Audio recording continues through radio probe windows unless power or a recording/storage fault requires a documented degraded mode. Radio scheduling, receive processing, and log draining do not block audio acquisition. Audio packet loss, timestamp gaps, overruns, file-finalization failures, and storage errors are surfaced in durable diagnostics when possible.

## UI

Use LEDS and buzzer to indicate device health and status:

Buzzer:
- Startup beep
- First fix beep
- UTC calibrated beep
- Error beep
- Optional packet rx beep (used for bench debugging, will be disabled for field testing to prevent irritation)

LEDs:
- GPS green LED: 
  - solid on: gps powered up, waiting
  - pulse: brief flash on PPS
  - off: GPS off
- System RED
  - Error
- Blue LED
  - Brief flash on audio packet ready (typically ~5Hz)
- System Green
  - Radio RX
- GPS red LED:
  - Flash on radio TX

## Long-run logging and power

Create a standard system log through the Storage and Logging Services. Persist startup traceability from the Versioning Service, including explicit unknown or unavailable states. Record at minimum:

- startup configuration, firmware/build identity, node/boot identity, and start/end/reset reason;
- Time Service readiness, PPS calibration/lock, holdover, degradation, and recovery transitions;
- location estimates and fix quality periodically and on validity changes;
- periodic PowerState snapshots including battery, solar input, charging, and power-source/status fields exposed by the service;
- audio segment start/end, packet/sample counts, timestamp bounds, and gaps;
- every heartbeat attempt and relevant reception with link metrics;
- profile changes, skipped windows, sleep/wake transitions, scheduler conflicts, queue pressure, radio errors/recoveries, and periodic summaries; and
- logging/storage errors, enqueue failures, dropped/truncated records, and media availability transitions.

The event path is bounded and sized for sustained traffic. Radio and audio timing do not wait on SD writes. Check every enqueue/write result and account for loss in a later summary if storage recovers. If persistent logging is unavailable, retain bounded loss counters and mark the run incomplete. A deployment run cannot pass data-integrity acceptance with unaccounted log or audio loss.

The application exposes enough periodic power information to assess solar charging and energy balance over the week. Sleep the radio for minutes 9–30 of each epoch and use service-supported low-power behavior elsewhere. The application must not claim that the entire device sleeps while audio recording is active. Any deeper system sleep requires compatible audio, time, storage, and wakeup behavior to be specified and validated separately.

## Error behavior

- Recoverable GPS, time, radio, sensor, queue, or storage issues are counted and reported without panicking.
- The Radio Messaging Service performs its documented recovery and returns to the scheduled profile/window policy.
- An unusable SD card or filesystem error stops creation of invalid WAV files and marks recording/logging as impaired. Whether the full app latches a severe fault or continues radio/power testing must be decided before hardware testing.
- A reset or brownout starts a new BootId/run segment and records the reason when available; persistent sequence continuity across reset is not assumed.
- Schedule/configuration mismatch, invalid UTC for a rendezvous, and profile setup failures are visible and are not reported as successful link observations.

## Test procedure

1. Use at least two nodes with identical Aardwolf firmware and common configuration.
2. Confirm the startup log captures device, firmware, storage-card, GPS, and radio identity/version states through the Versioning Service.
3. Confirm PPS-derived time reaches the defined valid state and recording begins on the next UTC minute boundary.
4. Observe a complete 30-minute probe epoch and verify all eight active profiles occur in order and radio sleep occupies minutes 9–30.
5. Confirm both nodes transmit heartbeats and receive peer broadcasts during each profile's active minute, with timestamps and link metrics persisted.
6. Confirm WAV files remain correctly segmented and timestamped while radio windows execute.
7. Run a short soak first, then the planned seven-day solar/battery campaign.
8. Inspect summaries for audio continuity, profile-specific reception statistics, power trends, queue pressure, recoveries, and complete logs.

## Acceptance criteria

- Aardwolf runs continuously for the configured target duration without unhandled panic or memory allocation during normal operation.
- Audio files are valid mono 16 kHz/32-bit WAV segments with GPS/PPS-calibrated minute boundaries and auditable time metadata.
- Every 30-minute epoch follows the eight-profile order, then keeps the radio inactive for the remaining 22 minutes.
- Every transmitted and received heartbeat is associated with its profile, epoch/minute, monotonic timestamp, and available UTC/link metadata.
- Link metrics are retained per band/SF so variability can be analyzed after the run; missed packets and radio/scheduler errors have explicit counters.
- Radio receive activity causes no unexplained audio loss and does not block audio acquisition or persistent log draining.
- PowerState, solar/battery observations, radio sleep windows, and run summaries are available to evaluate week-long operation.
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

## Open questions for implementation

- Define the exact active-minute TX/RX timeline: common fixed slot, deterministic per-node slots, or another rendezvous scheme. It must allow nodes to hear broadcasts while handling long SF12 airtime and collisions.
  > deterministic slots like in integration004 that follow a permutation governed by node id. This will prevent lock-step collision. Anticipated node density for this test is max 10 nodes, so collision probability will be low anyway.
- Confirm the LR1121-supported 2.4 GHz modulation/configuration represented by “SF7/SF8/SF10/SF12”, and verify each profile is supported by the current Radio Messaging Service API.
  > the modulation is supported by the chipset. The radio service and underlying driver might need modification.
- Specify bandwidth, coding rate, sync word/network ID, preamble, header/CRC, transmit power, and frequency/channel for each band. The 868 MHz channel, power, and duty-cycle policy depend on deployment region.
  > Done. UK operation.
- Decide whether the “minute 1” heartbeat occurs immediately at second zero or in a later rendezvous window, and define profile switch guard time.
  > 1 sec guard time between band switching i.e. 59 sec is an unavailable slot.
  > cycle starts at second zero
  > slots have a width of 1 sec, except for SF12 which might need a longer slot length.
- Define UTC validity and uncertainty thresholds for joining/suspending scheduled epochs, plus behavior during GPS outage and holdover.
  > Carry on transmitting like normal. Receiving nodes which might have tight sync can be used to post-hoc check holdover drift.
- Choose audio continuation policy when storage is full/unavailable and whether such a fault latches a severe application error.
  > Audio should stop (no overwrite)
  > Audio should leave some space (e.g. a hundred mbyte) so that log file can continue
  > Heartbeats should continue but flag the card is full
  > If the card is unavailable, then heartbeats should indicate an error 
- Set audio folder/file retention and estimate the seven-day WAV plus log storage requirement.
  > All files should be retained i.e. write only. Minute long files, hour long folders as before.
  > Log file should be less than 1Gbyte. Previous week long tests are ~200Mbyte, so this seems reasonable.
- Define heartbeat payload/version and the identity/boot/session fields used by the existing Radio Messaging Service.
  > Version 4
  > Node ID, boot ID should be sent as 32 bit numbers each
- Set log summary cadence and bounded queue sizes based on measured SD write throughput and radio/audio event rates.
  > 10 sec updates are sufficient
- Define the watchdog policy, reset-reason capture, and acceptable recovery behavior for multi-day operation.
  > Reset reason logging would be beneficial i.e. on startup.
  > SD card failure should not stop heartbeat service
  > Radio failure should trigger a self-reset
  > GPS unavailability is not a failure but a degradation of service

## Implementation location

Implement the application under `apps/aardwolf/`.
