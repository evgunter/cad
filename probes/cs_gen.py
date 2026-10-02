"""Pose generator for the circle x sphere probe (review of PR 3847).
Near-tangent and crossing poses: tilted frames, off-plane centres,
scales 1e-3..1e3, delta from 1e-4 down to the f64 floor, internal and
external tangencies, both straddle sides.  Writes hex f64 words."""
import random, struct, sys, math
import mpmath as mp
mp.mp.dps = 60
def hx(x): return '%x' % struct.unpack('<Q', struct.pack('<d', float(x)))[0]
def norm(v):
    l = math.sqrt(sum(a*a for a in v)); return [a/l for a in v]
def cross(a, b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def gen(seed, n, eps_list):
    rnd = random.Random(seed); out = []
    for i in range(n):
        scale = 10.0 ** rnd.choice([-3, 0, 3])
        flat = rnd.random() < 0.2
        if flat:
            nn, uu = [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]
        else:
            nn = norm([rnd.gauss(0, 1) for _ in range(3)])
            uu = norm(cross(nn, norm([rnd.gauss(0, 1) for _ in range(3)])))
        c = [rnd.uniform(-5, 5) * scale * (rnd.random() < 0.6) for _ in range(3)]
        rho = scale * 10 ** rnd.uniform(-1, 1)
        # sphere centre in the circle's frame (exact stored frame, v = n x u)
        vv = cross(nn, uu)
        a = rnd.uniform(0, 2*math.pi)
        rad = rho * 10 ** rnd.uniform(-1.5, 1.5) * rnd.choice([1, 1, 0.3, 3])
        h = rho * rnd.uniform(-2, 2) * rnd.choice([0, 0.1, 1])
        su = rad * math.cos(a); sv = rad * math.sin(a)
        s = [c[k] + su*uu[k] + sv*vv[k] + h*nn[k] for k in range(3)]
        # D- and D+ in high precision from the stored f64 values
        C = [mp.mpf(x) for x in c]; S = [mp.mpf(x) for x in s]
        N = [mp.mpf(x) for x in nn]; U = [mp.mpf(x) for x in uu]
        V = [N[1]*U[2]-N[2]*U[1], N[2]*U[0]-N[0]*U[2], N[0]*U[1]-N[1]*U[0]]
        e = [S[k]-C[k] for k in range(3)]
        eu = sum(e[k]*U[k] for k in range(3)); ev = sum(e[k]*V[k] for k in range(3)); en = sum(e[k]*N[k] for k in range(3))
        off = mp.sqrt(eu**2+ev**2)
        dm = mp.sqrt((off-rho)**2+en**2); dp = mp.sqrt((off+rho)**2+en**2)
        side = rnd.choice(['near', 'far'])
        sign = rnd.choice([1, -1])
        delta = 10 ** rnd.uniform(-16, -4) * float(dm if side == 'near' else dp)
        if rnd.random() < 0.3: delta = 10 ** rnd.uniform(-4, -0.5) * float(dm + dp) / 2
        base = dm if side == 'near' else dp
        # near: r = D- + delta crosses (near point inside); far: r = D+ - delta crosses
        r = float(base + sign*delta) if side == 'near' else float(base - sign*delta)
        if r <= 0: continue
        t0, t1 = -math.pi, math.pi
        for eps in eps_list:
            out.append(' '.join(hx(x) for x in c + nn + uu + [rho] + s + [r, t0, t1, eps]))
    return out
if __name__ == '__main__':
    seed, n = int(sys.argv[1]), int(sys.argv[2])
    print('\n'.join(gen(seed, n, [1e-12, 1e-9, 1e-6])))
