# MEASUREMENT ONLY: items 3-5 from the CAD_WM rows.
import glob,re,sys,collections
W='/home/user/wm-work'
BAND_Z,BAND_E=1e-9,1e-8
def rows(run):
    for f in glob.glob(f'{W}/log-{run}/*.log'):
        for l in open(f,errors='replace'):
            p=l.rstrip('\n').split('\t')
            yield p
def kv(p):
    d={}
    for x in p:
        if '=' in x: k,v=x.split('=',1); d[k]=v
    return d
out=[]
def say(*a): out.append(' '.join(str(x) for x in a))
for run in sys.argv[1:] or ['P','M']:
    D=[];V=[];F=[];E={}
    for p in rows(run):
        if p[0]=='WMD': d=kv(p[4:]); d['test']=p[2]; d['pair']=p[3]; D.append(d)
        elif p[0]=='WMV': d=kv(p[3:]); d['test']=p[2]; d['verdict']=p[-1]; V.append(d)
        elif p[0]=='WMF': F.append(p)
        elif p[0]=='WME':
            E[(p[2],p[3])]=(p[4],p[5],float(p[6].split('=')[1]),float(p[7].split('=')[1]))
    say(f'== run {run}: asks {len(V)}, tests {len(set(v["test"] for v in V))}')
    tot=collections.Counter()
    for v in V:
        for k in ('calls','crease_calls','flips','crease_final','fold_diff'): tot[k]+=int(v[k])
    say('  per-ask totals', dict(tot))
    flips=[d for d in D if d['cmut']=='true' and d['c7329']=='false']
    say(f'  item3 flips (cmut & !c7329): {len(flips)} cell pairs, {len(set((d["test"],d["pair"]) for d in flips))} test/face pairs, {len(set(d["test"] for d in flips))} tests')
    gaps=[float(d['gap'][5:-1]) for d in flips if d['gap'].startswith('Some(')]
    nog=sum(1 for d in flips if not d['gap'].startswith('Some('))
    if flips:
        bins=collections.Counter()
        for g in gaps:
            b='<0 (reaches the other side)' if g<0 else ('0' if g==0 else ('<zero 1e-9' if g<BAND_Z else ('<esc 1e-8' if g<BAND_E else ('<1e-6' if g<1e-6 else ('<1e-3' if g<1e-3 else '>=1e-3')))))
            bins[b]+=1
        say('  far-side gap distribution', dict(bins), 'no gap read', nog)
        sm=sorted(((float(d['gap'][5:-1]),d) for d in flips if d['gap'].startswith('Some(')),key=lambda x:x[0])[:10]
        for g,d in sm: say(f'   gap={g:.3e} {d["test"][:70]} {d["pair"]} edges={d["edges"]} depth={d["depth"]} base={d["base"]} arm={d["arm"]} fold={d["fold"]}')
    # face pair verdicts
    say(f'  WMF face pairs asked alone: {len(F)}')
    vc=collections.Counter()
    for p in F:
        a=re.sub(r'\(.*','',p[4].split('=',1)[1]); b=re.sub(r'\(.*','',p[5].split('=',1)[1]); vc[(a,b)]+=1
    say('   (pr7329, mutant) verdict pairs', dict(vc))
    fin=[d for d in D if d['crease']=='true' and d['c7329']=='false' and (d['base']=='refuse' or d['base']=='split@budget')]
    say(f'  item4 crease cell pairs refused for good: {len(fin)} ({collections.Counter(d["base"] for d in fin)}) in {len(set(d["test"] for d in fin))} tests')
    sc=collections.Counter()
    for d in fin:
        s=d['sum']
        bis=re.findall(r'"bis:(\d)(\d)(\d)(\d)',s); pl=re.findall(r'"pl:(\d)(\d)(\d)(\d)',s)
        sc['with bisector']+=bool(bis); sc['with conic plane']+=bool(pl); sc['no plane read']+=not(bis or pl)
        if any(t[3]=='1' and not (t[0]=='1' and t[1]=='1') for t in bis): sc['bis: sum clears, not both alone']+=1
        if any(t[3]=='1' and t[0]=='0' and t[1]=='0' for t in bis): sc['bis: sum clears, neither alone']+=1
        if any(t[3]=='1' and not (t[0]=='1' and t[1]=='1') for t in pl): sc['pl: sum clears, not both alone']+=1
        if any(t[0]=='1' and t[1]=='1' and t[2]=='0' for t in bis+pl): sc['both alone but same side']+=1
    say('  item4 sum test', dict(sc))
    say('  item4 presentations (x|y) at those cell pairs', collections.Counter(d.get('pres','?') for d in fin).most_common(8))
    say('  item4 flips among them (sign alone would certify)', sum(d['cmut']=='true' for d in fin))
    fd=[d for d in D if d['cfold'] not in ('None',) and (d['cfold']=='Some(true)')!=(d['c7329']=='true')]
    say(f'  item5 cell pairs whose verdict differs reach vs fold: {len(fd)}; c7329 cert & fold not: {sum(d["c7329"]=="true" for d in fd)}; fold cert & 7329 not: {sum(d["c7329"]=="false" for d in fd)}; tests {len(set(d["test"] for d in fd))}')
    ed=collections.Counter(); diff_edges=set()
    for (t,e),(k,c,r,f) in E.items():
        ed['edges']+=1
        if r>2*f or f>2*r: ed['differ>2x']+=1; diff_edges.add((t,e))
        ed['class '+c]+=1
    say('  item5 edges (test,edge key distinct)', dict(ed))
    fde=set()
    for d in fd:
        for e in re.findall(r'EdgeKey\(\w+\)',d['edges']): fde.add((d['test'],e))
    say(f'  item5 edges with a differing cell verdict: {len(fde)}; of them in differ>2x: {len(fde & diff_edges)}')
print('\n'.join(out))
