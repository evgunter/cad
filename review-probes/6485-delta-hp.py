# 50-digit sup distance of each dumped carrier from its chart, against the
# f64 and Interval envelopes (and incidence terms) the lane computed.
import sys, mpmath as mp
mp.mp.dps = 50
def V(s): return [mp.mpf(float(x)) for x in s.split(',')]
def dot(a,b): return a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
def sub(a,b): return [a[i]-b[i] for i in range(3)]
def add(a,b): return [a[i]+b[i] for i in range(3)]
def mul(a,k): return [x*k for x in a]
def cross(a,b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def norm(a): return mp.sqrt(dot(a,a))
worst = {}
bad = []
for line in open(sys.argv[1]):
    if line.startswith('#'): continue
    what, surf, car, t0, t1, e64, eI, i64, iI, sup64 = line.rstrip('\n').split('|')
    sp = surf.split(';'); cp = car.split(';')
    O = V(sp[1]); A = V(sp[2]); A = mul(A, 1/norm(A)) if False else A
    if cp[0] == 'circle':
        c, n, u, r = V(cp[1]), V(cp[2]), V(cp[3]), mp.mpf(float(cp[4]))
        v = cross(n, u)
        P = lambda t: add(c, add(mul(u, r*mp.cos(t)), mul(v, r*mp.sin(t))))
    else:
        o, d = V(cp[1]), V(cp[2])
        P = lambda t: add(o, mul(d, t))
    def dist(p):
        w = sub(p, O); z = dot(w, A); rho = norm(sub(w, mul(A, z)))
        if sp[0] == 'cyl': return abs(rho - mp.mpf(float(sp[3])))
        if sp[0] == 'cone':
            al = mp.mpf(float(sp[3])); return abs(rho*mp.cos(al) - abs(z)*mp.sin(al))
        R, rr = mp.mpf(float(sp[3])), mp.mpf(float(sp[4]))
        return abs(mp.sqrt((rho-R)**2 + z**2) - rr)
    t0, t1 = mp.mpf(float(t0)), mp.mpf(float(t1))
    N = int(sys.argv[2]) if len(sys.argv) > 2 else 600
    best, bt = mp.mpf(0), t0
    for i in range(N+1):
        t = t0 + (t1-t0)*i/N; dd = dist(P(t))
        if dd > best: best, bt = dd, t
    h = (t1-t0)/N; lo, hi = max(bt-h, t0), min(bt+h, t1)
    for _ in range(80):
        m1 = lo + (hi-lo)*mp.mpf('0.381966'); m2 = lo + (hi-lo)*mp.mpf('0.618034')
        if dist(P(m1)) > dist(P(m2)): hi = m2
        else: lo = m1
    best = max(best, dist(P((lo+hi)/2)))
    hp = float(best)
    def num(x):
        try: return float(x)
        except: return None
    E64, EI, I64, II = num(e64), num(eI), num(i64), num(iI)
    tag = []
    for name, e in (('env64', E64), ('envI', EI), ('inc64', I64), ('incI', II)):
        if e is not None and e < hp: tag.append(f'{name}<hp')
    kind = what.split(' ')[0] + ' ' + what.split(' ')[1]
    for name, e in (('64', E64), ('I', EI)):
        if e is not None and e > 0:
            k = (kind, name); worst[k] = max(worst.get(k, 0), hp / e)
    print(f"{what}: hp {hp:.6e} env64 {e64[:12]} envI {eI[:12]} inc64 {i64[:12]} incI {iI[:12]} sup64 {float(sup64):.3e} {' '.join(tag)}", flush=True)
    if tag: bad.append(what)
print("WORST hp/envelope by kind and scalar:")
for k, v in sorted(worst.items()): print("  ", k, f"{v:.4f}")
print("UNDER:", len(bad)); [print("  ", b) for b in bad]
