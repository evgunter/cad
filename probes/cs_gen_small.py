"""Small spheres grazing a large circle (review of PR 3847): r/rho from
1e-2 down to 1e-7, so the m^2 term bound over 2r dwarfs the near
extreme's own scale -- where acos(-c0/A1) would read c0/A1's rounding."""
import random, sys, math
import mpmath as mp
from cs_gen import hx, norm, cross
mp.mp.dps = 60
rnd = random.Random(int(sys.argv[1])); out = []
for i in range(int(sys.argv[2])):
    sc = 10.0 ** rnd.choice([-3, 0, 3])
    nn = norm([rnd.gauss(0, 1) for _ in range(3)]); uu = norm(cross(nn, norm([rnd.gauss(0, 1) for _ in range(3)])))
    vv = cross(nn, uu)
    c = [rnd.uniform(-5, 5) * sc * (rnd.random() < 0.5) for _ in range(3)]
    rho = sc * 10 ** rnd.uniform(-0.5, 0.5)
    D = rho * 10 ** rnd.uniform(-7, -2)
    a = rnd.uniform(0, 2 * math.pi); b = rnd.uniform(0, 2 * math.pi)
    # centre at distance D from the circle point at angle a, direction (radial, normal) at angle b
    rad = rho + D * math.cos(b); h = D * math.sin(b)
    s = [c[k] + rad * math.cos(a) * uu[k] + rad * math.sin(a) * vv[k] + h * nn[k] for k in range(3)]
    C = [mp.mpf(x) for x in c]; S = [mp.mpf(x) for x in s]; N = [mp.mpf(x) for x in nn]; U = [mp.mpf(x) for x in uu]
    V = [N[1]*U[2]-N[2]*U[1], N[2]*U[0]-N[0]*U[2], N[0]*U[1]-N[1]*U[0]]
    e = [S[k]-C[k] for k in range(3)]
    eu = sum(e[k]*U[k] for k in range(3)); ev = sum(e[k]*V[k] for k in range(3)); en = sum(e[k]*N[k] for k in range(3))
    dm = mp.sqrt((mp.sqrt(eu**2+ev**2)-rho)**2+en**2)
    r = float(dm * (1 + 10 ** rnd.uniform(-6, 0)))
    for eps in [1e-12, 1e-9, 1e-6]:
        out.append(' '.join(hx(x) for x in c + nn + uu + [rho] + s + [r, -math.pi, math.pi, eps]))
print('\n'.join(out))
