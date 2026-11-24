# ADR-L4-001: Caracal Evo hardware baseline and capability limits

## Status

Proposed, 2026-11-23. Hardware-document review complete; physical board revision,
fitted modules and behavior still require bench confirmation.

## Context

Raylar currently selects `stm32u595vj` in the workspace HAL dependency and uses
the STM32U595VJT6Q board model. The older Caracal Evo schematic identifies U1 as
`STM32L476RGT6`. A successful port needs a distinct hardware contract before
generalizing existing services.

Evidence has separate roles: ST defines silicon capabilities; the schematic
defines intended board wiring; inspection and measurements establish the actual
unit; Mbed source suggests historical configurations. Neither legacy behavior
nor an untested schematic reading establishes working Embassy support.

## Decision

Use the following baseline. The absence of radio and motion sensors is a target
requirement from the migration brief, supported by the published base-board
component list. Expansion signals or external accessories do not make those
devices present in the L4 platform profile.

### Main differences

| Area | U5 / Raylar upstream | L4 / Caracal Evo target | Consequence |
| --- | --- | --- | --- |
| MCU | STM32U595VJT6Q, Cortex-M33; silicon maximum 160 MHz, current application clock about 144 MHz | STM32L476RGT6, Cortex-M4F, maximum 80 MHz | Measure deadlines on L4; clock ratios do not predict application throughput. |
| Rust target | `thumbv8m.main-none-eabihf` | `thumbv7em-none-eabihf` | Separate compilation and executable artifacts. |
| SRAM | Approximately 2.5 MiB; usable layout depends on ECC/security configuration | **128 KiB total: 96 KiB SRAM1 + 32 KiB SRAM2** | Start within verified linker regions; no assumed extra 64 KiB. |
| Internal flash | 4 MiB for the fitted U595 VJ part | **1 MiB** for L476 RG | Link and enforce an L4 flash budget. |
| Security / core power | Optional TrustZone and fitted internal-core SMPS support | No TrustZone; different power controller and board supply | U5 secure aliases, regulator writes and power sequences stay in the U5 backend. |
| Microphones | Current driver publishes six microphones through MDF1 | Eight ICS-41350 microphones fitted; four DFSDM filters | Initial continuous array supports four selected microphones, with a mono diagnostic mode. |
| Audio DMA | MDF1 and GPDMA linked-list implementation | DFSDM1 and conventional DMA channels | New acquisition backend; retain coherent-frame and loss-accounting contracts. |
| Radio | Ebyte E80 / LR1121 and messaging service | No fitted radio in this target | Exclude radio driver, tasks, queues and diagnostics from the L4 image. |
| Motion / compass | LIS2HH12 and LIS2MDL services | No fitted accelerometer or magnetometer | Exclude those sources and derived motion/heading features. |
| GNSS | Board-integrated GPS wiring; existing module commands and power/reset adapter | External u-blox, potentially 6/M6, M8 or M10; header carries UART, PPS and enable | Explicit receiver and harness profile; shared Time/Location service contracts. |
| SD card | SDMMC1, card detect and software-controlled power | SDMMC1 four-bit wiring and card detect; no equivalent SD power switch identified | Verify L4 DMA construction and recovery without assuming card power cycling. |
| Indicators | Five independently driven logical LEDs and buzzer | Three active-low LEDs and buzzer | Map available roles explicitly and preserve indication priority. |
| Controls | Dedicated application user button | Published buttons are BOOT0 and reset | Do not port the U5 user-button assumption to these buttons. |
| Power sensing / charging | DC, battery and solar ADC inputs; charge-controller integration | External supply divider shown, but sensing pin conflicts with legacy wiring; no equivalent charger found | No inherited lithium-cell SOC, solar or charger assumptions. |
| USB / expansion | U5 USB controller and Raylar connector layout | USB OTG FS and Evo expansion headers | Preserve SWD as the initial recovery path; validate USB separately. |

Silicon figures and filter counts come from [ST L476 DS10198 Rev 11](https://www.st.com/resource/en/datasheet/stm32l476rg.pdf)
and [ST U59xxx DS13633 Rev 3](https://www.st.com/resource/en/datasheet/stm32u595vj.pdf).
Board facts come from the pinned [Evo schematic](https://github.com/OpenWild/CaracalEvo_HW/blob/28ac586353e4258b828d69277ab215bf99090ef9/pcb/caracal_evo.sch)
and current [Raylar board definition](../../../crates/board/raylar_v1p0/src/lib.rs).
The [Rust target list](https://doc.rust-lang.org/rustc/platform-support.html)
documents the architecture targets.

The 192 KiB estimate must not appear in an L4 linker script or resource profile.
The 32 KiB SRAM2 region and any alias describe the same storage, not extra RAM.
Physical RAM, linked RAM, DMA-accessible RAM and retained RAM are distinct facts;
L4-003 and L4-004 require those to be checked explicitly.

### Four useful channels from eight fitted microphones

The schematic has four shared PDM data lanes, each carrying two microphones
selected by clock edge. The four-filter ceiling applies to the chosen continuous,
synchronous hardware-decimation design. It is not a claim that the MCU can never
observe another input under a different design. Eight-channel software decimation,
filter multiplexing and alternative capture hardware are outside this migration.

The active four-channel physical order must be named in configuration and
recording metadata. It must not be inferred from microphone numbering in the
legacy source: that source's lane labels conflict with the schematic on lanes
2 and 3. See [L4-005](ADR-L4-005-schematic-and-legacy-reference.md).

### Capability declaration

Each board/firmware composition will declare MCU, board revision, physical
microphone count, active microphone identities/order, supported acquisition modes,
sample-rate choices, GNSS profile, memory limits and available indicators/control
lines. Receiver family is separate from MCU family: u-blox support must remain
usable by another board later.

L4 composition omits absent devices and their dependent work. If common telemetry
or identity schemas retain a radio/sensor field, report `Unavailable` or an
equivalent explicit unsupported status. Do not publish fabricated zero readings,
successful no-op radio operations or endless retries for absent hardware.
An external compass mentioned in the Mbed tree is an accessory, not an L4 base
capability.

## Consequences

The L4 platform is a smaller supported product profile of the upstream Rust
system. Feature parity, unchanged throughput and unchanged sleep current are
not acceptance assumptions. U5 capability limits, buffers and defaults stay
independent of L4 limits.

The first bench record must identify the unit's board revision, MCU marking and
reported flash size, fitted crystal, GNSS part/firmware/harness, selected
microphones and any modifications. Unresolved wiring is a gate for the affected
peripheral, not a reason to guess a pin.

## Alternatives considered

- Treat both chips as the same STM32 backend: rejected because MDF/DFSDM, DMA,
  timers, memory, power and fitted devices differ materially.
- Make all U5 builds use the L4 feature ceiling: rejected because U5 remains the
  development platform and must retain its existing capability.
- Start from Mbed as the behavioral specification: rejected; its wiring and
  historical workarounds are reference evidence only.

## Policy overrides

None. Repository policies apply to included capabilities. Absent capabilities
are omitted under the existing policy hierarchy. Hardware-specific power and
indicator mappings require explicit target decisions before use.
