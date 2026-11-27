# ADR-L4-017: Preserve the audio timeline through recoverable overruns

Date: 2026-11-27

Status: Accepted

Supersedes the fatal-on-every-audio-loss policy in
[ADR-L4-016](ADR-L4-016-four-channel-flac-crcl.md) for the four-channel L4 recorder.

## Context

The L476 has little audio retention RAM. A functioning SD card can occasionally
stall during internal maintenance longer than that buffer can retain. Stopping
the whole recorder on a transient overrun loses the remainder of the deployment.
Shortening the recording by deleting lost samples would corrupt its timing and
the relationship between the mono FLAC and the three-channel CRCL companion.

## Decision

L4 explicitly enables silence substitution in the shared paired recorder. The
shared default remains strict: close the continuous prefix on a discontinuity.
U5 does not enable the new policy, and its configuration and existing regression
tests remain unchanged. The policy is selected before recording starts.

For a slow consumer, AudioSource keeps producing and advances lagging readers
to the oldest retained whole frame. The recorder compares that absolute source
position with its expected next position. It emits exactly the missing number
of zero frames, in bounded codec blocks, before consuming the retained data.
The same zero PCM feeds the reference FLAC and all three CRCL lanes. Partial
blocks, file rotation, first-source-frame identity, timestamps and time anchors
therefore advance together. A gap can span multiple file boundaries, including
an entirely silent segment, without needing an allocation proportional to its
length. New losses during catch-up extend the gap using the same absolute cursor.

For a DMA ring overwrite, discard the uncertain four-lane read together. Keep
the DFSDM clock and filters running. TIM5 and DFSDM share the PLL, allowing the
elapsed bank number to be reconstructed from the original bank epoch, including
when half/full interrupt flags coalesce. Check DMA position parity and require
all four lanes to be in the same bank, at least 16 frames away from a boundary.
In a bounded critical section, clear all four HAL readers and stale completion
flags and rebase their interrupt counters. Alignment makes every reader skip
the partial bank and resume at the same following complete bank. Publish zeros
for the intervening source frames before publishing that bank's PCM. Resume
real data on the original sample-count timeline; do not restart the sample clock.

Recoverable loss does not latch the red LED or error buzzer. Failed SD writes,
card removal, peripheral transfer errors, unavailable DMA service for a second,
failure to rejoin a consistent bank within a second, backwards time or an
unquantifiable hardware epoch reset still fail through the existing fault path.
There is no retry of an SD operation whose partial completion is unknown. This
policy tolerates delayed successful I/O; it cannot make a removed or failed card
writable. Permanent throughput below the encoder output rate still loses audio,
which becomes increasing silence rather than unbounded RAM growth.

## Policy overrides

This is an explicit L4 exception to fatal-on-overrun recording. Only measured
forward audio loss is eligible for substitution. U5 retains its existing policy;
SD removal, failed writes and unrecoverable capture faults retain the fault UI.

## Data quality and diagnostics

The paired reference FLAC comments mark `gap_policy=silence_all_channels`.
CRCL retains its existing header schema and identifies that same reference file.
Syslog records substituted source-frame ranges and cumulative DMA rejoin,
DMA-silence and reader-padding counters. DMA silence and reader padding are
separate measurements: reader padding can include overwritten DMA zeros, so
adding those two counters does not yield a unique lost-frame total. Ordinary
silent acoustic input is not counted as substituted audio.

`RecorderProgress.dropped_samples` reports interleaved scalars replaced during
reader catch-up; `pcm_samples` includes those output scalars. DMA zeros already
retained in AudioSource are ordinary PCM to the consumer and have their own
producer counter. Existing CPU, queue-health and SD timing diagnostics continue.

## Validation

Host tests retain the strict U5 cases and add explicit opt-in cases for loss
before the first read, a partially collected block, two crossed segment
boundaries, exact recovered waveform position, and an unquantifiable epoch
reset that must still fail. Independent FFmpeg and Xiph FLAC decoding checks
the reference waveform; the CRCL decoder checks all-lane silence, retained side
channels, frame counts, sequence/completion records and reference alignment.

The `bench-glitches` L4 feature selects the finite fake-GPS codec test and
injects a 100 ms DMA-consumer pause and a 1500 ms audio-sink delay across a
minute boundary. The sink delay simulates consumer backpressure, without
changing the card firmware or deliberately corrupting an SD transfer. The test
must observe both DMA recovery and reader padding, then close and read back both
minute-long pairs and syslog through a fresh filesystem mount. It also retains
the existing first-block codec fixture checks. Fault injection is absent from
the continuous image left on the board.

See the [hardware validation record](../../../apps/aardwolf_l4/VALIDATION-GAPS-20261127.md).
