# ADR-0013: Identity and Versioning Service

## Status

Proposed

---

# Context

The firmware requires a central service for reporting device identity, firmware versioning and hardware traceability information.

Several lower-level drivers and services expose pieces of identity-related information:

* Traceability / Identity Driver
* Storage / SD card driver
* GPS Driver
* Radio Driver
* Build metadata
* Board crate / MCU support crate

Higher-level services should not query these sources independently. Instead, the system should provide one coherent identity and versioning view that can be used by:

* Logging Service
* Storage Service
* Diagnostics
* Manufacturing tests
* Field support
* Telemetry
* Deployment records

This service is distinct from the low-level Traceability Driver. The Traceability Driver exposes the STM32 factory UID and firmware image hash. The Identity and Versioning Service aggregates this and combines it with other system identity information.

---

# Decision

Introduce a dedicated Identity and Versioning Service.

The service aggregates identity and version information from multiple sources and publishes a single `IdentityState`.

The service should expose:

* STM32 factory UID
* derived short serial IDs
* firmware version
* firmware hash / CRC
* build metadata
* STM32 device type number
* board revision, if available
* SD card identity
* GPS module version
* radio module version
* any future hardware module identity

The service is read-mostly and should be safe to query from diagnostics, logging and support tooling.

---

# Design Goals

The Identity and Versioning Service shall:

* Provide one authoritative system identity view.
* Be heapless.
* Avoid dynamic allocation.
* Be safe to use early during startup where possible.
* Aggregate identity from multiple subsystems.
* Publish a stable identity state.
* Support optional fields where hardware modules are absent or not yet queried.
* Make logs and stored data traceable to both hardware and firmware.
* Keep low-level identity mechanisms out of application code.

---

# Non-Goals

The service is **not** responsible for:

* secure boot
* cryptographic authentication
* firmware signature verification
* anti-cloning protection
* cloud registration
* manufacturing database synchronisation
* provisioning secrets
* key storage

The service reports identity. It does not prove trust.

---

# Architecture

```text
Traceability Driver ───────┐
                           │
Storage / SD Driver ───────┤
                           │
GPS Driver ────────────────┤
                           │
Radio Driver ──────────────┤
                           │
Board / MCU Metadata ──────┤
                           ▼
             Identity and Versioning Service
             ┌───────────────────────────┐
             │ Device Identity           │
             │ Firmware Identity         │
             │ Hardware Module Versions  │
             │ Published Identity State  │
             └─────────────┬─────────────┘
                           ▼
                  Watch<IdentityState>

Consumers:
- Logging Service
- Storage Service
- Diagnostics
- Manufacturing tests
- Telemetry
```

---

# Responsibilities

The Identity and Versioning Service is responsible for:

* collecting identity fields from lower-level drivers
* caching identity fields once discovered
* publishing a coherent identity snapshot
* providing a simple read API for consumers
* distinguishing known, unknown and unavailable identity fields
* updating module version fields if they become available later

The service should not duplicate low-level logic already owned by drivers.

---

# Identity State

Suggested structure:

```rust
pub struct IdentityState {
    pub device: DeviceIdentity,
    pub firmware: FirmwareIdentity,
    pub hardware: HardwareIdentity,
}

pub struct DeviceIdentity {
    pub stm32_uid_96: DeviceUid,
    pub serial_64: u64,
    pub serial_48: u64,
    pub serial_32: u32,
    pub serial_16: u16,
    pub stm32_device_code: Option<Stm32DeviceCode>,
}

pub struct FirmwareIdentity {
    pub version: Option<&'static str>,
    pub git_hash: Option<&'static str>,
    pub build_timestamp: Option<&'static str>,
    pub build_profile: Option<&'static str>,
    pub runtime_crc32: Option<u32>,
    pub build_crc32: Option<u32>,
}

pub struct HardwareIdentity {
    pub board_revision: Option<&'static str>,
    pub sd_card: Option<SdCardIdentity>,
    pub gps_module: Option<GpsModuleIdentity>,
    pub radio_module: Option<RadioModuleIdentity>,
}
```

