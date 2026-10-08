# ADR-0010: User Button Driver Architecture

## Status

Proposed

---

# Context

The Raylar board currently exposes a single user button.

A working smoke test already demonstrates that the button can be read using Embassy `ExtiInput` and that button presses can be detected asynchronously using edge-triggered waits.

The existing smoke test:

* waits for a falling edge
* applies a short debounce delay
* confirms that the button is still asserted
* toggles an LED
* waits for a rising edge
* applies a second debounce delay

This validates the basic electrical and async behaviour of the button.

Although the current board only has one user button, the driver should be designed so that additional buttons can be added later without changing the public model.

---

# Decision

Implement a low-level User Button Driver.

The driver owns board-provided button inputs and exposes a simple async interface for detecting button events by logical button name.

The initial implementation should support the existing single user button, but the architecture should allow multiple buttons in the future.

---

# Design Goals

The User Button Driver shall:

* Be async.
* Be heapless.
* Own the board-provided button inputs.
* Support debounced button press detection.
* Refer to buttons by logical name.
* Hide GPIO, EXTI and electrical polarity details from callers.
* Support future boards with more than one button.
* Avoid dynamic allocation.

---

# Non-Goals

The driver is **not** responsible for:

* Application button policy
* Menu navigation
* Long-press actions
* Multi-click interpretation
* User-interface state machines
* LED control
* Power-mode decisions

Those behaviours belong to higher-level services.

---

# Existing Prototype

The current smoke test in:

```text
unitsmoke/02_userbutton
```

demonstrates:

```rust
button.wait_for_falling_edge().await;
Timer::after(Duration::from_millis(30)).await;

if button.is_low() {
    led.toggle();

    button.wait_for_rising_edge().await;
    Timer::after(Duration::from_millis(30)).await;
}
```

This sequence should be treated as the reference behaviour for the first driver implementation.

---

# Button Names

The driver should define logical button names.

Initial enum:

```rust
pub enum ButtonName {
    User,
}
```

Future boards may extend this to:

```rust
pub enum ButtonName {
    User,
    Mode,
    ResetRequest,
    Function1,
    Function2,
}
```

The public API should avoid exposing GPIO ports, pins or EXTI lines.

---

# Public API

Conceptual API:

```rust
pub fn init(buttons: BoardButtons) -> ButtonDriver

impl ButtonDriver {
    pub async fn wait_for_press(&mut self, button: ButtonName);
    pub async fn wait_for_release(&mut self, button: ButtonName);
    pub fn is_pressed(&self, button: ButtonName) -> bool;
}
```

The exact Rust types may evolve, but the public model should remain:

```text
logical button name

↓

debounced async event

↓

pressed / released state
```

---

# Debouncing

The driver should implement basic debounce handling internally.

Initial debounce duration:

```text
30 ms
```

Press detection should follow:

```text
falling edge

↓

debounce delay

↓

confirm input still active

↓

report press
```

Release detection should follow:

```text
rising edge

↓

debounce delay

↓

confirm input inactive

↓

report release
```

The debounce duration should be configurable at construction time or through a driver configuration structure.

---

# Electrical Polarity

The current user button is active-low.

The driver should hide this from callers.

Logical behaviour:

```text
pressed  = true
released = false
```

regardless of whether the physical input is active-low or active-high.

Electrical polarity should be represented internally, for example:

```rust
pub enum ButtonPolarity {
    ActiveLow,
    ActiveHigh,
}
```

The board crate or driver configuration should define the polarity.

Application code should never need to know whether the button is active-low.

---

# Ownership Model

The driver should take ownership of the board-level button resource provided by the board crate.

Conceptually:

```rust
let Board { buttons, .. } = Board::new(p);

let mut buttons = buttons::init(buttons);
```

The driver should not construct GPIOs or EXTI inputs directly.

The board crate remains responsible for:

* pin assignment
* EXTI input construction
* pull configuration
* initial electrical configuration

---

# Event Model

The first implementation may provide direct async wait methods.

Example:

```rust
buttons.wait_for_press(ButtonName::User).await;
```

This is sufficient for simple tests and low-level consumers.

Future versions may provide a stream or channel of button events:

```rust
pub enum ButtonEventKind {
    Pressed,
    Released,
}

pub struct ButtonEvent {
    pub button: ButtonName,
    pub event: ButtonEventKind,
    pub timestamp: Instant,
}
```

However, the initial low-level driver should remain minimal.

---

# Timing and Timestamps

The low-level button driver does not need to publish UTC timestamps.

If timestamps are required later, they should use monotonic system time.

UTC conversion belongs to the Time Service.

---

# Error Handling

Basic button operations are expected to be infallible once the driver owns valid EXTI inputs.

The initial API may therefore use infallible methods.

If future hardware or configuration introduces fallible behaviour, structured errors may be added.

---

# Relationship to LED Driver

The button driver should not depend on the LED driver.

The existing smoke test toggles an LED to demonstrate functionality, but that is test behaviour, not driver behaviour.

---

# Relationship to Higher-Level Services

The User Button Driver is a low-level hardware abstraction.

Higher-level services may consume button events to implement:

* wake/sleep requests
* test modes
* user commands
* calibration actions
* diagnostic modes
* factory reset workflows

Those policies should not be embedded in the low-level button driver.

---

# Test

Build a small smoke test in:

```text
drivertests/buttontest
```

The test should power up the board and use the User Button Driver to demonstrate basic functionality.

The test should:

* initialise the board
* construct the User Button Driver using the board-provided button resource
* blink `sys_main_green` periodically to show that the test is running
* wait for the user button to be pressed
* toggle `sys_main_red` when a debounced press is detected
* wait for release before detecting another press
* repeat continuously

Suggested behaviour:

```text
heartbeat LED blinks

↓

user button pressed

↓

press is debounced

↓

red LED toggles

↓

button release is debounced

↓

repeat
```

The test should demonstrate that:

* the driver initialises correctly
* the logical `User` button maps to the physical board button
* debounced press detection works
* debounced release detection works
* the driver hides active-low electrical behaviour
* repeated presses are detected reliably

---

# Future Extensions

Possible future additions include:

* multiple buttons
* long-press detection
* double-click detection
* hold-repeat behaviour
* event channels
* timestamped button events
* configurable debounce policies per button
* integration with a user-interface service
* integration with a power/wake service

These features should not complicate the initial low-level driver.

---

# Consequences

## Advantages

* Simple async API.
* Button electrical details remain hidden.
* Debouncing is implemented once.
* Future multi-button support is straightforward.
* Driver is independent of LED and UI policy.
* No heap allocation.
* Easy to test using the existing smoke test pattern.

## Disadvantages

* Initial implementation only supports simple press/release events.
* Higher-level behaviours such as long press and double click require additional services or future extensions.
* The direct async wait API may need to evolve if several tasks need to observe the same button events.

These trade-offs are acceptable because the initial requirement is a low-level user button driver, not a full user-interface framework.
