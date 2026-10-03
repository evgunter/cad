#!/usr/bin/env python3
"""The high-precision oracle for the ellipse x torus root lane.

Reads the JSON lines `topo`'s `ellipse_roots::torus_rows::dump_for_the_mpmath_oracle`
writes (one pose per line: the stored ellipse and torus, the arc, the band and
the door's answer) and re-solves every pose at 40 significant digits from the
STORED values, never from the kernel's arithmetic:

- the carrier is `C(t) = C0 + major*u*cos t + minor*(n x u)*sin t`, evaluated
  exactly on the stored frame, as `geom_brep::Conic::point` writes it;
- the torus's true signed distance is `hypot(rho - R, h) - r`.

The turn about the arc's midpoint is sampled densely; every sign change, and
every local extremum of the distance that reaches across zero (a graze pair
closer than the sampling step), is refined by bisection. Then:

- a certified count must equal the true count, and each certified root must lie
  within the band's zero, as arc length, of a true root;
- a `miss` must have no true root, and a least distance past the band's zero.

It prints the worst root error in units of the zero band, per family and band,
and exits non-zero on any violation. Run: `python3 scripts/oracles/ellipse_torus_mpmath.py <dump>`.
"""

import json
import sys

from mpmath import mp, mpf

mp.dps = 40
SAMPLES = 3000


def vec(xs):
    return [mpf(x) for x in xs]


def cross(a, b):
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


class Pose:
    def __init__(self, d):
        self.c = vec(d["center"])
        self.n = vec(d["axis"])
        self.u = vec(d["u_ref"])
        self.v = cross(self.n, self.u)
        self.a = mpf(d["major"])
        self.b = mpf(d["minor"])
        self.hub = vec(d["hub"])
        self.ax = vec(d["t_axis"])
        self.big = mpf(d["R"])
        self.small = mpf(d["r"])

    def point(self, t):
        ct, st = mp.cos(t), mp.sin(t)
        return [self.c[i] + self.u[i] * (self.a * ct) + self.v[i] * (self.b * st) for i in range(3)]

    def distance(self, t):
        p = self.point(t)
        q = [p[i] - self.hub[i] for i in range(3)]
        h = dot(q, self.ax)
        w = [q[i] - self.ax[i] * h for i in range(3)]
        rho = mp.sqrt(dot(w, w))
        return mp.hypot(rho - self.big, h) - self.small

    def speed(self, t):
        return mp.hypot(self.a * mp.sin(t), self.b * mp.cos(t))


def bisect(f, lo, hi):
    flo = f(lo)
    for _ in range(140):
        mid = (lo + hi) / 2
        fm = f(mid)
        if (fm < 0) == (flo < 0):
            lo, flo = mid, fm
        else:
            hi = mid
    return (lo + hi) / 2


def extremum(f, lo, hi):
    """Golden-section search for the extremum of `f` on [lo, hi] whose sign is
    `f`'s at the bracket's middle: the least |f| a graze reaches."""
    sign = 1 if f((lo + hi) / 2) > 0 else -1
    g = (mp.sqrt(5) - 1) / 2
    a, b = lo, hi
    for _ in range(160):
        c, d = b - g * (b - a), a + g * (b - a)
        if sign * f(c) < sign * f(d):
            b = d
        else:
            a = c
    return (a + b) / 2


def true_roots(pose, mid):
    f = pose.distance
    ts = [mid - mp.pi + 2 * mp.pi * k / SAMPLES for k in range(SAMPLES + 1)]
    fs = [f(t) for t in ts]
    roots = []
    least = min(abs(x) for x in fs)
    for k in range(SAMPLES):
        if (fs[k] < 0) != (fs[k + 1] < 0):
            roots.append(bisect(f, ts[k], ts[k + 1]))
    # A graze pair inside one step: a sampled local extremum of |f| whose
    # refined extremum crosses zero.
    for k in range(1, SAMPLES):
        a, b, c = abs(fs[k - 1]), abs(fs[k]), abs(fs[k + 1])
        same = (fs[k - 1] < 0) == (fs[k] < 0) == (fs[k + 1] < 0)
        if not (same and b <= a and b <= c):
            continue
        t = extremum(f, ts[k - 1], ts[k + 1])
        least = min(least, abs(f(t)))
        if (f(t) < 0) != (fs[k] < 0):
            roots.append(bisect(f, ts[k - 1], t))
            roots.append(bisect(f, t, ts[k + 1]))
    return sorted(roots), least


def main(path):
    worst = {}
    failures = []
    counts = {}
    with open(path) as fh:
        lines = [json.loads(line) for line in fh if line.strip()]
    for i, d in enumerate(lines):
        pose = Pose(d)
        eps = mpf(d["eps"])
        mid = (mpf(d["t0"]) + mpf(d["t1"])) / 2
        key = (d["family"], d["eps"])
        counts.setdefault(key, {"certified": 0, "miss": 0, "uncertain": 0})
        counts[key][d["answer"]] += 1
        if d["answer"] == "uncertain":
            continue
        roots, least = true_roots(pose, mid)
        label = f"line {i + 1} ({d['family']}, eps {d['eps']})"
        if d["answer"] == "miss":
            if roots or least <= eps:
                failures.append(f"{label}: a miss with {len(roots)} true roots, least distance {mp.nstr(least, 5)}")
            continue
        got = [mpf(t) for t in d["roots"]]
        if len(got) != len(roots):
            failures.append(f"{label}: {len(got)} certified roots, {len(roots)} true")
            continue
        for t in got:
            # The nearest true root, compared on the circle.
            def gap(r, t=t):
                return abs((t - r + mp.pi) % (2 * mp.pi) - mp.pi)

            r = min(roots, key=gap)
            arc = gap(r) * pose.speed(r)
            ratio = arc / eps
            worst[key] = max(worst.get(key, mpf(0)), ratio)
            if arc > eps:
                failures.append(f"{label}: root {mp.nstr(t, 17)} is {mp.nstr(arc, 5)} m of arc from {mp.nstr(r, 17)}")
    for key in sorted(counts):
        w = worst.get(key)
        w = mp.nstr(w, 3) if w is not None else "-"
        print(f"{key[0]:>10} eps {key[1]:<6}: {counts[key]}, worst root error {w} zero bands")
    for f in failures:
        print("FAIL", f)
    print(f"{len(lines)} poses, {len(failures)} failures")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
