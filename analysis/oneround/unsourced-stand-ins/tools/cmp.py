import sys,os,collections,re
def load(d):
    per=collections.defaultdict(collections.Counter)
    for f in os.listdir(d):
        for l in open(os.path.join(d,f),errors='replace'):
            p=l.rstrip('\n').split('\t')
            if len(p)<4: continue
            test=p[0].split(' ')[-1] if p[0] else '?'
            per[(test,p[1])][(p[2],p[3])]+=1
    return per
a=load(sys.argv[1]); b=load(sys.argv[2])
mode=sys.argv[3] if len(sys.argv)>3 else 'verdict'
keys=sorted(set(a)|set(b))
sites=collections.Counter(); moved=[]
for k in keys:
    sites[k[1]]+=1
    A=a.get(k,collections.Counter()); B=b.get(k,collections.Counter())
    va=collections.Counter(); vb=collections.Counter()
    for (s,m),c in A.items(): va[s]+=c
    for (s,m),c in B.items(): vb[s]+=c
    if va!=vb: moved.append(('VERDICT',k,dict(va),dict(vb)))
    elif A!=B: moved.append(('BITS',k,sum(A.values()),sum(B.values())))
print('site (test,site) pairs reached:',dict(sites))
c=collections.Counter(m[0]+' '+m[1][1] for m in moved); print('moves:',dict(c))
for m in moved:
    if m[0]=='VERDICT' or mode=='all': print(m)
