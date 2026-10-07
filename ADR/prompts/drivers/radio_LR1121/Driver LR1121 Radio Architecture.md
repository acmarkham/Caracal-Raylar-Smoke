# ADR: LR1121 Radio Driver for the Ebyte E80 Module

- **Status:** Proposed
- **Date:** 2026-09-21
- **Decision owners:** Firmware team

## Context

The Raylar board uses an LR1121-based Ebyte E80 radio module. The existing
`unitsmoke/15_ebyte_crate` and `unitsmoke/16_ebyte_crate_rx` applications prove
basic LoRa transmit and receive operation using `lr11xx` version 0.1.0. They
also establish the board-specific integration details:

- SPI with manually controlled chip select;
- `RF_BUSY` and active-low reset;
- `RF_IRQ`, connected to LR1121 DIO9 and an interrupt-capable MCU GPIO;
- a 1.8 V TCXO supply and startup delay;
- the Ebyte module's DIO-controlled RF-switch mapping; and
- packet status retrieval after reception, including RSSI and LoRa SNR.

The product needs a reusable, `no_std`, Embassy-async radio driver rather than
application-specific command sequences. It must support LoRa and GFSK, the
LR1121 frequency ranges exposed by the Ebyte module, low-power operation, and
accurately timed operations suitable for a higher-level contention-free TDMA
service.

## Decision

Create a single-owner async LR1121 driver under `crates/drivers/src/radio`.
The driver will wrap the `lr11xx` crate and own the complete Ebyte radio
hardware interface. It will expose typed, chipset-independent-enough
configuration and operation types while keeping LR1121 command details and
Ebyte board configuration private.

The driver is a hardware driver, not a networking service. A higher layer will
own reliability, addressing, framing, retries, TDMA schedules, UTC conversion,
regional policy, and multi-client arbitration.

## Goals

The driver shall:

- be `no_std`, Embassy async, statically allocated, and heap-free;
- support LoRa and GFSK packet transmission and reception;
- support both sub-GHz and 2.4 GHz operation where supported by the LR1121 and
  the fitted Ebyte module;
- support LoRa and GFSK on the 2.4 GHz RF path with band-specific modulation
  parameters, without substituting a different bandwidth or bitrate;
- validate complete channel configurations before changing radio state;
- timestamp every reported receive event from the `RF_IRQ` edge;
- attach modulation-appropriate packet metadata, including RSSI;
- start RX or TX against a caller-provided monotonic deadline with a target
  error of no more than a few milliseconds;
- make `STBY_XOSC` the normal idle state;
- support retained-configuration sleep; and
- recover to a known state after ordinary radio errors without panicking.

## Non-goals

The initial driver will not provide:

- UTC or GPS awareness;
- TDMA schedule storage or execution;
- packet acknowledgement, retry, deduplication, sequencing, encryption, or
  reliable delivery;
- application addressing or packet framing;
- regional channel plans, listen-before-talk, duty-cycle accounting, or other
  regulatory policy;
- dynamic channel discovery, ranging, Wi-Fi/GNSS scanning, or LR-FHSS;
- multiple simultaneous clients; or
- the LR1121's deepest cold-sleep mode that loses retained configuration.

## Ownership and layering

```text
TDMA / reliable radio service
  - maps UTC slots to monotonic Instant values
  - selects channels and TX policy
  - owns retries, framing and scheduling
                |
                v
LR1121 radio driver (single owner)
  - validates and applies configuration
  - controls state and timed transitions
  - timestamps IRQs and reads packet metadata
                |
                v
lr11xx crate + embedded-hal / embedded-hal-async
                |
                v
Ebyte E80: SPI, CS, BUSY, NRST and RF_IRQ/DIO9
```

The driver exclusively owns SPI, chip select, BUSY, reset, and IRQ resources.
No other code may issue LR1121 commands or clear its IRQ flags. The board layer
continues to construct the concrete pins and peripherals and passes them to the
driver.

