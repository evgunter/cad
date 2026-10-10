import sys
from fractions import Fraction as F
def fl(s): return [float(x) for x in s.split()] if s.strip() else []
def ivs(s): return [tuple(float(y) for y in x.split(':')) for x in s.split()] if s.strip() else []
def insert(knots, p, line, u):
    # Boehm: one insertion of u into knots (list of floats), line: list of tuples of Fractions (homogeneous)
    U = [F(k) for k in knots]; uq = F(u)
    k = max(i for i in range(len(U)-1) if U[i] <= uq and uq < U[i+1]) if uq < U[-1] else None
    n = len(line)
    new = []
    for i in range(n+1):
        if i <= k-p: new.append(line[i])
        elif i >= k+1: new.append(line[i-1])
        else:
            a = (uq-U[i])/(U[i+p]-U[i])
            new.append(tuple((1-a)*x + a*y for x,y in zip(line[i-1], line[i])))
    nk = sorted(knots + [u])
    return nk, new
def refine(knots, p, line, fine):
    old = list(knots); add = list(fine)
    for k in old: add.remove(k)
    for u in add:
        knots, line = insert(knots, p, line, u)
    assert knots == list(fine), (knots, fine)
    return line
def within(q, iv): return F(iv[0]) <= q <= F(iv[1])
bad = 0; tot = 0; worst_w = []
for ln in open(sys.argv[1]):
    tag, _, rest = ln.partition(' ')
    if tag == 'CURVE':
        f = rest.rstrip('\n').split('|')
        p = int(f[0]); k = fl(f[1]); w = fl(f[2]); cs = [fl(f[3]), fl(f[4]), fl(f[5])]; fine = fl(f[6])
        net = [ivs(f[7]), ivs(f[8]), ivs(f[9])]
        line = [tuple([F(w[i])*F(cs[c][i]) for c in range(3)] + [F(w[i])]) for i in range(len(w))]
        r = refine(k, p, line, fine)
        for j, h in enumerate(r):
            for c in range(3):
                tot += 1
                if not within(h[c]/h[3], net[c][j]):
                    bad += 1; print('CURVE miss', p, j, c, float(h[c]/h[3]), net[c][j])
    elif tag == 'WCHAN':
        f = rest.rstrip('\n').split('|')
        p = int(f[0]); k = fl(f[1]); w = fl(f[2]); fine = fl(f[3]); h = ivs(f[4]); rc = ivs(f[5])
        r = refine(k, p, [(F(x),) for x in w], fine)
        for j, q in enumerate(r):
            tot += 2
            if not within(q[0], h[j]): bad += 1; print('apply_homogeneous miss', j, float(q[0]), h[j])
            if not within(q[0], rc[j]): bad += 1; print('refine_certified miss', j, float(q[0]), rc[j])
            if rc[j][0] <= 0: print('rc weight not positive', rc[j])
    elif tag == 'SURF':
        f = rest.rstrip('\n').split('|')
        pu, pv, nu, nv = map(int, f[0].split())
        ku, kv, w = fl(f[1]), fl(f[2]), fl(f[3]); X, Y, Z = fl(f[4]), fl(f[5]), fl(f[6])
        gku, gkv = fl(f[7]), fl(f[8]); gw = ivs(f[9]); gp = [ivs(f[10]), ivs(f[11]), ivs(f[12])]
        net = [[tuple([F(w[i*nv+j])*F(A[i*nv+j]) for A in (X,Y,Z)] + [F(w[i*nv+j])]) for j in range(nv)] for i in range(nu)]
        # refine along u per column
        cols = [refine(ku, pu, [net[i][j] for i in range(nu)], gku) for j in range(nv)]
        nu2 = len(cols[0])
        rows = [refine(kv, pv, [cols[j][i] for j in range(nv)], gkv) for i in range(nu2)]
        nv2 = len(rows[0])
        assert nu2*nv2 == len(gw)
        for i in range(nu2):
            for j in range(nv2):
                h = rows[i][j]; idx = i*nv2+j
                tot += 4
                if not within(h[3], gw[idx]): bad += 1; print('grid weight miss', idx)
                for c in range(3):
                    if not within(h[c]/h[3], gp[c][idx]): bad += 1; print('grid point miss', idx, c)
    elif tag in ('NETREFUSED', 'GRIDREFUSED'):
        print(ln.strip())
print('checked', tot, 'bad', bad)
