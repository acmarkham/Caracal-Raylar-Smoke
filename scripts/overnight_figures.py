"""Plot overnight timing evidence and produce compact numeric diagnostics."""
import csv
import datetime as dt
import json
from pathlib import Path
import re
import statistics
from zoneinfo import ZoneInfo
import matplotlib
matplotlib.use('Agg')
import matplotlib.dates as mdates
import matplotlib.pyplot as plt

OUT=Path('analysis/aardwolf_overnight/20261016')
log=json.loads((OUT/'parsed-log.json').read_text());rows=log['rows']
files=json.loads((OUT/'files-with-time.json').read_text())
incidents=json.loads((OUT/'incidents.json').read_text())
rotations=list(csv.DictReader((OUT/'rotation-timeline.csv').open()))
tz=ZoneInfo('Europe/London')
def when(r):return dt.datetime.fromisoformat(r['uk_estimate'])
cpu=[(when(r),float(m[1])) for r in rows if (m:=re.search(r'cpu usage=([\d.]+)%',r['message']))]
timings=[(when(r),dict((k,int(v)) for k,v in re.findall(r'(\w+)=(\d+)',r['message']))) for r in rows if r['message'].startswith('audio timing')]
plt.rcParams.update({'font.size':10,'axes.grid':True,'grid.alpha':.22})
fig,axes=plt.subplots(3,1,figsize=(13,9),sharex=True,layout='constrained')
x=[dt.datetime.fromisoformat(r['logged_rotation']) for r in rotations]
axes[0].plot(x,[float(r['source_end_to_wall_delay_s'])/60 for r in rotations],'.',ms=3,color='#663399')
axes[0].set_ylabel('Rotation delay (minutes)');axes[0].set_title('Overnight recording: source timestamp lag, CPU and audio backlog')
axes[0].annotate('Capture restart resets source epoch',xy=(dt.datetime(2026,10,16,3,46,tzinfo=tz),1),xytext=(dt.datetime(2026,10,16,1,20,tzinfo=tz),2.2),arrowprops={'arrowstyle':'->'})
axes[1].plot(*zip(*cpu),color='#2474ad',lw=1);axes[1].set_ylabel('CPU usage (%)')
axes[2].plot([t for t,v in timings],[v['backlog_max_samples']/96000 for t,v in timings],lw=.7,color='#2474ad')
axes[2].axhline(127998/96000,color='#b22222',ls='--',label='Ring retention: 1.333 s');axes[2].legend(loc='upper left');axes[2].set_ylabel('Backlog max (seconds)')
for ax in axes:
    for i in incidents:ax.axvline(dt.datetime.fromisoformat(i['failure_bst']),color='#cc3333',alpha=.2,lw=.8)
axes[-1].xaxis.set_major_formatter(mdates.DateFormatter('%H:%M',tz=tz));axes[-1].set_xlabel('BST, 15–16 October 2026 (log UTC conversion approximate)')
fig.savefig(OUT/'overnight-timeline.png',dpi=160);plt.close(fig)

fig,axes=plt.subplots(2,1,figsize=(13,6.5),sharex=True,layout='constrained')
begin=dt.datetime(2026,10,16,6,20,tzinfo=tz);end=dt.datetime(2026,10,16,6,56,40,tzinfo=tz)
axes[0].plot([t for t,v in timings],[v['codec_max_us']/1000 for t,v in timings],label='Codec maximum / 10 s',color='#2474ad',lw=1.2)
axes[0].plot([t for t,v in timings],[v['write_max_us']/1000 for t,v in timings],label='Measured block storage maximum',color='#ee9922',lw=1)
axes[0].set_ylim(0,210);axes[0].legend(loc='upper left');axes[0].set_ylabel('Elapsed milliseconds')
axes[0].set_title('Morning: blue activity resumes during timestamp seeking, with codec/storage still idle')
axes[1].plot(*zip(*cpu),color='#2474ad');axes[1].set_ylabel('CPU usage (%)');axes[1].set_ylim(0,55)
for ax in axes:
    ax.set_xlim(begin,end)
    for i in incidents:
        if i['restart_bst']:ax.axvspan(dt.datetime.fromisoformat(i['failure_bst']),dt.datetime.fromisoformat(i['restart_bst']),color='#cc3333',alpha=.18)
    ax.axvspan(dt.datetime(2026,10,16,6,43,59,tzinfo=tz),dt.datetime(2026,10,16,6,46,59,tzinfo=tz),color='#44aa66',alpha=.13)
axes[1].text(dt.datetime(2026,10,16,6,44,tzinfo=tz),42,'GPS reacquisition',fontsize=9)
axes[1].xaxis.set_major_formatter(mdates.DateFormatter('%H:%M',tz=tz));axes[1].set_xlabel('BST, 16 October 2026; red shading = recorder stopped, green = GPS tracking')
fig.savefig(OUT/'morning-detail.png',dpi=160);plt.close(fig)

recorded=sum(f['crcl']['frames'] for f in files)/16000
first_start=next(r for r in rows if r['message'].startswith('audio started'))
window=rows[-1]['seconds']-first_start['seconds']
stats=dict(recording_window_s=window,recorded_finalized_s=recorded,not_in_finalized_pairs_s=window-recorded,
           retained_percent=100*recorded/window,stopped_wait_sum_s=sum(i['restart_wait_s'] or 0 for i in incidents),
           zero_codec_windows=sum(v['codec_max_us']==0 for t,v in timings),timing_windows=len(timings),
           cpu_median=statistics.median(v for t,v in cpu),cpu_max=max(v for t,v in cpu),
           last_rotation_delay_s=float(rotations[-1]['source_end_to_wall_delay_s']))
(OUT/'summary-stats.json').write_text(json.dumps(stats,indent=2)+'\n');print(json.dumps(stats,indent=2))
print('Last-epoch stop waits',sum(i['restart_wait_s'] for i in incidents if i['failure_ticks_s']>30336))
print('Latest files and metadata modified times')
for f in files[-6:]:
    p=Path(f['reference']['path']);print(p.name,'intended',f['uk_start'],'modified',dt.datetime.fromtimestamp(p.stat().st_mtime,tz).isoformat())
print('Selected log contexts for radio counters')
for a,b in [(2558,2572),(6480,6500)]:
    for r in rows[a:b]:print(r['line'],r['uk_estimate'],r['message'])
print('Final GPS',next(r['message'] for r in reversed(rows) if r['source']=='Gps'))
