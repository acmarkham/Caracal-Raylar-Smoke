"""Summarize the 16 October daytime run and verify its short paired prefixes."""
from pathlib import Path
import datetime as dt
import json
import re
import shutil
from zoneinfo import ZoneInfo
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from crcl_decode import reference,decode

OUT=Path('analysis/aardwolf_overnight/20261016_day')
summary=json.loads((OUT/'summary.json').read_text())
boots=json.loads((OUT/'log.json').read_text());rows=boots[-1]['rows']
incidents=json.loads((OUT/'incidents.json').read_text())
files=json.loads((OUT/'files.json').read_text())
for f in files:
    c=f['crcl'];r=f['reference']
    assert c['frames']==r['frames'],f['file']
    assert c['refbytes']==r['bytes'],f['file']
    assert c['first_frame']==int(r['first_source_frame']),f['file']
    assert c['sample_ticks']==int(r['system_timestamp']),f['file']
    assert c['blocks']==(c['frames']+1023)//1024,f['file']
print('Header/END totals, paired source frame/timestamp and block counts agree for',len(files),'pairs')
short=summary['short'];results=[];table=[]
copy=OUT/'short-recordings';copy.mkdir(exist_ok=True)
for f,incident in zip(short,incidents):
    path=Path(f['path']);side=path.with_suffix('.CRCL')
    ref=reference(path);pcm,report=decode(side.read_bytes(),ref)
    assert pcm[::6]==ref['pcm'] and report['frames']==1024 and report['completion_status']==4
    results.append(dict(file=f['file'],mic1_matches=True,**report))
    shutil.copy2(path,copy/path.name);shutil.copy2(side,copy/side.name)
    lifecycle=[x for x in incident['context'] if x['message'].startswith('audio lifecycle') and f"first_sample_ticks={f['crcl']['sample_ticks']}" in x['message']]
    assert len(lifecycle)==1,(f,lifecycle)
    values=dict((k,int(v)) for k,v in re.findall(r'(\w+)=(\d+)',lifecycle[0]['message']))
    stamp=dt.datetime.fromtimestamp(f['stamp'],ZoneInfo('Europe/London'))
    table.append(dict(file=f['file'],minute_bst=stamp.strftime('%H:%M'),error_bst=incident['event']['bst'],
        open_s=values['open_max_us']/1e6,finish_max_s=values['finish_max_us']/1e6,ref_bytes=f['reference']['bytes'],crcl_bytes=f['crcl']['bytes']))
(OUT/'decode-checks.json').write_text(json.dumps(results,indent=2)+'\n')
(OUT/'failure-table.json').write_text(json.dumps(table,indent=2)+'\n')

fig,ax=plt.subplots(figsize=(10,4.8),layout='constrained')
xs=range(len(table));ax.bar(xs,[t['open_s'] for t in table],color='#4477aa',label='Paired open, failed segment')
ax.plot(xs,[t['finish_max_s'] for t in table],'o',color='#dd9933',label='Maximum finish in incident window')
ax.axhline(127998/96000,color='#bb3333',ls='--',label='PCM ring retention: 1.333 s')
ax.set_xticks(list(xs),[t['minute_bst'] for t in table]);ax.set_xlabel('Failed recording minute, BST, 16 October 2026')
ax.set_ylabel('Elapsed seconds');ax.set_title('Storage transitions exceed the available audio-buffer time')
ax.grid(axis='y',alpha=.2);ax.set_axisbelow(True);ax.legend(loc='upper right');ax.set_ylim(0,4.2)
fig.savefig(OUT/'storage-transitions.png',dpi=160);plt.close(fig)

start=next(r for r in rows if r['message'].startswith('audio started'))
last_rotation=next(r for r in reversed(rows) if r['message'].startswith('audio rotated'))
normal_interval=last_rotation['seconds']-start['seconds']
complete_nominal_interval=(files[-1]['stamp']+60)-files[0]['stamp']
print('missing through last finalized nominal boundary',complete_nominal_interval-summary['recorded_s'])
print('timestamp status first/last error max',summary['rotation_delays'][0],summary['rotation_delays'][-1])
print('TABLE',json.dumps(table,indent=2))

