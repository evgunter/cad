#!/usr/bin/env python3
"""Task 1: per family, the counts right / refused / WRONG by semi-axis (both shapes, all its variants of one truth),
and the last decision the kernel's own log holds for each wrong answer."""
import json,collections,sys
d=sys.argv[1]
for eps in ('1e-9','1e-12'):
    rows=json.load(open(f'{d}/t1-{eps}.judged.json'))+json.load(open(f'{d}/t1under-{eps}.judged.json'))
    logs={}
    for l in open(f'{d}/t1log-{eps}.jsonl'):
        r=json.loads(l)
        if r['door']=='meet': logs[(r['a'],r['shape'],r['s'],r['fam'],r['var'])]=[x.rsplit('x',1)[0] for x in r['log']]
    sizes=sorted({r['a'] for r in rows})
    print(f'\n===== eps {eps}: right/refused/WRONG(+gap-definite, +point past K eps)   a = '+'  '.join(f'{a:.0e}' for a in sizes))
    g=collections.OrderedDict(); last=collections.defaultdict(collections.Counter)
    for r in rows:
        fam=r['fam']
        if fam=='G': fam='G-'+r['var']
        if fam=='E': fam='E-rev' if r['var'].endswith('rev') else 'E'
        c=g.setdefault((r['door'],fam,r['truth']),collections.defaultdict(lambda:[0,0,0,0,0]))[r['a']]
        c[{'right':0,'refused':1,'WRONG':2,'GAPDEF':3,'WRONG-POINT':4}[r['verdict']]]+=1
        if r['verdict']=='WRONG' and r['door']=='meet':
            lg=logs[(r['a'],r['shape'],r['s'],r['fam'],r['var'])]
            last[(fam,r['truth'])][lg[-1] if lg[-1].split(':')[0] not in ('curve_pair_window_width',) else next(x for x in reversed(lg) if not x.startswith('curve_pair_window_width'))]+=1
    for (door,fam,truth),by in g.items():
        cells=[]
        for a in sizes:
            c=by[a]; cells.append(f'{c[0]}/{c[1]}/{c[2]}'+(f'+{c[3]}g' if c[3] else '')+(f'+{c[4]}p' if c[4] else ''))
        print(f'{door[:6]:6s} {fam:10s} truth {truth:8s} | '+'  '.join(f'{x:9s}' for x in cells))
        if last[(fam,truth)]: print('          last decision of the wrong ones: '+', '.join(f'{k} {v}' for k,v in last[(fam,truth)].most_common()))
