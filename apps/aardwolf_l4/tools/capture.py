"""Run a built image through ST-Link; preserve exact identity and live RTT log."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import queue
import subprocess
import threading
import time

parser=argparse.ArgumentParser()
parser.add_argument('stage',help='Evidence filename prefix, e.g. bench-03')
parser.add_argument('--mode',choices=['bench','continuous'],default='bench')
parser.add_argument('--elf',type=Path,default=Path('target/thumbv7em-none-eabihf/release/aardwolf-l4'))
parser.add_argument('--connect-under-reset',action='store_true',help='Recover a target after an interrupted flash')
args=parser.parse_args()
if Path(args.stage).name != args.stage:
    parser.error('stage must be a filename prefix')
out=Path('apps/aardwolf_l4/evidence');out.mkdir(parents=True,exist_ok=True)
# The shell entry point is `rtk proxy python ...`. Launch the debugger as our
# direct child so its process handle can reliably release USB on Windows.
cmd=['probe-rs','run','--chip','STM32L476RGTx','--probe','0483:3748','--speed','100','--verify','--preverify','--non-interactive','--no-catch-reset','--disable-progressbars','--no-location',str(args.elf)]
if args.connect_under_reset:cmd.insert(-1,'--connect-under-reset')
record={'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd,'mode':args.mode,'elf_sha256':hashlib.sha256(args.elf.read_bytes()).hexdigest()}
process=subprocess.Popen(cmd,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,bufsize=1)
record['runner_pid']=process.pid
messages=queue.Queue()
def read_output():
    for line in process.stdout:messages.put(line)
    messages.put(None)
threading.Thread(target=read_output,daemon=True).start()
started=time.monotonic();passed=False;passed_at=None
try:
    with (out/(args.stage+'.log')).open('w',encoding='utf-8') as log:
        while time.monotonic()-started < (1250 if args.mode=='bench' else 480):
            try:line=messages.get(timeout=1)
            except queue.Empty:
                if process.poll() is not None or passed_at and time.monotonic()-passed_at>3:break
                continue
            if line is None:break
            log.write(line);log.flush();print(line.rstrip(),flush=True)
            marker='AARDWOLF L4 PASS' if args.mode=='bench' else 'AARDWOLF L4 closed index=2 '
            if marker in line:passed=True;passed_at=time.monotonic()
            if any(s in line for s in ['panicked at','AARDWOLF L4 FAIL','HardFault']):
                passed=False;break
            if passed_at and time.monotonic()-passed_at>3:break
finally:
    if process.poll() is None:process.terminate()
    try:process.wait(timeout=10)
    except subprocess.TimeoutExpired:process.kill();process.wait()
record.update({'pass':passed,'host_seconds':time.monotonic()-started})
(out/(args.stage+'.json')).write_text(json.dumps(record,indent=2)+'\n')
print(args.stage,'PASS' if passed else 'FAIL',flush=True)
raise SystemExit(0 if passed else 1)
