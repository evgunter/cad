import json,sys
from fractions import Fraction as F
for seed,names in [('1',['r320','r421']),('13',None),('21',None)]:
    meta=json.load(open(f'meta_{seed}.json'))
    worst=[]
    for m in meta:
        off=0
        for s in m['secs']:
            n=[F(x) for x in s['n']]
            for far in (s['fs'],s['fe']):
                o=abs(sum(F(a)*b for a,b in zip(far,n)))
                off=max(off,o)
        worst.append((float(off),m['name']))
    worst.sort(reverse=True)
    print(seed,'cones off-plane >1e-12:',sum(1 for w in worst if w[0]>1e-12),'>1e-9:',sum(1 for w in worst if w[0]>1e-9),'top',worst[:4])
    if names:
        for w in worst:
            if w[1].split('_')[0] in names: print('  ',w)
