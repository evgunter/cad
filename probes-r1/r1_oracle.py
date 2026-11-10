"""R1's independent oracle for PR 6806: exact (60-digit decimal) distance
from a point of the meridian half-plane to the sheet arc
{(R + r cos v, r sin v) : rho >= 0}; checks each dumped reading bounds it
at the box's corners and a 5x5 grid."""
import sys
from decimal import Decimal as D, getcontext
getcontext().prec = 60

def dist(R, r, rho, z):
    m = ((rho - R) ** 2 + z * z).sqrt()
    best = None
    if m > 0:
        rho_n = R + r * (rho - R) / m
        if rho_n >= 0:
            best = abs(m - r)
    h2 = (r - R) * (r + R)
    h = h2.sqrt() if h2 > 0 else D(0)
    for a in (h, -h):
        d = (rho * rho + (z - a) ** 2).sqrt()
        best = d if best is None or d < best else best
    return best

unc = 0; bad = 0; n = 0; worst = D(0); lem = 0
for line in open(sys.argv[1]):
    p = line.strip().split(',')
    if p[7] != 'true' or p[6] in ('NaN', 'inf'):
        unc += 1
        continue
    R, r, rl, rh, z0, z1, hi = (D(float(x)) for x in p[:7])
    if R <= 0:
        lem += 1
    n += 1
    K = 4
    for i in range(K + 1):
        for j in range(K + 1):
            rho = rl + (rh - rl) * i / K
            z = z0 + (z1 - z0) * j / K
            d = dist(R, r, rho, z)
            if d > hi:
                bad += 1
                if bad < 10:
                    print("MISS", line.strip(), "pt", float(rho), float(z), "d", float(d))
            elif hi > 0 and d / hi > worst:
                worst = d / hi
print(f"uncertified/NaN {unc}, cases {n}, lemon reads {lem}, misses {bad}, worst d/bound {float(worst):.6f}")
