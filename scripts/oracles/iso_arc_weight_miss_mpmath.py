#!/usr/bin/env python3
"""The high-precision oracle for an arc rim's weight miss.

An arc rim's chart column is `n` rational-quadratic sub-arcs; where a sub-arc's
interior weight at unit end weights is `w = cos(h/2) + d` rather than
`c = cos(h/2)`, the column misses the circle the image's map reads it as. The
certificate pays that miss as `geom_brep`'s `arc_rim_weight_miss`:

    R·|d|·sin(a)·sqrt(tan(a)^2 + 16/27) / ((1 + min(w, 1))·(1 + min(c, 1))),  a = h/2.

Two modes, at 50 digits:

- `derive` (no argument): the miss of a sub-arc built at held end weights
  (e0, w·sqrt(e0·e1), e1), read through the map's Möbius change, against the
  circle at the same carrier parameter. It checks the closed form the bound
  rests on, the reviewer's middle-of-arc radial reading, and then searches
  half-angles up to a half turn, end-weight ratios 1e-15..1e15 and weight
  errors of both signs for a sub-arc whose sup exceeds the bound.
- `<dump>`: the JSON lines `geom_brep`'s `iso_arc_weight_search` writes under
  `CAD_ISO_ARC_DUMP` (one certified row each: its column's control points and
  held weights, the sub-arc angle, the radius, the certificate's envelope). Each
  row's true sup of |B(map(t)) − C(t)| over its arc is re-derived from the
  dumped column alone and compared with the envelope.

Exits non-zero on any violation. Run:
`python3 scripts/oracles/iso_arc_weight_miss_mpmath.py [<dump>]`.
"""

import json
import random
import sys

from mpmath import atan, cos, mp, mpf, sin, sqrt, tan

mp.dps = 50


def bound(a, d):
    c, w = cos(a), cos(a) + d
    return (
        abs(d)
        * sin(a)
        * sqrt(tan(a) ** 2 + mpf(16) / 27)
        / ((1 + min(w, 1)) * (1 + min(c, 1)))
    )


def closed_form(a, d, s):
    c, w, v = cos(a), cos(a) + d, (2 * s - 1) ** 2
    num = abs(d) * sin(a) * (1 - v) * sqrt(tan(a) ** 2 * (1 + v) ** 2 + 4 * v)
    return num / (((1 + w) + v * (1 - w)) * ((1 + c) + v * (1 - c)))


def rq(p, wt, s):
    """A rational quadratic Bézier with control points p and weights wt, at s."""
    b = [(1 - s) ** 2 * wt[0], 2 * s * (1 - s) * wt[1], s**2 * wt[2]]
    den = sum(b)
    return [sum(b[i] * p[i][k] for i in range(3)) / den for k in range(2)]


def standard_s(phi, h):
    return mpf(1) / 2 + tan(phi / 2) / (2 * tan(h / 4))


def mobius(s, e0, e1):
    lam = sqrt(e0 / e1)
    return lam * s / (lam * s + 1 - s)


def unit_sub_arc_miss(a, e0, e1, d, s):
    """A sub-arc of half-angle a about +x on the unit circle at standard
    parameter s: (pointwise miss, radial miss)."""
    c = cos(a)
    p = [(c, -sin(a)), (1 / c, mpf(0)), (c, sin(a))]
    b = rq(p, [e0, (c + d) * sqrt(e0 * e1), e1], mobius(s, e0, e1))
    phi = 2 * atan((2 * s - 1) * tan(a / 2))
    dx, dy = b[0] - cos(phi), b[1] - sin(phi)
    return sqrt(dx * dx + dy * dy), abs(sqrt(b[0] ** 2 + b[1] ** 2) - 1)


def sup(f, lo=None, hi=None, n=200):
    lo = mpf(0) if lo is None else lo
    hi = mpf(1) if hi is None else hi
    xs = [lo + (hi - lo) * i / n for i in range(n + 1)]
    vals = [f(x) for x in xs]
    i = max(range(n + 1), key=lambda j: vals[j])
    a, b = xs[max(i - 1, 0)], xs[min(i + 1, n)]
    for _ in range(90):
        m1, m2 = a + (b - a) * mpf("0.381966"), a + (b - a) * mpf("0.618034")
        if f(m1) < f(m2):
            a = m1
        else:
            b = m2
    return max(vals[i], f((a + b) / 2))


