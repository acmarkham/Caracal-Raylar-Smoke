The FRACN and timer-prescaler correction are now working. Normal PPS intervals are centred on 1,000,000 ticks, and frequency calibration locked successfully.

### Calibration

- First UTC/PPS synchronization: approximately 24 seconds.
- Frequency calibration locked at 629.8 seconds.
- Calibration samples: 11.
- Final calibration: `cal_ppb=-116`.
- At lock:
  - residual: 7 µs
  - uncertainty: 171 µs
  - accepted/rejected anchors: `603/0`

During the uninterrupted calibration period, all 603 one-second PPS intervals were between 999,999 and 1,000,001 ticks:

```text
Mean:     1,000,000.0995 ticks
Median:   1,000,000 ticks
Range:    999,999–1,000,001
Bias:     approximately +100 ppb
```

This is an excellent result. The remaining correction is about 0.1 ppm, compared with the original approximately 9 ppm oscillator error.

Across the complete run, the clean intervals were distributed approximately as:

```text
1,000,000 ticks: 715
  999,999 ticks:  79
1,000,001 ticks:  69
```

The previous 1,006,994-tick problem is gone.

### Holdover and reacquisition

The run lasted about 10,208 seconds, or 2 hours 50 minutes. There were five successful reacquisitions, no search timeouts and no PPS timeouts.

| Cycle | First residual after gating | Residual when GPS returned to standby | Approximate effective holdover drift |
|---|---:|---:|---:|
| 1 | −225 µs | −102 µs | −0.13 ppm |
| 2 | +57 µs | +60 µs | +0.09 ppm |
| 3 | +547 µs | +316 µs | +0.27 ppm |
| 4 | +1,058 µs | +646 µs | +0.41 ppm |
| 5 | +1,673 µs | +963 µs | +0.57 ppm |

The approximately 30-minute holdover intervals therefore performed as follows:

- Early holdovers were comfortably within 1 ms.
- The fourth reacquisition measured about 1.06 ms error.
- The fifth measured about 1.67 ms error.
- The 60-second GPS-on window reduced that fifth residual to 963 µs before returning to standby.

The gradual progression is consistent with thermal/ageing drift of roughly 0.5–0.6 ppm relative to the initial calibration. FRACN has removed the large fixed oscillator bias, but a small changing residual remains.

The clean PPS mean during successive reacquisitions also shows this slow movement:

```text
Initial calibration: +0.100 ppm
Reacquisition 1:     +0.151 ppm
Reacquisition 2:     −0.096 ppm
Reacquisition 3:     −0.340 ppm
Reacquisition 4:     −0.440 ppm
Reacquisition 5:     −0.635 ppm
```

### PPS gating

The reacquisition gate is working as intended:

- Five initial edges were discarded on each reacquisition.
- Early L86 settling intervals included errors of several milliseconds.
- These generated 75 total gate rejections.
- After three clean intervals, the gate opened and subsequent PPS edges were accepted.
- Final anchor totals were `816 accepted / 75 rejected`.

All five reacquisitions succeeded:

```text
reacq=5/5
search=6/0
search_timeouts=0
pps_timeouts=0
checksum_err=0
uart_err=0
```

### UTC quality

The frequent `Degraded` state does not always mean a measured error greater than 1 ms. During holdover, `residual_us` is the last measured residual and remains frozen. Meanwhile, uncertainty grows to approximately 18 ms over a 30-minute standby, causing degradation after roughly 90 seconds.

At the end:

```text
last measured residual: 963 µs
holdover age:            268 seconds
reported uncertainty:   3.74 ms
status:                  Degraded
```

Using the latest observed drift of approximately 0.57 ppm, another 268 seconds would add around 150 µs. The probable instantaneous error at the end was therefore around 1.1 ms, although it was not directly observable without PPS.

### Conclusion

The fixed-frequency compensation is successful:

- The 7,000 ppm prescaler error is eliminated.
- The original approximately 9 ppm crystal bias is reduced to around 0.1 ppm at calibration.
- Thirty-minute holdover error remains below about 1.7 ms over this run.
- Reacquisition and PPS gating worked on every cycle.
- The remaining limitation is temperature-dependent drift and the fact that a 60-second tracking window does not always slew the accumulated phase error fully below 1 ms before returning to standby.

One telemetry caveat: `Gps state=Standby` continues to report `powered=true`. PPS and fixes stop, so standby behavior is clearly occurring, but this field does not prove that the GPS power rail is actually off.