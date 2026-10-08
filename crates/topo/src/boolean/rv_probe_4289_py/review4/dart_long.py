# A dart (4 faces, one reflex edge) whose reflex dent g is flat to within
# eps at its bounds' short reach Lb, read by long probes in the region
# below one reflex-adjacent plane and above the other.
import sys, json, math, random
from gen import sectors_from_ring, Oracle as OracleB
from cases import write_case
from oracle_a import *
random.seed(4)
meta = []
with open('cases_dl.txt', 'w') as f:
    cid = 0
    for g in (1e-5, -1e-5, 1e-6, -1e-6, 3e-7, -3e-7):
        for Lb in (1e-3, 1e-2):
            for hollow in (False, True):
                ring = [([1, 0, 0], True, Lb), ([0, 1, g], True, Lb), ([-1, 0, 0], True, Lb), ([0, -1, 0], True, Lb)]
                secs = sectors_from_ring(ring, hollow)
                P = []
                for Ld in (1.0, 50.0, 1000.0):
                    for k in range(60):
 
                        az = random.uniform(0.05, math.pi - 0.05)
                        el = random.choice([1, -1]) * random.choice([3e-9, 1e-8, 3e-8, 1e-7, 1e-6, 1e-5]) * random.uniform(0.5, 2) / Ld * 1e3
                        u = [math.cos(az) * math.cos(el), math.sin(az) * math.cos(el), math.sin(el)]
                        u = fnorm(u); P.append((u, [x * Ld for x in u], f'g{g} Lb{Lb} Ld{Ld}'))
                write_case(f, cid, secs, P); meta.append(dict(g=g, Lb=Lb, hollow=hollow, secs=secs, P=P)); cid += 1
json.dump(meta, open('meta_dl.json', 'w'))
