# Integration Test 004: radio heartbeat and rendezvous

This directory contains the single firmware image used by every board in the
Integration Test 004 campaign. Hold `USER` while resetting exactly one board
to latch the base-station role. Its `SysGpsRed` LED remains solid; normal nodes
leave that LED off. Role selection is sampled once after a 50 ms boot delay and
cannot change until reset.

The checked-in default uses LoRa SF7, 868.000 MHz, 125 kHz bandwidth, coding
rate 4/5, a 12-symbol preamble, sync word `0x12`, and 14 dBm. Confirm that the
fitted Ebyte E80 variant, antenna, channel, power, and duty-cycle policy are
legal at the campaign location before transmitting.

Each 60-second UTC epoch contains a 40-second active window followed by 20 idle
seconds. Presence uses one deterministic one-second slot in seconds 0–20;
heartbeat uses an independently derived slot in seconds 20–40. The base
station chains bounded receive jobs
around its own transmissions. Nodes scan the complete active window during
bootstrap, after time degradation or base expiry, and every fifth epoch; other
epochs merge guarded windows predicted from discovered peers.

Schedule version 3 derives a separate deterministic permutation of all 20
slots for every node and frame purpose. During each aligned 20-epoch block a
node visits every slot exactly once; the following block receives a newly
derived permutation. This removes the previous four-epoch lockstep pattern.

Local indications are deliberately short and best-effort. Boot plays an
ascending three-note sound, first GPS lock plays one confirmation tone, and
full UTC frequency calibration plays two ascending tones. Every decoded
heartbeat plays a high packet tone and every decoded presence advert a lower
tone. Accepted GPS/PPS anchors flash `SysGpsGreen`; TX flashes `SysMainRed`
(and retains the original `SysSdBlue` pulse), while RX flashes
`SysMainGreen`. A fatal error still latches `SysMainRed` solid, and the
base-station role still latches `SysGpsRed` solid.

All radio access passes through `RadioService`. Radio producers submit bounded
jobs and process bounded events. Diagnostics first enter a separate bounded
queue and are drained to the Logging and Storage Services, so SD writes cannot
block radio re-arming. Every persisted record includes node/boot identity,
monotonic time, UTC when usable, UTC status, and uncertainty. Any diagnostic
or logger drop is counted and makes that run's persistent record incomplete.
Long records are assigned a record number and split into reconstructable parts
instead of being truncated. Each epoch also persists a neighbour-table header
and one bounded record per entry.

Build and host checks:

```text
cargo test -p integration-test-004-radio-heartbeat --lib --target <host-target>
cargo build -p integration-test-004-radio-heartbeat --release
```

The workspace defaults to `thumbv8m.main-none-eabihf`, so ordinary Cargo builds
place the board firmware under `target/thumbv8m.main-none-eabihf/release`; the
Embassy package metadata requests
`out/integrationtests/integration004_radioheartbeat` from tooling that supports
artifact directories. The default features are `lora,channel-868`; the build
rejects mixed modulation or channel selections.

Use at least two boards with exFAT-formatted SD cards and GPS antennas. Run for
at least ten complete usable epochs. Preserve every board's `/syslog.txt` for
correlation and reject a run if its summaries report diagnostic drops, logger
drops/truncation, write failures, unexplained scheduling misses, or incomplete
storage availability. Follow the full procedure and acceptance criteria in
`ADR/prompts/tests/integration/integrationtest004.md`.
