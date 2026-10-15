"""Compare integrated CRCL reservoir measurements with the Analysis 003 baseline."""
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

OUT=ROOT/'docs/audio/multichannel/analysis004'
REPORT=OUT.with_suffix('.md')

def copy(source,destination):
    if source.resolve()!=destination.resolve():shutil.copy2(source,destination)

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--log',type=Path,required=True)
    parser.add_argument('--repeat-log',type=Path,required=True)
    parser.add_argument('--build-record',type=Path)
    parser.add_argument('--aardwolf-build-record',type=Path)
    args=parser.parse_args();OUT.mkdir(parents=True,exist_ok=True)
    a=read_log(args.log);b=read_log(args.repeat_log)
    for source,name in [(args.log,'target-run1.log'),(args.repeat_log,'target-run2.log')]:
        text=source.read_text(encoding='utf8')
        assert 'writer=reservoir32_single' in text
        assert 'hash=431565294 bytes=5183 shifts=[6, 6, 6, 6, 6] rice_k=[6, 6, 6, 6, 6]' in text
        copy(source,OUT/name)
    if args.build_record:copy(args.build_record,OUT/'benchmark-build.json')
    if args.aardwolf_build_record:copy(args.aardwolf_build_record,OUT/'aardwolf-build.json')
    build=json.loads((OUT/'benchmark-build.json').read_text())
    baseline=json.loads((ROOT/'docs/audio/multichannel/analysis003/cycles.json').read_text())
    old=baseline['run1'];repeat=max(abs(a[k]['cycles_per_block']-b[k]['cycles_per_block']) for k in a)
    def cycles(name):return a[name]['cycles_per_block']
    def before(name):return old[name]['cycles_per_block']
    def saving(name):return 100*(1-cycles(name)/before(name))
    result=dict(core_hz=HZ,frames=1024,auxiliary_channels=5,writer='checked_u32_reservoir_single_codeword',
        baseline_build=baseline['build'],build=build,run1=a,run2=b,max_median_difference_cycles=repeat,
        comparison={k:dict(before_cycles=before(k),after_cycles=cycles(k),saved_percent=saving(k)) for k in a},
        fixture_unchanged=True,record_bytes=5183,unit_tests_passed=13)
    (OUT/'cycles.json').write_text(json.dumps(result,indent=2)+'\n')
    with (OUT/'cycles.csv').open('w',newline='') as stream:
        writer=csv.writer(stream);writer.writerow(['stage','before_cycles','after_cycles','after_ms','saved_percent','batch','min','median','p90','max','repeat_median'])
        for k,r in a.items():writer.writerow([k,before(k),cycles(k),r['ms_per_block'],saving(k)]+[r[n] for n in ['batch','min','median','p90','max']]+[b[k]['median']])
    names=['production_transform_budget256','production_serialize','production_codec_budget256']
    fig,axes=plt.subplots(1,2,figsize=(12,5),constrained_layout=True)
    import numpy as np
    y=np.arange(3)
    axes[0].barh(y-.18,[before(k)/1e6 for k in names],height=.36,label='Before: byte writer')
    axes[0].barh(y+.18,[cycles(k)/1e6 for k in names],height=.36,label='After: checked u32 reservoir')
    axes[0].set_yticks(y,['Transform','Serialization','Complete loop']);axes[0].invert_yaxis()
    axes[0].set_xlabel('Million cycles / 1024 frames, MIC2–MIC6');axes[0].legend();axes[0].grid(axis='x',alpha=.2)
    axes[1].bar(['Transform','Serialization'],[cycles(k)/1e6 for k in names[:2]],color=['#629ec5','#dfac55'])
    axes[1].set(title='Remaining production cost',ylabel='Million cycles / block');axes[1].grid(axis='y',alpha=.2)
    fig.savefig(OUT/'before-after.png',dpi=160);plt.close(fig)
    total=cycles(names[2]);transform=cycles(names[0]);serialize=cycles(names[1])
    lines=['# Analysis 004: production Rice codewords and u32 reservoir','',
        '**Date:** 15 October 2026, Europe/London  ',
        '**Target:** Raylar v1.0 / STM32U595, 144 MHz, SMPS  ',
        '**Related:** [Analysis 003: baseline and stage pseudocode](analysis003.md)','',
        f'Production CRCL now emits common Rice codes with one append into a capacity-checked **u32 MSB-first reservoir**. On the same deterministic 1024-frame fixture, complete five-channel encoding falls from **{before(names[2]):,.0f} to {total:,.0f} cycles**: **{saving(names[2]):.1f}% fewer cycles**, {before(names[2])/HZ*1000:.3f} → **{total/HZ*1000:.3f} ms**. Serialization falls by **{saving(names[1]):.1f}%**. The DATA record remains exactly 5183 bytes, with identical packed payloads, quantization, Rice k and MIC1 samples.','',
        '![Before and after production cycle budget](analysis004/before-after.png)','',
        '## Production change','',
        'The old writer visited bytes as each field was emitted. It wrote unary zeros, a stop bit and the remainder separately. The production serializer now calls `bits.rice(folded_residual, k)`. For a Rice code that fits in 32 bits, its leading zeros are implicit in the width and one append emits the whole symbol. Longer codes append zero chunks, followed by the combined stop bit and remainder.','',
        '```text\nquotient = folded_residual >> k\nlength = checked_add(quotient, 1 + k)\nreject if length exceeds remaining output capacity\nremaining_capacity -= length\ncode = (1 << k) | (folded_residual & ((1 << k) - 1))\nif length <= 32:\n    append(code, length)\nelse:\n    append quotient zero bits in chunks no larger than 32\n    append(code, k + 1)\n```','',
        'The append operation accepts at most 32 bits and crosses at most one reservoir boundary. It has a fast path when the field fits and a flush path when a word fills. No loop is needed for a short codeword; shifts by 32 are explicitly avoided.','',
        '```text\nappend(value, width):\n    if width == 0: return\n    free = 32 - used\n    word |= (value << (32 - width)) >> used\n    if width < free:\n        used += width\n    else:\n        store_four_bytes_MSB_first(word)\n        used = width - free\n        word = 0 if used == 0 else value << (32 - used)\n\nfinish():\n    write only ceil(used / 8) bytes of the pending word\n    # Unused low bits are already zero: identical byte padding.\n```','',
        'The writer tracks remaining bit capacity as scalar state, avoiding repeated output-length multiplication for each field. Every field/code checks capacity before updating state. Widths >32, invalid k, overflowing Rice length and insufficient output return errors; no unchecked pointer stores are used. Full words and tails use safe byte-slice copies, so arbitrary payload start alignment is supported. The final write contains exactly ceil(pending_bits/8) bytes; bytes after the payload remain untouched.','',
        'Only scalar writer state changes: a u32 word, pending-bit count, remaining-bit count and flushed-byte position. On the 32-bit target, the writer structure adds 12 bytes relative to the previous writer; this is not a measured whole-function stack increase. There is no heap allocation, new PCM/residual array or u64 reservoir. Existing workspace capacities remain sufficient.','',
        'Both v1 and v2 use this internal writer, including warm-ups and packed v1 samples. File versions, metadata, reference handling, scale selection, rounded quantization, temporal prediction, Rice parameter estimation, sample alignment and payload format are unchanged. Aardwolf uses the library change through its existing cheap-codec configuration.','',
        '## Re-benchmark','',
        'Measurement conditions match Analysis 003: 1024 frames × six input channels, encode MIC2–MIC6; 500 Hz table sinusoid plus seeded noise; 10-bit target, min shift 6, per-channel k=6. DWT CYCCNT measures eight warm-ups followed by 129 runs. Interrupts are masked; no acquisition, SD, GPS, radio, FLAC or executor tasks run. Input preparation and logging are outside the spans. Two complete runs of the same new ELF passed every equality check.','',
        '| Production action | Before cycles | After cycles | Before ms | After ms | Fewer cycles |','|---|---:|---:|---:|---:|---:|']
    for name in ['production_peak_scale_phases','production_quant_stats_k_phases','production_transform','production_transform_budget256','production_serialize','production_codec_budget256']:
        lines.append(f'| {LABELS[name]} | {before(name):,.0f} | {cycles(name):,.0f} | {before(name)/HZ*1000:.3f} | {cycles(name)/HZ*1000:.3f} | {saving(name):.1f}% |')
    lines += ['',
        f'All cycles cover five auxiliary channels. Raw medians include timer/call overhead (empty baseline {cycles("empty"):.0f} cycles); scalar rows in the full CSV divide their batched counts by 256. Split peak/quantization rows are child measurements, not costs to add to their transform parent. Maximum difference between all stage medians across the two same-image runs: **{repeat:.0f} cycles**.','',
        f'The 64 ms block budget is 9,216,000 cycles. The isolated codec’s derived duty falls from **{before(names[2])/(HZ*.064)*100:.1f}% to {total/(HZ*.064)*100:.1f}%**. This excludes input copy, optional filtering, reference FLAC, SD and other application work. No current or energy saving was measured.','',
        'The transform algorithm was not edited, yet its timing shifted by about 6% between images. Unchanged byte-writer and copy kernels also moved in timing. Treat these as code-layout/cache/compiler-context variation, not extra algorithmic optimizations. The before/after figures are observations on the recorded ELF images, and the whole-loop reduction includes that variation. The serialization improvement is much larger than this effect.','',
        'The legacy comparison kernels deliberately retain the frozen byte writer. Rows named `rice_bytes_pre_folded_5ch`, `rice_single_codeword_5ch` and `payload_from_quant_5ch` therefore still measure the old writer, not the new production reservoir. `rice_words_codeword_5ch` remains the original unchecked-capacity prototype. **Use `production_serialize` and `production_codec_budget256` to judge the integrated change.** The new production implementation is faster on this fixture than that original reservoir prototype because its append path avoids the generic fragment loop and its Rice path combines field handling. Microkernel and production call contexts also differ, so their timings are not additive.','',
        '## Further optimization priorities','',
        f'1. **Refactor the resumable transform into bounded phase-specific inner loops.** It now accounts for {transform/total*100:.1f}% of the total. Peak/scale costs {cycles("production_peak_scale_phases"):,.0f} cycles and quantize/stats/k {cycles("production_quant_stats_k_phases"):,.0f}. The standalone checked peak scan is {cycles("peak_validate_5ch"):,.0f} cycles and the fused arithmetic kernel {cycles("quantize_delta_sum_fused_5ch"):,.0f}. Keep channel/offset/previous/sum/shift in locals during a chunk, and save scalar state only at chunk boundaries. Preserve resumability, range checks, ties-to-positive-infinity rounding and zero/partial block handling. These kernel gaps show headroom; they are not a guaranteed speedup.',
        f'2. **Move invariant serialization decisions outside the sample loop.** Serialization still costs {serialize:,.0f} cycles ({serialize/total*100:.1f}% of the total). A dedicated per-channel temporal path can handle the warm-up once and avoid generic representation dispatch for each sample. Check assembly before assuming a source expression, such as a mask or k conversion, is repeatedly evaluated.',
        f'3. **Consider peak collection while filling the existing interleaved block.** Input copy alone measures {cycles("copy_6ch"):,.0f} cycles and is not included in the codec total. Combining a necessary traversal with per-channel peak statistics could avoid a separate pass, but requires careful ownership/API design and a measured full-path comparison.',
        '4. **Defer lookup tables and parameter arithmetic.** The common Rice code is now emitted with shifts, a mask and an append. A table may replace only a few arithmetic instructions while adding loads, cache footprint and a fallback branch. Shift selection and Rice-k estimation remain a few hundred cycles across five channels. Measure any table build/storage cost and validate on quiet, tonal, noisy and transient blocks before adopting it.','',
        'No further transform or serialization-specialization changes are included in this iteration, so subsequent measurements can isolate those steps.','',
        '## Validation and saved evidence','',
        '- **13 desktop tests passed.** The frozen legacy writer is used to compare payload bytes for both CRCL versions, every representation, 6/8/10/12-bit precision, min shifts 0/6 and partial/full frame counts 1, 255, 256, 257, 1024 and 4096.',
        '- Field-level tests compare widths 0…32 at all pending bit offsets and all four byte alignments, with nonzero-filled outputs and sentinel bytes.',
        '- Rice tests cover k=0…31, zero/small/large residuals, the 32-/33-bit code boundary, long unary runs and u32 boundary values.',
        '- Capacity tests cover empty/exact/tail buffers, invalid widths/k, overflowing lengths and a valid continuation after an error. Padding and bytes outside the output slice are checked.',
        '- Both board runs preserve the fixture’s reference samples and reproduce all old byte-writer payloads and complete DATA headers exactly.',
        '- The normal release Aardwolf build also passes; it is built but not flashed. Its [build identity](analysis004/aardwolf-build.json) is saved. The board remains on the completed isolated benchmark, idle after PASS.','',
        'The legacy byte writer is compiled only with tests or `loop-benchmark`; normal Aardwolf builds contain the new writer. Production code is in [bits.rs](../../../crates/services/crclencoder/src/bits.rs) and the [serializer call site](../../../crates/services/crclencoder/src/format.rs). The quantizer and transform were unchanged.','',
        f'Benchmark ELF SHA-256: `{build["artifact_sha256"]}`. [Build identity](analysis004/benchmark-build.json), [first run](analysis004/target-run1.log), [repeat](analysis004/target-run2.log), [all stage cycles](analysis004/cycles.csv), [metrics](analysis004/cycles.json), [disassembly](analysis004/hot-functions.asm) and [manifest](analysis004/manifest.json) are preserved. The historical [Analysis 003 baseline](analysis003/cycles.json) is unchanged.','',
        '```powershell\n# Build, flash and profile only the CRCL loop:\nrtk proxy powershell -ExecutionPolicy Bypass -File scripts/run_crcl_loop_benchmark.ps1\n# Repeat without rebuilding:\nrtk proxy powershell -ExecutionPolicy Bypass -File scripts/firmware.ps1 -Package service-test-crclencoder -Bin crcl-loop-benchmark -NoBuild -MonitorSeconds 30 -Until \'CRCL LOOP BENCH PASS\'\n# Host checks:\nrtk cargo test -p raylar-crcl-encoder-service --features loop-benchmark --target x86_64-pc-windows-msvc\n# Recreate this report from saved logs:\nrtk proxy python scripts/crcl_reservoir_report.py --log docs/audio/multichannel/analysis004/target-run1.log --repeat-log docs/audio/multichannel/analysis004/target-run2.log\n```','',
        'For fresh measurements, pass the newly printed log paths and `--build-record firmware-builds/service-test-crclencoder.json`. Use `scripts/crcl_bench_inspect.py --output docs/audio/multichannel/analysis004` to regenerate assembly from that benchmark ELF without overwriting the historical Analysis 003 evidence.','']
    REPORT.write_text('\n'.join(lines),encoding='utf8')
    sources=['crates/services/crclencoder/src/bits.rs','crates/services/crclencoder/src/bits_legacy.rs',
        'crates/services/crclencoder/src/bits_tests.rs','crates/services/crclencoder/src/tests.rs',
        'crates/services/crclencoder/src/format.rs','crates/services/crclencoder/src/benchmark.rs',
        'scripts/crcl_reservoir_report.py','scripts/crcl_bench_inspect.py','servicetests/crclencoder/loop_benchmark.rs']
    manifest=dict(artifacts={p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(OUT.glob('*')) if p.is_file() and p.name!='manifest.json'},
        report_sha256=hashlib.sha256(REPORT.read_bytes()).hexdigest(),
        reproduction_source_sha256={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sources})
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    for target in re.findall(r'\]\(([^)]+)\)',REPORT.read_text(encoding='utf8')):assert (REPORT.parent/target).exists(),target
    print(json.dumps(dict(report=str(REPORT),total_cycles=total,total_ms=total/HZ*1000,
        total_saved_percent=saving(names[2]),serialize_saved_percent=saving(names[1]),max_repeat_median_difference_cycles=repeat),indent=2))

if __name__=='__main__':main()
