# Integration Test 003: Interchangeable Radio Range Test

- **Status:** Requirements
- **Date:** 2026-10-01
- **Decision owners:** Firmware team

This ADR defines a two-device radio range test. 

## Objective

Measure practical radio performance between two Raylar devices while keeping
the endpoints interchangeable. Each device normally listens for any compatible
packet and sends a short packet at randomized intervals. Neither device is
permanently assigned the sender or receiver role.

The test correlates transmitted and received packets with UTC and reported GPS
locations, records every packet and its radio metadata through the Logging
Service, and gives immediate local feedback when a packet is received.

## Context

The LR1121 radio driver supports complete LoRa and GFSK channel configuration,
timed operations in the Embassy monotonic time domain, caller-owned receive
buffers, IRQ timestamps, and RSSI/SNR metadata. The board provides GPS/PPS,
the Time and Location Services, a Logging Service backed by Storage, a buzzer,
and system LEDs.

The radio driver test in `drivertests/radio` already demonstrates interchangeable
devices that primarily receive and send at jittered intervals. Integration
Test 003 adds GPS-qualified UTC, filtered location, durable packet logging,
distance calculation, and receive indication.

## Decision

Implement a continuously running range-test firmware under
`integrationtests/integration003_radiorange/`.

Both devices run identical firmware built from one shared radio-test
configuration. Each device starts in promiscuous receive mode after acquiring
GPS PPS UTC synchronization and a valid filtered location. It continues
receiving between its own transmissions and sends a packet at independently
randomized intervals. The device ID in every packet identifies its sender.

The firmware uses the LR1121 Radio Driver for all RF operations, the GPS Time
Service for UTC conversion, the Location Service for its own filtered
coordinates, and the Logging Service for every transmit and receive event. It
does not use the Audio Service. Local signs of life comprise an accepted-PPS
flash on `SysGpsGreen`, a successful-TX flash on `SysSdBlue`, a short buzzer
tone and `SysMainGreen` flash on receive, and distinct startup/error beep
patterns.

## Goals

- Test configurable LoRa and GFSK channels on supported Ebyte E80 bands.
- Keep sender and receiver roles interchangeable on both devices.
- Require GPS PPS synchronized UTC before beginning radio activity.
- Include a coarse transmitter position and UTC send time in each packet.
- Log every transmit and successful receive, including timing and available
  radio metadata.
- Compute and report the distance between the sender and receiver positions
  for each successfully decoded packet.
- Show a brief buzzer tone and green system LED flash for each received packet.
- Flash the GPS green LED for each accepted GPS PPS anchor and the blue LED for
  each successfully completed transmission.
- Emit distinct audible startup and recoverable/fatal error patterns.
- Make packet loss, counters, logging loss, time quality, and location age
  visible during a range test.
- Use statically allocated, bounded resources and caller-owned radio buffers.

## Non-goals

The test does not provide:

- reliable delivery, acknowledgements, retries, encryption, or application
  networking;
- a permanent base station or fixed sender/receiver assignment;
- a voice, audio-recording, or Audio Service workload;
- a regional channel plan or regulatory decision engine;
- surveyed ground-truth distance or a guarantee of radio range;
- a GPS implementation outside the existing GPS, Time, and Location Services.

## Services and ownership

```text
Integration Test 003
  - common channel and packet configuration
  - UTC-lock startup gate and randomized TX cadence
  - packet encoding/decoding and distance calculation
  - packet event logging and receive indication
          |          |           |            |
          v          v           v            v
    Radio Driver  Time Service  Location Service  Logging Service
                                                   |
                                                   v
                                             Storage Service
```

- The GPS Driver feeds the Time Service's PPS-correlated UTC mapping and the
  Location Service's GPS-fix stream. Integration code must not create a second
  UTC mapping or use raw, unfiltered GPS coordinates as its application
  location.
- The Time Service owns UTC conversion. Packet timestamps are obtained by
  converting the radio driver's monotonic event instant through the current
  `TimeState`/`TimeResources::system_to_utc` mapping.
- The Location Service owns location filtering and publishes the latest
  application-facing estimate. The test uses that estimate both for its own
  transmit position and for receiver position in distance calculations.
- The radio driver remains single-owner. The test does not issue LR1121
  commands outside the Radio Driver.
- The Logging Service writes packet records to the standard system log stream
  through the Storage Service. Producers must check logging outcomes and make
  dropped or truncated diagnostic records observable.
- The buzzer driver and LED control are used directly by bounded indication
  tasks; there is no audio service in this test. Radio processing only enqueues
  indications and never waits for an LED pulse or tone to complete.

## Common configuration

One common configuration file is the source of radio and packet settings for
both devices. Implement it as a checked-in configuration module (proposed name
`integrationtests/integration003_radiorange/radio_test_config.rs`) or an
equivalent shared generated configuration. Both endpoints must use the same
configuration identifier and compatible channel and packet settings.

