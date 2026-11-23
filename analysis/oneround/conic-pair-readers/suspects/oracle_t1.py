#!/usr/bin/env python3
"""Task 1 oracle: the truth is the curves the STORED numbers state, in 60-digit mpmath; no kernel code.

  line   o + d t
  conic  S + a u (cos t - kc) + b (axis x u) (sin t - ks)      (centre-held: kc = ks = 0)

For a `meet` case the truth is the least distance between the two SPANS: seeds from a float grid over both
spans (the grid only seeds), each refined by Newton in mpmath on the stationarity of the squared distance,
plus every end against the other span. Verdicts: g <= eps MEETS, g > K eps APART, else GAP.
The kernel is WRONG where it answers Ok with no meeting on a truth of MEETS, or a meeting on a truth of
APART; a definite answer on a truth in the GAP is listed apart (GAPDEF). A refusal is never wrong.
"""

import json
import sys
import math
from fractions import Fraction
from mpmath import mp, mpf, sqrt, cos, sin

mp.dps = 60
K = 10


def F(x):
    f = Fraction(float(x))
    return mpf(f.numerator) / mpf(f.denominator)


def V(v):
    return [F(x) for x in v]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b, strict=False))


def sub(a, b):
    return [x - y for x, y in zip(a, b, strict=False)]


def cross(a, b):
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]


def norm(a):
    return sqrt(dot(a, a))


class Curve:
    def __init__(self, r):
        self.k = r["k"]
        if self.k == "line":
            self.o, self.d = V(r["o"]), V(r["d"])
            self.fo, self.fd = [float(x) for x in r["o"]], [float(x) for x in r["d"]]
        else:
            self.S, self.ax, self.u = V(r["S"]), V(r["axis"]), V(r["u"])
            self.v = cross(self.ax, self.u)
            self.a, self.b, self.kc, self.ks = (
                F(r["a"]),
                F(r["b"]),
                F(r["phase"][0]),
                F(r["phase"][1]),
            )
            self.f = [[float(x) for x in w] for w in (self.S, self.u, self.v)] + [
                float(self.a),
                float(self.b),
                float(self.kc),
                float(self.ks),
            ]

    def at(self, t):
        if self.k == "line":
            return [o + d * t for o, d in zip(self.o, self.d, strict=False)]
        ku, kv = cos(t) - self.kc, sin(t) - self.ks
        return [
            s + self.a * u * ku + self.b * v * kv
            for s, u, v in zip(self.S, self.u, self.v, strict=False)
        ]

    def d1(self, t):
        if self.k == "line":
            return self.d
        return [
            -self.a * u * sin(t) + self.b * v * cos(t)
            for u, v in zip(self.u, self.v, strict=False)
        ]

    def d2(self, t):
        if self.k == "line":
            return [mpf(0)] * 3
        return [
            -self.a * u * cos(t) - self.b * v * sin(t)
            for u, v in zip(self.u, self.v, strict=False)
        ]

    def fat(self, t):  # float, for seeds only; stable about an anchor at phase (1, 0)
        if self.k == "line":
            return [o + d * t for o, d in zip(self.fo, self.fd, strict=False)]
        S, u, v, a, b, kc, ks = self.f
        if kc == 1.0 and ks == 0.0:
            ku, kv = -2 * math.sin(t / 2) ** 2, math.sin(t)
        else:
            ku, kv = math.cos(t) - kc, math.sin(t) - ks
        return [
            s + a * uu * ku + b * vv * kv for s, uu, vv in zip(S, u, v, strict=False)
        ]


def foot(c, q, t0, span):
    """The least distance from q to c over span: Newton from t0 and from the ends' side, clamped."""
    lo, hi = span
    best = None
    for start in (t0, lo, hi, (lo + hi) / 2):
        t = min(max(start, lo), hi)
        for _ in range(60):
            w = sub(c.at(t), q)
            g = dot(w, c.d1(t))
            h = dot(c.d1(t), c.d1(t)) + dot(w, c.d2(t))
            if h <= 0:
                break
            tn = min(max(t - g / h, lo), hi)
            if abs(tn - t) < mpf(10) ** -50:
                t = tn
                break
            t = tn
        d = norm(sub(c.at(t), q))
        if best is None or d < best[0]:
            best = (d, t)
    for t in (lo, hi):
        d = norm(sub(c.at(t), q))
        if d < best[0]:
            best = (d, t)
    return best


