#!/usr/bin/env python3
"""The high-precision oracle for the section certificate's general-pose cone arms.

Reads the JSON lines `topo`'s `section_cert::cone_pair_search::dump_for_the_mpmath_oracle`
writes (one pose per line: the cone, its partner — an oblique cylinder or a tilted
or parallel-axis cone — the band's zero, and `classify`'s answer) and re-reads the
section's topology at 40 significant digits from the STORED values, never from the
kernel's arithmetic.

On each carrier's ruling chart (a cone's generator lines through its apex, a
cylinder's rulings round its wall, each built here from its own orthonormal frame)
the partner's implicit form `q` is quadratic along every line, so evaluating it at
three points of the line gives the line's quadratic exactly. The turn is sampled
densely; every sign change of the discriminant and of the leading coefficient, and
every local extremum of either that reaches across zero between two samples (a
graze pair), is refined by bisection. Then on each chart:

- each maximal arc where the discriminant is positive is one component, unbounded
  if the leading coefficient vanishes on it (once per vanishing), else bounded and
  null on that carrier;
- a positive discriminant round the whole turn is two branches, each bounded and
  essential on that carrier unless it runs to infinity where the leading
  coefficient vanishes (the branch `-sign(p1)` there).

An answered pose must match both charts exactly: the unbounded count, and the
bounded parts' classes (`essential_f` on the cone's chart, `essential_g` on the
partner's). Every witness must lie within `1e-9` of the scale of both carriers, on
a bounded component of the cone's chart, distinct parts on distinct components.
A pose the oracle reads within `1e-20` of the scale of a degeneracy is reported
and not judged. It prints the tally and exits non-zero on any mismatch. Run:
`python3 scripts/oracles/cone_pair_sections_mpmath.py <dump>`.
"""

import json
import sys

from mpmath import mp, mpf

mp.dps = 40
SAMPLES = 1500


def vec(xs):
    return [mpf(x) for x in xs]


def add(a, b):
    return [a[i] + b[i] for i in range(3)]


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def scale(a, k):
    return [a[i] * k for i in range(3)]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def cross(a, b):
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]


def unit(a):
    return scale(a, 1 / mp.sqrt(dot(a, a)))


def frame(axis):
    a = unit(axis)
    pick = [1, 0, 0] if abs(a[0]) < mpf("0.6") else [0, 1, 0]
    e1 = unit(cross(a, vec(pick)))
    return a, e1, cross(a, e1)


class Carrier:
    """A cone or a cylinder: its implicit form and its ruling chart."""

    def __init__(self, d):
        self.kind = d["kind"]
        if self.kind == "cone":
            self.apex = vec(d["apex"])
            self.a, self.e1, self.e2 = frame(vec(d["axis"]))
            self.s, self.c = mp.sin(mpf(d["half_angle"])), mp.cos(mpf(d["half_angle"]))
        else:
            self.o = vec(d["origin"])
            self.a, self.e1, self.e2 = frame(vec(d["axis"]))
            self.r = mpf(d["radius"])

    def q(self, x):
        if self.kind == "cone":
            u = sub(x, self.apex)
            return dot(self.a, u) ** 2 - self.c**2 * dot(u, u)
        w = cross(sub(x, self.o), self.a)
        return dot(w, w) - self.r**2

    def distance(self, x):
        """A metric residual: the cone's elevation, the wall's radial offset."""
        if self.kind == "cone":
            u = sub(x, self.apex)
            h = dot(self.a, u)
            rho = mp.sqrt(max(dot(u, u) - h * h, 0))
            return rho * self.c - abs(h) * self.s
        w = cross(sub(x, self.o), self.a)
        return mp.sqrt(dot(w, w)) - self.r

    def line(self, th):
        radial = add(scale(self.e1, mp.cos(th)), scale(self.e2, mp.sin(th)))
        if self.kind == "cone":
            return self.apex, add(scale(self.a, self.c), scale(radial, self.s))
        return add(self.o, scale(radial, self.r)), self.a

    def chart(self, x):
        """`(θ, t)` of a point on this carrier."""
        if self.kind == "cone":
            u = sub(x, self.apex)
            t = dot(self.a, u) / self.c
            radial = scale(sub(u, scale(self.a, t * self.c)), 1 / (t * self.s))
        else:
            u = sub(x, self.o)
            t = dot(u, self.a)
            radial = sub(u, scale(self.a, t))
        return mp.atan2(dot(radial, self.e2), dot(radial, self.e1)), t


