# Aardwolf firmware

This app implements the [Aardwolf ADR](../../ADR/prompts/apps/app/_aardwolf/ADR%20Aardwolf%20GPS-Calibrated%20Audio%20Logger.md) for Raylar v1.0. It starts in energy recovery, powers GPS and the microphone only after reported battery SOC exceeds 20%, and pauses audio, GPS, and radio when SOC falls below 10%. Heartbeats resume at the next complete UTC half-hour epoch after recovery.

Build with `cargo build -p aardwolf --release --target thumbv8m.main-none-eabihf`. The binary is `target/thumbv8m.main-none-eabihf/release/aardwolf`. No automatic flashing is part of the build.

`bench-external-power` bypasses the boot SOC gate for externally powered bench work, while still applying the below-10% pause once a valid SOC is reported. It must stay disabled for the solar run. `bench-rx-beep` enables a short audible indication for each valid heartbeat received; field builds leave it disabled.

The fixed run configuration is in `config.rs`: 868.1 MHz and 2441 MHz, twelve ordered profiles, a 30-minute UTC epoch, and 16-byte V4 heartbeats. Nodes must run the same network and schedule version. Slot permutation is based on NodeId, epoch and profile minute. Minutes 4 and 12 use two-second slots. The radio sleeps for minutes 13–30 and throughout energy recovery.

Audio uses the mono 16 kHz reference microphone configuration and creates one WAV per UTC minute in hourly folders. A partial WAV is finalized with its actual sample count. A file is opened exclusively, so a repeated timestamp cannot append to or overwrite an earlier segment. Audio starts only when the card has room for the next complete minute plus the 100 MB log reserve. System and radio events are written to the normal system log when the card is available; a missing card leaves the radio and heartbeat task running with storage error bits set.

The firmware uses a deliberately empty global allocator. The normal exFAT mount, file lookup, create, append and flush paths use bounded storage. Any unexpected allocation fails, which makes heap use visible during bring-up. This and the seven-day data-integrity target require hardware verification.

Before a week-long campaign, verify RF packet-complete margins, GPS holdover timing, SD throughput, audio continuity while radio RX is active, the two SOC thresholds, and the card-full and card-absent cases on physical boards. Configure the antenna and regional EIRP/duty-cycle policy for the deployment; the checked-in firmware logs the antenna as `unknown` until that detail is supplied.
