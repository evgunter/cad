#!/usr/bin/env python3
"""The high-precision oracle for an arc's sweep from a chord split.

Reads the JSON lines `profile`'s `path::tests::dump_tangent_arcs_for_the_mpmath_oracle`
writes: the tip `a`, the director's components `c`, the target `b`, and the arc
`tangent_arc` stored at f64 (centre, radius, sweep) and at Interval (the sweep's
and the radius's enclosures). Each case's true sweep is re-derived at 60 digits
from the dumped inputs alone, as twice the signed angle from `c` to `b - a`:

- f64: the stored sweep's error, in units of 2^-52, and the stored arc's miss of
  its own end vertex `b` (the stored arc landed from `a` at 60 digits), in units
  of 2^-52 times the larger of the radius and the coordinates — the resolution
  any f64 arc has there;
- Interval: the sweep's enclosure contains the true sweep, and its width.

Rows keyed `C` are arcs about a known centre (`path::center_sweep`, on the
carrier through `a`): the true
sweep is the angle between the two radii wound into the arc's sense, and the
f64 error and the enclosure are checked the same way.

For comparison it also replays, in Python's IEEE doubles, the sweep spelled
`4·atan(across / (|d| + along))`, the home's first arm alone, which the kernel
replaced with the select between both arms (`sugar::split_half_tan`).

It prints the worst of each, per region (short arcs, half turns, near a full
turn), and exits non-zero on any violation. Run:
`python3 scripts/oracles/tangent_arc_sweep_mpmath.py <dump>`.
"""

import json
import math
import sys

from mpmath import atan2, cos, mp, mpf, sin, sqrt

mp.dps = 60
ULP = 2.0**-52
# f64 bounds, in units of 2^-52: the director's rounding moves the sweep by
# about 2 units, and 4*atan(X) near a full turn rounds by up to 4 more.
SWEEP_ULPS = 16.0
MISS_ULPS = 32.0


def cancelling_sweep(a, c, b):
    """The near-side spelling at every turn, in the kernel's f64 operation order."""
    n = math.sqrt(c[0] * c[0] + c[1] * c[1])
    ux, uy = c[0] / n, c[1] / n
    dx, dy = b[0] - a[0], b[1] - a[1]
    along = ux * dx + uy * dy
    across = ux * dy - uy * dx
    den = math.sqrt(dx * dx + dy * dy) + along
    x = across / den if den != 0.0 else math.copysign(math.inf, across)
    return 4.0 * math.atan(x)


def region(turn):
    if turn < 1e-2:
        return "short"
    if turn > 2 * math.pi - 1e-2:
        return "near-full"
    return "middle"


def centre_row(row, worst, bad):
    """An arc about a known centre (`bulge_from_center`, and the `Center`
    leg's sweep from it). The true sweep is the angle between the two
    radii, wound into the sense `w` — in (0, 2π] or [−2π, 0). The bulge X
    is checked against the exact value of the arm the kernel reads (the
    select's, by the sign of the centre's offset), relatively: the two
    arms are equal only for ends exactly on one circle, which rounded
    ends are not, so that value, not tan(θ/4) of the radii's angle, is
    what an enclosure owes. It is also checked by what it builds: the
    carrier the chord and X imply, `L·(1 + X²)/(4|X|)`, against the one
    the exact X implies, in units of 2^-52 times the larger of the radius
    and the coordinates. (Against the authored radius `|a − o|` it also
    carries the ends' own rounding, which moves the authored centre off
    the chord's bisector by `r·δ/L` over a short chord, whatever X is.)"""
    a, o, b, w = row["a"], row["o"], row["b"], row["w"]
    am, om, bm = [[mpf(v) for v in p] for p in (a, o, b)]
    va = [am[0] - om[0], am[1] - om[1]]
    vb = [bm[0] - om[0], bm[1] - om[1]]
    turn = atan2(va[0] * vb[1] - va[1] * vb[0], va[0] * vb[0] + va[1] * vb[1])
    wound = w * turn
    if wound <= 0:
        wound += 2 * mp.pi
    truth = w * wound
    sweep_err = float(abs(mpf(row["f"]) - truth)) / ULP
    lo, hi = row["i"]
    # The kernel's split, exactly: (σ·h, σ·p) of hypotenuse r = |a − o|.
    ch = [bm[0] - am[0], bm[1] - am[1]]
    length = sqrt(ch[0] ** 2 + ch[1] ** 2)
    mid = [(am[0] + bm[0]) / 2, (am[1] + bm[1]) / 2]
    p_off = ((om[0] - mid[0]) * (-ch[1]) + (om[1] - mid[1]) * ch[0]) / length
    r = sqrt(va[0] ** 2 + va[1] ** 2)
    across, along = w * length / 2, w * p_off
    exact_x = across / (r + along) if along >= 0 else (r - along) / across
    x = mpf(row["x"])

    def implied(x):
        return length * (1 + x * x) / (4 * abs(x))

    scale = max(float(r), math.hypot(*a), math.hypot(*b), math.hypot(*o))
    radius_err = float(abs(implied(x) - implied(exact_x))) / (scale * ULP)
    x_err = float(abs(x - exact_x) / abs(exact_x)) / ULP
    xlo, xhi = row["xi"]
    key = "centre " + region(row["turn"])
    cw = worst.setdefault(
        key,
        {"sweep": 0.0, "x": 0.0, "radius": 0.0, "width": 0.0, "xwidth": 0.0, "n": 0},
    )
    cw["x"] = max(cw["x"], x_err)
    cw["n"] += 1
    cw["sweep"] = max(cw["sweep"], sweep_err)
    cw["radius"] = max(cw["radius"], radius_err)
    cw["width"] = max(cw["width"], hi - lo)
    cw["xwidth"] = max(cw["xwidth"], float((mpf(xhi) - mpf(xlo)) / abs(exact_x)))
    exact_sweep = 4 * mp.atan(exact_x)
    if not mpf(lo) <= exact_sweep <= mpf(hi):
        bad.append(
            f"Interval [{lo!r}, {hi!r}] misses the exact centre sweep {exact_sweep} at {row}"
        )
    if x_err > SWEEP_ULPS:
        bad.append(
            f"f64 bulge {x_err:.3g} units (relative) off its exact value at {row}"
        )
    if not mpf(xlo) <= exact_x <= mpf(xhi):
        bad.append(
            f"Interval X [{xlo!r}, {xhi!r}] misses the exact X {exact_x} at {row}"
        )
    if sweep_err > SWEEP_ULPS:
        bad.append(f"f64 centre sweep {sweep_err:.3g} units off at {row}")
    if radius_err > MISS_ULPS:
        bad.append(
            f"f64 bulge's carrier {radius_err:.3g} units off the radius at {row}"
        )