The configuration shall provide:

- modulation: LoRa or GFSK;
- RF frequency / selected channel;
- LoRa spreading factor, bandwidth, coding rate, header, CRC, sync word,
  preamble, IQ, and LDRO selection when LoRa is selected;
- GFSK bitrate, deviation, receiver bandwidth, pulse shape, preamble detector,
  sync word, packet length, address filtering, CRC, and whitening when GFSK is
  selected;
- per-band transmit power and ramp time;
- a configuration identifier included in each packet;
- the local coordinate origin used by the compact location encoding;
- the mean and allowed range for randomized transmit intervals;
- maximum acceptable location age and receive-indication duration; and
- logging and receive-indication policy values where they need to be tuned.

The driver validates frequency, modulation parameters, TX power, and packet
compatibility before changing the radio. Both devices must be configured with
the same modulation and channel to decode one another. Frequency and power
remain explicit configuration choices; this test does not infer that a
hardware-supported frequency is permitted in a deployment region.

## Startup and UTC lock

1. Initialize the board, Radio Driver, GPS Driver, Time Service, Location
   Service, Storage Service, Logging Service, buzzer driver, and LED control.
2. Start the standard system log stream and register a dedicated radio-test
   logger.
3. Keep the radio in standby while GPS time and filtered location are being
   acquired. Do not start RX or TX before the UTC gate is satisfied.
4. Open the UTC startup gate as soon as the Time Service has accepted at least
   one GPS PPS anchor and can map the current monotonic instant to UTC. Do not
   wait for `frequency_calibration_locked`, the long-term oscillator
   calibration window, or a particular non-invalid UTC quality classification.
   Record the current UTC status and uncertainty, which may improve while the
   radio test is already running.
5. Require `LocationState.valid` before entering the normal range-test loop.
   The Location Service's configured accepted-fix threshold (three fixes by
   default) applies. Log the UTC-lock and location-acquired transition,
   including their service status and timestamps.
6. Prepare the selected complete channel, enable RX IRQ handling, and enter
   promiscuous receive operation.

Once started, the test continues to use the Time Service during holdover and
logs the current UTC quality and uncertainty with each event. It must not label
degraded or unavailable time as GPS-locked time. If UTC becomes invalid, suspend
new TX and RX operations until the GPS PPS UTC gate is restored. If location is
temporarily invalid or older than the configured maximum age, do not transmit
a packet with stale coordinates; log the condition and continue reception if
the radio and UTC remain usable. A received packet whose receiver location is
unavailable is still logged, with distance marked unavailable.

## Interchangeable receive and transmit operation

- Both devices continuously listen for compatible packets, with GFSK address
  filtering disabled and no source-ID filter at the application layer. LoRa
  receives all packets matching the configured LoRa waveform and sync word.
- Each device chooses its own next transmit time using a jittered interval
  derived from the shared configuration. The initial default is a uniform
  interval from 3 to 17 seconds, giving a 10-second mean. The seed must differ
  between devices (for example, a mix of device ID and a changing monotonic
  timer value) so two boards do not repeatedly collide in lockstep.
- Before TX, the device validates current UTC and location state, constructs
  the packet, prepares the radio and issues timed TX using the driver. It returns
  to receive operation immediately after TX completion or timeout cleanup.
- Each successful RX is handled, logged, and indicated before the next receive
  window is armed. The scheduler should keep gaps between receive windows as
  short as radio cleanup and logging allow. Radio work must not wait for SD
  writes or the buzzer; packet information is copied into a bounded event/log
  record before those slower operations proceed.
- There is no fixed sender or receiver role, and no radio packet queue is
  required beyond the bounded event handling needed to avoid blocking the IRQ
  path.

## Packet format

Use a compact versioned binary packet with a fixed header and fixed-width
fields. All multi-byte integers use network byte order. The initial proposed
layout is:

| Field | Size | Meaning |
| --- | ---: | --- |
| Protocol version | 1 byte | Packet layout version |
| Configuration ID | 2 bytes | Must match the common test configuration |
| Sender device ID | 8 bytes | Stable 64-bit ID from the Identity Driver |
| Sender sequence | 2 bytes | Monotonically increasing counter for this device run |
| TX UTC seconds | 8 bytes | UTC seconds mapped from requested monotonic TX start |
| TX UTC microseconds | 4 bytes | Fractional part in `[0, 1_000_000)` |
| East offset | 2 bytes | Signed 10-metre units relative to the configured local origin |
| North offset | 2 bytes | Signed 10-metre units relative to the configured local origin |

