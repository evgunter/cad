---
id: the-sweep-oracles-convex-volume-misreads-some-corner-pair-poses
kind: issue
title: The pierce sweep's oracle convex_volume misreads some corner-pair poses, so a want can be wrong
status: open
opened: 2026-10-05
priority: P1
cost: M
---


## What

Found by PR 4061's review (NOTE 2). It is pre-existing: the lines are
identical on main.

The sweep's oracle clips each corner's convex pieces pairwise with
`convex_volume` (`crates/sweep/tests/join_pierce_runs_sweep.rs`,
`convex_volume` and its caller `corner_pair_runs`). At some poses it
returns a common volume the kernel does not build, and a Monte Carlo
over the same pieces sides with the kernel.
- `corner_pairs_battery` prints 154 `OK BAD` lines with `t2`, `t3p`,
  `cert` and `operand` all true and only the volume off `want`.
- Example: `w343-w330 xy psi=0.7000 ab I` gives `v=4.317829269`
  against `want=5.091613879`.
- A pure-Python Monte Carlo of `wedge(343)` ∩ `wedge(330)` turned by
  `frame([1, 1, 0], 0.7)`, 300 000 samples, seed 1, gives common
  ≈ 4.3286 (σ ≈ 0.013). Its script is below.

An oracle that can be wrong can also certify a wrong body: a kernel
volume that matched a wrong `want` would read `SOUND`.

## Repro

`cargo test -p sweep --release --test all corner_pairs_battery --
--ignored --nocapture | grep 'OK BAD' | grep t3p=true`

```python
import math, random
def dot(a,b): return sum(a[i]*b[i] for i in range(3))
def cross(a,b): return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
def unit(v): l=math.sqrt(dot(v,v)); return [x/l for x in v]
def frame(m,psi):
    m=unit(m); seed=[0,0,1.] if abs(m[2])<0.9 else [1.,0,0]
    u0=unit(cross(seed,m)); w0=cross(m,u0); c,s=math.cos(psi),math.sin(psi)
    u=[c*u0[i]+s*w0[i] for i in range(3)]; w=cross(m,u); return [u,w,m]
def wedge(alpha):
    def at(t):
        t=math.radians(t); r=2/max(abs(math.cos(t)),abs(math.sin(t))); return (r*math.cos(t),r*math.sin(t))
    prof=[(0,0),at(0)]; c=45
    while c<alpha: prof.append(at(c)); c+=90
    prof.append(at(alpha))
    return [[prof[0],prof[k],prof[k+1]] for k in range(1,len(prof)-1)],[0,0,1.]
def poly_prism(poly):
    P=[]
    for i in range(len(poly)):
        p,q=poly[i],poly[(i+1)%len(poly)]; out=[q[1]-p[1],p[0]-q[0],0]; P.append((out,out[0]*p[0]+out[1]*p[1]))
    return P+[([0,0,1.],1.),([0,0,-1.],0.)]
def run(aa,ab,m,psi,N=300000):
    pa,va=wedge(aa); pb,vb=wedge(ab); f=frame(m,psi)
    turn=lambda n:[f[0][t]*n[0]+f[1][t]*n[1]+f[2][t]*n[2] for t in range(3)]
    A=[poly_prism(p) for p in pa]
    B=[[(turn(n), d-dot(n,vb)+dot(turn(n),va)) for n,d in poly_prism(p)] for p in pb]
    ins=lambda P,x: all(dot(n,x)<=d for n,d in P)
    random.seed(1); ka=kab=0
    for _ in range(N):
        x=[random.uniform(-2,2),random.uniform(-2,2),random.uniform(0,1)]
        if any(ins(P,x) for P in A):
            ka+=1
            if any(ins(P,x) for P in B): kab+=1
    return 16*ka/N,16*kab/N
print(run(343,330,[1,1,0],0.7))  # (15.385, 4.3286)
```
