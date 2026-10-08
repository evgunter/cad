import sys, json, collections
sys.path.insert(0, '.')
from gen import *
seed = sys.argv[1]; outf = sys.argv[2] if len(sys.argv) > 2 else f'out_{seed}.txt'
meta = json.load(open(f'meta_{seed}.json'))
lines = open(outf).read().split('\n')
res = []; cur = None
for l in lines:
    w = l.split()
    if not w: continue
    if w[0] == 'case': cur = [l, []]; res.append(cur)
    else: cur[1].append((w[2], w[3]))
stats = collections.Counter(); bad = []
for m, (hdr, rows) in zip(meta, res):
    orc = Oracle(m['secs'])
    for (u, far, tag), (wc, cs) in zip(m['P'], rows):
        truth = orc.cls(far)
        dist = angdist(m['secs'], u)
        for which, got in (('wc', wc), ('cs', cs)):
            if got in ('In', 'Out', 'On'):
                if got == truth: stats[which+'_ok'] += 1
                elif (got == 'On' or truth == 'On') and dist < 4*float(__import__("os").environ.get("RV_EPS","1e-9")): stats[which+'_okband'] += 1
                elif dist < 1e-14: stats[which+'_okround'] += 1
                else:
                    stats[which+'_WRONG'] += 1
                    bad.append((m['name'], which, tag, got, truth, dist, hdr))
            else:
                stats[which+'_'+got.split(':')[0]] += 1
                if dist > 1e-6:
                    stats[which+'_undecided_far'] += 1
                    bad.append((m['name'], which, tag, got, truth, dist, hdr))
print(dict(stats))
import collections as C
byname=C.Counter((b[0].split('/')[0] if not b[0].startswith('r') else 'random', b[1], b[3] if b[3] in ('In','Out','On') else 'undec') for b in bad)
for k,v in sorted(byname.items()): print(k,v)
for b in bad[:int(sys.argv[3]) if len(sys.argv)>3 else 30]: print(b)
