"""Build the Markdown report and local audio comparison page from saved metrics."""
import hashlib
import html
import json
from pathlib import Path
import numpy as np
from outdoor_audio_analysis import OUT, FS

def main():
    a=json.loads((OUT/'analysis.json').read_text())
    d=json.loads((OUT/'decode.json').read_text())
    events=a['events'];noise=a['noise']['rows'];pillow=noise['Pillow MIC1 (4 s)']
    whole=noise['Outdoor MIC1 (60 s)'];quiet=noise['Outdoor MIC1 (quietest 4 s)']
    lines=['# Outdoor urban six-channel audio: 15 October 2026','',
        'Source: `E:\\1792062000\\aud_1792065540.flac` and its `.CRCL` companion. Analysis uses the saved copies below; source files were not modified. All call times are offsets into the recording.', '',
        '**Results:** the outdoor ambient spectrum is much stronger below 5 kHz than the pillow capture. The CRCL decodes completely and the exported six-channel FLAC preserves the reconstructed integer samples exactly, including lossless MIC1. Bird calls support useful directional enhancement, but the direction of the 53.5 s call is ambiguous. These recordings do not provide a calibrated absolute bearing or a clean ground-truth target for SNR measurement.','',
        '## Listen first','',
        'Open **[the local listening page](index.html)** for matched audio players. Start with the **1–5 kHz MIC1 / delay-and-sum / MVDR** comparisons. All methods in that group share the same filter and gain, across all five calls. Full-band versions are also provided. This report is based on signal analysis; subjective audio quality is left for listening review.','',
        '| Recording | Purpose |','|---|---|',
        '| [Original mono FLAC](aud_1792065540.flac) | Byte-for-byte source copy, original 32-bit scale |',
        '| [Six-channel archival FLAC](aud_1792065540_6ch.flac) | Exact reconstructed counts, 32-bit, channel order physical MIC1–MIC6 |',
        '| [Six-channel playback FLAC](listening/outdoor_6ch_playback.flac) | Same channels with common gain, 24-bit for player compatibility |',
        '| [Original CRCL](aud_1792065540.CRCL) | Preserved paired source for independent decoding |','',
        'Six-channel FLAC is often presented by players as 5.1 surround. These are **six microphone streams**, not surround speakers: disable surround processing/downmixing and select individual channels, or use the [separate mono files](index.html#channels). No per-microphone loudness normalization was applied.','',
        '| Call | AoA used | MIC1, 1–5 kHz | MVDR, 1–5 kHz | Delay-and-sum, 1–5 kHz | Unfiltered MVDR |','|---|---:|---|---|---|---|']
    for e in events:
        audio=e['audio'];c=e['center_s']
        lines.append(f"| {c:.1f} s | {e['azimuth_deg']:+.0f}° | [reference]({audio['mic1_band']}) | [MVDR]({audio['mvdr_band']}) | [DAS]({audio['das_band']}) | [full band]({audio['mvdr']}) |")
    lines += ['',
        'Each clip is 1.15 s, starting 0.45 s before the supplied call time. There are also six-channel clips and MIC1 quantization-proxy A/B files in `listening/`. For the two calls with strong alternative directions, compare [24.0 s at −18°](listening/call_24.0s_mvdr_alternative_band.flac) and [53.5 s at +9°](listening/call_53.5s_mvdr_alternative_band.flac) with the primary beams. These alternatives reuse the primary horizontal projection; they are listening diagnostics rather than independently optimized secondary solutions.','',
        '## 1. Capture identity and reconstruction','',
        '| Property | Value |','|---|---|',
        '| Start UTC, as recorded | 2026-10-15 11:59:00 UTC (`1792065540`) |',
        '| Sample rate / frames / duration | 16,000 Hz / 960,001 per channel / 60.0000625 s |',
        '| Reference / companion sizes | 1,392,130 / 2,645,628 bytes; combined 4,037,758 bytes |',
        '| Container / codec | CRCL v2, independent per-channel shift quantization and temporal Rice |',
        '| Block / precision target | 1,024 frames (64 ms), signed 10-bit symmetric target, per-channel/per-block shift |',
        '| Stream identity | `0009edc5df5d06009bbddb0000000000` |',
        '| Device / boot / first source frame | `381fe484` / `37257` / `14400923` |',
        '| Channel mapping | Post-fix physical MIC1–MIC6, confirmed by user; no 4/5 exchange applied |',
        '| Firmware identity | CRCL says `unknown`; FLAC firmware hash/version empty |',
        '| Recorded time quality | `utc_status=2`, uncertainty 3274 µs, calibrated clock −1387 ppb; last GPS PPS 11:53:45 UTC |','',
        'The existing independent decoder was run strictly, without recovery. All 938 DATA records, sample offsets/sequences, declared payload bounds, reference segment/device/boot identity, END totals and reference length matched. END status is complete (`1`), with no recovery error. The extra one sample beyond 60 s is present in the sources and retained; nothing was trimmed or resampled. CRCL v2 has **no CRC or reference PCM digest**: structural validation does not prove immunity to undetectable payload corruption. The hashes below identify these particular saved inputs.','',
        'MIC1 is copied directly from the reference. MIC2–MIC6 reconstruct as `q << shift`; temporal prediction/Rice decoding adds no further loss. An FFmpeg decode of the six-channel archival FLAC matches the entire decoded six-channel i32 byte stream exactly, and its MIC1 equals the source reference sample for sample.','',
        '## 2. Outdoor spectrum versus the indoor pillow baseline','',
        'The fair comparison uses **lossless MIC1** on both captures, in the recorder’s right-justified integer count units. The MDF settings agree with the earlier production-chain diagnostic: 3.2 MHz PDM, Sinc5/R50, scale 0x27 (−26.6 dB), reshape /4, approximately 10 Hz HPF, `pcm_shift=8`, 16 kHz PCM. A 32-bit FLAC word does not mean that the microphone uses its full 32-bit numerical range.','',
        'Welch PSD: Hann window, 8192 samples, 4096 overlap, constant detrending, density scaling, 1.953125 Hz bins. Integrations interpolate the band endpoints, matching [Analysis 001](../../../docs/audio/multichannel/analysis001.md). The pillow baseline is four seconds; outdoor statistics use the whole minute plus the quietest four-second window, **36–40 s**, selected by minimum A-weighted 20 Hz–7.1 kHz RMS among windows stepped by two seconds. This selected minimum is a descriptive lower ambient level, not an unbiased estimate of electronic self-noise.','',
        '| Band / weighting | Pillow MIC1 RMS, counts | Outdoor whole minute | Outdoor quiet 36–40 s | Whole vs pillow | Quiet vs pillow |','|---|---:|---:|---:|---:|---:|']
    for key,label in [('20-100','20–100 Hz'),('100-300','100–300 Hz'),('300-1000','300–1000 Hz'),('1000-5000','1–5 kHz'),('5000-7100','5–7.1 kHz'),('20-7100','20 Hz–7.1 kHz'),('A_20-7100','A-weighted, 20 Hz–7.1 kHz')]:
        lines.append(f'| {label} | {pillow[key]:.1f} | {whole[key]:.1f} | {quiet[key]:.1f} | {20*np.log10(whole[key]/pillow[key]):+.1f} dB | {20*np.log10(quiet[key]/pillow[key]):+.1f} dB |')
    lines += ['',
        '![Same-scale noise spectra](noise-comparison.png)','',
        'The outdoor spectrum is dominated by broad low-frequency energy consistent with urban rumble/wind or handling, with stronger time-varying components around 16–20 s. Sound identity is not established from PSD alone. In contrast, the pillow measurement was dominated by narrow common interference, including the ~48.8 Hz peak. Below 100 Hz contributes 97.4% of pillow 20–7100 Hz power, 87.4% over the outdoor minute and 98.9% in the selected quiet outdoor window.','',
        'The outdoor quiet 1–5 kHz level is 9.8 dB higher and its A-weighted level is 9.3 dB higher than the pillow capture. The change is frequency dependent: the selected outdoor 5–7.1 kHz level is **2.3 dB lower**. There is no evidence here that intrinsic microphone self-noise changed; environment, pillow attenuation, recorder/SD activity and supply configuration all differ. The original diagnostic bypassed SD/codecs and used a different supply-mode setup. See the linked earlier report for sensitivity, SNR and MDF-chain calculations.','',
        '![Outdoor reference spectrogram](overview.png)','',
        '## 3. Actual quantization and quality implications','',
        'The minimum six-LSB rule does **not** imply that every block uses a 64-count step. The peak/10-bit rule can choose a larger absolute shift, with one rounding operation. Measured shifts span 6–12 bits: steps of **64–4096 counts**. A low-frequency peak can therefore reduce the precision available to a simultaneous quiet high-frequency bird call.','',
        '| Channel | Observed shifts | Blocks at shift 6 | Blocks at shift ≥10 | Median shift |','|---|---|---:|---:|---:|']
    for c in range(5):
        shifts=np.array([r['shifts'][c] for r in d['records']]);n=len(shifts)
        lines.append(f'| MIC{c+2} | {shifts.min()}–{shifts.max()} | {np.sum(shifts==6)} / {n} ({100*np.mean(shifts==6):.1f}%) | {np.sum(shifts>=10)} / {n} ({100*np.mean(shifts>=10):.1f}%) | {np.median(shifts):.0f} |')
    proxy=a['mic1_quantizer_proxy']
    lines += ['',
        'Counts above are per DATA block (the final block is shorter). The uniform-rounding model gives error RMS Δ/√12: 18.5 counts for shift 6, up to 1182 counts for shift 12. That is a model, not measured MIC2–MIC6 error. Their raw originals were not saved, so this recording cannot directly measure their distortion or perform a same-microphone original-versus-packed test. Comparing MIC1 with another microphone also includes physical location, sensitivity and acoustic-path differences.','',
        'To offer a controlled listening comparison, the same production rule was applied offline to **lossless MIC1** in the actual record boundaries. The saved `mic1_quantized_proxy` files compare identical source data before/after quantization. These are explicitly simulated proxies; MIC1 in both original and six-channel archival FLAC remains untouched.','',
        f"The whole-minute MIC1 proxy error is {proxy['rms_error_counts']:.1f} counts RMS, {proxy['max_abs_error_counts']} counts peak; its 1–5 kHz error is {proxy['error_rms_1000_5000']:.1f} counts RMS, {proxy['signal_to_error_1000_5000_db']:.1f} dB below original signal-plus-ambient energy in that band. The 1.15 s call-clip band signal-to-error ratios are:", '',
        '| Call | MIC1 proxy 1–5 kHz signal-to-error |','|---|---:|']
    for e in events:lines.append(f"| {e['center_s']:.1f} s | {e['proxy_signal_to_error_band_db']:.1f} dB |")
    lines += ['',
        'These include ambient energy and are not isolated bird distortion ratios or listening scores. The 50.1 s proxy is the weakest of these comparisons. The outdoor result supports investigating the **peak-driven precision loss** separately from the six-bit minimum: further coarse quantization is not justified simply because total low-frequency RMS is large. A deliberate low-frequency conditioning strategy before quantization, or more retained precision during large peaks, could be evaluated against these clips; neither change was applied here.','',
        '![Actual shifts and simulated MIC1 error spectrum](quantization.png)','',
        '## 4. Bird-call direction estimates','',
        'Geometry is from the [pinned PCB acoustic-port coordinates](../../../docs/audio/multichannel/analysis002/geometry.json). Azimuth **0° points towards the top of the PCB**, +90° right, ±180° bottom, −90° left. This is board-relative, not geographic bearing. The user confirmed the recording follows the MIC4/MIC5 fix; the corrected mapping also gives a higher phase-coherence scan peak than the legacy mapping for all five events.','',
        'The estimator is a weighted broadband pair-phase/SRP-PHAT-style scan over all 15 pairs. It uses 512-sample Hann STFT windows, 128-sample hop, and 1.2–5 kHz bins weighted by target excess over adjacent background on MIC1. Bins must exceed twice local background power to receive weight. The local background is drawn from call time −0.8…−0.25 s and +0.4…+0.8 s, excluding the listed calls with 70 ms guards. Candidate azimuths are −180…179° at 1° spacing, with horizontal projection q=0.10…1.00 at 0.025 spacing, and c=343 m/s.','',
        'For source unit vector u=(sin θ, −cos θ), arrival delays are τᵢ=−q rᵢ·u/c relative to MIC1, and steering aᵢ(f)=exp(−j2πfτᵢ). The projection q allows a source out of the board plane without forcing an incorrect in-plane speed. Its apparent elevation is not treated as a calibrated height: a planar array cannot resolve above/below, and calibration/reflections also affect q.','',
        '| Call | Active window | Best azimuth | q | Phase score | Assessment |','|---|---|---:|---:|---:|---|']
    assessments=['Rear-left candidate; low/high bands agree, weak tail switches to front. Provisional.',
                 'Front-right; early/late and low/high-band estimates 26–28°. Internally consistent.',
                 'Front-right; subset estimates 16–18°. Strongest phase score of these calls.',
                 'Front; low/high bands 9–10°, weak late tail switches rear. Provisional.',
                 'Ambiguous front/rear: low band ~0°, high band ~173°; competing +9° peak.']
    for e,note in zip(events,assessments):
        lo,hi=e['active_s'];lines.append(f"| {e['center_s']:.1f} s | {lo:.2f}–{hi:.2f} s | {e['azimuth_deg']:+.0f}° | {e['horizontal_projection']:.3f} | {e['score']:.3f} | {note} |")
    lines += ['',
        '![Direction scan and horizontal-projection ambiguity](doa-scans.png)','',
        'The phase score is an average weighted pair agreement (an ideal noiseless model approaches 1), **not a probability**. Weighted phase-model residuals are about 47–76°, so these are preliminary direction candidates rather than degree-accurate localization. The grid spacing and narrow subset ranges do not establish absolute accuracy. Unknown channel phase response, reflections, short tonal calls and spatial aliasing at the array’s ~10 cm aperture can bias or create competing peaks. The 53.5 s result should not be reported as a resolved +178° source. It may contain reflections or mixed paths; the data do not identify which peak is the physical source.','',
        '![Bird-call spectral windows](calls-spectrogram.png)','',
        '![Bins and time frames used for direction estimates](doa-masks.png)','',
        '## 5. Beamforming and measured enhancement','',
        'Both delay-and-sum (DAS) and frequency-domain MVDR use the primary scan direction and all six decoded channels. MVDR noise covariance uses the same adjacent, guarded background frames; it is estimated separately for each call. STFT is 512 samples / hop 128, with inverse overlap-add reconstruction. Initial diagonal loading is 10% of mean diagonal covariance, increased where needed to keep squared weight norm ≤1 (white-noise gain ≥0 dB). The primary beams required 10–20% loading. The numerical distortionless constraint wᴴa=1 was checked at every bin.','',
        'Full-band outputs are unfiltered. The `_band` files apply the identical fourth-order Butterworth 1–5 kHz bandpass forward and backward to reference, DAS and MVDR; these are offline, zero-phase listening variants with the doubled magnitude response of forward/backward filtering. No neural enhancement, denoising mask, independent normalization or sample-rate conversion is used.','',
        '| Call | MIC1 contrast | DAS contrast | MVDR contrast | MVDR vs MIC1 |','|---|---:|---:|---:|---:|']
    for e in events:
        b=e['beam_metrics'];m=b['mic1']['excess_to_noise_db'];ds=b['das']['excess_to_noise_db'];mv=b['mvdr']['excess_to_noise_db']
        lines.append(f"| {e['center_s']:.1f} s | {m:.1f} dB | {ds:.1f} dB | {mv:.1f} dB | {mv-m:+.1f} dB |")
    lines += ['',
        'Contrast = 10 log₁₀[(mean active-window band power − mean adjacent-background band power) / mean adjacent-background band power], using STFT bins from 1–5 kHz. The observed MVDR contrast gains are about **3.7–6.8 dB**. This is an in-recording diagnostic: background stationarity is assumed, the same background estimates drive MVDR and its score, and there is no clean target. It is not an independent SNR benchmark or proof that the bird waveform is distortionless. Target power also changes with imperfect steering, so compare timbre and audibility using the provided clips rather than selecting by this number alone.','',
        '![Reference versus DAS and MVDR, identical spectral color scale](beamforming-comparison.png)','',
        '## 6. Audio scaling, provenance and reproduction','',
        'The archival original and archival six-channel FLAC use the recorder’s original signed i32 values. Their numerical level is quiet in a conventional 32-bit player. The listening files are 24-bit FLAC, generated from floating-point count values with explicit common gain and checked after writing to within one 24-bit quantization step. All outputs are finite and below full scale. Three gain groups make listening practical without confounding comparisons:', '',
        '| Group | Linear playback amplitude per input count | Gain versus original FLAC32 |','|---|---:|---:|']
    for label,key in [('Full-minute files','playback_gain_per_count'),('Full-band short clips, including six-channel clips','clip_gain_per_count'),('1–5 kHz short clips','band_clip_gain_per_count')]:
        gain=a[key];lines.append(f'| {label} | {gain:.12g} | {20*np.log10(gain*2**31):+.2f} dB |')
    lines += ['',
        'Gain is fixed across microphones, methods, alternatives and calls **within each group**. Do not compare loudness across gain groups. The extra playback quantization step is at most 0.22 input counts for full-minute files, much smaller than the 64-count minimum CRCL step; the archival FLAC has no added quantization.','',
        '| Saved input | SHA-256 |','|---|---|']
    for name,digest in d['source_sha256'].items():lines.append(f'| `{name}` | `{digest}` |')
    lines += ['',
        'The [decode report](decode.json) records header metadata, all block shifts/Rice k values, source identities and structural checks. [Analysis metrics](analysis.json) retain spectra-derived levels, scan scores, subset checks, gains and proxy errors; [noise-spectrum.csv](noise-spectrum.csv) contains the plotted PSDs. [manifest.json](manifest.json) lists artifact hashes; [verification.json](verification.json) records audio parameters and verification results. The saved `decoded_counts.npy` contains the exact decoded six-channel integers.','',
        'From the repository root, with Python, NumPy, SciPy, Matplotlib, SoundFile and FFmpeg installed:', '',
        '```powershell',
        'rtk proxy python scripts/outdoor_audio_probe.py --source analysis/aardwolf_multichannel/analysis20261015',
        'rtk proxy python scripts/outdoor_audio_analysis.py --overview',
        'rtk proxy python scripts/outdoor_audio_analysis.py',
        'rtk proxy python scripts/outdoor_audio_report.py',
        'rtk proxy python scripts/outdoor_audio_verify.py',
        '```','',
        'For acquisition from the original card, omit `--source` from the first command. The decoder requires strict complete paired inputs. The analysis checks STFT reconstruction of MIC1, unity steering response, finite/nonclipped audio, playback sample counts/scaling, and an analytic known-angle steering example (+37°, q=0.8). Additional verification exercises the end-to-end estimator on a synthetic finite-distance source and verifies the exported six-channel/reference PCM. These numerical checks verify software conventions and file export, not physical array calibration.','']
    (OUT/'report.md').write_text('\n'.join(lines),encoding='utf8')
    # Portable local HTML: no scripts, external assets, or network requests.
    parts=['<!doctype html><html lang="en"><meta charset="utf-8"><title>Outdoor multichannel listening comparison</title>',
        '<style>body{font:16px system-ui;margin:2rem auto;max-width:1400px;padding:0 1rem;color:#18212c;background:#f5f7fa}table{border-collapse:collapse;width:100%;background:white}td,th{border:1px solid #d4dbe3;padding:.6rem;text-align:left}audio{width:245px;display:block}img{max-width:100%;background:white}small{display:block;color:#526071}a{color:#1766aa}.scroll{overflow:auto}section{margin:2rem 0}</style>',
        '<h1>Outdoor six-microphone recording</h1><p>15 October 2026 · 16 kHz · post-fix MIC1–MIC6. <a href="report.md">Full report</a> · <a href="aud_1792065540_6ch.flac">Exact six-channel FLAC32</a> · <a href="aud_1792065540.flac">Original mono FLAC32</a></p>',
        '<p>Start with the band-limited comparisons below. Gain is identical across methods and calls within each group. Do not compare loudness across filtered/full-band/full-minute groups. Disable player normalization and surround processing. Directions are preliminary board-relative candidates: 0° top, +90° right. The 53.5 s call is front/rear ambiguous.</p>']
    def player(path):return f'<audio controls preload="none" src="{html.escape(path)}"></audio><small><a href="{html.escape(path)}">Download FLAC</a></small>'
    for title,keys in [('Bird calls: identical 1–5 kHz filtering and common gain',['mic1_band','das_band','mvdr_band','mic1_quantized_proxy_band']),
                       ('Full-band call clips: common gain',['mic1','das','mvdr','sixch'])]:
        labels=['MIC1 reference','Delay-and-sum','MVDR','MIC1 quantized proxy' if keys[-1].endswith('_band') else 'Six microphones (select channels)']
        parts+=['<section><h2>'+title+'</h2><div class="scroll"><table><tr><th>Call / angle</th>'+''.join('<th>'+v+'</th>' for v in labels)+'</tr>']
        for e in events:
            parts.append(f"<tr><td>{e['center_s']:.1f} s<br>{e['azimuth_deg']:+.0f}°"+('<br>AMBIGUOUS' if e['center_s']==53.5 else '')+'</td>'+''.join('<td>'+player(e['audio'][key])+'</td>' for key in keys)+'</tr>')
        parts+=['</table></div></section>']
    parts+=['<section><h2>Alternative directions, 1–5 kHz</h2>']
    for e in events:
        if 'alternative_mvdr_azimuth_deg' in e:
            parts.append(f"<p>{e['center_s']:.1f} s: alternative {e['alternative_mvdr_azimuth_deg']:+.0f}°</p>"+player(e['audio']['mvdr_alternative_band']))
    parts+=['</section><section id="channels"><h2>Full minute: compare individual microphones</h2><p>All share the same gain. MIC2–MIC6 are the actual reconstructed packed channels. The MIC1 proxy applies the codec rule offline to the same MIC1 signal; it isolates quantization from microphone placement.</p><div class="scroll"><table><tr>']
    for c in range(6):parts.append(f'<td>MIC{c+1}'+player(f'listening/outdoor_MIC{c+1}.flac')+'</td>')
    parts+=['</tr></table></div><p>MIC1 quantized proxy</p>'+player('listening/outdoor_MIC1_quantized_proxy.flac'),
        '<p>Six-channel playback file</p>'+player('listening/outdoor_6ch_playback.flac')+'</section>',
        '<section><h2>Noise spectrum and direction scans</h2><img src="noise-comparison.png" alt="Outdoor and pillow spectral comparison"><img src="doa-scans.png" alt="Direction scans"></section></html>']
    (OUT/'index.html').write_text('\n'.join(parts),encoding='utf8')
    print('Wrote report.md and index.html')

if __name__=='__main__':main()
