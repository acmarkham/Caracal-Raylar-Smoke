"""Reproducible outdoor six-microphone analysis; see report for assumptions."""
import argparse
import json
import subprocess
from pathlib import Path
import numpy as np
from scipy import signal
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from mic_noise_report import a_weight, integrate

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT/'analysis/aardwolf_multichannel/analysis20261015'
FS = 16000
CALLS = [24.,50.1,51.3,52.5,53.5]
ACTIVE = [(23.97,24.18),(50.05,50.22),(51.24,51.44),(52.49,52.70),(53.48,53.72)]

def noise_analysis(x):
    pillow=np.load(ROOT/'docs/audio/multichannel/analysis001/pillow-20261015.npz')['pcm']
    # Match analysis001 Welch bandwidth, integer scale, and integration endpoints.
    f,p=signal.welch(pillow.astype(float),FS,nperseg=8192,noverlap=4096,axis=0)
    _,o=signal.welch(x.astype(float),FS,nperseg=8192,noverlap=4096,axis=0)
    windows=[]; window_psd=[]
    for start in np.arange(0,56.001,2):
        _,w=signal.welch(x[int(start*FS):int((start+4)*FS),0].astype(float),FS,nperseg=8192,noverlap=4096)
        windows.append((float(start),np.sqrt(integrate(f,w*a_weight(f),20,7100))))
        window_psd.append(w)
    quiet_index=int(np.argmin(np.array(windows)[:,1])); quiet=window_psd[quiet_index]
    curves={'Pillow MIC1 (4 s)':p[:,0], 'Outdoor MIC1 (60 s)':o[:,0],
            'Outdoor MIC1 (quietest 4 s)':quiet}
    bands=[(20,100),(100,300),(300,1000),(1000,5000),(5000,7100),(20,7100)]
    rows={}
    for name,psd in curves.items():
        rows[name]={f'{lo}-{hi}':float(np.sqrt(integrate(f,psd,lo,hi))) for lo,hi in bands}
        rows[name]['A_20-7100']=float(np.sqrt(integrate(f,psd*a_weight(f),20,7100)))
        rows[name]['below100_power_percent']=100*integrate(f,psd,20,100)/integrate(f,psd,20,7100)
    fig,axes=plt.subplots(2,1,figsize=(12,9),constrained_layout=True)
    for name,psd in curves.items():
        axes[0].semilogx(f[1:],10*np.log10(np.maximum(psd[1:],1e-12)),label=name,alpha=.85)
    for c in range(6):axes[1].semilogx(f[1:],10*np.log10(np.maximum(o[1:,c],1e-12)),label=f'MIC{c+1}',alpha=.8)
    for a in axes:
        a.set(xlim=(20,8000),xlabel='Hz',ylabel='PSD (dB re 1 count²/Hz)');a.grid(alpha=.2);a.legend()
    axes[0].set_title('Same integer units and Welch settings: environmental spectra, not isolated self-noise')
    axes[1].set_title('Outdoor whole-recording spectra: MIC1 lossless, MIC2–6 reconstructed')
    fig.savefig(OUT/'noise-comparison.png',dpi=160);plt.close(fig)
    np.savetxt(OUT/'noise-spectrum.csv',np.c_[f,p[:,0],o,quiet],delimiter=',',
               header='Hz,pillow_MIC1,'+','.join(f'outdoor_MIC{i+1}' for i in range(6))+',quiet_MIC1',comments='')
    return dict(rows=rows,quiet_start_s=windows[quiet_index][0],quiet_end_s=windows[quiet_index][0]+4,
                quiet_selection='Minimum MIC1 A-weighted 20–7100 Hz RMS in 4 s windows stepped by 2 s',
                window_a_rms=windows)

def geometry():
    g=json.loads((ROOT/'docs/audio/multichannel/analysis002/geometry.json').read_text())
    r=np.array([v['port_xy_mm'] for v in g['microphones']])/1000
    return r-r[0]

