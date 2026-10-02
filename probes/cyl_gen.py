"""Circle x cylinder / torus corpus (review of PR 3847): the square arm
(wall axis = circle axis) at near-tangent and crossing depths, plus tilted
walls for the ladder.  Same word layout as cs_gen plus wall axis, u_ref."""
import random, sys, math
from cs_gen import hx, norm, cross
rnd = random.Random(int(sys.argv[1])); out = []
for i in range(int(sys.argv[2])):
    sc = 10.0 ** rnd.choice([-3, 0, 3])
    nn = norm([rnd.gauss(0, 1) for _ in range(3)]) if rnd.random() < 0.7 else [0.0, 0.0, 1.0]
    uu = norm(cross(nn, norm([rnd.gauss(0, 1) for _ in range(3)]))) if nn != [0.0, 0.0, 1.0] else [1.0, 0.0, 0.0]
    vv = cross(nn, uu)
    c = [rnd.uniform(-5, 5) * sc * (rnd.random() < 0.6) for _ in range(3)]
    rho = sc * 10 ** rnd.uniform(-1, 1)
    d = rho * 10 ** rnd.uniform(-1, 1); a = rnd.uniform(0, 2 * math.pi); h = rnd.uniform(-3, 3) * rho
    o = [c[k] + d * math.cos(a) * uu[k] + d * math.sin(a) * vv[k] + h * nn[k] for k in range(3)]
    base = rnd.choice([abs(d - rho), d + rho])
    delta = base * 10 ** rnd.uniform(-15, -1) * rnd.choice([1, -1])
    R = base + delta
    if R <= 0: continue
    if rnd.random() < 0.75:
        ax, ux = nn, uu
    else:
        ax = norm([nn[k] + rnd.gauss(0, 0.3) for k in range(3)]); ux = norm(cross(ax, norm([rnd.gauss(0, 1) for _ in range(3)])))
    for eps in [1e-12, 1e-9, 1e-6]:
        out.append(' '.join(hx(x) for x in c + nn + uu + [rho] + o + [R, -math.pi, math.pi, eps] + ax + ux))
print('\n'.join(out))