def main(path):
    worst = {}
    bad = []
    count = 0
    for line in open(path):
        row = json.loads(line)
        count += 1
        if row.get("k") == "C":
            centre_row(row, worst, bad)
            continue
        a, c, b = row["a"], row["c"], row["b"]
        am = [mpf(v) for v in a]
        d = [mpf(b[0]) - am[0], mpf(b[1]) - am[1]]
        cm = [mpf(v) for v in c]
        truth = 2 * atan2(cm[0] * d[1] - cm[1] * d[0], cm[0] * d[0] + cm[1] * d[1])
        cx, cy, rad, sw = row["f"]
        sweep_err = float(abs(mpf(sw) - truth)) / ULP
        v = [am[0] - mpf(cx), am[1] - mpf(cy)]
        cs, sn = cos(mpf(sw)), sin(mpf(sw))
        land = [mpf(cx) + v[0] * cs - v[1] * sn, mpf(cy) + v[0] * sn + v[1] * cs]
        miss = float(sqrt((land[0] - mpf(b[0])) ** 2 + (land[1] - mpf(b[1])) ** 2))
        scale = max(rad, math.hypot(*a), math.hypot(*b))
        miss_ulps = miss / (scale * ULP)
        old_err = float(abs(mpf(cancelling_sweep(a, c, b)) - truth)) / ULP
        lo, hi = row["i"][0], row["i"][1]
        encloses = mpf(lo) <= truth <= mpf(hi)
        width = hi - lo
        key = region(row["turn"])
        w = worst.setdefault(
            key, {"sweep": 0.0, "miss": 0.0, "old": 0.0, "width": 0.0, "n": 0}
        )
        w["n"] += 1
        for k, val in (
            ("sweep", sweep_err),
            ("miss", miss_ulps),
            ("old", old_err),
            ("width", width),
        ):
            w[k] = max(w[k], val)
        if not encloses:
            bad.append(
                f"Interval [{lo!r}, {hi!r}] misses the true sweep {truth} at {row}"
            )
        if sweep_err > SWEEP_ULPS:
            bad.append(f"f64 sweep {sweep_err:.3g} units off at {row}")
        if miss_ulps > MISS_ULPS:
            bad.append(f"f64 end vertex missed by {miss_ulps:.3g} units at {row}")
    print(f"{count} cases")
    for key, w in sorted(worst.items()):
        if key.startswith("centre"):
            print(
                f"{key:>19} ({w['n']}): f64 sweep error {w['sweep']:.3g} units of 2^-52, "
                f"bulge relative error {w['x']:.3g} units, "
                f"bulge's carrier off the exact one {w['radius']:.3g} units, Interval sweep "
                f"width {w['width']:.3g}, Interval X relative width {w['xwidth']:.3g}"
            )
            continue
        print(
            f"{key:>10} ({w['n']}): f64 sweep error {w['sweep']:.3g} units of 2^-52, "
            f"end miss {w['miss']:.3g} units, Interval width {w['width']:.3g}; "
            f"the cancelling spelling's sweep error {w['old']:.3g} units"
        )
    for line in bad[:20]:
        print("VIOLATION", line)
    if bad:
        print(f"{len(bad)} violations")
        sys.exit(1)


if __name__ == "__main__":
    main(sys.argv[1])