def quadratic(chart, partner, th):
    b, v = chart.line(th)
    q0, qp, qm = partner.q(b), partner.q(add(b, v)), partner.q(sub(b, v))
    return (qp + qm) / 2 - q0, (qp - qm) / 2, q0


def disc(chart, partner, th):
    p2, p1, p0 = quadratic(chart, partner, th)
    return p1 * p1 - 4 * p2 * p0


def lead(chart, partner, th):
    return quadratic(chart, partner, th)[0]


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


def extremum(f, lo, hi, sign):
    g = (mp.sqrt(5) - 1) / 2
    a, b = lo, hi
    for _ in range(130):
        c, d = b - g * (b - a), a + g * (b - a)
        if sign * f(c) < sign * f(d):
            b = d
        else:
            a = c
    return (a + b) / 2


def roots(f, norm, hot):
    """Every root of `f` round the turn, and the least |f|/norm at a graze or
    root pair the samples straddle (the pose's distance from a degeneracy).
    The samples close in geometrically on every `hot` angle, down to
    `1e-14` rad: a loop about the partner's apex can be that narrow."""
    tau = 2 * mp.pi
    ths = {tau * k / SAMPLES for k in range(SAMPLES)}
    for h in hot:
        for k in range(57):
            step = mpf(10) ** (-mpf(k) / 4)
            ths.add((h + step) % tau)
            ths.add((h - step) % tau)
        ths.add(h % tau)
    ths = sorted(ths)
    ths.append(ths[0] + tau)
    vals = [f(t) for t in ths]
    out, near = [], mpf("inf")
    for k in range(len(ths) - 1):
        if (vals[k] < 0) != (vals[k + 1] < 0):
            out.append(bisect(f, ths[k], ths[k + 1]))
    for k in range(1, len(ths) - 1):
        lo, mid, hi = vals[k - 1], vals[k], vals[k + 1]
        if (lo < 0) == (mid < 0) == (hi < 0) and abs(mid) <= abs(lo) and abs(mid) <= abs(hi):
            sign = 1 if mid > 0 else -1
            x = extremum(f, ths[k - 1], ths[k + 1], sign)
            fx = f(x)
            near = min(near, abs(fx) / norm)
            if (fx < 0) != (mid < 0):
                out.extend([bisect(f, ths[k - 1], x), bisect(f, x, ths[k + 1])])
    return sorted(x % tau for x in out), near


def hot(chart, partner):
    """The chart angles whose lines pass nearest the partner's apex."""
    if partner.kind != "cone":
        return []
    th, _ = chart.chart(partner.apex)
    return [th, th + mp.pi]