def scan_setup(f,r):
    az=np.arange(-180,180,1.)
    q=np.arange(.10,1.0001,.025)
    aa,qq=np.meshgrid(az,q)
    dirs=np.c_[np.sin(np.deg2rad(aa.ravel())),-np.cos(np.deg2rad(aa.ravel()))]*qq.ravel()[:,None]
    tau=-dirs@r.T/343.
    pairs=[(i,j) for i in range(6) for j in range(i+1,6)]
    # conj(a_i)*a_j with a=exp(-j 2 pi f tau).
    steer=np.exp(2j*np.pi*f[None,:,None]*np.array([tau[:,i]-tau[:,j] for i,j in pairs]).T[:,None,:])
    return az,q,tau,pairs,steer.astype(np.complex64)

def event_mask(f,t,z,active,center):
    on=(t>=active[0])&(t<=active[1])
    off=((t>=center-.8)&(t<=center-.25))|((t>=center+.4)&(t<=center+.8))
    for lo,hi in ACTIVE:off&=~((t>=lo-.07)&(t<=hi+.07))
    noise=np.mean(np.abs(z[:,off,0])**2,axis=1)
    p=np.abs(z[:,:,0])**2
    ratio=p/np.maximum(noise[:,None],1e-12)
    mask=np.sqrt(np.maximum(p-noise[:,None],0))*np.clip(1-2/np.maximum(ratio,1e-12),0,1)
    mask*=on[None,:]&((f>=1200)&(f<=5000))[:,None]
    return mask,off,on,noise

def doa(z,f,t,active,center,r,setup):
    az,q,tau,pairs,steer=setup
    mask,off,on,noise=event_mask(f,t,z,active,center)
    # Pair phase transform, weighted by bird excess and local reference SNR.
    zz=z/np.maximum(np.abs(z),1e-12)
    cross=np.stack([zz[:,:,i]*zz[:,:,j].conj() for i,j in pairs],axis=-1)
    c=np.einsum('ft,ftp->fp',mask,cross)/(mask.sum()+1e-30)
    scores=np.real(np.einsum('gfp,fp->g',steer,c,optimize=True))/len(pairs)
    best=int(np.argmax(scores)); qi,ai=np.unravel_index(best,(len(q),len(az)))
    # Test mapping hypothesis against identical observations, without changing audio order.
    other=r[[0,1,2,4,3,5]]
    alt_setup=scan_setup(f,other)
    alt=np.real(np.einsum('gfp,fp->g',alt_setup[-1],c,optimize=True))/len(pairs)
    del alt_setup
    # Sensitivity to early/late halves and frequency bands. These are not CIs.
    sensitivity=[]
    for name,subset in [('early',t<(active[0]+active[1])/2),('late',t>=(active[0]+active[1])/2),
                       ('1200-2500Hz',(f<2500)[:,None]),('2500-5000Hz',(f>=2500)[:,None])]:
        m=mask*subset
        cc=np.einsum('ft,ftp->fp',m,cross)/(m.sum()+1e-30)
        s=np.real(np.einsum('gfp,fp->g',steer,cc,optimize=True))/len(pairs)
        idx=int(np.argmax(s)); iq,ia=np.unravel_index(idx,(len(q),len(az)))
        sensitivity.append(dict(subset=name,azimuth_deg=float(az[ia]),horizontal_projection=float(q[iq]),score=float(s[idx]),weight_fraction=float(m.sum()/mask.sum())))
    profile=scores.reshape(len(q),len(az)).max(axis=0)
    # Strongest alternative separated by >=30 degrees from the main azimuth.
    separate=np.abs((az-az[ai]+180)%360-180)>=30
    secondary=int(np.argmax(np.where(separate,profile,-np.inf)))
    selected=np.where(mask.sum(axis=1)>0)[0]
    phase=np.angle(c[selected]); predicted=np.angle(np.conj(steer[best,selected]))
    weights=mask.sum(axis=1)[selected]
    phase_error=np.angle(np.exp(1j*(phase-predicted)))
    rms=np.sqrt(np.sum(weights[:,None]*phase_error**2)/(weights.sum()*len(pairs)))
    result=dict(center_s=center,active_s=active,azimuth_deg=float(az[ai]),horizontal_projection=float(q[qi]),
        apparent_abs_elevation_deg=float(np.rad2deg(np.arccos(q[qi]))),score=float(scores[best]),
        legacy_mapping_best_score=float(alt.max()),secondary_azimuth_deg=float(az[secondary]),
        secondary_score=float(profile[secondary]),phase_model_rms_deg=float(np.rad2deg(rms)),
        sensitivity=sensitivity, selected_frequency_bins=len(selected))
    return result,tau[best],mask,off,on,profile,scores.reshape(len(q),len(az))

