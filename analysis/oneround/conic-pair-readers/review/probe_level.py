#!/usr/bin/env python3
"""Review probe (C3) oracle: judges probe_level.rs's lines at 60 digits.

The curve is the one the stored numbers state: S + a*u*(cos t - kc) +
b*(axis x u)*(sin t - ks), (kc, ks) = (cos theta0, sin theta0) with theta0
the printed quarter-turn multiple (so (1,0) or (0,-1) exactly). Nothing
calls kernel code. For each printed point q: the true level F (about the
true centre S - a*u*kc - b*v*ks, in exact arithmetic), the true height over
the plane through S, and the true distance from q to the curve.
"""
import json, sys, collections
from fractions import Fraction
from mpmath import mp, mpf, sqrt, cos, sin, findroot, fabs
mp.dps = 60
def F(x):
    f = Fraction(float(x)); return mpf(f.numerator) / mpf(f.denominator)
def V(v): return [F(x) for x in v]
def dot(a, b): return sum(x*y for x, y in zip(a, b))
def cross(a, b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def sub(a, b): return [x-y for x, y in zip(a, b)]
def norm(a): return sqrt(dot(a, a))

def judge(r):
    S, n, u = V(r["S"]), V(r["axis"]), V(r["u"])
    v = cross(n, u)
    a, b = F(r["a"]), F(r["b"])
    kc, ks = (mpf(1), mpf(0)) if r["seam"] == 0 else (mpf(0), mpf(-1))
    q = V(r["q"])
    w = sub(q, S)
    # the level as the kernel defines it: components along the stored (u, v), over (a, b)
    X = dot(w, u) + a*kc
    Y = dot(w, v) + b*ks
    Ftrue = (X/a)**2 + (Y/b)**2 - 1
    htrue = dot(w, n)
    at = lambda t: [s + a*uu*(cos(t)-kc) + b*vv*(sin(t)-ks) for s, uu, vv in zip(S, u, v)]
    d = lambda t: [-a*uu*sin(t) + b*vv*cos(t) for uu, vv in zip(u, v)]
    th = F(r["theta"])
    sp = norm(d(th))
    g = lambda x: dot(sub(at(th + x/sp), q), d(th + x/sp))/sp
    try:
        x = findroot(g, mpf(0), tol=mpf(10)**-45, maxsteps=100)
    except Exception:
        x = mpf(0)
    dist = norm(sub(at(th + x/sp), q))
    # in-plane distance (what the level clearance lower-bounds, signed)
    return dict(r, Ftrue=float(Ftrue), htrue=float(htrue), dist=float(dist),
                Ferr=float(F(r["F"]) - Ftrue), herr=float(F(r["h"]) - htrue))

def main(path):
    out = [judge(json.loads(l)) for l in open(path)]
    json.dump(out, open(path.replace(".jsonl", ".judged.json"), "w"))
    tab = collections.defaultdict(lambda: [0.0, 0.0, 0.0, 0])
    wrong = []
    for j in out:
        zero, esc = j["zero"], j["esc"]
        place = "near" if abs(j["theta"] - j["theta0"]) < 0.3 else "far"
        key = (j["shape"], j["seam"], j["a"], place)
        t = tab[key]
        # F's error as metres: over |grad F|
        g = (j["gx"]**2 + j["gy"]**2) ** 0.5
        t[0] = max(t[0], abs(j["Ferr"]) / g)
        t[1] = max(t[1], abs(j["herr"]))
        # clearance is documented a LOWER bound on the distance
        over = j["cl"] - j["dist"]
        t[2] = max(t[2], over)
        t[3] += 1
        v = j["verdict"]
        if (v == "off" and j["dist"] <= esc) or (v == "on" and j["dist"] > esc):
            kind = "WRONG" if (v == "off" and j["dist"] <= zero) or (v == "on" and j["dist"] > esc) else "GAP"
            wrong.append((kind, j))
    print("shape seam a place | max |F err|/|gradF| (m) | max |h err| (m) | max (clearance - true dist) (m) | n")
    for k in sorted(tab):
        t = tab[k]
        print(*k, "| %.2e | %.2e | %.2e | %d" % tuple(t), sep="\t")
    for kind, j in wrong:
        print(kind, {k: j[k] for k in ("shape", "seam", "a", "theta", "dir", "k", "verdict", "dist", "cl", "lc", "F", "Ftrue", "h", "htrue")})
    print("definite-wrong:", sum(1 for k, _ in wrong if k == "WRONG"), "definite-in-gap:", sum(1 for k, _ in wrong if k == "GAP"), "of", len(out))

if __name__ == "__main__":
    main(sys.argv[1])
