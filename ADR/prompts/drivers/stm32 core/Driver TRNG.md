# ADR: STM32U5 True Random Number Generator Driver

## Status

Accepted

---

# Context

Raylar firmware needs random values from the STM32U5 true random number
generator (TRNG). One use is a boot ID: a value that identifies a single boot
session, stays unchanged for the whole powered interval, and is newly generated
after the next reset or power cycle. This ID can correlate logs and other
session-scoped data without writing a counter to non-volatile storage.

The STM32U5 TRNG is a hardware peripheral. Its clocking, initialization,
health/error status and data-ready behavior belong in a low-level driver rather
than in consumers such as logging or storage.

---

# Decision

Add a dedicated STM32U5 TRNG driver in `crates/drivers`, with two public
operations:

```rust
pub fn latest_trng() -> Result<u32, TrngError>;

pub fn boot_id() -> Result<u32, TrngError>;
```

`latest_trng()` obtains a fresh word from the hardware peripheral for each
successful call. It waits for data readiness within a bounded timeout and
reports peripheral, health-test or timeout failures. It does not return a
previously cached value when a fresh read fails.

`boot_id()` is a one-shot memoizing operation. On its first successful call in
a boot session it obtains a word from `latest_trng()` and stores it in
static, heapless RAM. Later calls return that same value without reading the
peripheral again. If generation fails, it returns the error and leaves the
cache unset so a later call can retry. The cache is naturally cleared by MCU
reset or power loss; the ID is not persisted in flash or backup registers.

Conceptual implementation:

```rust
static BOOT_ID: Mutex<NoopRawMutex, RefCell<Option<u32>>> = ...;

pub fn boot_id() -> Result<u32, TrngError> {
    // If set, return the cached ID. Otherwise generate, cache on success,
    // and return it. Synchronize access if this API can be called concurrently.
}
```

The exact synchronization primitive should match the crate's execution model.
The implementation must ensure concurrent first calls cannot publish
different IDs. If TRNG acquisition is asynchronous in the selected HAL, the
public operation may be async rather than blocking; the one-shot caching
semantics remain the same.

---

# Boot ID Semantics

The boot ID is a session identifier, not a persistent device identifier and
not a security credential. It is intended for associating data produced during
one powered session. Consumers should obtain it through `boot_id()` rather
than caching their own independently generated values.

The initial representation is one 32-bit TRNG word. This gives a compact ID
with a finite collision probability across boot sessions. If fleet-wide
uniqueness requirements exceed that probability, the representation can be
expanded using multiple independently obtained words, while preserving the
same memoizing API contract.

---

# TRNG Driver Responsibilities

The driver owns or coordinates:

* STM32U5 TRNG peripheral initialization and clock requirements;
* waiting for fresh data with a finite timeout;
* checking and reporting peripheral health/error status;
* returning a fresh random word to callers;
* generating and memoizing the current boot ID in RAM.

It does not own:

* log/session metadata formatting;
* persistent boot counters or storage;
* cryptographic key lifecycle or security policy;
* statistical post-processing unless required by the STM32 reference manual
  or HAL contract.

The implementation shall follow the STM32U5 reference manual and the selected
Embassy/HAL API for TRNG startup, data-ready handling, conditioning and health
tests. It must not bypass hardware error indications or treat an error as
random output.

---

# API and Error Handling

The API is synchronous if the selected HAL can provide bounded synchronous
access without blocking an executor. Otherwise use async functions with the
same names and semantics. Errors should distinguish at least:

* initialization or clock configuration failure;
* hardware health/error status;
* data-ready timeout;
* any HAL-specific acquisition failure that cannot be mapped to the above.

No heap allocation is required. The driver should avoid panics and should not
silently substitute a constant, UID-derived value or previously generated
random word when TRNG output is unavailable.

---

# Startup and Ownership

Initialize the TRNG once after `embassy_stm32::init` has configured the system
clock and before any consumer requests randomness. The driver should have one
peripheral owner. The boot ID cache is shared read-only after its first
successful initialization.

Applications and services may use `latest_trng()` for independent fresh
values, and `boot_id()` for the stable current-session identifier. Calls to
`latest_trng()` do not alter the already memoized boot ID.

---

# Testing

Host-side tests should cover the boot-ID memoization logic through an injectable
word source: the first successful request reads once, later requests return the
same value, and a failed first attempt leaves the cache available for retry.
Hardware validation should confirm successful fresh reads, bounded timeout
behavior and propagation of TRNG health/error flags on STM32U5.

---

# Consequences

## Advantages

* Gives consumers a single fresh-random-word interface.
* Provides one consistent ID for all work in a boot session.
* Requires no flash writes and naturally changes after reset.
* Keeps peripheral details and error handling inside the driver.

## Disadvantages

* Boot-ID availability depends on successful TRNG initialization and health
  checks.
* A 32-bit identifier can collide across different boot sessions.
* Consumers must handle the possibility that initial generation fails.

These trade-offs are acceptable for a compact session correlation identifier.
