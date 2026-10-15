# Pillow-covered raw microphone capture, 2026-10-15

Four continuous seconds from all six microphones on the connected STM32U595,
after two seconds of settling. Production MDF configuration: 3.2 MHz PDM,
Sinc5/R50, SCALE 0x27, reshape /4, 16 kHz PCM and approximately 10 Hz HPF.
The diagnostic bypassed CRCL and FLAC and did not start GPS, SD or radio tasks.
Core supply selection was left at the HAL default, unlike Aardwolf's explicit
SMPS setting. This is a board/environment observation, not isolated self-noise
or a complete production-concurrency noise test.

Each channel contains 64,000 right-justified signed-24-bit samples in i32 words.
DMA sequences 21 through 60 were contiguous, with no reported capture error.
The 1,536,000-byte debugger dump exactly matched each channel's target-side sum,
sum of squares, minimum and maximum. One least-significant bit is zero in all
captured values; no additional precision reduction was applied by the diagnostic.

| MIC | Full-band AC RMS, counts | log2(RMS) | 20 Hz–7.1 kHz RMS | A-weighted RMS, 20 Hz–7.1 kHz |
| --- | ---: | ---: | ---: | ---: |
| 1 | 3683 | 11.85 | 3068 | 259 |
| 2 | 3767 | 11.88 | 3096 | 234 |
| 3 | 3826 | 11.90 | 3150 | 219 |
| 4 | 4332 | 12.08 | 3795 | 337 |
| 5 | 4003 | 11.97 | 3355 | 249 |
| 6 | 3933 | 11.94 | 3352 | 316 |

The A-weighted levels correspond to log2(RMS) = 7.77–8.40, or −88.67 to
−84.91 dBFS24(A), referenced to a full-scale signed-24-bit sine's RMS.
They are not referenced to full-scale i32 or directly to microphone PDM full scale.

Approximately 97.4–98.0% of the measured 20 Hz–7.1 kHz power is below 100 Hz.
The dominant PSD peak is at 48.83 Hz (1.95 Hz spectral-bin spacing); bandpassed
20–100 Hz interchannel correlations are 0.978–0.995. There is strong common
low-frequency interference/acoustic content. This test cannot distinguish mains
pickup, room sound, mechanical vibration or another common source. Above 300 Hz,
correlations fall to 0.035–0.266; channel-dependent upper-band humps also remain.
The broadband 12-bit RMS level must not be treated as 12 dispensable low bits.

## Quantizing these same samples

Round to nearest, ties toward positive infinity, matching cheap CRCL. The table
uses measured error PSD, integrated with A weighting over 20 Hz–7.1 kHz. Noise
increases assume the quantization error is uncorrelated with existing noise.

| Discarded LSBs | Integer step | Error RMS, counts | Estimated A-weighted noise increase across six channels |
| --- | ---: | ---: | ---: |
| 6 | 64 | approximately 18.5 | 0.012–0.029 dB |
| 7 | 128 | approximately 37.0 | 0.049–0.118 dB |
| 8 | 256 | approximately 73.9 | 0.193–0.445 dB |
| 9 | 512 | approximately 147.8 | 0.731–1.577 dB |
| 10 | 1024 | approximately 295.6 | 2.351–4.241 dB |

Six discarded bits remain a conservative starting point; seven is a useful next
experiment, and eight trades more upper-band precision for rate reduction.
Noise is colored, so A-weighted totals do not establish beamforming or AoA fidelity.
These are absolute shifts in recorder sample units, not retained block bit depths.

## Artifacts and reproduction

- Raw PCM: `target/noise-floor/pillow-raw-01.i32le`, SHA-256
  `09aabf72e4ef26444c019dd2a83df085a131f69d565f80ed78e52ce17605b011`.
- Target log: `.probe-rs-logs/20261015-112123-service-test-crclencoder-21000/target.log`;
  preserved with the artifacts as `target/noise-floor/capture-target.log`.
- Capture ELF SHA-256:
  `4387b3febf4714e755c13ce175e06f9f2cba13f83f301e142cff746acdbe5c30`.
- Capture ELF and build record: `target/noise-floor/noise-capture.elf` and
  `target/noise-floor/capture-build.json`.
- Spectrum PNG, CSV, report JSON and Markdown: `target/noise-floor/pillow-01/`.
- Welch PSD: Hann windows, 8192 frames, 4096 overlap, constant detrending.
- Reproduction commands: [README](README.md#raw-microphone-noise-diagnostic).

The pre-test Aardwolf ELF was preserved and restored with flash verification.
Its SHA-256 is `b991f2a843f315941bcabfc2360d72bdb15ba066c20e72d20179728134d01bec`.
Restore log: `.probe-rs-logs/20261015-112616-aardwolf-36572/`.
