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

Each 60-second UTC epoch contains a 20-second active window. Presence uses one
deterministic one-second slot in seconds 0–10; heartbeat uses an independently
derived slot in seconds 10–20. The base station chains bounded receive jobs
around its own transmissions. Nodes scan the complete active window during
bootstrap, after time degradation or base expiry, and every fifth epoch; other
epochs merge guarded windows predicted from discovered peers.

All radio access passes through `RadioService`. Radio producers submit bounded
jobs and process bounded events. Diagnostics first enter a separate bounded
queue and are drained to the Logging and Storage Services, so SD writes cannot
block radio re-arming. Every persisted record includes node/boot identity,
monotonic time, UTC when usable, UTC status, and uncertainty. Any diagnostic
or logger drop is counted and makes that run's persistent record incomplete.

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
