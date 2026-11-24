# Caracal Evo L4 migration ADRs

Migration design baseline: 2026-11-23. The migration ADRs remain proposed;
[L4-006](ADR-L4-006-first-led-bring-up.md) defines the first implemented LED
smoke target added on 2026-11-24, with its own validation record.

The STM32U595 Raylar platform remains the upstream development platform and
source of truth. Caracal Evo adds a smaller, explicitly composed feature set
using the same Rust services and formats. The C++/Mbed firmware supplies hardware
reference evidence only.

Read these decisions in order:

| ADR | Scope |
| --- | --- |
| [L4-001: Hardware baseline](ADR-L4-001-hardware-baseline.md) | Device differences, fitted hardware, capability limits and evidence |
| [L4-002: Shared source and build chain](ADR-L4-002-shared-source-and-build-chain.md) | Platform boundaries, Cargo selection, host portability and artifacts |
| [L4-003: Resource and feature profiles](ADR-L4-003-resource-and-feature-profiles.md) | RAM/flash budgets, four-channel audio, codecs and u-blox integration |
| [L4-004: U5 regression and staged bring-up](ADR-L4-004-regression-and-bring-up.md) | Unchanged U5 acceptance suite, build matrix, milestones and rollback |
| [L4-005: Schematic and legacy reference](ADR-L4-005-schematic-and-legacy-reference.md) | Initial pin map, conflicting evidence and hardware verification gates |
| [L4-006: First LED bring-up](ADR-L4-006-first-led-bring-up.md) | Implemented L4 HSE/PLL, Embassy time, GPIO and ST-Link smoke target |
| [L4-007: SDMMC/DMA and exFAT](ADR-L4-007-sdmmc-dma-exfat.md) | L4 card inspection, exclusive file I/O, fresh-mount readback and reset persistence |

The most consequential correction to the initial brief is **128 KiB physical
SRAM, not 192 KiB**, for the STM32L476RGT6. The flash limit is 1 MiB.
[ST's STM32L476xx datasheet](https://www.st.com/resource/en/datasheet/stm32l476rg.pdf)
defines the device limits; L4-001 records the comparison and L4-003 budgets within
them. KiB and MiB mean powers of 1024 throughout these ADRs.

Existing [repository policies](../../Policy/policy_hierarchy.md) continue to
apply to each included capability. These ADRs propose architectural boundaries
and hardware limits; they do not edit policy defaults or redefine existing U5
target ADRs. Any later target-specific behavioral exception needs a structured
`Policy overrides` entry in that target's ADR.

The first LED image selects its HAL chip at the leaf and preserves existing U5
defaults; L4-006 records why broad U5 manifest extraction is deferred. It does
not complete the full migration baseline. Four-channel recording comes after
memory placement, SDMMC, GPS and microphone acquisition have separate evidence.
The milestone gates in L4-004 determine when a capability can be called supported.
