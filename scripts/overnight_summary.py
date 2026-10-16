"""Condensed evidence, rotation latency and GPS correlation for the overnight run."""
from pathlib import Path
import collections
import csv
import datetime as dt
import json
import re
import struct
from zoneinfo import ZoneInfo
from crcl_decode import number,tlvs

OUT=Path('analysis/aardwolf_overnight/20261016')
files=json.loads((OUT/'files.json').read_text());log=json.loads((OUT/'parsed-log.json').read_text());rows=log['rows']
rotations=[r for r in rows if r['message'].startswith('audio rotated')]
failures=[r for r in rows if r['message'].startswith('audio discontinuity')]
starts=[r for r in rows if r['message'].startswith('audio started')]
stops=[r for r in rows if r['message'].startswith('audio stopped')]
boundaries=[r for r in files if r['crcl']['end_status']==1]
assert len(rotations)==len(boundaries),(len(rotations),len(boundaries))
assert all(r['sequence']==i for i,r in enumerate(rows))
for r in files:
    with Path(r['crcl']['path']).open('rb') as f:
        data=f.read(4096)
    meta=tlvs(data[92:number(data,10,'H')]);origin,ticks=struct.unpack('<QQ',meta[1])
    utc,uncertainty=struct.unpack_from('<QQ',meta[2])
    r.update(capture_origin=origin,sample_ticks=ticks,sample_utc_us=utc,uncertainty_us=uncertainty)
timeline=[]
for f,r in zip(boundaries,rotations):
    predicted_end=f['sample_ticks']/1e6+f['crcl']['frames']/16000
    timeline.append(dict(file=Path(f['reference']['path']).name,intended_start=f['uk_start'],logged_rotation=r['uk_estimate'],
                         rotation_line=r['line'],frames=f['crcl']['frames'],source_end_to_wall_delay_s=r['seconds']-predicted_end))
with (OUT/'rotation-timeline.csv').open('w',newline='') as out:
    w=csv.DictWriter(out,fieldnames=timeline[0].keys());w.writeheader();w.writerows(timeline)
incidents=[]
for fail in failures:
    previous_rotation=next((r for r in reversed(rotations) if r['seconds']<fail['seconds']),None)
    next_start=next((r for r in starts if r['seconds']>fail['seconds']),None)
    previous_start=next(r for r in reversed(starts) if r['seconds']<fail['seconds'])
    candidates=[f for f in files if f['crcl']['end_status']==4 and previous_start['seconds']-.2<=f['sample_ticks']/1e6<fail['seconds']]
    # Finalized discontinuity prefixes remain in acquisition order; pair separately below.
    incidents.append(dict(log_line=fail['line'],failure_bst=fail['uk_estimate'],failure_ticks_s=fail['seconds'],
        dropped_scalars=int(re.search(r'dropped=(\d+)',fail['message'])[1]),
        restart_bst=next_start['uk_estimate'] if next_start else None,
        restart_wait_s=next_start['seconds']-fail['seconds'] if next_start else None,
        since_previous_rotation_s=fail['seconds']-previous_rotation['seconds'] if previous_rotation else None))
partials=[f for f in files if f['crcl']['end_status']==4]
# One capture-origin change closes no file: identify which failure has no file by matching causal order.
# Gap/restart at 03:46 occurred while seeking the 03:33 intended segment.
file_i=0
for incident in incidents:
    if incident['dropped_scalars']==9600 and '03:46:' in incident['failure_bst']:
        incident['file']=None;continue
    f=partials[file_i];file_i+=1
    incident.update(file=Path(f['reference']['path']).name,intended_start=f['uk_start'],frames=f['crcl']['frames'],
                    duration_s=f['crcl']['frames']/16000,synthetic_first_sample_s=f['sample_ticks']/1e6)
assert file_i==len(partials)
(OUT/'incidents.json').write_text(json.dumps(incidents,indent=2)+'\n')
(OUT/'files-with-time.json').write_text(json.dumps(files,indent=2)+'\n')
print('Sequences contiguous',len(rows),'rotations',len(rotations),'discontinuities',len(failures),'start/stop',len(starts),len(stops))
print('STATUS',dict(collections.Counter(f['crcl']['end_status'] for f in files)))
print('RECORDED',sum(f['crcl']['frames'] for f in files)/16000,'reference bytes',sum(f['reference']['bytes'] for f in files),'companion bytes',sum(f['crcl']['bytes'] for f in files))
print('TIMELINE',len(timeline),'first',timeline[0],'last',timeline[-1])
for i in incidents:print('INCIDENT',json.dumps(i))
print('RADIO ERRORS')
last_errors=0
for r in rows:
    m=re.search(r'\berrors=(\d+)',r['message'])
    if m and int(m[1])>last_errors:
        print('COUNTER INCREASE',r['line'],r['uk_estimate'],r['message']);last_errors=int(m[1])
    if r['source']=='Radio' and re.search(r'fail|miss|error|timeout',r['message']) and not r['message'].startswith(('summary','minute end')):
        print(r['line'],r['uk_estimate'],r['message'])
print('GPS transitions')
for r in rows:
    if r['source']=='Gps' and ('2026-10-16T06:' in r['uk_estimate'] or '2026-10-16T03:4' in r['uk_estimate']):print(r['line'],r['uk_estimate'],r['message'])
print('MORNING / FINAL selected')
for r in rows:
    if r['seconds']>39900 and (r['source']=='Time' or r['message'].startswith(('audio started','audio stopped','audio discontinuity','audio rotated'))):print(r['line'],r['uk_estimate'],r['message'])
print('GPS last accepted at',dt.datetime.fromtimestamp(log['offset_utc_s']+39155153779/1e6,dt.timezone.utc).astimezone(ZoneInfo('Europe/London')).isoformat())
