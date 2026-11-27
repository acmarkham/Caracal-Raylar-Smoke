# L4 four-channel runtime fault investigation — 2026-11-27

This is the historical DMA-margin investigation. The subsequent user-requested
[silence recovery change](VALIDATION-GAPS-20261127.md) supersedes its strict
fatal-on-overrun behavior for four-channel L4 recording.

GPS is absent. All hardware tests here use the explicit `local-time` laboratory
clock; file timestamps remain synthetic and metadata marks UTC unknown.

The original running image was attached without flashing/resetting. Its saved
RTT data reports a DMA overrun at 54.253 s, after audio began at 37.457 s:
`Ok((Ok(0), Ok(1), Err(Overrun), Err(Overrun)))`. All four bank counters had
advanced to 6656 while the consumer was on bank 6654. The audio fault (code 3)
latched, followed by the recorder's source timeout. This was actual unread-data
loss, not an SD-card-detect assertion or lack of GPS.

A separate read-only inspection image exported the last 16384 bytes of the
2292410-byte `/syslog.txt`. The latest boot's log confirms recording, GPS off,
no UART/storage/logging errors and a modest 1464-scalar backlog before it ends
partway through a log line at 50.829 s. The fault latch prohibits subsequent SD
writes, so the terminal error is available in RTT rather than in that file.
No existing recordings or log bytes were overwritten or deleted. The inspection
image parks after reading, before enabling writes or opening the append stream.

Changes are local to L4:

- SDMMC1 and DMA2_CH5 interrupts move from the default P0 to P2, below the P1
  microphone executor. Audio DMA completion interrupts remain at P0. Bulk SD
  interrupts previously could preempt capture; this priority inversion is removed.
- The continuous `local-time` profile uses the RAM otherwise occupied by GPS:
  256-frame DMA halves (16 ms) and 6144 source scalars (96 ms). The GPS-enabled
  and fixture-export bench profiles keep their smaller memory allocation.
- Sample-clock checks and periodic statistics derive their cadence from the
  selected bank size. Hardware overrun/transfer-error checks, continuity checks,
  and the red LED/buzzer fault latch stay enabled. There is no loss recovery that
  discards audio silently.
- The capture helper supports attaching without resetting, observing a chosen
  number of rotations, and a read-only syslog inspection completion marker.

The saved trace identifies loss of DMA service margin; it does not by itself
attribute every microsecond of the original delay to a particular ISR. Priority
ordering and buffer margin are both addressed, and acceptance depends on the
longer hardware run rather than on suppressing the original checks.

Read-only syslog extraction:

```powershell
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --locked --offline --features local-time,inspect-syslog
rtk proxy python apps/aardwolf_l4/tools/capture.py syslog --mode inspect --timeout-seconds 200
rtk proxy python apps/aardwolf_l4/tools/read_syslog_export.py apps/aardwolf_l4/evidence/syslog.log
```

Build without `inspect-syslog` to resume recording. The raw original trace,
read-only syslog export, decoded tail and new run are retained locally in
`evidence/20261127-fix/`. Earlier validation records remain historical results;
the two-minute checks on 2026-11-26 did not establish sustained stability.

## Ten-minute hardware soak

The fixed continuous `local-time` image completed ten consecutive minute-long
FLAC/CRCL file pairs on the connected Evo. Each pair reported exactly 960000
frames per lane. DMA errors, dropped audio and logging errors remained zero.
The full active CPU windows ranged from 58.0% to 61.0%, averaging 59.6%; the
startup window is excluded because it includes the pre-recording wait.

The run exercised an SD append taking 51119 us. Maximum thread poll was
53291 us, codec block 10500 us, and source backlog 5612 of 6144 scalars.
Stack high-water use was 28976 of 32768 bytes, with the guard intact. These
measurements show improved service margin, but do not establish indefinite
endurance or qualify the smaller GPS-enabled buffer profile on hardware.

Evidence: `evidence/20261127-fix/soak-01.{elf,log,json}` and
`soak-01.summary.json`. The tested ELF SHA-256 is
`e5e33619e707e4fb7ad3570b4ce42c47163e5dabfb84056f80192a4d31221731`.
The monitor exited cleanly after the tenth closure, leaving recording active.

To repeat the continuous fake-GPS soak:

```powershell
rtk cargo build -p aardwolf-l4 --release --target thumbv7em-none-eabihf --locked --offline --features local-time
rtk proxy python apps/aardwolf_l4/tools/capture.py soak --mode continuous --segments 10 --timeout-seconds 850
```

## Standalone continuation

After over four minutes with no live debugger session, the read-only inspector
exported the last 16384 bytes of the now 2459533-byte syslog. It shows recording
still advancing at firmware time 890.365 s, compared with the tenth closure at
636.155 s. Fourteen file pairs had closed and the fifteenth was active. Closures
13 and 14 in the exported tail each report 960000 frames per lane. DMA errors
remain zero throughout; the 840 s health record reports no dropped, truncated or
failed log entries and 60.3% CPU. GPS remains off, with no invented fix/location.
The maximum codec block had increased to 10932 us; the SD append maximum and
source backlog maximum were unchanged.

Evidence: `standalone-syslog.{elf,log,json}` and the decoded
`standalone-syslog.syslog.txt`, in the same evidence directory. Inspection
intentionally resets recording and leaves the current partial pair incomplete;
completed files are retained. The successful soak ELF was restored for
continued recording using fake GPS. `resume-01.{elf,log,json}` confirms the
same ELF hash, a fresh 960000-frame pair closed at 96.157 s, and the second
pair opened before the monitor disconnected cleanly. The board was left
recording. The standard build output was also rebuilt with `local-time` alone,
so it contains the continuous recorder rather than the inspection image.

Checks also passed for the L4 default, continuous local-time, fixture bench and
syslog inspection builds, 25 host library tests, the U5 release build, Python
syntax checks and targeted Rust formatting. This investigation changes no U5
or shared source files; shared changes already present belong to the preceding
four-channel implementation.
