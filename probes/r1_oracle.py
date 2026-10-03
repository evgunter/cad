"""Reviewer r1 oracle for PR #3973 (independent of scripts/oracles/).

Reads R1ET JSON lines (probes/r1_ellipse_torus_probe.rs) and, at 60 digits
on the exact f64 inputs, finds every real root over the whole turn of the
torus implicit F along the stored ellipse C(t) = c + a u cos t + b (n x u) sin t
(F's 9 trig coefficients by an mpmath DFT of 32 direct evaluations, then the
degree-8 polynomial z^4 F(z) by mpmath.polyroots; a root is real when
||z| - 1| < 1e-25). Checks: certified count == true count; every certified
root within eps of arc of a true root; no Miss with a real root.
"""
import json, sys
import mpmath as mp
mp.mp.dps = 60

def V(x): return [mp.mpf(t) for t in x]
def dot(p, q): return sum(a*b for a, b in zip(p, q))
def cross(p, q): return [p[1]*q[2]-p[2]*q[1], p[2]*q[0]-p[0]*q[2], p[0]*q[1]-p[1]*q[0]]

def setup(d):
    c, n, u, hub, ax = V(d["c"]), V(d["n"]), V(d["u"]), V(d["hub"]), V(d["ax"])
    a, b, R, r = mp.mpf(d["a"]), mp.mpf(d["b"]), mp.mpf(d["R"]), mp.mpf(d["r"])
    v = cross(n, u)
    def P(t):
        ct, st = mp.cos(t), mp.sin(t)
        return [c[i] + u[i]*a*ct + v[i]*b*st for i in range(3)]
    def F(t):
        q = [x - y for x, y in zip(P(t), hub)]
        S, h = dot(q, q), dot(q, ax)
        return (S + R*R - r*r)**2 - 4*R*R*(S - h*h)
    def dist(t):
        q = [x - y for x, y in zip(P(t), hub)]
        h = dot(q, ax)
        w = [q[i] - ax[i]*h for i in range(3)]
        rho = mp.sqrt(dot(w, w))
        return mp.sqrt((rho - R)**2 + h*h) - r
    def speed(t):
        return mp.sqrt((a*mp.sin(t))**2 + (b*mp.cos(t))**2)
    return F, dist, speed

def true_roots(F):
    N = 32
    vals = [F(2*mp.pi*j/N) for j in range(N)]
    coef = []  # complex f_k, k=-4..4
    for k in range(-4, 5):
        coef.append(sum(vals[j]*mp.expjpi(-2*mp.mpf(k)*j/N) for j in range(N))/N)
    # z^4 F = sum_k f_k z^(k+4); polyroots wants highest degree first
    poly = list(reversed(coef))
    roots = mp.polyroots(poly, maxsteps=400, extraprec=400)
    out = []
    for z in roots:
        if abs(abs(z) - 1) < mp.mpf('1e-25'):
            out.append(mp.arg(z))
    nearpairs = sorted(float(abs(abs(z)-1)) for z in roots if abs(abs(z)-1) >= mp.mpf('1e-25'))
    return out, nearpairs

def main(path):
    bad, stats = [], {}
    for line in open(path):
        line = line.strip()
        if not line.startswith("{"): continue
        d = json.loads(line)
        F, dist, speed = setup(d)
        tr, near = true_roots(F)
        eps = d["eps"]
        key = (d["fam"], d["scale"], eps)
        st = stats.setdefault(key, {"certified": 0, "miss": 0, "uncertain": 0, "err": 0})
        ans = d["ans"]
        tag = ans if ans in st else "err"
        st[tag] += 1
        nt = len(tr)
        mind = None
        if ans == "certified":
            if len(d["roots"]) != nt:
                bad.append(("COUNT", nt, len(d["roots"]), near[:2], d))
            worst = 0.0
            for t in d["roots"]:
                t = mp.mpf(t)
                if not tr:
                    worst = float('inf'); break
                off = min(abs(mp.atan2(mp.sin(t - x), mp.cos(t - x))) for x in tr) * speed(t)
                worst = max(worst, float(off))
            if worst > eps:
                bad.append(("PLACE", worst / eps, nt, d))
            d["_worst_bands"] = worst / eps
        elif ans == "miss":
            if nt:
                bad.append(("MISS_ON_CROSSING", nt, d))
        elif ans.startswith("err") or ans.startswith("other"):
            bad.append(("ERR", ans, d))
    for k in sorted(stats): print(k, stats[k])
    print("FAILURES", len(bad))
    import collections
    print("BYKIND", dict(collections.Counter(b[0] for b in bad)))
    for b in [b for b in bad if b[0] != "ERR"][:40]: print(b[:2], b[-1].get("tag", ""), b[-1]["fam"], b[-1]["scale"], b[-1]["eps"])

if __name__ == "__main__":
    main(sys.argv[1])