def export_flac(path,y,gain):
    """24-bit listening PCM; one common gain maps recorder counts to float FS."""
    import soundfile as sf
    scaled=np.asarray(y)*gain
    assert np.isfinite(scaled).all() and np.abs(scaled).max()<1
    sf.write(str(path),scaled,FS,subtype='PCM_24')
    decoded,rate=sf.read(str(path),always_2d=True)
    expected=scaled[:,None] if scaled.ndim==1 else scaled
    assert rate==FS and decoded.shape==expected.shape
    assert np.max(np.abs(decoded-expected))<=2**-23+1e-10
    return float(np.abs(scaled).max())

def beamform(x,result,tau):
    center=result['center_s']; start=center-.45; end=center+.70
    # 512 sample STFT, 75% overlap. Local covariance excludes the target call.
    lo=max(0,center-1);hi=min(len(x)/FS,center+1)
    segment=x[int(lo*FS):int(hi*FS)].astype(float)
    f,tt,z0=signal.stft(segment,FS,nperseg=512,noverlap=384,axis=0)
    z=z0.transpose(0,2,1); t=tt+lo
    _,off,on,_=event_mask(f,t,z,result['active_s'],center)
    rn=np.einsum('fti,ftj->fij',z[:,off],z[:,off].conj())/off.sum()
    a=np.exp(-2j*np.pi*f[:,None]*tau[None,:])
    trace=np.trace(rn,axis1=1,axis2=2).real/6
    # 10% diagonal loading, increased until >=0 dB white-noise gain.
    loading=np.full(len(f),.1)
    for _ in range(8):
        inva=np.linalg.solve(rn+(loading*trace+1e-9)[:,None,None]*np.eye(6),a[...,None])[...,0]
        w=inva/np.einsum('fi,fi->f',a.conj(),inva)[:,None]
        bad=np.sum(np.abs(w)**2,axis=1)>1
        loading[bad]*=2
    assert np.max(np.abs(np.einsum('fi,fi->f',w.conj(),a)-1))<1e-8
    das=a/6
    spectra={'mic1':z[:,:,0], 'das':np.einsum('fi,fti->ft',das.conj(),z),
             'mvdr':np.einsum('fi,fti->ft',w.conj(),z)}
    outputs={}
    for name,zz in spectra.items():
        _,y=signal.istft(zz,FS,nperseg=512,noverlap=384)
        outputs[name]=y[int(round((start-lo)*FS)):int(round((end-lo)*FS))]
    # Raw MIC1 from exact samples: test STFT reconstruction before replacing.
    raw=x[int(round(start*FS)):int(round(end*FS)),0].astype(float)
    assert np.max(np.abs(raw-outputs['mic1']))<1e-5
    outputs['mic1']=raw
    outputs['sixch']=x[int(round(start*FS)):int(round(end*FS))].astype(float)
    # Identical zero-phase filter on the three comparison variants.
    sos=signal.butter(4,[1000,5000],btype='bandpass',fs=FS,output='sos')
    for name in ['mic1','das','mvdr']:
        outputs[name+'_band']=signal.sosfiltfilt(sos,outputs[name])
    band=(f>=1000)&(f<=5000)
    metrics={}
    for name,zz in spectra.items():
        noise=float(np.mean(np.abs(zz[band][:,off])**2))
        total=float(np.mean(np.abs(zz[band][:,on])**2))
        metrics[name]=dict(noise_power=noise,call_power=total,
            excess_to_noise_db=float(10*np.log10(max(total-noise,1e-30)/noise)))
    result.update(clip_s=[start,end],beam_metrics=metrics,
                  max_weight_norm2=float(np.max(np.sum(np.abs(w)**2,axis=1))),
                  loading_range= [float(loading.min()),float(loading.max())])
    return outputs

