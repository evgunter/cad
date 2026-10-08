import sys, collections
def rd(p): return [l.split() for l in open(p) if l.startswith('p ')]
c = collections.Counter()
for a, b in zip(rd(sys.argv[1]), rd(sys.argv[2])):
    for i, nm in ((2, 'wc'), (3, 'cs')):
        if a[i] != b[i]: c[f'{nm} {a[i]} -> {b[i]}'] += 1
for k, v in sorted(c.items()): print(f'  {v:4d} {k}')
