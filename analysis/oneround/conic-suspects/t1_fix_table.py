#!/usr/bin/env python3
"""Task 1, the cause of the conic-conic misses: the same poses under scratch interventions
(SUSPECT_FIX: a = Window::height_jet's plane through a point of b's arc; b = off_by's clearance dropped;
c = conic_conic's two planes stated through a point of each arc). Truth from the main run's judged file."""
import json,collections,sys
d=sys.argv[1]
for eps in ('1e-9','1e-12'):
    judged={(r['a'],r['shape'],r['s'],r['fam'],r['var']):r for r in json.load(open(f'{d}/t1-{eps}.judged.json')) if r['door']=='meet'}
    print('== eps',eps,'  cells: right / refused / WRONG, by family and truth')
    for fix in ('none','a','b','ab','c','abc'):
        c=collections.defaultdict(lambda:[0,0,0])
        for l in open(f'{d}/t1fix-{fix}-{eps}.jsonl'):
            r=json.loads(l); j=judged[(r['a'],r['shape'],r['s'],r['fam'],r['var'])]
            k='refused' if r['kind']=='refused' else ('MEETS' if (r['points'] or r['shared']) else 'APART')
            fam=r['fam']+('-across' if r['fam']=='G' and r['var']=='across' else '-coplanar' if r['fam']=='G' else '')
            c[(fam,j['truth'])][1 if k=='refused' else (0 if k==j['truth'] else 2)]+=1
        print(f'  fix {fix:4s} '+'  '.join(f'{f} {t}: {v[0]}/{v[1]}/{v[2]}' for (f,t),v in sorted(c.items())))
