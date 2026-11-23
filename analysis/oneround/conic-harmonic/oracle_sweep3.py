#!/usr/bin/env python3
"""Sweep item 3: checks the probe's point p against the CONE and the PLANE
themselves in 60-digit mpmath (no ellipse, no kernel code), and reports
what `curve_pair::meet` answered for the line through p."""
import json, sys
from fractions import Fraction
from mpmath import mp, mpf, sqrt, cos, sin
mp.dps = 60
F = lambda x: mpf(Fraction(float(x)).numerator) / mpf(Fraction(float(x)).denominator)
V = lambda v: [F(x) for x in v]
dot = lambda a, b: sum(x * y for x, y in zip(a, b)); sub = lambda a, b: [x - y for x, y in zip(a, b)]; norm = lambda a: sqrt(dot(a, a))
for tree in ("pre7203", "branch7203"):
    for eps in ("1e-9", "1e-12"):
        print("==", tree, "eps", eps)
        for l in open(f"{sys.argv[1]}/{tree}-{eps}.txt"):
            r = json.loads(l.split(" ", 1)[1])
            if "mint" in r: print("  ", r); continue
            apex, ax, q, n, p = V(r["apex"]), V(r["axis"]), V(r["q"]), V(r["n"]), V(r["p"])
            w = sub(p, apex); h = dot(w, ax) / norm(ax); rho = sqrt(dot(w, w) - h * h); al = F(r["alpha"])
            cone = abs(rho * cos(al) - abs(h) * sin(al)); plane = abs(dot(sub(p, q), n)) / norm(n)
            off = "" if r["meet"] is None else " ; meeting off p %.1e" % float(norm(sub(V(r["meet"]), p)))
            print(f"  a={r['a']:.2e} az={r['azimuth']:5} |p-apex| {float(norm(w)):.1e}  p off cone {float(cone):.1e} plane {float(plane):.1e} | stored section's foot off p {r['foot_miss']:.1e} | {r['kind'][:40]}{off}")