def quantizer_proxy(x,decode_report):
    """Apply the production peak/10-bit/min-six-bit rule to lossless MIC1.

    This is an A/B proxy on identical data, not the error on other microphones.
    """
    raw=x[:,0].astype(np.int64);q=np.empty_like(raw);shifts=[]
    for rec in decode_report['records']:
        lo=rec['frame'];hi=lo+rec['count'];v=raw[lo:hi];peak=int(np.abs(v).max())
        shift=max(0,peak.bit_length()-9)
        if shift and (peak+(1<<(shift-1)))>>shift>511:shift+=1
        shift=max(shift,6);shifts.append(shift)
        q[lo:hi]=((v+(1<<(shift-1)))>>shift)<<shift
    error=q-raw
    f,p=signal.welch(error.astype(float),FS,nperseg=8192,noverlap=4096)
    _,orig=signal.welch(raw.astype(float),FS,nperseg=8192,noverlap=4096)
    snr=10*np.log10(integrate(f,orig,1000,5000)/integrate(f,p,1000,5000))
    # Compare against measured pillow quantization justification without assuming
    # error is white when variable shifts follow wind/transient peaks.
    details=dict(rms_error_counts=float(np.sqrt(np.mean(error.astype(float)**2))),
                 max_abs_error_counts=int(np.abs(error).max()),
                 error_rms_1000_5000=float(np.sqrt(integrate(f,p,1000,5000))),
                 signal_to_error_1000_5000_db=float(snr),
                 shifts=dict(zip(*[a.tolist() for a in np.unique(shifts,return_counts=True)])))
    fig,axes=plt.subplots(3,1,figsize=(13,10),constrained_layout=True)
    times=np.array([rec['frame']/FS for rec in decode_report['records']])
    for c in range(5):axes[0].step(times,[rec['shifts'][c] for rec in decode_report['records']],where='post',label=f'MIC{c+2}',alpha=.6)
    axes[0].set(ylabel='Bits discarded',title='Actual CRCL block shifts: low-frequency peaks also set bird-band precision');axes[0].legend(ncol=5)
    for c in CALLS:axes[0].axvline(c,color='black',alpha=.3,ls='--')
    axes[1].plot(np.arange(len(raw))[::160]/FS,raw[::160],lw=.5)
    axes[1].set(ylabel='MIC1 counts',xlabel='Seconds',title='Reference waveform (decimated for display only)')
    axes[2].semilogx(f[1:],10*np.log10(np.maximum(orig[1:],1e-12)),label='MIC1 original')
    axes[2].semilogx(f[1:],10*np.log10(np.maximum(p[1:],1e-12)),label='Simulated MIC1 codec error')
    axes[2].set(xlim=(20,8000),xlabel='Hz',ylabel='PSD (dB re 1 count²/Hz)');axes[2].legend()
    for ax in axes:ax.grid(alpha=.2)
    fig.savefig(OUT/'quantization.png',dpi=150);plt.close(fig)
    return q,details

def validate_steering():
    # An analytic delayed plane wave checks azimuth sign, channel convention,
    # scan grid, and recovery of a nonhorizontal slowness vector.
    f=np.arange(1250,5001,250.);r=geometry();setup=scan_setup(f,r)
    theta=np.deg2rad(37);direction=np.array([np.sin(theta),-np.cos(theta)])*.8
    tau=-r@direction/343.;a=np.exp(-2j*np.pi*f[:,None]*tau)
    c=np.stack([a[:,i]*a[:,j].conj() for i,j in setup[3]],axis=-1)
    scores=np.real(np.einsum('gfp,fp->g',setup[-1],c))
    iq,ia=np.unravel_index(np.argmax(scores),(len(setup[1]),len(setup[0])))
    assert setup[0][ia]==37 and abs(setup[1][iq]-.8)<1e-8
    return dict(known_azimuth_deg=37,recovered_azimuth_deg=float(setup[0][ia]),
                known_q=.8,recovered_q=float(setup[1][iq]))

