"""Run a built image through ST-Link; preserve exact identity and live RTT log."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import queue
import os
import signal
import subprocess
import threading
import time

parser=argparse.ArgumentParser()
parser.add_argument('stage',help='Evidence filename prefix, e.g. bench-03')
parser.add_argument('--mode',choices=['bench','continuous','fault'],default='bench')
parser.add_argument('--evidence-dir',type=Path,default=Path('apps/aardwolf_l4/evidence'))
parser.add_argument('--elf',type=Path,default=Path('target/thumbv7em-none-eabihf/release/aardwolf-l4'))
parser.add_argument('--connect-under-reset',action='store_true',help='Recover a target after an interrupted flash')
parser.add_argument('--timeout-seconds',type=int,help='Override the normal observation window for long cold starts')
args=parser.parse_args()
if Path(args.stage).name != args.stage:
    parser.error('stage must be a filename prefix')
if args.timeout_seconds is not None and args.timeout_seconds <= 0:
    parser.error('--timeout-seconds must be positive')
out=args.evidence_dir;out.mkdir(parents=True,exist_ok=True)
# The shell entry point is `rtk proxy python ...`. Launch the debugger as our
# direct child so its process handle can reliably release USB on Windows.
cmd=['probe-rs','run','--chip','STM32L476RGTx','--probe','0483:3748','--speed','100','--verify','--preverify','--non-interactive','--no-catch-reset','--no-catch-hardfault','--disable-progressbars','--no-location',str(args.elf)]
if args.connect_under_reset:cmd.insert(-1,'--connect-under-reset')
record={'started_utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'command':cmd,'mode':args.mode,'elf_sha256':hashlib.sha256(args.elf.read_bytes()).hexdigest()}
timeout=args.timeout_seconds if args.timeout_seconds is not None else (1250 if args.mode=='bench' else 480)
record['timeout_seconds']=timeout
launch={}
if os.name=='nt':
    startup=subprocess.STARTUPINFO()
    startup.dwFlags|=subprocess.STARTF_USESHOWWINDOW
    startup.wShowWindow=0
    launch={'creationflags':subprocess.CREATE_NEW_PROCESS_GROUP|subprocess.CREATE_NEW_CONSOLE,'startupinfo':startup}
process=subprocess.Popen(cmd,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True,bufsize=1,**launch)
record['runner_pid']=process.pid
messages=queue.Queue()
def read_output():
    for line in process.stdout:messages.put(line)
    messages.put(None)
threading.Thread(target=read_output,daemon=True).start()
started=time.monotonic();passed=False;passed_at=None
try:
    with (out/(args.stage+'.log')).open('w',encoding='utf-8') as log:
        while time.monotonic()-started < timeout:
            try:line=messages.get(timeout=1)
            except queue.Empty:
                if process.poll() is not None or passed_at and time.monotonic()-passed_at>(12 if args.mode=='fault' else 3):break
                continue
            if line is None:break
            log.write(line);log.flush();print(line.rstrip(),flush=True)
            marker={'bench':'AARDWOLF L4 PASS','continuous':'AARDWOLF L4 closed index=2 ','fault':'AARDWOLF L4 FAULT code='}[args.mode]
            if marker in line:passed=True;passed_at=time.monotonic()
            if args.mode != 'fault' and 'AARDWOLF L4 FAULT code=' in line:
                passed=False;break
            if any(s in line for s in ['panicked at','AARDWOLF L4 FAIL','HardFault','AARDWOLF L4 panic']):
                passed=False;break
            if passed_at and time.monotonic()-passed_at>(12 if args.mode=='fault' else 3):break
finally:
    if process.poll() is None:
        # Give probe-rs its Ctrl-C/Break cleanup path instead of terminating
        # during USB I/O, which can leave the Windows ST-Link endpoint wedged.
        if os.name=='nt':
            helper='import ctypes; k=ctypes.windll.kernel32; k.FreeConsole(); ok=k.AttachConsole('+str(process.pid)+'); k.SetConsoleCtrlHandler(None,True); sent=k.GenerateConsoleCtrlEvent(1,'+str(process.pid)+'); k.FreeConsole(); raise SystemExit(0 if ok and sent else 1)'
            subprocess.run([os.sys.executable,'-c',helper],stdout=subprocess.DEVNULL,stderr=subprocess.DEVNULL)
        else:process.send_signal(signal.SIGINT)
    try:process.wait(timeout=10);record['console_stop_completed']=True
    except subprocess.TimeoutExpired:
        record['console_stop_completed']=False;process.terminate()
    try:process.wait(timeout=10)
    except subprocess.TimeoutExpired:process.kill();process.wait()
record.update({'pass':passed,'host_seconds':time.monotonic()-started,'monitor_exit_code':process.returncode})
(out/(args.stage+'.json')).write_text(json.dumps(record,indent=2)+'\n')
print(args.stage,'PASS' if passed else 'FAIL',flush=True)
raise SystemExit(0 if passed else 1)
