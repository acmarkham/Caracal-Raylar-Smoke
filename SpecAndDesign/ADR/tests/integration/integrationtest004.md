# Integration Test 004: Radio Heartbeat, Discovery, and Rendezvous

- **Status:** Requirements
- **Date:** 2026-10-05
- **Decision owners:** Firmware team

## Objective

Validate the Phase I Radio Messaging Service on two or more Raylar nodes by
exercising:

1. periodic heartbeat transmission and reception;
2. presence-based neighbour discovery and soft-state refresh; and
3. deterministic UTC rendezvous prediction.

The test deliberately runs faster than a deployment configuration. Every node
shall advertise presence and transmit one heartbeat during a 40-second active
window followed by 20 idle seconds in each 60-second UTC epoch.

All boards run the identical firmware image. A board becomes the base station
only when the `USER` button is held during boot role selection. The base
station indicates its latched role by keeping `SysGpsRed` continuously on.

## Context

The Phase I Radio Messaging Service is the sole owner of the LR1121 driver and
provides bounded radio arbitration, compact frames, UTC epoch/rendezvous
utilities, heartbeat and presence protocols, a fixed-capacity neighbour table,
passive RSSI/SNR observations, and a static link estimator.

This integration test validates those facilities together. It is distinct
from Integration Test 003, which tests interchangeable range-test endpoints
using direct range-test traffic. Integration Test 004 shall use the Radio
Messaging Service and its heartbeat, presence, scheduler, neighbour, and link
profile APIs. No task outside `RadioService` may access the LR1121 driver.

## Decision

Implement a continuously running firmware under:

```text
integrationtests/integration004_radioheartbeat/
```

One binary supports both roles. Role selection is a boot-time runtime choice;
there shall be no base-station build feature and no separate base-station
source tree.

The test uses a 60-second UTC epoch with a 20-second common active window:

```text
epoch start                                              next epoch
    |---------------------------------------------------------|
    | presence rendezvous | heartbeat rendezvous | inactive  |
    |      0 to 20 s      |      20 to 40 s      | 40 to 60 s|
```

Presence and heartbeat use distinct purpose values and subwindows. Each
subwindow contains twenty one-second slots. For each node and purpose, the
Radio Messaging Service derives a deterministic permutation of all twenty
slots from the shared network ID, schedule version, purpose, node ID,
occurrence, and twenty-epoch block number. Epoch position within that block
selects an element of the permutation. Every node therefore visits every slot
exactly once per block, while all receivers can independently derive the same
schedule. A new permutation is derived for each block. The heartbeat subwindow
offset is added only after the heartbeat slot is derived.

This split prevents a node's own presence advertisement from conflicting with
its own heartbeat. Transmissions from different nodes can still collide; that
is expected for an unacknowledged contention-based protocol. Independent
permutations spread those collisions across epochs and prevent the short
lockstep cycles caused by reducing the previous hash directly modulo twenty.

## Goals

- Prove that the Radio Service exclusively owns and recovers the physical
  radio.
- Transmit and decode compact Phase I heartbeat frames at an accelerated rate.
- Discover peers from presence advertisements and retain them in the bounded
  neighbour table.
- Verify that independent devices calculate compatible presence and heartbeat
  rendezvous slots.
- Verify base-station continuous promiscuous reception and normal-node
  duty-cycled rendezvous reception with the same firmware image.
- Record timing, scheduler, frame, neighbour, RSSI/SNR, and error diagnostics.
- Remain `no_std`, heapless, statically allocated, and Embassy-async.

## Non-goals

This test does not add:

- acknowledgements, retries, reliable delivery, or end-to-end transport;
- tree, mesh, geographic, or collaborative routing;
- adaptive profile selection or active probing;
- security, authentication, or encryption;
- a range guarantee or regulatory approval;
- a runtime role change after boot;
- persistent neighbour or sequence state across reset.

## Services and ownership

