import itertools
def cmpr(a,b): return None if a==b else a<b
def bfix(p,q):
    return cmpr(p[1],q[1]) if p[0]==q[0] else p[0]<q[0]
def bold(o,n):
    def f(p,q):
        if p[0]==q[0]: return cmpr(p[1],q[1])
        r=lambda e:(e-o)%n
        return r(p[0])<r(q[0])
    return f
def arm(r):
    if False in r: return 'F'
    if r[0] is None: return 'TL'
    if r[1] is None: return 'TH'
    return 'T'
def on_arc(b,lo,hi,x):
    pl=b(lo,x)
    if b(hi,lo)!=True: return [pl,b(x,hi)]
    return [True,b(x,hi)] if pl==False else [pl,True]
diff={}
for n in range(1,6):
    cuts=[(e,r) for e in range(n) for r in range(3)]
    for lo,hi,x in itertools.product(cuts,repeat=3):
        if lo==hi: continue
        bo=bold(lo[0],n)
        old=arm([bo(lo,x),bo(x,hi)]); new=arm(on_arc(bfix,lo,hi,x))
        if old!=new:
            k=('same-entry' if lo[0]==hi[0] else 'diff-entry')
            diff.setdefault(k,[]).append((n,lo,hi,x,old,new))
for k,v in diff.items(): print(k,len(v),v[:4])
print("fan rows (lo.0!=hi.0) differing:",len(diff.get('diff-entry',[])))
print("same-entry diffs with lo before hi:", sum(1 for d in diff.get('same-entry',[]) if d[1][1]<d[2][1]))