The proposed payload length is 29 bytes. Radio CRC remains enabled according to
the selected modulation configuration. Invalid version, configuration ID,
length, timestamp, or coordinate fields are rejected as application packets
and counted/logged as decode errors; they must not be used in distance
calculations.

The common config supplies a reference latitude/longitude near the test area.
The transmitter converts its filtered Location Service estimate to local east
and north offsets from that origin, quantized to 10-metre units. Signed 16-bit
offsets give ample coverage for a test area where devices are within 10 km of
each other. The same origin and projection are used by both endpoints. This
avoids transmitting two full-precision global coordinates while allowing the
receiver to reconstruct coarse positions. Quantization contributes up to
approximately 7.1 m per reported point in the local plane; the distance record
must also include the Location Service uncertainties and the resulting
estimated distance uncertainty where it can be calculated.

The sender sequence is monotonic within a device boot/run and is not reset
between packets. A device ID change separates streams across devices. If a
future implementation persists the sequence across reboot, that change must
preserve monotonicity or add an explicit boot/session identifier.

## UTC and event timestamps

- The TX UTC field is derived through the Time Service from the monotonic
  instant requested for TX. The packet's UTC represents the scheduled local TX
  start; the TX log also records the driver's requested start, command timing,
  TX-done IRQ time, and the current UTC status/uncertainty.
- On RX, capture the radio driver's `packet_complete_at` monotonic IRQ timestamp
  and convert that exact instant to UTC through the Time Service before logging.
  Do not timestamp the packet using the later time at which the application
  finishes parsing it or completes an SD write.
- RX logs include both sender TX UTC from the packet and receiver RX UTC from
  the local Time Service. Their difference is a diagnostic packet age, not a
  propagation-time measurement: it includes radio airtime, scheduling error,
  and clock uncertainty.
- Every log record includes UTC where available, monotonic event ticks, UTC
  status/source, and uncertainty. If the Time Service cannot map an event to
  UTC, retain and log the monotonic event rather than substituting the current
  UTC value.

## Range and link measurements

For a valid received packet, reconstruct the sender's coarse local east/north
position from the configured origin and packet offsets. Convert the receiver's
current filtered Location Service estimate using the same origin/projection,
then calculate horizontal separation in metres using a local tangent-plane
distance (or an equivalent geodesic calculation). The test area's expected
maximum separation is 10 km. The output is an estimate based on GPS filter
uncertainty and 10-metre coordinate quantization, not surveyed ground truth.

Record and display at least:

- received packet count, sender ID, and sender sequence;
- modulation, channel/frequency, protocol/configuration ID;
- sender TX UTC and receiver RX UTC, monotonic event ticks, and UTC quality;
- packet age and time uncertainty;
- packet RSSI and modulation-specific metrics (LoRa signal RSSI/SNR or GFSK
  status as exposed by the driver);
- sender and receiver coarse positions, estimated separation, and uncertainty;
- sequence gaps per sender; and
- CRC/header/decode errors, TX/RX timeouts, driver recovery events, and logging
  queue drops/truncation.

Sequence gaps show packets not observed by this receiver. With no ACK protocol,
they do not distinguish RF loss from collisions, receiver TX intervals, or
radio/processor scheduling gaps. TX and RX totals from both logs may be
compared after a test run; the test must not claim guaranteed delivery or
attribute every gap to range.

## Packet logging

Every TX attempt and every successfully decoded RX packet must be submitted to
the Logging Service. TX records include the encoded sender ID and sequence,
packet UTC/location, selected channel, TX power, requested/command/TX-done
timing, and result. RX records include the complete decoded packet identity,
sender and receiver UTC/location, RSSI and other available metadata, distance,
sequence-gap status, and result.

Also record startup and state transitions, UTC-lock loss/reacquisition,
location validity/staleness, TX/RX timeouts, invalid packets, radio recoveries,
and logging losses. Use bounded logger queues and ensure the configured
producer rate is supportable by the storage sink. Check every enqueue outcome;
expose dropped or truncated events through logging statistics and a visible
diagnostic counter rather than silently losing evidence. If durable storage is
temporarily unavailable, keep radio operation bounded and report the logging
failure when the sink recovers; do not block IRQ capture or radio timing on an
unbounded write.

Human-readable log formatting may evolve, but timestamps, units, device IDs,
sequence numbers, and missing-value semantics must remain explicit.

## Local indications

After a valid packet has been captured and its essential event data has been
queued for logging:

1. flash `SysMainGreen` briefly (initial suggestion: 50 ms); and
2. play a short audible beep through the Buzzer Driver (initial suggestion:
   2 kHz for 50 ms at a low configured volume).

The tone and LED duration are configurable. Do not invoke the Audio Service or
create audio recordings. Receive indication must not delay IRQ timestamp
capture, packet extraction, or rearming RX. If several packets arrive close
together, coalesce overlapping LED flashes and beeps rather than creating an
unbounded indication queue; every packet is still logged.