def derive():
    bad = 0
    rng = random.Random(0x150A)  # noqa: S311 — a fixed oracle sample, not a secret
    worst = mpf(0)
    for _ in range(300):
        a = mpf(rng.uniform(1e-3, 1.5))
        e0, e1 = mpf(10) ** rng.uniform(-8, 8), mpf(10) ** rng.uniform(-8, 8)
        d = mpf(rng.choice([-1, 1])) * mpf(10) ** rng.uniform(-14, -3)
        s = mpf(rng.random())
        zero = unit_sub_arc_miss(a, e0, e1, 0, s)[0]
        m = unit_sub_arc_miss(a, e0, e1, d, s)[0]
        worst = max(worst, abs(m - closed_form(a, d, s)) / m, zero)
    print(f"closed form against the built sub-arc: worst {mp.nstr(worst, 3)}")
    if worst > mpf(10) ** -30:
        bad += 1
    a, d = mpf("0.4"), mpf("1e-9")
    c = cos(a)
    ratio = closed_form(a, d, mpf(1) / 2) / (d * (1 - c) / (c * (1 + c)))
    print(
        f"middle against R·d·(1-c)/(c(1+c)): {mp.nstr(ratio, 12)} (w = c + d in one factor)"
    )
    print("  h deg   sup/(R|d|)   radial sup/(R|d|)   bound/sup")
    for hdeg in [1, 10, 45, 90, 120, 129.43, 131.06, 150, 170, 179, 179.9]:
        a = mpf(hdeg) * mp.pi / 360
        s_pt = sup(lambda s, a=a: unit_sub_arc_miss(a, mpf(3), mpf(1), d, s)[0])
        s_rad = sup(lambda s, a=a: unit_sub_arc_miss(a, mpf(3), mpf(1), d, s)[1])
        print(
            f"  {hdeg:7}  {mp.nstr(s_pt / d, 8):>11}  {mp.nstr(s_rad / d, 8):>17}"
            f"  {mp.nstr(bound(a, d) / s_pt, 8):>10}"
        )
    lowest = mpf(10)
    for i in range(400):
        a = mpf(rng.uniform(1e-4, 1.5707)) if i % 4 else mpf(rng.uniform(1.4, 1.5707))
        e0, e1 = mpf(10) ** rng.uniform(-15, 15), mpf(1)
        d = mpf(rng.choice([-1, 1])) * mpf(10) ** rng.uniform(-15, -2)
        d = max(d, -cos(a) / 2)
        s_pt = sup(
            lambda s, a=a, e0=e0, e1=e1, d=d: unit_sub_arc_miss(a, e0, e1, d, s)[0],
            n=60,
        )
        r = bound(a, d) / s_pt
        lowest = min(lowest, r)
        if r < 1:
            bad += 1
            print(f"VIOLATION: a={a} e0={e0} d={d}: bound/sup {r}")
    print(f"search, 400 sub-arcs: lowest bound/sup {mp.nstr(lowest, 8)}")
    return bad


def dumped(path):
    bad, rows, lowest = 0, 0, None
    with open(path, encoding="utf-8") as fh:
        for line in fh:
            row = json.loads(line)
            r, h, n = mpf(row["radius"]), mpf(row["h"]), row["spans"]
            pts = [(mpf(x), mpf(y)) for x, y in row["control"]]
            wt = [mpf(x) for x in row["weights"]]
            env = mpf(row["envelope"])
            worst = mpf(0)
            for k in range(n):
                p, w = pts[2 * k : 2 * k + 3], wt[2 * k : 2 * k + 3]

                def miss(t, k=k, p=p, w=w, h=h, r=r):
                    s = standard_s(t - (k + mpf(1) / 2) * h, h)
                    b = rq(p, w, mobius(s, w[0], w[2]))
                    return sqrt((b[0] - r * cos(t)) ** 2 + (b[1] - r * sin(t)) ** 2)

                worst = max(worst, sup(miss, k * h, (k + 1) * h, n=120))
            rows += 1
            # An f64 row's column is its control points rounded: the
            # certificate meters that rounding through the band, so it is
            # allowed here, scaled by the map's Möbius skew.
            allow = mpf(0)
            if row["scalar"] == "f64":
                skew = max(
                    max(
                        sqrt(wt[2 * k] / wt[2 * k + 2]), sqrt(wt[2 * k + 2] / wt[2 * k])
                    )
                    for k in range(n)
                )
                allow = 16 * mpf(2) ** -52 * max(abs(x) for q in pts for x in q) * skew
            if worst > allow:
                ratio = env / worst
                lowest = ratio if lowest is None else min(lowest, ratio)
            if env < worst - allow:
                bad += 1
                print(
                    f"VIOLATION ({row['scalar']}): envelope {env} under the miss {mp.nstr(worst, 6)}: {line.strip()}"
                )
    shown = "n/a" if lowest is None else mp.nstr(lowest, 6)
    print(
        f"{rows} certified rows; lowest envelope/miss over a miss past rounding {shown}"
    )
    return bad


if __name__ == "__main__":
    violations = dumped(sys.argv[1]) if len(sys.argv) > 1 else derive()
    sys.exit(1 if violations else 0)
