"""Analyze six-microphone delay alignment from an unquantized hardware capture.

Positive delay means the second channel arrives later. Uses a coherence-weighted
GCC-PHAT estimate and a secondary coherent cross-spectrum phase-slope fit.
"""
import argparse
import hashlib
import json
import re
from pathlib import Path

import matplotlib
matplotlib.use("Agg")
import matplotlib.pyplot as plt
import numpy as np
from scipy import signal
from scipy.optimize import least_squares

FS = 16000
SPEED = 343.0


def geometry(pcb):
    microphones = {}
    for chunk in pcb.read_text().split("\n\t(footprint ")[1:]:
        if '(property "Value" "IM69D129FV01"' not in chunk:
            continue
        reference = re.search(r'\(property "Reference" "(MK\d+)"', chunk)[1]
        placement = re.search(r'\(at ([^)]*)\)', chunk)[1].split()
        x, y = map(float, placement[:2])
        rotation = float(placement[2]) if len(placement) > 2 else 0.0
        # Use the acoustic through-hole rather than just the footprint origin.
        hole = re.search(r'\(pad "" np_thru_hole circle\s*\(at ([^)]*)\)', chunk)
        hx, hy = map(float, hole[1].split()[:2]) if hole else (0, 0)
        theta = np.deg2rad(rotation)
        port = [x+np.cos(theta)*hx+np.sin(theta)*hy,
                y-np.sin(theta)*hx+np.cos(theta)*hy]
        pads = chunk.split("\n\t\t(pad ")
        pad_nets = {}
        for pad in pads[1:]:
            number = re.match(r'"([1-5])"', pad)
            net = re.search(r'\(net \d+ "([^"\n]+)"\)', pad)
            if number and net:
                pad_nets[number[1]] = net[1]
        microphones[int(reference[2:])] = dict(reference=reference,
            footprint_xy_mm=[x,y], port_xy_mm=port, rotation_deg=rotation,
            data_net=pad_nets.get("1"), lr_net=pad_nets.get("2"), clock_net=pad_nets.get("4"))
    assert sorted(microphones) == list(range(1,7))
    return [microphones[i] for i in range(1,7)]


def spectra(data, nperseg=8192):
    f, p = signal.welch(data, FS, nperseg=nperseg, noverlap=nperseg//2,
                        window="hann", axis=0, scaling="density")
    return f, p


