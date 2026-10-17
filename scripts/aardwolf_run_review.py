"""Read-only SD review with boot-aware parsing and compact file/latency evidence."""
from pathlib import Path
import bisect
import collections
import csv
import datetime as dt
import hashlib
import json
import re
import shutil
import struct
from zoneinfo import ZoneInfo
from overnight_files import flac
from crcl_decode import number,tlvs

OUT=Path('analysis/aardwolf_overnight/20261016_day')
OUT.mkdir(parents=True,exist_ok=True)
raw=Path('E:/syslog.txt').read_bytes();(OUT/'syslog.txt').write_bytes(raw)
text=raw.decode('utf8',errors='replace');newlines=[m.start() for m in re.finditer('\n',text)]
matches=list(re.finditer(r'(\d{10}) ([\d.]+) (TRACE|DEBUG|INFO|WARN|ERROR)\s+(\w+)\s+',text))
boots=[]
for i,m in enumerate(matches):
    seq,seconds,level,source=m.groups()
    msg=text[m.end():matches[i+1].start() if i+1<len(matches) else len(text)].strip()
    if msg.startswith('boot node='):
        boot=int(re.search(r'boot=(\d+)',msg)[1]);boots.append(dict(boot=boot,rows=[]))
    if not boots:continue
    boots[-1]['rows'].append(dict(sequence=int(seq),seconds=float(seconds),level=level,source=source,message=msg,line=bisect.bisect_left(newlines,m.start())+1))
for b in boots:
    anchor=next((r for r in b['rows'] if r['message'].startswith('audio started utc_us=')),None)
    if anchor:
        utc,ticks=map(int,re.search(r'utc_us=(\d+) ticks=(\d+)',anchor['message']).groups());b['offset']=utc/1e6-ticks/1e6
        for r in b['rows']:
            r['bst']=dt.datetime.fromtimestamp(b['offset']+r['seconds'],dt.timezone.utc).astimezone(ZoneInfo('Europe/London')).isoformat(timespec='milliseconds')
    print('BOOT',b['boot'],'rows',len(b['rows']),'uptime',b['rows'][-1]['seconds'],'first/last',b['rows'][0].get('bst'),b['rows'][-1].get('bst'))
    for r in b['rows']:
        if r['level'] in ('WARN','ERROR') and r['source']=='Audio':print('EVENT',r.get('bst'),r['line'],r['message'])
(OUT/'log.json').write_text(json.dumps(boots,indent=2)+'\n')
latest=boots[-1];rows=latest['rows'];offset=latest['offset']
failures=[r for r in rows if r['level'] in ('WARN','ERROR') and r['source']=='Audio']
incidents=[]
for r in failures:
    context=[x for x in rows if r['seconds']-12<=x['seconds']<=r['seconds']+12 and x['source'] in ('Audio','Storage','Gps','Cpu')]
    incidents.append(dict(event=r,context=context))
    print('\nINCIDENT',r['bst'],r['message'])
    for x in context:
        if not x['message'].startswith(('storage timing op=append','storage timing op=log_append')):print(x['line'],x['bst'],x['source'],x['message'])
(OUT/'incidents.json').write_text(json.dumps(incidents,indent=2)+'\n')
maxima={}
for r in rows:
    if r['message'].startswith('storage timing'):
        vals=dict(re.findall(r'(\w+)=([^ ]+)',r['message']));op=vals['op']
        if op not in maxima or int(vals['total_max_us'])>int(maxima[op]['values']['total_max_us']):maxima[op]=dict(values=vals,line=r['line'],bst=r['bst'])
print('\nMAX STORAGE',json.dumps(maxima,indent=2))
for key in ['timestamp_error_ticks','nominal_clock_error_ticks','call_max_us','codec_max_us','write_max_us','backlog_max_samples','dropped_scalars','capture_restarts','missing_capture_frames','errors','log_drops','log_failures','log_truncated']:
    values=[(int(m[1]),r) for r in rows if (m:=re.search(r'\b'+key+r'=(-?\d+)',r['message']))]
    if values:
        value,r=max(values,key=lambda t:abs(t[0]));print('MAX',key,value,r['bst'],r['line'])
first=offset+rows[0]['seconds'];last=offset+rows[-1]['seconds']
files=[];unfinalized=[]
for folder in sorted(Path('E:/').iterdir()):
    if not folder.is_dir() or not folder.name.isdigit() or not first-3600<=int(folder.name)<=last:continue
    for path in sorted(folder.glob('*.flac')):
        ref=flac(path)
        if ref.get('boot_id')!=str(latest['boot']):
            if ref.get('bytes')==0:unfinalized.append(str(path))
            continue
        side=path.with_suffix('.CRCL');crcl={}
        with side.open('rb') as f:
            data=f.read(4096);length=number(data,10,'H');meta=tlvs(data[92:length])
            origin,sample_ticks=struct.unpack('<QQ',meta[1]);sample_utc_us=number(meta[2],0,'Q')
            f.seek(-52,2);end=f.read(52)
            total,refbytes,blocks,status=struct.unpack_from('<QQII',end,28) if end[:4]==b'CRBK' and end[4]==3 else (None,None,None,None)
        crcl.update(bytes=side.stat().st_size,first_frame=number(data,44,'Q'),sample_ticks=sample_ticks,sample_utc_us=sample_utc_us,frames=total,blocks=blocks,status=status,
                    refbytes=refbytes,planned=number(data,60,'Q'),origin=origin)
        stamp=int(re.search(r'aud_(\d+)',path.name)[1]);files.append(dict(file=path.name,path=str(path),stamp=stamp,reference=ref,crcl=crcl))
files.sort(key=lambda f:f['stamp'])
(OUT/'files.json').write_text(json.dumps(files,indent=2)+'\n')
short=[f for f in files if f['crcl']['status']!=1 or f['reference']['frames']<950000]
print('\nFILES',len(files),'status',dict(collections.Counter(f['crcl']['status'] for f in files)),'zero length',unfinalized)
print('SHORT FILES')
for f in short:print(json.dumps(f))
rotations=[r for r in rows if r['message'].startswith('audio rotated')];normal=[f for f in files if f['crcl']['status']==1]
delays=[]
if len(rotations)==len(normal):
    for f,r in zip(normal,rotations):
        delays.append(dict(file=f['file'],bst=r['bst'],seconds=r['seconds']-(f['crcl']['sample_ticks']/1e6+f['crcl']['frames']/16000)))
    print('ROTATION DELAY first/last/max',delays[:1],delays[-1:],max(delays,key=lambda d:d['seconds']))
else:print('ROTATION MISMATCH',len(rotations),len(normal))
frames=sum(f['crcl']['frames'] or 0 for f in files)
start=next(r['seconds'] for r in rows if r['message'].startswith('audio started'))
summary=dict(sha256=hashlib.sha256(raw).hexdigest(),boot=latest['boot'],files=len(files),frames=frames,recorded_s=frames/16000,
             elapsed_s=rows[-1]['seconds']-start,short=short,unfinalized=unfinalized,storage_maxima=maxima,rotation_delays=delays)
(OUT/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
print('TOTAL frames',frames,'recorded_s',frames/16000,'elapsed_s',summary['elapsed_s'])
