# Overnight recording recovery changes — 16 October 2026

These changes address the recording interruptions and timestamp drift found in `analysis/aardwolf_overnight/20261016/report.md`. Aardwolf retains its default hexaphonic configuration: MIC1 lossless FLAC32, MIC2–6 cheap CRCL v2, 1,024-frame blocks, 10-bit retained precision and a minimum six-LSB discard. The container format is unchanged.

## Recovery and storage

- Microphone packets continue entering `AudioSource` while the recorder waits for valid UTC or its next recording start. Deliberately waiting no longer removes time from the sample-count timeline. Reader restart clears intentionally accumulated backlog/loss notifications.
- Capture-origin changes and missed publications always report a source discontinuity, including while the recorder is stopped. A restart cannot leave a stale timestamp epoch behind.
- The latest producer timestamp is compared with the source's derived timeline. A discrepancy above 200 ms causes a warning, source re-anchoring and recorder restart. Five seconds without encoded output also causes a warning and restart, instead of prolonged silent seeking.
- The exFAT free-space scan is primed before microphone capture starts. Successfully created folder paths are cached by the storage service to avoid redundant directory creation traversals for every paired file.
- SD buffering remains **16 KiB per stream**, with final partial buffers written on close. No additional PCM buffer or pre-created pair is allocated. Worst-case SD stalls can still exceed the approximately 1.333-second hexaphonic ring; the new measurements identify those operations for further optimization.
- Recording metadata now includes the build's firmware Git identity, including its dirty marker when applicable.

## Capture timing guard

The DMA guard checks successive completion intervals, plus all six DMA counters and MDF overrun status. It no longer rejects capture solely because oscillator drift has accumulated more than 25 ms against the nominal sample clock over many hours. A collapsed/missed completion interval is still rejected.

Cumulative nominal clock phase remains available as `nominal_clock_error_ticks`. Detailed fault reasons are latched separately from the latest frame so that a quick restart cannot overwrite the diagnostic before it is logged. `capture fault reasons` is a bitmask: 1 = completion interval; 2 = DMA counts; 4 = MDF overrun. Multiple bits can be set.

## Indicators and syslog

**Blue pulses indicate encoded audio progress**, rather than microphone forwarding alone. An active recorder with no encoded progress for five seconds raises the main red warning independently of the recorder task's progress. Audio warnings clear after approximately two seconds of successful encoded frames following recovery; historical loss counters remain available. Severe storage flags continue to hold red on.

GPS reception is still required to establish normal recording time. Before valid UTC, audio files and blue encoding activity are absent; there is no laboratory clock in the normal Aardwolf build.

New records include:

| Record | Interpretation |
|---|---|
| `audio health` | Encoded frames, backend append bytes, individually closed files, dropped scalar samples, event counts and source timestamp error |
| `audio capture` | DMA sequence, capture origin/completion ticks, cumulative nominal clock error and forwarded packet count |
| `audio lifecycle` | Maximum complete open/finish/final-reference-flush durations, ring backlog before/after and latest segment's first source frame/ticks |
| `storage timing` | Call count and maximum mutex wait, backend execution and total wall time, separately for admission, both opens, append, rewrite, both closes and log persistence |
| `capture fault` | Latched reason mask, DMA deltas, MDF status, completion interval and nominal clock error |

Timing maxima are reported for ten-second windows; lifecycle maxima survive segment rotation until collected. Backend execution includes directory/SD operations and executor scheduling. Separately reported maxima may come from different calls, so do not add them to infer a particular operation's duration.

`encoded_frames` is encoder progress, not a durability guarantee. `backend_bytes` excludes bytes still buffered in RAM, log traffic and in-place header rewrites. Metadata/allocation durability still requires a successful close. `closed_files` counts individual files: a completed paired minute normally adds two. Counters wrap modulo 2^32; derive deltas accordingly for multi-day tests.

`dropped_scalars` contains source-reported missing scalar samples; six scalars form one hexaphonic frame. `errors` counts recorder/error events separately, `capture_restarts` counts origin changes and `missing_capture_frames` counts missed DMA publication frames. Hardware-gap estimates can be lower bounds. Intentional restart waits are not part of the dropped-sample count.

Discontinuities and stalls now use WARN; write failures use ERROR. Historical errors alone no longer permanently latch the red LED.

## Validation

Host checks passed:

- Audio source, recorder and storage suites: **27 tests**.
- Microphone driver tests: **6 tests**, including 24 simulated hours of oscillator drift and rejection of a collapsed DMA period.
- New recorder regressions cover repeated minute restarts with continuous capture, a forced slow-open overrun, common-prefix finalization and capture-origin change during a pause.

The broader drivers suite has an unrelated failure in the unchanged `gps::framer::tests::frames_sentence` fixture. Targeted microphone tests pass with the normal host time/defmt support supplied by the recorder test package.

The connected STM32U595 passed a GPS-independent recovery test:

1. Capture and encode real input from all six microphones, including capture stop/start checks.
2. Inject a two-second file-open stall, exceeding the production ring retention. The recorder reports the expected overrun and closes the aligned prefix.
3. Pause recording for eight seconds while capture continues, then record a five-second pair.
4. Repeat the pause/restart and five-second recording.

Both recovered recordings encoded **80,000 frames**, with **zero source timestamp error**. Measured recovered opens were approximately 59–62 ms and finishes 19–21 ms. The deliberately stalled open took approximately 2.617 s. This is a short recovery test, not a substitute for the next overnight run or a proof of worst-case SD latency.

Reproduce the target test with:

```powershell
rtk proxy powershell -NoProfile -ExecutionPolicy Bypass -File scripts/test_audio_recovery.ps1
```

The test uses a named laboratory UTC source and writes test audio under its laboratory timestamps. Restore normal Aardwolf afterwards. The flashing script now starts the monitoring window on target output, allowing up to three minutes for startup/programming when monitoring is bounded. It reports absent target output as unverified startup and handles an empty RTT log without an exception.

Normal Aardwolf was restored and its boot and SD initialization were observed. It subsequently acquired real GPS/PPS time in the test location. A final read-only SWD health snapshot showed **four closed audio files (two pairs), 2,191,359 encoded frames and 13,669,144 backend bytes**, with **zero audio errors, dropped scalars, capture restarts, missing capture frames, storage flags and active audio warnings**. Counters were read sequentially while recording continued. The normal firmware has no laboratory-time feature.

For the long-duration run, check for approximately one complete paired minute per minute, steady encoded/backend progress, no repeated WARN discontinuities, and no growing source timestamp error. Blue can be active even when the current pair has not yet finalized, so validate the files and counters as well as the LEDs.