def span_gap(A, sa, B, sb, hint):
    sa = (F(sa[0]), F(sa[1]))
    sb = (F(sb[0]), F(sb[1]))
    N = 160
    fa = [float(sa[0]) + (float(sa[1]) - float(sa[0])) * i / N for i in range(N + 1)]
    fb = [float(sb[0]) + (float(sb[1]) - float(sb[0])) * i / N for i in range(N + 1)]
    pa = [A.fat(t) for t in fa]
    pb = [B.fat(t) for t in fb]
    cells = []
    for i, x in enumerate(pa):
        for j, y in enumerate(pb):
            cells.append(
                ((x[0] - y[0]) ** 2 + (x[1] - y[1]) ** 2 + (x[2] - y[2]) ** 2, i, j)
            )
    cells.sort()
    seeds = [(F(hint[0]), F(hint[1]))]
    taken = []
    for _d2, i, j in cells:
        if all(abs(i - p) > 6 or abs(j - q) > 6 for p, q in taken):
            taken.append((i, j))
            seeds.append((F(fa[i]), F(fb[j])))
        if len(taken) == 4:
            break
    best = None

    def consider(d, ta, tb):
        nonlocal best
        if best is None or d < best[0]:
            best = (d, ta, tb)

    for ta, tb in seeds:
        ta = min(max(ta, sa[0]), sa[1])
        tb = min(max(tb, sb[0]), sb[1])
        ok = True
        for _ in range(80):
            w = sub(A.at(ta), B.at(tb))
            da, db = A.d1(ta), B.d1(tb)
            g1, g2 = dot(w, da), -dot(w, db)
            h11 = dot(da, da) + dot(w, A.d2(ta))
            h22 = dot(db, db) - dot(w, B.d2(tb))
            h12 = -dot(da, db)
            det = h11 * h22 - h12 * h12
            if det == 0:
                ok = False
                break
            sa_ = (g1 * h22 - g2 * h12) / det
            sb_ = (g2 * h11 - g1 * h12) / det
            ta, tb = ta - sa_, tb - sb_
            if abs(sa_) * norm(da) + abs(sb_) * norm(db) < mpf(10) ** -48:
                break
        if ok and sa[0] <= ta <= sa[1] and sb[0] <= tb <= sb[1]:
            consider(norm(sub(A.at(ta), B.at(tb))), ta, tb)
    for t in sa:
        d, tb = foot(B, A.at(t), F(hint[1]), sb)
        consider(d, t, tb)
    for t in sb:
        d, ta = foot(A, B.at(t), F(hint[0]), sa)
        consider(d, ta, t)
    return best


def truth_class(g, eps):
    return "MEETS" if g <= eps else ("APART" if g > K * eps else "GAP")


