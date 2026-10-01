# Radio driver test

This firmware continuously receives and reports packets, RSSI/SNR, and local
Embassy system-tick timestamps. At a uniformly jittered interval from 3 to 17
seconds (10 seconds average), it transmits a 12-byte packet containing the
device's 64-bit serial ID and a big-endian 32-bit counter.

Select exactly one modulation and one channel. The default is LoRa at 868 MHz:

```text
cargo build -p driver-test-radio --release
cargo build -p driver-test-radio --release --no-default-features --features gfsk,channel-915
cargo build -p driver-test-radio --release --no-default-features --features lora,channel-2445
```

Both communicating boards must use the same modulation and channel features.
Timing is local monotonic time only; this test does not initialize GPS or use
the time service.
