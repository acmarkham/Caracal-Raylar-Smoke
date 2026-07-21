# ADR: Microphone Driver Architecture


---

# Context

The platform uses digital PDM microphones connected to the STM32U5 MDF (Multi-function Digital Filter) peripheral.

A prototype implementation has already demonstrated successful operation using:

* MDF filters
* DMA double-buffering
* PDM microphones
* Embassy async tasks


The prototype you should look at is: unitsmoke/20_pdm_mic_all_dma as this has the basic framework that you should build on.

You should *not* use earlier variants (e.g. 19_pdm_micarray_dma and 18_pdm_mic1_dma and 10_pdm_mic1_mono) as these are incorrect.

The board-specific pin mapping is defined within the `board` crate and should not be duplicated within the driver.

The microphone driver should provide a reusable abstraction over the STM32 MDF peripheral while remaining independent of higher-level concepts such as WAV files, storage or audio processing.

---

# Decision

Implement a dedicated Microphone Driver that owns the STM32 MDF peripherals and DMA resources.

The driver continuously captures microphone samples into DMA double buffers and publishes completed buffers to consumers.

The driver is responsible only for acquisition.

Higher-level services own:

* Audio recording
* WAV formatting
* Compression
* DSP
* Voice detection
* Storage

---

# Design Goals

The Microphone Driver shall:

* Support runtime configuration.
* Support mono and hexaphonic capture.
* Operate continuously using DMA.
* Minimise CPU overhead.
* Be heapless.
* Be zero-copy.
* Allow low-latency or low-power operation through configurable DMA buffer sizes.
* Remain independent of storage and audio processing.
* Timestamp (according to system time) the interrupt time of the DMA half/full ready signals to be used for upper level precise synchronization/timestamping. 
* Very importantly: timestamp the precise time the filter(s) are started and make this part of the mic-array state.
* Keep track of how many buffers have been completed as this also useful (assuming no buffers are missed/dropped) for sample-level timing.

---

# Non-Goals

The driver is **not** responsible for:

* WAV file generation
* Audio compression
* Beamforming
* FFT processing
* Voice activity detection
* File writing
* Timestamping - this will have to be done in conjunction with the Time Service (uses GPS PPS EXTI time)
* Audio synchronisation

These belong to higher-level services.

---

# Existing Prototype

An existing prototype demonstrates:

* MDF configuration
* DMA operation
* Successful microphone capture

The implementation should build upon this work rather than replacing it.

The prototype should be treated as the reference implementation for MDF configuration. This is unitsmoke/20_pdm_mic_all_dma. This prototype does use a specific RAM bank, but this should not be necessary in this initial implementation - general purpose RAM should be perfectly sufficient.

---

# Architecture

```text
          Audio Service
                │
                ▼
        Microphone Driver
                │
         DMA Double Buffers
                │
                ▼
          STM32 MDF Filters
                │
                ▼
         PDM Microphones
```

The driver owns:

* MDF peripherals
* DMA channels
* Clock generation
* Double buffers

Consumers never interact directly with MDF or DMA.

---

# Runtime Configuration

The driver should support runtime configuration through a single configuration structure.

Suggested fields include:

```rust
pub struct MicrophoneConfig {
    microphone_mode,
    sample_rate,
    bit_depth,
    sample_packing,
    high_pass_filter,
    sinc_filter,
    decimation,
    dma_buffer_blocks,
}
```

The exact representation may evolve.

---

# Microphone Configuration

Initially support:

```text
Mono
```

Uses:

* MCO1

and

```text
Hexaphonic
```

Uses:

* MCO1
* MCO2

The number of microphones should be selectable at runtime.

Current supported values:

* 1 microphone
* 6 microphones

The architecture should allow additional configurations in future if required.

---

# Sampling Rates

Support preset sample rates.

Initially:

* 8 kHz
* 16 kHz
* 32 kHz
* 44.1 kHz
* 96 kHz

The driver owns all clock configuration required to realise these rates.

---

# Clock Generation

The driver is responsible for selecting an appropriate microphone clock.

Clock selection depends on:

* sample rate
* filter selection
* decimation ratio

The generated clock must remain within the supported operating range of the IM69D129FV01 microphones.

Reference operating ranges:

| Mode             |         Clock Range |
| ---------------- | ------------------: |
| Standby          |             330 kHz |
| Low Power        |  380 kHz – 1.02 MHz |
| Normal           | 1.17 MHz – 1.70 MHz |
| High Performance | 1.90 MHz – 3.40 MHz |

The driver should validate requested configurations and reject combinations that violate microphone operating limits.

Clock generation should remain an implementation detail hidden from applications. The microphone clock will always be derived from 16MHz HSE for accuracy.

---

# High Pass Filter

