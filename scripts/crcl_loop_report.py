"""Archive and report the isolated CRCL DWT loop benchmark."""
import argparse
import csv
import hashlib
import json
from pathlib import Path
import re
import shutil
import struct
import subprocess
import numpy as np
import matplotlib
matplotlib.use('Agg')
import matplotlib.pyplot as plt

ROOT=Path(__file__).resolve().parents[1]
OUT=ROOT/'docs/audio/multichannel/analysis003'
REPORT=OUT.with_suffix('.md')
HZ=144_000_000
LABELS={
 'empty':'Empty timer/call baseline', 'copy_6ch':'Copy 1024 × 6 i32 values',
 'optional_hpf_5ch':'Optional first-order HPF', 'peak_validate_5ch':'Peak scan + range validation',
 'scale_select_5ch':'Select power-of-two scale (CLZ)', 'truncate_shift_5ch':'Shift/truncate in place (comparison only)',
 'round_quantize_5ch':'Round and quantize in place', 'temporal_delta_5ch':'Temporal first difference → diagnostic buffer',
 'sum_pre_folded_5ch':'Sum already-folded residuals', 'fold_sum_residual_5ch':'Fold + sum precomputed residuals',
 'rice_k_5ch':'Estimate Rice k (division + CLZ)', 'quantize_delta_sum_fused_5ch':'Fused quantization + delta + fold + sum',
 'rice_bytes_pre_folded_5ch':'Current byte writer, pre-folded input',
 'rice_single_codeword_5ch':'Single codeword → current byte writer',
 'rice_words_codeword_5ch':'Single codeword → 32-bit reservoir',
 'payload_from_quant_5ch':'Specialized temporal payload → byte writer',
 'crcl_header_only':'CRCL DATA header + descriptors only',
 'crcl_header_payload_copy':'Header + copy existing payload (diagnostic)',
 'production_transform':'Production Transform, unlimited budget',
 'production_transform_budget256':'Production Transform, budget 256',
 'production_serialize':'Production Encoder::data',
 'production_codec_budget256':'Production Transform + Encoder::data, budget 256',
 'production_peak_scale_phases':'Production peak/scale passes (five spans)',
 'production_quant_stats_k_phases':'Production quantize/stats/k passes (five spans)',
}

def read_log(path):
    text=path.read_text(encoding='utf8')
    assert 'CRCL LOOP BENCH PASS' in text
    assert 'interrupts=off' in text and 'repeats=129' in text
    rows={}
    pattern=r'CRCL_LOOP_ROW stage=(\w+) batch=(\d+) min=(\d+) median=(\d+) p90=(\d+) max=(\d+) checksum=(\d+)'
    for name,*fields in re.findall(pattern,text):
        assert name not in rows
        r=dict(zip(['batch','min','median','p90','max','checksum'],map(int,fields)))
        r['cycles_per_block']=r['median']/r['batch']
        r['ms_per_block']=r['cycles_per_block']/HZ*1000
        rows[name]=r
    assert rows.keys()==LABELS.keys(),(rows.keys()^LABELS.keys())
    return rows

