# Integration Test: Power Monitoring and Logging

## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing

## Objective

Validate the complete integration of the Voltage Monitor Driver, Charger Driver, Power Management Service, Logging Service, Storage Service and optionally Time Service.

The test should verify that the complete data path from hardware measurement through to persistent logging operates correctly.

---

## Test Setup

Start the following services:

* Time Service
* Storage Service
* Logging Service
* Voltage Monitor Driver
* BQ25186 Charger Driver
* Power Management Service

The Storage Service should create the standard system log stream.

The Logging Service should write to this stream.

---

## Test Operation

Every **10 seconds (0.1 Hz)**:

1. Read the latest `PowerState` published by the Power Management Service.
2. Generate a human-readable log message from the power service, and a message from the time service that can be used to relate system time back to human time (not with high precision - second level accuracy will be sufficient)
3. Append the messages to the system log using the Logging Service.
4. Flash the Green LED as a hearbeat signal to show that it is correctly operating
5. In the event of any error, turn and leave on the RED led.
The test should run continuously. 


---

## Example Log Output

Example log messages:

```text
00000124 1234.567 INFO  Power: source=Usb batt=3722mV solar=39mV ext_dc=323mV charging=true percent=Some(44) health=Normal charger_state=FastCharge charger_fault=None

00000125 1234.867 INFO Time: UTC 174829820 GPS ON

00000126 1244.567 INFO  Power: source=Usb batt=3722mV solar=39mV ext_dc=323mV charging=true percent=Some(44) health=Normal charger_state=FastCharge charger_fault=None

00000127 1244.967 INFO Time: UTC 174829830 GPS OFF

00000128 1254.567 INFO  Power: source=Usb batt=3722mV solar=39mV ext_dc=323mV charging=true percent=Some(44) health=Normal charger_state=FastCharge charger_fault=None
```

The precise formatting may evolve, but the log should remain human-readable.

---

## Verification

The test should verify that:

* Voltage measurements are updated correctly.
* Charger state is reflected in the published `PowerState`.
* Time state is reflected in the log message.
* Logging messages are correctly formatted.
* Log sequence numbers remain contiguous.
* Log timestamps increase monotonically.
* Messages are successfully written to the system log.
* No memory allocation occurs during normal operation.
* The system operates continuously without queue overflows or dropped log messages under the nominal 0.1 Hz logging rate.

---

## Future Extensions

This test provides the basis for additional integration scenarios, including:

* Applying external DC power during runtime.
* Applying and removing USB power.
* Simulating battery discharge.
* Exercising charger state transitions.
* Verifying low-battery warnings.
* Confirming correct behaviour during GPS acquisition and holdover.
* Long-duration soak testing to validate stability and continuous logging over extended periods.


# Implementation

Implement under 'integrationtests/integration001_powermonitorlog/'