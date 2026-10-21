import json, sys
from fractions import Fraction as F
from check import tri_inter
rows=[json.loads(l) for l in open(sys.argv[1])]
ring={(r['name'],r['pose']):r for r in rows if r['kind']=='ring'}
probe={(r['k'],r['pose']):r for r in rows if r['kind']=='probe'}
def Fv(v): return tuple(F(x) for x in v)
for pose in ['at rest','turned about z','turned','flipped','flipped and turned']:
    old=ring[('fin old',pose)]['tris']; new=ring[('fin new',pose)]['tris']
    # names from old crown: MEET, c0..c4, low_old; new low
    o=old
    names={tuple(o[0][1]):'M',tuple(o[0][0]):'c0',tuple(o[2][1]):'c1',tuple(o[3][1]):'c2',tuple(o[4][1]):'c3',tuple(o[0][2]):'c4',tuple(o[1][1]):'lo'}
    for t in new:
        for v in t:
            if tuple(v) not in names: names[tuple(v)]='ln'
    lab=lambda t:''.join(sorted(names[tuple(v)] for v in t))
    if pose=='at rest':
        print('faces at MEET old', sorted(lab(t) for t in old if any(names[tuple(v)]=='M' for v in t)))
        print('faces at MEET new', sorted(lab(t) for t in new if any(names[tuple(v)]=='M' for v in t)))
    for k in range(5):
        P=[tuple(map(Fv,t)) for t in probe[(k,pose)]['tris']]
        def met(C):
            s=set()
            for t in C:
                T=tuple(map(Fv,t))
                for p in P:
                    I=tri_inter(p,T) or tri_inter(T,p)
                    if I and any(q!=Fv(ring[('fin old',pose)]['tris'][0][1]) for q in I):
                        s.add(lab(t))
            return sorted(s)
        mo,mn=met(old),met(new)
        print(pose[:10].ljust(10),'probe',k,'old',mo,'| new',mn)
