#!/usr/bin/env python3
"""The 50-digit oracle for an ellipse's projected envelope.

Reads the `ROW` lines `geom_brep::pcurve_cache::ellipse_lane_rows::the_search_as_rows`
prints (one draw per line: the chart, the stored ellipse, the span, and check
4's `f64` and `Interval` envelopes) and measures, at 50 significant digits from
the STORED values, never from the kernel's arithmetic, the carrier's largest
distance from its chart over the span:

- the carrier is `C(t) = C0 + a*u*cos t + b*(n x u)*sin t` on the stored frame,
  each printed number read as the `f64` it round-trips to, then exactly;
- the distances are the surfaces' own: `||q| - R|` (sphere), `|rho - R|`
  (cylinder), `|rho cos(alpha) - sigma h sin(alpha)|` (the image's own nappe, `sigma`
  the sign the row carries, so excess on the other nappe is read) and
  `|hypot(rho - R, h) - r|` (torus), with the axis read as unit.

The span is sampled at 512 points and the largest sample refined by golden
section. The envelope bounds the image's miss, which is at least that distance,
so each envelope must be at least it. Prints the count and the tightest ratio
envelope / distance per chart, and exits non-zero on any envelope under its
distance. Run: `python3 scripts/oracles/ellipse_lane_50_digit.py <dump>`
(mpmath).
"""

import sys

import mpmath as mp

mp.mp.dps = 50


def vec(xs):
    return [mp.mpf(float(x)) for x in xs]


def dot(a, b):
    return sum(x * y for x, y in zip(a, b, strict=True))


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def unit(a):
    n = mp.sqrt(dot(a, a))
    return [x / n for x in a]


def split(q, axis):
    h = dot(q, axis)
    w = [x - axis_i * h for x, axis_i in zip(q, axis, strict=True)]
    return h, mp.sqrt(dot(w, w))


def distance_of(chart):
    kind, nums = chart[0], vec(chart[1:])
    if kind == "sphere":
        c, r = nums[0:3], nums[3]
        return lambda p: abs(mp.sqrt(dot(sub(p, c), sub(p, c))) - r)
    if kind == "cylinder":
        o, ax, r = nums[0:3], unit(nums[3:6]), nums[6]
        return lambda p: abs(split(sub(p, o), ax)[1] - r)
    if kind == "cone":
        o, ax, alpha, sigma = nums[0:3], unit(nums[3:6]), nums[6], nums[7]
        sa, ca = mp.sin(alpha), mp.cos(alpha)

        def cone(p):
            h, rho = split(sub(p, o), ax)
            return abs(rho * ca - sigma * h * sa)

        return cone
    if kind == "torus":
        o, ax, big, small = nums[0:3], unit(nums[3:6]), nums[6], nums[7]

        def torus(p):
            h, rho = split(sub(p, o), ax)
            return abs(mp.hypot(rho - big, h) - small)

        return torus
    raise ValueError(kind)


def sub(a, b):
    return [x - y for x, y in zip(a, b, strict=True)]


def largest_distance(chart, carrier, span, k=512):
    """The carrier's largest 50-digit distance from its chart over the span."""
    nums = vec(carrier)
    c0, n, u, a, b = nums[0:3], nums[3:6], nums[6:9], nums[9], nums[10]
    v = cross(n, u)
    t0, t1 = vec(span)
    dist = distance_of(chart)

    def at(t):
        ct, st = mp.cos(t), mp.sin(t)
        return dist([c0[i] + a * u[i] * ct + b * v[i] * st for i in range(3)])

    ts = [t0 + (t1 - t0) * i / k for i in range(k + 1)]
    ds = [at(t) for t in ts]
    j = max(range(k + 1), key=lambda i: ds[i])
    lo, hi = ts[max(j - 1, 0)], ts[min(j + 1, k)]
    g = (mp.sqrt(5) - 1) / 2
    for _ in range(80):
        m1, m2 = hi - g * (hi - lo), lo + g * (hi - lo)
        if at(m1) > at(m2):
            hi = m2
        else:
            lo = m1
    return max(ds[j], at((lo + hi) / 2))


def main(path):
    worst = {}
    bad = 0
    with open(path) as dump:
        for line in dump:
            if not line.startswith("ROW "):
                continue
            chart, carrier, span, env = (part.split() for part in line[4:].split("|"))
            true = largest_distance(chart, carrier, span)
            for name, e in zip(("f64", "Interval"), env, strict=True):
                e = mp.mpf(float(e))
                ratio = e / true if true > 0 else mp.inf
                key = (chart[0], name)
                ratio_so_far, count = worst.get(key, (mp.inf, 0))
                worst[key] = (min(ratio_so_far, ratio), count + 1)
                if e < true:
                    bad += 1
                    print(f"UNDER {name}: envelope {mp.nstr(e, 17)} < distance {mp.nstr(true, 20)}: {line.strip()}")
    for (chart, name), (ratio, count) in sorted(worst.items()):
        print(f"{chart:8} {name:8} rows {count:4}  tightest envelope/distance {mp.nstr(ratio, 6)}")
    print(f"under: {bad}")
    return 1 if bad else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1]))
