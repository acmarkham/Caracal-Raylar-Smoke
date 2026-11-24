# ADR-L4-007: Evo SDMMC/DMA and exFAT smoke bring-up

## Status

Accepted for implementation, 2026-11-24. Target:
[`unitsmokeL4/02_exfat`](../../../unitsmokeL4/02_exfat/README.md).
Hardware results are recorded in the target's evidence directory; this ADR
defines acceptance and does not itself claim a passed card test.

## Context

L4-005 identifies the SDMMC1 four-bit pins, PC6 card switch and candidate DMA2
channel 5. L4-006 established the 16 MHz crystal, 80 MHz core, TIM5 monotonic
clock and ST-Link programming path. This next image qualifies card/block I/O
and the same patched exFAT engine used upstream. It does not qualify recording,
card removal during writes, power-loss recovery or worst-case storage latency.

## Decision

Keep one workspace/lockfile and the existing U5 defaults. Select the L476 HAL
directly in this leaf, as in 01. Use `raylar-drivers` with its portable `defmt`
feature and **without** `stm32`/TrustZone, reusing its volume detection,
partition bounds and BlockDevice contract. Use the existing patched
`exfat-slim` engine for filesystem/file operations. Do not change U5 drivers,
filesystem implementation, fixtures or test expectations to admit L4.

The smoke uses one filesystem owner and one active file at a time, with a
one-block cache parameter and a fixed aligned 4 KiB data chunk. Its helpers
borrow the filesystem across awaits instead of nesting owning storage-driver
futures. The initial owning composition exceeded SRAM1 at link time; it was
discarded before flashing. This is a low-level filesystem diagnostic, not an
L4 implementation of the full Storage Service or its four-handle driver.

