# Integration Test 004 updated syslog analysis

Source: `E:\syslog.txt`

## Run summary

- One boot session: node `0xcc1455b0`, boot `0xc323a602`, role `BaseStation`.
- Firmware: `e73674e49d8093b0e22dbb9680b3f6751b5e296b-dirty`; schedule version 2.
- Duration: 22.39 minutes; 22 finished epochs.
- Local TX completions: 44 (22 presence, 22 heartbeat).
- Valid peer receptions: 39 (17 presence, 22 heartbeat).
- Reception rates: presence 77.3%, heartbeat 100.0%, overall 88.6%.
- Missing expected receptions in finished epochs: 5 (29853275:Presence, 29853279:Presence, 29853283:Presence, 29853287:Presence, 29853291:Presence).
- Every miss was presence in peer slot(s) [3]; slot collisions with local TX: 0.
- Mean valid-frame signal quality: -47.5 dBm RSSI, 15.3 dB SNR.
- Final neighbour count: 1; neighbour snapshots: 22.
- GPS location became valid: 8.0 s after boot; UTC frequency calibration lock: 617.0 s after boot.
- UTC remained synchronized after acquisition; logged post-acquisition uncertainty ranged from 100 to 661 us.
- Final GPS/PPS anchor counters: 754 accepted, 152 rejected.

## Logging integrity

- Physical lines: 1206; logical records: 1162.
- Multipart records: 44; maximum parts: 2.
- Missing physical sequences: 0; missing logical record sequences: 0; incomplete multipart records: 0.
- Final diagnostic/logger drop/truncation counters: 0/0/0 (`clean`).

## Radio anomalies

- RX jobs reported as `MissedSlot`: 39; 38 align within 10 ms of local TX and one is the startup RX replacement.
- Unsupported-version frames: 6.
- Packet-rejected events: 9.
- Final malformed/unsupported/radio-error counters: 0/6/0.
- Scheduler conflicts and radio queue drops: 0/0.

## Plots

- `packet_timeline.png`: packet TX/RX plus rejected/unsupported events and RX displacement.
- `epoch_reception.png`: reception completeness for every finished epoch.
- `slot_map.png`: scheduled local slots and received peer rendezvous slots across the 20/20/20 layout.
- `radio_time_quality.png`: RSSI/SNR, UTC uncertainty/calibration lock, satellites and HDOP.
