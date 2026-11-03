import re, json, collections, statistics, pathlib, os, wave, datetime

OUT = pathlib.Path('analysis_integration002')
OUT.mkdir(exist_ok=True)
rows = collections.defaultdict(list)
levels=collections.Counter(); modules=collections.Counter(); anomalies=[]; warnings=[]
prevseq=None; prevtime=None; first=None; last=None; bad=[]
rx=re.compile(r'^(\d+) ([\d.]+) (\w+)\s+(\w+)\s+(.*)')
kv=re.compile(r'(\w+)=([^ ]+)')
for lineno,line in enumerate(pathlib.Path('E:/syslog.txt').open(errors='replace'),1):
    m=rx.match(line)
    if not m: bad.append([lineno,line.strip()]); continue
    seq,t,level,module,msg=m.groups(); seq=int(seq); t=float(t)
    if first is None: first=[lineno,t,line.strip()]
    last=[lineno,t,line.strip()]
    if prevseq is not None and seq!=prevseq+1: anomalies.append(['sequence',lineno,prevseq,seq])
    if prevtime is not None and t<prevtime: anomalies.append(['monotonic',lineno,prevtime,t])
    prevseq,prevtime=seq,t
    levels[level]+=1; modules[module]+=1
    if level!='INFO': warnings.append([lineno,t,line.strip()])
    typ=None
    if module=='Time': typ='gate' if msg.startswith('PPS_GATE') else 'time'
    elif module=='Gps':
        if msg.startswith('state='): typ='gps'
        elif msg.startswith('PHASE'): typ='phase'
    elif module=='Pps' and msg.startswith('EDGE'): typ='edge'
    elif module=='GpsCorr' and msg.startswith('PAIR'): typ='corr'
    elif module=='Power': typ='power'
    elif module=='Cpu': typ='cpu'
    elif module=='Audio': typ='audio'
    if typ:
        d=dict(kv.findall(msg)); d.update(t=t,line=lineno)
        if typ=='time':
            u=re.search(r'UTC (\d+)',msg)
            if u: d['utc']=int(u[1])
        rows[typ].append(d)

def num(d,k):
    s=d.get(k,'None')
    if s.startswith('Some('): s=s[5:-1]
    try: return float(s.rstrip('%'))
    except (ValueError,AttributeError): return None
def stats(v):
    v=sorted(x for x in v if x is not None)
    if not v:return {}
    return dict(n=len(v),min=v[0],median=statistics.median(v),mean=statistics.mean(v),p95=v[int((len(v)-1)*.95)],p99=v[int((len(v)-1)*.99)],max=v[-1])
def transitions(typ,keys):
    ret=[]; prev=None
    for d in rows[typ]:
        cur=tuple(d.get(k) for k in keys)
        if cur!=prev: ret.append(d); prev=cur
    return ret

edges=rows['edge']; intervals=[num(d,'delta_ticks') for d in edges]; clean=[v for v in intervals if v is not None and abs(v-1000000)<=20]
gaps=[d for d in edges if num(d,'delta_ticks') is not None and num(d,'delta_ticks')>1500000]
outliers=[d for d in edges if num(d,'delta_ticks') is not None and abs(num(d,'delta_ticks')-1000000)>20]
pps_seq_gaps=[]
for a,b in zip(edges,edges[1:]):
    if num(b,'count')!=num(a,'count')+1: pps_seq_gaps.append([a,b])
audio=rows['audio']; audio_bad=[d for d in audio if d.get('packets')!='10' or d.get('samples')!='16000']
audio_gaps=[]
for a,b in zip(audio,audio[1:]):
    x=num(a,'last_timestamp_ticks'); y=num(b,'first_timestamp_ticks')
    if x is not None and y is not None and y-x!=100000: audio_gaps.append([a,b,y-x])
times=rows['time']; valid=[d for d in times if 'utc' in d]
synced=[d for d in valid if d.get('status')=='Synced']
locked=[d for d in valid if d.get('cal_lock')=='true']
summary=dict(first=first,last=last,levels=levels,modules=modules,anomalies=anomalies,unparsed=bad,warnings=warnings,
    first_last={k:[v[0],v[-1]] for k,v in rows.items() if v},
    time_status=collections.Counter(d.get('status') for d in times),
    time_status_transitions=transitions('time',['status']),calibration_transitions=transitions('time',['cal_ppb','cal_n','cal_lock']),
    gps_transitions=transitions('gps',['state']), phase_transitions=transitions('phase',['shutdowns','timeouts']),
    power_transitions=transitions('power',['source','health','charger_fault']),
    pps=dict(interval_stats=stats(intervals),clean_interval_stats=stats(clean),outlier_count=len(outliers),outliers=outliers,gaps=gaps,sequence_gaps=pps_seq_gaps),
    correlation=dict(matched=sum(d.get('pps_count')!='None' for d in rows['corr']),unmatched=sum(d.get('pps_count')=='None' for d in rows['corr']),offset_us=stats(num(d,'offset_us') for d in rows['corr'])),
    residual_us=stats(num(d,'residual_us') for d in valid),locked_residual_us=stats(num(d,'residual_us') for d in locked),
    audio=dict(records=len(audio),bad=audio_bad,gaps=audio_gaps,samples=sum(int(d.get('samples',0)) for d in audio)),
    cpu_active=stats(num(d,'active') for d in rows['cpu']),
    gps_max_counters={k:max((num(d,k) or 0) for d in rows['gps']) for k in ['checksum_err','uart_err','overflow','pps_timeouts','search_timeouts']})
(OUT/'summary.json').write_text(json.dumps(summary,indent=2))
(OUT/'pps_edges.json').write_text(json.dumps(edges,indent=2))
print(json.dumps({k:v for k,v in summary.items() if k in ['first','last','levels','modules','time_status','cpu_active','gps_max_counters','correlation']},indent=2))
print('SEQUENCE/TIMESTAMP ANOMALIES',collections.Counter(x[0] for x in anomalies))
print('AUDIO RECORDS',len(audio),'INCOMPLETE OR UNEXPECTED COUNTS',len(audio_bad),'TIMESTAMP DEVIATIONS',len(audio_gaps))
print('WARNING COUNT',len(warnings),'PPS',len(edges),'OUTLIERS',len(outliers),'GAPS',len(gaps))
print('PPS CLEAN',stats(clean))
print('SUMMARY WRITTEN',OUT/'summary.json')
