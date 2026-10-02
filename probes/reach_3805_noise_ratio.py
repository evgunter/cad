"""Is `noise = 8u·terms/2r` (NOISE_ULPS = 16 half-ulps) a bound on the
harmonics' rounding sup|E|, the premise of the Bernstein charge? The f64
harmonics replicate `geom_brep::quadric_harmonics` (sphere arm) operation
by operation in Python floats (IEEE binary64); the exact ones are taken
in 60-digit decimal from the same stored data. Prints the largest
sup|E| / noise over random poses."""
import random, math
from decimal import Decimal as D, getcontext
getcontext().prec = 60
u = 2.0**-52
def cross(a, b):
    return (a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0])
def dot(a, b): return a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
def unit(rng):
    while True:
        v = [rng.uniform(-1,1) for _ in range(3)]
        n = math.sqrt(dot(v,v))
        if 0.2 < n < 1: return tuple(x/n for x in v)
def harm(center, axis, u_ref, a, b, o, r, Z):
    # Z: number constructor (float or Decimal)
    c = tuple(Z(x) for x in center); ax = tuple(Z(x) for x in axis); up = tuple(Z(x) for x in u_ref)
    a, b, r = Z(a), Z(b), Z(r); o = tuple(Z(x) for x in o)
    vp = cross(ax, up)
    d = tuple(c[i]-o[i] for i in range(3)); e = d
    aa = (a*a) * dot(up, up); bb = (b*b) * dot(vp, vp)
    per = Z(2)*r; half = Z(1)/Z(2)
    c0 = (dot(e,e) + (aa+bb)*half - r*r)/per
    c1 = Z(2)*a*dot(e,up)/per; s1 = Z(2)*b*dot(e,vp)/per
    c2 = (aa-bb)*half/per; s2 = a*b*dot(up,vp)/per
    dn = (dot(d,d)).sqrt() if Z is D else math.sqrt(dot(d,d))
    terms = (dn + max(abs(a),abs(b)))**2 + r*r
    return (c0,c1,s1,c2,s2), terms
rng = random.Random(7); worst = (0, None)
for i in range(4000):
    n = unit(rng); v = unit(rng); w = tuple(v[k]-n[k]*dot(v,n) for k in range(3))
    wn = math.sqrt(dot(w,w)); u_ref = tuple(x/wn for x in w)
    S = rng.choice([1.0, 1000.0]); big = S*rng.uniform(0.5,5); small = big/rng.uniform(1,40)
    a, b = (big, small) if i % 2 == 0 else (small, big)
    far = rng.choice([1.0, 1000.0]); center = tuple(rng.uniform(-far,far) for _ in range(3))
    th = rng.choice([0, math.pi/2, math.pi, 3*math.pi/2]); r = 10**rng.uniform(-6,0)
    p = tuple(center[k] + u_ref[k]*a*math.cos(th) + cross(n,u_ref)[k]*b*math.sin(th) for k in range(3))
    out = tuple(p[k]-center[k] for k in range(3)); on = math.sqrt(dot(out,out))
    o = tuple(p[k] + out[k]/on*r for k in range(3))
    hf, terms = harm(center, n, u_ref, a, b, o, r, float)
    hd, _ = harm(center, n, u_ref, a, b, o, r, D)
    dl = [hf[k] - float(hd[k]) for k in range(5)]  # differences exact enough (hd to 60 digits)
    dl = [float(D(hf[k]) - hd[k]) for k in range(5)]
    sup = max(abs(dl[0] + dl[1]*math.cos(t) + dl[2]*math.sin(t) + dl[3]*math.cos(2*t) + dl[4]*math.sin(2*t))
              for t in [2*math.pi*j/512 for j in range(512)])
    noise = 16*u*0.5*terms/(2*r)
    if sup/noise > worst[0]: worst = (sup/noise, (i, S, far, a, b, r))
print("largest sup|E|/noise over 4000 poses:", worst)
