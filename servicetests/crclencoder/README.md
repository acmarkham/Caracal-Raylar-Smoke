# CRCL target and desktop validation

The default firmware captures six real microphones using the production MDF driver, exercises 6/8/10/12 bits with direct, temporal and automatic Rice selection, checks the unchanged MIC1 digest and record CRCs, and tests stopping/restarting capture. The allocator remains uninitialized; workspaces are statically allocated.

```powershell
rtk proxy powershell -ExecutionPolicy Bypass -File scripts/firmware.ps1 -Package service-test-crclencoder -MonitorSeconds 20 -Until 'CRCL TARGET PASS'
```

Add `sd-recording` to record and finalize one matching minute of mono 32-bit FLAC plus CRCL on the inserted card. This profile uses GPS UTC and stops its acquisition wait after 180 seconds:

```powershell
rtk proxy powershell -ExecutionPolicy Bypass -Command "& ./scripts/firmware.ps1 -Package service-test-crclencoder -MonitorSeconds 300 -Until 'CRCL SD TARGET PASS' -CargoArgs @('--features','sd-recording')"
```

For a bench without GPS reception, use `laboratory-time`, which includes `sd-recording`. It injects an explicitly logged Laboratory anchor through the Time Service. The anchor and file names vary with firmware build time; this profile does not validate GPS time and is unavailable in Aardwolf:

```powershell
rtk proxy powershell -ExecutionPolicy Bypass -Command "& ./scripts/firmware.ps1 -Package service-test-crclencoder -MonitorSeconds 240 -Until 'CRCL SD TARGET PASS' -CargoArgs @('--features','laboratory-time')"
```

Both profiles preserve existing card files, create the pair exclusively, check space for both members, and stop after successful finalization. A repeated filename fails instead of overwriting prior data. The test uses no radio or persistent logging task; Aardwolf concurrency and deployment energy require a separate soak.

Allow enough wall time for the next minute boundary, a full captured minute, flashing and debugger overhead. Target timestamps may advance more slowly than the host monitoring clock while the debugger is attached.

Generate paired host fixtures and run the independent decoder checks:

```powershell
$env:RAYLAR_CRCL_TEST_OUTPUT = (Join-Path (Get-Location) 'target/crcl-fixtures')
rtk cargo test -p raylar-audio-recorder-service --target x86_64-pc-windows-msvc multichannel
rtk proxy python scripts/test_crcl_decode.py
```

Use an absolute fixture path because Cargo runs tests from the package directory. The tests cover WAV and FLAC reference equality, partial tails, rotation, source loss, setup/write/rewrite/close failures, wrong references, truncated tails and corruption.

The final 2026-10-14 STM32U595 board run passed all 12 live codec profiles and capture restart. The paired SD test then finalized 960,000 aligned frames at laboratory UTC `1792046700`, using 1024-frame/10-bit/direct CRCL blocks and 2048-sample/fixed-order-1 FLAC. Maximum combined codec time was 38,051 microseconds, write time 20,681 microseconds, and source backlog 9,918 interleaved scalar samples. The test used a 96,000-scalar-sample ring. These are workload observations, not a worst-case proof or energy result.

The GPS profile did not acquire valid UTC in the three-minute observation window. Acoustic phase/delay calibration, real beamforming/angle-estimation degradation limits, independent desktop decoding of card-extracted recordings, and sustained radio/logging concurrency remain deployment checks. The [build record](../../firmware-builds/service-test-crclencoder.json) identifies the target test image; bounded RTT captures are in `.probe-rs-logs/`.
