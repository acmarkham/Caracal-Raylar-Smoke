# Integration Test: Audio recording, Time, and Logging

## Constraints
Read
ADR\common\AGENTS.md
before planning and implementing

## Objective

Validate the complete integration of the Power Management Service, Logging
Service, Storage Service, Time Service, Location Service, Audio Service, Sensor
Service and Identity and Versioning Service.

---

## Test Setup

Start the following services:

* Time Service
* Storage Service
* Logging Service
* Power Management Service
* Location Service
* Audio Service
* Sensor Service
* Identity and Versioning Service

The Storage Service should create the standard system log stream.

The Logging Service should write to this stream.

The Audio Service should write audio files to the storage stream in .wav format.

The services should interface to the drivers, and not talk directly to hardware/bypass a driver.

The Identity and Versioning Service should aggregate the lower-level
Traceability Driver with firmware, board, storage-card, GPS-module and
radio-module identity/version information for the system log. Unsupported or
not-yet-populated sources must be represented explicitly as `Unknown` or
`Unavailable`, rather than queried directly by this integration test.

---

## Test Operation

Startup:
1. Configure the STM32 core regulator through the STM32 Core Driver. Select
   SMPS by default for the SMPS-capable STM32U595 Q package and fitted Raylar
   v1.0 inductor; retain an explicit LDO diagnostic build option.
2. Enable lipo charger for 200mA limit
3. Enable all services needed e.g. GPS, SD card, audio
4. Beep 1000Hz for 0.25s on/0.25s off three times
5. Start the Identity and Versioning Service, open the standard system log and
   immediately record its complete startup snapshot before audio startup:
   * raw STM32 96-bit UUID/UID
   * derived 64-bit, 48-bit, 32-bit and 16-bit serial IDs
   * STM32 device code and board revision
   * firmware semantic version, Git hash, build timestamp/profile, runtime
     CRC32 and build CRC32, with explicit unknown/unavailable states
   * SD-card manufacturer ID, OEM ID, product name/revision, serial number,
     manufacture month/year and capacity, obtained through the Storage Driver
     and Storage Service rather than direct SDMMC access
   * GPS module vendor/model and firmware/protocol/hardware versions
   * radio module vendor/model and firmware/protocol/hardware versions
   The integration firmware must consume these fields through the Versioning
   Service and must not call the lower-level Traceability Driver directly.
6. Flush the startup log records so `/syslog.txt` contains traceability
   information even if GPS acquisition or microphone capture later stalls.
7. Initialize the LIS2HH12 accelerometer and LIS2MDL magnetometer through their
   drivers, register their raw measurements with the Sensor Service, and share
   the sensor I2C bus with the battery charger without bypassing any driver.
   Register acceleration and magnetic field with 10-second polling intervals,
   and each sensor's die temperature with a 30-second polling interval. Do not
   register composite sensors, thresholds, or delta-change events in this
   integration test.

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
   The PPS capture is the STM32U59xxx **32-bit TIM4_CH4** counter at 1 MHz.
   Preserve all 32 capture bits and use a 2^32-tick modulus; do not apply the
   16-bit TIM4 assumption used by many other STM32 families. Authoritative
   references are DS13633 Rev 3 section 3.44 Table 19 (p. 80/385), section
   3.44.2 (p. 81/385), and the RM0456 general-purpose TIM2-TIM5 chapter plus
   its `TIMx_ARR` and `TIMx_CCR4` register definitions.
   Explicitly program `TIM4_ARR = 0xFFFF_FFFF` during capture initialization;
   the RM0456 reset value is `0x0000_FFFF`, and configuring only the prescaler
   would leave even this 32-bit timer wrapping every 65.536 ms.
   With embassy-stm32 0.6, use the asynchronous input-capture future only to
   await the edge because that future returns CCR4 through a 16-bit register
   view. Immediately re-read the latched CCR4 using the driver's 32-bit
   `get_capture_value()` path before calculating `capture_ticks` or deltas.
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

Every **10 seconds (0.1 Hz)**: Raw motion sensors

1. Let the Sensor Service poll the LIS2HH12 acceleration source and retain the
   latest X/Y/Z reading in milli-g.
2. Let the Sensor Service poll the LIS2MDL magnetic-field source and retain the
   latest X/Y/Z reading in nanotesla.
3. Append a human-readable `Sensor` record for each new raw acceleration and
   magnetic-field reading to `/syslog.txt` through the Logging Service.
4. If a poll fails, retain the last valid value and log the source status and
   error counters rather than substituting zero.

Every **30 seconds**: Sensor die temperatures

1. Let the Sensor Service poll the LIS2HH12 and LIS2MDL die-temperature
   sources independently.
2. Append one human-readable `Sensor` record per source to `/syslog.txt`, in
   milli-degrees Celsius and with the source origin identified.
