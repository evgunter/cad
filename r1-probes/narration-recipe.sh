#!/bin/bash
python3 - <<'PY'
import hashlib
def narr(path, outdir):
    lines = open(path, encoding='utf-8', errors='replace').read().split('\n')
    # first narration line = the one after cargo's `Running` line
    a = next(i for i,l in enumerate(lines) if l.lstrip().startswith('Running `')) + 1
    # drop the harness's own trailing lines (EXIT:, the sha256sum, the count, DONE-)
    b = len(lines) - 1
    while b > a and (lines[b].startswith(('EXIT:','DONE-')) or lines[b].strip()==''
                     or lines[b].strip().isdigit()
                     or '/home/user/scalar-rate-r1-scratch/listing-' in lines[b]):
        b -= 1
    body = '\n'.join(lines[a:b+1]) + '\n'
    return body.replace(outdir, 'OUTDIR'), b-a+1
for lbl, p, d in [
    ('head-run1','/home/user/scalar-rate-r1-scratch/tour-head.log','/home/user/scalar-rate-r1-scratch/tourout-head'),
    ('head-run2','/home/user/scalar-rate-r1-scratch/tour-mb.log','/home/user/scalar-rate-r1-scratch/tourout-mb'),
    ('MERGEBASE','/home/user/scalar-rate-r1-scratch/tour-mbtree.log','/home/user/scalar-rate-r1-scratch/tourout-mbtree'),
]:
    t, n = narr(p, d)
    print(f'{lbl:10} lines={n} sha256={hashlib.sha256(t.encode()).hexdigest()}')
PY
