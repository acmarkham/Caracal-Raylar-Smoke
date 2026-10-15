"""Archive chunked-transform measurements and the same-image frozen controls."""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import re
import shutil
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt
from crcl_loop_report import ROOT,HZ,LABELS,read_log

OUT=ROOT/'docs/audio/multichannel/analysis005'
REPORT=OUT.with_suffix('.md')
EXTRA={'reference_transform_budget256':'Frozen pre-chunk transform, budget 256',
       'reference_codec_budget256':'Frozen pre-chunk transform + current serializer'}

def copy(source,destination):
    if source.resolve()!=destination.resolve():shutil.copy2(source,destination)

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--log',type=Path,required=True)
    parser.add_argument('--repeat-log',type=Path,required=True)
    parser.add_argument('--build-record',type=Path)
    parser.add_argument('--aardwolf-build-record',type=Path)
    args=parser.parse_args();OUT.mkdir(parents=True,exist_ok=True)
    a=read_log(args.log,EXTRA);b=read_log(args.repeat_log,EXTRA)
    for path,name in [(args.log,'target-run1.log'),(args.repeat_log,'target-run2.log')]:
        text=path.read_text(encoding='utf8')
        assert 'writer=reservoir32_single transform=chunked_v1' in text
        assert 'hash=431565294 bytes=5183 shifts=[6, 6, 6, 6, 6] rice_k=[6, 6, 6, 6, 6]' in text
        copy(path,OUT/name)
    if args.build_record:copy(args.build_record,OUT/'benchmark-build.json')
    if args.aardwolf_build_record:copy(args.aardwolf_build_record,OUT/'aardwolf-build.json')
    build=json.loads((OUT/'benchmark-build.json').read_text())
    baseline=json.loads((ROOT/'docs/audio/multichannel/analysis004/cycles.json').read_text())
    historical=baseline['run1']
    def cycles(key):return a[key]['cycles_per_block']
    def old(key):return historical[key]['cycles_per_block']
    total=cycles('production_codec_budget256');transform=cycles('production_transform_budget256')
    ref_total=cycles('reference_codec_budget256');ref_transform=cycles('reference_transform_budget256')
    serialize=cycles('production_serialize')
    repeat=max(abs(a[k]['cycles_per_block']-b[k]['cycles_per_block']) for k in a)
    result=dict(core_hz=HZ,frames=1024,auxiliary_channels=5,repeats=129,warmups=8,
        build=build,baseline_build=baseline['build'],run1=a,run2=b,max_repeat_median_difference_cycles=repeat,
        same_image_transform_reduction_percent=100*(1-transform/ref_transform),
        same_image_complete_loop_reduction_percent=100*(1-total/ref_total),
        historical_transform_reduction_percent=100*(1-transform/old('production_transform_budget256')),
        historical_complete_loop_reduction_percent=100*(1-total/old('production_codec_budget256')),
        output_record_bytes=5183,unit_tests_passed=15,valid_state_and_byte_comparison_cases=867,error_progress_cases=120)
    (OUT/'cycles.json').write_text(json.dumps(result,indent=2)+'\n')
    with (OUT/'cycles.csv').open('w',newline='') as stream:
        w=csv.writer(stream);w.writerow(['stage','batch','raw_min','raw_median','raw_p90','raw_max','cycles_per_block','ms_per_block','historical_analysis004_cycles','repeat_median'])
        for k,r in a.items():w.writerow([k]+[r[n] for n in ['batch','min','median','p90','max','cycles_per_block','ms_per_block']]+[historical.get(k,{}).get('cycles_per_block',''),b[k]['median']])
    fig,axes=plt.subplots(1,2,figsize=(12,5),constrained_layout=True)
    import numpy as np
    y=np.arange(2)
    axes[0].barh(y-.18,[ref_transform/1000,ref_total/1000],height=.36,label='Frozen prior transform, same ELF')
    axes[0].barh(y+.18,[transform/1000,total/1000],height=.36,label='Chunked production transform')
    axes[0].set_yticks(y,['Transform, budget 256','Complete encoding loop']);axes[0].invert_yaxis()
    axes[0].set_xlabel('Thousands of cycles / 1024 frames, MIC2–MIC6');axes[0].legend();axes[0].grid(axis='x',alpha=.2)
    axes[1].bar(['Peak + scale','Quantize + stats + k','Serialization'],
        [cycles(k)/1000 for k in ['production_peak_scale_phases','production_quant_stats_k_phases','production_serialize']],color=['#629ec5','#87bd92','#dfac55'])
    axes[1].tick_params(axis='x',labelrotation=15);axes[1].set(title='New production stage cost',ylabel='Thousands of cycles / block');axes[1].grid(axis='y',alpha=.2)
    fig.savefig(OUT/'cycle-budget.png',dpi=160);plt.close(fig)
    lines=['# Analysis 005: bounded production transform loops','',
        '**Date:** 15 October 2026, Europe/London  ',
        '**Target:** Raylar v1.0 / STM32U595, 144 MHz, SMPS  ',
        '**Scope:** implement recommendation 1 from [Analysis 004](analysis004.md); keep the reservoir serializer and codec semantics unchanged.','',
        f'The cheap v2 transform now runs a bounded peak loop or a bounded quantization/statistics loop, keeping working state local until each chunk completes. Compared with a frozen previous transform in the **same firmware image**, its cost falls from **{ref_transform:,.0f} to {transform:,.0f} cycles** ({result["same_image_transform_reduction_percent"]:.1f}% fewer cycles). Complete encoding falls from **{ref_total:,.0f} to {total:,.0f} cycles**, **{ref_total/HZ*1000:.3f} → {total/HZ*1000:.3f} ms**, a **{result["same_image_complete_loop_reduction_percent"]:.1f}% reduction**. Output remains byte-identical.','',
        '![Same-image control and production stage budget](analysis005/cycle-budget.png)','',
        '## Implementation','',
        'The old loop visited one scalar work item at a time and repeatedly accessed phase, channel, offset, shift, peak, previous q and sum through the Transform state. It checked phase completion and decremented the caller budget after each scalar sample. The new outer loop calculates one bounded interval for the current channel and phase, then uses a stride-six slice iterator for that channel. MIC1 is never selected.','',
        '```text\nwhile budget > 0 and channel < 6:\n    start = offset\n    count = min(budget, frames - start)\n    end = start + count\n    samples = pcm[start*6 + channel : end*6], stride 6\n\n    if phase == PEAK:\n        local_peak = peak\n        local_bound = max_abs_pcm\n        for n, x in samples:\n            if x outside [-local_bound, local_bound]:\n                offset = start + n\n                peak = local_peak\n                return SampleOutOfRange\n            local_peak = max(local_peak, abs(x))\n        peak = local_peak\n    else:\n        local_shift = shift\n        half = 0 if local_shift == 0 else 1 << (local_shift - 1)\n        local_previous = previous\n        local_sum = sum_folded\n        for n, sample_slot in samples:\n            q = (sample_slot + half) >> local_shift\n            overwrite sample_slot with q\n            if start + n != 0:\n                local_sum += fold(q - local_previous)\n            local_previous = q\n        previous = local_previous\n        sum_folded = local_sum\n\n    offset = end\n    budget -= count\n    if end == frames:\n        offset = 0\n        if phase == PEAK:\n            shift = max(shift_for_peak(peak, precision), min_shift)\n            phase = QUANTIZE\n        else:\n            store channel shift, temporal mode and estimated Rice k\n            move to next channel and reset scalar state\n```','',
        'The caller’s budget still counts scalar visits, not frames across all channels: a full block requires 2 × 5 × frames work items. `advance(256)` therefore retains its existing work limit and call count. A call can cross a phase or channel boundary only with unused budget. A zero budget does no work; completion remains idempotent. Each chunk stores persistent state before returning.','',
        'On a range failure, the new loop commits the peak and offset of the valid prefix but leaves the rejected sample unconsumed. This matches the previous state exactly, including when an earlier channel has already been quantized. Range checks precede abs(), so i32::MIN is rejected safely. Rounding still uses one add and arithmetic shift, with ties towards positive infinity; shift-zero and the minimum-discard rule are unchanged. No u64 PCM arithmetic, heap allocation, new sample arrays or residual buffers are introduced.','',
        'The first quantized sample still has no temporal residual; subsequent samples continue from the saved previous q across chunks. Per-sample profiling keeps the same absolute sample indices and stage assignments, and normal builds compile those probes away. V1’s transform and the v1/v2 serializers were not edited in this iteration.','',
        '## Correctness checks','',
        '**15 host tests passed.** A test/benchmark-only frozen copy of the previous cheap transform provides the equivalence reference. Normal application builds exclude that copy. New checks compare all transformed PCM, every persistent scalar field, all channel descriptors, return values and complete encoded record buffers, rather than only checking successful decode.','',
        '| Coverage | Cases |','|---|---|',
        '| Valid state/output comparisons | 867 input/configuration/budget scenarios |',
        '| Frame counts | 1, 7, 255, 256, 257, 1023, 1024, 1025, 4096 |',
        '| Precision / minimum shift | 6, 8, 10, 12 bits; min shift 0, 6, 30 |',
        '| Inputs | Silence, constant, alternating extrema, deterministic noise; differing amplitudes per channel |',
        '| Budgets | Zero, one-item, fixed 256, mixed sizes around phase boundaries, and usize::MAX |',
        '| State comparison | After every advance, including completed calls with zero/large budgets |',
        '| MIC1 | Alternating i32::MIN/MAX, verified unchanged after every successful advance |',
        '| Maximum legal bound | Additional tests with max_abs_pcm=i32::MAX/2 |',
        '| Range failures | 120 scenarios across all auxiliary channels and sample positions 0, 1, 112, 113, 255, 256 |',
        '| Failure values | Values just outside each configured bound, i32::MIN and i32::MAX |',
        '| Failure continuation | Exact cursor/peak/state retained; zero-budget and repeated failing calls compared |','',
        'Existing payload tests against the frozen byte writer, Rice reservoir boundary/capacity tests and codec roundtrips also pass. Both hardware runs preserve MIC1 and reproduce the original 5183-byte DATA record, shifts=[6,6,6,6,6], k=[6,6,6,6,6] and payload byte counts [989,1008,1024,1034,1040]. The normal Aardwolf release build passes.','',
        '## Hardware timing','',
        'The same deterministic 1024-frame sinusoid/noise fixture and measurement conditions from Analyses 003/004 are retained: 16 kHz, five auxiliary channels, 10-bit target, min shift 6, 144 MHz/SMPS, eight warm-ups and 129 measured runs per row. DWT CYCCNT is used with interrupts masked. No microphone capture, SD, GPS, FLAC, radio or executor tasks run during measurement. Preparation, logging and equality checks are outside measured spans. Raw medians include timer/call overhead; split phases are children of the transform, not costs to add to it.','',
        '### Same-image A/B comparison','',
        '| Measurement | Previous transform control | Chunked production | Previous ms | New ms | Fewer cycles |','|---|---:|---:|---:|---:|---:|']
    for title,ref,new in [('Transform, budget 256','reference_transform_budget256','production_transform_budget256'),('Complete loop, budget 256','reference_codec_budget256','production_codec_budget256')]:
        r=cycles(ref);n=cycles(new);lines.append(f'| {title} | {r:,.0f} | {n:,.0f} | {r/HZ*1000:.3f} | {n/HZ*1000:.3f} | {100*(1-n/r):.1f}% |')
    lines += ['',
        'Both controls share the same fixture, reset method, current reservoir serializer, configuration and image as the production measurements. The frozen control calls its internal cheap transform directly; production goes through the normal `Transform::advance` dispatch. Normal profiling probes compile away. This control reduces between-build code-layout variation; the two functions still occupy different addresses and may have different compiler/caching effects. The previous control closely matches the independently archived pre-change result below.','',
        '### Historical production comparison','',
        '| Action | Analysis 004 cycles | New cycles | New ms |','|---|---:|---:|---:|']
    for k in ['production_peak_scale_phases','production_quant_stats_k_phases','production_transform','production_transform_budget256','production_serialize','production_codec_budget256']:
        lines.append(f'| {LABELS[k]} | {old(k):,.0f} | {cycles(k):,.0f} | {cycles(k)/HZ*1000:.3f} |')
    lines += ['',
        f'Against the archived prior production image, the transform is **{result["historical_transform_reduction_percent"]:.1f}%** cheaper and the full loop **{result["historical_complete_loop_reduction_percent"]:.1f}%** cheaper. Serialization is essentially unchanged ({old("production_serialize"):,.0f} → {serialize:,.0f} cycles), making the attribution clearer than in the previous packing iteration.','',
        f'Two separate runs of this exact ELF reproduced **all stage medians**, maximum median difference {repeat:.0f} cycles. Raw min/median/p90/max values and all primitive/control rows are saved in [cycles.csv](analysis005/cycles.csv) and [cycles.json](analysis005/cycles.json). Historical evidence remains unchanged.','',
        f'The 64 ms capture-block budget is 9,216,000 cycles. The complete isolated codec consumes **{total/(HZ*.064)*100:.1f}%** of that budget, compared with {ref_total/(HZ*.064)*100:.1f}% for the same-image old transform. These percentages are derived compute duty, not measured application utilization or current. Input copy, optional filtering, reference FLAC, storage and other tasks remain excluded.','',
        '## Remaining optimization avenues','',
        f'1. **Specialize the temporal serialization loop next.** Serialization is now {serialize/total*100:.1f}% of the total ({serialize:,.0f} cycles), with the transform {transform/total*100:.1f}%. Consider moving representation selection outside each sample loop and handling the first signed warm-up separately, while preserving v1 representations and the current checked reservoir. Inspect assembly before assuming source-level invariants still execute repeatedly.',
        f'2. **Compare alternate safe strided loops and warm-up peeling inside quantization.** The chunked peak/scale phases cost {cycles("production_peak_scale_phases"):,.0f} cycles, and quantize/stats/k {cycles("production_quant_stats_k_phases"):,.0f}. Specialized standalone kernels remain cheaper, but have different bounds and compiler context. The current quantization loop still distinguishes the first sample inside its inner loop; a separate first-sample case may help if not already peeled by the compiler. Any further change must pass the same state/error equivalence matrix.',
        '3. **Peak collection during input copy remains an API-level experiment.** It could avoid a traversal of the same interleaved data, but was deliberately excluded from this narrowly scoped change.',
        '4. **Keep cooperative limits and avoid extra buffers.** Increasing the budget to unlimited saves only a small fraction of the new transform cost and would change scheduling responsiveness. Rice-k/scale arithmetic and container header writes remain low priority; lookup tables still need evidence of a net benefit.','',
        '## Artifacts and reproduction','',
        f'Benchmark ELF SHA-256: `{build["artifact_sha256"]}`. See [build identity](analysis005/benchmark-build.json), [Aardwolf build identity](analysis005/aardwolf-build.json), [first run](analysis005/target-run1.log), [repeat](analysis005/target-run2.log), [assembly](analysis005/hot-functions.asm), and [artifact manifest](analysis005/manifest.json). The original fixture is in [Analysis 003](analysis003/fixture.json).','',
        'Source: [chunked transform](../../../crates/services/crclencoder/src/cheap.rs), [frozen reference](../../../crates/services/crclencoder/src/cheap_reference.rs), [state/error equivalence tests](../../../crates/services/crclencoder/src/cheap_tests.rs), [benchmark controls](../../../crates/services/crclencoder/src/benchmark.rs). The board remains idle on the completed benchmark. Aardwolf is built with the new production transform but has not been flashed.','',
        '```powershell\nrtk cargo test -p raylar-crcl-encoder-service --features loop-benchmark --target x86_64-pc-windows-msvc\nrtk proxy powershell -ExecutionPolicy Bypass -File scripts/run_crcl_loop_benchmark.ps1\n# Repeat the same ELF:\nrtk proxy powershell -ExecutionPolicy Bypass -File scripts/firmware.ps1 -Package service-test-crclencoder -Bin crcl-loop-benchmark -NoBuild -MonitorSeconds 30 -Until \'CRCL LOOP BENCH PASS\'\n# Regenerate the report from archived logs, without a board:\nrtk proxy python scripts/crcl_transform_report.py --log docs/audio/multichannel/analysis005/target-run1.log --repeat-log docs/audio/multichannel/analysis005/target-run2.log\n```','',
        'For fresh measurements pass the new log paths plus `--build-record firmware-builds/service-test-crclencoder.json` and `--aardwolf-build-record firmware-builds/aardwolf.json`. Save assembly with `scripts/crcl_bench_inspect.py --output docs/audio/multichannel/analysis005`. The shared log parser now accepts explicit extra stage names, while historical report scripts continue to validate their original stage sets.','']
    REPORT.write_text('\n'.join(lines),encoding='utf8')
    sources=['crates/services/crclencoder/src/cheap.rs','crates/services/crclencoder/src/cheap_reference.rs',
        'crates/services/crclencoder/src/cheap_tests.rs','crates/services/crclencoder/src/benchmark.rs',
        'servicetests/crclencoder/loop_benchmark.rs','scripts/crcl_loop_report.py','scripts/crcl_transform_report.py']
    manifest=dict(artifacts={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(OUT.glob('*')) if p.is_file() and p.name!='manifest.json'},
        report_sha256=hashlib.sha256(REPORT.read_bytes()).hexdigest(),reproduction_sources_sha256={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sources})
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    for target in re.findall(r'\]\(([^)]+)\)',REPORT.read_text(encoding='utf8')):assert (REPORT.parent/target).exists(),target
    print(json.dumps({k:v for k,v in result.items() if k not in ['build','baseline_build','run1','run2']},indent=2))

if __name__=='__main__':main()
