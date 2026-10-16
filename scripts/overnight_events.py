"""Summarize overnight log events in UTC/BST without changing the SD card."""
from pathlib import Path
import collections
import csv
import datetime as dt
import json
import re
from zoneinfo import ZoneInfo

OUT=Path('analysis/aardwolf_overnight/20261016')
lines=(OUT/'syslog.txt').read_text(encoding='utf8',errors='replace').splitlines()
rows=[]
for number,line in enumerate(lines,1):
    match=re.match(r'(\d+) ([\d.]+) (\w+)\s+(\w+)\s+(.*)',line)
    if match:
        seq,t,level,source,message=match.groups()
        rows.append(dict(line=number,sequence=int(seq),seconds=float(t),level=level,source=source,message=message))
anchor=next(r for r in rows if 'audio started utc_us=' in r['message'])
utc_us,ticks=map(int,re.search(r'utc_us=(\d+) ticks=(\d+)',anchor['message']).groups())
offset=utc_us/1e6-ticks/1e6
for r in rows:
    utc=dt.datetime.fromtimestamp(offset+r['seconds'],dt.timezone.utc)
    r['utc_estimate']=utc.isoformat(timespec='milliseconds')
    r['uk_estimate']=utc.astimezone(ZoneInfo('Europe/London')).isoformat(timespec='milliseconds')
events=[]
for r in rows:
    if ((r['source']=='Audio' and not r['message'].startswith(('audio timing','audio rotated')))
        or (r['source']=='Radio' and re.search(r'fail|miss|skipped|late',r['message']) and not r['message'].startswith('summary'))
        or 'microphone gap' in r['message'] or 'capture restarted' in r['message']):events.append(r)
(OUT/'events.json').write_text(json.dumps(events,indent=2)+'\n')
with (OUT/'events.csv').open('w',newline='') as file:
    writer=csv.DictWriter(file,fieldnames=rows[0].keys());writer.writeheader();writer.writerows(events)
print('UTC offset estimate',offset,'first',rows[0]['uk_estimate'],'last',rows[-1]['uk_estimate'])
print('Boots',[r for r in rows if 'boot node=' in r['message']])
print('Event messages',collections.Counter(re.sub(r'\d+','N',r['message']) for r in events))
for r in events:print(r['line'],r['uk_estimate'],r['message'])
target=dt.datetime(2026,10,16,5,30,tzinfo=dt.timezone.utc).timestamp()-offset
print('TARGET',target)
near=[r for r in rows if abs(r['seconds']-target)<240]
(OUT/'around-0630.txt').write_text('\n'.join(f"{r['line']} {r['uk_estimate']} {r['source']} {r['message']}" for r in near),encoding='utf8')
print('Near target important')
for r in near:
    if r['source'] in ('Gps','Time','Audio') and not r['message'].startswith(('audio timing','audio rotated')):print(r['line'],r['uk_estimate'],r['message'])
for key in ['codec_max_us','write_max_us','backlog_max_samples','audio_losses','errors','misses','pps_timeouts','uart_errors','log_drops','log_truncated','log_failures','radio_conflicts','radio_queue_drops']:
    vals=[(int(m.group(1)),r) for r in rows if (m:=re.search(r'\b'+key+r'=(\d+)',r['message']))]
    if vals:
        v,r=max(vals,key=lambda t:t[0]);print('MAX',key,v,r['line'],r['uk_estimate'])
(OUT/'parsed-log.json').write_text(json.dumps(dict(offset_utc_s=offset,rows=rows),indent=2)+'\n')
