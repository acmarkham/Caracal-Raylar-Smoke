"""Reproduce Analysis 002 from the portable music/chirp evidence archives.

Run from the repository root. Requires NumPy, SciPy and Matplotlib; no board,
network connection or files under target/ are required.
"""
import hashlib
import json
import subprocess
import sys
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np

ROOT=Path(__file__).resolve().parents[1]
ASSETS=ROOT/'docs/audio/multichannel/analysis002'
HASHES={
    'music':'0ad7c081dfae503c9499fc984beee3655b1a9555852e98c2f8d912fe7c751759',
    'chirp':'faa9486733ddaf9c04279ec4f3c4aa58760a46ab0d6757f87ce14d2fe98d818e',
}


def main():
    from audio_doa_report import delay
    rng=np.random.default_rng(8)
    reference=rng.normal(size=64000)
    frequencies=np.fft.rfftfreq(len(reference))
    for lag in [-2.25,0,1.375,3,4]:
        delayed=np.fft.irfft(np.fft.rfft(reference)*np.exp(-2j*np.pi*frequencies*lag),n=len(reference))
        assert abs(delay(reference,delayed)[0]['gcc_samples']-lag)<.005
    reports={}
    for kind,digest in HASHES.items():
        directory=ASSETS/kind
        raw=directory/'capture.npz'
        with np.load(raw,allow_pickle=False) as saved:
            pcm=saved['pcm']
            assert int(saved['sample_rate_hz'])==16000
        assert hashlib.sha256(pcm.astype('<i4').tobytes()).hexdigest()==digest
        subprocess.run([sys.executable,str(ROOT/'scripts/audio_doa_report.py'),str(raw),
            '--geometry',str(ASSETS/'geometry.json'),
            '--target-log',str(directory/'capture-target.log'),
            '--output',str(directory)],check=True,stdout=subprocess.DEVNULL)
        reports[kind]=json.loads((directory/'report.json').read_text())
        assert reports[kind]['raw_sha256']==digest
    fig,axes=plt.subplots(1,2,figsize=(12,4.5),constrained_layout=True)
    x=np.arange(1,7)
    colors=['#3b82bd','#d78526']
    for color,(kind,report) in zip(colors,reports.items()):
        pairs=[p for p in report['pairs'] if p['first']==1]
        values=np.array(report['mic1_relative_gcc_us'])
        axes[0].plot(x,values,'o-',color=color,label=f'{kind}: full four seconds')
        for j,p in enumerate(pairs,2):
            axes[0].vlines(j,p['window_p10_us'],p['window_p90_us'],color=color,lw=3,alpha=.45)
        axes[1].plot(x[1:],[p['median_coherence'] for p in pairs],'o-',color=color,label=kind)
    axes[0].set(xlabel='Recorded stream',ylabel='Arrival relative to stream 1 (µs)',xticks=x,
        title='Music versus chirps; bars = window 10th–90th percentiles')
    axes[1].set(xlabel='Recorded stream, paired with stream 1',ylabel='Median magnitude-squared coherence',
        xticks=x[1:],ylim=(0,1.04),title='400–5500 Hz interchannel coherence')
    for ax in axes:
        ax.grid(alpha=.25); ax.legend(fontsize=8)
    fig.savefig(ASSETS/'music-vs-chirp.png',dpi=180); plt.close(fig)
    report=reports['chirp']
    mapping=np.array(report['stream_to_physical_mic'])-1
    physical_delays=np.array(report['mic1_relative_gcc_us'])[np.argsort(mapping)]
    nominal=np.array(report['nominal_source_models']['100mm_from_centroid'])
    fig,axes=plt.subplots(1,2,figsize=(11,4.5),constrained_layout=True)
    axes[0].plot(x,physical_delays,'o-',label='Measured chirps, labels corrected in analysis')
    axes[0].plot(x,nominal,'s--',label='Illustrative coplanar source, 100 mm from centroid')
    axes[0].set(xticks=x,xlabel='Physical PCB microphone MK number',ylabel='Arrival relative to MK1 (µs)',
        title='Physical arrival order; illustrative model is not a calibration')
    axes[0].legend(fontsize=8)
    fit=report['spherical_fits']
    names=['labels_as_recorded','pcb_inferred_MK4_MK5_swap']
    values=[fit[n]['plane_pair_error_rms_us'] for n in names]
    bars=axes[1].bar(['Original labels','Streams 4/5 relabelled'],values,color=['#b94d48','#45865c'])
    axes[1].bar_label(bars,labels=[f'{v:.2f} µs' for v in values],padding=3)
    axes[1].set(ylabel='All-pair RMS timing residual (µs)',title='Best planar gradient fit to all 15 pairs',ylim=(0,100))
    for ax in axes: ax.grid(axis='y',alpha=.25)
    fig.savefig(ASSETS/'geometry-check.png',dpi=180); plt.close(fig)
    print('Verified synthetic signed/fractional delays, both raw-byte SHA-256 values and all target-side summaries; regenerated Analysis 002 figures.')


if __name__=='__main__': main()
