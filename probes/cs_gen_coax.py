"""Near-coaxial crossings (review of PR 3847): the sphere's centre a tiny
offset off the circle's axis, the circle nearly ON the sphere, so the
roots hang on the phase phi = atan2(e.v, e.u) and its error."""
import random, sys, math
from cs_gen import hx, norm, cross
def gen(seed, n, eps_list):
    rnd = random.Random(seed); out = []
    for i in range(n):
        scale = 10.0 ** rnd.choice([-3, 0, 3])
        nn = norm([rnd.gauss(0, 1) for _ in range(3)]) if rnd.random() < 0.8 else [0.0, 0.0, 1.0]
        uu = norm(cross(nn, norm([rnd.gauss(0, 1) for _ in range(3)]))) if nn[2] != 1.0 else [1.0, 0.0, 0.0]
        vv = cross(nn, uu)
        c = [rnd.uniform(-5, 5) * scale * (rnd.random() < 0.6) for _ in range(3)]
        rho = scale * 10 ** rnd.uniform(-1, 1)
        h = rho * 10 ** rnd.uniform(-1, 1.5) * rnd.choice([1, -1])
        o = rho * 10 ** rnd.uniform(-9, -2)
        a = rnd.uniform(0, 2 * math.pi)
        s = [c[k] + o * math.cos(a) * uu[k] + o * math.sin(a) * vv[k] + h * nn[k] for k in range(3)]
        # c0 = (h^2 + o^2 + rho^2 - r^2)/2r, A1 = rho*o/r : put c0 = -t*A1
        t = rnd.uniform(-0.999, 0.999)
        r0 = math.sqrt(h * h + o * o + rho * rho)
        r = r0 + t * rho * o / r0
        for eps in eps_list:
            out.append(' '.join(hx(x) for x in c + nn + uu + [rho] + s + [r, -math.pi, math.pi, eps]))
    return out
if __name__ == '__main__':
    print('\n'.join(gen(int(sys.argv[1]), int(sys.argv[2]), [1e-12, 1e-9, 1e-6])))
