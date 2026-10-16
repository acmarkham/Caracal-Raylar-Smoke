"""Targeted follow-up of overnight file timestamps and failures (read-only card)."""
import collections
import datetime as dt
import json
from pathlib import Path
import re
import struct
from overnight_files import OUT, flac
from crcl_decode import tlvs, number

files=json.loads((OUT/'files.json').read_text())
log=json.loads((OUT/'parsed-log.json').read_text())
rows=log['rows']
matched={r['reference']['path'] for r in files}
for folder in Path('E:/').iterdir():
    if not folder.is_dir() or not folder.name.isdigit() or not 1792087200<=int(folder.name)<=1792126800:continue
    for path in sorted(folder.glob('*.flac')):
        if str(path) not in matched:
            print('UNMATCHED',json.dumps(flac(path)))
            side=path.with_suffix('.CRCL')
            if side.exists():
                with side.open('rb') as f:head=f.read(4096)
                print('UNMATCHED SIDECAR',side,side.stat().st_size,'prefix',head[:128].hex())
print('STATUS COUNTS',collections.Counter(r['crcl'].get('end_status') for r in files))
print('START EXAMPLES')
for i,r in enumerate(files):
    if i<3 or r['crcl']['frames']<10000 or (i and files[i-1]['crcl']['frames']<10000) or i>=len(files)-3:
        data=Path(r['crcl']['path']).read_bytes()[:4096]
        meta=tlvs(data[92:number(data,10,'H')])
        origin,sample=struct.unpack('<QQ',meta[1])
        utc,unc=struct.unpack_from('<QQ',meta[2]);ref=r['reference']
        print(r['uk_start'],'dur',r['crcl']['frames']/16000,'first_frame',r['crcl']['first_source_frame'],
              'origin',origin,'sample',sample,'sample_log_bst',dt.datetime.fromtimestamp(sample/1e6+log['offset_utc_s'],dt.timezone(dt.timedelta(hours=1))).isoformat(),
              'actual_utc_delta_s',(utc-r['stamp']*1000000)/1e6,'ref',ref.get('system_timestamp'))
print('AUDIO FAILURES')
for i,r in enumerate(rows):
    if 'discontinuity' in r['message'] or 'capture restarted' in r['message'] or 'microphone gap' in r['message']:
        print('\nEVENT',r['line'],r['uk_estimate'],r['message'])
        nearby=[x for x in rows[max(0,i-35):i+10] if x['source'] in ('Audio','Gps','Power') or 'cpu=' in x['message']]
        for x in nearby:print(x['line'],x['uk_estimate'],x['source'],x['message'])
print('RADIO NONZERO ERRORS')
for r in rows:
    if r['source']=='Radio' and re.search(r'error|fail|miss|late|skip|timeout',r['message']) and not r['message'].startswith(('summary','minute end')):print(r['line'],r['uk_estimate'],r['message'])
print('GPS NEAR MORNING')
for r in rows:
    if '2026-10-16T06:' in r['uk_estimate'] and (r['source']=='Gps' or 'pps' in r['message'] and r['source']!='Time'):print(r['line'],r['uk_estimate'],r['source'],r['message'])
