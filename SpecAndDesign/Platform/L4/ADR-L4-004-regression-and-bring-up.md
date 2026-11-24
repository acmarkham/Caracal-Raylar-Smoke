# ADR-L4-004: Unchanged U5 regression gates and staged L4 bring-up

## Status

Proposed, 2026-11-23. This is the required migration/acceptance process; no new
CI, build, host-test or hardware result is claimed by this documentation change.

## Context

The principal migration risk is a shared change that compiles on L4 while
changing U5 timing, channel order, storage behavior, resource sizing or enabled
features. Passing L4 diagnostics cannot compensate for a U5 regression. This
repository has host tests, firmware tests, analysis tools and hardware evidence,
but no `.github` workflow directory was present at review. Automated enforcement
must be added deliberately rather than assumed to exist.

## Decision

### Define what stays identical

For an L4-only change, retain U5 test inputs, assertions, golden fixtures,
feature combinations, pass criteria and required hardware scenarios. Existing
U5 tests must not be skipped, weakened, replaced by L4 mocks or compiled out to
get the port through. New L4/shared tests are additive. Mechanical relocation
must preserve original test identities/results and be reviewed separately.

Keep U5 board mappings, clock settings/corrections, release optimization,
default SMPS selection, default six-channel/audio codec behavior, radio and
diagnostic defaults, queue/buffer capacities and file formats unchanged unless
an independently justified upstream change is explicitly reviewed. Shared
generic code must retain the existing U5 specialization.

Identical regression tests means identical behavioral checks and acceptance
thresholds. It does not require a bit-identical ELF: build timestamps, Git
identity and linking can change. Compare deterministic sample/codec/format
outputs exactly; compare hardware timing and performance against the captured
U5 tolerances using the same protocol. Never regenerate a golden fixture merely
because the L4 output differs.

### Establish the baseline before extraction

The source review started at local commit
`4647a55c014d40fde13a7a3e9b09b37bd277fabb`. This identifies inspected architecture,
not a clean or tested U5 release. Capture the actual clean upstream revision and
environment used for the baseline when implementation starts; keep unrelated
working-tree edits out of that comparison.

Record exact compiler/Cargo/probe/LLVM/Python/codec-tool versions, dependency
lockfile, supported host targets, release flags and effective features. Inventory
every executable member in `unitsmoke`, `drivertests`, `metadrivertests`,
`servicetests`, `integrationtests` and `apps`, including feature-gated benchmark
binaries. Record what currently passes, what already fails and what requires
hardware or local configuration. Pre-existing failure is visible baseline debt,
not a pass or an opportunity to drop coverage.

For each suite record command, test IDs/count, fixture hashes, outcome and
artifact identity. For board tests record board/card/GNSS/probe identity,
configuration, observation duration, log completeness and acceptance bounds.
Keep this manifest append-only for the migration; additions do not remove its
original U5 entries.

### Required build and regression matrix

Each row is a separate build/test graph. Existing package-specific host flags
remain part of its baseline command; host tests must use an explicit host
target because the repository default is embedded U5.

| Gate | Required coverage | Purpose |
| --- | --- | --- |
| Portable host regression | Existing driver, metadriver and service test suites; Aardwolf/integration library tests where host-enabled; current analysis/artifact checks | Preserve parsers, state machines, timing math, policies, formats and tooling. |
| U5 firmware compile/link | Every inventoried existing firmware binary and its baseline feature variants, including default Aardwolf | Detect broken manifests, U5 PAC paths, interrupt bindings, link layout and defaults. |
| U5 variant checks | Existing mono/default-six-channel, codec, radio, diagnostics and LDO/SMPS configurations actually used; secure builds only if supported in baseline | Preserve real configurations without assuming every theoretical feature combination works. |
| L4 compile/link | Each introduced leaf profile on `thumbv7em-none-eabihf` | Confirm selected backend, link regions, missing-device exclusion and image budgets. |
| Selection failures | Deliberately combined chip/backends, wrong-board feature requests, L4 TrustZone and unsupported channel modes | Ensure a wrong configuration fails instead of silently building another board. |
| U5 target hardware regression | Existing microphone/SD/GPS/power/radio tests and affected integration/application scenarios | Catch effects host tests and type checking cannot observe. |
| L4 target hardware acceptance | Milestone-specific tests below, additional to the U5 suite | Establish actual board capability, not just compilation. |

Portable test coverage includes the existing `raylar-drivers`,
`raylar-audiosource`, Time, Location, Storage, Logging, Power Management, Sensors,
Versioning, audio recorder, FLAC/CRCL encoder and radio service tests wherever
present, plus application/integration library checks. Capture their actual
commands rather than assuming `cargo test --workspace` can combine MCU and host
features safely. Preserve codec fixture decoding with an independent decoder.

Run portable host tests and both embedded compile jobs on Windows and Linux.
Add macOS compilation/host tests before claiming macOS support. Separate probes
or serialized lab jobs run hardware tests; CI without a board reports those as
pending, never passed. The existing U5 runner remains usable throughout.

For pure L4 adapter changes, the complete U5 host/compile suite is still required.
For shared source, manifest, startup, time, storage or toolchain changes, also run
the affected U5 hardware scenarios before merge. Before each bring-up milestone
is accepted, rerun the complete captured U5 hardware suite. Hardware unavailability
does not waive a required result; hold the milestone or shared refactor until
the evidence exists.

