#!/usr/bin/env python3
"""Task 3: who read the derived centre of a section ellipse (semi-axis >= SUSPECT_MIN) during each boolean,
tallied by the boolean's outcome. Input: the instrumented example's stdout."""
import json,collections,re,sys
depth=int(sys.argv[2]) if len(sys.argv)>2 else 2
for path in sys.argv[1].split(','):
    rows=[json.loads(l) for l in open(path)]
    tr=None; agg=collections.defaultdict(collections.Counter); runs=collections.Counter()
    for r in rows:
        if r['row']=='trace': tr=r
        elif r['row']=='bool':
            w=r.get('why','')
            out='built' if r['kind']=='built' else ('refused:'+('KernelLimit' if 'KernelLimit' in w else 'props_cone_ellipse_on_cone' if 'props_cone' in w else 'pn_conic_type' if 'pn_conic' in w else 'OpenConic' if 'OpenConic' in w else w[:40]))
            runs[out]+=1
            for n,chain in tr['chains']:
                agg[out][' < '.join(chain.split(' < ')[:depth])]+=n
    print('=====',path)
    for out,c in agg.items():
        print(' --',out,'(%d booleans)'%runs[out])
        for k,n in sorted(c.items(), key=lambda kv:-kv[1]): print(f'    {n:7d} {k}')
