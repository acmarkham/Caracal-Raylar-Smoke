# ADR-L4-002: One upstream source with explicit platform builds

## Status

Proposed, 2026-11-23. All layouts, feature names and new build entry points below
are intended design, not implemented interfaces.

## Context

The workspace already separates board definitions, drivers, metadrivers,
services and applications. However, its build selection is currently U5-wide:

- Root `Cargo.toml` enables `stm32u595vj` and `time-driver-tim5` on the shared
  `embassy-stm32` 0.6.0 dependency.
- `raylar-drivers/stm32` exposes backend modules containing U5-specific PAC,
  pin, timer, ADC and MDF/GPDMA assumptions.
- `.cargo/config.toml` defaults to the U5 target and a PowerShell runner.
- `scripts/firmware.ps1` and `.cargo/probe-rs-runner.ps1` embed the U5 target or
  probe chip; `scripts/firmware-artifacts.ps1` recognizes U5 flash aliases.
- `apps/aardwolf` directly composes the Raylar board and imports common startup
  code through `servicetests/storage/common.rs`. That file contains U5 RCC,
  fractional-PLL, prescaler and peripheral construction code.

A target triple alone cannot select a board, fitted devices or the correct
Embassy chip feature. Adding an L4 chip feature to the root dependency would
combine incompatible chip selections.

## Decision

### Source ownership and boundaries

Keep one repository, one dependency lockfile and one shared implementation of
portable behavior. The U5 path remains the reference for existing algorithms,
state machines, file formats and application behavior. Fixes and improvements
land in shared upstream modules; L4 changes add adapters or explicit resource
profiles. There will be no copied `services_l4` tree or long-lived L4 firmware
fork to reconcile later.

| Layer | Owns | Must not own |
| --- | --- | --- |
| Portable driver contracts / parsers | Data types, validated protocols, errors and peripheral-independent state | Concrete MCU pins, register layouts or board initialization |
| MCU backend | MDF versus DFSDM, DMA/IRQ mechanics, timer width, ADC, power, CRC/UID and clock implementation | Application feature policy or pretend support for missing devices |
| Board support | Pin ownership, polarity, oscillator facts, fitted devices, safe power states, resource routing | Storage naming or Time/Location algorithms |
| Shared metadrivers / services | AudioSource, time estimation, location filtering, storage/logging, recording and codecs where feasible | New direct imports of a board crate or concrete MCU PAC |
| Firmware composition | Board selection, tasks, capabilities, queues, buffers and chosen policy/profile | A second implementation of shared services |

Preserve the current Raylar board package. Add an Evo board package beside it,
provisionally `crates/board/caracal_evo`. Distinguish the silicon/board adapter
from the external GNSS module adapter. Use existing traits and const capacities
first; add narrow seams only where concrete coupling prevents L4 compilation.
Static dispatch and compile-time allocation are the default. A universal board
object, dynamic device registry and blanket conversion of all drivers to new
traits are unnecessary for the first board.

The driver crate can initially retain portable contracts and family-specific
modules in the same package, with mutually exclusive backend features such as
`stm32u5` and `stm32l4`. Keep established U5 public imports working through
compatibility re-exports. Its existing `stm32` feature must retain a documented
U5-compatible meaning during transition; L4 must never enable that compatibility
selector accidentally. A later crate split is justified only by a concrete
dependency or maintenance need.

Move startup responsibilities from test-owned code into board/backend modules
incrementally. Extract the U5 behavior without changing its clock arithmetic,
ordering, prescaler corrections or SMPS defaults. Keep existing test call sites
working through wrappers. L4 firmware must not import `servicetests/storage/common.rs`.

### Cargo feature ownership

After the U5 baseline is captured:

1. Keep the HAL version and portable settings centrally specified, but remove
   concrete chip and time-driver selection from shared workspace dependencies.
2. Make each leaf firmware graph explicitly select exactly one board/chip and
   Embassy time driver. Board crates may forward these choices. Standalone
   smoke programs that do not use a board crate still need an explicit selector.
   Audit every current firmware member so no U5 target loses its selection.
3. Keep host-test graphs free of MCU backend activation. Embedded services can
   continue using portable Embassy synchronization/time interfaces with their
   existing host test configuration.
4. Reject conflicting MCU backends, zero/multiple boards in a firmware image,
   TrustZone on L4 and unsupported feature/capability combinations early.
5. Separate hardware availability from application requests. Optional radio
   behavior requires optional dependency edges as well as task omission; an
   `if cfg!(...)` branch does not remove dependencies or type-checking needs.

