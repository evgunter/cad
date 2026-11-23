#!/usr/bin/env python3
"""Task 3: what the boolean ladder built and refused, per file (no truth here; see oracle_t3.py)."""
import json,re,sys,collections,glob
def why(s):
    m=re.search(r'KernelLimit\{operand:(\w+),bound:([-\de.]+)',s)
    if m: return 'KernelLimit:'+m.group(1)+(':Minted' if 'edge:Minted' in s else ':other')
    m=re.search(r'predicate:Some\("(\w+)"\)',s)
    if m: return 'Esc:'+m.group(1)
    return re.sub(r'FaceKey\(\w+\)','F',s)[:90]
d=sys.argv[1]
for eps in ('1e-9','1e-12'):
  for al in ('0.46','1.0','1.3'):
    for r in '01':
        rows=[json.loads(l) for l in open(f'{d}/t3-a{al}-r{r}-{eps}.jsonl')]
        runs=[]  # (class, amin, amax, n)
        for x in sorted(rows[1:], key=lambda x:-x.get('gap',0)):
            if x['row']!='bool': cls='slab:'+x.get('why','')[:60]; a=None
            elif x['kind']=='built':
                es=[e for e in x['body']['ellipses'] if e['a']>30]
                a=max([e['a'] for e in es], default=0)
                cls=x['op']+' built '+('+vol' if 'v' in x['vol'] else 'vol:'+str(x['vol'])[:50])
            else: cls=x['op']+' '+x['kind']+':'+why(x.get('why','')); a=None
            cls=(cls, x.get('gap'))
            runs.append((cls[0],x.get('gap'),a))
        print(f'== eps {eps} alpha {al} rot {r}')
        g=collections.OrderedDict()
        for c,gap,a in runs: g.setdefault(c,[]).append((gap,a))
        for c,v in g.items():
            gaps=[x[0] for x in v]; aa=[x[1] for x in v if x[1]]
            print(f'   {c:70s} n={len(v):2d} gap {max(gaps):.1e}..{min(gaps):.1e}'+(f' a {min(aa):.2e}..{max(aa):.2e}' if aa else ''))