3. Do not calculate or log tilt, heading, e-compass, VeDBA, ODBA, absolute
   thresholds, or delta thresholds in this integration test.

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
00000122 1234.100 INFO  System: versioning device uuid=00112233-44556677-8899AABB serial64=0123456789ABCDEF serial48=456789ABCDEF serial32=9F34A102 serial16=A102 stm32=Known(...)

00000123 1234.110 INFO  System: versioning firmware version=Known("0.1.0") git_hash=Known("...") build_timestamp=Known("...") profile=Known("release") runtime_crc32=Known(...) build_crc32=Unknown

00000124 1234.120 INFO  System: versioning sd_card=Known(SdCardIdentity { manufacturer_id: Known(3), oem_id: Known([83, 68]), product_name: Known([...]), product_revision: Known(33), serial_number: Known(...), manufacture_year: Known(2026), manufacture_month: Known(9), capacity_bytes: Known(128043712512) })

00000125 1234.130 INFO  System: versioning gps_module=Unknown

00000126 1234.140 INFO  System: versioning radio_module=Unknown

00000127 1234.200 INFO  System: integration002 monoaudiolog started; format=16000Hz mono, 60-second WAV files in hourly folders

00000124 1234.567 INFO  Power: source=Usb batt=3722mV solar=39mV ext_dc=323mV charging=true percent=Some(44) health=Normal charger_state=FastCharge charger_fault=None

00000125 1234.867 INFO Time: UTC 174829820 GPS ON

00000126 1244.567 INFO  Power: source=Usb batt=3722mV solar=39mV ext_dc=323mV charging=true percent=Some(44) health=Normal charger_state=FastCharge charger_fault=None

00000127 1244.967 INFO Time: UTC 174829830 GPS OFF

00000128 1254.567 INFO  Power: source=Usb batt=3722mV solar=39mV ext_dc=323mV charging=true percent=Some(44) health=Normal charger_state=FastCharge charger_fault=None

00000129 1255.100 INFO  Location: event=acquired valid=true source=Gps lat_e7=520000010 lon_e7=-10000010 fix_age_us=125000 fixes_used=3 fixes_seen=3 sats=Some(8) hdop_centi=Some(120) uncertainty_m=Some(6) fix_utc=Some(...)

00000130 1264.600 INFO  Sensor: raw acceleration x_mg=2 y_mg=-4 z_mg=998 sample_ticks=1264600000

00000131 1264.610 INFO  Sensor: raw magnetic_field x_nt=24750 y_nt=-1200 z_nt=43100 sample_ticks=1264610000

00000132 1284.600 INFO  Sensor: die_temperature origin=Lis2hh12 milli_celsius=26125 sample_ticks=1284600000

00000133 1284.610 INFO  Sensor: die_temperature origin=Lis2mdl milli_celsius=26750 sample_ticks=1284610000
```

The precise formatting may evolve, but the log should remain human-readable.

---

## Verification

The test should verify that:

* The default build reports that the STM32 core supply transitioned to SMPS
  before board peripherals and services start.
* The no-default-features diagnostic build retains LDO through the same STM32
  Core Driver API.
* Voltage measurements are updated correctly.
* Charger state is reflected in the published `PowerState`.
* Time state is reflected in the log message.
* The Location Service publishes a filtered GPS-derived estimate after its
  acceptance threshold, logs it immediately, and logs the latest estimate at
  one-minute intervals thereafter.
* The Sensor Service polls and publishes LIS2HH12 acceleration and LIS2MDL
  magnetic field independently every 10 seconds.
* The Sensor Service polls and publishes the two die-temperature sources
  independently every 30 seconds, retaining their distinct sensor IDs and
  origins.
* Every new raw sensor reading is written through the Logging Service to
  `/syslog.txt`; failures retain the previous valid value and expose stale or
  fault state without generating a zero reading.
* No composite sensor or sensor threshold is registered by Integration Test
  002.
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
* Startup system log records come from the Identity and Versioning Service and
  include device/firmware traceability plus explicit SD-card, GPS-module and
  radio-module identity states. The integration test does not call the
  lower-level Traceability Driver directly.
* With a mounted SD card, the Versioning Service's SD identity is `Known` and
  matches CID/CSD metadata exposed through the Storage Driver, including the
  card serial number and capacity. Integration code does not access SDMMC
  identity registers directly.
* No memory allocation occurs during normal operation.
* Audio is correctly recorded
* Audio wav files correctly start and terminate at the correct time intervals
* User interface (LEDS) correctly display state
* Missing/unusable SD media and filesystem failures latch the severe-error
  state, stop recording, play the error signal, and flash both red LEDs



# Implementation

Implement under 'integrationtests/integration002_monoaudiolog/'
