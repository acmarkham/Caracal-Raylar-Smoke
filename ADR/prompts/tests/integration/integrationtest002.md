# Integration Test: Audio recording, Time, and Logging

## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing

## Objective

Validate the complete integration of the Power Management Service, Logging Service, Storage Service, Time Service, Audio Service
and Identity Driver

---

## Test Setup

Start the following services:

* Time Service
* Storage Service
* Logging Service
* Power Management Service
* Audio Service
* Identity Driver

The Storage Service should create the standard system log stream.

The Logging Service should write to this stream.

The Audio Service should write audio files to the storage stream in .wav format.

The services should interface to the drivers, and not talk directly to hardware/bypass a driver.

The Identity Driver should provide device and firmware traceability information
for the system log.

---

## Test Operation

Startup:
1. Enable lipo charger for 200mA limit
2. Enable all services needed e.g. GPS, SD card, audio
3. Beep 1000Hz for 0.25s on/0.25s off three times
4. Open the standard system log and immediately record the device identity
   before audio startup:
   * raw STM32 96-bit UUID/UID
   * derived 64-bit, 48-bit, 32-bit and 16-bit serial IDs
   * runtime firmware CRC32 hash, or an explicit firmware-hash error if the
     image range cannot be measured
5. Flush the startup log records so `/syslog.txt` contains traceability
   information even if GPS acquisition or microphone capture later stalls.

After GPS fix is acquired: Audio
1. Use only a GPS PPS-correlated timestamp as a Time Service anchor; never use
   the NMEA serial-arrival timestamp as an anchor.
2. Issue a different "successful GPS" beep after the first GPS PPS anchor has
   been accepted.
3. Keep GPS continuously powered for 10 minutes after the first fix to
   calibrate oscillator frequency before entering the normal GPS power cycle.
   Use hardware timer input capture for PPS and an outlier-resistant regression
   spanning this calibration interval. Correct unambiguous adjacent-second NMEA
   labels, reject remaining large residuals, slew smaller phase errors without
   stepping UTC, and include residual phase error in published uncertainty.
4. Start the audio service in mono, 16kHz using high quality e.g. SINC5 buffer, 32 bit int wav file (even though the effective resolution is probably 18 bit)
5. Save data to minute long wav files, in hourly folders. 
6. Start on top-of-the minute boundary e.g. 00s
7. Toggle the SysSdBlue LED after each packet is written to storage
8. Log each timestamp of the audio packet/buffer to the logfile


Every **10 seconds (0.1 Hz)**: Power State

1. Read the latest `PowerState` published by the Power Management Service.
2. Generate a human-readable log message from the power service, and a message from the time service that can be used to relate system time back to human time (not with high precision - second level accuracy will be sufficient)
3. Append the messages to the system log using the Logging Service.
4. Briefly flash the Green sys_LED as a hearbeat signal to show that it is correctly operating


Every **10 seconds (0.1 Hz)**:

1. Read the latest time information published by the Time Service.
2. Generate a human-readable log message from the time service to show if it is under GPS PPS sync or not, and what the drift/tolerance is. Also handle the case where no fix has yet been acquired.
   Include the first anchor type, latest PPS residual, accepted and rejected
   anchor counts, and GPS calibration/reacquisition state and counters.
3. Append the messages to the system log using the Logging Service.
4. Briefly flash the SysGpsGreen LED as a hearbeat signal to show that it is correctly operating


Error:
In the event of a severe error that prevents the test from operating safely or
recording valid data (for example, no SD card, SD-card initialization failure,
filesystem mount/open/write failure, or an unrecoverable service failure):

1. Stop normal test operation and do not attempt to record further audio.
2. Play a distinctive error signal on the buzzer: three short descending tones.
   This must be clearly different from both the three 1000 Hz startup beeps and
   the successful-GPS trill.
3. Flash both red system LEDs (`SysMainRed` and `SysGpsRed`) together at 1 Hz
   with a 50% duty cycle, indefinitely. Normal green heartbeat indications must
   stop while this severe-error state is active.
4. Repeat the three-tone error signal every 10 seconds so that the fault remains
   discoverable when the LEDs are not visible.

Recoverable errors may continue to be logged and counted without entering this
latched severe-error indication. Once entered, the severe-error state remains
latched until the board is reset.

Termination:
The test should run continuously. 


---

## Example Log Output

Example log messages:

```text
00000122 1234.100 INFO  System: identity uuid=00112233-44556677-8899AABB serial64=0123456789ABCDEF serial48=456789ABCDEF serial32=9F34A102 serial16=A102 firmware_crc32=7C91D42E

00000123 1234.200 INFO  System: integration002 monoaudiolog started; format=16000Hz mono, 60-second WAV files in hourly folders

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
* Startup system log records include raw device UUID/UID, derived serial IDs,
  and runtime firmware CRC32 or an explicit firmware-hash error.
* No memory allocation occurs during normal operation.
* Audio is correctly recorded
* Audio wav files correctly start and terminate at the correct time intervals
* User interface (LEDS) correctly display state
* Missing/unusable SD media and filesystem failures latch the severe-error
  state, stop recording, play the error signal, and flash both red LEDs



# Implementation

Implement under 'integrationtests/integration002_monoaudiolog/'