Cargo features are additive and dependency features are unified within a build
graph. Resolver 2 does not make two chip features mutually exclusive. Build U5
and L4 firmware in **separate Cargo invocations**, even in the same workspace.
Neither `cargo build --workspace` nor `--all-features` is the dual-platform
acceptance command. Use `cargo tree -e features` for each supported leaf graph
to verify the selected chip and timer. These constraints follow the
[Cargo feature model](https://doc.rust-lang.org/cargo/reference/features.html).

### Build contract and host portability

An image is identified by board, package/binary, Rust target, exact feature set,
profile and security mode. Chip selection and target selection must agree:

| Build | Board / HAL feature | Rust target | Time-driver rule |
| --- | --- | --- | --- |
| Existing U5 | Raylar v1.0 / `stm32u595vj` | `thumbv8m.main-none-eabihf` | Retain TIM5 and its existing configuration. |
| New L4 | Evo verified revision / `stm32l476rg` | `thumbv7em-none-eabihf` | Initially reserve TIM5, subject to L4 pin/timer allocation review; exactly one Embassy driver. |
| Portable tests/tools | No board or chip | Native host target | Host time/test implementation only. |

[Embassy 0.6.0 documents the L476RG target](https://docs.embassy.dev/embassy-stm32/0.6.0/stm32l476rg/index.html),
but this does not establish support for every required peripheral. The inspected
0.6.0 source has no high-level DFSDM driver. Plan for a small L4 PAC-based DFSDM
adapter, following the ownership, bounded operation and async integration
contracts used upstream. An Embassy upgrade is a separate change with its own
U5 regression evidence; it is not a prerequisite assumed by this design.

Use explicit `--target`, leaf package/binary and features in automated builds;
do not derive the platform from the developer's machine or current working
directory. Keep the present U5 developer default during migration. Later neutral
defaults are a separate reviewed usability change. Isolate output by platform
and profile, and have artifact capture consume Cargo JSON executable paths
rather than reconstructing a path containing a hard-coded target triple.

Capture the compiler/tool versions that pass the U5 baseline, then pin those
versions and the dependency resolution. Add both embedded targets through
rustup. Use Rust's linker and LLVM tools for Rust firmware; the Mbed/Arm C++
toolchain is not a dependency of this port. Record the lockfile even though the
root `.gitignore` currently ignores `*.lock`; reproducible locked builds need
an explicit tracked workspace lockfile in a later implementation change.

Compilation must work from a clean checkout on Windows and Linux, with macOS
included in the portability matrix before claiming support there. Build-only
paths must not require PowerShell, a probe, USB drivers or hardware. Provide
shell-neutral Cargo commands first. Put shared metadata, image validation and
bounded probe execution in a portable Rust host utility if orchestration is
needed; retain the PowerShell entry points as compatible U5 wrappers until the
portable route reproduces their outputs. Tool upgrades and wrapper replacement
must not be bundled into the initial hardware extraction.

Document executable commands only as they become available. For example, the
present U5 build remains:

```text
rtk cargo build -p aardwolf --release --locked --target thumbv8m.main-none-eabihf
```

A future L4 command will name its new leaf package; this ADR does not imply that
such a package already exists.

### Firmware composition and missing features

Use a small L4 bring-up executable first, followed by an L4 recorder composition
using the shared services. This permits distinct startup/resource wiring without
conditionalizing the existing U5 application throughout. Any application policy
that must be shared is extracted from U5 with unchanged U5 tests before reuse.
The existing Aardwolf defaults remain intact, including radio, six-channel
capture, codec settings and diagnostics.

Profiles declare numeric limits such as active channel count and buffer size;
they are not simply a list of on/off features. Unsupported requests fail at
configuration/build admission, rather than silently degrading a recording.

### Artifact and flashing contract

Extend existing provenance with board identifier/revision, chip, Rust target,
resolved features, GNSS profile, channel map, resource profile, tool versions
and memory report. Preserve firmware version, source/dirty identity, build UTC,
profile, exact lockfile, ELF, binary and hashes.

Derive flash ranges and binary origin from the selected board memory layout and
ELF segments. Reject an L4 image outside its 1 MiB flash or using a U5 secure
alias. Require agreement between manifest, ELF and probe target; a `-Chip`
override alone is insufficient platform selection. U5 archives retain their
existing format/semantics unless an additive schema change is explicitly
versioned and tested. Keep bounded logs and pass/fail exit status on every host.
Validate ROM USB DFU separately from compilation and SWD flashing.

## Consequences and alternatives

This needs careful manifest and composition changes, but limits hardware
conditionals to the edges of the system. Duplicated workspaces would avoid some
feature interactions initially while creating independent versions of shared
code and lockfiles; reject that as the long-term design. A single executable
with pervasive MCU `cfg` branches is also rejected for initial bring-up because
it enlarges the U5 regression surface before the L4 hardware works.

## Acceptance

Each leaf graph has one chip and time driver; host graphs have neither. Old U5
commands resolve to the same effective defaults and release settings. U5 and L4
build independently from clean output directories on the supported hosts. The
unchanged U5 regression requirements in [L4-004](ADR-L4-004-regression-and-bring-up.md)
are mandatory for each extraction.

## Policy overrides

None. Build and ownership decisions do not change repository behavioral policy.
