import re,sys,glob
num=r'(-?(?:[0-9.]+(?:e-?[0-9]+)?|NaN|inf))'
for f in sys.argv[1:]:
    rows=[]
    for line in open(f):
        if not line.startswith('STOPPROBE'): continue
        parts=line.split(' | ')
        outs=[]
        for k in (1,2,3):
            p=parts[k]
            m=re.search(r'st \('+num+', '+num+', '+num+r'\) -> (met\+\d+)(.*?)held(\[.*\])',p)
            if m is None: print('UNPARSED', p[:300]); break
            outs.append((float(m.group(1)),float(m.group(2)),float(m.group(3)),m.group(4),m.group(5).strip(),m.group(6)))
        og=float(re.search(r'gap '+num,parts[4]).group(1))
        rows.append((outs,og))
    n=len(rows)
    agree=sum(1 for o,_ in rows if o[0][3]==o[1][3]==o[2][3] and o[0][5]==o[1][5]==o[2][5])
    faults=sum(1 for o,_ in rows if any(x[5]!='[]' for x in o))
    print(f, n,'stops; outcomes agree', agree, '; any fault', faults)
    for name,k in (('polished',0),('whole',1),('held',2)):
        excess=[o[k][0]-og for o,og in rows]
        met=[o[k][0]-og for o,og in rows if o[k][3]!='met+0']
        nonmin=[(o[k][0],og,o[k][1],o[k][2]) for o,og in rows if o[k][0]-og>1e-9]
        print(f'  {name}: max gap-oracle {max(excess):.3e}; met stops {len(met)}, their max excess {max(met) if met else 0:.3e}; stops with gap > oracle+1e-9: {len(nonmin)}')
        for x in nonmin[:4]: print('     gap %.3e oracle %.3e cos_a %.3e cos_b %.3e'%x)
    # met point differences between variants
    def pts(s): return re.findall(r'x: '+num+', y: '+num+', z: '+num, s)
    dmax=0
    for o,_ in rows:
        a,b,c=pts(o[0][4]),pts(o[1][4]),pts(o[2][4])
        for P,Q in ((a,b),(a,c)):
            for p,q in zip(P,Q):
                d=sum((float(x)-float(y))**2 for x,y in zip(p,q))**.5; dmax=max(dmax,d)
    print('  max met-point distance polished vs others: %.3e'%dmax)
