import sys, random, math, json
sys.path.insert(0, '.')
from gen import *

def star_ring(n, zlo, zhi, thin=False, axis=None):
    th = sorted(random.uniform(0, 2*math.pi) for _ in range(n))
    if thin and n > 3:
        k = random.randrange(n-1)
        th[k+1] = th[k] + random.choice([1e-3, 1e-6, 1e-8, 3e-9])
        th.sort()
    # ensure gaps < pi
    gaps = [(th[(i+1)%n]-th[i]) % (2*math.pi) for i in range(n)]
    if max(gaps) >= math.pi*0.98: return None
    ring = []
    for t in th:
        z = random.uniform(zlo, zhi)
        if random.random() < 0.15: z = random.choice([zlo, zhi])
        p = [math.cos(t), math.sin(t), z]
        ring.append((p, True))
    if axis is not None:
        ax, ang = axis
        ring = [(rot(ax, ang, p), e) for p, e in ring]
    return ring

def handcrafted():
    out = []
    r2 = 1/math.sqrt(2)
    # L-prism corner: top z=0 with a 270-degree corner, walls x=0 (y>0) and y=0 (x>0)
    A, B, C = [1,0,0], [0,0,-1], [0,1,0]
    D = [-r2, -r2, 0]
    out.append(('Ltop', [(A, True), (B, True), (C, True), (D, False)]))
    out.append(('Ltop_r', [(D, False), (C, True), (B, True), (A, True)]))
    # staircase-ish: two reflex
    out.append(('dart', [([1,0,1],True),([0,1,1],True),([0.3,0,1],True),([0,-1,1],True)]))
    for x in (0.9, 0.99, 0.999999, 1-1e-8, 1-2e-9, 1-1e-9, 1-3e-10):
        out.append((f'sliver{x}', [([1,0,1],True),([0,1,1],True),([x,0,1],True),([0,-1,1],True)]))
    for x in (1e-3, 2e-8, 5e-9, 1e-9, 1e-10, 0.0, -1e-10, -5e-9):
        out.append((f'shallow{x}', [([1,0,1],True),([0,1,1],True),([x-1,0,1],True),([0,-1,1],True)]))
    # saddles
    for a in (0.4, 0.05, 1e-6):
        out.append((f'saddle{a}', [([1,0,a],True),([0,1,-a],True),([-1,0,a],True),([0,-1,-a],True)]))
    out.append(('asym_saddle', [([1,0,.4],True),([0,1,-.4],True),([-1,0,.9],True),([0,-1,-.1],True)]))
    out.append(('monkey', [([math.cos(k*math.pi/3), math.sin(k*math.pi/3), 0.3*(-1)**k], True) for k in range(6)]))
    # straight face through the vertex (a face corner of 180): bisector mid
    out.append(('straight', [([1,0,0],True),([0,0,-1],False),([-1,0,0],True),([0,1,0.5],True)]))
    # near-flat quad pyramid void-ish
    for dent in (1e-3, 1e-8, 1e-7):
        out.append((f'flat{dent}', [([1,0,dent],True),([0,1,dent],True),([-1,0,dent],True),([0,-1,dent],True)]))
        out.append((f'flatdart{dent}', [([1,0,dent],True),([0,1,dent],True),([-0.2,0,-dent],True),([0,-1,dent],True)]))
    for x in (1-1e-6, 1-1e-7, 1-3e-8, 1-1e-8):
        for Lb in (0.01, 0.001):
            out.append((f'longsliver{x}_{Lb}', [([1,0,1],True,2.0),([0,1,1],True,Lb),([x,0,1],True,2.0),([0,-1,1],True,1.0)]))
    def sph(az, el, L):
        az, el = math.radians(az), math.radians(el)
        return [math.cos(az)*math.cos(el), math.sin(az)*math.cos(el), math.sin(el)]
    for g in (1e-3, 1e-4, 1e-5, 2e-6, 1e-6, 2e-7):
        for Lb in (0.001, 0.01, 0.1):
            out.append((f'fin{g}_{Lb}', [(sph(0,0,0),True,0.5),(sph(g/2,80,0),True,Lb),(sph(g,0,0),True,0.5),(sph(120,-10,0),True,.5),(sph(240,10,0),True,.5)]))
    return out