The initial API uses exclusive `&mut self` access. It does not contain locks or
an internal multi-client command queue. A future radio service may own the
driver and provide arbitration without changing the hardware abstraction.

## Module structure

The implementation should use small modules with responsibilities similar to:

```text
radio/
  mod.rs          public API and RadioDriver
  config.rs       channel, LoRa, GFSK and TX configuration
  state.rs        state transitions and invariants
  irq.rs          IRQ decoding and receive timestamps
  packet.rs       received packet and metadata types
  ebyte_e80.rs    TCXO, RF switch, PA and band-specific details
  error.rs        validation, timing, transport and radio errors
```

This is guidance rather than a required public module layout. Raw `lr11xx`
operation types should not leak through the public API unless doing so avoids a
meaningless duplicate type.

## Hardware initialization

Construction takes ownership of all hardware but performs no I/O. An async
`initialize()` operation shall:

1. place CS in its inactive state and reset the LR1121;
2. wait for BUSY to deassert with a bounded timeout;
3. initialize the `lr11xx` backend;
4. clear recoverable device and IRQ errors;
5. select the DC-DC regulator mode;
6. configure the Ebyte RF-switch table;
7. configure the 1.8 V TCXO and its startup delay;
8. configure and calibrate the clock and radio as required; and
9. finish in `STANDBY` using `STBY_XOSC`.

The tested RF-switch bytes and TCXO delay from the smoke tests are
module-specific constants and shall live in the Ebyte integration module, not
in application code. Magic raw values must be named and documented with their
source. Initialization failure returns a structured error and never blocks
forever.

Image calibration is band-dependent. The driver shall repeat it when a channel
change crosses an LR1121 calibration band for which the current calibration is
not valid. Calibration and RF-path selection must happen before a timed slot's
start deadline.

## Public operating states

The externally visible state is exactly:

```rust
pub enum RadioState {
    Sleep,
    Standby,
    Rx,
    Tx,
}
```

Initialization and wake-up may have private transient substates but shall not
add public steady states.

### State rules

- `STANDBY` means `STBY_XOSC` and is the default state after initialization,
  packet completion, timeout, or a recoverable error.
- `RX` may be continuous or bounded by a monotonic deadline.
- `TX` is bounded by both the LR1121 hardware timeout and a host-side Embassy
  timeout.
- `SLEEP` uses the LR1121 retained-RAM/retained-configuration option. Entering
  it is explicit because wake latency must be included in a caller's schedule.
- The coldest non-retentive sleep mode is out of scope.
- Channel configuration is applied only from `STANDBY`. Requests made in an
  incompatible state return `InvalidState`, or first use an explicitly named
  operation that aborts the active operation.
- Wake-up finishes in `STANDBY` and verifies BUSY/status before returning.

The driver tracks its expected state in software and reconciles status/IRQ
flags on the next call after a cancelled async future. This makes cancellation
recoverable: cancellation may leave the physical radio in RX or TX briefly,
but the next operation must first stop or normalize it. Callers should still
avoid cancelling an in-flight TX when packet delivery matters.

## Channel configuration

A channel is an immutable, complete description of receive/transmit waveform
compatibility. Frequency and packet format belong to the channel; TX power and
ramp behavior belong to each transmission.

### LR1121 2.4 GHz modulation envelope

Table 3-9 of the LR1121 datasheet is the receiver specification for the
`RFIO_HF` path. It specifies 2400–2500 MHz reception for both LoRa and FSK.
Its 2.4 GHz test points are distinct from the S-band LoRa rows in the same
table and from the general programmable limits in Table 3-7:

