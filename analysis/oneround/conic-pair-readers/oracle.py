#!/usr/bin/env python3
"""The arc-held ladder's oracle (`ladder.rs` beside it).

The truth is the curve the STORED numbers state,
`S + a*u*(cos t - kc) + b*(axis x u)*(sin t - ks)`, evaluated at 60 digits
(mpmath); every f64 the harness prints is its shortest round-trip form, so
`Fraction(float(x))` is the stored number exactly. Nothing here calls
kernel code.

Each reading is judged against the true gap of its pose, `g`:
  meets   g <= zero            the kernel must answer a meeting, or refuse
  apart   g >  esc             the kernel must answer none, or refuse
  between                     any answer, or a refusal
A meeting the kernel answers is also judged by its point: the true
closest point lies within `esc` of the kernel's along the ellipse, or the
meeting is wrong.

Usage: oracle.py ladder-eps<eps>.jsonl  -> prints the table, writes .judged.json
"""

import json
import re
import sys
import collections
from fractions import Fraction
from mpmath import mp, mpf, sqrt, cos, sin, findroot

mp.dps = 60


def F(x):
    f = Fraction(float(x))
    return mpf(f.numerator) / mpf(f.denominator)


def V(v):
    return [F(x) for x in v]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b, strict=False))


def cross(a, b):
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]


def sub(a, b):
    return [x - y for x, y in zip(a, b, strict=False)]


def add(a, b):
    return [x + y for x, y in zip(a, b, strict=False)]


def scale(a, s):
    return [x * s for x in a]


def norm(a):
    return sqrt(dot(a, a))


class Ellipse:
    def __init__(self, r):
        self.S, self.axis, self.u = V(r["S"]), V(r["axis"]), V(r["u"])
        self.a, self.b = F(r["a"]), F(r["b"])
        self.kc, self.ks = F(r["phase"][0]), F(r["phase"][1])
        self.v = cross(self.axis, self.u)

    def at(self, t):
        ku, kv = cos(t) - self.kc, sin(t) - self.ks
        return [
            s + self.a * u * ku + self.b * v * kv
            for s, u, v in zip(self.S, self.u, self.v, strict=False)
        ]

    def d(self, t):
        return [
            -self.a * u * sin(t) + self.b * v * cos(t)
            for u, v in zip(self.u, self.v, strict=False)
        ]


def closest(f, df, g, dg, t0, s0, scale_t):
    """The closest pair of two curves from a seed, `t` scaled to metres."""

    def eqs(x, s):
        t = t0 + x / scale_t
        w = sub(f(t), g(s))
        return [dot(w, df(t)) / scale_t, -dot(w, dg(s))]

    sol = findroot(eqs, (mpf(0), mpf(s0)), tol=mpf(10) ** -40, maxsteps=80)
    t = t0 + sol[0] / scale_t
    return t, sol[1], norm(sub(f(t), g(sol[1])))


def refusing_row(fault):
    """The row a refusal names: a meter's `row`, a gap decision's
    `predicate`, else the fault itself (`Unsettled(Tangency)`)."""
    if fault is None:
        return None
    m = re.search(r'row: "([^"]+)"', fault) or re.search(
        r'predicate: Some\("([^"]+)"\)', fault
    )
    return m.group(1) if m else fault


def judge(r):
    e = Ellipse(r)
    zero, esc = r["zero"], r["esc"]
    theta = F(r["theta"])
    sp = norm(e.d(theta))
    if r["door"] == "line":
        lo, ld = V(r["lo"]), V(r["ld"])

        def f(s):
            return add(lo, scale(ld, s))

        def df(s):
            return ld

        # seed: the line's foot of the target
        s0 = dot(sub(e.at(theta), lo), ld) / dot(ld, ld)
        t, _s, gap = closest(e.at, e.d, f, df, theta, s0, sp)
    else:
        c, m, cu = V(r["c"]), V(r["m"]), V(r["cu"])
        cv = cross(m, cu)

        def g(s):
            return [
                x + u * cos(s) + v * sin(s) for x, u, v in zip(c, cu, cv, strict=False)
            ]

        def dg(s):
            return [-u * sin(s) + v * cos(s) for u, v in zip(cu, cv, strict=False)]

        t, _s, gap = closest(e.at, e.d, g, dg, theta, 0, sp)
    gap = float(gap)
    truth = "meets" if gap <= zero else ("apart" if gap > esc else "between")
    pts = r["points"]
    if r["kind"] == "refused":
        said = "refused"
    elif pts:
        said = "meets"
    else:
        said = "apart"
    wrong = (said == "apart" and truth == "meets") or (
        said == "meets" and truth == "apart"
    )
    off = []
    for p in pts:
        tk = F(p["tb"] if r["door"] == "line" else p["ta"])
        off.append(float(norm(sub(e.at(tk), e.at(t)))))
    if said == "meets" and truth != "apart" and off and min(off) > esc:
        wrong = True
    return {
        **{
            k: r[k]
            for k in (
                "door",
                "lane",
                "fat",
                "target",
                "case",
                "place",
                "theta",
                "gamma",
                "shift",
                "kind",
            )
        },
        "fault": r.get("fault"),
        "row": refusing_row(r.get("fault")),
        "in_box": r.get("in_box"),
        "seam_at_anchor": r["phase"] == [1.0, 0.0],
        "gap": gap,
        "truth": truth,
        "said": said,
        "wrong": wrong,
        "off": off,
    }


def main(path):
    out = [judge(r) for r in map(json.loads, open(path)) if r["door"] != "mint"]
    json.dump(out, open(path.replace(".jsonl", ".judged.json"), "w"), indent=0)
    table = collections.Counter()
    for j in out:
        verdict = (
            ("WRONG" if j["wrong"] else "right")
            if j["said"] != "refused"
            else "refused:" + j["row"]
        )
        table[(j["door"], j["lane"], j["truth"], verdict)] += 1
    for k in sorted(table):
        print(*k, table[k], sep="\t")
    for j in out:
        if j["wrong"]:
            print("WRONG", json.dumps(j))
    return out


if __name__ == "__main__":
    main(sys.argv[1])
