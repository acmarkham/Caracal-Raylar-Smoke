# L4 audio overrun recovery — 2026-11-27

The connected Evo has no GPS. Tests and the continuous image use `local-time`:
GPS stays off, timestamps are synthetic, and UTC/location remain unknown.
The recovery decision is recorded in
[ADR-L4-017](../../SpecAndDesign/Platform/L4/ADR-L4-017-overrun-silence-recovery.md).

## Behavior

The four-channel L4 recorder explicitly opts into replacing measured forward
sample loss with zero PCM. Both the reference FLAC and the three CRCL lanes
advance by the same number of frames. File boundaries and the sample clock do
not move. FLAC metadata marks the policy; syslog records substituted source
frame ranges and cumulative recovery counters. The CRCL schema is unchanged.

DMA ring overwrites rejoin the next common complete bank without restarting
DFSDM. Source retention exhaustion advances to retained PCM and fills the
overwritten prefix. The work and storage remain bounded by existing codec and
source buffers. Actual storage errors/card removal and unquantifiable capture
faults still latch red/error beeps. This does not retry a possibly partial write.
The legacy mono reference modes retain their previous strict acquisition policy.

## Host and U5 checks

- All 46 audio-source, paired-recorder, FLAC and CRCL host library tests pass;
  the 15 L4 library tests pass too.
- New cases include loss before the first read, a partial codec block, a gap
  crossing two segment boundaries, and further production overruns while silence
  is being encoded. Unknown hardware epoch resets still fail.
- Twelve gap fixture pairs decode successfully. FFmpeg and Xiph FLAC recover
  exactly the expected reference PCM. The CRCL decoder confirms zero PCM in all
  four lanes over each missing interval, retained audio at the correct position,
  exact frame counts and valid completion records.
- All six CRCL decoder regression tests pass. The L4 default and laboratory
  builds and separate U5 Aardwolf, audio-recorder and CRCL service builds pass.
- All 24 pre-existing six-channel fixture files match the saved SHA-256
  baseline byte for byte. U5 does not opt into the new policy; its existing
  strict tests and golden outputs are unchanged. No U5 hardware is connected.

Repeat fixture checks:

```powershell
rtk proxy python apps/aardwolf_l4/tools/verify_gap_fixtures.py --generate
```

The optional `--golden <manifest.json>` also compares existing six-channel
fixture hashes. The local baseline is `target/l4-quad/fixtures-before.json`.

## Hardware qualification

`bench-glitches` selects the finite two-minute, fake-GPS codec bench. It pauses
the DMA consumer for 100 ms after three recording seconds and delays an audio
append by 1500 ms at 59.75 recording seconds. DMA continues producing throughout
both delays; the append delay intentionally straddles the first rotation. This
is injected sink backpressure, not a claim to have induced a particular card's
internal wear-levelling behavior. The bench requires nonzero DMA recovery and
reader padding, closes all three streams, then freshly mounts the card and
hashes every recorded byte and the newly appended syslog.

```powershell
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --locked --offline --features bench-glitches
rtk proxy python apps/aardwolf_l4/tools/capture.py glitches --mode bench --timeout-seconds 500
rtk proxy python apps/aardwolf_l4/tools/verify_quad_fixture.py apps/aardwolf_l4/evidence/glitches.log --flac target/flac-tools/flac-1.5.0-win/Win64/flac.exe
```

Local evidence is in `evidence/20261127-gaps/`. The first exploratory run,
`glitches-01`, recovered a four-lane HAL DMA overrun and substituted 1664 frames
(104 ms). It also handled a 1502468 us append across the first file boundary;
both pairs closed with exactly 960000 frames per lane, whole-file readback
passed, and the full active CPU window measured 59.5%, with no log errors.

`glitches-02` was an exploratory diagnostics build. Its attempted extra CRCL
capture-configuration key was rejected by the independent decoder, so it is
not accepted format evidence. Its test files (`aud_249119940420.*` and
`aud_249119940480.*`) are retained as evidence, not deployment recordings.
That extension was removed; the reference FLAC carries the policy marker and
the final CRCL metadata keeps the established schema.

The accepted final run is `glitches-03`:

- ELF SHA-256:
  `1fbbadee2e2f2390dbb488d4329b66f09031b911bd2d429dbc10d34a5c8a4f87`.
- Forced HAL overrun on all four lanes recovered at 39.212 s. The source
  inserted 1664 zero frames starting at source frame 48101. The clock and
  filters kept running. The guarded elapsed-bank calculation is also tested
  against coalesced-counter scenarios on the host; IRQ masking itself was not
  injected on hardware.
- The delayed append measured 1500089 us. The original minute boundary at
  96.104 s fell inside that wait. The first pair finished at 97.448 s with
  960000 frames per lane; its intended time span remained one minute. The second
  pair also finished with 960000 frames per lane.
- Fresh-mount readback matched all four complete file hashes. Syslog readback
  matched its hash and sequence numbers, with 15 checkpoints during audio and
  no lost/truncated/failed log entries. It explicitly contains the DMA interval
  and reader-padding ranges on both sides of the minute boundary.
- CPU was 61.1% in the complete active 60-second window. The startup window
  (28.0%) includes waiting for the synthetic minute and is excluded. This is
  a workload measurement, not a controlled codec-cost comparison.
- The existing first-block microphone fixture also passes independent Xiph,
  FFmpeg and CRCL checks with the final header schema. Exact synthesized-gap
  waveform decoding is covered by the host fixtures; the hardware bench reads
  back complete file hashes and frame counts rather than exporting entire SD
  recordings over RTT.
- Stack guard remained intact; final high-water bound was 29324/32768 bytes.
  The monitor disconnected cleanly after the finite test completed.

The frozen ELF, RTT log, capture metadata, independent decode report and
`glitches-03.summary.json` are retained in the evidence directory.

The continuous firmware must be rebuilt without `bench-glitches`/`bench-codec`
before leaving the board recording:

```powershell
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --locked --offline --features local-time
rtk proxy python apps/aardwolf_l4/tools/capture.py continuous --mode continuous
```

## Image left running

The continuous `local-time` image, without either bench feature, was flashed and
verified through two additional rotations (`continuous-01`). Both pairs closed
with 960000 frames per lane; the third pair opened before clean monitor detach.
No injected pauses, DMA recovery warnings or latched faults appeared. Maximum
source backlog observed was 6036/6144 scalars and an actual SD append reached
53969 us. CPU was 62.0% in the complete active window, with zero logging drops,
truncations or errors. Stack guard remained intact (28896/32768 bound observed).
The board is left recording with fake GPS, and the standard build output is
this continuous profile rather than the fault-injection image.
