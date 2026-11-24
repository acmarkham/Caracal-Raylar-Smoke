# First Evo L4 LED validation: 2026-11-24

Implemented decision: [ADR-L4-006](../../../SpecAndDesign/Platform/L4/ADR-L4-006-first-led-bring-up.md).
Tests ran on Windows with the connected ST-Link V2 over SWD at a requested
100 kHz. Firmware reported device/revision `0x10076415` and 1024 KiB flash.
The exact physical PCB revision was not independently identified.

| Check | Result |
| --- | --- |
| Flash programming | Passed with probe-rs `--verify`. |
| Clock source | HSE ready, crystal bypass off; PLL ready and selected as SYSCLK. |
| Clock configuration | HSE 16 MHz / M=2 x N=20 / R=2 = nominal 80 MHz; all bus divisors 1. |
| Power / flash / timer | Range 1, four flash wait states, TIM5 prescaler 79; firmware readback assertions passed. |
| GPIO | PC1 blue output latch and input pad matched low/on and high/off for 22 edges; PB11/red and PC3/green remained high/off. |
| First observation | Target PASS at edge 20 / 10,000 ms; subsequent edges continued. First interval 500,008 us, remaining observed intervals 500,000 us. |
| Independent coarse cadence | Host elapsed time from edges 1 to 20 was 9.469 s, compared with nominal 9.5 s. RTT delivery is not a precision frequency measurement. |
| Restart | A second `run --preverify --verify` session restarted the image and passed the same 20-edge gate, with 22 edges captured. |
| Final board state | Standalone `probe-rs reset` succeeded after monitoring; no subsequent debug attach, leaving the LED image running. |
| Linked image | 17,016 bytes of flash load; 1,316 bytes static RAM. Stack high water was not measured. |
| Memory regions | Generated layout: 1 MiB flash at `0x08000000`, 96 KiB SRAM1 at `0x20000000`; no SRAM2 allocation. |
| Wrong build target | Explicit U5 target rejected by the new package build script with the documented L4 target message. |
| Feature graph | L476RG + TIM5 only; no U5 chip, Raylar board/drivers or TrustZone. |
| U5 firmware regression | All 61 existing firmware packages built individually before and after this change; 61/61 passed both times. |
| U5 selection | The original 76 workspace members remain the defaults; U5 hello-world and Aardwolf feature graphs match the captured originals exactly. |
| Dependency resolution | Existing package entries unchanged; shared lockfile only adds this L4 smoke package. |
| Portable host tests | 203 passed, five failed. The identical five failures and pass count reproduce against untouched pre-change source at `4647a55c014d40fde13a7a3e9b09b37bd277fabb`. |
| Integration004 host tests | 13 passed using its existing default radio/channel features. |
| Optical LED observation | User confirmed on 2026-11-24: "Yes, the blue LED is blinking." This supplies the visual check alongside the recorded electrical/timing checks. |
| U5 physical tests / other hosts | Not run; no U5 board or Linux/macOS host used for this validation. |

No existing U5 source, test assertion, fixture or release setting was changed.
The root manifest adds an L4 member and preserves the U5 default member set;
the root ignore rule now permits the existing shared Cargo lockfile to be
tracked. This evidence does not claim a fully green historical host suite or
complete the migration's wider hardware qualification.

The five baseline host failures, retained without changing their expectations:

- `gps::framer::tests::frames_sentence`
- `estimator::tests::estimates_frequency_error_with_long_baseline_regression`
- `estimator::tests::frequency_regression_rejects_one_in_window_outlier`
- `estimator::tests::hardware_scale_sample_handles_small_capture_quantization`
- `estimator::tests::holdover_warning_is_one_way_until_an_anchor_is_accepted`

The control source was materialized from the pre-change Git commit under ignored
`target/l4-bringup-baseline/pre-change-source`, using the exact original lockfile.
The portable suite used the same package/feature/host-target arguments in both
trees. Integration004 was run separately with its default features; disabling
those features omits its radio frequency/power constants and is not its normal
host-test configuration.

An initial workspace-wide U5 build, before any manifest/code changes, failed
because executor trace features were unified into binaries without trace hooks.
The 61 separate package builds avoid that existing incompatibility. This change
does not make a combined U5/L4 workspace invocation valid.

One exploratory `reset` followed by a live `attach` failed the strict interval
assertion after the first edge. The timestamp disturbance was consistent with
one 65.536 ms timer wrap while the CPU was halted during attachment. This is an
inference from the observed offset and the pinned Embassy timer implementation,
not an oscillator measurement. The raw failure is retained; both attach-before-
execution runs passed without changing firmware or acceptance bounds. Avoid
live attach for cadence acceptance, and reset after an exploratory halt.

Artifacts and evidence:

- [First flash RTT log](first-run.log) and [host observation timings](first-run.json).
- [Successful restart RTT log](restart-run.log) and [restart timings](restart-run.json).
- [Final standalone reset](standalone-reset.json).
- [Exploratory live-attach failure](reset-run.log) and [its result](reset-run.json).
- [Build provenance](build.json), [ELF load segments](image-layout.json),
  [L4 dependency features](l4-features.txt), and [regression commands/results](regression.json).
- [Current host-test log](host-tests.log) and [untouched-source control log](pre-change-host-tests.log).
- [Exact flashed firmware archive](firmware.zip) and [archive SHA-256](firmware.sha256).

The archive contains the exact ELF/BIN, lockfile, build record and relevant
source/build inputs. Restore sources at the recorded commit before rebuilding;
do not infer reproducible bytes from a dirty Git identity alone. The ELF SHA-256
is `0630cfcf9edc53b8a9fe9e90a4e98fe4907bbcf8dd3045a0758bbf8f0f2c9774`.
The BIN starts at `0x08000000` and was generated from its flash load segments;
debug information in the 3.7 MB ELF does not occupy device flash.
