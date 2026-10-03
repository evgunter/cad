#!/usr/bin/env python3
"""Reviewer oracle (PR 3973 dual, lane reach-dual3973-r2), independent of
the PR's own scripts/oracles/ellipse_torus_mpmath.py.

Reads PROBE json lines (stdin or files). For each pose it builds the EXACT
carrier from the f64 inputs, C(t) = c + a*u*cos t + b*(n x u)*sin t (cross
product exact in mpmath), and the exact signed distance to the torus,
D(t) = hypot(rho - R, h) - r. It finds every sign change of D over the turn
(dense sampling + extremum refinement, so a graze pair is not missed) and
the least |D|, at 50 digits. Then it judges the door's answer:

  certified : count == true crossings, every root within eps of arc length
              of a distinct true root, no true crossing unmatched;
  miss      : no true crossing AND min|D| > eps;
  uncertain : always allowed (counted).
"""
import json
import sys

import mpmath as mp

mp.mp.dps = 34  # 34 digits: ~1e-34 relative, far below every band
N = 2500


def vec(a):
    return [mp.mpf(x) for x in a]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def judge(p):
    c, n, u = vec(p["center"]), vec(p["axis"]), vec(p["u_ref"])
    v = cross(n, u)
    a, b = mp.mpf(p["major"]), mp.mpf(p["minor"])
    hub, ax = vec(p["hub"]), vec(p["t_axis"])
    R, r = mp.mpf(p["R"]), mp.mpf(p["r"])
    eps = p["eps"]

    def point(t):
        ct, st = mp.cos(t), mp.sin(t)
        return [c[i] + a * u[i] * ct + b * v[i] * st for i in range(3)]

    def D(t):
        q = [x - y for x, y in zip(point(t), hub)]
        h = dot(q, ax)
        w = [q[i] - ax[i] * h for i in range(3)]
        rho = mp.sqrt(dot(w, w))
        return mp.hypot(rho - R, h) - r

    def speed(t):
        return mp.sqrt((a * mp.sin(t)) ** 2 + (b * mp.cos(t)) ** 2)

    mid = (mp.mpf(p["t0"]) + mp.mpf(p["t1"])) / 2
    lo = mid - mp.pi
    ts = [lo + 2 * mp.pi * k / N for k in range(N)]
    ds = [D(t) for t in ts]
    step = 2 * mp.pi / N
    # refine every extremum of D between samples, CYCLICALLY (a graze
    # straddling the grid's seam is refined too), so a graze pair is not
    # missed; each refined point is folded back into [lo, lo + 2pi)
    extra = []
    for k in range(N):
        d0, d1, d2 = ds[k - 1], ds[k], ds[(k + 1) % N]
        if (d1 <= d0 and d1 <= d2) or (d1 >= d0 and d1 >= d2):
            f = (lambda t: D(t)) if d1 <= d0 else (lambda t: -D(t))
            x0, x1 = ts[k] - step, ts[k] + step
            for _ in range(80):  # golden section
                m1 = x0 + (x1 - x0) * mp.mpf("0.381966011250105")
                m2 = x0 + (x1 - x0) * mp.mpf("0.618033988749895")
                if f(m1) < f(m2):
                    x1 = m2
                else:
                    x0 = m1
            xm = (x0 + x1) / 2
            xm = lo + ((xm - lo) % (2 * mp.pi))
            extra.append((xm, D(xm)))
    pts = sorted(list(zip(ts, ds)) + extra, key=lambda z: z[0])
    pts.append((lo + 2 * mp.pi, ds[0]))
    roots = []
    for (t0, d0), (t1, d1) in zip(pts, pts[1:]):
        if d0 == 0:
            roots.append(t0)
        elif (d0 < 0) != (d1 < 0) and d1 != 0:
            roots.append(mp.findroot(D, (t0, t1), solver="anderson"))
    least = min(abs(d) for _, d in pts)
    ans = p["answer"]
    got = [mp.mpf(t) for t in p["roots"]]
    bad = None
    worst = 0.0
    if ans == "certified":
        if len(got) != len(roots):
            bad = f"count {len(got)} vs true {len(roots)}"
        else:
            free = list(roots)
            for t in got:
                best = min(free, key=lambda z: abs(((t - z + mp.pi) % (2 * mp.pi)) - mp.pi))
                dt = abs(((t - best + mp.pi) % (2 * mp.pi)) - mp.pi)
                arc = float(dt * speed(best))
                worst = max(worst, arc / eps)
                free.remove(best)
                if arc > eps:
                    bad = f"root {float(t)} is {arc:.3e} m of arc ({arc/eps:.2f} bands) off {float(best)}"
    elif ans == "miss":
        if roots or least <= eps:
            bad = f"miss with {len(roots)} true crossings, least |D| {float(least):.3e} ({float(least)/eps:.2f} bands)"
    elif ans != "uncertain":
        bad = f"answer {ans}"
    return bad, len(roots), float(least) / eps, worst


def main():
    files = sys.argv[1:] or ["-"]
    tally = {}
    fails = 0
    worst_all = 0.0
    ps = []
    for fn in files:
        fh = sys.stdin if fn == "-" else open(fn)
        for line in fh:
            line = line.strip()
            if line.startswith("PROBE "):
                line = line[6:]
            if line.startswith("{"):
                ps.append(json.loads(line))
    import multiprocessing
    with multiprocessing.Pool() as pool:
        out = pool.map(judge, ps, chunksize=4)
    for p, (bad, ntrue, least, worst) in zip(ps, out):
        if True:
            worst_all = max(worst_all, worst)
            key = (p["family"], p["eps"], p["answer"].split()[0])
            tally[key] = tally.get(key, 0) + 1
            if bad:
                fails += 1
                print("FAIL", p["family"], "eps", p["eps"], "scale", p["scale"], "far", p["far"], "true", ntrue,
                      "least", f"{least:.2f}b", bad, "|", json.dumps(p))
    for k in sorted(tally):
        print(*k, tally[k])
    print("failures", fails, "worst certified root (bands)", worst_all)


if __name__ == "__main__":
    main()
