# Integration Test 003: radio range

This firmware runs the same role on both Raylar boards. It waits for GPS/PPS
UTC synchronization and a valid filtered location, then listens continuously
between independently jittered transmissions. Received packets are reported
over RTT, written to `/syslog.txt`, and indicated with a short green LED flash
and buzzer beep.

An exFAT-formatted microSD card and a GPS antenna with a view of the sky are
required. Radio activity begins once the Time Service has accepted a GPS/PPS
UTC anchor and can map the current monotonic time to UTC, and the Location
Service has a valid estimate. It does not wait for long-term frequency
calibration to lock.

## Campaign configuration

Edit `radio_test_config.rs` before building both boards. In particular, set the
local coordinate origin and its east/west scale for the campaign area. The
checked-in defaults are centred at 52 N, 0 E and accept positions within 20 km
of that origin. Packet coordinates are signed east/north offsets quantized to
10 m. `REMOTE_POSITION_ERROR_BUDGET_METRES` accounts for transmitter GPS error
that is not carried in the compact packet; the logged distance uncertainty is
therefore an estimate, not surveyed accuracy.

Increment `CONFIGURATION_ID` whenever an interoperability-affecting radio,
packet, or coordinate setting changes so mismatched devices reject one
another's packets explicitly.

The default build is LoRa at 868 MHz:

```text
cargo build -p integration-test-003-radio-range --release
cargo build -p integration-test-003-radio-range --release --no-default-features --features gfsk,channel-915
cargo build -p integration-test-003-radio-range --release --no-default-features --features lora,channel-2445
```

Select exactly one modulation and one channel feature. Both devices must use
the same source configuration and configuration ID. Hardware support does not
by itself establish that the selected frequency or transmit power is legal in
the test region.

The 16-bit sequence is never wrapped. After packet 65535 is attempted, TX is
disabled for the rest of that boot while reception and logging continue. This
avoids making a wrapped value appear monotonic during a long range campaign.
