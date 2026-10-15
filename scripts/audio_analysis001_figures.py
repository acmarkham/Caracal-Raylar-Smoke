"""Reproduce the figures and derived metrics for docs/audio/multichannel/analysis001.md.

Default input is the losslessly compressed evidence archive beside the report.
Use --raw and --target-log to import a fresh copy of the original debugger dump.
Requires NumPy, SciPy and Matplotlib. No target connection or firmware changes.
"""
import argparse
import hashlib
import json
import shutil
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from scipy import signal

from mic_noise_report import a_weight, integrate

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "docs/audio/multichannel/analysis001"
RAW_SHA256 = "09aabf72e4ef26444c019dd2a83df085a131f69d565f80ed78e52ce17605b011"
FS = 16000


def rice_model(pcm, shift):
    """V2's k estimate and Rice word lengths; residuals only, MIC2–MIC6.

    No first-sample anchors, headers, padding, metadata or reference FLAC.
    Fixed absolute shifts replace the production peak/retained-bit selection.
    """
    q = (pcm[:, 1:].astype(np.int64) + (1 << (shift-1) if shift else 0)) >> shift
    bit_count = 0
    residual_count = 0
    ks = []
    for start in range(0, len(q), 1024):
        delta = np.diff(q[start:start+1024], axis=0)
        folded = np.where(delta >= 0, 2*delta, -2*delta-1)
        n = len(delta)
        if n == 0:
            continue
        for c in range(5):
            mean = (int(folded[:, c].sum()) + n-1) // n
            k = max(0, mean.bit_length()-1)
            bit_count += int(((folded[:, c] >> k) + 1 + k).sum())
            residual_count += n
            ks.append(k)
    return dict(shift=shift, residual_bits=bit_count, residual_count=residual_count,
                bits_per_residual=bit_count/residual_count, min_k=min(ks), max_k=max(ks))


