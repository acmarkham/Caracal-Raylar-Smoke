"""Inspect and strictly decode the outdoor capture; preserve sources in workspace."""
import hashlib
import argparse
import json
from pathlib import Path
import shutil
import struct
import subprocess
import numpy as np
from crcl_decode import reference, decode, number, tlvs

OUT = Path('analysis/aardwolf_multichannel/analysis20261015')
OUT.mkdir(parents=True, exist_ok=True)
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--source',type=Path,default=Path('E:/1792062000'))
args=parser.parse_args()
for suffix in ['flac', 'CRCL']:
    src = args.source / ('aud_1792065540.' + suffix)
    dst = OUT / src.name
    if src.resolve()!=dst.resolve():shutil.copy2(src, dst)
ref = reference(OUT / 'aud_1792065540.flac')
data = (OUT / 'aud_1792065540.CRCL').read_bytes()
meta = tlvs(data[92:number(data, 10, 'H')])
print('metadata', {k: v.decode('utf8', errors='backslashreplace') for k,v in meta.items() if k in (3,5,7,10)})
pcm, report = decode(data, ref)
x = np.asarray(pcm, dtype=np.int32).reshape(-1,6)
assert np.array_equal(x[:,0], ref['pcm'])
np.save(OUT / 'decoded_counts.npy', x)
records = []
offset = number(data, 10, 'H')
while offset < len(data):
    size = number(data, offset+8, 'I')
    if data[offset+4] == 1:
        desc = [struct.unpack_from('<BBBBII', data, offset+28+c*12) for c in range(5)]
        records.append(dict(frame=number(data,offset+16,'Q'), count=number(data,offset+24,'H'),
                            shifts=[d[4] for d in desc], rice_k=[d[3] for d in desc], payload=[d[5] for d in desc]))
    offset += size
report.update(metadata={str(k): (v.decode('utf8') if k in (3,5,7,10) else v.hex()) for k,v in meta.items()},
    reference_comments=ref['metadata'].decode('utf8', errors='replace'),
    block_frames=number(data,40,'H'), peak_counts=np.abs(x.astype(np.int64)).max(axis=0).tolist(),
    source_sha256={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in
                   [OUT/'aud_1792065540.flac',OUT/'aud_1792065540.CRCL']}, records=records)
(OUT/'decode.json').write_text(json.dumps(report,indent=2)+'\n')
# FFmpeg FLAC 32-bit requires experimental compliance. Round-trip below is mandatory.
dst = OUT/'aud_1792065540_6ch.flac'
subprocess.run(['ffmpeg','-v','error','-y','-f','s32le','-ar','16000','-ac','6','-i','pipe:0',
                '-c:a','flac','-strict','experimental','-sample_fmt','s32','-bits_per_raw_sample','32',
                '-metadata','comment=Channels are recorder streams 1..6; original integer PCM scale; not surround audio',str(dst)],
               input=x.astype('<i4').tobytes(),check=True)
raw = subprocess.run(['ffmpeg','-v','error','-i',str(dst),'-f','s32le','-'],capture_output=True,check=True).stdout
assert raw == x.astype('<i4').tobytes(), 'FLAC export altered PCM'
print(json.dumps({k:v for k,v in report.items() if k not in ('records','reference_comments','metadata')},indent=2))
print('shifts', [np.unique([r['shifts'][c] for r in records],return_counts=True) for c in range(5)])
print('FLAC32 six-channel round trip: exact')
