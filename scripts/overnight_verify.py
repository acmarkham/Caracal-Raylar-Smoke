"""Decode representative overnight pairs and retain small failure examples."""
from pathlib import Path
import json
import shutil
import time
from crcl_decode import decode,reference

OUT=Path('analysis/aardwolf_overnight/20261016')
files=json.loads((OUT/'files.json').read_text())
names={'aud_1792088520.flac','aud_1792129620.flac'}
selected=[f for f in files if f['crcl']['end_status']==4 or Path(f['reference']['path']).name in names]
results=[]
for f in selected:
    start=time.monotonic();path=Path(f['reference']['path']);side=Path(f['crcl']['path'])
    try:
        ref=reference(path);pcm,report=decode(side.read_bytes(),ref)
        assert pcm[::6]==ref['pcm']
        result=dict(file=path.name,reference_frames=len(ref['pcm']),mic1_matches=True,**report)
        if f['crcl']['end_status']==4:
            dest=OUT/'failure-examples';dest.mkdir(exist_ok=True)
            shutil.copy2(path,dest/path.name);shutil.copy2(side,dest/side.name)
    except Exception as exc:result=dict(file=path.name,error=str(exc))
    result['elapsed_s']=round(time.monotonic()-start,3);results.append(result)
    print(json.dumps(result),flush=True)
    (OUT/'decode-checks.json').write_text(json.dumps(results,indent=2)+'\n')
assert all('error' not in r for r in results)
