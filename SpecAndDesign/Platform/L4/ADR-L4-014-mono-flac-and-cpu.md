# ADR-L4-014: Mono FLAC recording and CPU comparison on Evo

Date: 2026-11-26. Status: implemented; hardware results are recorded in
[the FLAC validation record](../../../apps/aardwolf_l4/VALIDATION-FLAC-20261126.md).
This replaces WAV as the normal L4 Aardwolf recording format from
[L4-013](ADR-L4-013-fault-ui-telemetry-audio.md).

## Decision

Use the existing upstream Audio Recorder Service and allocation-free FLAC
encoder. The current U5 Aardwolf [profile](../../../apps/aardwolf/config.rs)
uses fixed predictor order 1; retain that selection on L4. L4 records **mono,
16 kHz, signed 24-bit FLAC**, with **1024-sample blocks** (64 ms), in sixty-second
segments. U5 retains its existing 2048-sample / 32-bit configuration. No shared
service, encoder, U5 manifest, test or source change is required.

The DFSDM clock/filter settings remain 3.2 MHz PDM, Sinc4/FOSR200/IOSR1,
right shift 8. For FLAC publish the arithmetic-right-shifted raw result, signed
24-bit right-justified in `i32`. Do not feed WAV's left-aligned PCM32 values to
a 24-bit encoder. The opt-in `wav` leaf feature retains that PCM32 reference
path; it is not part of the normal L4 build.

Use `new_flac_with_buffers` with static 1024-element PCM and 4128-byte output
workspace, avoiding large constructor copies on the SRAM2 stack. The existing
4096-sample AudioSource retention and 512-sample DMA halves are initially kept.
They must pass a sustained target check before the profile is accepted.

Count consumed and published **samples**, including the trimmed first packet,
rather than assuming a recorder call consumes a whole DMA half. The FLAC
service collects partial packets into its block workspace. Retention checks
must still detect every lost sample; do not silently reset a failed DMA ring
or discard old PCM to obtain a passing result.

## Storage lifecycle

Files use the existing exclusive hourly path, `/<hour>/aud_<utc>.flac`, with
concurrent `/syslog.txt`. Keep the 100 MB log reserve and include bounded header
and per-frame overhead in the worst-case audio allocation budget. Reserve is
based on a conservative uncompressed size, not an assumed compression ratio.

The shared FLAC service normally opens its file only after the first PCM
arrives. L4's first bitmap/directory allocation previously exceeded its capture
deadline. Preserve the L4 early-allocation behavior: the storage adapter opens
the first file ahead of the planned UTC boundary, then returns that prepared
handle when the shared service asks for it. Match its UTC and suffix exactly.
This is local to the L4 adapter; U5's deferred materialization is unchanged.
A reset before capture can consequently leave an empty prepared L4 file.

On closure the service rewrites the 34-byte STREAMINFO at offset 8 with actual
sample/frame information. Require 960000 samples, 16 kHz, mono, 24-bit format
and the configured block size. Track actual variable file length. To verify
every byte without retaining an entire file, compare a rolling content checksum
with STREAMINFO normalized to zero, plus a separate checksum of the final
STREAMINFO bytes. Fresh-mount readback verifies both regions and file length.
Label the normalized checksum explicitly rather than describing it as the hash
of the finalized file. Retain the encoder's frame CRCs and deliberately absent
STREAMINFO MD5; do not invent an MD5 value.

Existing latched SD/acquisition fault indication, buzzer priority, syslog
checkpoints and real GPS/Time Service policy remain active.

## CPU measurement and validation

Use the same one-minute executor-idle CPU measurement as the WAV baseline.
Only compare windows wholly inside active recording. Search, initial file
preparation, UTC gating, stopped-writer verification and fixture export are not
representative recording windows. The measurement includes application work
and storage handling, but does not attribute interrupt work before an executor
wake. Keep that limitation in the comparison.

Record maximum encoder-frame wall time (including its cooperative yields),
storage append latency, buffer backlog, DMA errors and log drops. Trace hooks
also observe maximum executor poll duration to diagnose deadline failures;
they do not change the CPU idle accounting. Reset that peak at first published
PCM so filesystem preparation is not mistaken for an acquisition-time stall.

The `bench-codec` feature stops after two complete minute files. It closes both
writers, stops acquisition, remounts read-only and checks all file contents and
syslog, including CPU records. It does not add an acoustic test tone or shorten
the production GPS policy. The old `bench-audio-ui` WAV test is selected with
`wav,bench-audio-ui`.

For an independent hardware encoder check, retain the first native FLAC frame
and its metadata in the existing readback workspace. After acquisition stops,
finalize a standalone 1024-sample fixture by patching only STREAMINFO's counts
and sizes; the encoded frame itself is unchanged. Export it in bounded RTT
chunks, then test its CRCs and decode it using Xiph FLAC and FFmpeg. Both decoded
integer streams must match the checksum of the first 1024 microphone samples
recorded before encoding. This is independent decoding of a real target block,
not a claim that every sample of the two full files was independently decoded.

Keep failed development runs and distinguish them from accepted recording
measurements. Build the original U5 matrix and compare its images/features and
portable outcomes to the clean baseline. Leave the normal FLAC image recording
after its own rotation check so the user can listen to the SD files.

## Policy overrides

This replaces the L4-012 WAV-only format exception. The smaller FLAC block and
24-bit source precision are explicit L4 resource choices. All other L4-012
capability/GPS exceptions and the shared UI/storage policies remain in effect.
