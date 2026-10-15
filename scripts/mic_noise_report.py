"""Analyze interleaved, unquantized i32 PCM read from the noise-floor target bench.

Requires numpy, scipy and matplotlib. Integer counts are exactly the recorder's
right-justified PCM units; dBFS24 uses the RMS of a full-scale signed-24-bit sine.
This measures board + environment noise, not isolated microphone self-noise.
"""
import argparse
import hashlib
import json
import re
from pathlib import Path

import numpy as np
from scipy import signal


def a_weight(f):
    f2 = f * f
    amplitude = (12194.0**2 * f2**2 /
                 ((f2 + 20.6**2) * np.sqrt((f2 + 107.7**2) * (f2 + 737.9**2)) *
                  (f2 + 12194.0**2)))
    return amplitude**2 * 10**0.2


def integrate(f, psd, low, high):
    inside = (f > low) & (f < high)
    frequencies = np.r_[low, f[inside], high]
    values = np.r_[np.interp(low, f, psd), psd[inside], np.interp(high, f, psd)]
    return float(np.trapezoid(values, frequencies))


def analyze(raw, output, target_log=None):
    data_bytes = raw.read_bytes()
    integers = np.frombuffer(data_bytes, dtype="<i4").reshape(-1, 6)
    assert integers.shape == (64000, 6), "expected four seconds, six channels at 16 kHz"
    assert np.all((integers >= -(1 << 23)) & (integers < (1 << 23)))
    if target_log:
        matches = re.findall(r"NOISE channel=(\d+) frames=(\d+) sum=(-?\d+) squares=(\d+) min=(-?\d+) max=(-?\d+)", target_log.read_text())
        assert len(matches) == 6, "target summaries missing"
        for channel, frames, total, squares, minimum, maximum in matches:
            x = integers[:, int(channel) - 1].astype(np.int64)
            assert tuple(map(int, (len(x), x.sum(), (x*x).sum(), x.min(), x.max()))) == tuple(map(int, (frames, total, squares, minimum, maximum))), "RAM dump does not match target"
    data = integers.astype(np.float64)
    data -= data.mean(axis=0)
    f, spectra = signal.welch(data, fs=16000, nperseg=8192, noverlap=4096, axis=0,
                              detrend="constant", scaling="density")
    bands = [(20, 100), (100, 300), (300, 1000), (1000, 3000), (3000, 7100)]
    channels = []
    for c in range(6):
        x = data[:, c]
        psd = spectra[:, c]
        rms = np.sqrt(np.mean(x*x))
        band_rms = np.sqrt(integrate(f, psd, 20, 7100))
        a_rms = np.sqrt(integrate(f, psd * a_weight(f), 20, 7100))
        quarters = np.sqrt(np.mean(x.reshape(-1, 4000)**2, axis=1))
        bits_or = int(np.bitwise_or.reduce(np.abs(integers[:, c])))
        channels.append(dict(channel=c+1, mean=float(integers[:, c].mean()),
            rms=float(rms), rms_bits=float(np.log2(rms)), peak=int(np.abs(integers[:, c]).max()),
            rms_20_7100=float(band_rms), a_rms_20_7100=float(a_rms),
            dbfs24_20_7100=float(20*np.log10(band_rms/(2**23/np.sqrt(2)))),
            a_dbfs24_20_7100=float(20*np.log10(a_rms/(2**23/np.sqrt(2)))),
            zero_lsbs=(bits_or & -bits_or).bit_length()-1,
            quarter_rms_min=float(quarters.min()), quarter_rms_median=float(np.median(quarters)),
            quarter_rms_max=float(quarters.max()),
            delta_rms=float(np.sqrt(np.mean(np.diff(x)**2))),
            band_rms={f"{lo}-{hi}": float(np.sqrt(integrate(f, psd, lo, hi))) for lo,hi in bands}))
    quantization = []
    for shift in [5, 6, 7, 8, 9, 10, 11, 12]:
        # Match cheap CRCL: round to nearest, ties towards positive infinity.
        step = 1 << shift
        reconstructed = ((integers.astype(np.int64) + step//2) >> shift) << shift
        error = reconstructed - integers
        ef, ep = signal.welch(error.astype(float), fs=16000, nperseg=8192,
                              noverlap=4096, axis=0, scaling="density")
        a_errors = np.sqrt([integrate(ef, ep[:, c]*a_weight(ef), 20, 7100) for c in range(6)])
        ratios = a_errors / np.array([c["a_rms_20_7100"] for c in channels])
        quantization.append(dict(discard_bits=shift, step=step,
            error_rms=np.sqrt(np.mean(error.astype(float)**2, axis=0)).tolist(),
            a_error_rms=a_errors.tolist(),
            estimated_a_noise_increase_db=(10*np.log10(1+ratios**2)).tolist()))
    sos = signal.butter(4, [300, 7100], btype="bandpass", fs=16000, output="sos")
    filtered = signal.sosfiltfilt(sos, data, axis=0)
    correlation = np.corrcoef(filtered[1600:-1600], rowvar=False)
    spectrum_peaks = []
    for c in range(6):
        peaks, _ = signal.find_peaks(spectra[:, c])
        peaks = peaks[(f[peaks] >= 100) & (f[peaks] <= 7100)]
        peaks = peaks[np.argsort(spectra[peaks, c])[-5:]][::-1]
        spectrum_peaks.append([dict(hz=float(f[p]), density=float(spectra[p,c])) for p in peaks])
    result = dict(raw_file=str(raw), sha256=hashlib.sha256(data_bytes).hexdigest(),
        frames=len(data), channels=6, sample_rate=16000, seconds=len(data)/16000,
        target_summaries_verified=bool(target_log), psd_segment_frames=8192,
        psd_overlap_frames=4096, channels_measured=channels, quantization=quantization,
        correlation_300_7100=correlation.tolist(), largest_spectral_peaks=spectrum_peaks,
        limitations="Pillow-covered board; includes residual acoustic/electrical noise. GPS off, SD idle, no codec or radio. dBFS24 references signed-24-bit sine RMS; not microphone-input dBFS.")
    output.mkdir(parents=True, exist_ok=True)
    (output / "report.json").write_text(json.dumps(result, indent=2) + "\n")
    lines = ["# Pillow-covered board: raw microphone noise", "", result["limitations"], "",
        f"Capture: 4 seconds, 64,000 frames/channel, 16 kHz; SHA-256 `{result['sha256']}`.",
        f"Target integer sums, sums of squares, extrema verified: {bool(target_log)}.", "",
        "| MIC | RMS, all frequencies | log2(RMS) | RMS, 20 Hz–7.1 kHz | A-weighted RMS, same band | dBFS24(A) |",
        "|---|---:|---:|---:|---:|---:|"]
    for c in channels:
        lines.append(f"| {c['channel']} | {c['rms']:.1f} | {c['rms_bits']:.2f} | {c['rms_20_7100']:.1f} | {c['a_rms_20_7100']:.1f} | {c['a_dbfs24_20_7100']:.2f} |")
    lines += ["", "| Discard bits | Step | Error RMS, range across channels | Estimated A-weighted noise increase, range |", "|---|---:|---:|---:|"]
    for q in quantization:
        e, increase = q['error_rms'], q['estimated_a_noise_increase_db']
        lines.append(f"| {q['discard_bits']} | {q['step']} | {min(e):.1f}–{max(e):.1f} | {min(increase):.4f}–{max(increase):.4f} dB |")
    lines += ["", "Noise increases use measured quantization-error spectra and assume error is uncorrelated with existing noise; they do not establish beamforming accuracy."]
    (output / "report.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    np.savetxt(output / "spectrum.csv", np.c_[f, spectra], delimiter=",",
               header="Hz," + ",".join(f"MIC{c+1}_counts2_per_Hz" for c in range(6)), comments="")
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    fig, axes = plt.subplots(2, 1, figsize=(11, 8), constrained_layout=True)
    for c in range(6):
        axes[0].semilogx(f[1:], 10*np.log10(np.maximum(spectra[1:, c], 1e-30)), label=f"MIC{c+1}", alpha=.8)
        axes[1].plot(np.arange(16)/4+.125, np.sqrt(np.mean(data[:, c].reshape(-1,4000)**2,axis=1)), label=f"MIC{c+1}")
    axes[0].set(xlim=(20, 8000), xlabel="Frequency (Hz)", ylabel="PSD (dB re 1 count²/Hz)", title="Unquantized PCM: pillow-covered board, production MDF settings")
    for shift in [6, 8, 10]:
        density = (2**shift)**2 / 12 / 8000
        axes[0].axhline(10*np.log10(density), ls="--", alpha=.5, color="gray")
        axes[0].text(22, 10*np.log10(density)+.5, f"{shift} discarded bits: ideal white quantization noise", fontsize=8)
    axes[0].legend(ncol=6)
    axes[1].set(xlabel="Capture time (seconds)", ylabel="RMS (stored integer counts)", title="RMS in consecutive 250 ms windows")
    for ax in axes:
        ax.grid(True, alpha=.25)
    fig.savefig(output / "noise-spectrum.png", dpi=160)
    plt.close(fig)
    print("\n".join(lines[6:16]))
    print("Quantization:", [(q['discard_bits'], round(max(q['estimated_a_noise_increase_db']),4)) for q in quantization])
    print("Correlation 300-7100 Hz:\n", np.round(correlation, 3))
    print("Spectral peaks:", [[round(p['hz'],1) for p in peaks] for peaks in spectrum_peaks])


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("raw", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--target-log", type=Path)
    args = parser.parse_args()
    analyze(args.raw, args.output, args.target_log)
