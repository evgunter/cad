#!/usr/bin/env python3
"""Reviewer r2's oracle for PR 7203 (C1). Reads the rows
`review_r2_common::dump_points` prints (f64 bit patterns) and checks, at 60
digits (mpmath):
  * each f64 point's distance to the plane and to the cone it was cut from;
  * (anchored trees) the running bound against the stored form's exact value,
    stored form read as: anchor = point at t=0, u_ref, axis, semi-axes;
  * the Interval enclosure of the f64 curve against that exact value;
  * the Interval-minted curve's enclosure against the two true surfaces;
  * the parameter read back.
usage: oracle.py rows.txt {anchored|centred}
"""
import sys, struct, math
from collections import defaultdict
from mpmath import mp, mpf, sin, cos, sqrt
mp.dps = 60
U = mpf(2) ** -53

def f(hx):
    return struct.unpack(">d", bytes.fromhex(hx))[0]
def m(hx):
    return mpf(f(hx))
def v3(xs):
    return [m(x) for x in xs]
def dot(a, b): return sum(x * y for x, y in zip(a, b))
def sub(a, b): return [x - y for x, y in zip(a, b)]
def add(a, b): return [x + y for x, y in zip(a, b)]
def scl(a, k): return [x * k for x in a]
def cross(a, b):
    return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def norm(a): return sqrt(dot(a, a))

form = sys.argv[2]
S = None
anchor = None
stats = defaultdict(lambda: defaultdict(lambda: mpf(0)))
counts = defaultdict(int)
viol = []
def zone(t, a, p=None):
    if p is not None and anchor is not None and form == "anchored":
        if norm(sub(p, anchor)) <= 500: return "near(<=500m)"
    elif abs(t) * b <= 500 and abs(t) < 1.3: return "near(<=500m)"
    q = abs(abs(t) - math.pi/2) * a
    if q <= 5: return "quarter"
    h = abs(abs(t) - math.pi) * a
    if h <= 5: return "half"
    return "general"

for line in open(sys.argv[1]):
    w = line.split()
    if not w: continue
    if w[0] == "S":
        q, n, A, d = v3(w[1:4]), v3(w[4:7]), v3(w[7:10]), v3(w[10:13])
        alpha = m(w[13]); C = v3(w[14:17]); ax = v3(w[17:20]); ur = v3(w[20:23])
        a, b = m(w[23]), m(w[24])
        vr = cross(ax, ur)
        dh = scl(d, 1 / norm(d)); nh = scl(n, 1 / norm(n))
        S = True; anchor = None
        dec = int(math.floor(math.log10(float(a))))
        def dists(p):
            dp = dot(sub(p, q), nh)
            wv = sub(p, A); h = dot(wv, dh)
            rho = norm(sub(wv, scl(dh, h)))
            dc = rho * cos(alpha) - abs(h) * sin(alpha)
            return dp, dc
    elif w[0] == "P":
        t = m(w[1]); p = v3(w[2:5]); err = v3(w[5:8])
        lo = v3(w[8:11]); hi = v3(w[11:14]); back = m(w[15])
        if float(t) == 0.0: anchor = p
        z = zone(float(t), float(a), p); key = (dec, z)
        dp, dc = dists(p)
        st = stats[key]; counts[key] += 1; lastz = z
        st["plane"] = max(st["plane"], abs(dp)); st["cone"] = max(st["cone"], abs(dc))
        pn = norm(p)
        st["miss/(u|p|)"] = max(st["miss/(u|p|)"], max(abs(dp), abs(dc)) / (U * (1 + pn)))
        st["miss/(u a)"] = max(st["miss/(u a)"], max(abs(dp), abs(dc)) / (U * a))
        if form == "anchored":
            ex = add(anchor, add(scl(ur, a * (cos(t) - 1)), scl(vr, b * sin(t))))
        else:
            ex = add(C, add(scl(ur, a * cos(t)), scl(vr, b * sin(t))))
        for i in range(3):
            e = abs(p[i] - ex[i])
            if err[i] > 0:
                st["err/bound"] = max(st["err/bound"], e / err[i])
            elif e > 0:
                viol.append(("zero bound, nonzero error", float(a), float(t), i, float(e)))
            if e > err[i]:
                viol.append(("running bound", float(a), float(t), i, float(e), float(err[i])))
            if not (lo[i] <= ex[i] <= hi[i]):
                viol.append(("interval(lifted f64 curve)", float(a), float(t), i,
                             float(lo[i] - ex[i]), float(ex[i] - hi[i])))
            st["bound[m]"] = max(st["bound[m]"], err[i])
            st["ivl width[m]"] = max(st["ivl width[m]"], hi[i] - lo[i])
        st["param back[m]"] = max(st["param back[m]"], abs(back - t) * a)
    elif w[0] == "I":
        t = m(w[1]); lo = v3(w[2:5]); hi = v3(w[5:8])
        z = lastz; key = (dec, z); st = stats[key]
        corners = [[(lo, hi)[(k >> i) & 1][i] for i in range(3)] for k in range(8)]
        ds = [dists(c) for c in corners]
        for j, name in ((0, "plane"), (1, "cone")):
            vals = [x[j] for x in ds]
            if not (min(vals) <= 0 <= max(vals)):
                viol.append(("interval mint misses " + name, float(a), float(t),
                             float(min(vals)), float(max(vals))))
        st["mint ivl width[m]"] = max(st["mint ivl width[m]"], max(h - l for l, h in zip(lo, hi)))
        counts[(dec, z, "I")] += 1

cols = ["plane", "cone", "miss/(u|p|)", "miss/(u a)", "err/bound", "bound[m]",
        "ivl width[m]", "mint ivl width[m]", "param back[m]"]
print("decade zone n nI | " + " | ".join(cols))
for key in sorted(stats):
    st = stats[key]
    print(f"1e{key[0]} {key[1]} {counts[key]} {counts[(key[0], key[1], 'I')]} | " +
          " | ".join(f"{float(st[c]):.3g}" for c in cols))
print("violations:", len(viol))
for v in viol[:40]: print("  ", v)
