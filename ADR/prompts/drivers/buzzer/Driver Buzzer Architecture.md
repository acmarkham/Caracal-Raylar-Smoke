# ADR-0008: Low-Level Buzzer Driver Architecture

## Status

Proposed

---

# Context

The Raylar board includes a buzzer driven from an STM32 timer output using PWM.

A working smoke test (unitsmoke/03_buzzer) has already demonstrated that the buzzer can generate sound by configuring a timer as complementary PWM and driving the buzzer pin with a 1 kHz square wave.

The prototype verifies:

* The board pin mapping is correct.
* The selected timer can drive the buzzer.
* The buzzer produces audible output.
* A 50% duty cycle square wave is sufficient for a basic tone.
* The buzzer can be enabled and disabled asynchronously.

The board-specific timer and pin mapping are defined in the board crate and should not be duplicated inside the driver.

The next step is to turn the prototype into a reusable low-level async buzzer driver.

---

# Decision

Implement a dedicated low-level Buzzer Driver.

The driver owns the timer and buzzer output pin and exposes a small async API for playing simple tones.

The initial API should be:

```rust
buzzer::init(...)
buzzer.play_tone(pitch, duration, volume).await
```

The driver should remain independent of higher-level application concepts such as alerts, melodies, alarms or user-interface states.

---

# Design Goals

The Buzzer Driver shall:

* Be simple.
* Be async.
* Be heapless.
* Own the PWM timer and buzzer pin.
* Allow callers to play a tone at a requested pitch, duration and volume.
* Disable the PWM output when no tone is playing.
* Hide all timer/PWM implementation details from callers.
* Reuse the known-good configuration pattern from the existing smoke test.

---

# Non-Goals

The driver is **not** responsible for:

* Alert policy
* Melody sequencing
* User-interface state
* Error beeps
* Startup sounds
* Notification priorities
* Audio synthesis
* Polyphony
* Complex waveform generation

These may belong to a future higher-level Sound or Alert Service.

---

# Existing Prototype

The existing smoke test configures the buzzer using:

```rust
ComplementaryPwm::new(...)
```

with:

* complementary PWM output
* push-pull output type
* edge-aligned up-counting mode
* 1 kHz PWM frequency
* 50% duty cycle
* explicit enable/disable around the audible period

The implementation should use this prototype as the reference for the low-level hardware setup.

---

# Architecture

```text
Application / Service
        │
        ▼
  Buzzer Driver
        │
        ▼
STM32 Timer PWM
        │
        ▼
 Buzzer Output Pin
```

The Buzzer Driver owns:

* the timer peripheral
* the buzzer pin
* PWM configuration
* tone duration timing

Consumers never interact directly with the timer or PWM channel.

---

# Public API

Initial conceptual API:

```rust
pub fn init(buzzer: BoardBuzzer) -> BuzzerDriver

impl BuzzerDriver {
    pub async fn play_tone(
        &mut self,
        pitch: Pitch,
        duration: Duration,
        volume: Volume,
    ) -> Result<(), BuzzerError>;
}
```

The exact Rust types may evolve, but the API should preserve the same basic model:

```text
initialise buzzer

↓

play tone for duration

↓

turn buzzer off automatically
```

---

# Pitch

Pitch represents the requested tone frequency.

Initially, this may be a simple frequency in hertz:

```rust
pub struct PitchHz(pub u32);
```

or:

```rust
pub enum Pitch {
    Hz(u32),
}
```

The driver converts pitch into the corresponding PWM frequency.

The driver should reject unsupported frequencies rather than silently misconfiguring the timer.

Suggested initial validation:

* minimum audible/useful frequency: implementation-defined
* maximum buzzer/timer-supported frequency: implementation-defined

The first implementation may only require simple tones such as 1 kHz.

---

# Duration

Duration is the length of time the tone should play.

The driver should use Embassy async timing:

```rust
embassy_time::Timer::after(duration).await
```

After the duration expires, the driver must disable the PWM output.

