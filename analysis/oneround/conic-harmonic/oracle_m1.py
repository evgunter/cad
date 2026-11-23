#!/usr/bin/env python3
"""M1 oracle: the truth is the curve the STORED numbers state.

Reads the harness's JSON lines (every f64 printed in its shortest
round-trip form, so `Fraction(float(x))` is the stored number exactly)
and evaluates the conic `S + a*u*(cos t - kc) + b*(axis x u)*(sin t - ks)`
in 60-digit arithmetic (mpmath). Nothing here calls kernel code: the
roots are solved in mpmath and checked by their own residual.
"""
import json, sys, collections
from fractions import Fraction
from mpmath import mp, mpf, sqrt, atan2, cos, sin, pi, findroot, matrix

mp.dps = 60
K = float(sys.argv[2]) if len(sys.argv) > 2 else 10.0


def F(x):
    f = Fraction(float(x))
    return mpf(f.numerator) / mpf(f.denominator)


def V(v):
    return [F(x) for x in v]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def sub(a, b):
    return [x - y for x, y in zip(a, b)]


def norm(a):
    return sqrt(dot(a, a))


class Conic:
    def __init__(self, r):
        self.S, self.axis, self.u = V(r["S"]), V(r["axis"]), V(r["u"])
        self.a, self.b = F(r["a"]), F(r["b"])
        self.kc, self.ks = F(r["phase"][0]), F(r["phase"][1])
        self.v = cross(self.axis, self.u)

    def at(self, t):
        ku, kv = cos(t) - self.kc, sin(t) - self.ks
        return [s + self.a * u * ku + self.b * v * kv for s, u, v in zip(self.S, self.u, self.v)]

    def d(self, t):
        return [-self.a * u * sin(t) + self.b * v * cos(t) for u, v in zip(self.u, self.v)]

    def dd(self, t):
        return [-self.a * u * cos(t) - self.b * v * sin(t) for u, v in zip(self.u, self.v)]

    def plane_roots(self, o, n):
        """Both roots of (P(t) - o).n round the turn."""
        h = dot(sub(self.S, o), n)
        p, q = self.a * dot(self.u, n), self.b * dot(self.v, n)
        P, Q = p * self.kc + q * self.ks, q * self.kc - p * self.ks
        a2, c = h - 2 * P, h
        disc = Q * Q - a2 * c
        if disc < 0:
            return []
        w = -(Q + (sqrt(disc) if Q >= 0 else -sqrt(disc)))
        t0 = atan2(self.ks, self.kc)
        out = []
        for num, den in ((w, a2), (c, w)):
            phi = atan2(2 * num * den, den * den - num * num)
            out.append(t0 + phi)
        for t in out:
            res = dot(sub(self.at(t), o), n)
            assert abs(res) < mpf(10) ** -40 * (1 + abs(self.a)), res
        return out


def wrap(x):
    return x - 2 * pi * mp.nint(x / (2 * pi))


def judge_root(k, tk, truths, span):
    """A kernel root against the nearest true root."""
    tk = F(tk)
    best = min(truths, key=lambda t: abs(wrap(tk - t)))
    return {
        "dtheta": float(wrap(tk - best)),
        "arc": float(norm(sub(k.at(tk), k.at(best)))),
        "ulp_floor": float(abs(F(tk)) * mpf(2) ** -53 * norm(k.d(best))),
    }


def in_span(t, span, k, margin):
    """Whether a true root is inside the span by more than `margin` metres of arc."""
    t0, t1 = F(span[0]), F(span[1])
    mid = (t0 + t1) / 2
    tt = mid + wrap(t - mid)
    if not (t0 < tt < t1):
        return False
    return min(norm(sub(k.at(tt), k.at(t0))), norm(sub(k.at(tt), k.at(t1)))) > margin


