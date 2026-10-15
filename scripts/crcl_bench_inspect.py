"""Inspect reproducibility inputs and disassemble benchmark hot functions."""
from pathlib import Path
import argparse
import hashlib
import json
import subprocess

root=Path(__file__).resolve().parents[1]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output',type=Path,default=root/'docs/audio/multichannel/analysis003')
out=parser.parse_args().output
out.mkdir(parents=True,exist_ok=True)
for name in ['aardwolf','crcl-loop-benchmark']:
    p=root/'target/thumbv8m.main-none-eabihf/release'/name
    print(name, p.exists(), hashlib.sha256(p.read_bytes()).hexdigest() if p.exists() else '')
sysroot=Path(subprocess.check_output(['rustc','--print','sysroot'],text=True).strip())
objdump=sysroot/'lib/rustlib/x86_64-pc-windows-msvc/bin/llvm-objdump.exe'
binary=root/'target/thumbv8m.main-none-eabihf/release/crcl-loop-benchmark'
result=subprocess.check_output([str(objdump),'--demangle','--disassemble',str(binary)],text=True,encoding='utf8')
names=['Transform::advance_cheap','Transform::advance','Encoder::data','crcl_encoder_service::bits::','crcl_encoder_service::benchmark::']
sections=result.split('\n\n')
selected=[s for s in sections if any(name in s.split('\n')[0] for name in names)]
(out/'hot-functions.asm').write_text('\n\n'.join(selected),encoding='utf8')
print('assembly sections',len(selected))
print('\n'.join(s.split('\n')[0] for s in selected))
