import glob,re,collections,sys
W='/home/user/wm-work'
def load(run):
    D=[];F={};V=collections.defaultdict(list)
    for f in glob.glob(f'{W}/log-{run}/*.log'):
        for l in open(f,errors='replace'):
            p=l.rstrip('\n').split('\t')
            if p[0]=='WMD':
                d={k:v for k,v in (x.split('=',1) for x in p[4:] if '=' in x)}; d['test']=p[2]; d['pair']=p[3]; D.append(d)
            elif p[0]=='WMF':
                a,b=p[3].split('/'); key=(p[2],a,b)
                F[key]=(re.sub(r'\(.*','',p[4].split('=',1)[1]),re.sub(r'\(.*','',p[5].split('=',1)[1]),p[4][:200],p[5][:200])
            elif p[0]=='WMV':
                V[p[2]].append(p[-1][:60])
    return D,F,V
D,F,V=load('M')
fl=[d for d in D if d['cmut']=='true' and d['c7329']=='false']
g=lambda d: float(d['gap'][5:-1]) if d['gap'].startswith('Some(') else None
neg=[g(d) for d in fl if g(d) is not None and g(d)<0]
print('negatives',len(neg),'of which > -1e-8:',sum(x>-1e-8 for x in neg),' > -1e-3:',sum(x>-1e-3 for x in neg),' < -1:',sum(x<-1 for x in neg))
nn=sorted([d for d in fl if g(d) is not None and g(d)>=0],key=g)
print('ten smallest non-negative far-side gaps (M run):')
for d in nn[:10]:
    a,b=d['pair'].split('/'); fv=F.get((d['test'],a,b))
    print(f"  {g(d):.3e} | {d['test'][:72]} | {d['pair']} {d['edges']} d={d['depth']} arm={float(d['arm'][5:-1]):.3g} fold={float(d['fold'][5:-1]):.3g} | pres={d['pres']} | pair alone: 7329={fv[0] if fv else '?'} mut={fv[1] if fv else '?'}")
print('ten smallest |negative| (enclosure reaching the other side):')
ng=sorted([d for d in fl if g(d) is not None and g(d)<0],key=lambda d:-g(d))
for d in ng[:10]:
    a,b=d['pair'].split('/'); fv=F.get((d['test'],a,b))
    print(f"  {g(d):.3e} | {d['test'][:72]} | {d['pair']} {d['edges']} d={d['depth']} | pres={d['pres']} | alone 7329={fv[0] if fv else '?'} mut={fv[1] if fv else '?'}")
# face pairs: distinct, verdicts
fp=set((d['test'],*d['pair'].split('/')) for d in fl)
c=collections.Counter((F[k][0],F[k][1]) if k in F else ('?','?') for k in fp)
print('flip face pairs (test,a,b):',len(fp),'alone verdicts (7329,mutant):',dict(c))
for k in fp:
    if k in F and F[k][0]!=F[k][1]: print('   differ:',k[0][:80],k[1],k[2],'|',F[k][2][7:150])
# per-test top-level verdict diffs P vs M
DP,FP,VP=load('P')
diff=[t for t in set(V)|set(VP) if V.get(t)!=VP.get(t)]
print('tests whose top-level ask verdict sequence differs P vs M:',len(diff))
for t in diff[:10]: print('  ',t[:80])
# gap bins against band among flips in M
bins=collections.Counter()
for d in fl:
    x=g(d)
    bins['none' if x is None else ('<0' if x<0 else '0..1e-9' if x<1e-9 else '1e-9..1e-8' if x<1e-8 else '1e-8..1e-6' if x<1e-6 else '1e-6..1e-3' if x<1e-3 else '>=1e-3')]+=1
print('M flip gap bins',dict(bins))
print('M flips by presentation',collections.Counter(d['pres'] for d in fl).most_common(6))
print('M flips by base outcome',collections.Counter(d['base'] for d in fl))
