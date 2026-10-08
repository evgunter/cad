import sys, json, collections
from gen import Oracle as OracleB
from oracle_a import *
from adv import est_flip
eps = float(sys.argv[2]); meta = json.load(open('meta_dl.json'))
rows = []; hdr = []
for l in open(sys.argv[1]):
    w = l.split()
    if w[0] == 'case': rows.append([]); hdr.append(l.strip())
    else: rows[-1].append((w[2], w[3]))
st = collections.Counter(); ex = []
for m, rs, h in zip(meta, rows, hdr):
    oa, ob = OracleA(m['secs']), OracleB(m['secs'])
    for (u, D, tag), (wc, cs) in zip(m['P'], rs):
        ta, tb = oa.cls(D), ob.cls(D)
        fl = est_flip(m['secs'], D)
        for nm, gv in (('wc', wc), ('cs', cs)):
            if gv in ('In', 'Out'):
                if gv != ta:
                    st[f'{nm}_WRONG_A'] += 1; ex.append((nm, gv, ta, tb, fl/eps, tag, m['hollow'], h))
                else: st[f'{nm}_ok'] += 1
            else: st[f'{nm}_{gv.split(":")[0]}'] += 1
        if ta != tb: st['A!=B'] += 1
print(dict(st))
for e in ex[:12]: print(e)