Allow runtime selection of the MDF high-pass filter.

Initially support:

* Enabled
* Disabled

The default behaviour should provide approximately a 10 Hz roll-off suitable for removing microphone DC offset.

---

# Sample Format

Support runtime selection of:

Bit depth:

* 16-bit
* 24-bit

Packing:

* 16-bit
* 24-bit
* 32-bit

This allows the capture format to be optimised for storage, DSP or bandwidth.

---

# MDF Filter Configuration

Allow runtime selection of:

Filter:

* Sinc4
* Sinc5

Decimation ratio.

The driver owns the mapping between:

* requested sample rate
* decimation ratio
* microphone clock frequency
* MDF configuration

Applications should not manipulate MDF registers directly.

---

# DMA Buffering

The driver continuously fills DMA double buffers.

The DMA buffer size should be selectable at compile time.

Buffer size directly influences:

* interrupt frequency
* CPU wake-up interval
* processing latency
* power consumption

This allows applications to optimise for either latency or efficiency.

---

# Buffer Ownership

DMA is free-running.

Consumers receive references to completed DMA buffers.

The consumer is best-effort.

If DMA overwrites a buffer before the consumer has completed processing:

* capture continues uninterrupted
* no backpressure is applied
* data loss is considered acceptable

Continuous acquisition has priority over guaranteed delivery.

This behaviour is intentional.

---

# Buffer Ready Notification

Mono capture:

A notification is generated each time a DMA half-buffer or full-buffer becomes available.

Hexaphonic capture:

Each MDF filter completes simultaneously.

The driver should therefore generate a single notification only after the final channel buffer is ready.

Consumers should receive one coherent audio frame containing all microphone channels.

Per-channel notifications are unnecessary.

---

# Published Interface

The driver should publish completed buffers using Embassy synchronisation primitives.

The exact mechanism remains open.

Possible approaches include:

* Signal
* Watch
* Channel

The published interface should avoid copying audio samples.

Zero-copy buffer ownership is preferred.

---

# Error Handling

The driver should detect and report:

* DMA errors
* MDF configuration errors
* Unsupported sampling configurations
* Clock generation failures

Capture should continue whenever recovery is possible.

---

# Driver Independence

The driver should remain independent of:

* Storage Service
* Logging Service
* Audio recording
* WAV formatting
* Time Service

It provides only microphone acquisition.

The main "consumer" of the mic_array driver is the audio service. This will handle longer buffering, passing on to other services like neural network detection, wav formatting, time-alignment etc.


---

# Future Extensions

The architecture should naturally support:

* Additional microphone counts
* Runtime gain control (if available)
* TDM microphones
* Different PDM microphone families
* Synchronisation with external clocks
* Hardware timestamping
* Beamforming
* Audio triggering
* Multiple simultaneous consumers

These features should not require significant architectural changes.

---

# Consequences

## Advantages

* Clear separation between acquisition and audio processing.
* Continuous zero-copy capture.
* Runtime configurable operating modes.
* Efficient DMA-based implementation.
* Flexible trade-off between latency and power consumption.
* Independent of storage and recording policy.
* Naturally supports future DSP and beamforming services.

## Disadvantages

* Continuous DMA capture may overwrite data if consumers fall behind.
* Configuration validation is more complex due to interactions between sample rate, filter type, decimation ratio and microphone clock frequency.
* Additional runtime configuration increases driver complexity.

These trade-offs are acceptable because reliable continuous acquisition is the primary objective, and higher-level services can choose the appropriate buffering and processing strategies for their application.


# Implementation Specifics

The driver should be implemented under crates/drivers/mic_array

Split out the very hardware specific functions into their own config file e.g. properties such as the MDF base address and the individual register offsets:

const MDF1_BASE: usize = 0x4002_5000;
const MDF_GCR: usize = 0x0000;
const MDF_CKGCR: usize = 0x0004;
const MDF_FILTER_STRIDE: usize = 0x0080;
const MDF_SITFCR0: usize = 0x0080;
const MDF_BSMXCR0: usize = 0x0084;
const MDF_DFLTCR0: usize = 0x0088;
const MDF_DFLTCICR0: usize = 0x008c;
const MDF_DFLTRSFR0: usize = 0x0090;
const MDF_DFLTDR0: usize = 0x00f0;

should live in their own file. This will make porting to earlier variants e.g. that use DFDSM instead of MDF easier.

Use a default config of 24 bit, 32 bit packing, 16kHz, Sinc5, high-pass-fiter enabled.

# Testing

Make a small, relatively standalone testsuite under:
drivertests/mic_array

This should init the microphone(s), start them running, and then just print out simple stats like min/max level and dB RMS level at a rate of ~1Hz. Flag any error codes that arise. 