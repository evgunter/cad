# Review 4: directed band-move attack on every decided reading.
# usage: adv.py gen SEED N  -> cases_adv_SEED.txt, meta_adv_SEED.json
#        adv.py check SEED OUT EPS
import sys, json, math, random
sys.path.insert(0, '..'); sys.path.insert(0, '.')
from gen import sectors_from_ring, rot
from cases import write_case
from oracle_a import *

def pt(a, z): return [math.cos(a), math.sin(a), z]

def star(n, lo, hi, thin, turn):
    th = sorted(random.uniform(0, 2*math.pi) for _ in range(n))
    if thin:
        k = random.randrange(n-1); th[k+1] = th[k] + random.choice([1e-3, 1e-6, 1e-8, 3e-9]); th.sort()
    gaps = [(th[(i+1) % n]-th[i]) % (2*math.pi) for i in range(n)]
    if max(gaps) >= 0.98*math.pi: return None
    ax = fnorm([random.uniform(-1, 1) for _ in range(3)]); ang = random.uniform(0, 3) if turn else 0
    out = []
    for t in th:
        z = random.uniform(lo, hi) if random.random() > 0.15 else random.choice([lo, hi])
        p = [math.cos(t), math.sin(t), z]
        if turn: p = rot(ax, ang, p)
        out.append((p, True, random.choice([1e-3, 1e-2, 0.1, 0.5, 1.0, 2.0, 10.0])))
    return out

def near_flat_180():
    # a face whose two bounds are nearly opposite (sector ~ 180 - 2b)
    b = random.choice([1e-2, 1e-3, 1e-4, 3e-5, 1e-5])
    z = random.choice([0.3, -0.3, 0.1])
    Ls = [random.choice([1e-3, 0.01, 1.0, 5.0]) for _ in range(5)]
    return [(pt(0, 0), True, Ls[0]), (pt(math.pi - 2*b, 0), True, Ls[1]),
            (pt(math.pi + 0.8, z), True, Ls[2]), (pt(math.pi + 2.2, -z), True, Ls[3]),
            (pt(-0.6, z*0.5), True, Ls[4])]

def graze():
    a1 = random.choice([0.03, 0.04, 0.045, 0.0, -0.1, -0.3])
    dz = random.choice([1e-9, 3e-9, 1e-8, 5e-8, 1e-7, 3e-7]) * random.choice([1, -1])
    la = random.choice([0.01, 0.001, 0.1, 1.0]); lr = random.choice([1.0, 0.5, 2.0]); zc = random.choice([0.3, -0.3, 0.0, 0.5])
    return [(pt(a1, dz), True, la), (pt(0.05, 0.0), True, la), (pt(0.5, 0.0), True, lr),
            (pt(2.0, zc), True, lr), (pt(3.5, -zc), True, lr), (pt(5.0, zc*0.5), True, lr)]

def fin():
    g = random.choice([1e-3, 1e-5, 1e-6, 2e-7])
    def sph(az, el): az, el = math.radians(az), math.radians(el); return [math.cos(az)*math.cos(el), math.sin(az)*math.cos(el), math.sin(el)]
    Lb = random.choice([1e-3, 1e-2, 0.1]); L = random.choice([0.5, 2.0])
    return [(sph(0, 0), True, L), (sph(g/2, 80), True, Lb), (sph(g, 0), True, L), (sph(120, -10), True, L), (sph(240, 10), True, L)]

def family(rng_kind):
    while True:
        if rng_kind == 0: r = star(3 + random.randrange(8), 0.1, 3.0, random.random() < 0.3, False)
        elif rng_kind == 1: r = star(3 + random.randrange(8), -2.0, 2.0, random.random() < 0.3, False)
        elif rng_kind == 2: r = star(3 + random.randrange(8), -1.0, 3.0, random.random() < 0.3, True)
        elif rng_kind == 3: r = near_flat_180()
        elif rng_kind == 4: r = graze()
        else: r = fin()
        if r: return r

