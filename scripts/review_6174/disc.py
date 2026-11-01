"""Exact disc sign of each dumped Nearest line against the plain and the
charged f64 disc: a charged disc of definite sign opposite to the exact
one is a charge that does not bound."""
import sys
from fractions import Fraction as F


def fr(s):
    return F(float(s))


def dot(a, b):
    return sum(x * y for x, y in zip(a, b))


n = flips_plain = flips_charged = zeroed = 0
worst = []
for line in open(sys.argv[1]):
    f = line.split()
    kind = f[0]
    if kind == 'wall':
        q, d, o, a = [list(map(fr, f[2 + 3 * i:5 + 3 * i])) for i in range(4)]
        r = fr(f[14]); plain, charged = float(f[15]), float(f[16])
        w0 = [x - y for x, y in zip(q, o)]
        w0p = [x - y * dot(w0, a) for x, y in zip(w0, a)]
        dp = [x - y * dot(d, a) for x, y in zip(d, a)]
        A, B, C = dot(dp, dp), dot(w0p, dp), dot(w0p, w0p) - r * r
    else:
        q, d, c = [list(map(fr, f[2 + 3 * i:5 + 3 * i])) for i in range(3)]
        r = fr(f[11]); plain, charged = float(f[12]), float(f[13])
        w0 = [x - y for x, y in zip(q, c)]
        A, B, C = dot(d, d), dot(w0, d), dot(w0, w0) - r * r
    # The exact disc in the shifted parameter equals B^2 - AC (shift-invariant).
    ex = B * B - A * C
    s = (ex > 0) - (ex < 0)
    sp = (plain > 0) - (plain < 0)
    sc = (charged > 0) - (charged < 0)
    n += 1
    if sp and sp != s:
        flips_plain += 1
    if sc == 0:
        zeroed += 1
    if sc and sc != s:
        flips_charged += 1
        if len(worst) < 5:
            worst.append((kind, float(ex), plain, charged))
print(f"n={n} plain-sign-wrong={flips_plain} charged-sign-wrong={flips_charged} charged-zeroed={zeroed}")
for w in worst:
    print(w)