Exact field names may evolve during implementation.

---

# STM32 Device Identity

The service should expose both:

1. The STM32 factory UID.
2. The actual STM32 device type / part number.

These are different concepts.

The STM32 factory UID identifies the specific physical microcontroller.

Example:

```text
UID: 96-bit factory unique identifier
```

The STM32 device type identifies what chip variant is fitted.

Example:

```text
STM32U595xx
STM32U595VIT6
STM32U595ZIT6
```

The exact representation depends on what can be obtained from the HAL, PAC, DBGMCU IDCODE, compile-time target metadata, or board crate.

Suggested representation:

```rust
pub struct Stm32DeviceCode {
    pub family: &'static str,
    pub device: &'static str,
    pub package: Option<&'static str>,
}
```

If the exact orderable part number cannot be read at runtime, it may be provided at compile time by the board crate.

---

# Firmware Versioning

Firmware identity should include build-time metadata where available.

Suggested fields:

* semantic version
* git commit hash
* build timestamp
* build profile
* target board revision
* build-time firmware CRC
* runtime-calculated firmware CRC

The service should pass through firmware metadata from the Traceability Driver or build metadata module rather than recalculating it itself unless explicitly required.

---

# SD Card Identity

The service should expose SD card identity if available from the storage stack.

Useful fields may include:

```rust
pub struct SdCardIdentity {
    pub manufacturer_id: Option<u8>,
    pub oem_id: Option<[u8; 2]>,
    pub product_name: Option<[u8; 5]>,
    pub product_revision: Option<u8>,
    pub serial_number: Option<u32>,
    pub manufacture_year: Option<u16>,
    pub manufacture_month: Option<u8>,
    pub capacity_bytes: Option<u64>,
}
```

The exact fields should match what the SDMMC / SD card layer can reliably provide.

If the SD card has not yet been initialised, this field should remain `None` or `Unknown`.

---

# GPS Module Identity

The service should expose GPS module identity and firmware version where available.

The GPS Driver remains responsible for communicating with the GPS module and retrieving any module-specific version strings.

The Identity and Versioning Service only consumes and republishes this information.

Suggested structure:

```rust
pub struct GpsModuleIdentity {
    pub vendor: Option<&'static str>,
    pub model: Option<&'static str>,
    pub firmware_version: Option<FixedString>,
    pub protocol_version: Option<FixedString>,
    pub hardware_version: Option<FixedString>,
}
```

Because module version strings may be read at runtime, use a heapless fixed-capacity string type rather than heap allocation.

If the GPS module has not yet responded, the identity should be marked unknown rather than blocking the entire service.

---

# Radio Module Identity

The service should expose radio module identity and version where available.

The Radio Driver remains responsible for querying the radio module.

Suggested structure:

```rust
pub struct RadioModuleIdentity {
    pub vendor: Option<&'static str>,
    pub model: Option<&'static str>,
    pub firmware_version: Option<FixedString>,
    pub hardware_version: Option<FixedString>,
    pub protocol_version: Option<FixedString>,
}
```

The exact fields should follow the capabilities of the actual radio module.

---

# Known, Unknown and Unavailable Fields

The service should distinguish between three cases:

```text
Known
Unknown
Unavailable
```

For example:

* `Known`: GPS module responded with firmware version.
* `Unknown`: GPS module has not yet been queried.
* `Unavailable`: hardware or driver does not support this field.

This avoids ambiguity between "not yet discovered" and "not supported".

A possible representation:

```rust
pub enum IdentityField<T> {
    Known(T),
    Unknown,
    Unavailable,
}
```

This may be more explicit than using `Option<T>` for every field.

---

# Publishing Model

The service should publish `IdentityState` using an Embassy watch.

```rust
Watch<IdentityState>
```

The state may initially contain only information available at boot.

