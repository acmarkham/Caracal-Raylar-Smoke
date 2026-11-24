# ADR-L4-003: Bounded L4 resources, audio and external GNSS

## Status

Proposed, 2026-11-23. Budgets are design ceilings and working allocations, not
measured firmware sizes or demonstrated performance.

## Context

The current [Aardwolf audio composition](../../../apps/aardwolf/audio.rs) allocates
127,998 interleaved `i32` samples in six-channel mode (511,992 bytes) or 128,000
samples in mono (512,000 bytes). Either ring exceeds the L4's total SRAM by
nearly four times, before DMA, tasks, storage or codecs.

The [microphone contract](../../../crates/drivers/src/mic_array/mod.rs) exposes
only `Mono` and `Hexaphonic`; its DMA resources allocate six channel buffers
even when fewer channels are active. Filter presets describe U5 MDF behavior.
The current [paired recorder](../../../crates/services/audiorecorder/src/multichannel/mod.rs)
also explicitly requires six channels. Changing a pin map or disabling the
radio cannot make that composition fit or create a four-channel pipeline.

The GPS driver already has portable serial/PPS/power contracts and NMEA parsing,
but [module command defaults](../../../crates/drivers/src/gps/types.rs) include
PMTK standby. Its STM32 adapter assumes PB9, TIM4 and a separate reset output.
Those assumptions do not describe an external u-blox on the Evo header.

## Decision

### Incremental feature profiles

| Profile | Intended composition | Admission condition |
| --- | --- | --- |
| L4 boot | Clocks, monotonic time, identity, RTT, LEDs/buzzer | Linked memory layout, recovery and timer checks pass. |
| L4 peripheral diagnostics | Separate SD, GNSS and microphone images | Each peripheral has independent evidence; diagnostics do not imply integrated fit. |
| L4 mono recorder | One verified microphone, bounded AudioSource, SD/storage/logging, Time/Location as needed | Full resource accounting and end-to-end sample continuity pass. |
| L4 four-channel recorder | Four named microphones, common clock, uncompressed PCM/WAV first, GPS timing and persistent log | Four-filter coherence, storage stalls and integrated timing pass. |
| Optional L4 compression / advanced timing | Shared FLAC or a separately approved multichannel codec; tighter PPS timing or GNSS duty cycling | Additional RAM, CPU, format and receiver-specific acceptance evidence. |

The initial recording rate is a **16 kHz design target**, subject to a verified
L4 clock/filter configuration. Record actual rate, filter settings, sample width,
scaling and channel identities. Begin with 32-bit PCM words to preserve the
existing internal representation; any narrower storage or quantization is an
explicit later quality/format decision. Rate or precision reductions must be
named profiles with recorded metadata, never automatic responses to overload.

No profile includes radio, accelerometer/magnetometer sources or their derived
features. Do not start charge-controller, SOC gating or deep sleep using the
U5 assumptions. A bench-powered profile can bring up acquisition while these
hardware capabilities remain unverified; field-power support is a separate gate.

### RAM and flash accounting

Account for `.data`, `.bss`, custom sections, alignment, DMA ownership, all async
task futures, queues, caches, RTT buffers, stack and any temporary storage.
Async futures consume static RAM as well as their poll-time stack. Report
initialized RAM in both its RAM occupancy and flash load image. No heap is
required for the recording path.

Initial integrated allocation envelope:

| Category | Working allowance |
| --- | ---: |
| Four-channel DMA ping-pong buffers | 8 KiB |
| Interleaved AudioSource ring | 24 KiB |
| SD/exFAT caches, write/directory workspace and file state | 24 KiB |
| GNSS, Time and Location resources | 12 KiB |
| Logging, identity, indicators and RTT buffers | 8 KiB |
| Other static state / task futures not charged above | 12 KiB |
| Main/interrupt stack allowance | 16 KiB |
| Uncommitted RAM margin | 24 KiB |
| **Total physical SRAM envelope** | **128 KiB** |

These allowances sum once: a task's stored local buffers must not be counted
again as separate statics. The table targets at most 88 KiB of static allocation
plus 16 KiB stack, preserving 24 KiB margin. It is a starting envelope to test,
not a reason to hide a larger measured storage or GNSS requirement.

Start bring-up within the **96 KiB SRAM1 region** until the generated memory
layout and SRAM2 initialization/access have been verified. Using the full table
requires explicit SRAM2 placement. Check the pinned Embassy/metapac generated
`memory.x`; do not assume its default RAM region includes both banks. Define
extra sections with correct startup zeroing/initialization and keep aliases
from overlapping allocations. Place DMA buffers in a bank/address proven
reachable by the relevant DMA controller; move other state to SRAM2 first.
Link-time checks must enforce each region separately, including stack space.

Target a normal flash load image no larger than **896 KiB**, leaving 128 KiB of
the 1 MiB device for growth. This margin is not an allocated bootloader or update
slot. A later bootloader/persistent configuration layout must subtract its exact
reservation before applying image limits. Count vectors, read-only data and RAM
initializers. Debug information in the host ELF is not device flash occupancy.

Emit a per-profile section/symbol size report and linker map. Measure stack high
water under interrupt/error/storage peaks, not only normal capture. If a
profile misses its allowance, first inspect duplication and optional resources,
then resize the L4 profile or explicitly revise this budget. Never reduce U5
buffers or accuracy settings to make the L4 image fit.

### Audio backend and admission

Implement DFSDM/DMA beneath a portable capture contract. Generalize channel
count and resource allocation narrowly, preserving the existing U5 six-channel
API/default behavior. A defaulted channel-capacity parameter or a parallel
resource type can avoid six physical DMA buffers in an L4 mono build. Keep
MDF-specific preset calculations inside the U5 adapter; derive L4 DFSDM clock,
oversampling, shift, passband and filter delay independently.

