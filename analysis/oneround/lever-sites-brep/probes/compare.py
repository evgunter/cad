#!/usr/bin/env python3
"""Compare the decision logs of base and head for one eps row (see differential.sh, klog.patch.py).
Usage: compare.py <diffrun dir> <eps>"""
import os, sys, re, collections
d, eps = sys.argv[1], sys.argv[2]
def load(tree):
    out = {}
    root = f'{d}/klog-{tree}-{eps}'
    for f in os.listdir(root):
        t = {}
        for line in open(os.path.join(root, f)):
            ty, pred, o, n, h = line.rstrip('\n').split('\t')
            t[(ty, pred, o)] = (int(n), h)
        out[f[:-4]] = t
    return out
def outcomes(tree):
    res = {}
    for line in open(f'{d}/run-{tree}-{eps}.log', errors='replace'):
        m = re.match(r'\s+(PASS|FAIL|SIGABRT|SIGSEGV|TIMEOUT|LEAK|FLAKY)\s+\[[^\]]*\]\s+\(\s*\d+/\d+\)\s+(\S+ \S+)', line)
        if m: res[m.group(2)] = m.group(1)
    return res
B, H = load('base'), load('ihead')
ob, oh = outcomes('base'), outcomes('ihead')
def short(ty): return ty.split('::')[-1] if '<' not in ty else re.sub(r'\w+::', '', ty)
tot = collections.Counter(); moved = []; valued = []
for t in sorted(set(B) & set(H)):
    b, h = B[t], H[t]
    for k in set(b) | set(h):
        tot[('base', short(k[0]))] += b.get(k, (0,))[0]; tot[('head', short(k[0]))] += h.get(k, (0,))[0]
    cnt = {k: (b.get(k, (0,))[0], h.get(k, (0,))[0]) for k in set(b) | set(h) if b.get(k, (0,))[0] != h.get(k, (0,))[0]}
    if cnt: moved.append((t, cnt))
    elif any(b[k][1] != h[k][1] for k in b): valued.append((t, sorted({(short(k[0]), k[1]) for k in b if b[k][1] != h[k][1]})))
print(f'eps {eps}: tests with a decision log: base {len(B)}, head {len(H)}; in both {len(set(B)&set(H))}')
print('only base:', sorted(set(B) - set(H))[:20]); print('only head:', sorted(set(H) - set(B))[:20])
print('decisions by scalar:', dict(sorted(tot.items())))
print(f'test outcomes: base {collections.Counter(ob.values())}, head {collections.Counter(oh.values())}')
print('outcome differs:', sorted((t, ob.get(t), oh.get(t)) for t in set(ob) | set(oh) if ob.get(t) != oh.get(t))[:40])
print('non-pass on both:', sorted(t for t in ob if ob[t] != 'PASS' and oh.get(t) == ob[t])[:40])
print(f'\nTESTS WHOSE DECISION COUNTS DIFFER (a verdict moved or a decision was added/dropped): {len(moved)}')
for t, cnt in moved[:60]:
    print('  ', t)
    for k, (x, y) in sorted(cnt.items())[:12]: print(f'        {short(k[0])} {k[1]} {k[2]}: base {x} head {y}')
print(f'\nTESTS WHOSE COUNTS AGREE AND SOME MARGIN VALUE DIFFERS: {len(valued)}')
preds = collections.Counter(p for _, ps in valued for p in ps)
print('   by (scalar, predicate):', preds.most_common(40))
for t, ps in valued[:400]: print('  ', t, '|', ', '.join(f'{a}:{b}' for a, b in ps[:6]))
