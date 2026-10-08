import sys, math
from multiprocessing import Pool
import mpmath as m
m.mp.dps = 50
TAU = 2*m.pi

def case(line):
    f = line.split()
    ell = f[0] == '1'
    c = [m.mpf(x) for x in f[1:4]]; k = [m.mpf(x) for x in f[4:7]]; u = [m.mpf(x) for x in f[7:10]]
    a, b, t0, t1 = (m.mpf(x) for x in f[10:14]); p = [m.mpf(x) for x in f[14:17]]
    got, lever, crest, cap = (float(x) for x in f[17:21])
    v = [k[1]*u[2]-k[2]*u[1], k[2]*u[0]-k[0]*u[2], k[0]*u[1]-k[1]*u[0]]
    bb = b if ell else a
    def fd(t):
        co, s = m.cos(t), m.sin(t)
        return m.sqrt(sum((c[i] + u[i]*a*co + v[i]*bb*s - p[i])**2 for i in range(3)))
    lo, hi = min(t0, t1), max(t0, t1)
    if hi - lo > TAU: hi = lo + TAU
    best = max(fd(t0), fd(t1))
    n = 400
    ts = [lo + (hi-lo)*i/n for i in range(n+1)]
    ds = [fd(t) for t in ts]
    best = max(best, max(ds))
    peaks = [i for i in range(1, n) if ds[i] >= ds[i-1] and ds[i] >= ds[i+1]]
    peaks = sorted(peaks, key=lambda i: -ds[i])[:3]
    for i in peaks:
        A, B = ts[i-1], ts[i+1]
        for _ in range(70):
            m1 = A + (B-A)*m.mpf('0.381966011250105151795'); m2 = A + (B-A)*m.mpf('0.618033988749894848205')
            if fd(m1) < fd(m2): A = m1
            else: B = m2
        best = max(best, fd((A+B)/2))
    crest_in = None
    if not ell:
        w = [p[i]-c[i] for i in range(3)]
        uw = sum(u[i]*w[i] for i in range(3)); vw = sum(v[i]*w[i] for i in range(3))
        ts_ = m.atan2(-a*vw, -a*uw)
        # bring into [lo, hi]
        q = m.ceil((lo - ts_)/TAU); tt = ts_ + q*TAU
        crest_in = tt <= hi
        if crest_in: best = max(best, fd(tt))
    scale = math.sqrt(sum(float(x)**2 for x in c)) + math.sqrt(sum(float(x)**2 for x in p)) + abs(float(a))
    ulp = scale * 2.0**-52
    truth = float(best)
    return (ell, (truth - got)/ulp, (got - truth)/ulp, crest_in, got < crest, (got - lever), line.strip(), truth, abs(float(b)) if ell else abs(float(a)), float(hi-lo))

if __name__ == '__main__':
    lines = open(sys.argv[1]).read().splitlines()
    with Pool(int(sys.argv[2]) if len(sys.argv) > 2 else 3) as pool:
        res = pool.map(case, lines, chunksize=50)
    import pickle; pickle.dump(res, open(sys.argv[1] + '.pkl', 'wb'))
    for kind in (False, True):
        R = [r for r in res if r[0] == kind]
        sh = [r[1] for r in R]
        print('ellipse' if kind else 'circle', len(R),
              'short>0:', sum(s > 0 for s in sh), '>1ulp:', sum(s > 1 for s in sh), '>4ulp:', sum(s > 4 for s in sh), '>16ulp:', sum(s > 16 for s in sh), 'max short ulps:', max(sh))
        print('  past lever:', sum(r[5] > 0 for r in R))
        if not kind:
            over = [r[2] for r in R]
            print('  circle over >4ulp:', sum(o > 4 for o in over), 'max over ulps:', max(over))
            capb = [r for r in R if r[3] and r[4]]
            print('  crest in span & got < f64 crest (cap binding below crest):', len(capb), 'of which short>0:', sum(r[1] > 0 for r in capb), 'max short', max([r[1] for r in capb], default=0))
        worst = sorted(R, key=lambda r: -r[1])[:3]
        for w in worst: print('   worst short', w[1], w[6][:200])
