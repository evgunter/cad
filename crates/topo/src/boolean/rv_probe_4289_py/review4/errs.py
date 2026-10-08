import sys, json, collections
from adv import est_flip
from oracle_a import *
eps = float(sys.argv[3]); meta = json.load(open(sys.argv[1])); rows = []
for l in open(sys.argv[2]):
    w = l.split()
    if w[0] == 'case': rows.append([])
    else: rows[-1].append((w[2], w[3]))
b = collections.Counter(); samp = []
for ci, (m, rs) in enumerate(zip(meta, rows)):
    for pi, ((u, D, tag), (wc, cs)) in enumerate(zip(m['P'], rs)):
        if cs.startswith('Err'):
            f = est_flip(m['secs'], D) / eps
            k = '<=1' if f <= 1 else '<=10' if f <= 10 else '<=100' if f <= 100 else '>100'
            b[(cs, k)] += 1; samp.append((cs, f, ci, pi, m.get('name', '')))
for k, v in sorted(b.items()): print(k, v)
import random; random.seed(0)
for s in sorted(random.sample(samp, min(20, len(samp))), key=lambda x: x[1]): print('  %s flip/eps %.3g case %d probe %d %s' % s)
