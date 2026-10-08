# Review 3: in_sector levers only at the read point's reach, not at S's
# bound's own reach.  A cone whose face S = B-C (z = 0) has a 1 mm edge at
# B, read by directions just past B; the same cone with B's far point
# moved in-plane by delta*1e-3 m (< the zero band) is case 2k+1.
import sys, json, math
sys.path.insert(0, '.')
from gen import *
from cases import write_case
def pt(a, z): return [math.cos(a), math.sin(a), z]
meta = []
with open('cases_bnd.txt', 'w') as f:
    cid = 0
    for zA in (0.2, -0.2):
      for hollow in (False, True):
        for delta in (0.0, 3e-7, 6e-7):   # B's bearing shift; far-point move = delta * 1e-3 m
            ring = [(pt(-0.3, zA), True, 1.0), (pt(0.05 - delta, 0.0), True, 1e-3),
                    (pt(0.5, 0.0), True, 1.0), (pt(2.0, 0.3), True, 1.0),
                    (pt(3.5, -0.3), True, 1.0), (pt(5.0, 0.15), True, 1.0)]
            secs = sectors_from_ring(ring, hollow)
            P = []
            for al in (1e-7, 2e-7, 4e-7):
                for h in (0.0, 1e-10, -1e-10, 4e-10, -4e-10, 2e-9, -2e-9):
                    u = norm(pt(0.05 - al, h)); P.append((u, u[:], f'al{al}h{h}'))
            write_case(f, cid, secs, P); cid += 1
            meta.append(dict(name=f'bnd_zA{zA}_{"h" if hollow else "s"}_d{delta}', secs=secs, P=P))
json.dump(meta, open('meta_bnd.json', 'w'))
