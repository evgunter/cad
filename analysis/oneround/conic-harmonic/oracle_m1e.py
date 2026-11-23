#!/usr/bin/env python3
"""M1 oracle for topo's `plane_roots`: the truth is the unit circle as
stored against the plane through the ellipse's STORED point (its anchor)
with its stored axis, in 60-digit arithmetic."""
import json, sys
from fractions import Fraction
from mpmath import mp, mpf, sqrt, atan2, acos, cos, sin, pi
mp.dps = 60
F = lambda x: mpf(Fraction(float(x)).numerator) / mpf(Fraction(float(x)).denominator)
V = lambda v: [F(x) for x in v]
dot = lambda a, b: sum(x * y for x, y in zip(a, b))
cross = lambda a, b: [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
wrap = lambda x: x - 2 * pi * mp.nint(x / (2 * pi))
for eps in ("1e-9", "1e-12"):
    e = float(eps)
    print(f"plane_roots, eps {eps}:  a  gap(asked)  true gap  slack | (i) verdict, worst root miss on the unit circle | (ii) ...")
    rows = {}
    for l in open(f"{sys.argv[1]}/m1e-eps{eps}.jsonl"):
        r = json.loads(l)
        S, n, c, ax, cu = V(r["S"]), V(r["n"]), V(r["c"]), V(r["caxis"]), V(r["cu"])
        cv = cross(ax, cu)
        A, B, D = dot(n, cu), dot(n, cv), dot(n, [x - y for x, y in zip(c, S)])
        rho = sqrt(A * A + B * B)
        gap = rho - abs(D)
        miss = None
        if gap > 0 and r["roots"]:
            phi = atan2(B, A); off = acos(-D / rho)
            truths = [phi + off, phi - off]
            miss = max(min(abs(wrap(F(t) - x)) for x in truths) for t in r["roots"])
        truth = "meets" if gap > 10 * e else ("misses" if gap < -10 * e else "band")
        v = r["kind"]
        wrong = (v == "meets" and truth == "misses") or (v == "misses" and truth == "meets")
        rows.setdefault((r["a"], r["case"]), [r["gap"], float(gap), r["slack"], truth])
        rows[(r["a"], r["case"])].append(f"{v}{'(WRONG)' if wrong else ''} {'' if miss is None else '%.1e' % float(miss)}")
    for (a, i), x in sorted(rows.items()):
        print(f"  {a:.0e} {x[0]:8.1e} {x[1]:10.3e} {x[2]:.1e} truth {x[3]:6} | {x[4]:24} | {x[5]}")
