# Raylar Radio Messaging Service — Phase I

This crate implements the Phase I boundary defined by
`ADR/prompts/services/radio/Service Radio Phase 1.md`.

## Fixed interoperability choices

- `NodeId`: 32-bit `IdentityDriver::serial_32`; deployment provisioning must
  check uniqueness.
- `BootId`: 32 random bits supplied by the platform TRNG.
- sequence: 16 bits, volatile, wrapping within one boot session.
- byte order: network/big-endian for every multi-byte wire field.
- common frame byte 0: four-bit wire version followed by four-bit frame type.
- common broadcast header: 12 bytes (`version/type`, flags, source, boot ID,
  sequence); directed data adds a four-byte destination.
- wire version: 1.
- rendezvous version: 1, using FNV-1a 64 over the ordered fields documented in
  `rendezvous.rs`. Its checked-in host vectors are compatibility contracts.
- default epoch: five minutes; default broadcast window: one minute; default
  slot: five seconds.
- default heartbeat repetitions: two, separated by at least two slots.
- default radio preparation guard: one second (deliberately conservative until
  LR1121 setup timing is characterised on hardware).
- neighbour expiry: fifteen minutes; replacement uses an expired oldest entry
  first, then the least recently heard entry.

`ChannelProfile::phase_one_eu868_bootstrap()` defines the repository's Phase I
common profile as 868.000 MHz, LoRa SF9, 125 kHz, CR 4/5, 14 dBm, sync word
`0x12`. Firmware must only select it after product radio policy confirms the
fitted Ebyte E80 variant, deployment region, power and duty-cycle limits.

## Ownership and overload behaviour

`RadioService` takes the radio driver by value. Protocol clients only receive a
cloneable `RadioHandle`, so they cannot invoke LR1121 operations directly.
Requests, results, state watchers, scheduler reservations, frames and neighbour
tables are all fixed-capacity.

`try_submit_tx` and `try_reserve_rx` report a full request queue immediately;
their async counterparts apply back-pressure. If the result-event queue fills,
the newest event is dropped and the saturating `queue_drops` statistic records
the loss. State remains available through the watch. Higher-priority
reservations may evict overlapping lower-priority reservations, whose clients
receive a conflict result when result capacity permits.

The service validates outgoing and incoming common frames before use. Unknown
versions/types and malformed traffic are counted and reported without panic.
Driver faults enter the driver's hardware-reset recovery path; an ordinary RX
window timeout does not reset the radio.

## Time and location

UTC is never estimated here. `HeartbeatProtocol` and `PresenceProtocol` convert
their UTC slots through `TimeState`, and `RadioRxJob::guarded_utc_window` widens
receive windows by local uncertainty, expected remote uncertainty, scheduling
uncertainty, propagation allowance and engineering margin. Tight scheduled
jobs are refused above the configured uncertainty threshold.

`Heartbeat::from_service_states` consumes `LocationState`; invalid location is
omitted instead of consulting GPS directly.
