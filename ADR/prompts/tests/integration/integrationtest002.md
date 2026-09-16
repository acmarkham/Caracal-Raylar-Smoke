# Integration Test: Audio recording, Time, and Logging

## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing

## Objective

Validate the complete integration of the Power Management Service, Logging
Service, Storage Service, Time Service, Location Service, Audio Service and
Identity Driver.

---

## Test Setup

Start the following services:

* Time Service
* Storage Service
* Logging Service
* Power Management Service
* Location Service
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
   Once PPS is absent for 1.5 seconds, remove the temporary phase slew without
   stepping UTC and use only the calibrated oscillator rate during holdover.
   On PPS return, reject the gap edge and require three consecutive PPS
   intervals within 50 ms of one second before accepting another time anchor.
   Gap and qualification samples must not enter oscillator regression. Freeze
   oscillator calibration after the initial eleven-point, ten-minute window so
   later reacquisition artefacts cannot move the learned frequency.
4. Start the audio service in mono, 16kHz using high quality e.g. SINC5 buffer, 32 bit int wav file (even though the effective resolution is probably 18 bit)
5. Save data to minute long wav files, in hourly folders. 
6. Start on top-of-the minute boundary e.g. 00s
7. Toggle the SysSdBlue LED after each packet is written to storage
8. Aggregate audio packet timestamps into one logfile record per second. Each
   record includes the first and last packet timestamps plus packet and sample
   counts, preserving timing diagnostics without generating 10 Hz SD traffic.

After GPS location is acquired:

1. Feed the GPS Driver's fix stream into the Location Service.
2. Use the Location Service's filtered, application-facing estimate rather
   than logging raw coordinates as the device location. The default filter
   requires at least three accepted fixes.
3. Immediately append the first valid location estimate to `/syslog.txt`.
4. Subsequently append the latest retained location estimate every 60 seconds.
   Each record must include latitude and longitude in signed degrees times
   10^7, fix age, fixes used/seen, satellite and HDOP metadata, estimated
   uncertainty, source, and the UTC label of the contributing fix.


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

Continuously while GPS is active:

1. Log every PPS edge with its monotonic timestamp, hardware capture timestamp,
   capture interval, capture frequency, timing backend, and PPS sequence number.
2. Log every emitted NMEA/PPS correlation, including unmatched NMEA time
   records, with UTC label, NMEA arrival timestamp, matched PPS timestamp,
   arrival offset, and hardware capture values. These records must be suitable
   for reconstructing or correcting the UTC mapping after the deployment.
3. Report any bounded diagnostic-stream lag explicitly in the log rather than
   silently omitting records.


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

00000129 1255.100 INFO  Location: event=acquired valid=true source=Gps lat_e7=520000010 lon_e7=-10000010 fix_age_us=125000 fixes_used=3 fixes_seen=3 sats=Some(8) hdop_centi=Some(120) uncertainty_m=Some(6) fix_utc=Some(...)
```

The precise formatting may evolve, but the log should remain human-readable.

---

## Verification

The test should verify that:

* Voltage measurements are updated correctly.
* Charger state is reflected in the published `PowerState`.
* Time state is reflected in the log message.
* The Location Service publishes a filtered GPS-derived estimate after its
  acceptance threshold, logs it immediately, and logs the latest estimate at
  one-minute intervals thereafter.
* Holdover uses calibrated frequency only; phase slew is zero once PPS loss is
  declared.
* Reacquisition anchors are withheld until three clean one-second PPS intervals
  have been observed, and the withheld samples do not alter calibration.
* Frequency calibration locks after the initial ten-minute window.
* Per-edge PPS and per-correlation records are present without unexplained
  sequence gaps and contain enough raw timestamps for post-hoc correction.
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
