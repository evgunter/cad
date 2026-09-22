#!/usr/bin/env python3
"""Exact check of the bilinear census dump: (1) truth at corners and at the
sampler's argmax vs certified muu/muv/mvv, BARE; (2) sampler error in ulps."""
import sys, struct, importlib.util, statistics, math
from fractions import Fraction as F
spec = importlib.util.spec_from_file_location("ref", sys.argv[2]); ref = importlib.util.module_from_spec(spec); spec.loader.exec_module(ref)
def fh(h): return struct.unpack(">d", bytes.fromhex(h))[0]
def fr(h): return F(fh(h))
def norm2(v): return sum(x*x for x in v)
def sqrt_lower_f64(q):
    # largest f64 x with x*x <= q
    x = math.sqrt(float(q))
    while F(x)*F(x) > q: x = math.nextafter(x, -math.inf)
    while F(math.nextafter(x, math.inf))**2 <= q: x = math.nextafter(x, math.inf)
    return x
knots = [F(0),F(0),F(1),F(1)]
errs = {'uu':[], 'uv':[], 'vv':[]}
esc = []; trials = 0; worst_ratio = 0.0
for ln in open(sys.argv[1]):
    if not ln.startswith("TRIAL"): continue
    t = ln.split()
    w = [fr(h) for h in t[3].split(",")]; p = [fr(h) for h in t[5].split(",")]
    muu, muv, mvv = fh(t[7]), fh(t[9]), fh(t[11])
    arg = [fh(h) for h in t[13:22]]
    spec = {"degree_u":1,"degree_v":1,"nu":2,"nv":2,
            "knots_u":"0000000000000000,0000000000000000,3ff0000000000000,3ff0000000000000",
            "knots_v":"0000000000000000,0000000000000000,3ff0000000000000,3ff0000000000000",
            "weights":t[3], "control":t[5], "at":(F(0),F(0))}
    P = ref.Patch(spec)
    trials += 1
    cert = {'uu':muu,'uv':muv,'vv':mvv}
    # corners + argmax points
    pts = [(F(0),F(0)),(F(0),F(1)),(F(1),F(0)),(F(1),F(1)),(F(1,2),F(1,2))]
    for k in range(3):
        pts.append((F(arg[3*k]), F(arg[3*k+1])))
    for (u,v) in pts:
        jet = P.second_partials(1, 1, u, v)
        for comp in ('uu','uv','vv'):
            q = norm2(jet[comp])
            lower = sqrt_lower_f64(q)   # an f64 provably <= truth
            worst_ratio = max(worst_ratio, lower/cert[comp])
            if lower > cert[comp]:
                esc.append((trials-1, comp, float(u), float(v), lower, cert[comp], (lower-cert[comp])/(cert[comp]*2**-52)))
    for k, comp in enumerate(('uu','uv','vv')):
        u, v, sampled = F(arg[3*k]), F(arg[3*k+1]), arg[3*k+2]
        jet = P.second_partials(1, 1, u, v)
        q = norm2(jet[comp])
        truth = float(F(str(ref.decimal_of(q).sqrt())))
        errs[comp].append(abs(sampled - truth)/(cert[comp]*2**-52))
print(f"trials={trials} escapes(bare, exact truth rounded down)={len(esc)} worst truth/certified={worst_ratio:.17e}")
for e in esc[:20]: print("ESCAPE", e)
for comp in errs:
    a = sorted(errs[comp]); n=len(a)
    print(f"sampler error {comp}: median {statistics.median(a):.3f} p99 {a[int(0.99*(n-1))]:.3f} max {a[-1]:.3f} ulps of certified")