```text
Integration Test 004
  - boot role selection
  - accelerated epoch policy
  - heartbeat/presence orchestration
  - neighbour/rendezvous verification
  - bounded diagnostics and LED indication
                 |
                 v
       Radio Messaging Service  <---- Time Service
                 |                    UTC mapping/uncertainty
                 v
          LR1121 Radio Driver

       Identity Driver ----> compact NodeId
       TRNG Driver ---------> BootId
       Button Driver -------> boot role
       LED Driver ----------> role/activity/fault indication
```

- `RadioService` takes the LR1121 driver by value and is its only owner.
- The Time Service is the sole UTC authority. The test shall not estimate UTC
  from GPS or monotonic time independently.
- The platform Identity Driver supplies the stable identity used to derive
  `NodeId`.
- The TRNG Driver supplies one `BootId` per boot before protocol state is
  created.
- Presence and heartbeat tasks submit bounded timed jobs through a
  `RadioHandle`; they never invoke driver operations directly.
- Latest service state and statistics use watches. Received-frame and radio-job
  events use bounded channels because every retained event must be processed.
- Every node, including the base station, uses the Logging Service backed by
  the Storage Service to persist pertinent test information to the SD card for
  later analysis. The test shall not rely on RTT/`defmt` as its durable record.
- Logging and SD writes must not block radio scheduling or RX re-arming. Radio
  tasks enqueue bounded, timestamped records; a separate logging task drains
  them through the Logging Service. Queue capacity and the configured record
  rate must be sufficient for the full test workload.

## Persistent test logging

Each node shall save all pertinent test information to its SD card via the
Logging Service and Storage Service. Logs from the base station and normal
nodes are all required; RTT/`defmt` output may supplement them but is not a
substitute. Each record shall identify the node and boot session and include a
monotonic timestamp plus UTC and UTC-validity/uncertainty when available, so
records from different nodes can be correlated without treating invalid UTC as
authoritative.

Persist at minimum:

- boot/startup with test name and firmware version/hash, role selection,
  node/boot IDs, network and schedule versions, radio profile, and UTC
  readiness/degradation/recovery transitions;
- sampled GPS location/fix quality and UTC frequency-calibration progress,
  including lock transitions without duplicating every terminal update;
- every scheduled presence and heartbeat TX attempt and completion/rejection,
  including epoch, purpose, derived slot, sequence, and scheduler outcome;
- every received frame relevant to the test, including decoded type/source/
  boot ID/sequence, packet-complete timestamp, RSSI/SNR, validation result, and
  observed-versus-predicted slot classification;
- peer discovery, refresh, boot-session change, expiry, per-epoch neighbour
  table snapshots, and base-station capability observations;
- rendezvous windows opened/skipped, scan-versus-predicted mode, rendezvous
  successes/misses, UTC guard decisions, and scheduler conflicts;
- periodic per-epoch summaries and final counters for TX/RX, neighbour count,
  link observations, malformed/unsupported traffic, time state, scheduler
  outcomes, queue pressure, and radio errors/recoveries; and
- all logging/storage errors, enqueue failures, dropped/truncated records,
  SD-card availability transitions, and logger recovery outcomes.

The Logging Service shall use bounded buffering. Check and account for every
logging outcome; log overload or SD unavailability must be visible in a later
persisted summary when storage recovers, and must never silently discard
pertinent records. If the SD card or Logging Service is unavailable, continue
radio operation without blocking, retain bounded loss/error counters, and
report that the run's persistent record is incomplete. Such a run cannot pass
the logging acceptance criterion. Size the queue and verify sustained SD write
throughput against the configured event rate before the hardware campaign.
Records longer than one Logging Service message shall be emitted as numbered,
reconstructable parts rather than silently truncated.

## Identical firmware and boot role selection

The expected operator sequence for the base station is to hold `USER` while
resetting or powering the board.

At startup:

1. Initialize the Button and LED Drivers before starting radio protocol tasks.
2. Wait 50 ms for electrical and mechanical settling.
3. Sample the debounced `USER` state once.
4. Latch `BaseStation` if pressed; otherwise latch `Node`.
5. Never change the role until the next reset.
6. If the role is `BaseStation`, turn `SysGpsRed` on continuously before radio
   activity begins.