In addition:

- pulse `SysGpsGreen` briefly whenever the Time Service publishes a newly
  accepted GPS PPS anchor;
- pulse `SysSdBlue` briefly after every successful TX completion;
- play a short ascending pattern during firmware startup; and
- play a distinct descending error pattern for radio recovery failures,
  logging failures, and fatal startup failures. Fatal failures repeat the error
  indication periodically.

All indication delivery is bounded and best-effort. It must not block radio
timing, packet processing, UTC capture, or logging.

## Configuration and invalid packets

The test has one operational configuration per firmware build. It does not
switch channels dynamically during a range run. A channel or modulation change
requires rebuilding both devices from the same common configuration and
configuration ID. The startup log records the complete selected modulation and
channel parameters so data from different builds can be distinguished.

Reject malformed packets, unsupported configuration IDs, out-of-range local
coordinates, invalid UTC fractional values, and duplicate/stale sequence
values according to an explicit policy. Log the reason and relevant available
metadata. Duplicate detection is diagnostic only; it does not add a reliable
delivery or deduplication service.

## Verification plan

### Build and configuration checks

- Build the default LoRa configuration and at least one GFSK configuration.
- Build supported sub-GHz and 2.4 GHz configurations with the same config
  consumed by both devices.
- Confirm invalid feature/configuration combinations fail clearly at build or
  startup.
- Confirm the LR1121 Driver rejects invalid modulation, frequency, and power
  configurations before applying them.

### Hardware tests

- Confirm neither device starts RX or TX before a usable GPS PPS UTC anchor
  and a valid filtered location are published, and confirm radio activity
  starts without waiting for frequency calibration to lock.
- Confirm each device independently begins promiscuous RX, emits randomized
  TX packets, and returns to RX after every TX completion or timeout.
- Run both boards with identical configuration and verify packets are decoded
  in both directions without assigning fixed roles.
- Verify sender ID and increasing sequence, TX UTC, and coarse position decode
  correctly at the peer.
- Compare reported positions and separation with known stationary test
  locations; quantify GPS, projection, and packet-quantization uncertainty.
- Verify each successful packet has one durable TX/RX record with the expected
  UTC, monotonic timestamp, RSSI/SNR or GFSK metadata, and distance.
- Verify sequence gaps, malformed packets, IRQ errors, queue overflow, storage
  failure, and radio recovery remain visible in diagnostics.
- Verify a short beep and green LED flash occur on packet reception and that
  receive indication does not create RX gaps beyond the configured test
  tolerance.
- Verify accepted GPS PPS anchors flash `SysGpsGreen`, successful transmissions
  flash `SysSdBlue`, and startup/error beep patterns are distinguishable.
- Repeat at increasing device separation and record channel, modulation,
  antenna orientation, environment, firmware/configuration ID, and observed
  packet delivery and signal metrics.

The test is a measurement tool. Acceptance thresholds for packet reception
rate, RSSI, and maximum range are specific to the selected channel, antennas,
environment, and regional constraints; they should be recorded with each
measurement campaign rather than treated as universal driver guarantees.

## Consequences

### Positive

- Either board can be moved or replaced without changing firmware role.
- Packet timestamps can be correlated between devices using the Time Service's
  GPS PPS disciplined UTC mapping.
- Coarse local positions support useful distance estimates with less packet
  overhead than full global coordinates.
- Per-packet logging combines range, signal, location, and timing evidence in
  one reviewable record.

### Negative

- Both devices need usable GPS PPS UTC and a valid location before the test
  begins, which increases setup time and excludes indoor runs without a GPS
  time source.
- Random transmit intervals can collide, and the test has no ACK mechanism to
  classify missed packets.
- Reported separation inherits GPS location error, local projection error,
  and coordinate quantization.
- Durable per-packet logging can become the throughput bottleneck and requires
  deliberate queue sizing and loss diagnostics.

## Open questions for implementation

- Confirm whether the shared test configuration should be a Rust source module
  included by both firmware builds or generated from a small data file, while
  preserving compile-time validation and one authoritative copy.
- Confirm the local coordinate origin and map projection for the intended
  test area; the initial design assumes both devices are within 10 km of that
  origin.
- Confirm the desired maximum acceptable location age and UTC uncertainty for
  transmitting and for reporting a distance.
- Measure time from the driver's IRQ timestamp to logging enqueue and verify
  queue capacity against the expected packet rate and storage latency.
- Decide how often to emit periodic summaries (packet counts, sequence gaps,
  RSSI statistics, logging drops) in addition to mandatory per-packet records.
- Confirm which Ebyte E80 module variant, supported PA path, antenna, and
  regional channel/power limits apply to each range campaign.