def main(path):
    out = []
    for line in open(path):
        r = json.loads(line)
        eps = float(r["eps"])
        rec = {k: r[k] for k in ("eps", "a", "shape", "s", "fam", "var", "door")}
        if r["door"] == "meet":
            A, B = Curve(r["A"]), Curve(r["B"])
            g, _ta, _tb = span_gap(A, r["sa"], B, r["sb"], r["hint"])
            t = truth_class(g, eps)
            k = (
                "REFUSED"
                if r["kind"] == "refused"
                else ("K_MEET" if (r["points"] or r["shared"]) else "K_APART")
            )
            off = mpf(0)
            for p in r["points"]:
                q = V(p["p"])
                off = max(
                    off,
                    foot(A, q, F(p["ta"]), (F(r["sa"][0]), F(r["sa"][1])))[0],
                    foot(B, q, F(p["tb"]), (F(r["sb"][0]), F(r["sb"][1])))[0],
                )
            rec.update(
                gap=float(g),
                truth=t,
                kernel=k,
                shared=r["shared"],
                npts=len(r["points"]),
                pt_off=float(off),
                why=r.get("why", ""),
            )
            if k == "REFUSED":
                v = "refused"
            elif t == "GAP":
                v = "GAPDEF"
            elif (t == "MEETS") == (k == "K_MEET"):
                v = "right"
            else:
                v = "WRONG"
            if v == "right" and k == "K_MEET" and off > K * eps:
                v = "WRONG-POINT"
            rec["verdict"] = v
        elif r["door"] == "on_carrier":
            E = Curve(r["E"])
            near = F(r["near"])
            g, _ = foot(E, V(r["q"]), near, (near - 1, near + 1))
            t = truth_class(g, eps).replace("MEETS", "ON").replace("APART", "OFF")
            k = r["kind"].split(":")[0]
            rec.update(gap=float(g), truth=t, kernel=k, why=r["kind"])
            rec["verdict"] = (
                "refused"
                if k == "refused"
                else (
                    "GAPDEF" if t == "GAP" else ("right" if t.lower() == k else "WRONG")
                )
            )
        elif r["door"] == "under":
            # the edge's plane as its stored numbers state it: through the stored point, normal the stored axis
            E, L = Curve(r["E"]), Curve(r["L"])

            def h(q, E=E):
                return dot(sub(q, E.S), E.ax) / norm(E.ax)

            h0, h1 = h(L.at(F(r["span"][0]))), h(L.at(F(r["span"][1])))
            lo, hi = min(abs(h0), abs(h1)), max(abs(h0), abs(h1))
            if hi <= eps:
                t = "INPLANE"
            elif h0 * h1 < 0 and lo > K * eps:
                t = "CROSS"
            elif h0 * h1 > 0 and lo > K * eps:
                t = "APART"
            else:
                t = "GAP"
            k = r["kind"]
            off = mpf(0)
            if t == "CROSS":
                tc = F(r["span"][0]) + (F(r["span"][1]) - F(r["span"][0])) * h0 / (
                    h0 - h1
                )
                x = L.at(tc)
                off = max([norm(sub(V(q), x)) for q in r["pts"]] or [mpf(0)])
            if k == "refused":
                v = "refused"
            elif t == "GAP":
                v = "GAPDEF"
            elif t == "INPLANE":
                v = "right" if k == "inplane" else "WRONG"
            elif t == "APART":
                v = "right" if (k == "roots" and not r["pts"]) else "WRONG"
            else:
                v = (
                    "WRONG"
                    if not (k == "roots" and len(r["pts"]) == 1)
                    else ("WRONG-POINT" if off > K * eps else "right")
                )
            rec.update(
                gap=float(hi),
                truth=t,
                kernel=k + str(len(r["pts"])),
                why=r.get("why", ""),
                pt_off=float(off),
                verdict=v,
            )
        else:
            E = Curve(r["E"])
            o, n = V(r["o"]), V(r["n"])
            hs = [
                abs(dot(sub(E.at(F(t)), o), n)) / norm(n)
                for t in (r["span"][0], r["span"][1], (r["span"][0] + r["span"][1]) / 2)
            ]
            lo, hi = min(hs), max(hs)
            t = "INPLANE" if hi <= eps else ("MISS" if lo > K * eps else "GAP")
            kind = r["kind"]
            k = (
                "inplane"
                if "InPlane" in kind
                else (
                    "miss"
                    if "Miss" in kind
                    else ("refused" if "Err" in kind else "roots")
                )
            )
            rec.update(gap=float(hi), truth=t, kernel=k, why=kind)
            rec["verdict"] = (
                "refused"
                if k == "refused"
                else (
                    "GAPDEF" if t == "GAP" else ("right" if t.lower() == k else "WRONG")
                )
            )
        out.append(rec)
    json.dump(out, open(path.replace(".jsonl", ".judged.json"), "w"))


if __name__ == "__main__":
    main(sys.argv[1])
