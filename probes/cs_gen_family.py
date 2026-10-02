"""The PR's near-tangent family widened (review of PR 3847): unit circle
x sphere of radius R centred at (rho + R - delta, 0, 0) and the internal
twin (rho - R + delta), scales 1e-3..1e3, delta = 2^-k, k = 4..52."""
import sys
from cs_gen import hx
out = []
for sc in [1e-3, 1.0, 1e3]:
    for R in [0.75, 0.01, 100.0, 1.0]:
        for k in range(4, 53):
            d = 2.0 ** -k
            for internal in [False, True]:
                if internal and R <= d: continue
                x = (1.0 - R + d) if internal else (1.0 + R - d)
                for eps in [1e-12, 1e-9, 1e-6]:
                    out.append(' '.join(hx(v) for v in [0, 0, 0, 0, 0, 1, 1, 0, 0, sc, x * sc, 0, 0, R * sc, -3.14159, 3.14159, eps]))
print('\n'.join(out))