def delay(x, y, band=(400,5500), nperseg=8192, max_samples=10):
    nperseg = min(nperseg, len(x))
    f, cross = signal.csd(x,y,FS,nperseg=nperseg,noverlap=nperseg//2,window="hann")
    _, px = signal.welch(x,FS,nperseg=nperseg,noverlap=nperseg//2,window="hann")
    _, py = signal.welch(y,FS,nperseg=nperseg,noverlap=nperseg//2,window="hann")
    coherence = np.clip(np.abs(cross)**2/np.maximum(px*py,1e-30),0,1)
    keep = (f>=band[0]) & (f<=band[1])
    # Coherence weighting suppresses independent noise and weak spectral bins.
    weights = coherence**2 * keep
    weights[coherence<.5] = 0
    phase = cross/np.maximum(np.abs(cross),1e-30)
    oversampling = 64
    size = nperseg*oversampling
    cc = np.fft.irfft(phase*weights,n=size)*oversampling
    indices = np.arange(-max_samples*oversampling,max_samples*oversampling+1)
    curve = cc[indices % size]
    peak = int(np.argmax(curve))
    offset = 0.0
    if 0<peak<len(curve)-1:
        left,center,right = curve[peak-1:peak+2]
        offset = .5*(left-right)/(left-2*center+right)
    samples = (indices[peak]+offset)/oversampling
    # Fit cross phase after removing the GCC delay, avoiding wrapped slopes.
    valid = keep & (coherence>=.8)
    phase_corrected = np.angle(cross*np.exp(2j*np.pi*f*samples/FS))
    w = coherence[valid]**2
    if valid.sum()>=8:
        design = np.c_[f[valid],np.ones(valid.sum())]
        coefficients = np.linalg.lstsq(design*np.sqrt(w[:,None]),phase_corrected[valid]*np.sqrt(w),rcond=None)[0]
        fitted = samples/FS-coefficients[0]/(2*np.pi)
        phase_rms = np.sqrt(np.average((phase_corrected[valid]-design@coefficients)**2,weights=w))
        intercept = float(np.rad2deg(coefficients[1]))
    else:
        fitted, phase_rms, intercept = float("nan"),float("nan"),float("nan")
    return dict(gcc_samples=float(samples),gcc_us=float(samples/FS*1e6),
        phase_fit_us=float(fitted*1e6), phase_residual_deg=float(np.rad2deg(phase_rms)),
        phase_intercept_deg=intercept, coherent_bins=int(valid.sum()),
        median_coherence=float(np.median(coherence[keep]))), (indices/oversampling,curve), (f,cross,coherence)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("raw",type=Path)
    source=parser.add_mutually_exclusive_group(required=True)
    source.add_argument("--pcb",type=Path)
    source.add_argument("--geometry",type=Path,help="Archived PCB-derived microphone geometry JSON")
    parser.add_argument("--target-log",type=Path,required=True)
    parser.add_argument("--output",type=Path,required=True)
    args=parser.parse_args()
    if args.raw.suffix=='.npz':
        with np.load(args.raw,allow_pickle=False) as saved:
            pcm=saved['pcm']
            assert int(saved['sample_rate_hz'])==FS
    else:
        pcm=np.fromfile(args.raw,dtype="<i4").reshape(-1,6)
    assert pcm.shape==(64000,6)
    rows=re.findall(r"NOISE channel=(\d+) frames=(\d+) sum=(-?\d+) squares=(\d+) min=(-?\d+) max=(-?\d+)",args.target_log.read_text())
    assert len(rows)==6
    for c,frames,total,squares,minimum,maximum in rows:
        x=pcm[:,int(c)-1].astype(np.int64)
        assert tuple(map(int,(len(x),x.sum(),(x*x).sum(),x.min(),x.max())))==tuple(map(int,(frames,total,squares,minimum,maximum)))
    microphones=geometry(args.pcb) if args.pcb else json.loads(args.geometry.read_text())['microphones']
    xy=np.array([m["port_xy_mm"] for m in microphones])/1000
    center=xy.mean(axis=0)
    # Match current BSMX selection [BS0_R, BS1_R, BS1_F, BS2_R,
    # BS2_F, BS3_R] to PCB data/LR nets. Single-mic buses are unambiguous.
    bus_edges=[("MIC_SD0",None),("MIC_SD1","GND"),("MIC_SD1","3.3V"),
               ("MIC_SD2","GND"),("MIC_SD2","3.3V"),("MIC_SD3",None)]
    mapping=np.array([next(i for i,m in enumerate(microphones)
        if m['data_net']==bus and (edge is None or m['lr_net']==edge))
        for bus,edge in bus_edges])
    f,p=spectra(pcm.astype(float))
    pairs=[]
    curves={}
    spectral={}
    for i in range(6):
        for j in range(i+1,6):
            result,curve,spec=delay(pcm[:,i],pcm[:,j])
            result.update(first=i+1,second=j+1)
            window_delays=[]
            for start in range(0,64000,8000):
                window_delays.append(delay(pcm[start:start+8000,i],pcm[start:start+8000,j],nperseg=2048)[0]["gcc_us"])
            result["half_second_gcc_us"]=window_delays
            result["window_median_us"]=float(np.median(window_delays))
            result["window_p10_us"]=float(np.percentile(window_delays,10))
            result["window_p90_us"]=float(np.percentile(window_delays,90))
            pairs.append(result)
            curves[(i,j)]=curve
            spectral[(i,j)]=spec
    observed=np.array([0]+[r["gcc_us"] for r in pairs if r["first"]==1])/1e6
    # Raw geometry, spherical source 10 cm above array centroid, vs top-front row.
    models={}
    for name,sy in [("100mm_from_centroid",center[1]-.1),("100mm_from_front_row",xy[:,1].min()-.1)]:
        distances=np.linalg.norm(xy-np.array([center[0],sy]),axis=1)
        models[name]=(distances-distances[0])/SPEED*1e6
    # Fit all pair differences, rather than treating MIC1's estimator as exact.
    incidence=np.zeros((15,5)); observations=[]
    for row,r in enumerate(pairs):
        if r['first']>1: incidence[row,r['first']-2]=-1
        incidence[row,r['second']-2]=1
        observations.append(r['gcc_us'])
    delays=np.r_[0,np.linalg.lstsq(incidence,np.array(observations),rcond=None)[0]]
    fit_results={}
    for name,order in [("labels_as_recorded",np.arange(6)),("pcb_inferred_MK4_MK5_swap",mapping)]:
        positions=xy[order]
        def residuals(source):
            distances=np.linalg.norm(positions-np.array(source),axis=1)
            return np.array([(distances[r['second']-1]-distances[r['first']-1])/SPEED*1e6-r['gcc_us'] for r in pairs])
        fit=least_squares(residuals,[center[0],xy[:,1].min()-.10],bounds=([center[0]-.5,xy[:,1].min()-2],[center[0]+.5,xy[:,1].min()-.005]))
        fit_results[name]=dict(source_xy_mm=(fit.x*1000).tolist(),rms_pair_error_us=float(np.sqrt(np.mean(fit.fun**2))),max_pair_error_us=float(np.max(np.abs(fit.fun))))
        # An unconstrained planar gradient is a diagnostic of direction and
        # label consistency, not a calibrated 10 cm source-location estimate.
        design=np.array([positions[r['second']-1]-positions[r['first']-1] for r in pairs])*1000
        gradient=np.linalg.lstsq(design,np.array(observations),rcond=None)[0]
        fit_results[name]['plane_gradient_us_per_mm']=gradient.tolist()
        fit_results[name]['plane_azimuth_degrees_from_top']=float(np.rad2deg(np.arctan2(-gradient[0],gradient[1])))
        fit_results[name]['plane_pair_error_rms_us']=float(np.sqrt(np.mean((design@gradient-observations)**2)))
    quantized=pcm.astype(np.int64).copy()
    shifts=[]
    for start in range(0,len(pcm),1024):
        block=quantized[start:start+1024]
        for c in range(1,6):
            peak=int(np.abs(block[:,c]).max()); s=max(0,peak.bit_length()-9)
            if s and (peak+(1<<(s-1)))>>s>511: s+=1
            s=max(6,s)
            block[:,c]=((block[:,c]+(1<<(s-1)))>>s)<<s
            shifts.append(s)
    quantized_results=[]
    for r in pairs:
        i,j=r['first']-1,r['second']-1
        q=delay(quantized[:,i],quantized[:,j])[0]
        quantized_results.append(dict(first=i+1,second=j+1,gcc_us=q['gcc_us'],difference_us=q['gcc_us']-r['gcc_us']))
    out=args.output; out.mkdir(parents=True,exist_ok=True)
    result=dict(raw_sha256=hashlib.sha256(pcm.astype('<i4').tobytes()).hexdigest(),frames=64000,rate=FS,microphones=microphones,
        source_description="User reports music/chirps towards PCB top at approximately 10 cm; source height and precise location unknown.",
        geometry_model_limit="Plane azimuth is indicative in this near-field setup. Spherical fits assume a coplanar source and must not be interpreted as a calibrated source position.",
        quantization_limit="Same-PCM host simulation of production rounding/peak shifts; not a decode of a card-extracted CRCL recording.",
        sign="positive delay means second stream arrives later",band_hz=[400,5500],pairs=pairs,
        mic1_relative_gcc_us=(observed*1e6).tolist(),all_pair_least_squares_us=delays.tolist(),
        nominal_source_models={k:v.tolist() for k,v in models.items()},spherical_fits=fit_results,
        stream_to_physical_mic=[int(i+1) for i in mapping],quantized_pairs=quantized_results,
        actual_quantizer_shifts=sorted(set(shifts)))
    (out/'report.json').write_text(json.dumps(result,indent=2)+'\n')
    np.savez_compressed(out/'capture.npz',pcm=pcm,sample_rate_hz=np.array(FS))
    fig,axes=plt.subplots(2,1,figsize=(11,8),constrained_layout=True)
    for j in range(1,6):
        lags,curve=curves[(0,j)]
        axes[0].plot(lags*1e6/FS,curve/np.max(curve),label=f'Stream {j+1} relative to 1')
        sf,cross,coh=spectral[(0,j)]; keep=(sf>=400)&(sf<=5500)&(coh>.8)
        expected=observed[j]
        phase=np.angle(cross*np.exp(2j*np.pi*sf*expected))-2*np.pi*sf*expected
        axes[1].plot(sf[keep],np.rad2deg(phase[keep]),'.',ms=1.5,label=f'Stream {j+1}')
    axes[0].set(xlabel='Second stream arrival delay relative to stream 1 (µs)',ylabel='Normalized weighted GCC-PHAT',title='Measured relative delays; positive = later arrival',xlim=(-400,400)); axes[0].legend(fontsize=8)
    axes[1].set(xlabel='Frequency (Hz)',ylabel='Cross-spectrum phase (degrees)',title='Coherent-bin phase branch referenced to the GCC estimate'); axes[1].legend(fontsize=8)
    for ax in axes: ax.grid(alpha=.25)
    fig.savefig(out/'phase-delays.png',dpi=170); plt.close(fig)
    fig,ax=plt.subplots(figsize=(8,7),constrained_layout=True)
    sc=ax.scatter(xy[:,0]*1000,xy[:,1]*1000,c=delays[np.argsort(mapping)],s=140,cmap='viridis')
    for i,(x,y) in enumerate(xy*1000): ax.annotate(f'MK{i+1}\nstream {int(np.where(mapping==i)[0][0])+1}',(x,y),xytext=(8,0),textcoords='offset points')
    ax.invert_yaxis(); ax.set_aspect('equal'); ax.set(xlabel='PCB X (mm)',ylabel='PCB Y (mm)',title='PCB acoustic-port positions; source towards top')
    fig.colorbar(sc,ax=ax,label='Measured arrival relative to stream 1 (µs)')
    fig.savefig(out/'layout.png',dpi=170); plt.close(fig)
    print('Pairs (first,second,GCC us,phase us,coherence,window p10/p90):')
    for r in pairs: print(r['first'],r['second'],*[round(r[k],3) for k in ['gcc_us','phase_fit_us','median_coherence','window_p10_us','window_p90_us']])
    print('MIC1 relative',result['mic1_relative_gcc_us']);print('All-pair LS',delays.tolist()); print('Models',{k:v.tolist() for k,v in models.items()});print('Fits',fit_results)
    print('Quantizer shifts',result['actual_quantizer_shifts'],'max delay change us',max(abs(r['difference_us']) for r in quantized_results))


if __name__=='__main__': main()
