"""Audit paired overnight FLAC/CRCL structure without decoding every Rice bit."""
from pathlib import Path
import csv
import datetime as dt
import json
import re
import struct
from zoneinfo import ZoneInfo
from crcl_decode import number,tlvs

OUT=Path('analysis/aardwolf_overnight/20261016')
BOOT=57923

def flac(path):
    result=dict(path=str(path),bytes=path.stat().st_size)
    comments=bytearray()
    with path.open('rb') as f:
        if f.read(4)!=b'fLaC':return dict(result,error='not FLAC')
        while True:
            head=f.read(4)
            if len(head)!=4:return dict(result,error='truncated metadata')
            kind=head[0]&127;size=int.from_bytes(head[1:4],'big');data=f.read(size)
            if len(data)!=size:return dict(result,error='truncated metadata payload')
            if kind==0:
                packed=int.from_bytes(data[10:18],'big')
                result.update(rate=packed>>44,channels=((packed>>41)&7)+1,bits=((packed>>36)&31)+1,frames=packed&((1<<36)-1))
            if kind==4:comments.extend(data)
            if head[0]&128:break
    text=comments.decode('utf8',errors='replace')
    for key in ['start_utc','system_timestamp','boot_id','first_source_frame','segment_id','device_id','firmware_hash','firmware_version','utc_status','uncertainty_us','calibration_ppb','last_gps_pps_utc']:
        m=re.search(r'\b'+key+r'=([^;\x00]*)',text)
        if m:result[key]=m[1]
    return result

def sidecar(path,ref):
    data=path.read_bytes();result=dict(path=str(path),bytes=len(data));errors=[]
    if len(data)<92:return dict(result,error='short header')
    length=number(data,10,'H');meta=tlvs(data[92:length])
    result.update(version=number(data,8,'H'),rate=number(data,32,'I'),block_frames=number(data,40,'H'),bits=data[42],
        first_source_frame=number(data,44,'Q'),utc_us=number(data,52,'Q'),planned_frames=number(data,60,'Q'),
        boot=number(meta[4],0,'I'),device=meta[3].decode(),firmware=meta[5].decode(),capture=meta[7].decode(),segment_id=data[16:32].hex())
    offset=length;frames=0;sequence=0;records=0;ended=False;last_time=None
    while offset<len(data):
        if offset+28>len(data) or data[offset:offset+4]!=b'CRBK':errors.append(f'invalid/truncated record at {offset}');break
        kind=data[offset+4];head=number(data,offset+6,'H');size=number(data,offset+8,'I')
        seq=number(data,offset+12,'I');at=number(data,offset+16,'Q');count=number(data,offset+24,'H')
        if seq!=sequence or at!=frames:errors.append(f'sequence/alignment at {offset}')
        if size<head or head<28 or size==0 or offset+size>len(data):errors.append(f'truncated size at {offset}');break
        if kind==1:
            if head!=88 or not 1<=count<=result['block_frames'] or data[offset+26]!=5:errors.append(f'DATA dimensions at {offset}')
            if head==88:
                total=88
                for c in range(5):
                    ch,mode,reserved,k,shift,payload=struct.unpack_from('<BBBBII',data,offset+28+c*12)
                    if ch!=c+2 or mode!=2 or reserved or k>result['bits'] or shift>32-result['bits']:errors.append(f'descriptor at {offset}')
                    total+=payload
                if total!=size:errors.append(f'payload lengths at {offset}')
            frames+=count;records+=1
        elif kind==2:
            cp=tlvs(data[offset+28:offset+size],checkpoint=True)
            last_time=cp[2].hex()
        elif kind==3:
            if size!=52:errors.append('END size')
            else:
                total,refbytes,blocks,status=struct.unpack_from('<QQII',data,offset+28)
                result.update(end_status=status,end_total_frames=total,end_reference_bytes=refbytes,end_blocks=blocks)
                if total!=frames or blocks!=records or refbytes!=ref['bytes']:errors.append('END totals/reference size')
            ended=True
            if offset+size!=len(data):errors.append('bytes after END')
        else:errors.append(f'unknown kind {kind}')
        sequence+=1;offset+=size
    result.update(frames=frames,data_records=records,ended=ended,errors=errors,last_time_anchor_hex=last_time)
    if result['segment_id']!=ref.get('segment_id'):errors.append('reference segment identity')
    if str(result['first_source_frame'])!=ref.get('first_source_frame'):errors.append('reference first frame')
    if ref['frames']!=frames:errors.append('FLAC STREAMINFO vs CRCL frame totals')
    return result

def main():
    files=[];folders=[p for p in Path('E:/').iterdir() if p.is_dir() and p.name.isdigit() and 1792087200<=int(p.name)<=1792126800]
    paths=sorted(p for folder in folders for p in folder.iterdir() if p.suffix.lower()=='.flac')
    for i,path in enumerate(paths):
        ref=flac(path)
        if ref.get('boot_id')!=str(BOOT):continue
        companion=path.with_suffix('.CRCL')
        try:crcl=sidecar(companion,ref) if companion.exists() else dict(error='missing companion')
        except Exception as e:crcl=dict(error=str(e))
        stamp=int(re.search(r'aud_(\d+)',path.name)[1]);uk=dt.datetime.fromtimestamp(stamp,dt.timezone.utc).astimezone(ZoneInfo('Europe/London'))
        files.append(dict(stamp=stamp,uk_start=uk.isoformat(),reference=ref,crcl=crcl))
        if i%50==0:print(f'files {i+1}/{len(paths)} matched={len(files)} latest={uk.isoformat()}',flush=True)
    files.sort(key=lambda r:r['stamp'])
    (OUT/'files.json').write_text(json.dumps(files,indent=2)+'\n')
    flat=[]
    for i,r in enumerate(files):
        c=r['crcl'];ref=r['reference'];previous=files[i-1] if i else None
        gap=(r['stamp']-(previous['stamp']+previous['crcl'].get('frames',0)/16000)) if previous else None
        source_gap=(c.get('first_source_frame',0)-previous['crcl'].get('first_source_frame',0)-previous['crcl'].get('frames',0)) if previous else None
        flat.append(dict(file=Path(ref['path']).name,start_uk=r['uk_start'],frames=c.get('frames'),duration_s=c.get('frames',0)/16000,
            flac_frames=ref['frames'],flac_bytes=ref['bytes'],crcl_bytes=c.get('bytes'),end_status=c.get('end_status'),
            utc_gap_s=gap,source_gap_frames=source_gap,complete=c.get('ended'),errors=';'.join(c.get('errors',[]))+c.get('error','')))
    with (OUT/'files.csv').open('w',newline='') as file:
        writer=csv.DictWriter(file,fieldnames=flat[0].keys());writer.writeheader();writer.writerows(flat)
    bad=[r for r in flat if r['errors'] or not r['complete']]
    partial=[r for r in flat if r['duration_s']<59.99]
    gaps=[r for r in flat if (r['utc_gap_s'] or 0)>.01]
    summary=dict(files=len(files),first=files[0]['uk_start'],last=files[-1]['uk_start'],
        frames=sum(r['crcl'].get('frames',0) for r in files),bad=bad,partial=partial,gaps=gaps,
        recorded_s=sum(r['duration_s'] for r in flat),positive_gap_s=sum(max(r['utc_gap_s'] or 0,0) for r in flat))
    (OUT/'file-summary.json').write_text(json.dumps(summary,indent=2)+'\n')
    print(json.dumps(summary,indent=2))

if __name__=='__main__':main()
