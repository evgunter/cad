import re, sys, mpmath as mp
mp.mp.dps = 40
def nums(s): return [mp.mpf(x) for x in re.findall(r'-?[0-9][0-9.e+-]*', s)]
def vec(s, name):
    m = re.search(name + r': (?:Point3|Vec3) \{ x: ([^,]+), y: ([^,]+), z: ([^ ]+) \}', s)
    return mp.matrix([mp.mpf(m.group(i)) for i in (1,2,3)])
def fld(s, name): return mp.mpf(re.search(name + r': ([-0-9.e+]+)', s).group(1))
def cross(a,b): return mp.matrix([a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]])
def dot(a,b): return sum(a[i]*b[i] for i in range(3))
def norm(a): return mp.sqrt(dot(a,a))
rows = [l.rstrip('\n').split('|') for l in open(sys.argv[1])]
thr = float(sys.argv[2]) if len(sys.argv) > 2 else 0.999
worst = None
for fam, t0, t1, hi, env, sup, car, surf in rows:
    hi, env, sup = float(hi), float(env), float(sup)
    if sup < thr * hi: continue
    t0, t1 = mp.mpf(t0), mp.mpf(t1)
    if car.startswith('Circle'):
        c, n, u = vec(car,'center'), vec(car,'axis'), vec(car,'u_ref'); r = fld(car,'radius'); v = cross(n,u)
        P = lambda t: c + r*(mp.cos(t)*u + mp.sin(t)*v)
    else:
        o, d = vec(car,'origin'), vec(car,'dir')
        P = lambda t: o + t*d
    kind = surf.split(' ')[0]
    if kind == 'Cylinder': O = vec(surf,'origin')
    elif kind == 'Cone': O = vec(surf,'apex')
    else: O = vec(surf,'center')
    A = vec(surf,'axis'); A = A / norm(A)
    def dist(p):
        w = p - O; z = dot(w, A); rho = norm(w - z*A)
        if kind == 'Cylinder': return abs(rho - fld(surf,'radius'))
        if kind == 'Cone':
            al = fld(surf,'half_angle'); return abs(rho*mp.cos(al) - abs(z)*mp.sin(al))
        R, rr = fld(surf,'major_radius'), fld(surf,'minor_radius')
        return abs(mp.sqrt((rho-R)**2 + z**2) - rr)
    N = 600
    best, bt = mp.mpf(0), t0
    for i in range(N+1):
        t = t0 + (t1-t0)*i/N; dd = dist(P(t))
        if dd > best: best, bt = dd, t
    h = (t1-t0)/N; lo, hi_ = max(bt-h, t0), min(bt+h, t1)
    for _ in range(50):
        m1 = lo + (hi_-lo)*mp.mpf('0.382'); m2 = lo + (hi_-lo)*mp.mpf('0.618')
        if dist(P(m1)) > dist(P(m2)): hi_ = m2
        else: lo = m1
    best = max(best, dist(P((lo+hi_)/2)))
    ratio = float(best)/hi
    flag = 'UNDER' if best > hi else ''
    print(f"fam {fam} {kind}: hp {float(best):.10e} hi {hi:.10e} env64 {env:.10e} sup64 {sup:.10e} hp/hi {ratio:.8f} hp/env64 {float(best)/env if env>0 else 0:.6f} {flag}")
