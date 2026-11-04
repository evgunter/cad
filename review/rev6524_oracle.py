"""50-digit residual oracle for review_probe_6524 rows (reviewer's own)."""
import sys
import mpmath as mp
mp.mp.dps = 50
F = lambda x: mp.mpf(float(x))
def dot(a,b): return sum(x*y for x,y in zip(a,b))
def sub(a,b): return [x-y for x,y in zip(a,b)]
def cross(a,b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def unit(a):
    n = mp.sqrt(dot(a,a)); return [x/n for x in a]
def res_fn(kind, o, ax, p1, p2, nappe):
    ax = unit(ax)
    def split(q):
        d = sub(q, o); h = dot(d, ax); wv = [x - y*h for x, y in zip(d, ax)]
        return h, mp.sqrt(dot(wv, wv))
    if kind == "sphere":
        return lambda q: abs(mp.sqrt(dot(sub(q,o),sub(q,o))) - p1)
    if kind == "cylinder":
        return lambda q: abs(split(q)[1] - p1)
    if kind == "cone":
        sa, ca = mp.sin(p1), mp.cos(p1)
        def f(q):
            h, rho = split(q); return abs(rho*ca - nappe*h*sa)
        return f
    if kind == "torus":
        def f(q):
            h, rho = split(q); return abs(mp.hypot(rho - p1, h) - p2)
        return f
def main():
    worst = {}
    bad = []
    n = 0
    for line in open(sys.argv[1]):
        if not line.startswith("PROBE "): continue
        parts = [x.strip() for x in line[6:].split("|")]
        fam = parts[0]
        ch = parts[1].split(); kind = ch[0]; nums = [F(x) for x in ch[1:]]
        o, ax, p1, p2 = nums[0:3], nums[3:6], nums[6], nums[7]
        el = [F(x) for x in parts[2].split()]
        c, nrm, u, a, b = el[0:3], el[3:6], el[6:9], el[9], el[10]
        # the kernel's frame: u_ref as stored (unit), v = axis x u_ref
        v = cross(nrm, u)
        t0, t1 = [F(x) for x in parts[3].split()]
        nappe = int(parts[4])
        env, lo, hi, sampled = [float(x) for x in parts[5].split()]
        r = res_fn(kind, o, ax, p1, p2, nappe)
        C = lambda t: [c[i] + a*u[i]*mp.cos(t) + b*v[i]*mp.sin(t) for i in range(3)]
        g = lambda t: r(C(t))
        N = 400
        ts = [t0 + (t1-t0)*i/N for i in range(N+1)]
        vals = [g(t) for t in ts]
        best = max(range(N+1), key=lambda i: vals[i])
        m = vals[best]
        # refine around the 3 best samples by golden section
        for i in sorted(range(N+1), key=lambda i: -vals[i])[:3]:
            lo_t, hi_t = ts[max(i-1,0)], ts[min(i+1,N)]
            for _ in range(80):
                m1 = lo_t + (hi_t-lo_t)*mp.mpf('0.381966'); m2 = hi_t - (hi_t-lo_t)*mp.mpf('0.381966')
                if g(m1) > g(m2): hi_t = m2
                else: lo_t = m1
            m = max(m, g((lo_t+hi_t)/2))
        n += 1
        md = float(m)
        for name, e in (("f64", env), ("iv_hi", hi), ("iv_lo", lo)):
            if e != e: continue
            ratio = e / md if md > 0 else float('inf')
            key = (kind, fam, name)
            if key not in worst or ratio < worst[key][0]:
                worst[key] = (ratio, md, e)
            if name != "iv_lo" and e < md:
                bad.append((fam, name, e, md, line.strip()))
    print("rows", n)
    for k in sorted(worst): print(k, "tightest env/residual %.6g (residual %.3e env %.3e)" % worst[k])
    print("UNDER:", len(bad))
    for x in bad[:20]: print(x)
main()
