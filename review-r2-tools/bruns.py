import itertools
def run_order(n,p0,p1):
    a=(p0+1)%n==p1; b=(p1+1)%n==p0
    if a and b: return ('some',None)
    if a: return ('some',False)
    if b: return ('some',True)
    return None
def b_runs(n,pairs,b_pos):
    spans=[(min(b_pos[i0],b_pos[i1]),max(b_pos[i0],b_pos[i1])) for i0,i1 in pairs]
    for k,(lo,hi) in enumerate(spans):
        for lo2,hi2 in spans[k+1:]:
            if (lo<lo2<hi)!=(lo<hi2<hi): return None
    runs=[]
    for i0,i1 in pairs:
        p0,p1=b_pos[i0],b_pos[i1]
        r=run_order(n,p0,p1)
        runs.append(r[1] if r else (p1<p0))
    def interval(k):
        i0,i1=pairs[k]
        return (b_pos[i1],b_pos[i0]) if runs[k] is True else (b_pos[i0],b_pos[i1])
    def holds(o,p):
        f,t=interval(o); return f<t and f<p<t
    out=[]
    for k in range(len(pairs)):
        i0,_=pairs[k]
        hs=[o for o in range(len(pairs)) if o!=k and holds(o,b_pos[i0])]
        h=min(hs,key=lambda o:interval(o)[1]-interval(o)[0]) if hs else None
        out.append((runs[k],h))
    return out,[interval(k) for k in range(len(pairs))]
def ncm(pts):
    if not pts: yield []; return
    a=pts[0]
    for j in range(1,len(pts),2):
        for l in ncm(pts[1:j]):
            for r in ncm(pts[j+1:]):
                yield [(a,pts[j])]+l+r
bad=0; tot=0; maxdepth={}
for n in (4,6,8,10):
    for m in ncm(list(range(n))):
        # every reading of B: rotation of positions, and each pair either orientation, any pair order
        for rot in range(n):
            for flips in itertools.product([0,1],repeat=n//2):
                tot+=1
                pairs=[]; b_pos=[0]*n
                # germ ids = positions rotated
                for idx,(x,y) in enumerate(m):
                    gx,gy=(x+rot)%n,(y+rot)%n  # germ ids
                    pairs.append((gy,gx) if flips[idx] else (gx,gy))
                for g in range(n): b_pos[g]=g  # germ id == B position after rotation
                res=b_runs(n,pairs,b_pos)
                if res is None: bad+=1; print("None",n,m,rot); continue
                out,iv=res
                # checks: each run's interval (forward from->to) contains exactly whole pairs; adjacency respected
                for k,(i0,i1) in enumerate(pairs):
                    f,t=iv[k]
                    inside=set(range(f+1,t)) if f<t else set()
                    # walking forward cyclically from f to t:
                    cyc=set(); p=(f+1)%n
                    while p!=t: cyc.add(p); p=(p+1)%n
                    if f>t and cyc: bad+=1; print("wrap run holds",n,m,rot,k,iv[k],cyc)
                    if f<t and cyc!=inside: bad+=1; print("mismatch",n,m)
                    held=[j for j in range(len(pairs)) if j!=k and b_pos[pairs[j][0]] in cyc]
                    for j in held:
                        if b_pos[pairs[j][1]] not in cyc: bad+=1; print("partial",n,m)
                    # direct holder = innermost
                    h=out[k][1]
                    # depth
                d=0
                for k in range(len(pairs)):
                    dd=0;h=out[k][1]
                    while h is not None: dd+=1;h=out[h][1]
                    d=max(d,dd)
                maxdepth[n]=max(maxdepth.get(n,0),d)
print("total",tot,"bad",bad,"maxdepth",maxdepth)
# crossing matchings must be refused
cr=0
for n in (4,6,8):
    for perm in itertools.permutations(range(n)):
        pairs=[(perm[2*i],perm[2*i+1]) for i in range(n//2)]
        def crosses(p,q):
            a,b=sorted(p);c,d=sorted(q); return (a<c<b)!=(a<d<b)
        anyc=any(crosses(p,q) for p,q in itertools.combinations(pairs,2))
        r=b_runs(n,pairs,list(range(n)))
        if (r is None)!=anyc: cr+=1
print("crossing-refusal disagreements",cr)