7. If the role is `Node`, keep `SysGpsRed` off.

`SysGpsRed` is reserved for the base-station role in this test and shall not be
reused as a GPS-lock or recoverable-error indicator. A fatal error uses
`SysMainRed` so the base-station indication remains unambiguous.

Multiple boards can technically select the base-station role, because the code
is identical and the protocol remains valid. The formal test setup requires
exactly one base station. If a node hears more than one presence advert with
the base-station capability bit, it shall report a topology-configuration
warning.

## Common radio and schedule configuration

One checked-in configuration module shall be consumed by every board. The
initial configuration is:

| Parameter | Phase I test value |
| --- | ---: |
| Network ID | dedicated Integration Test 004 constant |
| Wire version | 1 |
| Schedule version | 3 |
| Epoch duration | 60 s |
| Common active window | 40 s |
| Presence subwindow | epoch offset 0-20 s |
| Heartbeat subwindow | epoch offset 20-40 s |
| Idle interval | epoch offset 40-60 s |
| Slot duration | 1 s |
| Presence transmissions | 1 per epoch |
| Heartbeat transmissions | 1 per epoch |
| Modulation | LoRa |
| Frequency | 868.000 MHz default |
| Spreading factor | SF7 |
| Bandwidth | 125 kHz |
| Coding rate | 4/5 |
| Preamble | 12 symbols |
| Header / CRC | explicit / enabled |
| Sync word | `0x12` |
| TX power | 14 dBm default |

SF7 is intentional: the test is performed at short range and prioritizes
short airtime and many observation cycles over maximum link budget. The one-
second logical slots remain deliberately generous relative to expected SF7
airtime.

The selected Ebyte E80 module, antenna, frequency, power, and duty-cycle policy
must be confirmed for the test location. Hardware support alone does not make
a channel legal. A different approved profile may replace the default, but
all nodes in one run must use exactly the same profile and configuration ID.

Configuration validation shall reject:

- a broadcast window longer than the epoch;
- zero or fractional slot counts;
- overlapping presence and heartbeat subwindows;
- an invalid frequency, modulation, packet size, or TX power; and
- incompatible wire, schedule, network, or profile identifiers.

## Startup and UTC gate

1. Initialize Identity, TRNG, Button, LED, GPS, Time, Radio Driver, and Radio
   Messaging Service resources.
2. Derive the compact `NodeId` from the Identity Driver and obtain the boot's
   random `BootId` from the TRNG Driver.
3. Perform and latch boot role selection.
4. Start the Time Service and its GPS/PPS source.
5. Start the sole-owner Radio Service and initialize the common SF7 profile.
6. Wait until the Time Service can convert between current monotonic time and
   UTC. Long-term frequency calibration need not be locked.
7. Log the node ID, boot ID, role, network/schedule versions, radio profile,
   current UTC, UTC status, and uncertainty.
8. Join the next complete 60-second epoch rather than transmitting in a
   partially elapsed startup window.

If UTC is invalid, scheduled presence and heartbeat transmissions are
suspended. The base station continues chained monotonic receive operations and
marks received traffic as not slot-verifiable until UTC returns. A normal node
falls back to a conservative full common-window listen after UTC becomes usable
again; it must not invent an independent UTC estimate.

If UTC uncertainty exceeds the configured narrow-rendezvous threshold, normal
nodes shall not claim a rendezvous success or failure from a narrow window.
They may widen the guard within the 40-second active window or fall back to a
full-window scan. The derived guard shall include local uncertainty, expected
remote uncertainty, scheduler uncertainty, propagation allowance, and an
engineering margin.

## Presence and heartbeat traffic

Every node, including the base station, sends both frame types once per epoch.

### Presence advertisement

Presence carries:

- the boot ID in the common frame header;
- schedule version;
- protocol capability flags; and
- a `BASE_STATION` capability flag when the runtime role is `BaseStation`.