def probes_for(secs, n_random=12):
    P = []
    def add(d, tag):
        L = random.uniform(0.5, 2.0)
        u = norm(d); P.append((u, mul(u, L), tag))
    for _ in range(n_random):
        add([random.gauss(0,1) for _ in range(3)], 'rand')
    ps = random.sample(secs, min(len(secs), 4))
    for s in ps:
        n = s['n']
        for where in ('in', 'out'):
            a, b = random.random(), random.random()
            if where == 'in': base = add_(mul(s['s'], a), mul(s['e'], b))
            else: base = add_(mul(s['s'], -a), mul(s['e'], b*0.3))
            base = norm(base)
            for off in (0, 1e-12, -1e-12, 3e-9, -3e-9, 2e-8, -2e-8, 1e-6, -1e-6, 1e-3, -1e-3):
                add(add_(base, mul(n, off)), f'{where}{off:g}')
        # near and opposite bounds
        for b in (s['s'], s['e']):
            add(mul(b, -1), 'antibound')
            add(add_(b, [random.gauss(0, 1e-7) for _ in range(3)]), 'nearbound')
        p = norm(add_(s['s'], s['e']))
        add(mul(p, -1), 'antip')
        add(add_(mul(p, -1), [random.gauss(0, 1e-6) for _ in range(3)]), 'nearantip')
        add(p, 'p')
    c = [0, 0, 0]
    for s in secs: c = add_(c, add_(s['s'], s['e']))
    if math.sqrt(dot(c, c)) > 1e-12: add(mul(c, -1), 'antic')
    return P

def add_(a, b): return [x+y for x, y in zip(a, b)]

def write_case(f, cid, secs, P):
    f.write(f"case {cid} {len(secs)} {len(P)}\n")
    for s in secs:
        vals = s['s'] + s['e'] + s['n'] + [1.0 if s['es'] else 0.0, 1.0 if s['ee'] else 0.0] + s['fs'] + s['fe'] + [s['arm'], s['face']]
        f.write("s " + " ".join(repr(float(x)) for x in vals) + "\n")
    for u, far, tag in P:
        f.write("p " + " ".join(repr(float(x)) for x in u + far) + "\n")

ONLY = None
if __name__ == '__main__':
    seed = int(sys.argv[1]); N = int(sys.argv[2]); random.seed(seed)
    cases = []
    for name, ring in handcrafted():
        for hollow in (False, True):
            cases.append((f'{name}/{"h" if hollow else "s"}', ring, hollow))
    if len(sys.argv) > 3: cases = [c for c in cases if sys.argv[3] in c[0]]
    while len(cases) < N:
        kind = random.random()
        n = random.randint(3, 14)
        if kind < 0.4: ring = star_ring(n, 0.1, 3.0, thin=random.random() < 0.3)
        elif kind < 0.7: ring = star_ring(n, -2.0, 2.0, thin=random.random() < 0.3)
        else: ring = star_ring(n, -1.0, 3.0, thin=random.random() < 0.3,
                               axis=([random.gauss(0,1) for _ in range(3)], random.uniform(0, 3)))
        if ring is None: continue
        cases.append((f'r{len(cases)}_n{n}', ring, random.random() < 0.5))
    meta = []
    with open(f'cases_{seed}.txt', 'w') as f:
        for cid, (name, ring, hollow) in enumerate(cases):
            secs = sectors_from_ring(ring, hollow)
            P = probes_for(secs)
            write_case(f, cid, secs, P)
            meta.append(dict(name=name, secs=secs, P=[(u, far, tag) for u, far, tag in P]))
    json.dump(meta, open(f'meta_{seed}.json', 'w'))
    print(len(cases), 'cases', sum(len(m['P']) for m in meta), 'probes')
