// Optional dependency-free Chromium/CDP smoke test (Node.js 22+).
// Usage: node browser_batch_smoke.mjs batch.report.html [chrome.exe]
import {spawn} from 'node:child_process';
import {mkdtemp,readFile,writeFile} from 'node:fs/promises';
import {resolve,join,dirname} from 'node:path';
import {pathToFileURL} from 'node:url';
import assert from 'node:assert/strict';

const report=resolve(process.argv[2]||'report.html');
const chrome=process.argv[3]||'C:/Program Files/Google/Chrome/Application/chrome.exe';
const profile=await mkdtemp(join(dirname(report),'browser-test-'));
const browser=spawn(chrome,['--headless=new','--no-sandbox','--disable-gpu-sandbox','--in-process-gpu',
  '--disable-gpu','--no-first-run','--no-default-browser-check','--disable-background-networking',
  '--disable-extensions','--remote-debugging-port=0',`--user-data-dir=${profile}`,'about:blank'],
  {stdio:'ignore',windowsHide:true});
const sleep=ms=>new Promise(r=>setTimeout(r,ms));
let ws;
try{
  let port;
  for(let i=0;i<100;i++){
    try{port=Number((await readFile(join(profile,'DevToolsActivePort'),'utf8')).split('\n')[0]);break}catch{}
    await sleep(100);
  }
  assert(port,'Chromium did not expose a debugging port');
  const pages=await (await fetch(`http://127.0.0.1:${port}/json/list`)).json();
  ws=new WebSocket(pages.find(p=>p.type==='page').webSocketDebuggerUrl);
  await new Promise((resolve,reject)=>{ws.addEventListener('open',resolve,{once:true});ws.addEventListener('error',reject,{once:true})});
  let next=0;
  const pending=new Map(),errors=[],requests=[];
  ws.addEventListener('message',event=>{
    const msg=JSON.parse(event.data);
    if(msg.id){const p=pending.get(msg.id);if(p){pending.delete(msg.id);msg.error?p.reject(new Error(JSON.stringify(msg.error))):p.resolve(msg.result)}}
    if(msg.method==='Runtime.exceptionThrown')errors.push(msg.params.exceptionDetails.text);
    if(msg.method==='Network.requestWillBeSent')requests.push(msg.params.request.url);
  });
  const call=(method,params={})=>new Promise((resolve,reject)=>{
    const id=++next;pending.set(id,{resolve,reject});ws.send(JSON.stringify({id,method,params}));
  });
  const evaluate=async expression=>{
    const r=await call('Runtime.evaluate',{expression,awaitPromise:true,returnByValue:true});
    if(r.exceptionDetails)throw new Error(JSON.stringify(r.exceptionDetails));
    return r.result.value;
  };
  await call('Runtime.enable');await call('Network.enable');await call('Page.enable');
  await call('Emulation.setDeviceMetricsOverride',{width:1500,height:1050,deviceScaleFactor:1,mobile:false});
  await call('Page.navigate',{url:pathToFileURL(report).href});
  await sleep(2500);
  const initial=await evaluate(`({plots:document.querySelectorAll('.js-plotly-plot').length,nodes:D.runs.length,links:D.links.length,options:sel.options.length,detailLinks:document.querySelectorAll('#nodes a').length})`);
  assert.equal(initial.plots,5);assert(initial.nodes>=4);assert.equal(initial.detailLinks,initial.nodes);assert(initial.links>0);
  const filtered=await evaluate(`(async()=>{sel.value='0';drawLinks();await new Promise(r=>setTimeout(r,500));return {rows:document.querySelectorAll('#links tbody tr').length,expected:D.links.filter(l=>l.config===configs[0]).length}})()`);
  assert.equal(filtered.rows,filtered.expected);
  await evaluate(`(async()=>{document.getElementById('metric').value='snr';drawLinks();await new Promise(r=>setTimeout(r,300));return true})()`);
  const external=requests.filter(u=>/^https?:/.test(u));assert.equal(external.length,0);assert.equal(errors.length,0,JSON.stringify(errors));
  const screen=await call('Page.captureScreenshot',{format:'png'});await writeFile(join(dirname(report),'batch-preview.png'),Buffer.from(screen.data,'base64'));
  const result={initial,filtered,external_requests:external,javascript_errors:errors};await writeFile(join(dirname(report),'batch-browser-check.json'),JSON.stringify(result,null,2));process.stdout.write(JSON.stringify(result,null,2));
}finally{ws?.close();browser.kill();}
