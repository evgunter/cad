import sys,collections
def load(p):
    d=collections.defaultdict(lambda: collections.defaultdict(list))
    for l in open(p):
        parts=l.rstrip('\n').split('\t')
        if len(parts)<4: continue
        t,pred,s=parts[:3]; m="\t".join(parts[3:])
        d[t][pred].append((s,m))
    return d
h=load(sys.argv[1]); b=load(sys.argv[2])
preds=sorted({p for d in (h,b) for t in d for p in d[t]})
for p in preds:
    reach=[t for t in set(h)|set(b) if p in h.get(t,{}) or p in b.get(t,{})]
    vmov=[];bmov=0;only=0
    for t in reach:
        hs=h.get(t,{}).get(p,[]); bs=b.get(t,{}).get(p,[])
        if t not in h or t not in b: only+=1; continue
        if [x[0] for x in hs]!=[x[0] for x in bs]: vmov.append(t)
        elif [x[1] for x in hs]!=[x[1] for x in bs]: bmov+=1
    print(f"{p}: reached {len(reach)} tests, verdict-seq moved {len(vmov)}, bits-only {bmov}, one-side-only {only}")
    for t in sorted(vmov)[:40]: 
        hs=[x[0] for x in h[t][p]]; bs=[x[0] for x in b[t][p]]
        print("   ",t, "base",collections.Counter(bs),"head",collections.Counter(hs))
