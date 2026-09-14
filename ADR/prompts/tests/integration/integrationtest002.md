# Integration Test: Audio recording, Time, and Logging

## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing

## Objective

Validate the complete integration of the Power Management Service, Logging Service, Storage Service, Time Service, Audio Service

---

## Test Setup

Start the following services:

* Time Service
* Storage Service
* Logging Service
* Power Management Service
* Audio Service

The Storage Service should create the standard system log stream.

The Logging Service should write to this stream.

The Audio Service should write audio files to the storage stream in .wav format.

The services should interface to the drivers, and not talk directly to hardware/bypass a driver.

---

## Test Operation

Startup:
1. Enable lipo charger for 200mA limit
2. Enable all services needed e.g. GPS, SD card, audio
3. Beep 1000Hz for 0.25s on/0.25s off three times

After GPS fix is acquired: Audio
1. Issue a different "successful GPS" beep to indicate a good PPS fix has been obtained
2. Start the audio service in mono, 16kHz using high quality e.g. SINC5 buffer, 32 bit int wav file (even though the effective resolution is probably 18 bit)
3. Save data to minute long wav files, in hourly folders. 
4. Start on top-of-the minute boundary e.g. 00s
5. Toggle the SysSdBlue LED after each packet is written to storage
6. Log each timestamp of the audio packet/buffer to the logfile


Every **10 seconds (0.1 Hz)**: Power State

1. Read the latest `PowerState` published by the Power Management Service.
2. Generate a human-readable log message from the power service, and a message from the time service that can be used to relate system time back to human time (not with high precision - second level accuracy will be sufficient)
3. Append the messages to the system log using the Logging Service.
4. Briefly flash the Green sys_LED as a hearbeat signal to show that it is correctly operating


Every **10 seconds (0.1 Hz)**:

1. Read the latest time information published by the Time Service.
2. Generate a human-readable log message from the time service to show if it is under GPS PPS sync or not, and what the drift/tolerance is. Also handle the case where no fix has yet been acquired.
3. Append the messages to the system log using the Logging Service.
4. Briefly flash the SysGpsGreen LED as a hearbeat signal to show that it is correctly operating


Error:
In the event of any error, turn and leave on the RED sys_led.

Termination:
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
* Audio is correctly recorded
* Audio wav files correctly start and terminate at the correct time intervals
* User interface (LEDS) correctly display state



# Implementation

Implement under 'integrationtests/integration002_monoaudiolog/'