# ADR-0009: LED Driver Architecture

## Status

Proposed

---

# Context

The Raylar board exposes five system LEDs connected to STM32 GPIO outputs.

The LEDs are currently defined in the board crate with board-specific pin mappings:

```rust
sys_gps_green: Output::new(PB4, Level::Low, Speed::Medium),

sys_gps_red: Output::new(PD7, Level::Low, Speed::Medium),

sys_main_red: Output::new(PB15, Level::Low, Speed::Medium),

sys_main_green: Output::new(PD10, Level::Low, Speed::Medium),

sys_sd_blue: Output::new(PD5, Level::Low, Speed::Medium),
```

The board crate remains the authority for pin mapping. The LED driver should not duplicate GPIO pin definitions.

A simple low-level LED driver is required so that application code and integration tests can refer to LEDs by logical name rather than directly manipulating GPIO outputs.

---

# Decision

Implement a dedicated low-level LED Driver.

The driver owns the board-provided LED GPIO outputs and exposes simple methods to turn LEDs on, turn LEDs off, and toggle LEDs by name.

The initial API should support:

```rust
leds.on(LedName::SysGpsGreen);

leds.off(LedName::SysGpsGreen);

leds.toggle(LedName::SysGpsGreen);
```

The driver should remain deliberately small and should not implement higher-level blinking or status policy.

---

# Design Goals

The LED Driver shall:

* Be simple.
* Be heapless.
* Own the GPIO outputs for all board LEDs.
* Allow LEDs to be controlled by logical name.
* Hide GPIO pin details from application code.
* Avoid dynamic allocation.
* Be suitable for use in smoke tests, diagnostics and higher-level services.
* Keep board pin mapping centralised in the board crate.

---

# Non-Goals

The LED Driver is **not** responsible for:

* Blink patterns
* Heartbeat policy
* GPS status policy
* SD card status policy
* Error indication policy
* Power-state indication
* User-interface state machines

These may belong to higher-level services or test tasks.

---

# LED Names

The driver should define logical LED names corresponding to the board LEDs.

Suggested enum:

```rust
pub enum LedName {
    SysGpsGreen,
    SysGpsRed,
    SysMainRed,
    SysMainGreen,
    SysSdBlue,
}
```

These names are stable logical identifiers.

They should not expose STM32 port or pin numbers.

---

# Public API

Conceptual API:

```rust
pub fn init(leds: BoardLeds) -> LedDriver

impl LedDriver {
    pub fn on(&mut self, led: LedName);
    pub fn off(&mut self, led: LedName);
    pub fn toggle(&mut self, led: LedName);
}
```

The exact Rust types may evolve, but the public API should preserve the same model:

```text
logical LED name

↓

simple on/off/toggle operation

↓

GPIO output state change
```

---

# Ownership Model

The driver should take ownership of the board-level LED resource provided by the board crate.

Conceptually:

```rust
let Board { leds, .. } = Board::new(p);

let mut leds = led::init(leds);
```

The LED driver should not construct GPIO pins directly.

The board crate remains responsible for:

* pin assignment
* GPIO output construction
* initial electrical state
* output speed

---

# Electrical Behaviour

The LEDs are initialised low in the board crate.

The driver should define the logical meaning of:

```text
on
off
toggle
```

based on the actual board electrical behaviour.

If the LEDs are active-high:

```text
on  = set_high()
off = set_low()
```

If the LEDs are active-low, the driver should invert this internally.

Application code should never need to know whether an LED is active-high or active-low.

---

# Initial State

After initialisation, the driver should preserve the board crate's initial LED state unless an explicit operation is requested.

Current board initialisation sets all LEDs to low.

The driver should not flash or change LEDs during construction.

---

# Error Handling

Basic LED operations are expected to be infallible because they operate on already-owned GPIO outputs.

Therefore, the initial API may use infallible methods.

If future implementations introduce fallible behaviour, errors can be added without changing the conceptual model.

---

# Relationship to Higher-Level Services

The LED Driver should remain a low-level hardware abstraction.

Higher-level services may use it to implement policies such as:

* heartbeat blinking
* GPS fix status
* SD card activity
* low battery indication
* fault indication
* manufacturing test patterns

Those policies should not be embedded in the low-level driver.

---

# Test

Build a small smoke test in:

```text
drivertests/ledtest
```

The test should power up the board and use the LED Driver to demonstrate basic functionality.

The test should:

* initialise the board
* construct the LED Driver using the board-provided LED resource
* turn each LED on by name
* wait briefly
* turn each LED off by name
* wait briefly
* toggle each LED by name
* repeat continuously

Suggested behaviour:

```text
SysGpsGreen ON

↓

SysGpsGreen OFF

↓

SysGpsRed ON

↓

SysGpsRed OFF

↓

...

↓

Toggle all LEDs

↓

repeat
```

The test is intended as a basic functional validation rather than a user-interface test.

It should demonstrate that:

* the driver initialises correctly
* each logical LED name maps to the correct physical LED
* `on()` enables the LED
* `off()` disables the LED
* `toggle()` changes the current LED state
* the driver can be reused repeatedly

---

# Future Extensions

Possible future additions include:

* blink helper functions
* non-blocking blink state machines
* predefined status patterns
* LED groups
* test sequences
* integration with a system status service
* compile-time active-high / active-low configuration

These features should not complicate the initial low-level LED driver.

---

# Consequences

## Advantages

* Simple and explicit API.
* Application code does not manipulate GPIO pins directly.
* Board pin mapping remains centralised.
* Logical LED names improve readability.
* The driver is easy to test.
* No heap allocation or async machinery is required.

## Disadvantages

* The driver only provides primitive LED control.
* Blink patterns and status policies require higher-level code.
* Adding or removing LEDs requires updating the logical LED name enum.

These trade-offs are acceptable because this ADR covers a low-level LED driver, not a complete status-indication framework.
