# decided readings the PR's own cone_fuzz rule calls in band (least_flip <= eps), and D-only flips
import sys, json, collections
from adv import est_flip
from oracle_a import *
seed, out, eps = sys.argv[1], sys.argv[2], float(sys.argv[3])
meta = json.load(open(f'meta_adv_{seed}.json')); rows = []
for l in open(out):
    w = l.split()
    if w[0] == 'case': rows.append([])
    else: rows[-1].append((w[2], w[3]))
st = collections.Counter(); ex = []
for ci, (m, rs) in enumerate(zip(meta, rows)):
    orc = OracleA(m['secs'])
    for pi, ((u, D, tag), (wc, cs)) in enumerate(zip(m['P'], rs)):
        for nm, g in (('cs', cs), ('wc', wc)):
            if g not in ('In', 'Out'): continue
            f = est_flip(m['secs'], D)
            if f <= eps:
                st[nm + '_decided_inband'] += 1; ex.append((nm, g, round(f / eps, 3), ci, pi, tag))
            if f < 3 * eps:
                t = orc.cls(D)
                for lab, s2, D2 in moves(m['secs'], D, eps):
                    if s2 is m['secs']:
                        t2 = orc.cls(D2)
                        if t2 in ('In', 'Out') and t2 != t:
                            st[nm + '_Dflip'] += 1; break
print(dict(st))
for e in ex[:8]: print(e)