def full():
    x=np.load(OUT/'decoded_counts.npy')
    noise=noise_analysis(x)
    r=geometry()
    f,t,z0=signal.stft(x.astype(float),FS,nperseg=512,noverlap=384,axis=0)
    z=z0.transpose(0,2,1)
    # Restrict scans to 1.2–5 kHz to avoid allocating steering for unused bins.
    keep=(f>=1200)&(f<=5000); ff=f[keep]
    setup=scan_setup(ff,r)
    events=[];audio=[];profiles=[];masks=[];maps=[]
    decode_report=json.loads((OUT/'decode.json').read_text())
    proxy,proxy_metrics=quantizer_proxy(x,decode_report)
    for center,active in zip(CALLS,ACTIVE):
        # Limit cross-spectral arrays to local frames for speed and memory.
        use=(t>=center-1)&(t<=center+1)
        result,tau,mask,off,on,profile,grid=doa(z[keep][:,use],ff,t[use],active,center,r,setup)
        outputs=beamform(x,result,tau)
        start,end=result['clip_s'];sl=slice(int(round(start*FS)),int(round(end*FS)))
        outputs['mic1_quantized_proxy']=proxy[sl].astype(float)
        sos=signal.butter(4,[1000,5000],btype='bandpass',fs=FS,output='sos')
        outputs['mic1_quantized_proxy_band']=signal.sosfiltfilt(sos,outputs['mic1_quantized_proxy'])
        event_error=outputs['mic1_quantized_proxy_band']-outputs['mic1_band']
        result['proxy_signal_to_error_band_db']=float(10*np.log10(np.sum(outputs['mic1_band']**2)/np.sum(event_error**2)))
        if center in (24.,53.5):
            theta=np.deg2rad(result['secondary_azimuth_deg'])
            direction=np.array([np.sin(theta),-np.cos(theta)])*result['horizontal_projection']
            alt=dict(result);alt_outputs=beamform(x,alt,-r@direction/343.)
            outputs['mvdr_alternative']=alt_outputs['mvdr']
            outputs['mvdr_alternative_band']=alt_outputs['mvdr_band']
            result['alternative_mvdr_azimuth_deg']=result['secondary_azimuth_deg']
            result['alternative_mvdr_metrics']=alt['beam_metrics']['mvdr']
        events.append(result);audio.append(outputs);profiles.append(profile);maps.append(grid)
        masks.append((ff,t[use],mask))
        print(f"Call {center}: azimuth {result['azimuth_deg']:+.0f}; phase score {result['score']:.3f}",flush=True)
    # Gain is fixed across channels, algorithms and all five calls within each
    # comparison group. Filtered comparisons get more gain for easy listening.
    gain=.9/float(np.max(np.abs(x)))
    clip_gain=.9/max(float(np.max(np.abs(y))) for d in audio for name,y in d.items() if not name.endswith('_band'))
    band_gain=.9/max(float(np.max(np.abs(y))) for d in audio for name,y in d.items() if name.endswith('_band'))
    audio_dir=OUT/'listening';audio_dir.mkdir(exist_ok=True)
    export_flac(audio_dir/'outdoor_6ch_playback.flac',x,gain)
    for c in range(6):export_flac(audio_dir/f'outdoor_MIC{c+1}.flac',x[:,c],gain)
    export_flac(audio_dir/'outdoor_MIC1_quantized_proxy.flac',proxy,gain)
    for event,outputs in zip(events,audio):
        stem=f"call_{event['center_s']:04.1f}s"
        event['audio']={}
        for name,y in outputs.items():
            path=audio_dir/f'{stem}_{name}.flac'
            export_flac(path,y,band_gain if name.endswith('_band') else clip_gain)
            event['audio'][name]=str(path.relative_to(OUT)).replace('\\','/')
    fig,axes=plt.subplots(5,2,figsize=(14,15),constrained_layout=True)
    for i,(event,profile,grid) in enumerate(zip(events,profiles,maps)):
        a=axes[i,0];a.plot(setup[0],profile);a.axvline(event['azimuth_deg'],color='r',ls='--')
        a.set(xlim=(-180,180),xlabel='Board azimuth (degrees)',ylabel='Weighted phase score',title=f"{event['center_s']:.1f}s: {event['azimuth_deg']:+.0f}°, q={event['horizontal_projection']:.3f}");a.grid(alpha=.2)
        a=axes[i,1];im=a.pcolormesh(setup[0],setup[1],grid,shading='auto',cmap='viridis')
        a.plot(event['azimuth_deg'],event['horizontal_projection'],'rx');a.set(xlabel='Board azimuth (degrees)',ylabel='Horizontal projection q');fig.colorbar(im,ax=a)
    fig.savefig(OUT/'doa-scans.png',dpi=150);plt.close(fig)
    fig,axes=plt.subplots(5,1,figsize=(12,13),constrained_layout=True)
    for ax,(ff,tt,m),event in zip(axes,masks,events):
        ax.pcolormesh(tt,ff,m,shading='auto',cmap='magma');ax.set(xlim=(event['center_s']-.3,event['center_s']+.4),ylabel='Hz',title=f"{event['center_s']:.1f}s: target excess weights for AoA")
    axes[-1].set_xlabel('Seconds');fig.savefig(OUT/'doa-masks.png',dpi=150);plt.close(fig)
    fig,axes=plt.subplots(5,3,figsize=(15,13),constrained_layout=True)
    for row,(event,outputs) in enumerate(zip(events,audio)):
        for col,name in enumerate(['mic1','das','mvdr']):
            f0,t0,z1=signal.stft(outputs[name],FS,nperseg=512,noverlap=384)
            axes[row,col].pcolormesh(t0+event['clip_s'][0],f0,20*np.log10(np.maximum(np.abs(z1),1e-9)),vmin=15,vmax=62,shading='auto',cmap='magma')
            axes[row,col].set(ylim=(1000,5000),title=f"{event['center_s']:.1f}s {name}",xlabel='Seconds',ylabel='Hz')
    fig.savefig(OUT/'beamforming-comparison.png',dpi=150);plt.close(fig)
    result=dict(sample_rate_hz=FS,frames=len(x),mapping='post-fix physical MIC1..MIC6 (confirmed by user)',
                noise=noise,events=events,playback_gain_per_count=gain,
                playback_gain_db_relative_to_original_flac32=float(20*np.log10(gain*2**31)),
                clip_gain_per_count=clip_gain,band_clip_gain_per_count=band_gain,
                mic1_quantizer_proxy=proxy_metrics,steering_self_check=validate_steering(),listening_pcm_bits=24)
    (OUT/'analysis.json').write_text(json.dumps(result,indent=2)+'\n',encoding='utf8')
    print('gain',gain,'noise',json.dumps(noise['rows']),flush=True)