| Table 3-9 2.4 GHz condition | Typical RX sensitivity |
| --- | --- |
| LoRa BW406 kHz, SF5 | -111 dBm |
| LoRa BW406 kHz, SF7 | -114 dBm |
| LoRa BW812 kHz, SF5 | -108 dBm |
| LoRa BW812 kHz, SF7 | -112 dBm |
| 2-FSK 1.2 kb/s, 5 kHz deviation, 20 kHz nominal RX BW | -117 dBm |
| 2-FSK 4.8 kb/s, 5 kHz deviation, 20 kHz nominal RX BW | -112 dBm |
| 2-FSK 38.4 kb/s, 40 kHz deviation, 160 kHz nominal RX BW | -103 dBm |
| 2-FSK 250 kb/s, 125 kHz deviation, 500 kHz nominal RX BW | -97.5 dBm |

These are receiver measurements, not an exhaustive list of programmable
profiles. Table 3-9 also characterizes LoRa rejection at BW406/BW812 with
SF7/SF12. It gives neither LoRa coding rates nor raw data rates. Table 3-7
separately gives the 2.4 GHz LoRa range BW203–BW812 and raw-rate endpoints
SF12/BW203/CR4/5 at 0.476 kb/s and SF5/BW812/CR4/5 at 101.5 kb/s. Its
general (G)FSK programmable limits are 0.6–300 kb/s bitrate and 0.6–200 kHz
deviation; the published 2.4 GHz receiver points above reach 250 kb/s.

The LR1121 user manual defines 2.4 GHz LoRa BW203/BW406/BW812 command
values `0x0D`/`0x0E`/`0x0F`, SF5–SF12, short-interleaver CR4/5, 4/6,
4/7, 4/8, and long-interleaver CR4/5, 4/6, 4/8. Long-interleaver payload
limits apply. The driver must preserve BW, SF, CR, and interleaver mode
exactly and validate band-specific bandwidths. GFSK has no LoRa SF or CR.

The datasheet's 20/160/500 kHz FSK sensitivity bandwidths are nominal test
conditions for unfiltered 2-FSK; they are not GFSK sensitivity guarantees.
Use an explicit Gaussian pulse shape for GFSK profiles. The LR1121
`SetModulationParams` filter table exposes 19.5,
156.2, and at most 467 kHz DSB settings, respectively; it has no exact
20/160/500 kHz settings. In particular, the current driver's conservative
`bitrate + 2 * deviation <= RX bandwidth` check rejects 250 kb/s with
125 kHz deviation even at 467 kHz. Do not invent a 500 kHz enum or silently
map the 500 kHz condition to 467 kHz. Verify the 250 kb/s operating point
against the LR1121 user manual and hardware, then document the selected
filter, frequency-error allowance, and any justified validation change before
enabling that profile. The lower-rate examples may use documented 19.5 and
156.2 kHz filter settings after validation and on-board RX testing; record
the actual setting alongside the nominal datasheet condition.

