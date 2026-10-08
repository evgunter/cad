import itertools
def on_arc(b,lo,hi,x):
    pl=b(lo,x)
    if b(hi,lo)!=True: return [pl,b(x,hi)]
    return [True,b(x,hi)] if pl==False else [pl,True]
b=lambda p,q: None if p==q else p<q
def holds_new(o,i): return [False not in on_arc(b,o[0],o[1],x) for x in i]
def holds_old(n,o,i):
    L=(o[1]-o[0])%n
    return [0<(p-o[0])%n<L for p in i]
bad=0;tot=0
for m in range(1,4):
    n=2*m
    for perm in itertools.permutations(range(n)):
        arcs=[(perm[2*k],perm[2*k+1]) for k in range(m)]
        for o in arcs:
            for i in arcs:
                if o==i: continue
                tot+=1
                if holds_new(o,i)!=holds_old(n,o,i): bad+=1; print(n,o,i,holds_old(n,o,i),holds_new(o,i)) if bad<5 else None
print("pairs",tot,"diffs",bad)
# shared endpoint
print("shared-end example: outer (0,3) inner (0,2): old",holds_old(4,(0,3),(0,2)),"new",holds_new((0,3),(0,2)))
