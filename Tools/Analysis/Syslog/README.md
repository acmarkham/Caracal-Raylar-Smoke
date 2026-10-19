# Interactive syslog report

Generate one self-contained HTML file from an Aardwolf syslog. Plotly is embedded: opening the result needs no Python, internet connection, CDN, map service or local server.

```powershell
rtk proxy python -m pip install -r Tools/Analysis/Syslog/requirements.txt
rtk proxy python Tools/Analysis/Syslog/syslog_report.py E:\syslog.txt -o analysis\syslog-report.html
```

The input log and recordings are read only. Output is written only to the specified destination. Without `-o`, output defaults to `syslog.report.html` beside the input; specify `-o` to keep the card untouched.

The report includes:

- A boot selector; device, firmware and SD identity/capacity where available; reset flags and logging gaps.
- Recording configuration, finalized duration, errors, incomplete/missing minutes, source-frame joins and compression against full-resolution raw PCM for all recorded channels.
- GPS acquisition states, accepted/rejected PPS anchors and retained frequency-calibration lock.
- Actual logged PPS residuals and the first observed PPS return after each holdover gap. Equivalent divergence in ppm is `residual_us / interval_seconds`; uncertainty growth and estimated holdover drift are excluded from these graphs.
- Battery SOC, voltage, charging/source and external supply curves.
- Sensor die temperatures, acceleration, gravity-based pitch/roll, magnetic field and uncalibrated magnetic XY bearing.
- Offline position plots with X/Y coordinates obfuscated modulo a 50 km grid.
- Radio states, TX/RX counts per hour and configuration, hourly configuration heatmaps, and received RSSI/SNR when packets are present.
- CPU load, audio buffering and explicit fault records.

Time axes use **UTC**, anchored from the log, or **hours since boot** if no UTC anchor exists. Zooming a time chart updates the other time charts when “Link time-axis zoom” is enabled. Categorical configuration charts and the coordinate view keep their own axes. Legends toggle individual series; the toolbar exports individual plots as PNG.

## Recording validation

When numeric hourly recording folders are beside the syslog, they are discovered automatically. For an archived log and recordings stored elsewhere:

```powershell
rtk proxy python Tools/Analysis/Syslog/syslog_report.py analysis\archived-syslog.txt --audio-root E:\ -o analysis\archived-report.html --json analysis\archived-summary.json
```

Files are restricted to the run's boot ID and recording time span. Logged `audio stopped` / `audio started` pairs exclude intentional pauses from missing-minute and cross-session continuity checks. Checks read FLAC/WAV metadata and CRCL v2 END records: reference sample totals/length, completion status, block totals, segment identity and adjacent source-frame continuity. CRCL planned lengths allow a small tolerance for UTC boundary adjustment. Missing companions, missing minute entries, incomplete segments and zero-length final files appear red. Compression uses **finalized file bytes** divided into the corresponding uncompressed PCM size; separate log-counter estimates include open files.

**Green means finalized and aligned at this metadata/completion-check level.** This quick report does not decode every FLAC frame or Rice payload bit. It cannot prove payload integrity. Use the deeper audio verification tools when that is required. WAV without source-frame metadata has weaker alignment evidence. CRCL versions other than v2 are reported as unsupported instead of silently accepted.

For log-only analysis:

```powershell
rtk proxy python Tools/Analysis/Syslog/syslog_report.py some-syslog.txt --no-audio-check -o report.html
```

Audio correctness stays amber/unverified unless an explicit logged error makes it red. Missing telemetry stays unavailable. If audio dimensions are absent, log-counter duration/compression estimates assume the Aardwolf defaults of 16 kHz and 32 bits; recording metadata is preferable. SD startup records may be missing due to early logger drops: recording metadata can supply device/firmware identity, but an absent SD serial is not invented. Mounted filesystem capacity is clearly distinguished from logged card identity.

## Interpreting timing and sensors

Location obfuscation is on by default. Coordinates are projected to Web Mercator metres, then each axis is reduced modulo 50,000 m. The report shows only wrapped X/Y in metres, without latitude/longitude or the original grid cell. The 50 km period is in projected coordinates; ground distance varies with latitude. Local position variation remains visible, with jumps possible at grid boundaries. Recognized precise coordinate fields are removed from parsed records and scrubbed from diagnostic text before generating HTML or optional JSON. Source logs and audio files are unchanged.

This is simple deterministic obfuscation, not encryption or anonymization: outside knowledge can narrow down the possible grid cells. Other report fields, such as device identity and timestamps, remain present. Older generated reports and raw logs still contain their original data; regenerate the HTML before sharing it.

A retained `locked=true` frequency calibration does not mean GPS currently supplies PPS. At relock the plotted residual is an actual observation; sparse logs may publish after the first accepted edge, and correction may already have started. The ppm value is an average divergence proxy relative to the last **logged** PPS, not a direct oscillator frequency measurement. Its sign follows the logged residual.

Accelerometer tilt assumes gravity dominates. Magnetic bearing is `atan2(Y,X)`, without tilt compensation, hard/soft-iron calibration or declination correction. Die temperatures are not calibrated ambient temperature. Missing SNR for GFSK or missing packet metadata is not replaced with zero.

## Tests

```powershell
rtk proxy python -m unittest discover -s Tools/Analysis/Syslog -p test_syslog_report.py -v
```

Tests use temporary synthetic logs and recording headers; they do not access the card. They cover boots/sequence gaps, optional values, actual residual selection, invalid recordings, source-frame gaps, packet matching/units, 50 km coordinate wrapping, removal of precise coordinates from diagnostic text/report payloads, and safe offline HTML embedding.

An optional browser smoke test uses Node.js 22+ and Chromium, without npm packages:

```powershell
rtk proxy node Tools/Analysis/Syslog/browser_smoke.mjs analysis\syslog-report.html
```

Pass a Chromium executable path as a second argument if needed. This checks chart rendering, boot switching, linked zoom, wrapped location ranges, JavaScript exceptions and absence of external network requests. It saves `browser-check.json` and drift/location screenshots alongside the HTML.
