# diff CALL outcomes: baseline dir vs probe dir
import sys, re, glob, collections
def load(d):
    out = collections.defaultdict(list)
    for f in glob.glob(d + '/*.log'):
        for line in open(f):
            if line.startswith('CALL '):
                m = re.match(r'CALL fp=(\S+) d=(\S+) (ok tris=\d+|err \S*) test=(.*)$', line.strip())
                if m: out[(m.group(4), m.group(1), m.group(2))].append(m.group(3))
    return out
a, b = load(sys.argv[1]), load(sys.argv[2])
lost, gained, moved, same = [], [], 0, 0
for k in sorted(set(a) | set(b)):
    xa, xb = a.get(k, []), b.get(k, [])
    for i in range(max(len(xa), len(xb))):
        oa = xa[i] if i < len(xa) else 'absent'
        ob = xb[i] if i < len(xb) else 'absent'
        if oa.startswith('ok') and not ob.startswith('ok'): lost.append((k, oa, ob))
        elif ob.startswith('ok') and not oa.startswith('ok'): gained.append((k, oa, ob))
        elif oa != ob: moved += 1
        else: same += 1
print('calls same', same, 'tri-count moved', moved, 'lost', len(lost), 'gained', len(gained))
for x in lost: print('LOST', x)
for x in gained[:60]: print('GAINED', x)