md='''# Aardwolf daytime SD run — 16 October 2026

Seven recordings are truncated to **1,024 frames (64 ms)**. The new instrumentation establishes the remaining failure mechanism: the recorder is blocked by file finalization/creation for longer than its **1.333-second PCM ring** can retain. The existing timestamp-recovery fix is working; source timestamps do not accumulate the minutes-long offset seen in the previous run.

The card and firmware were not modified during this review. The log and short file pairs were copied locally.

## Run and recording impact

- The log contains three boots, including two earlier bench/startup sessions. The main field run is **boot 10466**, approximately **09:35:11–13:29:45 BST**; recording begins **09:37 BST**. Boot transitions include concatenated partial log lines, which the parser handles explicitly.
- The main run has **232 finalized pairs: 225 normal minute pairs and seven discontinuity prefixes**. Each short CRCL is one valid DATA block followed by END status 4; companion FLACs contain the same 1,024 frames.
- Finalized audio totals **3 h 45 min 0.474 s**. Approximately **6 min 59.5 s** is absent between the first recording start and the last finalized nominal boundary at 13:29. This is essentially seven minutes, each retaining just 64 ms.
- Another approximately **44.8 s** to the last log record lies in the final unfinalized segment. `aud_1792153740.flac` and its CRCL have zero visible lengths. A separate zero-length reference at `aud_1792139700.flac` belongs to the earlier bench interval and is not counted as a new field-run failure.
- Firmware metadata identifies the tested build as `c61328edf55b918bfec0e03a90ca1fd913a516e5-dirty`, agreeing with the build used for the recovery changes.

All event times in this report are BST. Event timestamps use the run's first UTC/system-time anchor and are approximate; subsequent GPS calibration changes the mapping slightly. The minute labels in the table come from the actual recording filenames.

## The short files

| Recording minute (BST) | FLAC bytes | CRCL bytes | Paired open (s) | Max finish in incident window (s) |
|---|---:|---:|---:|---:|
'''
for t in table:md+=f"| {t['minute_bst']} | {t['ref_bytes']:,} | {t['crcl_bytes']:,} | {t['open_s']:.3f} | {t['finish_max_s']:.3f} |\n"
md+='''
![Measured transition durations and ring retention](storage-transitions.png)

The finish metric is a window maximum, not necessarily the tiny prefix's own finish. It includes finalization of the preceding full minute. It must not be added indiscriminately to maxima from unrelated calls. Here all seven failures occur at a minute transition; the lifecycle and backlog records show the recorder entering the next open with data already waiting.

The 10:22 example matters: its open takes only 0.832 s, below the nominal ring retention, but a preceding finish takes about 0.809 s. The recorder has not drained the accumulated samples before it starts the open, so the combined transition still overruns. Merely requiring each individual storage call to be shorter than 1.333 s would be insufficient.

## What the new timing proves

At 10:57, the reference open takes **1.911 s**, the companion open **1.412 s**, and the paired lifecycle open **3.325 s**. At 11:27 both individual opens are about **1.4 s**. These are backend execution times; their mutex waits are only approximately **1–2 microseconds**.

Admission itself peaks at **7 microseconds**, so repeated free-space scans are excluded as the cause in this run. Conversely, log append can wait **3.321 s** for the storage mutex: logging is waiting behind the audio transition in that incident.

The longest logged ordinary audio append is approximately **392 ms**, predominantly mutex wait behind other storage work; its measured backend execution maximum in that window is about 45 ms. Log checkpoint execution peaks at **436 ms**. These can consume buffer headroom too, but the seven observed failures consistently coincide with file transitions. The maximum codec elapsed time is **119 ms**, far below the multi-second transition delays.

The recorder reads its first chunk of the new segment, then awaits admission, reference creation, companion creation and header writes before accepting further source samples. During these waits capture keeps publishing. The ring fills, the next read reports loss, and the recorder correctly closes only its aligned prefix. Application policy then waits for the next UTC minute, losing most of that minute even if the direct overrun was only a few seconds.

The open measurement includes directory lookup/creation and metadata persistence. It does not isolate time spent in directory scanning, allocation, individual card commands or card-internal busy/garbage collection. Thus this run proves **storage transition latency exceeds buffering**, but does not prove the card itself is defective or identify a particular exFAT sub-operation.

## What the previous changes fixed

- `timestamp_error_ticks=0` throughout all logged health records in the main run.
- No capture restarts or missed DMA publication frames are reported. Nominal capture-clock phase stays within about 5 microseconds of its nominal timeline.
- The 225 successful minute pairs match the 225 rotation messages. Rotation completion minus the file's sample-end timestamp is about 0.313 s initially and 0.102 s finally, with a maximum around 0.990 s. This includes processing/close latency; it shows no accumulating minutes-long offset.
- Recovery resumes at the next minute and normal encoding returns promptly, rather than discarding several extra minutes while seeking stale time.
- Storage flags, logging drops and logging write failures remain zero. Seven recorder errors correspond to the seven discontinuities.

The timestamp/guard changes solved one failure mode. The limited buffering and synchronous storage lifecycle still cause recording loss; the short hardware recovery test exercised recovery, not elimination of long SD stalls.

## Validation and next change

All seven short pairs were independently decoded using FFmpeg and the CRCL decoder. Their reconstructed MIC1 exactly matches the reference, with 1,024 aligned frames and discontinuity status 4. The tiny files are valid prefixes, not evidence of corrupted Rice packing. Full-length files were checked here using reference metadata, CRCL header/END totals and rotation correspondence; their entire payloads were not exhaustively decoded.

The next fix must let capture/encoding continue while storage completes file transitions. A bounded queue of **encoded** output and lifecycle commands, drained by a storage writer, is the memory-efficient approach. This must cover headers, reference rewrite, END and close ordering as well as ordinary audio data; putting only raw SD writes on another task while the recorder still awaits open/close would retain the same blocking behavior.

A larger raw PCM ring is a simpler interim alternative. The worst incident contains approximately 0.921 s of finish work and 3.325 s of open work, before allowing for existing backlog. Five seconds of six-channel 32-bit PCM requires **1,920,000 bytes**, versus the current 511,992 bytes. That is an additional **1,408,008 bytes**, and five seconds is only a measured-run sizing example, not a bound on all future card latency.

An encoded queue is substantially smaller for the observed audio. Its capacity must be checked against the configured worst-case encoded rate and transition latency; a nominal 512 KiB queue is not automatically a worst-case guarantee. Preallocation and fewer metadata writes may reduce stalls, but should complement adequate buffering rather than be assumed to eliminate arbitrary card latency. Further filesystem/SD instrumentation can distinguish the slow sub-operations.

Artifacts: [preserved log](syslog.txt), [incident contexts](incidents.json), [file metadata](files.json), [summary and storage maxima](summary.json), [decoder results](decode-checks.json), and [copied short recordings](short-recordings/). The log fingerprint is recorded in `summary.json`.
'''
(OUT/'report.md').write_text(md,encoding='utf8')
print('REPORT',OUT/'report.md')