A valid presence reception inserts or refreshes the peer in the bounded
neighbour table and records passive link metadata. A changed boot ID replaces
the previous execution-session state for that node. Presence is the
authoritative discovery event for this test.

### Heartbeat

The heartbeat uses the Phase I common frame and heartbeat payload. It includes
the source node ID, boot ID, and volatile sequence in the common header. Status
fields unavailable in this focused test are encoded using their defined
unknown/absent representations rather than fabricated values. UTC validity,
holdover, fix-quality class, and uncertainty class are derived from service
state. Location is included only if the Location Service is started and has a
valid application-facing estimate; the integration test shall never query GPS
directly for a location.

A valid heartbeat from an already discovered peer refreshes its last-seen and
passive link observations. A heartbeat from an unknown peer is reported, but
the peer is not considered fully discovered until a compatible presence advert
has supplied its schedule version and capabilities.

Sequence numbers increase within the boot session and wrap according to the
Phase I protocol contract. Duplicate identity uses `(NodeId, BootId,
Sequence)`, not sequence alone.

## Base-station behaviour

The base station is promiscuous and does not duty-cycle the receiver. It shall:

- never enter retained radio sleep during normal operation;
- accept every compatible common frame regardless of source or destination;
- use consecutive bounded RX reservations rather than one unbounded driver
  operation;
- re-arm RX immediately after a packet, timeout, its own TX, or recoverable
  driver cleanup;
- schedule its own presence advert and heartbeat just like every other node;
  and
- return to promiscuous RX immediately after each transmission.

One physical transceiver cannot receive while transmitting. “Always listening”
therefore means no intentional sleep or idle duty cycle; the only allowed RX
gaps are its own scheduled transmissions, required channel preparation,
bounded cleanup/recovery, and the time needed to copy a received frame before
re-arming.

The scheduler shall split base-station receive reservations around its known TX
slots. It shall not reserve one 60-second RX job that prevents the already-known
presence and heartbeat jobs from running.

## Normal-node discovery and rendezvous behaviour

A normal node operates in two modes.

### Bootstrap scan

Until it has discovered a base station, the node listens for the complete
40-second active window. It also uses a full active-window scan:

- for the first two complete epochs after startup;
- after UTC invalidity or excessive uncertainty;
- after the known base station expires or changes boot ID; and
- once every fifth epoch to discover late-starting or rebooted peers.

### Predicted rendezvous

After receiving a compatible presence advert, the node derives that peer's
future presence and heartbeat slots from shared network state. In non-scan
epochs it reserves guarded RX windows only around predicted peer activity,
merging overlapping peer windows before submitting them to the Radio Service.

For each received presence or heartbeat, the node records whether reception
occurred:

- during a full bootstrap scan;
- inside a predicted guarded rendezvous window;
- outside the predicted window; or
- during a window made unverifiable by UTC uncertainty or a local radio
  conflict.

A local TX or higher-priority scheduler reservation may make a predicted peer
window unavailable. Such a case is a scheduler conflict, not evidence that the
peer used the wrong rendezvous slot.

## Frame processing and passive link observations

Every received packet is copied into a bounded event before slower diagnostics
or LED indication. The receiver shall:

1. validate wire version, frame type, flags, optional fields, and exact payload
   length;
2. decode multi-byte values in network byte order;
3. reject malformed or unsupported traffic without panicking;
4. convert the driver's packet-complete monotonic IRQ timestamp to UTC where
   the Time Service allows;
5. compare the observed UTC epoch/slot with the source's independently predicted
   slot;
6. update RSSI and LoRa SNR passive observations for valid peer traffic; and
7. update heartbeat, presence, malformed, unsupported, conflict, queue-drop,
   timeout, and radio-error counters.

The base station's continuous reception is the primary oracle for slot
placement: for every decoded frame it calculates the slot expected for that
source, purpose, and epoch and compares it with the observed packet-complete
time, allowing for packet airtime and the configured uncertainty guard.

## Diagnostics and local indication

At startup and once per epoch, emit a bounded diagnostic summary through the
Logging Service containing:

```text
role and base-station capability
NodeId and BootId
current UTC/status/uncertainty
epoch and current offset
own next presence and heartbeat slots
radio mode and selected profile
frames/heartbeats/presence TX and RX
malformed/unsupported frames
scheduler misses/conflicts and queue drops
neighbour count
last peer, RSSI, SNR, and observed/predicted slot result
radio errors and recoveries
```

Persisted per-event diagnostics shall identify TX submission/completion/rejection,
presence discovery/refresh, heartbeat reception, neighbour expiry, predicted
rendezvous success/miss, malformed input, and driver recovery. Diagnostic
formatting must not block radio timing or use an unbounded queue.

LED policy is:

- `SysGpsRed`: solid on only for the latched base-station role;
- `SysSdBlue`: short pulse after a successful local heartbeat or presence TX;
- `SysMainGreen`: short pulse after a valid peer heartbeat or presence RX;
- `SysMainRed`: solid on for a fatal initialization/service failure.

Activity pulses are bounded and best-effort. They must not delay RX re-arming.

## Error and overload behaviour

- A full radio-request queue returns an explicit error; it does not allocate or
  silently overwrite another job.
- A full diagnostic/event queue follows its documented bounded drop policy and
  increments a visible counter; loss of a pertinent record invalidates the
  persistent-log acceptance criterion even if radio operation continues.
- Missed slots and scheduler conflicts remain visible and do not trigger an
  immediate retry outside the deterministic schedule.
- Ordinary RX-window timeout or CRC/header rejection is not treated as a fatal
  service failure.
- A recoverable driver fault follows the Radio Driver recovery path and then
  restores the role's receive policy.
- A fatal initialization or repeated unrecoverable service fault latches
  `SysMainRed`; the base station keeps `SysGpsRed` on so its selected role is
  still visible.
- Malformed frames, unknown versions, unknown types, and random RF traffic must
  never panic a receiver or terminate the Radio Service task.

## Test procedure

Use at least two boards and preferably three.

1. Flash the identical Integration Test 004 image onto every board.
2. Hold `USER` while resetting exactly one board. Confirm that board latches
   `BaseStation` and turns `SysGpsRed` on continuously.
3. Reset the remaining boards without pressing `USER`. Confirm `SysGpsRed`
   remains off and their role is `Node`.
4. Confirm every board reports the same network ID, schedule version, profile,
   epoch duration, active window, and slot duration.
5. Wait for usable Time Service UTC and the next full epoch.
6. Run for at least ten complete epochs (ten minutes).
7. Confirm each board transmits one presence advert in seconds 0-20 and one
   heartbeat in seconds 20-40 of each usable epoch, then remains idle from
   seconds 40-60.
8. Confirm the base station remains in chained promiscuous RX except for its
   own TX and bounded radio maintenance.
9. Confirm every peer appears in the base-station neighbour table and every
   normal node discovers the base-station capability.
10. After bootstrap, confirm normal nodes receive the base station in predicted
    guarded windows during non-scan epochs.
11. Compare each decoded frame's observed epoch/slot with the receiver's
    independent prediction.
12. Reset one normal node. Confirm its stable `NodeId` remains the same, its
    `BootId` changes, its sequence restarts, and peers refresh the execution
    session without creating an unbounded duplicate entry.
13. Stop or shield one normal node long enough to exercise the configured test
    neighbour expiry, if expiry is included in the campaign.
14. Inject or replay truncated, unknown-version, unknown-type, and invalid
    heartbeat/presence payloads from a test transmitter. Confirm explicit
    counters increase and the service continues.
15. Remove GPS reception or otherwise increase UTC uncertainty. Confirm narrow
    rendezvous claims stop, normal nodes use the documented fallback, and the
    base station remains promiscuously listening.

## Acceptance criteria

The test passes when:

- the same firmware image successfully runs both roles selected solely by the
  boot-time `USER` state;