Retain the generated 96 KiB SRAM1 region for statics, async task storage and DMA.
Reserve the separate 32 KiB SRAM2 bank (`0x10000000..0x10008000`) for the CPU
stack, using explicit linker stack bounds and the vector table's initial MSP.
This is the actual second bank documented in
[DS10198 section 3.5](https://www.st.com/resource/en/datasheet/stm32l476rg.pdf#page=20),
not an extension or alias of SRAM1. No SRAM2 `.data` or `.bss` is introduced.
Stack frames initialize their contents through normal execution. Before any
peripheral work, paint and read back the unused SRAM2 stack span, excluding
active frames. Preserve a 1 KiB bottom guard, report the painted-stack watermark
after each phase, and check the guard before each block write. Reject DMA
buffers outside SRAM1, so this test does not assume DMA access to SRAM2.

The initial SRAM1-only write image linked but its nested filesystem poll stack
exceeded the available gap at runtime. A later guard stopped a retry before any
replacement-card write; an identical read-only root fingerprint/free-space
check confirmed that card remained unchanged. Keep separate non-inlined phase
poll boundaries and one shared static 4 KiB transfer buffer to limit copies.
Record actual RAM/flash and stack watermarks for both profiles. A 2 KiB diagnostic heap supports the
existing allocating directory-name accessor; listing processes one name at a
time. No whole-file allocation, full-card buffer or heap-sized audio ring is
introduced. Production recorder heap behavior remains a separate concern.

### Hardware configuration

| Resource | Selection |
| --- | --- |
| Core / monotonic clock | Retain HSE 16 MHz, main PLL M=2/N=20/R=2, 80 MHz buses and TIM5 at 1 MHz. |
| SDMMC kernel | PLLSAI1 from the same HSE and shared M=2, N=12, Q=2: 48 MHz; select `CLK48SEL=PLLSAI1_Q`. |
| SDMMC pins | PC12 clock, PD2 command, PC8..PC11 D0..D3, AF12. |
| Card detect | PC6 input with pullup; require inserted/low after startup settling. |
| Card power | Already supplied by the board; no invented U5-style power GPIO or power-cycle operation. |
| DMA | DMA2 channel 5, request 7; one channel serialized across RX/TX by Embassy SDMMC v1. |
| Interrupts | SDMMC1 and DMA2_CHANNEL5 each bind the Embassy handler and a bounded observation counter. |
| Card bus rate | Initialize at Embassy's identification rate, then request 24 MHz four-bit transfers. Check actual CLKCR. |
| Timeout | Hardware data timeout 24,000,000 bus periods; 15-second outer software bounds per high-level operation. |

Software deadlines are cooperative: they take effect when the polled operation
returns control. They do not preempt a blocking HAL poll. The host observation
deadline is separate; this smoke does not qualify every stuck-card recovery path.

Use the pinned Embassy 0.6.0 L4 SDMMC constructor with an explicit DMA resource.
Do not copy Mbed channel-turnaround abort/reinitialization sequences. Count
successful block calls/sectors, maximum blocks per call, DMA and SDMMC IRQs.
Multi-sector writes and direction changes must be exercised before claiming
write/DMA support. These counters are evidence, not a throughput guarantee.

### Read-only inspection first

The default build rejects every block write in its adapter. Initialize the card,
record CID/size and relevant clocks, detect a raw or MBR-partitioned exFAT volume
through the shared detector, mount and inspect the root. Stop on unsupported
format or invalid geometry. No formatting, raw-sector write test, partition
editing or filesystem repair is part of this task.

Bound the streaming root scan to 131,072 entries and 60 seconds. The initial
256-entry limit stopped safely on this populated card before writes and was
increased without retaining additional entries in RAM. Record count and an order-independent
fingerprint of entry names/types/lengths without dumping existing file contents.
If the listing exceeds the bound, stop before a write test. The fingerprint is
a diagnostic consistency check, not a cryptographic proof of full-card integrity.

### Explicit write-test profile

Only `--features write-test` admits writes, and its write gate starts disabled.
After the read-only inspection, enable it solely for the new-file phase:

1. Require sufficient free space before creating a file. Find the first free
   `/L4E00000.BIN` through `/L4E00255.BIN`, then open it with `create_new` so the
   final open still rejects a collision. Never truncate or append to an existing file.
2. Write an offset-dependent deterministic pattern in aligned 4 KiB chunks.
   Include a 16-byte `CARACAL_L4_EXFAT` signature and an eight-byte length header.
   Size is at least 256 KiB + 173 bytes and larger than one cluster; reject a
   profile requiring more than 2 MiB. This exercises multi-sector transfers,
   cluster growth and a final partial sector.
3. Flush and close. Reattempt exclusive creation of the same name
   and require `AlreadyExists`. Do not alter the completed file.
4. Disable writes, discard the filesystem/cache instance and mount a new one.
   Read the entire file, compare every pattern byte, require exact EOF and log
   its diagnostic hash. Compare the pre-existing root fingerprint/count while
   excluding the newly created file.
5. Leave the file on the card for independent inspection and reset verification.
   Do not remove previous smoke files or other content automatically.

The subsequent default read-only image finds the last test-named root file and
verifies its signature, declared length and complete pattern after MCU/card
reinitialization. A foreign or damaged test-named file fails verification and
remains untouched. A read-only pass after writing must still report zero block
writes. This qualifies reset persistence, not card power-loss durability.

### Indication and evidence

Keep red and GPS green off; blink blue at 1 Hz only after final PASS. Report
failures through RTT/panic and never print PASS after an I/O, timeout, allocation,
content or bounds failure. The host monitor has a finite observation deadline.
Do not live-attach to a running timed I/O operation; start an attached reset run.

Capture exact ELF/BIN/lockfile, tool/source identity, leaf feature graph, generated
memory layout, selected DMA/clock configuration, card/volume identity, successful
and failed hardware logs, new-file path/size/hash and regression results.
Return the board to the read-only profile after the write test so resetting it
cannot create files repeatedly.

## Acceptance

- Read-only SDMMC four-bit initialization, valid exFAT mount and bounded root
  inspection pass with zero writes and observed DMA interrupts.
- Write profile passes full-byte readback after a fresh mount; multi-block writes,
  RX/TX use and collision rejection are observed. Existing root entries match
  their initial name/type/length fingerprint, excluding the new file.
- Default image verifies the retained file after reflash/reset with zero writes.
- Both images fit the explicit SRAM1/static and SRAM2/stack regions. SRAM2
  painting/readback and the runtime guard pass; report static allocation and
  observed stack watermark separately. Do not claim a recorder RAM budget from this test.
- Existing U5/L4-01 source and effective features remain unchanged. Build each
  existing U5 firmware package independently and compare relevant storage/board
  feature graphs. Retain known baseline test failures without weakening tests.

## Policy overrides

The storage naming policy applies to Storage Service streams; this test directly
exercises the filesystem and creates diagnostic files without a UTC source.
It does not invent UTC or change service naming, collision or recovery policy.
Persistent Logging and Versioning services are outside this smoke image; RTT
and its captured build record supply test evidence.

```yaml
policy_overrides:
  - policy: ui
    key: startup.enabled
    value: false
    reason: The SD/DMA smoke image does not initialize the buzzer.
  - policy: ui
    key: errors.enabled
    value: false
    reason: Diagnostic failures halt through panic-probe/RTT; application audible and latched fault indications are outside this image.
```
