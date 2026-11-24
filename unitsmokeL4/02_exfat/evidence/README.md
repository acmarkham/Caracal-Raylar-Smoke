# Evo L4 SDMMC/DMA/exFAT validation

Validated on 2026-11-24, Windows, Rust/Cargo 1.96.0 and probe-rs 0.30.0, using
the connected ST-Link V2 at a requested 100 kHz SWD rate. Implements
[ADR-L4-007](../../../SpecAndDesign/Platform/L4/ADR-L4-007-sdmmc-dma-exfat.md).

The **replacement card** passed the complete test. It is identified by CID
serial `7cb1530c`, manufacturer ID 27 and 250,347,520 sectors. Its exFAT
partition starts at LBA 32,768, contains 250,314,752 sectors and uses 128 KiB
clusters. The earlier card has a different serial and separate results below.

| Check | Observed result |
| --- | --- |
| Clocks / bus | 80 MHz SYSCLK; 48 MHz PLLSAI1 SDMMC kernel; CLKCR `0x900` confirms four-bit, 24 MHz card bus. |
| DMA | DMA2 channel 5/request 7, with actual DMA and SDMMC interrupt counters. The adapter checks every filesystem DMA buffer stays in SRAM1. |
| Initial inspection | 166 root entries; fingerprint `eb30e8ae62c27814`; 64,810,123,264 bytes free; zero writes. |
| Test file | Exclusively created `/L4E00000.BIN`, 262,317 bytes. Existing names are never truncated or resumed. |
| Write / flush / close | Passed in approximately 505 ms, including the collision-rejection assertion. No claim of worst-case latency. |
| Fresh mount | Entire file compared byte for byte, including its header and 173-byte final partial sector; exact EOF passed. |
| Data hash | FNV-1a-64 `7c76907847852d9d`, repeated after reset. This is a diagnostic hash, not a cryptographic integrity guarantee. |
| Existing root metadata | Original name/type/length count and fingerprint matched after excluding the new file. Existing file contents were not exhaustively hashed. |
| Allocation | Free space decreased by 393,216 bytes: three 128 KiB clusters. |
| Write-run block traffic | 1,303 read calls/sectors; 68 write calls covering 516 sectors; largest write eight sectors. |
| Write-run interrupts | 1,373 DMA and 1,437 SDMMC interrupts. |
| Reset persistence | Reflashed/reset into the default read-only image, remounted and verified the same complete file/hash; zero writes. |
| Read-only persistence traffic | 821 reads/sectors, 823 DMA and 823 SDMMC interrupts. Reads through this filesystem path were single-sector calls. |
| Final root | 167 entries, including the one new test file. |

The final read-only profile disables block writes unconditionally. A subsequent
reset reinitializes the card and verifies the retained file without creating
another. No format, partition edit, raw-sector write test, deletion or automatic
repair was performed.

After monitoring, the [final standalone reset](standalone-reset.json) succeeded
with the read-only image installed. No further debug attach was made.

Memory qualification is specific to these two standalone images:

| Profile | Flash load | SRAM1 static extent | Dedicated SRAM2 stack | Observed stack watermark / unused |
| --- | ---: | ---: | ---: | ---: |
| Read-only | 86,328 B | 28,664 B | 32,768 B | 21,956 B / 10,812 B during persistence verification |
| Write test | 96,384 B | 78,764 B | 32,768 B | 30,244 B / 2,524 B |

The vector table initializes MSP to `0x10008000`; its lower stack bound is
`0x10000000`. The runtime paints/read-verifies the unused span and checks a
1 KiB bottom guard. Both profiles passed. These are painted-stack observations
from this workload, not a guarantee for a concurrent recorder or every card/error
path. Stack headroom was the reason for using the actual separate SRAM2 bank;
SRAM1 remains 96 KiB and all filesystem DMA buffers remain there.

Initial development failures are retained rather than counted as passes:

- The first composition exceeded SRAM1 at link time and was not flashed.
- The first card (`7cb05359`) mounted as raw exFAT at LBA 0, with 573 root
  entries and only 1,179,648 bytes free. The original 256-entry root limit
  stopped inspection without writes; a larger streaming bound then passed.
- A write attempt on that earlier card created `/L4E00000.BIN` but stopped
  during append when an overused SRAM1 stack disturbed the write-gate state.
  The user replaced that card before it could be rechecked. Treat that test
  file/allocation state as interrupted; no post-failure integrity claim or
  cleanup is made for the removed card.
- On the replacement card, the added stack guard stopped an exclusive-create
  retry. A later read-only run still found the original 166-entry fingerprint
  and identical free-space count. The successful test subsequently created
  the first free test name, `/L4E00000.BIN`.
- Explicit SRAM2 stack placement resolved the measured stack constraint, while
  preserving the guard and the file/data acceptance assertions.
- Two host monitor attempts could not open a probe held by an earlier monitor
  child process. They did not execute card tests. The bounded capture helper
  now owns and stops the probe process directly.

Regression checks:

- All 61 existing U5 firmware packages build individually with the locked
  dependency resolution. Ten captured U5/L4-01 leaf feature graphs are identical
  to their pre-change graphs. The original 76 U5 default members are unchanged.
- Existing dependency entries are unchanged; the lockfile adds only the new
  smoke package. No U5/shared Rust source, test, fixture or policy was edited.
- Portable host suite: 203 passed, five existing GPS/time failures matched the
  prior baseline. Integration004 adds 13 passing host tests with its default
  features. See [regression.json](regression.json) and [host-tests.json](host-tests.json).
- U5 hardware tests, other host operating systems, sustained recording, card
  removal and power-loss durability were not qualified by this test.

Primary successful evidence:

- [Replacement-card inspection](replacement-inspect.log).
- [Read-only SRAM2 stack validation](sram2-inspect.log).
- [Write, flush, collision and fresh-mount verification](write-success.log).
- [Read-only persistence after reset](persistence.log).
- [Image layouts and hashes](layouts.json), [build provenance](build.json),
  [read-only features](features-read-only.txt), [write features](features-write-test.txt).
- [Exact ELF/BIN/source/lockfile archive](firmware.zip), with
  [archive checksum](firmware.sha256).

Diagnostic failure evidence:

- [First card root-limit stop](initial-root-limit.log),
  [successful first-card inspection](inspect.log),
  [interrupted first-card write](old-card-write-failed.log).
- [Replacement-card stack guard stop](replacement-write.log).

The archive contains both exact final images and their relevant source/build
inputs. Rebuilding from a dirty Git identity alone is insufficient; use the
archived inputs and dependency lockfile with the recorded compiler. For the
exact tested bytes, use the saved ELF/BIN and verify their recorded SHA-256.