def overview():
    x = np.load(OUT/'decoded_counts.npy')
    f,t,z=signal.stft(x[:,0].astype(float),FS,nperseg=512,noverlap=384)
    db=20*np.log10(np.maximum(np.abs(z),1e-9))
    fig,ax=plt.subplots(2,1,figsize=(15,8),constrained_layout=True)
    for a,lim in zip(ax,[(0,60),(22.8,54.5)]):
        im=a.pcolormesh(t,f,db,shading='auto',vmin=15,vmax=65,cmap='magma')
        a.set(xlim=lim,ylim=(500,6000),xlabel='Seconds into recording',ylabel='Hz')
        for c in CALLS:a.axvline(c,color='cyan',alpha=.7,ls='--')
    fig.colorbar(im,ax=ax,label='STFT magnitude (dB re 1 count)')
    fig.savefig(OUT/'overview.png',dpi=160);plt.close(fig)
    fig,axes=plt.subplots(5,1,figsize=(12,13),constrained_layout=True)
    for a,c in zip(axes,CALLS):
        a.pcolormesh(t,f,db,shading='auto',vmin=15,vmax=62,cmap='magma')
        a.set(xlim=(c-.5,c+.7),ylim=(1000,5000),ylabel='Hz',title=f'Call near {c:.1f}s')
        a.axvline(c,color='cyan',ls='--')
    axes[-1].set_xlabel('Seconds into recording')
    fig.savefig(OUT/'calls-spectrogram.png',dpi=160);plt.close(fig)

if __name__=='__main__':
    parser=argparse.ArgumentParser();parser.add_argument('--overview',action='store_true');args=parser.parse_args()
    if args.overview:overview()
    else:full()
