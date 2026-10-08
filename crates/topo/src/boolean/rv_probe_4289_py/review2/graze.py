# Targeted: exemption (a). Face s0 = P1-A whose bisector p reads Zero against
# S's plane (S = A-B, in z=0) yet lies off it, on the far side from the probe,
# so the true arc d->p crosses z=0 well away from p, inside S's sector.
import sys, json, random, math
sys.path.insert(0, '.')
from gen import *
from cases import write_case
seed = int(sys.argv[1]); random.seed(seed)
def pt(a, z): return [math.cos(a), math.sin(a), z]
meta = []
with open(f'cases_g{seed}.txt', 'w') as f:
    cid = 0
    for _ in range(int(sys.argv[2])):
        a1 = random.choice([0.03, 0.04, 0.045, 0.0, -0.1, -0.3])  # P1 angle
        zA = 0.0
        dz = random.choice([1e-9, 3e-9, 1e-8, 5e-8, 1e-7, 1.8e-7, 3e-7]) * random.choice([1, -1])
        La = random.choice([0.01, 0.001, 0.1, 1.0])          # s0's chord lengths (its arm)
        Lrest = random.choice([1.0, 0.5, 2.0])
        zc = random.choice([0.3, -0.3, 0.0, 0.5])
        ring = [(pt(a1, dz), True, La), (pt(0.05, 0.0), True, La), (pt(0.5, 0.0), True, Lrest),
                (pt(2.0, zc), True, Lrest), (pt(3.5, -zc), True, Lrest), (pt(5.0, zc*0.5), True, Lrest)]
        hollow = random.random() < 0.5
        secs = sectors_from_ring(ring, hollow)
        P = []
        for _k in range(40):
            th = random.uniform(-0.5, 1.6)
            h = random.choice([1e-8, 2e-8, 3e-8, 5e-8, 1e-7, 1e-6]) * random.choice([1, -1])
            L = random.choice([0.5, 1.0, 2.0])
            u = norm(pt(th, h)); P.append((u, mul(u, L), 'graze'))
        write_case(f, cid, secs, P); cid += 1
        meta.append(dict(name=f'graze_a{a1}_dz{dz}_La{La}', secs=secs, P=P))
json.dump(meta, open(f'meta_g{seed}.json', 'w'))
print(cid, 'cases')