def psd(data):
    return signal.welch(data, FS, nperseg=8192, noverlap=4096, axis=0,
                        detrend="constant", scaling="density")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--raw", type=Path)
    parser.add_argument("--target-log", type=Path)
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    archive = OUT / "pillow-20261015.npz"
    if args.raw:
        pcm = np.fromfile(args.raw, dtype="<i4").reshape(-1, 6)
    else:
        with np.load(archive, allow_pickle=False) as saved:
            pcm = saved["pcm"]
            assert int(saved["sample_rate_hz"]) == FS
    assert pcm.shape == (64000, 6)
    raw_hash = hashlib.sha256(pcm.astype("<i4").tobytes()).hexdigest()
    assert raw_hash == RAW_SHA256, "this report is bound to the original four-second capture"
    if args.target_log:
        shutil.copyfile(args.target_log, OUT / "capture-target.log")
    # Reuse the original report's independent target/host integer verification.
    import re
    log = (OUT / "capture-target.log").read_text()
    rows = re.findall(r"NOISE channel=(\d+) frames=(\d+) sum=(-?\d+) squares=(\d+) min=(-?\d+) max=(-?\d+)", log)
    assert len(rows) == 6
    for channel, frames, total, squares, minimum, maximum in rows:
        x = pcm[:, int(channel)-1].astype(np.int64)
        assert tuple(map(int, (len(x), x.sum(), (x*x).sum(), x.min(), x.max()))) == tuple(map(int, (frames, total, squares, minimum, maximum)))
    if args.raw:
        np.savez_compressed(archive, pcm=pcm, sample_rate_hz=np.array(FS))
    data = pcm.astype(float)
    data -= data.mean(axis=0)
    f, p = psd(data)
    weights = a_weight(f)
    bands = [(20, 100), (100, 300), (300, 1000), (1000, 3000), (3000, 7100)]
    band_names = [f"{lo}-{hi}" for lo, hi in bands]
    rms_band = np.array([[np.sqrt(integrate(f, p[:, c], lo, hi)) for lo,hi in bands] for c in range(6)])
    a_rms = np.sqrt([integrate(f, p[:, c]*weights, 20, 7100) for c in range(6)])
    rms_audio = np.sqrt([integrate(f, p[:, c], 20, 7100) for c in range(6)])
    correlations = {}
    for lo,hi in [(20,100),(300,7100)]:
        sos = signal.butter(4, [lo,hi], btype="bandpass", fs=FS, output="sos")
        filtered = signal.sosfiltfilt(sos, data, axis=0)[1600:-1600]
        correlations[f"{lo}-{hi}"] = np.corrcoef(filtered, rowvar=False).tolist()
    quantization = []
    errors = {}
    for shift in range(5,11):
        reconstructed = ((pcm.astype(np.int64)+(1 << (shift-1))) >> shift) << shift
        error = (reconstructed-pcm).astype(float)
        ef, ep = psd(error)
        errors[shift] = ep
        a_error = np.sqrt([integrate(ef, ep[:,c]*a_weight(ef),20,7100) for c in range(6)])
        error_bands = np.array([[np.sqrt(integrate(ef,ep[:,c],lo,hi)) for lo,hi in bands] for c in range(6)])
        quantization.append(dict(shift=shift, step=1 << shift,
            error_rms=np.sqrt(np.mean(error**2,axis=0)).tolist(),
            a_error_rms=a_error.tolist(),
            a_noise_increase_db=(10*np.log10(1+(a_error/a_rms)**2)).tolist(),
            band_noise_increase_db=(10*np.log10(1+(error_bands/rms_band)**2)).tolist()))
    rice = [rice_model(pcm,s) for s in [0,1,5,6,7,8,9,10]]
    baseline = rice[1]["bits_per_residual"]
    for row in rice:
        row["reduction_from_observed_lossless_shift1_percent"] = 100*(1-row["bits_per_residual"]/baseline)
    metrics = dict(raw_sha256=raw_hash, sample_rate_hz=FS, frames=64000,
        a_rms=a_rms.tolist(), audio_band_rms=rms_audio.tolist(), bands=band_names,
        band_rms=rms_band.tolist(), low_frequency_power_percent=(100*(rms_band[:,0]/rms_audio)**2).tolist(),
        correlations=correlations, quantization=quantization, rice_residual_model=rice)
    (OUT / "derived-metrics.json").write_text(json.dumps(metrics,indent=2)+"\n")
    np.savetxt(OUT / "spectrum.csv",np.c_[f,p],delimiter=",",
               header="Hz,"+",".join(f"MIC{c+1}_counts2_per_Hz" for c in range(6)),comments="")
    plt.rcParams.update({"font.size":10,"axes.spines.top":False,"axes.spines.right":False})
    colors = plt.get_cmap("tab10").colors
    fig, axes = plt.subplots(2,1,figsize=(11,8),constrained_layout=True)
    for c in range(6):
        for ax in axes:
            ax.plot(f[1:],10*np.log10(p[1:,c]),label=f"MIC{c+1}",color=colors[c],lw=.8,alpha=.8)
    axes[0].set(xscale="log",xlim=(20,8000),ylim=(-25,70),title="Noise spectrum: all six unquantized channels")
    axes[0].annotate("Shared peak near 49 Hz",xy=(48.83,64),xytext=(110,60),arrowprops={"arrowstyle":"->"})
    axes[0].legend(ncol=6,loc="lower left")
    axes[1].set(xlim=(300,7100),ylim=(-25,35),title="Upper-band detail and measured quantization-error spectra")
    for s,color in zip([5,6,7],["#222222","#666666","#999999"]):
        axes[1].plot(f,10*np.log10(errors[s][:,1:].mean(axis=1)),ls="--",lw=1.2,color=color,label=f"Discard {s} LSBs: mean error, MIC2–6")
    axes[1].legend(fontsize=8,loc="upper left")
    for ax in axes:
        ax.set(xlabel="Frequency (Hz)",ylabel="PSD (dB re 1 count²/Hz)")
        ax.grid(alpha=.25)
    fig.savefig(OUT / "spectrum.png",dpi=180)
    plt.close(fig)
    fig, axes = plt.subplots(3,2,figsize=(12,10),sharex=True,sharey=True,constrained_layout=True)
    for c,ax in enumerate(axes.flat):
        sf,st,sp=signal.spectrogram(data[:,c],fs=FS,window="hann",nperseg=2048,noverlap=1536,detrend="constant",scaling="density",mode="psd")
        selected=sf>=20
        im=ax.pcolormesh(st,sf[selected],10*np.log10(np.maximum(sp[selected],1e-30)),shading="auto",cmap="magma",vmin=-5,vmax=45)
        ax.set(yscale="log",ylim=(20,8000),title=f"MIC{c+1}",xlabel="Time within capture (s)",ylabel="Frequency (Hz)")
        ax.set_yticks([30,50,100,300,1000,3000,7000],labels=["30","50","100","300","1k","3k","7k"])
    fig.colorbar(im,ax=axes.ravel().tolist(),label="PSD (dB re 1 count²/Hz); shared scale")
    fig.suptitle("Four-second pillow capture: 128 ms Hann windows, 32 ms hop; 7.8125 Hz bins")
    fig.savefig(OUT / "spectrograms.png",dpi=180)
    plt.close(fig)
    fig,axes=plt.subplots(1,2,figsize=(10,4.5),constrained_layout=True)
    for ax,(band,matrix) in zip(axes,correlations.items()):
        image=ax.imshow(matrix,vmin=0,vmax=1,cmap="viridis")
        ax.set(title=f"{band} Hz: time-domain correlation",xticks=range(6),yticks=range(6),xticklabels=range(1,7),yticklabels=range(1,7),xlabel="Microphone",ylabel="Microphone")
        for i in range(6):
            for j in range(6):
                ax.text(j,i,f"{matrix[i][j]:.2f}",ha="center",va="center",color="black" if matrix[i][j]>.6 else "white",fontsize=8)
    fig.colorbar(image,ax=axes,label="Pearson correlation coefficient")
    fig.savefig(OUT / "correlation.png",dpi=180)
    plt.close(fig)
    fig,axes=plt.subplots(1,2,figsize=(11,4.5),constrained_layout=True)
    shifts=np.array([q["shift"] for q in quantization])
    increases=np.array([q["a_noise_increase_db"] for q in quantization])
    for c in range(1,6):
        axes[0].plot(shifts,increases[:,c],"o-",label=f"MIC{c+1}",color=colors[c])
    axes[0].set(yscale="log",xlabel="Discarded LSBs",ylabel="Estimated A-weighted noise increase (dB)",title="Measured error; assumes uncorrelated noise")
    axes[0].legend(fontsize=8,ncol=2)
    axes[1].plot([r["shift"] for r in rice],[r["bits_per_residual"] for r in rice],"o-",color=colors[0])
    axes[1].set(xlabel="Discarded LSBs",ylabel="Modeled Rice bits per residual sample",title="Same PCM, MIC2–6; 1024-frame blocks")
    for ax in axes:
        ax.axvspan(4.8,7.2,color="#43864d",alpha=.12,label="Candidate range")
        ax.grid(alpha=.25)
    fig.savefig(OUT / "quantization-and-rate.png",dpi=180)
    plt.close(fig)
    print("Verified raw SHA-256 and all six target summaries.")
    print("Archive bytes:",archive.stat().st_size)
    print("Rice model:",[(r["shift"],round(r["bits_per_residual"],3),round(r["reduction_from_observed_lossless_shift1_percent"],1)) for r in rice])
    print("MIC2–6 noise increases:",[(q["shift"],round(min(q["a_noise_increase_db"][1:]),4),round(max(q["a_noise_increase_db"][1:]),4)) for q in quantization])
    print("Worst band increases MIC2–6:",[(q["shift"],np.round(np.max(np.array(q["band_noise_increase_db"])[1:],axis=0),4).tolist()) for q in quantization[:3]])


if __name__ == "__main__":
    main()
