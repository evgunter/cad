# Exact (50-digit) cone-form coefficients of the dumped stored inputs,
# against the f64 coefficients and the charge rounding_charge(terms).
import sys, mpmath as mp
mp.mp.dps = 50
def V(s): return [mp.mpf(float(x)) for x in s.split(',')]
dot = lambda a,b: a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
cross = lambda a,b: [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
rows = []
for l in open(sys.argv[1]):
    p = l.strip().split('|'); sp = p[0].split(';'); cp = p[1].split(';')
    O, A, al = V(sp[1]), V(sp[2]), mp.mpf(float(sp[3]))
    c, n, u, r = V(cp[1]), V(cp[2]), V(cp[3]), mp.mpf(float(cp[4]))
    v = cross(n, u); d = [c[i]-O[i] for i in range(3)]
    perp = lambda x: [x[i]-A[i]*dot(A,x) for i in range(3)]
    e, up, vp = perp(d), perp(u), perp(v)
    aa, bb = r*r*dot(up,up), r*r*dot(vp,vp)
    wall=[dot(e,e)+(aa+bb)/2, 2*r*dot(e,up), 2*r*dot(e,vp), (aa-bb)/2, r*r*dot(up,vp)]
    h0,hu,hv = dot(d,A), r*dot(u,A), r*dot(v,A)
    height=[h0*h0+(hu*hu+hv*hv)/2, 2*h0*hu, 2*h0*hv, (hu*hu-hv*hv)/2, hu*hv]
    cc, ss = mp.cos(al)**2, mp.sin(al)**2
    ex = [cc*wall[k]-ss*height[k] for k in range(5)]
    f64 = [mp.mpf(float(x)) for x in p[2:7]]; charge = float(p[7])
    # per-coefficient error and the sup-bound error (|c0| + hyp + hyp)
    err = max(abs(f64[k]-ex[k]) for k in range(5))
    supf = abs(f64[0]) + mp.hypot(f64[1],f64[2]) + mp.hypot(f64[3],f64[4])
    supx = abs(ex[0]) + mp.hypot(ex[1],ex[2]) + mp.hypot(ex[3],ex[4])
    deficit = supx - supf   # what the charge must cover
    rows.append((float(deficit)/charge if charge > 0 else float('inf'), float(err)/charge, float(al), sp[1][:20], float(h0), float(deficit), charge))
rows.sort(reverse=True)
print("worst (sup deficit)/charge, (max coeff err)/charge, alpha, apex, h0, deficit, charge:")
for x in rows[:15]: print("  %.3g  %.3g  a=%.4g apex=%s h0=%.3g def=%.3g charge=%.3g" % x)
print("rows", len(rows), "deficit > charge:", sum(1 for x in rows if x[0] > 1), "deficit > 2*charge (conic_form_sup's):", sum(1 for x in rows if x[0] > 2))