The output should also be disabled if an error occurs during setup.

---

# Volume

Volume controls PWM duty cycle.

For a simple buzzer, volume may initially be represented as a small bounded value:

```rust
pub struct Volume(pub u8);
```

where:

```text
0   = muted
255 = maximum configured volume
```

or as a percentage:

```text
0%  = muted
100% = maximum configured volume
```

The driver maps volume to PWM duty cycle.

For the initial implementation:

* 0 volume disables the channel.
* Maximum volume may use approximately 50% duty cycle.
* Intermediate values may scale duty cycle below 50%.

The driver should avoid duty cycles above the intended safe maximum for the buzzer circuit.

---

# PWM Behaviour

The driver should:

* configure PWM during initialisation
* set PWM frequency based on pitch
* set duty cycle based on volume
* enable the PWM channel when a tone starts
* disable the PWM channel when the tone ends

The output should be off by default after initialisation.

---

# Async Behaviour

`play_tone()` is an async operation.

A call to `play_tone()` should not return until the tone has finished or an error occurs.

Initial behaviour may be single-owner and non-reentrant:

```text
one caller owns &mut BuzzerDriver

↓

only one tone can play at a time
```

This avoids the need for internal arbitration.

A future higher-level service may provide queuing or priority handling.

---

# Ownership Model

The driver should take ownership of the board-level buzzer resource provided by the board crate.

Conceptually:

```rust
let Board { buzzer, .. } = Board::new(p);

let mut buzzer = buzzer::init(buzzer);
```

The driver should not define or duplicate pin mappings.

The board crate remains the authority for hardware mapping.

---

# Error Handling

The driver should return structured errors.

Possible errors include:

* unsupported pitch
* unsupported PWM configuration
* invalid volume
* timer configuration failure

The driver should avoid panics in normal operation.

---

# Relationship to Logging

The Buzzer Driver should not depend on the Logging Service.

Low-level drivers should remain independently testable.

Higher-level services may log buzzer-related events if required.

---

# Relationship to Power Management

The Buzzer Driver should not decide whether sound is allowed based on battery level or power state.

If power-aware behaviour is required, it should be implemented in a higher-level Sound or Alert Service that consumes the Power Management Service.

---

# Future Extensions

Possible future additions include:

* melody playback
* predefined beep patterns
* asynchronous cancellation
* non-blocking start/stop API
* alert priorities
* volume calibration
* perceptual loudness mapping
* compile-time pitch tables
* integration with a system Alert Service

These features should not complicate the initial low-level driver.

---

# Consequences

## Advantages

* Simple reusable abstraction over the known-good PWM prototype.
* Clean separation between buzzer hardware control and application alert policy.
* Async API fits the rest of the Embassy-based firmware.
* No heap allocation.
* Board pin mapping remains centralised in the board crate.
* Easy to test with simple smoke tests.

## Disadvantages

* Initial implementation only supports simple single-tone playback.
* No built-in queuing or priority model.
* Caller is blocked until the tone completes unless the caller spawns a separate task.
* More advanced alert behaviour requires a higher-level service.

These trade-offs are acceptable because the initial requirement is a low-level buzzer driver, not a complete sound or alert framework.

# Test

Build a small smoke test in:

```text
drivertests/buzzertest
```

The test should power up the board and use a minimal proof-of-concept to demonstrate that the low-level Buzzer Driver works.

The test should:

* initialise the board
* construct the Buzzer Driver using the board-provided buzzer resource
* play a simple tone, for example 1 kHz for 1 second
* pause for 1 second
* repeat continuously

Suggested behaviour:

```text
1 kHz tone ON for 1 second

↓

buzzer OFF for 1 second

↓

repeat
```

The test is intended as a basic functional validation rather than a comprehensive audio test.

It should demonstrate that:

* the driver initialises correctly
* PWM output is generated
* `play_tone()` waits for the requested duration
* the buzzer output is disabled after the tone completes
* the driver can be reused for repeated tones
