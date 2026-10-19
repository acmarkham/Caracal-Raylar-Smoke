// Optional dependency-free Chromium/CDP smoke test (Node.js 22+).
// Usage: node browser_smoke.mjs report.html [chrome.exe]
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
  let ready=false;
  for(let i=0;i<150;i++){
    ready=await evaluate("document.body?.dataset.reportReady==='true'");
    if(ready)break;
    await sleep(100);
  }
  assert(ready,'Report did not finish rendering');
  const initial=await evaluate(`({plots:activePlots.length,error:document.getElementById('error').textContent,
    boot:report.runs[Number(select.value)].boot_id,tx:report.runs[Number(select.value)].summary.radio_tx,
    sections:document.querySelectorAll('main section').length})`);
  assert(initial.plots>10,'Expected multiple telemetry charts');assert.equal(initial.error,'');
  const linked=await evaluate(`(async()=>{
    const plots=activePlots.filter(p=>p.timeAxis),a=plots[0],b=plots[1];
    const range=['2026-10-18T18:00:00Z','2026-10-18T19:00:00Z'];
    await Plotly.relayout(a.node,{'xaxis.range[0]':range[0],'xaxis.range[1]':range[1]});
    await new Promise(r=>setTimeout(r,300));
    return {range:b.node.layout.xaxis.range,linked:!b.node.layout.xaxis.autorange};
  })()`);
  assert(linked.linked,'Linked time zoom failed');
  await evaluate(`(async()=>{select.value='0';await showRun();return true})()`);
  const first=await evaluate(`({plots:activePlots.length,boot:report.runs[Number(select.value)].boot_id,error:document.getElementById('error').textContent})`);
  assert.equal(first.error,'');
  await evaluate(`(async()=>{select.value=String(report.runs.length-1);await showRun();return true})()`);
  const restored=await evaluate("document.getElementById('error').textContent");assert.equal(restored,'');
  await evaluate("document.getElementById('measured-pps-drift').scrollIntoView();true");
  const screen=await call('Page.captureScreenshot',{format:'png'});
  await writeFile(join(dirname(report),'report-drift-preview.png'),Buffer.from(screen.data,'base64'));
  const location=await evaluate(`(()=>{
    const sec=document.getElementById('device-location');sec.scrollIntoView();
    const chart=sec.querySelector('.js-plotly-plot'),p=chart?.data[0];
    return {wrapped:sec.textContent.includes('Wrapped X'),
      in_range:p ? p.x.every(v=>v>=0&&v<50000)&&p.y.every(v=>v>=0&&v<50000) : null};
  })()`);
  if(location.in_range!==null)
    assert(location.wrapped&&location.in_range,'Expected obfuscated location coordinates within the 50 km grid');
  const locationScreen=await call('Page.captureScreenshot',{format:'png'});
  await writeFile(join(dirname(report),'report-location-preview.png'),Buffer.from(locationScreen.data,'base64'));
  const external=requests.filter(u=>/^https?:/.test(u));
  assert.equal(external.length,0,'Report made external network requests');
  assert.equal(errors.length,0,'Browser JavaScript errors');
  const result={initial,first_boot:first,linked_zoom:linked,location,external_requests:external,javascript_errors:errors};
  await writeFile(join(dirname(report),'browser-check.json'),JSON.stringify(result,null,2)+'\n');
  process.stdout.write(JSON.stringify(result,null,2)+'\n');
}finally{
  ws?.close();browser.kill();
}
