"""Read-only first-pass inventory of the overnight SD card evidence."""
from pathlib import Path
import collections
import hashlib
import json
import re
import shutil

OUT=Path('analysis/aardwolf_overnight/20261016')
OUT.mkdir(parents=True,exist_ok=True)
source=Path('E:/syslog.txt');data=source.read_bytes()
shutil.copy2(source,OUT/'syslog.txt')
lines=data.decode('utf8',errors='replace').splitlines()
print('LOG',len(data),'bytes',len(lines),'lines','sha256',hashlib.sha256(data).hexdigest())
print('FIRST\n'+'\n'.join(lines[:23]))
print('LAST\n'+'\n'.join(lines[-22:]))
print('LEVELS',collections.Counter(re.findall(r'\b(INFO|WARN|ERROR|DEBUG|TRACE)\s',data.decode('utf8',errors='replace'))))
issues=[(i+1,s) for i,s in enumerate(lines) if re.search(r'\b(WARN|ERROR)\s',s)]
(OUT/'warnings.txt').write_text('\n'.join(f'{i}: {s}' for i,s in issues),encoding='utf8')
print('WARNINGS',len(issues),'first',issues[:30],'last',issues[-20:])
inventory=[]
for path in Path('E:/').iterdir():
    if path.is_dir() and path.name.isdigit():
        files=list(path.iterdir())
        exts=collections.Counter(p.suffix.lower() for p in files if p.is_file())
        inventory.append(dict(folder=path.name,files=len(files),extensions=dict(exts),bytes=sum(p.stat().st_size for p in files if p.is_file())))
    elif path.is_file():print('ROOT',path.name,path.stat().st_size)
(OUT/'folders.json').write_text(json.dumps(inventory,indent=2)+'\n')
print('FOLDERS',json.dumps(inventory))
