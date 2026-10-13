# FLAC encoder validation

The firmware test writes twelve deterministic minute files at 16 kHz: all four
bit depths and all three block sizes. It leaves the global allocator empty and
reports the maximum synchronous encoding time alongside each frame's real-time
budget. It exercises SD materialization and STREAMINFO rewriting. Aardwolf is
the capture/radio/logging concurrency test; this synthetic test does not operate
the microphone.

Build with `rtk cargo build -p service-test-flacencoder --release`. Flash and run
through the normal probe runner only when hardware testing is intended. The
first sample of every file uses the same xorshift32 seed `0x12345678`. Update the
state with shifts 13, 17 and 5, interpret it as signed 32-bit, then arithmetic
shift right by `32 - depth`. Compare 960,000 decoded samples to that sequence.

Host validation uses the allocation-free service with host-only fixture tools:

```powershell
rtk cargo test -p raylar-flac-encoder-service --target x86_64-pc-windows-msvc
rtk cargo run -p raylar-flac-encoder-service --example fixtures --release --target x86_64-pc-windows-msvc -- target/flac-fixtures
rtk proxy python servicetests/flacencoder/verify.py target/flac-fixtures --flac path/to/flac.exe
```

The verifier requires FFmpeg and the Xiph FLAC command-line tool. It compares
both decoders' integer output to the expected PCM and uses Xiph test mode to
validate frame CRCs. A warning about the deliberately absent MD5 is acceptable.
Host fixture files and downloaded verification tools belong under `target/`.
