# FacLever witness: a face whose sector is 180 - 2b degrees, probed exactly
# along a bound it shares with a neighbour. Both sector orders.
import json, math
from gen import sectors_from_ring
from cases import write_case
from oracle_a import *
def pt(a, z): return [math.cos(a), math.sin(a), z]
meta = []
with open('cases_fac.txt', 'w') as f:
    cid = 0
    for b in (1e-3, 1e-4, 3e-5, 1e-5):
        for Lb in (1.0, 0.01):
            ring = [(pt(0, 0), True, Lb), (pt(math.pi - 2*b, 0), True, Lb), (pt(math.pi + 0.8, 0.3), True, 1.0),
                    (pt(math.pi + 2.2, -0.3), True, 1.0), (pt(-0.6, 0.15), True, 1.0)]
            secs = sectors_from_ring(ring, False)
            for order in (0, 1):
                ss = secs if order == 0 else secs[1:] + secs[:1]
                P = []
                for B in (ring[1][0], ring[0][0]):
                    u = fnorm(B)
                    for Ld in (1.0, 0.01):
                        P.append((u, [x*Ld for x in u], f'b{b} Lb{Lb} order{order} Ld{Ld}'))
                write_case(f, cid, ss, P); meta.append(dict(b=b, Lb=Lb, order=order, secs=ss, P=P)); cid += 1
json.dump(meta, open('meta_fac.json', 'w'))