For the working DMA allowance, four channels times two halves times 256 samples
times four bytes equals 8,192 bytes. At 16 kHz, each half gives a **16 ms**
producer service interval. The 24 KiB `i32` ring stores 1,536 four-channel frames,
or **96 ms**. These are calculated capacities, not measured scheduling margins.

Use one shared microphone clock and synchronized filter starts. Publish a half
only when every active channel has the matching generation and completed half.
Check overruns and missed/duplicated DMA events. Do not assume the interrupt
from the last numbered channel proves every channel is ready. Keep interrupt
handlers bounded; encoding, formatting and filesystem work belong in tasks.

Preserve sample order, complete interleaved frames, sequence/loss counters and
first-sample timestamps. DMA completion time is not acquisition time: include
half length and filter delay, and characterize cross-channel skew. The legacy
option to insert sequence bits into PCM must not be carried into normal audio.

At four channels, 16 kHz and four-byte PCM, raw payload is **256,000 B/s** (about
244 KiB/s). A 96 ms ring cannot conceal a long SD stall. Measure worst observed
no-progress intervals during steady writes, flush, directory growth and segment
close/open, with GNSS and logging active. At admission, require
`retained_frames / actual_sample_rate > worst_no_progress_interval + scheduling_margin`,
and sufficient sustained throughput and producer scheduling headroom. Account
for other pending buffers separately; do not count a buffer after DMA reuses it.
If this fails, enlarge the L4 ring within measured bank budgets, improve bounded
storage behavior, or name a lighter profile. Loss must remain explicit.

Use the shared Storage and Audio contracts, including collision protection,
invalid-UTC handling, finalization and error accounting. Exercise storage's
bounded directory workspace under this budget; current optimizations include
an 8 KiB temporary zeroing workspace. A successful average-rate SD benchmark
does not establish safe file transitions.

Uncompressed four-channel WAV is the initial integration format under the
repository audio policy. Do not pass four channels to `HexaphonicRecorder` or
reinterpret its files. Mono FLAC can follow using the shared encoder after an
integrated CPU/RAM check. A four-channel FLAC/CRCL design needs explicit channel
count, metadata, reference/companion behavior and decoder compatibility work.
Preserve existing six-channel fixtures and formats unchanged throughout.

### External u-blox adaptation

Keep Time Service ownership of UTC and Location Service ownership of filtered
position. Reuse bounded NMEA framing/parsing when the module's configured output
is compatible. Add a receiver command/identity adapter, not a second time
estimator or a port of the Mbed GPS state machine.

| Concern | Required design |
| --- | --- |
| Selection | Name the physical module, firmware/protocol revision, UART rate, enabled messages, PPS configuration, harness and available controls. Treat 6/M6, M8 and M10 as separate qualification profiles. |
| Startup | Begin by observing configured output at the expected rate; bound discovery and record identity where supported. Unknown identity stays unknown. Do not send u-blox commands to the U5 receiver by default. |
| Navigation | Qualify RMC/GGA/date/fix output for each profile; add bounded UBX parsing only where needed. Check talker variants, invalid UTC and mixed binary/ASCII traffic. |
| Commands | Explicitly replace PMTK defaults for u-blox. Select supported commands using the exact receiver protocol; no family-wide assumption that all generations share configuration or power messages. |
| Power/reset | The Evo header shows enable but no dedicated reset signal. Model absent controls explicitly; do not fabricate a reset GPIO or report a software standby request as hardware power removal. Verify enable electrical behavior per harness/module. |
| PPS | Use PB5 and a new L4 PPS source. Preserve provenance, monotonic correlation and uncertainty. Test UART/PPS epoch association before admitting anchors. |
| Duty cycling | Introduce only after continuous operation is sound. Qualify wake, backup retention, reacquisition and power measurement per module. Hardware mode limitations need a target ADR policy override if common behavior cannot be met. |

The existing U5 TIM4 capture implementation deliberately programs a **32-bit**
timer and a U5-specific prescaler. L476 TIM4 is **16-bit**; its source cannot be
reused by substituting pin names. With fixed PB5 wiring, investigate TIM3_CH2
capture with a correctly extended 16-bit counter, or use an EXTI timestamp for
initial diagnostics with measured interrupt latency and honest uncertainty.
At 1 MHz a 16-bit counter wraps every 65.536 ms, so consecutive 1 Hz edges
require reliable overflow accounting, including capture/update races and missed
interrupts. A wider timer requires verified routing or a documented hardware
modification. Reserve Embassy's monotonic timer independently.

The [M8 receiver protocol reference](https://content.u-blox.com/sites/default/files/products/documents/u-blox8-M8_ReceiverDescrProtSpec_UBX-13003221.pdf)
is a starting source for M8 qualification. Obtain the matching primary protocol
manual for each actual 6/M6 or M10 module before implementing commands. The
pinned legacy GPS code is evidence about past experiments, not proof of
cross-generation command compatibility.

## Acceptance and consequences

L4 support is advertised per validated board/profile/module combination. A
working M10 is not evidence for M6 or M8, and a successful mono diagnostic is
not four-channel support. Publish flash/RAM/stack, loss, skew, backlog, storage
latency and CPU/deadline evidence with the image identity. U5 functional and
performance gates remain those in L4-004.

## Policy overrides

None at this architectural stage. Included recording, timing, GPS, location,
logging and power behavior inherits the relevant
[repository policy](../../Policy/policy_hierarchy.md). Diagnostic synthetic time
or intentionally different receiver duty-cycle behavior needs a scoped test or
application ADR override before implementation.
