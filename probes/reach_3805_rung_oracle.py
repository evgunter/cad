"""60-digit oracle for probes/reach_3805_rung_fuzz.rs (PR #3805, third delta).

Reads the probe's TSV; for every row whose rung answer is in ANSWERS
(default: `pos`, a certified clearance), computes the least TRUE signed
distance of the stored arc [t0, t1] from the stored surface (and the greatest), the f64
inputs read exactly as decimals (the frame's v = axis x u_ref exact,
sin/cos by series). A coarse f64 scan finds the basins; each candidate,
and both ends, are refined by golden section in 60 digits. Never reads
the kernel. Prints counts per (eps, kind): certified-clear rows whose
arc crosses (least < -eps < most) or touches in band, i.e. its true
margin max(least, -most) is not beyond eps. An arc wholly inside
(most < -eps) is a true clearance.

usage: python3 reach_3805_rung_oracle.py rows.tsv [answers=pos]
"""
import math
import sys
from decimal import Decimal as D, getcontext

getcontext().prec = 60
PI = D("3.14159265358979323846264338327950288419716939937510582097494459")
TAU = 2 * PI


def sin_cos(x):
    x = x - TAU * int(x / TAU)
    s, c, t, k = D(0), D(0), D(1), 0
    while True:
        m = k % 4
        if m == 0:
            c += t
        elif m == 1:
            s += t
        elif m == 2:
            c -= t
        else:
            s -= t
        k += 1
        t = t * x / k
        if abs(t) < D("1e-75") and k > 10:
            return s, c


def cross(a, b):
    return (a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0])


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def vec(s, conv):
    return tuple(conv(x) for x in s.split(","))


def parse(row, conv):
    f = row.rstrip("\n").split("\t")
    r = dict(eps=float(f[0]), i=int(f[1]), curve=f[2], combo=int(f[3]),
             center=vec(f[4], conv), axis=vec(f[5], conv), major=conv(f[6]), minor=conv(f[7]),
             u=vec(f[8], conv), t0=float(f[9]), t1=float(f[10]), gap=float(f[12]),
             size=float(f[13]), far=float(f[14]), psi=float(f[15]), rung=f[16], door=f[17], kind=f[18])
    g = f[19:]
    if r["kind"] == "S":
        r["s"] = (vec(g[0], conv), conv(g[1]))
    elif r["kind"] == "C":
        r["s"] = (vec(g[0], conv), vec(g[1], conv), conv(g[2]))
    elif r["kind"] == "T":
        r["s"] = (vec(g[0], conv), vec(g[1], conv), conv(g[2]), conv(g[3]))
    else:
        r["s"] = None
    return r


def make_dist(r, sqrt, sc):
    c, ax, u = r["center"], r["axis"], r["u"]
    v = cross(ax, u)
    a, b = r["major"], r["minor"]
    kind, s = r["kind"], r["s"]

    def point(th):
        sn, cs = sc(th)
        return tuple(c[k] + u[k] * a * cs + v[k] * b * sn for k in range(3))

    def dist(th):
        p = point(th)
        if kind == "S":
            o, rad = s
            w = tuple(p[k] - o[k] for k in range(3))
            return sqrt(dot(w, w)) - rad
        if kind == "C":
            o, w_ax, rad = s
            w = tuple(p[k] - o[k] for k in range(3))
            h = dot(w, w_ax)
            q = tuple(w[k] - w_ax[k] * h for k in range(3))
            return sqrt(dot(q, q)) - rad
        o, w_ax, big, rad = s
        w = tuple(p[k] - o[k] for k in range(3))
        h = dot(w, w_ax)
        q = tuple(w[k] - w_ax[k] * h for k in range(3))
        rho = sqrt(dot(q, q))
        return sqrt((rho - big) ** 2 + h * h) - rad

    return dist


def refine(dd, ts, k, n, sign):
    g = (D(5).sqrt() - 1) / 2
    lo = D(ts[max(k - 2, 0)])
    hi = D(ts[min(k + 2, n)])
    f = lambda t: sign * dd(t)
    for _ in range(90):
        x1 = hi - g * (hi - lo)
        x2 = lo + g * (hi - lo)
        if f(x1) < f(x2):
            hi = x2
        else:
            lo = x1
    return dd((lo + hi) / 2)


def extremes(row):
    """(least, most) of the true signed distance over the stored arc."""
    rf = parse(row, float)
    # D(float): the BINARY value exactly. D(repr) is the shortest
    # round-trip decimal, up to half an ulp away (6e-11 m at 1000 km).
    rd = parse(row, lambda x: D(float(x)))
    fd = make_dist(rf, math.sqrt, lambda t: (math.sin(t), math.cos(t)))
    dd = make_dist(rd, lambda x: x.sqrt(), lambda t: sin_cos(t))
    t0, t1 = rf["t0"], rf["t1"]
    n = int(__import__("os").environ.get("ORACLE_N", "4000"))
    ts = [t0 + (t1 - t0) * k / n for k in range(n + 1)]
    vals = [fd(t) for t in ts]
    ends = (dd(D(t0)), dd(D(t1)))
    out = []
    for sign in (1, -1):
        cands = sorted(range(n + 1), key=lambda k: sign * vals[k])[:1]
        cands += [k for k in range(1, n)
                  if sign * vals[k] <= sign * vals[k - 1] and sign * vals[k] <= sign * vals[k + 1]][:6]
        best = min(ends) if sign == 1 else max(ends)
        for k in set(cands):
            v = refine(dd, ts, k, n, sign)
            best = min(best, v) if sign == 1 else max(best, v)
        out.append(best)
    return rf, out[0], out[1]


def main():
    path = sys.argv[1]
    answers = set((sys.argv[2] if len(sys.argv) > 2 else "pos").split(","))
    tally = {}
    shown = 0
    for row in open(path):
        f = row.split("\t")
        if f[16] not in answers or f[18] == "K":
            continue
        rf, least, most = extremes(row)
        eps = rf["eps"]
        margin = max(least, -most)
        key = (eps, rf["kind"], rf["curve"])
        t = tally.setdefault(key, [0, 0, 0])
        t[0] += 1
        if margin > eps:
            continue
        t[1 if (least < -eps and most > eps) else 2] += 1
        if shown < 8:
            shown += 1
            print("DEFECT", f"eps {eps} case {rf['i']} {rf['kind']} {rf['curve']} combo {rf['combo']} "
                  f"size {rf['size']} far {rf['far']} psi {rf['psi']:.3f} gap {rf['gap']:.3e} "
                  f"least {float(least):.4e} most {float(most):.4e}")
    for key in sorted(tally):
        n, cross_, band = tally[key]
        print(f"eps {key[0]:g} {key[1]} {key[2]}: {n} answered {sorted(answers)}, {cross_} cross, {band} in band")


main()
