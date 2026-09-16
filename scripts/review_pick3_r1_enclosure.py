"""Exact check of EDIT-PICK3's headline claim, lane pick3-r1.

Reads the `SPAN` lines the Rust row `dump_admitted_spans_for_the_exact_
enclosure_check` prints (18 f64 bit patterns: the three corners, the ray
origin, the ray direction, then t / t_lo / t_hi as the door answered
them) and recomputes the crossing in EXACT rational arithmetic from the
same f64 inputs.

The claim under test, verbatim from `crates/editor-core/src/resolve/
pick.rs`'s `TSpan` doc: "[t_lo, t_hi] encloses the parameter of the true
crossing whenever that crossing is a point of the closed triangle,
taking the corners and the ray as exact."

So: exact u, v, t from the exact ray/triangle; skip the draw when the
exact crossing is NOT a point of the closed triangle (that is outside
what the door claims); otherwise assert t_lo <= t_exact <= t_hi.
"""

import struct
import sys
from fractions import Fraction as F


def bits_to_frac(h):
    x = struct.unpack("<d", struct.pack("<Q", int(h, 16)))[0]
    return F(x), x


def sub(a, b):
    return [a[i] - b[i] for i in range(3)]


def cross(a, b):
    return [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def main():
    seen = out_of_triangle = checked = escapes = 0
    worst_tight = F(0)
    worst = []
    for line in sys.stdin:
        if not line.startswith("SPAN "):
            continue
        f = line.split()[1:]
        seen += 1
        vals = [bits_to_frac(h) for h in f]
        q = [v[0] for v in vals]
        a = q[0:3]
        b = q[3:6]
        c = q[6:9]
        o = q[9:12]
        d = q[12:15]
        t_r, t_lo, t_hi = q[15], q[16], q[17]
        e1 = sub(b, a)
        e2 = sub(c, a)
        p = cross(d, e2)
        det = dot(e1, p)
        if det == 0:
            continue
        s = sub(o, a)
        u = dot(s, p) / det
        qq = cross(s, e1)
        v = dot(d, qq) / det
        t = dot(e2, qq) / det
        if not (0 <= u <= 1 and 0 <= v <= 1 and u + v <= 1):
            out_of_triangle += 1
            continue
        checked += 1
        half = (t_hi - t_lo) / 2
        mid = (t_hi + t_lo) / 2
        if half > 0:
            tight = abs(t - mid) / half
            if tight > worst_tight:
                worst_tight = tight
        if not (t_lo <= t <= t_hi):
            escapes += 1
            if len(worst) < 6:
                miss = max(t - t_hi, t_lo - t)
                worst.append(
                    "ESCAPE t_exact=%s  span=[%r, %r] rounded %r  miss=%.3e  "
                    "miss/half=%.3e\n  line: %s"
                    % (
                        float(t),
                        vals[16][1],
                        vals[17][1],
                        vals[15][1],
                        float(miss),
                        float(miss / half) if half else float("inf"),
                        line.strip(),
                    )
                )
    print("dumped rows read: %d" % seen)
    print("exact crossing off the closed triangle (outside the claim): %d" % out_of_triangle)
    print("exact crossing ON the closed triangle, checked: %d" % checked)
    print("escapes: %d" % escapes)
    print("worst |t_exact - centre| / half-width: %.6f" % float(worst_tight))
    for w in worst:
        print(w)
    return 1 if escapes else 0


if __name__ == "__main__":
    sys.exit(main())