def directed_probes(secs, eps, budget):
    P = []
    for _ in range(budget):
        s = random.choice(secs)
        n = s['n']
        key = random.choice(('fs', 'fe'))
        FB = s[key]; Lb = flen(FB)
        ub = fnorm([FB[i] - fdot(FB, n)*n[i] for i in range(3)])
        perp = fnorm(cross(n, ub))
        other = s['fe'] if key == 'fs' else s['fs']
        inward = 1.0 if fdot(perp, other) > 0 else -1.0
        Ld = random.choice([0.01, 0.3, 1.0, 3.0, 50.0])
        kind = random.random()
        if kind < 0.6:
            r = random.choice([1.5, 4, 8, 10.5, 12, 20, 40, 200])
            lever = random.choice([min(Ld, Lb), Ld, Lb, Ld*Lb/(Ld+Lb)])
            al = min(0.3, r*eps/lever)
            side = random.choice([1, -1])  # -1: past the bound
            h = random.choice([0, 0.5, -0.5, 0.95, -0.95, 3, -3, 11, -11, 30]) * eps
            dirv = [math.cos(al)*ub[i] + side*inward*math.sin(al)*perp[i] for i in range(3)]
            D = [Ld*dirv[i] + h*n[i] for i in range(3)]
            tag = f'bnd r{r} side{side} h{h/eps:g} Ld{Ld} Lb{Lb:g}'
        elif kind < 0.8:
            # off the edge out of plane: along the bound, nudged by r eps in a random normal direction
            r = random.choice([0.5, 0.95, 1.5, 3, 9, 11, 30])
            w = fnorm([random.gauss(0, 1) for _ in range(3)])
            w = fnorm([w[i] - fdot(w, ub)*ub[i] for i in range(3)])
            D = [Ld*ub[i] + r*eps*w[i] for i in range(3)]
            tag = f'edge r{r} Ld{Ld} Lb{Lb:g}'
        else:
            # over the face interior, or opposite it
            a, b = random.random(), random.random()
            m = fnorm([a*s['fs'][i]/flen(s['fs']) + b*s['fe'][i]/flen(s['fe']) for i in range(3)])
            if random.random() < 0.3: m = [-x for x in m]
            h = random.choice([0.5, 0.95, 1.5, 9, 11]) * eps * random.choice([1, -1])
            D = [Ld*m[i] + h*n[i] for i in range(3)]
            tag = f'face h{h/eps:g} Ld{Ld}'
        u = fnorm(D)
        P.append((u, D, tag))
    for _ in range(budget // 4):
        u = fnorm([random.gauss(0, 1) for _ in range(3)]); L = random.uniform(0.5, 2)
        P.append((u, mul(u, L), 'rand'))
    return P

def est_flip(secs, D):
    best = 1e300
    for s in secs:
        n = s['n']; h = fdot(D, n); pr = [D[i]-h*n[i] for i in range(3)]
        if fdot(cross(s['fs'], pr), n) >= 0 and fdot(cross(pr, s['fe']), n) >= 0:
            best = min(best, abs(h)); continue
        for B in (s['fs'], s['fe']):
            b = fnorm(B); Lb = flen(B); t = max(0.0, fdot(pr, b))
            w = flen([pr[i]-t*b[i] for i in range(3)])
            best = min(best, max(abs(h), w/(1+t/Lb)))
    return best

def planar(secs, eps):
    for s in secs:
        n = fr(s['n'])
        for far in (s['fs'], s['fe']):
            if abs(dot(fr(far), n)) > F(eps): return False
    return True

if __name__ == '__main__':
    mode = sys.argv[1]; seed = sys.argv[2]
    if mode == 'gen':
        random.seed(seed); N = int(sys.argv[3]); eps = float(sys.argv[4])
        meta = []
        with open(f'cases_adv_{seed}.txt', 'w') as f:
            cid = 0
            while cid < N:
                kind = cid % 6
                ring = family(kind)
                hollow = random.random() < 0.5
                secs = sectors_from_ring(ring, hollow)
                if not planar(secs, eps): continue
                P = directed_probes(secs, eps, 60)
                write_case(f, cid, secs, P)
                meta.append(dict(kind=kind, hollow=hollow, secs=secs, P=P)); cid += 1
        json.dump(meta, open(f'meta_adv_{seed}.json', 'w'))
    else:
        out = sys.argv[3]; eps = float(sys.argv[4]); random.seed(1)
        meta = json.load(open(f'meta_adv_{seed}.json'))
        rows = []; cur = None
        for l in open(out):
            w = l.split()
            if not w: continue
            if w[0] == 'case': cur = []; rows.append(cur)
            else: cur.append((w[2], w[3]))
        import collections
        st = collections.Counter(); hits = []
        for ci, (m, rs) in enumerate(zip(meta, rows)):
            orc = OracleA(m['secs'])
            for pi, ((u, D, tag), (wc, cs)) in enumerate(zip(m['P'], rs)):
                truth = orc.cls(D)
                dec = [(nm, g) for nm, g in (('cs', cs), ('wc', wc)) if g in ('In', 'Out', 'On')]
                for nm, g in (('cs', cs), ('wc', wc)):
                    st[f'{nm}_{g.split(":")[0]}'] += 1
                if not dec: continue
                for nm, g in dec:
                    if g in ('In', 'Out') and g != truth:
                        st['WRONG_' + nm] += 1; hits.append(('WRONG', ci, pi, nm, g, truth, tag))
                # band moves (single point and joint)
                if any(g in ('In', 'Out') for _, g in dec) and est_flip(m['secs'], D) < 3*eps:
                    st['near'] += 1
                    for lab, s2, D2 in moves(m['secs'], D, eps):
                        o2 = orc if s2 is m['secs'] else OracleA(s2)
                        t2 = o2.cls(D2)
                        if t2 in ('In', 'Out') and t2 != truth and truth != 'On':
                            for nm, g in dec:
                                if g in ('In', 'Out'):
                                    st['FLIP_' + nm] += 1
                                    hits.append(('FLIP', ci, pi, nm, g, truth, t2, lab, tag))
                            break
        print(dict(st))
        for h in hits[:40]: print(h)