### Performance and data gates

Capture existing U5 CPU/backlog, codec timings, storage transition latency,
audio losses, channel order, PPS uncertainty and memory footprint. Preserve the
baseline limits and measurement methods. Numerical tolerances must be recorded
before comparing a change; do not expand them after a failure. Where no limit
exists, establish a repeatable baseline first and keep the resulting migration
threshold fixed.

Replay deterministic fixtures to require identical U5 PCM ordering, timestamps
where deterministic, encoded bytes and decoded samples. On hardware compare
continuity, counters, timing bounds and formats rather than demanding identical
analog microphone samples or GNSS acquisition times. Verify startup/versioning
and log-loss counters; missing required evidence makes a run incomplete.

### Migration milestones

| Stage | Work boundary | Exit evidence |
| --- | --- | --- |
| 0. Freeze U5 evidence | Inventory, clean revision, command/feature manifest, tool versions, host/compile and physical U5 baseline | Reproducible U5 artifact and complete results; no L4 code needed yet. |
| 1. Isolate platform selection | Remove root chip/timer coupling while explicitly preserving each U5 leaf; isolate backend modules and retain compatibility paths | Same U5 effective features/defaults, unchanged host suite, all U5 firmware links; invalid dual-chip selection rejected. |
| 2. L4 boot and time | Add Evo board/leaf, verified SRAM1/flash layout, conservative clock startup, RTT, identity and indications; reserve timer/DMA/IRQ ownership | Bounded startup, measured clocks, monotonic timer/wrap behavior, correct LED polarity, reset/SWD recovery, size report; no U5 behavior change. |
| 3. Independent peripherals | Separate SDMMC, external u-blox UART/PPS and one microphone diagnostics; complete the L4-005 conflict checks | Card read/write/flush/remount evidence, receiver identity/UTC-PPS association, microphone identity/edge/rate, peripheral errors and timeouts; memory access verified. |
| 4. Mono recording | Compose shared storage/logging/time and bounded mono audio; enable SRAM2 only with proven placement/startup | Decoded sample counts and gaps, segment transitions/reboot collision tests, measured stack/RAM/flash, GPS loss/reacquisition; initial integrated soak. |
| 5. Four-channel recording | Add four-filter coherent capture and shared recording path with declared channel map | Channel identity/polarity/skew, all-channel DMA generation checks, sustained rate and worst SD stall admission, exact frame counts across multiple boundaries; U5 suite still unchanged. |
| 6. Deployment qualification | Qualify each GNSS family/harness, field power behavior and optional compression/low power separately | Recorded support matrix; integrated overnight run crossing hour boundaries, resets/card faults, then seven-day recorder soak before field-readiness claim. |

Stages 3 peripherals can progress independently after shared timer/DMA ownership
is reserved. Do not enable combined workloads based solely on their individual
passes. Move from one microphone to four only after the mono integrated resource
and file lifecycle are understood.

At stage 2, validate executor timing, timer prescaler and long-duration/wrap
behavior; do not import U5 clock corrections. At stage 3, identify whether the
published board or a modified unit is on the bench. Full SRAM2 utilization and
write-intensive SD tests need explicit, repeatable initialization/recovery
evidence before recording depends on them.

### Hardware scenarios to preserve or add

The U5 baseline keeps all its existing radio and sensor tests despite their
absence from L4. In addition, L4 diagnostics must cover:

- GNSS missing/unrecognized, malformed traffic, invalid UTC, PPS missing or
  rejected, receiver off/on, reacquisition and extended counter wrap. Qualify
  each receiver/harness independently, including PPS under maximum SD load.
- SD absent/full/removed, read/write/timeout failures, interrupted segment
  finalization, reboot and filename collision, repeated close/open, hourly
  directory growth and persistent logging contention. Use dedicated test media.
- Audio one-channel identity, four-channel order/skew, deliberate consumer
  delay, DMA overrun/missed interrupts, start/stop/restart and counter wrap.
  Compare output frame count and discontinuity reporting with injected faults.
- Peak stack and CPU scheduling load during the above, with durable evidence
  and loss counters. Boot-only or isolated codec timing does not qualify the
  recorder. Benchmarks cannot substitute for an integrated soak.

Any laboratory synthetic UTC source must be labeled and allowed by its test
ADR; it cannot establish GNSS timing or normal application readiness.

### Change control and rollback

Keep implementation slices reviewable: first a behavior-preserving U5
extraction, then the L4 adapter using that seam, then a feature integration.
Do not combine Embassy/compiler upgrades, codec redesign and hardware bring-up.
An L4-only change may have no U5 runtime effect; a shared refactor must explain
the U5 equivalence evidence.

If a shared change regresses U5, revert or split that change and pause the
dependent milestone. Preserve the failure evidence and the prior working U5
ELF/manifest so the lab can restore the upstream baseline. Do not edit expected
results, reduce diagnostics or shrink U5 resources to approve an L4 milestone.

## Consequences

This adds initial inventory and lab work, but creates a concrete definition of
"U5 remains unchanged." L4 support can grow in small stages while upstream
development continues. Future upstream behavioral improvements can change U5
tests through their normal reviewed process; L4 migration alone is never the
justification for changing them.

## Policy overrides

None. Test-specific exceptions belong in the target test ADR using the shared
[policy hierarchy](../../Policy/policy_hierarchy.md), not in U5 test skips or
implicit implementation differences.