Sources: [Semtech LR1121 Datasheet, Rev 2.1, Tables 3-7 and 3-9](https://static6.arrow.com/aropdfconversion/558d7379c488375138d6317a5a5c06f1a144bd3/61252685.lr1121_v2_1_data_sheet.pdf)
and [Semtech LR1121 User Manual, Rev 1.1, Sections 8.3.1 and 8.5.1](https://www.mouser.com/pdfdocs/usermanual_lr1121_v1_1.pdf).

Conceptually:

```rust
pub struct ChannelConfig {
    pub frequency_hz: u32,
    pub modulation: ModulationConfig,
}

pub enum ModulationConfig {
    LoRa(LoRaChannel),
    Gfsk(GfskChannel),
}

pub struct LoRaChannel {
    pub spreading_factor: LoRaSpreadingFactor,
    pub bandwidth: LoRaBandwidth,
    pub coding_rate: LoRaCodingRate,
    pub low_data_rate_optimization: LowDataRateOptimization,
    pub preamble_symbols: u16,
    pub header: LoRaHeaderMode,
    pub payload_length: Option<u8>,
    pub crc: bool,
    pub invert_iq: bool,
    pub sync_word: u8,
}

pub struct GfskChannel {
    pub bit_rate_bps: u32,
    pub frequency_deviation_hz: u32,
    pub receiver_bandwidth: GfskBandwidth,
    pub pulse_shape: GfskPulseShape,
    pub preamble_bits: u16,
    pub preamble_detector: GfskPreambleDetector,
    pub sync_word: heapless::Vec<u8, MAX_GFSK_SYNC_WORD_LEN>,
    pub address_filtering: GfskAddressFiltering,
    pub packet_length: GfskPacketLength,
    pub crc: GfskCrc,
    pub whitening: bool,
}

pub struct TxConfig {
    pub power_dbm: i8,
    pub ramp_time: TxRampTime,
}
```

An explicit LoRa payload length is required for implicit-header mode and is
optional for explicit-header mode. GFSK configuration shall also represent any
fixed/variable length, CRC, whitening, address, and sync-word fields needed for
two peers to interoperate.

`LowDataRateOptimization::Auto` should be available and should be the default;
the driver derives the correct setting from symbol duration. An explicit
override may be retained for testing and unusual interoperability needs.

Configuration validation occurs before any register changes and covers at
least:

- chipset and module frequency limits;
- supported bandwidth, spreading-factor and coding-rate combinations for the
  selected band, including 2.4 GHz-only LoRa BW203/BW406/BW812;
- LoRa implicit-header payload requirements;
- GFSK bitrate, deviation and receiver-bandwidth relationships;
- preamble, sync-word and payload bounds;
- PA path and legal hardware power range for the selected band; and
- caller buffer sizes.

Changing between LoRa and GFSK, or between RF bands, must fully apply all
modulation, packet, RF-switch, PA, calibration, IRQ, and fallback settings.
The implementation must not depend on register state left by a previous
channel.

Regional limits are intentionally not embedded in `ChannelConfig`. The
higher-level service supplies a policy-approved frequency and power; the
driver rejects values unsafe or unsupported by the hardware.

## Receive operation and packet metadata

The caller provides the payload buffer. The driver does not allocate or retain
it. A receive result conceptually has this form:

```rust
pub struct ReceivedPacket<'a> {
    pub payload: &'a [u8],
    pub metadata: RxMetadata,
}

pub struct RxMetadata {
    pub packet_complete_at: embassy_time::Instant,
    pub frequency_hz: u32,
    pub metrics: RxMetrics,
}

pub enum RxMetrics {
    LoRa {
        rssi_dbm_x2: i16,
        signal_rssi_dbm_x2: i16,
        snr_db_x4: i16,
    },
    Gfsk {
        rssi_dbm_x2: i16,
        status: GfskPacketStatus,
    },
}
```

Fixed-point physical units avoid floating-point requirements and must be
documented. The exact scale may change during implementation if the `lr11xx`
crate already exposes a clear lossless representation.

The IRQ path shall:

1. wait asynchronously for the rising edge of `RF_IRQ`/DIO9;
2. capture `embassy_time::Instant::now()` immediately after the edge wakes the
   task, before any SPI transaction or logging;
3. read and decode all pending LR1121 IRQ flags;
4. on RX-done, read buffer status, payload, and modulation-specific packet
   status;
5. clear only the handled flags; and
6. return the packet and metadata, or a typed receive event/error.

`packet_complete_at` is the local monotonic timestamp of the RX-done IRQ. It
represents end-of-packet detection, not arrival of the first preamble symbol.
This semantic must remain stable. If later protocol work requires a
start-of-packet timestamp, it must either derive it from time-on-air with a
documented uncertainty or add dedicated hardware capture support.

CRC error, header error, timeout, command error, and generic radio error IRQs
must retain the same captured IRQ timestamp in diagnostics. Invalid packets
are not returned as successful payloads. Counters ensure these events are not
silently lost.

The initial timestamp accuracy target is within a few milliseconds. The IRQ
task must contain no polling delay and must do no logging before capture. If
measurements show unacceptable scheduler latency, the same API can be backed
by timer input capture or a minimal EXTI ISR without changing timestamp
semantics.

## Timed RX and TX for TDMA

The driver operates only in the local monotonic time domain. It accepts
`embassy_time::Instant`; it never accepts UTC values and never reads GPS. The
higher-level time/TDMA service converts UTC slot boundaries to monotonic
deadlines.

Conceptual operations are:

```rust
async fn prepare_channel(&mut self, channel: &ChannelConfig) -> Result<()>;

async fn receive_at<'a>(
    &'a mut self,
    start: Instant,
    end: Instant,
    buffer: &'a mut [u8],
) -> Result<ReceivedPacket<'a>>;

async fn transmit_at(
    &mut self,
    start: Instant,
    payload: &[u8],
    tx: &TxConfig,
) -> Result<TxReport>;

async fn standby(&mut self) -> Result<()>;
async fn sleep(&mut self) -> Result<()>;
async fn wake(&mut self) -> Result<()>;
```

The final Rust API may separate preparation and arming more strongly, but it
must preserve these semantics:

- expensive channel setup, TCXO wake-up, calibration, PA selection, payload
  upload, and IRQ routing happen before the requested start;
- the final `SetRx` or `SetTx` command is issued as close as possible to
  `start` using `Timer::at` or equivalent;
- a configurable preparation guard time accounts for measured worst-case SPI,
  BUSY, TCXO, calibration, and wake latency;
- a request received too late returns `DeadlineMissed` rather than silently
  starting in the wrong slot;
- `receive_at` never waits beyond `end`, apart from bounded cleanup time;
- RX timeout, TX timeout, and completion are IRQ-driven with a host-side
  timeout as a fault backstop; and
- completion or timeout returns the radio to `STBY_XOSC`.

For a window containing multiple packets, the higher layer repeatedly calls a
receive-one operation against the same end deadline. The implementation should
avoid reapplying an unchanged channel and may keep RX armed between packet
reads where the LR1121 behavior permits it. No packet queue is required in the
initial driver.

`TxReport` should include the requested start, the monotonic time immediately
before or after the final `SetTx` command, the TX-done IRQ timestamp, and the
result. This allows TDMA integration tests to measure scheduling error without
claiming a precision that has not been measured.

## Low-power behavior

Low-power behavior is part of the API contract:

- `STBY_XOSC` is the default idle and fallback mode because it gives
  predictable, short transition latency for scheduled slots.
- RX boost is a channel/driver policy with an explicit power trade-off; it
  shall not be enabled accidentally by stale configuration.
- Retained `SLEEP` is opt-in for longer idle periods. Its documented minimum
  useful interval shall include sleep entry, wake, TCXO stabilization, and
  schedule guard time.
- The driver must not busy-poll BUSY or IRQ. It should use async GPIO waits with
  bounded timeout support.
- The current smoke-test millisecond polling adapters are prototypes, not the
  production waiting strategy.

## Errors, recovery, and observability

Public errors should distinguish:

- invalid or unsupported configuration;
- unsupported frequency or TX power;
- invalid state;
- missed deadline or invalid time window;
- caller buffer too small;
- SPI/CS/BUSY/reset transport failure;
- BUSY, RX, or TX timeout;
- CRC or header rejection;
- LR1121 command and device errors; and
- wake-up or calibration failure.

Recoverable packet errors do not panic and do not terminate the driver. The
driver clears the relevant IRQ, records the event, and normally returns to
`STBY_XOSC`. After transport or command errors it should attempt bounded
normalization; if the state is uncertain, the next explicit `initialize()` or
`recover()` performs a hardware reset and complete reconfiguration.

Expose a snapshot suitable for an `embassy_sync::watch` owned by a higher-level
service. The driver itself need not spawn a publishing task. Suggested fields
are:

```rust
pub struct RadioStats {
    pub state: RadioState,
    pub rx_packets: u32,
    pub tx_packets: u32,
    pub crc_errors: u32,
    pub header_errors: u32,
    pub rx_timeouts: u32,
    pub tx_timeouts: u32,
    pub command_errors: u32,
    pub transport_errors: u32,
    pub deadline_misses: u32,
    pub resets: u32,
}
```

Counters use saturating or explicitly wrapping behavior and shall not cause a
panic.

## Memory and concurrency constraints

- No heap allocation.
- Caller-owned payload buffers are borrowed only for the duration of a call.
- Fixed-capacity fields use arrays or `heapless`.
- No `unsafe` is introduced solely for the driver.
- SPI transactions and shared state have one owner.
- A receive packet is delivered exactly once to its caller; a future service
  that queues packets should use a bounded channel rather than a watch.
- A watch is appropriate for state/statistics because consumers require the
  latest snapshot, not every historical value.

## Verification plan

No firmware is implemented by this ADR. When implementation is authorized,
verification should include:

### Host tests

- valid and invalid LoRa/GFSK configuration combinations;
- 2.4 GHz LoRa BW203/BW406/BW812 with SF5/SF12 and CR4/5, rejection of those
  bandwidths on sub-GHz, and rejection of unsupported 2.4 GHz bandwidths;
- the documented 2.4 GHz FSK reference triples, including an explicit result
  for the unresolved 250 kb/s / 125 kHz deviation / nominal 500 kHz case;
- band and PA selection at all boundaries;
- low-data-rate optimization derivation;
- state-machine transitions and invalid operations;
- deadline/guard-time calculations, including wrap-safe time comparisons;
- IRQ decoding when multiple flags are set;
- fixed-point packet-status conversion; and
- cancellation recovery using a mocked backend.

### Hardware tests

Build on `unitsmoke/15_ebyte_crate` and `unitsmoke/16_ebyte_crate_rx` to verify:

- LoRa TX/RX at 868 MHz, then at another supported band;
- GFSK TX/RX with matching complete channel configurations;
- 2.4 GHz LoRa and GFSK TX/RX at the documented settings, including the
  actual programmable GFSK RX filter used for each nominal test condition;
- RSSI and modulation-specific metadata on every valid packet;
- a unique monotonic RX-done timestamp for each packet;
- channel changes between LoRa/GFSK and sub-GHz/2.4 GHz;
- TX power selection across each supported PA path;
- STBY_XOSC, retained sleep, wake, and configuration retention;
- timed RX/TX against PPS-derived monotonic deadlines; and
- measured start error and IRQ timestamp latency under representative Embassy
  task load.

Acceptance for the initial timed API is a worst-case observed scheduling error
within a few milliseconds. Measurements, board revision, firmware build, SPI
frequency, guard time, and test load must be recorded before tightening that
guarantee.

## Consequences

### Positive

- TDMA and reliable-delivery policy remain independent of chipset commands.
- Complete typed channels make peer compatibility explicit and reduce stale
  register configuration errors.
- IRQ-first timestamp capture provides stable receive timing semantics and a
  path to hardware capture if later required.
- Single ownership and caller-provided buffers keep memory use predictable.
- Explicit power states make energy/latency trade-offs visible to services.

### Negative

- The wrapper must maintain mappings between public types and `lr11xx` types.
- Preparing before a precise slot requires the higher layer to submit work
  early enough for the guard time.
- `STBY_XOSC` consumes more power than RC standby, but is chosen for predictable
  scheduled-radio latency.
- An IRQ timestamp marks packet completion rather than the first received bit.
- A single-owner API requires a higher-level service when multiple clients are
  introduced.

## Open questions to resolve during implementation

- Confirm the exact Ebyte E80 variant's certified frequency and PA limits from
  its hardware documentation; LR1121 capability alone is not sufficient.
- Measure whether Embassy EXTI wake latency meets the timestamp target under
  worst-case application load.
- Measure guard times for same-channel operation, band changes, and wake from
  retained sleep.
- Confirm which `lr11xx` 0.1.0 operations require a thin local extension for
  GFSK status, retained sleep, or typed STBY_XOSC selection.
- Decide whether rejected-packet diagnostics are returned directly as receive
  events or exposed only through counters and logging; successful packet API
  semantics must remain unchanged.