def fixture():
    # Independent deterministic generator, including Rust's truncating division.
    sine=[0,6393,12539,18204,23170,27245,30273,32137,32767,32137,30273,27245,23170,18204,12539,6393,0,-6393,-12539,-18204,-23170,-27245,-30273,-32137,-32767,-32137,-30273,-27245,-23170,-18204,-12539,-6393]
    state=0x12345678;values=[]
    for n in range(1024):
        for c in range(6):
            state^=(state<<13)&0xffffffff;state^=state>>17;state^=(state<<5)&0xffffffff
            product=sine[(n+c*2)%32]*(20000+c*1000)
            value=(product//32768 if product>=0 else -((-product)//32768))+(state&255)-128
            values.append(value)
    x=np.array(values,dtype='<i4').reshape(-1,6)
    q=(x.astype(np.int64)+32)>>6
    delta=np.diff(q[:,1:],axis=0)
    folded=np.where(delta>=0,2*delta,-2*delta-1)
    sums=folded.sum(axis=0)
    k=[int((int(s)+1022)//1023).bit_length()-1 for s in sums]
    lengths=[(10+int(np.sum((folded[:,c]>>k[c])+1+k[c]))+7)//8 for c in range(5)]
    h=2166136261
    for byte in x.tobytes():h=((h^byte)*16777619)&0xffffffff
    assert h==431565294 and k==[6]*5 and lengths==[989,1008,1024,1034,1040]
    x.tofile(OUT/'fixture.i32le')
    return dict(fnv1a32=h,sha256=hashlib.sha256(x.tobytes()).hexdigest(),
        frames=1024,channels=6,sample_rate_hz=16000,sine_hz=500,
        amplitudes_counts=[20000,21000,22000,23000,24000,25000],noise_range_counts=[-128,127],
        seed='0x12345678',shifts=[6]*5,k=k,folded_sums=sums.tolist(),payload_bytes=lengths,
        record_bytes=88+sum(lengths),folded_max=int(folded.max()),
        folded_under_64_percent=100*float(np.mean(folded<64)),folded_under_128_percent=100*float(np.mean(folded<128)),
        folded_under_256_percent=100*float(np.mean(folded<256)))

def copy(src,dst):
    if src.resolve()!=dst.resolve():shutil.copy2(src,dst)

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--log',type=Path,required=True)
    parser.add_argument('--repeat-log',type=Path,required=True)
    parser.add_argument('--build-record',type=Path)
    args=parser.parse_args();OUT.mkdir(parents=True,exist_ok=True)
    a=read_log(args.log);b=read_log(args.repeat_log)
    copy(args.log,OUT/'target-run1.log');copy(args.repeat_log,OUT/'target-run2.log')
    build_path=OUT/'benchmark-build.json'
    if args.build_record:copy(args.build_record,build_path)
    assert build_path.exists()
    build=json.loads(build_path.read_text())
    f=fixture();(OUT/'fixture.json').write_text(json.dumps(f,indent=2)+'\n')
    repeat=max(abs(a[k]['cycles_per_block']-b[k]['cycles_per_block']) for k in a)
    result=dict(core_hz=HZ,block_frames=1024,aux_channels=5,repeats=129,warmups=8,
                build=build,fixture=f,run1=a,run2=b,max_median_difference_cycles=repeat,
                compiler=subprocess.check_output(['rustc','--version'],text=True).strip(),
                interpretation='Raw median divided by batch; no empty-baseline subtraction. Standalone kernels overlap; only production parents are totals.')
    (OUT/'cycles.json').write_text(json.dumps(result,indent=2)+'\n')
    with (OUT/'cycles.csv').open('w',newline='') as stream:
        writer=csv.writer(stream);writer.writerow(['stage','batch','raw_min','raw_median','raw_p90','raw_max','cycles_per_block','ms_per_block','repeat_cycles_per_block'])
        for name,r in a.items():writer.writerow([name]+[r[key] for key in ['batch','min','median','p90','max','cycles_per_block','ms_per_block']]+[b[name]['cycles_per_block']])
    def cycles(name):return a[name]['cycles_per_block']
    total=cycles('production_codec_budget256'); transform=cycles('production_transform_budget256');emit=cycles('production_serialize')
    byte=cycles('rice_bytes_pre_folded_5ch');single=cycles('rice_single_codeword_5ch');word=cycles('rice_words_codeword_5ch')
    fig,axes=plt.subplots(1,2,figsize=(13,6),constrained_layout=True)
    names=['production_peak_scale_phases','production_quant_stats_k_phases','production_serialize']
    axes[0].barh(['Peak + scale, production','Quantize + stats + k, production','Serialize, production'],[cycles(v)/1000 for v in names],color=['#629ec5','#87bd92','#dfac55'])
    axes[0].set(xlabel='Thousands of cycles / 1024 frames, five auxiliary microphones',title='Production work: serialization dominates')
    axes[1].barh(['Byte writer, separate fields','Byte writer, one codeword','32-bit reservoir, one codeword'],[byte/1000,single/1000,word/1000],color=['#629ec5','#87bd92','#dfac55'])
    axes[1].set(xlabel='Thousands of cycles / identical five-channel payload',title='Test-only packing candidates: byte-identical fixture')
    for ax in axes:ax.grid(axis='x',alpha=.2);ax.invert_yaxis()
    fig.savefig(OUT/'cycle-budget.png',dpi=160);plt.close(fig)
    lines=['# Analysis 003: isolated CRCL loop cycle budget','',
        '**Date:** 15 October 2026 (Europe/London)  ',
        '**Target:** Raylar v1.0 / STM32U595 Cortex-M33, 144 MHz, SMPS  ',
        '**Scope:** one 1024-frame six-microphone buffer; encode MIC2–MIC6 with the cheap CRCL v2 algorithm, leave MIC1 unchanged. No microphone, FLAC, SD, GPS, radio or concurrent application tasks run.','',
        '## Findings','',
        f'The existing production transform plus DATA-record serialization costs **{total:,.0f} cycles = {total/HZ*1000:.3f} ms** for the five extra channels. A 1024-frame block at 16 kHz spans 64 ms, so this isolated workload consumes **{total/(HZ*.064)*100:.1f}%** of the nominal core budget. This is a cycle-derived compute duty estimate, not a measured application utilization or energy figure. Input-buffer copying and the optional high-pass filter are outside that total.','',
        f'Production serialization costs **{emit:,.0f} cycles ({emit/total*100:.1f}%)**; the production transform costs **{transform:,.0f} cycles ({transform/total*100:.1f}%)**. Rice-k estimation and scale selection each cost only a few hundred cycles across five channels. The CRCL header itself is also only a few hundred cycles. The main opportunities are the resumable transform’s inner loop and Rice bit emission, rather than parameter calculations or container metadata stores.','',
        f'Test-only packing kernels produced byte-identical output: combining Rice unary/remainder fields into one codeword saves **{100*(1-single/byte):.1f}%** against the isolated byte-writer baseline; using a 32-bit reservoir with that codeword saves **{100*(1-word/byte):.1f}%** ({byte/word:.2f}× faster). These are isolated packing comparisons, **not measured end-to-end speedups**, and the reservoir prototype has a narrower capacity-checking contract than production. No production codec behavior was changed.','',
        '![Measured cycle budget and packing candidates](analysis003/cycle-budget.png)','',
        '## Fixture and measurement method','',
        'The fixture is a deterministic 500 Hz sinusoid generated from a fixed 32-entry Q15 table at 16 kHz, plus uniform integer pseudonoise −128…127 counts. For channels 1…6 the amplitudes are 20,000…25,000 counts and the table phase offset is two samples per channel. A single xorshift32 generator, seed `0x12345678`, advances once per interleaved scalar sample. Generation, fixture validation and data reset/copy for destructive stages occur outside timed spans. The explicit copy benchmark is the exception: that row deliberately measures copying.','',
        'The production configuration is 1024 frames, signed 10-bit target, minimum shift 6, independent temporal prediction, estimated Rice k, and no CRC or SHA. All five channels select shift 6 and k=6 on this fixture. The resulting DATA record is **5183 bytes**, including an 88-byte header and five byte-padded payloads of 989, 1008, 1024, 1034 and 1040 bytes. The payload averages 7.96 bits per auxiliary-channel sample; fixed 10-bit packing would take 6400 payload bytes. This sinusoid is a reproducible profiling input, not a representative outdoor compression corpus or a worst-case test.','',
        f'Canonical interleaved little-endian i32 fixture SHA-256: `{f["sha256"]}`. The target reports FNV-1a32 `{f["fnv1a32"]}`; the independent host generator reproduces that identity, shifts, k values and payload lengths. The [raw fixture](analysis003/fixture.i32le) and [parameters](analysis003/fixture.json) are saved.','',
        'The dedicated synchronous binary initializes the normal 144 MHz clock tree and SMPS, enables DWT CYCCNT, then masks interrupts. It starts no executor or acquisition/storage tasks. The heap remains uninitialized; benchmark buffers are static. Each row gets eight warm-ups and 129 measured runs. Input/output are passed through `black_box` and compiler fences prevent reordering across timestamps. Logging, sorting, preparation and byte comparisons occur outside the measured region. Normal stage probes compile away; there are no per-sample profiling calls in the production path.','',
        f'Tables use **raw median cycles / batch**, without subtracting overhead. The empty call/timer baseline is {cycles("empty"):.0f} cycles. Scale and k rows batch 256 sets of five parameter calculations per span, then divide by 256. The split production-pass rows accumulate five DWT spans; their small instrumentation overhead is retained. Raw min/median/p90/max counts are in [cycles.csv](analysis003/cycles.csv) and [cycles.json](analysis003/cycles.json).','',
        f'Two separate runs of the **same ELF** produced a maximum median difference of **{repeat:.0f} cycles**; within-run maxima differ from medians by at most a few cycles. An earlier build with a different code layout measured the whole loop at 2.583 million cycles (~2.4% higher). Do not treat deterministic same-image timing as a guarantee across code layout, memory/cache conditions or different input distributions. Defmt timestamps are not used: the Embassy time driver cannot maintain its normal overflow accounting while interrupts are masked. DWT spans are short enough to avoid a full counter wrap (about 29.8 s at 144 MHz).','',
        '## Production totals','',
        'All rows below cover five auxiliary microphones, each with 1024 samples. “Per channel” is the five-channel average, not an independently timed mono encoder. The production serializer row includes creating an Encoder and ready Transform shell; sample transformation and input copying are excluded. The total row measures the real Transform and Encoder::data API together, with 256 scalar work items per advance and no async yields.','',
        '| Production measurement | Cycles / block | ms / block | Cycles / channel-block |','|---|---:|---:|---:|']
    for name in ['production_peak_scale_phases','production_quant_stats_k_phases','production_transform','production_transform_budget256','production_serialize','production_codec_budget256']:
        value=cycles(name);lines.append(f'| {LABELS[name]} | {value:,.0f} | {value/HZ*1000:.3f} | {value/5:,.1f} |')
    lines += ['',
        'The split peak and quantization passes are measured by advancing exactly 1024 work items at each phase boundary. Peak includes range checking and selecting the shift; quantization includes temporal differences, folding, residual sum, and k selection. Their sum is close to the unsplit transform but is a separately instrumented measurement. **Do not add these children to the transform parent.** Likewise, serialization already includes residual recomputation, Rice writing, padding and header/descriptor stores.','',
        '## Basic action costs','',
        'These standalone kernels separate arithmetic and packing mechanisms. They share the same fixture, strided six-channel layout where relevant, and encoder crate optimization level. Some use precomputed diagnostic arrays to isolate one action; those arrays are **not a proposed production memory requirement**. Independent kernels have different bounds-checking, specialization, register allocation and unrolling opportunities, so their costs do not add up to the production state machine.','',
        '| Action | Cycles / five-channel block | ms / block | Cycles / 1024-sample channel |','|---|---:|---:|---:|']
    kernel_names=['optional_hpf_5ch','peak_validate_5ch','scale_select_5ch','truncate_shift_5ch','round_quantize_5ch','temporal_delta_5ch','sum_pre_folded_5ch','fold_sum_residual_5ch','rice_k_5ch','quantize_delta_sum_fused_5ch','rice_bytes_pre_folded_5ch','rice_single_codeword_5ch','rice_words_codeword_5ch','payload_from_quant_5ch']
    for name in kernel_names:
        value=cycles(name);lines.append(f'| {LABELS[name]} | {value:,.1f} | {value/HZ*1000:.4f} | {value/5:,.1f} |')
    lines+=['','| Data movement / container action | Cycles | ms |','|---|---:|---:|']
    for name in ['copy_6ch','crcl_header_only','crcl_header_payload_copy']:
        value=cycles(name);lines.append(f'| {LABELS[name]} | {value:,.0f} | {value/HZ*1000:.4f} |')
    lines += ['',
        'The plain shift row is included to distinguish truncation from the current rounded quantizer; truncation is **not** a suggested silent codec change. The optional HPF is also **not** in the current encoder. The header-only kernel writes the equivalent 88-byte v2 DATA header/descriptors with an existing payload; it is not an invocation of the entire production serializer. Header + copy is an artificial staging comparison: production writes payload directly into the record buffer and does not perform this extra copy. No SD write is measured here.','',
        '## Pipeline pseudocode and optimization opportunities','',
        'The following uses native signed i32 PCM arithmetic. No sample needs promotion to i64. Counter/format fields may be 64-bit, as required by CRCL. Arithmetic headroom follows the encoder’s validated PCM bound; a future filter must preserve an explicit range guarantee. MIC1 is skipped in every transform.','',
        '### A. Optional first-order high-pass filter','',
        'This diagnostic estimates the cost of a cheap DC blocker (approximately 10 Hz corner at 16 kHz). It uses no multiply or divide. It is separate from temporal prediction, and adding it would change reconstructed audio/phase and require a documented format/calibration policy. Filter state is two i32 values per channel; a real stream must preserve it across blocks. The timing fixture starts those states at zero each run.','',
        '```text\nfor channel in MIC2..MIC6:\n    for x in this channel, striding by 6:\n        y = x - previous_x + previous_y - (previous_y >> 8)\n        previous_x = x\n        previous_y = y\n        overwrite sample with y\n```','',
        '### B. Peak scan and scale selection','',
        '```text\nfor channel in MIC2..MIC6:\n    peak = 0\n    for x in channel:\n        reject x outside configured bounded PCM range\n        peak = max(peak, abs(x))\n    shift = max(0, bit_length(peak) - (bits - 1))\n    if shift != 0 and ((peak + 2^(shift-1)) >> shift) > 2^(bits-1)-1:\n        shift += 1\n    shift = max(shift, 6)\n```','',
        'The shift decision is already CLZ-based and cheap. Preserve rounding-headroom checks and the per-channel peak. The production peak pass is substantially slower than the simple scanning kernel, so move the phase decision, cursor updates and slice/budget checks outside a bounded inner chunk where possible. If a future streaming input copy already touches every scalar value, peak detection could be fused into that traversal without adding a sample buffer. That is a future experiment, not a measured saving in this report.','',
        '### C. In-place quantization, temporal residual and sum','',
        '```text\nhalf = 0 if shift == 0 else 2^(shift-1)\nprevious_q = 0\nsum_folded = 0\nfor n in 0..1023:\n    q = (pcm[n, channel] + half) >> shift\n    pcm[n, channel] = q\n    if n > 0:\n        residual = q - previous_q\n        u = 2*residual if residual >= 0 else -2*residual - 1\n        sum_folded += u\n    previous_q = q\n```','',
        'This is already one fused production pass: rounding, storing q, first difference, fold and sum. It stores neither residual nor folded arrays. The predictor restarts at each block; the first q is saved explicitly as a 10-bit warm-up. Scale and six-bit minimum are combined into one absolute shift, **not applied successively**.','',
        'The standalone temporal-delta row writes a diagnostic residual array; the fold/sum row then reads that array; the sum-only row reads pre-folded u. They show individual mechanisms but should not replace the fused production loop with extra passes or allocations. The isolated fused kernel demonstrates that holding previous/sum/offset/shift in local registers and handling a contiguous work chunk has much more headroom than changing the arithmetic formula.','',
        '### D. Rice parameter estimate','',
        '```text\ncount = frames - 1\nif sum_folded == 0 or count == 0:\n    k = 0\nelse:\n    mean = ceil(sum_folded / count)\n    k = floor(log2(mean))          # CLZ\n```','',
        'Only one integer division and CLZ occur per channel-block. There is no histogram or candidate-k search. Keep dynamic count handling for partial final blocks. Optimizing this step has little effect on the full budget. The compiler already emits the residual fold as a shift/XOR sequence in the inspected production hot path, so merely rewriting the source fold into a branchless expression is unlikely to be the principal win.','',
        '### E. Current Rice bit emission','',
        '```text\nwrite_signed(q[0], bits=10)\nprevious_q = q[0]\nfor n in 1..1023:\n    residual = q[n] - previous_q\n    previous_q = q[n]\n    u = fold_signed(residual)\n    quotient = u >> k\n    remainder = u & (2^k - 1)\n    write_zero_bits(quotient)\n    write_bits(1, 1)\n    write_bits(remainder, k)\npad_with_zero_bits_to_next_byte()\n```','',
        'The residual is recomputed from stored q during serialization. That avoids a residual scratch buffer. The current bit writer splits fields at byte boundaries, resets fresh bytes, masks/shifts fragments, checks capacity, and updates its bit position. Unary coding invokes write repeatedly in chunks of at most 32 zero bits, plus a stop-bit write; the remainder invokes write again. Generated assembly confirms an out-of-line unary call in Encoder::data as well as runtime representation dispatch and bounds checks.','',
        '### F. Tested single-codeword and word-reservoir candidates','',
        '```text\nlength = quotient + 1 + k\ncode = (1 << k) | remainder       # leading unary zeros are implicit\nif length <= 32:\n    append_low_bits(code, length) # one append, not three writes\nelse:\n    use_generic_unary_and_remainder_path()\n```','',
        'For the 32-bit reservoir, the live bits occupy the high end of a u32. Appending takes the minimum of remaining codeword width and free reservoir bits; a full word is stored in big-endian/MSB order. The final partial word writes only ceil(used/8) bytes with zero padding. No u64 accumulator is needed.','',
        '```text\nwhile remaining_code_bits > 0:\n    take = min(remaining_code_bits, 32 - used)\n    fragment = next_high_bits_of_codeword(take)\n    reservoir |= fragment << (32 - used - take)\n    used += take\n    remaining_code_bits -= take\n    if used == 32:\n        store_4_bytes_msb_first(reservoir)\n        used = 0\n        reservoir = 0\n```','',
        'The single-codeword byte-writer candidate retains the actual checked Bits implementation. The reservoir candidate retains Rust slice bounds checks but assumes the record capacity was prevalidated and omits the production writer’s checked_add and per-field capacity errors. Its measured improvement combines fewer/larger stores, reduced field overhead and that narrower contract. Do not attribute all of it to 32-bit stores. Production adoption needs a once-per-channel/per-record capacity proof, long-unary fallback, k=0, partial blocks, overflow handling and unchanged error behavior. A byte-safe output path is needed because channel payloads start at arbitrary byte offsets; a “word reservoir” does not automatically guarantee aligned u32 SRAM stores. Inspect the final generated instructions after integration.','',
        '### G. Small-residual lookup: proposed, not measured','',
        '```text\nif folded_residual < TABLE_SIZE:\n    (code, length) = table[k][folded_residual]\n    append_low_bits(code, length)\nelse:\n    compute code and length normally\n```','',
        f"On this fixture {f['folded_under_64_percent']:.1f}% of folded residuals are <64, {f['folded_under_128_percent']:.1f}% <128 and {f['folded_under_256_percent']:.1f}% <256 (maximum {f['folded_max']}). A fixed-k=6 table can pack the code/length into 16-bit entries: 256 entries cost 512 bytes. General support for all selected k values, flash placement versus SRAM, fallback branches and table-build cost change the tradeoff. The one-codeword arithmetic is only shifts/masks/adds, so lookup is lower priority than removing repeated writer calls and per-sample state work. There is **no measured lookup speedup** in this report.", '',
        '### H. CRCL record assembly','',
        '```text\nreserve and zero 88 header bytes\nfor channel in MIC2..MIC6:\n    write descriptor: channel_id, temporal_mode, k, absolute_shift\n    emit Rice payload directly into output at current position\n    zero-pad channel to byte boundary\n    write descriptor.payload_bytes\nwrite DATA magic/kind/header_size/total_size/sequence/frame_offset/frame_count\nreturn complete record slice\n```','',
        'No CRC or digest is appended in v2. Record assembly should keep payload emission in place and avoid a staging copy. The header-only diagnostic takes less than 0.0022 ms; even its artificial full-payload copy is below 0.06 ms. “CRCL packing” in earlier inclusive profiles mostly reflects Rice emission, not these descriptor stores. SD buffering and card write bursts belong to a different measurement and are intentionally absent here.','',
        '## Optimization order','',
        '1. **Specialize the v2 temporal payload path and combine each common Rice symbol into one append.** Retain the generic long-code fallback and exact padding. This has a measured byte-identical packing benefit using the existing checked writer.','2. **Prototype a u32 reservoir with record-level capacity validation.** Compare complete Encoder::data and Transform+data after integration; keep error behavior and arbitrary payload alignment. The current result is a useful upper-bound experiment, not a production patch.','3. **Refactor advance_cheap around bounded inner chunks.** Keep sample cursor, previous q, sum and shift in locals; separate peak and quantization loops so each sample does not re-enter generic phase logic. Persist scalar state at chunk boundaries to retain responsiveness and in-place memory behavior.','4. **Retain fused arithmetic; consider interleaved peak collection while loading the existing buffer.** Avoid residual/LUT sample workspaces. Measure the full path because independent kernel sums overstate precision.','5. **Evaluate lookup only after these changes.** Parameter calculations and the 88-byte header are already negligible.','',
        f'Changing Transform::advance from budget 256 to unlimited changes this fixture’s transform by only {cycles("production_transform_budget256")-cycles("production_transform"):,.0f} cycles ({100*(cycles("production_transform_budget256")/cycles("production_transform")-1):.2f}%). Simply removing cooperative call boundaries does not explain the much larger gap between the stateful transform and the specialized arithmetic kernels. That gap includes generic state management, bounds/phase/budget checks, specialization and compiler code generation; it is not a direct isolated measurement of one branch.','',
        '## Validation, artifacts and reproduction','',
        'Target-side checks compare each candidate payload byte-for-byte with production Encoder::data, compare complete header+payload records, verify quantized samples equal the production Transform, and verify MIC1 is unchanged. The optional HPF and shift-only truncation rows are intentionally separate arithmetic experiments. Ten desktop tests pass with the benchmark feature enabled, including reservoir writes of widths 0…32 across word boundaries, deterministic fixture/payload equivalence and existing encoder tests. Neither benchmark buffers nor candidate routines are included in normal firmware builds.','',
        f'Tested ELF SHA-256: `{build["artifact_sha256"]}`. Build: `{build["source_git"]}`, release, {result["compiler"]}; encoder crate opt-level=3, workspace opt-level="s", LTO disabled, one codegen unit. Original code and all measurements use the standard release settings rather than applying a new optimization profile. See [build identity](analysis003/benchmark-build.json), [first run](analysis003/target-run1.log), [repeat](analysis003/target-run2.log), and [hot-function assembly](analysis003/hot-functions.asm).','',
        'The board is left running the completed benchmark, idle after PASS; acquisition and SD recording are not active. No Aardwolf configuration or encoding algorithm was changed.','',
        'Build, flash and collect a fresh run:', '',
        '```powershell\nrtk proxy powershell -ExecutionPolicy Bypass -File scripts/run_crcl_loop_benchmark.ps1\n# Logs are saved in the session directory printed by firmware.ps1.\n# Repeat the same ELF without rebuilding:\nrtk proxy powershell -ExecutionPolicy Bypass -File scripts/firmware.ps1 -Package service-test-crclencoder -Bin crcl-loop-benchmark -NoBuild -MonitorSeconds 30 -Until \'CRCL LOOP BENCH PASS\'\n```','',
        'Regenerate the report from the saved evidence without a board:', '',
        '```powershell\nrtk proxy python scripts/crcl_loop_report.py --log docs/audio/multichannel/analysis003/target-run1.log --repeat-log docs/audio/multichannel/analysis003/target-run2.log\nrtk cargo test -p raylar-crcl-encoder-service --features loop-benchmark --target x86_64-pc-windows-msvc\n```','',
        'For new sessions, pass their `target.log` paths and `--build-record firmware-builds/service-test-crclencoder.json` to the report script. Source: [target entrypoint](../../../servicetests/crclencoder/loop_benchmark.rs), [diagnostic kernels](../../../crates/services/crclencoder/src/benchmark.rs), [runner](../../../scripts/run_crcl_loop_benchmark.ps1), [report generator](../../../scripts/crcl_loop_report.py). The [artifact manifest](analysis003/manifest.json) records hashes of saved evidence and reproduction sources.','']
    REPORT.write_text('\n'.join(lines),encoding='utf8')
    sources=['crates/services/crclencoder/src/benchmark.rs','servicetests/crclencoder/loop_benchmark.rs',
             'crates/services/crclencoder/src/cheap.rs','crates/services/crclencoder/src/bits.rs',
             'crates/services/crclencoder/src/format.rs','scripts/crcl_loop_report.py','scripts/run_crcl_loop_benchmark.ps1']
    manifest=dict(artifacts={str(p.relative_to(OUT)).replace('\\','/'):hashlib.sha256(p.read_bytes()).hexdigest()
                  for p in sorted(OUT.glob('*')) if p.is_file() and p.name!='manifest.json'},
                  report_sha256=hashlib.sha256(REPORT.read_bytes()).hexdigest(),
                  reproduction_sources_sha256={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in sources})
    (OUT/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    for target in re.findall(r'\]\(([^)]+)\)',REPORT.read_text(encoding='utf8')):
        assert (REPORT.parent/target).exists(),target
    print(json.dumps(dict(report=str(REPORT),total_cycles=total,total_ms=total/HZ*1000,
          same_image_median_difference=repeat,byte_codeword_saving_percent=100*(1-single/byte),
          word_codeword_saving_percent=100*(1-word/byte)),indent=2))

if __name__=='__main__':main()