- exactly the selected base station shows a solid `SysGpsRed` indication;
- no task other than `RadioService` owns or calls the LR1121 driver;
- every usable epoch schedules one local presence advert and one local
  heartbeat in their specified 10-second subwindows;
- each node records successful local heartbeat transmission in at least nine
  of ten usable test epochs, with every miss explained by an explicit bounded
  error;
- the base station decodes at least one heartbeat and one presence advert from
  every nearby peer within two usable epochs;
- each normal node discovers the base station within two usable epochs;
- a discovered peer occupies one bounded neighbour-table entry keyed by node
  and current boot session, and a reboot refreshes that session correctly;
- every decoded, slot-verifiable frame agrees with the receiver's independent
  rendezvous calculation after applying airtime and uncertainty guards;
- during at least three non-scan epochs, each normal node receives at least one
  base-station frame inside a predicted guarded rendezvous window;
- the base station never deliberately duty-cycles or enters retained sleep;
- RSSI and LoRa SNR observations are retained for valid peer receptions;
- queue exhaustion, scheduler conflicts, slot misses, malformed frames, time
  degradation, and radio recovery are visible in bounded diagnostics; and
- every node produces a complete SD-card log through the Logging Service with
  the required timestamped events and epoch summaries, with no unaccounted
  pertinent-record loss; and
- malformed traffic and recoverable radio faults do not panic or permanently
  stop heartbeat, presence, or receive operation.

Packet delivery is unacknowledged and probabilistic. A missed remote frame is
not by itself a failure if local scheduling and subsequent discovery/rendezvous
criteria pass. Persistent unexplained misses at bench range are a failure and
must be investigated rather than hidden by retries.

## Host and build checks

Before hardware testing, add host tests for:

- the 60-second epoch and both 10-second subwindow boundaries;
- fixed presence and heartbeat rendezvous vectors;
- heartbeat and presence encode/decode, including exact encoded sizes;
- base-station capability encoding;
- role selection state latching independent of later button changes;
- RX-window guard derivation from Time Service uncertainty;
- merged predicted windows for multiple neighbours;
- neighbour insertion, refresh, boot-ID change, expiry, and bounded replacement;
- classification of observed frames as predicted, scan, outside, or
  unverifiable;
- scheduler conflicts between RX windows and the node's own TX slots; and
- malformed, truncated, unsupported, and maximum-size frames.

Build the firmware in release mode for the board target. The build shall fail
clearly for incompatible feature combinations or invalid compile-time radio
configuration.

## Consequences

### Positive

- One image can turn any board into the base station in the field.
- Accelerated one-minute epochs provide useful evidence in a short bench run.
- The continuously listening base station provides an independent oracle for
  heartbeat timing and slot placement.
- Normal nodes exercise the low-power rendezvous path instead of relying only
  on promiscuous reception.
- Presence, heartbeat, neighbour state, time uncertainty, scheduling, and
  passive link observations are tested together through the intended service
  boundary.

### Negative

- The accelerated cadence and SF7 profile do not represent deployment energy
  use or maximum range.
- A base station cannot receive during its own transmissions, despite having
  no deliberate duty cycle.
- Contention traffic can collide, so the test cannot demand receipt of every
  unacknowledged heartbeat.
- GPS/PPS UTC acquisition can delay the start of deterministic scheduling.
- RTT capture from several boards must be correlated carefully during review.

## Open questions for implementation

- Select the dedicated Integration Test 004 network/configuration identifiers.
- Confirm the fitted Ebyte E80 variant and approved 868 MHz power/duty-cycle
  policy for the test location.
- Measure SF7 heartbeat and presence airtime and set the packet-complete slot
  tolerance accordingly.
- Select the narrow-rendezvous UTC uncertainty threshold and remote uncertainty
  assumption from measured Time Service behaviour.
- Choose the fixed neighbour-table capacity for the intended board count.
- Decide whether the shortened neighbour expiry is enabled in the default
  ten-minute run or only in a dedicated expiry campaign.
- Measure and report the base station's maximum RX re-arm gap, excluding its
  own transmissions and explicit recovery.
