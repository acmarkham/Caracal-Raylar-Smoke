# ADR-L4-005: Pin mapping evidence and the role of Mpala firmware

## Status

Proposed, 2026-11-23. Candidate mappings are extracted from source documents,
not electrically validated. Conflicts below are explicit bring-up gates.

## Context and evidence

The public Eagle schematic is machine-readable and identifies U1's part number
as STM32L476RGT6 and XTAL1 as a 16 MHz part. `EvoC_007_Mpala` supplies useful
historical pin aliases, DFSDM/DMA configuration and GPS experiments, alongside
comments and hardware modifications that cannot be treated as current board
truth.

Reviewed snapshots:

| Source | Pinned evidence |
| --- | --- |
| OpenWild hardware | Commit `28ac586353e4258b828d69277ab215bf99090ef9`: [Eagle schematic](https://github.com/OpenWild/CaracalEvo_HW/blob/28ac586353e4258b828d69277ab215bf99090ef9/pcb/caracal_evo.sch), [PDF](https://github.com/OpenWild/CaracalEvo_HW/blob/28ac586353e4258b828d69277ab215bf99090ef9/pcb/caracal_sch.pdf), [PCB](https://github.com/OpenWild/CaracalEvo_HW/blob/28ac586353e4258b828d69277ab215bf99090ef9/pcb/caracal_evo.brd) |
| Mpala reference firmware | Commit `5c655351dbd9f85ec1f119ef6680f6f78fa30a25`: [EvoC_007_Mpala tree](https://github.com/acmarkham/CARACAL_EVO_COMBRETUM/tree/5c655351dbd9f85ec1f119ef6680f6f78fa30a25/EvoC_007_Mpala) |
| Silicon | [ST DS10198 Rev 11](https://www.st.com/resource/en/datasheet/stm32l476rg.pdf), including pin alternate functions and timer widths; consult the matching reference manual and errata when implementing register/DMA behavior. |

The pin extraction below used the Eagle XML nets and part attributes directly.
The PDF and PCB are linked for visual/layout cross-checking. No assertion is
made that the physical unit has this exact published wiring or fitted options.

## Decision

Use the schematic as the intended netlist, ST documentation to establish legal
MCU functions, and the actual unit to resolve modifications. Use Mpala to find
candidate settings and known problem areas. Preserve disagreements rather than
choosing a convenient alias. Implement the resulting adapter in Embassy Rust;
do not import Mbed/HAL startup code, RTOS ownership or error loops.

### Initial schematic pin map

"Agrees" means agreement between reviewed documents, not a bench pass. Signal
direction is from the MCU perspective unless explicitly identified as a
receiver-side net label.

| Function / schematic net | MCU / connection | Reference comparison and required check |
| --- | --- | --- |
| HSE | PH0/PH1, 16 MHz XTAL1 | Verify fitted part and derived peripheral clocks; do not copy U5 PLL settings. |
| Microphone clock `MIC_CLK` | PC2, DFSDM clock-output candidate | One net feeds all eight microphone CLK pins; verify output AF and clock waveform. |
| `MIC_STREAM_0` | PB12, M1 + M5 DATA | Mpala uses DFSDM channel 1, rising edge. Confirm which physical microphone that edge selects. |
| `MIC_STREAM_1` | PC7, M2 + M6 DATA | Mpala uses channel 3, rising edge. Same identity check. |
| `MIC_STREAM_2` | PB6, M4 + M8 DATA | Mpala uses channel 5, rising edge, but calls this MIC3. Schematic names it M4/M8. |
| `MIC_STREAM_3` | PB10, M3 + M7 DATA | Mpala uses channel 7, rising edge, but calls this MIC4. Schematic names it M3/M7. |
| SD data | PC8=D0, PC9=D1, PC10=D2, PC11=D3 | Agrees with Mpala SDMMC1 four-bit setup. Validate pin AF, pullups and bus clock. |
| SD clock / command | PC12=CLK, PD2=CMD | Agrees. Construct the L4 SDMMC/DMA adapter rather than reusing U5 constructor assumptions. |
| `SD_SW` | PC6 | Verify inserted level and debounce on the actual socket; schematic shows a pullup. |
| GPS UART transmit | PA2 / USART2_TX, schematic `GPS_RX`, SV1 pin 3 | Receiver RX label. Mpala calls PA2 `GPS_TX`, from the MCU perspective. These names differ without implying crossed wiring. |
| GPS UART receive | PA3 / USART2_RX, schematic `GPS_TX`, SV1 pin 4 | Receiver TX label. Mpala calls PA3 `GPS_RX`. Verify harness direction. |
| `GPS_PPS` | PB5, SV1 pin 6 | Agrees. EXTI or verified timer AF; no reuse of the U5 PB9/TIM4 adapter. |
| `GPS_EN` | PB4, SV1 pin 1 | Agrees on pin only. Enable polarity/electrical drive, actual power switching and receiver reset behavior remain module/harness-specific. |
| GPS supply / ground | SV1 pin 5=VDD, pin 2=GND | Header marked `DO_NOT_FIT` in schematic; inspect fitted connector and module supply compatibility. No dedicated GPS reset pin shown. |
| Red LED `LED_R` | PB11 | Agrees; active-low circuit. Initialize off before normal indication. |
| Green LED `LED_G` | PC3 | Agrees; active low. Map the GPS-green role explicitly. |
| Blue LED `LED_B` | PC1 | Agrees; active low. Assign SD/activity role with one owner. |
| Buzzer `BUZ` | PB14 through R15 | Mpala uses PB14 alternate PWM output on TIM8. Validate AF/channel/polarity. |
| USB | PA11=DM, PA12=DP, PA9=VBUS | Verify FS controller configuration and VBUS sensing; DFU boot/recovery is a separate test. |
| SWD / trace | PA13=SWDIO, PA14=SWDCLK, PB3=SWO, NRST on X1 | Preserve debug/recovery during clock and low-power experiments. |
| Expansion I2C | PB7=`I2C_SDA`, PB8=`I2C_SCL`, X2 pins 6/8 | Generic Mpala aliases say SDA=PB9 instead. Resolve before using accessories. |
| Expansion SPI | PA5=SCK, PA6=MISO, PA7=MOSI, X2 pins 7/5/3 | PA5 also used as battery ADC in Mpala; mutually exclusive if physically repurposed. |
| Expansion serial `LORA_RX/TX` | PC4/PC5, X2 pins 4/2 | Signal names do not establish a fitted radio. L4 profile has no radio. |
| Expansion PB15 | PB15, X2 pin 9 | Uncommitted until an accessory requires it. |
| `BATT_ADC` | Schematic connects R11/R12 supply divider to PA8 | Conflicts with Mpala PA5 ADC and PA8's lack of a normal ADC input. Block voltage/SOC support pending physical verification. |
| Buttons | SW1 drives BOOT0; SW2 drives NRST | They are boot/reset controls, not a normal U5-style application button. |

The expansion connector X2 has a similar debug-header footprint; do not mistake
it for X1 or infer pinout from connector shape. All connector numbering above
comes from the schematic gate/net assignments and needs orientation checking
on the physical board.

### Candidate filter, DMA and timer allocation

[Mpala `micmapper.h`](https://github.com/acmarkham/CARACAL_EVO_COMBRETUM/blob/5c655351dbd9f85ec1f119ef6680f6f78fa30a25/EvoC_007_Mpala/micmapper.h)
uses DFSDM input channels **1, 3, 5, 7**, all sampling on the rising edge. Its
December 2022 note records an edge change for microphone mapping. The schematic
lanes alone do not prove which of the pair produces valid data on that edge.

[Mpala `multi_audio.cpp`](https://github.com/acmarkham/CARACAL_EVO_COMBRETUM/blob/5c655351dbd9f85ec1f119ef6680f6f78fa30a25/EvoC_007_Mpala/multi_audio.cpp)
assigns filter 0/1/2/3 to DMA1 channel 4/5/6/7 respectively. Treat this as an
allocation candidate, not a copied register program. Confirm request selection,
interrupt names/priorities, filter synchronization, buffer lifetime/alignment
and DMA-accessible memory against the exact part and pinned Embassy PAC.

The [legacy SD adapter](https://github.com/acmarkham/CARACAL_EVO_COMBRETUM/blob/5c655351dbd9f85ec1f119ef6680f6f78fa30a25/EvoC_007_Mpala/component_sdio/TARGET_STM/TARGET_CARACAL_EVO/sdio_device.c)
uses DMA2 channel 5 for both directions and comments about channel 4 problems.
Record this as a test lead. Validate Embassy's supported route, direction
turnaround, cancellation and error recovery; do not transplant Mbed abort and
reinitialization sequences without a demonstrated need.

Reserve a resource ledger before combining drivers: Embassy monotonic timer
(TIM5 candidate), PPS on PB5 (TIM3_CH2 or EXTI candidate), buzzer timer/output,
four DFSDM filters and their DMA channels, SDMMC DMA, UART buffering and all IRQ
owners. This ledger must show no duplicate peripheral/pin ownership. L4 timer
width and rollover handling are specified in
[L4-003](ADR-L4-003-resource-and-feature-profiles.md).

### Conflicts that block a capability

| Issue | Evidence | Required resolution |
| --- | --- | --- |
| Physical audio order | Schematic lane 2=M4/M8 and lane 3=M3/M7; legacy comments label them MIC3/MIC4 | Trace channel-select wiring/edge and stimulate each physical microphone separately. Publish one verified physical order and geometry before array recording/analysis claims. |
| Battery measurement | Schematic `BATT_ADC` goes to PA8; legacy `PIN_BATT_ADC=PA5` says external connection shared with SPI SCK | Inspect PCB/continuity and modifications, verify ADC-capable destination and divider/input range; no voltage or SOC output until resolved. |
| I2C SDA | Schematic PB7; generic `PinNames.h` PB9 | Trace actual accessory wiring and choose the appropriate pin/AF. Do not expose a working-I2C capability from a generic alias. |
| GPS controls | Header gives PB4 enable but no reset; legacy switches between hardware/software control and warns about receiver-dependent cycling | Document exact module/harness circuitry, levels, power/reset effects and supported protocol before cycling. |
| User button | Published switches are BOOT0/NRST, unlike U5's dedicated input | Omit application-button capability unless a separately wired input is documented. |
| SD recovery | No equivalent U5 SD power switch identified | Test controller reset/remount recovery; add hardware-dependent power cycling only for a verified board modification. |

### How to use the legacy tree

Useful reference points include
[`PinNames.h`](https://github.com/acmarkham/CARACAL_EVO_COMBRETUM/blob/5c655351dbd9f85ec1f119ef6680f6f78fa30a25/EvoC_007_Mpala/TARGET_CARACAL_EVO/PinNames.h),
[`CaracalCore.cpp`](https://github.com/acmarkham/CARACAL_EVO_COMBRETUM/blob/5c655351dbd9f85ec1f119ef6680f6f78fa30a25/EvoC_007_Mpala/CaracalCore.cpp),
[`system_clock.c`](https://github.com/acmarkham/CARACAL_EVO_COMBRETUM/blob/5c655351dbd9f85ec1f119ef6680f6f78fa30a25/EvoC_007_Mpala/TARGET_CARACAL_EVO/system_clock.c)
and [`GPS.cpp`](https://github.com/acmarkham/CARACAL_EVO_COMBRETUM/blob/5c655351dbd9f85ec1f119ef6680f6f78fa30a25/EvoC_007_Mpala/GPS.cpp).
They show active-low indicators, a historical 80 MHz clock setup, UART/PPS
handling and receiver experiments. They do not define the new scheduler,
watchdog/reset policy, UTC estimator, filesystem semantics or file format.

In particular, legacy GPS timing gates, aggressive cold-start/reset choices,
hard-coded power-cycle intervals, sample-bit sequence tagging and infinite
error loops must not override upstream Rust service contracts. Any useful
workaround needs a named failure case, current ST/module evidence and a bounded
Rust implementation with U5 regression coverage where shared code is touched.
Preserve source attribution and inspect licenses before copying any source;
this design review copies no implementation into firmware.

## Acceptance and consequences

Before the Evo board crate is called verified, record the actual board revision,
electrical continuity, legal alternate functions, safe startup levels, measured
clock/polarity and resource ownership for each enabled peripheral. Keep unresolved
rows visibly unsupported. Later evidence updates the candidate map through a
reviewed board revision record; it does not silently rewrite old recordings'
channel identities or U5 fixtures.

This initial reference review is sufficient to plan isolated boot, SD, GPS and
microphone diagnostics. It is deliberately insufficient to claim a complete
working board support package or a field-ready power profile.

## Policy overrides

None. Physical pin and resource evidence does not alter shared policy defaults.
