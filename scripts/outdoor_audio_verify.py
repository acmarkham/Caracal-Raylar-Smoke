"""Verify audio exports, source identity and a known-source DOA simulation."""
import datetime
import hashlib
import json
import re
import subprocess
from pathlib import Path
import numpy as np
import soundfile as sf
from scipy import signal
from outdoor_audio_analysis import OUT,ROOT,FS,geometry,scan_setup,doa

def synthetic_check():
    rng=np.random.default_rng(20261015)
    r=geometry();theta=np.deg2rad(37.)
    # Independent finite-distance propagation in 3D; +37° azimuth, q=.8.
    source=np.array([8*np.sin(theta),-8*np.cos(theta),6.])
    xyz=np.c_[r,np.zeros(6)]
    distance=np.linalg.norm(xyz-source,axis=1)
    delay=(distance-distance[0])/343.
    t=np.arange(2*FS)/FS;local=t-.97
    waveform=signal.chirp(local,1300,.21,4800)*np.where((local>=0)&(local<=.21),np.sin(np.pi*np.clip(local/.21,0,1))**2,0)
    freq=np.fft.rfftfreq(len(t),1/FS)
    raw=np.column_stack([np.fft.irfft(np.fft.rfft(waveform)*np.exp(-2j*np.pi*freq*dt),n=len(t)) for dt in delay])
    raw=raw*3000+rng.normal(0,30,raw.shape)
    f,tt,z0=signal.stft(raw,FS,nperseg=512,noverlap=384,axis=0)
    keep=(f>=1200)&(f<=5000);ff=f[keep]
    result=doa(z0[keep].transpose(0,2,1),ff,tt+23,(23.97,24.18),24,r,scan_setup(ff,r))[0]
    assert abs(result['azimuth_deg']-37)<=2
    assert abs(result['horizontal_projection']-.8)<=.05
    return dict(method='10 m finite-distance 3D chirp, fractional propagation delays, additive independent noise',
                expected_azimuth_deg=37,expected_horizontal_projection=.8,
                recovered_azimuth_deg=result['azimuth_deg'],recovered_horizontal_projection=result['horizontal_projection'],score=result['score'])

def main():
    a=json.loads((OUT/'analysis.json').read_text());d=json.loads((OUT/'decode.json').read_text())
    x=np.load(OUT/'decoded_counts.npy');assert x.shape==(960001,6)
    for name,expected in d['source_sha256'].items():assert hashlib.sha256((OUT/name).read_bytes()).hexdigest()==expected
    def ffdecode(path):
        return np.frombuffer(subprocess.run(['ffmpeg','-v','error','-i',str(path),'-f','s32le','-'],check=True,capture_output=True).stdout,dtype='<i4')
    assert np.array_equal(ffdecode(OUT/'aud_1792065540.flac'),x[:,0])
    assert np.array_equal(ffdecode(OUT/'aud_1792065540_6ch.flac').reshape(-1,6),x)
    # Validate MIC1/gain and all stream lengths, bit depths, finite/nonclipped PCM.
    verified=[]
    for path in sorted((OUT/'listening').glob('*.flac')):
        y,fs=sf.read(path,always_2d=True);info=sf.info(path)
        assert fs==FS and info.subtype=='PCM_24' and np.isfinite(y).all() and np.abs(y).max()<1
        expected_channels=6 if '6ch' in path.name or 'sixch' in path.name else 1
        assert y.shape[1]==expected_channels
        if path.name.startswith('outdoor_'):assert len(y)==len(x)
        else:assert len(y)==18400
        if path.name=='outdoor_MIC1.flac':assert np.max(np.abs(y[:,0]-x[:,0]*a['playback_gain_per_count']))<=2**-23+1e-10
        verified.append(dict(file=str(path.relative_to(OUT)).replace('\\','/'),frames=len(y),channels=y.shape[1],peak=float(np.abs(y).max())))
    for target in re.findall(r'\]\(([^)]+)\)',(OUT/'report.md').read_text(encoding='utf8')):
        if target=='manifest.json' or target.startswith(('https:','#')):continue
        assert (OUT/target.split('#')[0]).exists(),target
    synthetic=synthetic_check()
    results=dict(source_hashes_verified=True,archival_six_channel_pcm_exact=True,mic1_reference_pcm_exact=True,
                 listening_files_checked=len(verified),clip_frames=18400,synthetic_source_check=synthetic,
                 source_start_utc=datetime.datetime.fromtimestamp(1792065540,datetime.timezone.utc).isoformat(),audio=verified)
    (OUT/'verification.json').write_text(json.dumps(results,indent=2)+'\n')
    manifest={}
    for path in sorted(OUT.rglob('*')):
        if path.is_file() and path.name!='manifest.json':
            manifest[str(path.relative_to(OUT)).replace('\\','/')]=dict(bytes=path.stat().st_size,sha256=hashlib.sha256(path.read_bytes()).hexdigest())
    code={str(p.relative_to(ROOT)).replace('\\','/'):hashlib.sha256(p.read_bytes()).hexdigest() for p in
          [ROOT/'scripts'/name for name in ['outdoor_audio_probe.py','outdoor_audio_analysis.py','outdoor_audio_report.py','outdoor_audio_verify.py','crcl_decode.py','mic_noise_report.py']]}
    (OUT/'manifest.json').write_text(json.dumps(dict(artifacts=manifest,reproduction_sources_sha256=code),indent=2)+'\n')
    print(json.dumps({k:v for k,v in results.items() if k!='audio'},indent=2))
    print('Total artifact bytes:',sum(v['bytes'] for v in manifest.values()))

if __name__=='__main__':main()