def main(path, eps):
    out = []
    for line in open(path):
        r = json.loads(line)
        if r["door"] == "mint":
            continue
        k = Conic(r)
        rec = {key: r[key] for key in ("door", "variant", "fat", "target", "case", "place", "theta", "gamma", "kind")}
        rec["kind"] = r["kind"].split("{")[0].split("(")[0] if r["kind"].startswith("refused") and r["door"] != "A" else r["kind"]
        rec["kind_full"] = r["kind"]
        if r["door"] == "A":
            truths = k.plane_roots(V(r["o"]), V(r["n"]))
            n_in = sum(in_span(t, r["span"], k, K * eps) for t in truths)
            n_maybe = sum(in_span(t, r["span"], k, 0) for t in truths)
            rec["true_in_span"] = [n_in, n_maybe]
            rec["roots"] = [judge_root(k, t, truths, r["span"]) for t in r["roots"]] if truths else []
            # the plane's height at the kernel's root
            for t, j in zip(r["roots"], rec["roots"]):
                j["height"] = float(abs(dot(sub(k.at(F(t)), V(r["o"])), V(r["n"]))))
            # the crossing's own slope (sine of the crossing angle), at the true root nearest the case
            near = min(truths, key=lambda t: abs(wrap(t - F(r["theta"])))) if truths else None
            if near is not None:
                rec["sin_cross"] = float(abs(dot(k.d(near), V(r["n"]))) / norm(k.d(near)) / norm(V(r["n"])))
        elif r["door"] == "B":
            c, m, cu = V(r["c"]), V(r["m"]), V(r["cu"])
            cv = cross(m, cu)

            def circ(s):
                return [x + u * cos(s) + v * sin(s) for x, u, v in zip(c, cu, cv)]

            def dcirc(s):
                return [-u * sin(s) + v * cos(s) for u, v in zip(cu, cv)]

            def g(t, s):
                w = sub(k.at(t), circ(s))
                return [dot(w, k.d(t)), -dot(w, dcirc(s))]

            # scaled unknown so both are O(1): t = theta + x / |E'|
            sp = norm(k.d(F(r["theta"])))
            sol = findroot(lambda x, s: g(F(r["theta"]) + x / sp, s), (mpf(0), mpf(0)), tol=mpf(10) ** -45, maxsteps=60)
            tstar, sstar = F(r["theta"]) + sol[0] / sp, sol[1]
            rec["true_gap"] = float(norm(sub(k.at(tstar), circ(sstar))))
            rec["points"] = []
            for p in r["points"]:
                ta, tb = F(p["ta"]), F(p["tb"])
                rec["points"].append({
                    "arc": float(norm(sub(k.at(ta), k.at(tstar)))),
                    "gap_at_pair": float(norm(sub(k.at(ta), circ(tb)))),
                    "pt_off": float(norm(sub(V(p["p"]), k.at(tstar)))),
                    "ulp_floor": float(abs(ta) * mpf(2) ** -53 * sp),
                })
            rec["log"] = r["log"]
        elif r["door"] == "C":
            lo, ld = V(r["lo"]), V(r["ld"])
            n = cross(k.axis, ld)
            truths = k.plane_roots(lo, n)
            near = min(truths, key=lambda t: abs(wrap(t - F(r["theta"]))))
            pt = k.at(near)
            # the line's own distance from that point (out of plane)
            w = sub(pt, lo)
            along = dot(w, ld) / dot(ld, ld)
            rec["true_gap"] = float(norm(sub(w, [along * x for x in ld])))
            rec["sin_cross"] = float(abs(dot(k.d(near), n)) / norm(k.d(near)) / norm(n))
            rec["points"] = []
            for p in r["points"]:
                tb = F(p["tb"])
                rec["points"].append({
                    "arc": float(norm(sub(k.at(tb), pt))),
                    "pt_off": float(norm(sub(V(p["p"]), pt))),
                    "ulp_floor": float(abs(tb) * mpf(2) ** -53 * norm(k.d(near))),
                })
            rec["log"] = r["log"]
        out.append(rec)
    json.dump(out, open(path.replace(".jsonl", ".judged.json"), "w"))
    return out


if __name__ == "__main__":
    main(sys.argv[1], float(sys.argv[1].split("eps")[1].split(".jsonl")[0]))
