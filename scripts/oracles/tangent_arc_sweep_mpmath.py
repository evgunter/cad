#!/usr/bin/env python3
"""The high-precision oracle for the tangent arc's sweep.

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

For comparison it also replays, in Python's IEEE doubles, the quarter-tangent
spelled `across / (|d| + along)` at every turn (the kernel takes it only up to
a half turn, and `(|d| - along) / across` past it).

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


def main(path):
    worst = {}
    bad = []
    count = 0
    for line in open(path):
        row = json.loads(line)
        count += 1
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
        w = worst.setdefault(key, {"sweep": 0.0, "miss": 0.0, "old": 0.0, "width": 0.0, "n": 0})
        w["n"] += 1
        for k, val in (("sweep", sweep_err), ("miss", miss_ulps), ("old", old_err), ("width", width)):
            w[k] = max(w[k], val)
        if not encloses:
            bad.append(f"Interval [{lo!r}, {hi!r}] misses the true sweep {truth} at {row}")
        if sweep_err > SWEEP_ULPS:
            bad.append(f"f64 sweep {sweep_err:.3g} units off at {row}")
        if miss_ulps > MISS_ULPS:
            bad.append(f"f64 end vertex missed by {miss_ulps:.3g} units at {row}")
    print(f"{count} cases")
    for key, w in sorted(worst.items()):
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
