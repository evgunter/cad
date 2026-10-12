import itertools, random
# face: list of loops (lists of vertex classes). he index (loop, i) runs x_i -> x_{i+1}.
# true twin: each loop mirrored with its own shift s_l; true copy of he(l,i) is twin he (l, (s_l - i - 1) mod len).
def twin_copy(face, twin, i):  # flat, as the PR does
    m=len(face)
    if len(twin)!=m: return None
    ss=[s for s in range(m) if all(face[(s-j)%m]==twin[j] for j in range(m))]
    if len(ss)!=1: return None
    return (ss[0]-i-1)%m
import sys; NL=int(sys.argv[1]); random.seed(1)
wrong=none=right=0; ex=None
for trial in range(200000):
    nl=NL
    loops=[[random.randrange(4) for _ in range(random.randint(2,5))] for _ in range(nl)]
    shifts=[random.randrange(len(l)) for l in loops]
    tw=[[l[(s-j)%len(l)] for j in range(len(l))] for l,s in zip(loops,shifts)]
    order=list(range(nl)); random.shuffle(order) if random.random()<0.5 else None
    face=[x for l in loops for x in l]; twin=[x for k in order for x in tw[k]]
    # flat index map
    fidx=[(l,i) for l in range(nl) for i in range(len(loops[l]))]
    tidx=[(k,j) for k in order for j in range(len(loops[k]))]
    for f,(l,i) in enumerate(fidx):
        got=twin_copy(face,twin,f)
        truth=(l,(shifts[l]-i-1)%len(loops[l]))
        if got is None: none+=1
        elif tidx[got]==truth: right+=1
        else:
            wrong+=1
            if ex is None or (nl==1 and len(ex[0])>1): ex=(loops,shifts,order,f,tidx[got],truth)
print("right",right,"none",none,"wrong",wrong); print(ex)