As drivers initialise and hardware modules respond, the service may update the watch with additional identity fields.

Consumers should tolerate partial identity.

---

# Startup Behaviour

The service should start with immediately available identity:

* STM32 UID
* derived serial IDs
* STM32 device code if compile-time known
* firmware build metadata
* board revision if compile-time known

Later fields may become available asynchronously:

* SD card ID
* GPS module version
* radio module version
* runtime firmware CRC

The service should not block system startup while waiting for slow or optional hardware identity queries.

---

# Relationship to Traceability Driver

The Traceability Driver remains responsible for:

* reading STM32 UID
* deriving serial IDs
* calculating firmware CRC
* reading embedded firmware metadata

The Identity and Versioning Service consumes this information and presents it alongside module identities.

---

# Relationship to Logging Service

The Logging Service should be able to include an identity header at the beginning of the logfile.

Example:

```text
Device UID:      00112233-44556677-8899AABB
Serial64:        0x123456789ABCDEF0
Serial32:        0x9F34A102
MCU:             STM32U595xx
Firmware:        1.2.0
Git:             a1b2c3d
Build:           2026-09-16T12:00:00Z
Firmware CRC32:  0x7C91D42E
SD Card:         MID=0x03 SN=0x12345678 Capacity=128GB
GPS Module:      <model> FW=<version>
Radio Module:    <model> FW=<version>
```

The Identity Service should not depend on the Logging Service.

---

# Relationship to Storage Service

The Storage Service may use identity fields in metadata files or manifests.

Examples:

* device serial in deployment metadata
* firmware version in recording manifest
* SD card identity in storage diagnostics
* GPS and radio module versions in support reports

The Identity Service should not own file writing.

---

# Relationship to Diagnostics and Support

Diagnostics commands may query the Identity Service to produce a compact support report.

This report should allow a non-technical user or field engineer to identify:

* which physical device is running
* which firmware is installed
* which STM32 device variant is fitted
* which SD card is installed
* which GPS module version is installed
* which radio module version is installed

---

# Error Handling

The service should handle missing or unavailable information without panicking.

Examples:

* SD card absent
* GPS module not powered
* GPS version query timeout
* radio module absent
* firmware CRC unavailable
* board revision not encoded

In these cases, the published identity state should remain valid but partial.

---

# Test

Build a small integration test in:

```text
servicetests/identity
```

The test should:

* start the Traceability Driver
* start the Identity and Versioning Service
* optionally start or mock the Storage, GPS and Radio drivers
* print or log the current `IdentityState`
* verify that the STM32 UID and derived serials are populated
* verify that firmware version metadata is populated if available
* verify that the STM32 device code is populated if available
* verify that SD card identity is populated when an SD card is present
* verify that GPS module version is populated when the GPS module responds
* verify that radio module version is populated when the radio module responds
* verify that unavailable fields are represented explicitly and do not cause failure

The test should be suitable for both hardware-in-the-loop and mocked module identity providers.

---

# Future Extensions

Possible future additions include:

* USB diagnostic identity endpoint
* RF identity advertisement
* manufacturing metadata block
* calibration identity
* board serial number separate from MCU UID
* sensor module identities
* signed firmware metadata
* secure firmware measurement
* human-readable support code generation
* device manifest file generation on SD card

These features should not complicate the initial aggregation service.

---

# Consequences

## Advantages

* Provides one coherent identity and versioning view.
* Keeps low-level UID, SD, GPS and radio version mechanisms isolated in their drivers.
* Simplifies logging and diagnostics.
* Supports partial identity during startup.
* Improves field support and manufacturing traceability.
* Avoids duplicating identity-gathering logic across services.

## Disadvantages

* Requires coordination between multiple drivers.
* Some identity fields become available only after hardware initialisation.
* Module version strings require fixed-capacity storage.
* The service must handle partial and stale information carefully.

These trade-offs are acceptable because identity and versioning are cross-cutting system concerns and should be centralised in a thin aggregation service.