def read(chart, partner, length):
    """`(bounded arcs or branches, essential, unbounded count, closeness)`."""
    e = lambda t: disc(chart, partner, t)  # noqa: E731
    folds, near_e = roots(e, length**2, hot(chart, partner))
    asym = []
    near_p = mpf("inf")
    if partner.kind == "cone" and chart.kind == "cone":
        found, near_p = roots(lambda t: lead(chart, partner, t), 1, hot(chart, partner))
        for th in found:
            _, p1, _ = quadratic(chart, partner, th)
            asym.append((th, -1 if p1 > 0 else 1))
            near_p = min(near_p, abs(p1) / length)
    close = min(near_e, near_p)
    for i, a in enumerate(folds):
        for b in folds[i + 1 :]:
            close = min(close, abs(a - b))
        for th, _ in asym:
            close = min(close, abs(a - th))
    comps = []  # (kind, data): ("arc", (lo, hi)) or ("branch", sign)
    unbounded = 0
    if not folds:
        if e(0) < 0:
            return comps, False, 0, close
        for sign in (1, -1):
            hits = sum(1 for _, s in asym if s == sign)
            if hits:
                unbounded += hits
            else:
                comps.append(("branch", sign))
        return comps, True, unbounded, close
    n = len(folds)
    for i in range(n):
        lo = folds[i]
        hi = folds[i + 1] if i + 1 < n else folds[0] + 2 * mp.pi
        if e((lo + hi) / 2) <= 0:
            continue
        hits = 0
        for th, _ in asym:
            t = th if th >= lo else th + 2 * mp.pi
            hits += 1 if lo < t < hi else 0
        if hits:
            unbounded += hits
        else:
            comps.append(("arc", (lo, hi)))
    return comps, False, unbounded, close


def which(chart, partner, comps, w):
    th, t = chart.chart(w)
    for idx, (kind, data) in enumerate(comps):
        if kind == "arc":
            lo, hi = data
            x = th if th >= lo else th + 2 * mp.pi
            if lo < x < hi:
                return idx
        else:
            p2, p1, _ = quadratic(chart, partner, th)
            if (1 if 2 * p2 * t + p1 > 0 else -1) == data:
                return idx
    return None


def judge(d):
    """`None` when the answer is right, else the mismatch; or "near"."""
    cone, partner = Carrier(d["cone"]), Carrier(d["partner"])
    length = mpf(d["scale"])
    ans = d["answer"]
    if isinstance(ans, str):
        return None
    mine = read(cone, partner, length)
    theirs = read(partner, cone, length)
    if min(mine[3], theirs[3]) < mpf("1e-20"):
        return "near"
    parts = ans["parts"]
    if sum(1 for p in parts if p["unbounded"]) != mine[2] or mine[2] != theirs[2]:
        return f"unbounded: answered {parts}, read {mine[2]} and {theirs[2]}"
    bounded = [p for p in parts if not p["unbounded"]]
    if len(bounded) != len(mine[0]) or len(mine[0]) != len(theirs[0]):
        return f"bounded: answered {len(bounded)}, read {len(mine[0])} and {len(theirs[0])}"
    for p in bounded:
        if bool(p["ef"]) != mine[1] or bool(p["eg"]) != theirs[1]:
            return f"class: answered {p}, read essential {mine[1]}, {theirs[1]}"
    seen = set()
    for p in bounded:
        w = vec(p["witness"])
        for c in (cone, partner):
            if abs(c.distance(w)) > mpf("1e-9") * length:
                return f"witness {p['witness']} is {c.distance(w)} off a carrier"
        idx = which(cone, partner, mine[0], w)
        if idx is None or idx in seen:
            return f"witness {p['witness']} on no component of its own"
        seen.add(idx)
    if ans["single"] != (len(parts) == 1):
        return f"single {ans['single']} with {len(parts)} parts"
    return None


def main():
    tally, bad = {}, []
    with open(sys.argv[1]) as f:
        for line in f:
            d = json.loads(line)
            verdict = judge(d)
            key = (d["family"], "refused" if isinstance(d["answer"], str) else "answered")
            tally[key] = tally.get(key, 0) + 1
            if verdict == "near":
                tally[(d["family"], "near")] = tally.get((d["family"], "near"), 0) + 1
            elif verdict is not None:
                bad.append(f"{d['family']}: {verdict}\n  {line.strip()}")
    for key in sorted(tally):
        print(key, tally[key])
    for b in bad:
        print(b)
    print(f"{len(bad)} mismatches")
    sys.exit(1 if bad else 0)


if __name__ == "__main__":
    main()
