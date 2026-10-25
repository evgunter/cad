import json, sys, mpmath as mp
mp.mp.dps = 40
def basis(i, p, U, t):
    # value and derivative of N_{i,p} at t (half-open spans, right end handled)
    if p == 0:
        a, b = U[i], U[i+1]
        return (mp.mpf(1) if (a <= t < b) or (t == U[-1] and b == U[-1] and a < b) else mp.mpf(0)), mp.mpf(0)
    n1, d1 = basis(i, p-1, U, t); n2, d2 = basis(i+1, p-1, U, t)
    v = mp.mpf(0); d = mp.mpf(0)
    if U[i+p] != U[i]:
        c = 1/(U[i+p]-U[i]); v += (t-U[i])*c*n1; d += c*n1 + (t-U[i])*c*d1
    if U[i+p+1] != U[i+1]:
        c = 1/(U[i+p+1]-U[i+1]); v += (U[i+p+1]-t)*c*n2; d += -c*n2 + (U[i+p+1]-t)*c*d2
    return v, d
def va(c):
    p = c['deg']; U = [mp.mpf(x) for x in c['knots']]
    P = [[mp.mpf(x) for x in q] for q in c['pts']]; W = [mp.mpf(x) for x in c['w']]
    R = [mp.mpf(x) for x in c['ref']]
    n = len(P)
    def f(t, k):
        A = [mp.mpf(0)]*3; Ad=[mp.mpf(0)]*3; w=mp.mpf(0); wd=mp.mpf(0)
        for i in range(n):
            N, dN = basis(i, p, U, t)
            if N == 0 and dN == 0: continue
            w += N*W[i]; wd += dN*W[i]
            for j in range(3):
                A[j] += N*W[i]*P[i][j]; Ad[j] += dN*W[i]*P[i][j]
        Pt = [A[j]/w - R[j] for j in range(3)]
        Pd = [(Ad[j]*w - A[j]*wd)/w**2 for j in range(3)]
        cr = [Pt[1]*Pd[2]-Pt[2]*Pd[1], Pt[2]*Pd[0]-Pt[0]*Pd[2], Pt[0]*Pd[1]-Pt[1]*Pd[0]]
        return cr[k]/2
    t0, t1 = mp.mpf(c['t0']), mp.mpf(c['t1'])
    brk = sorted(set([t0, t1] + [u for u in U if t0 < u < t1]))
    out = []
    for k in range(3):
        s = mp.mpf(0)
        for a, b in zip(brk, brk[1:]):
            # integrate on the open span, nudging the evaluation off the right end
            s += mp.quad(lambda t: f(t if t < b else b - mp.mpf(10)**-35, k), [a, b])
        out.append(s)
    return out
bad = 0; cache = {}
for line in open(sys.argv[1]):
    c = json.loads(line.replace("-inf","-1e999").replace("inf","1e999").replace("NaN","null"))
    if not c.get("cert", True): print("UNCERT", c["name"], c["pieces"]); continue
    if 'refused' in c: print('REFUSED', c['name'], c['refused']); continue
    key = c['name']
    if key not in cache: cache[key] = va(c)
    truth = cache[key]
    for k in range(3):
        lo, hi = c['va'][k]
        ok = c['cert'] and mp.mpf(lo) <= truth[k] <= mp.mpf(hi)
        if not ok:
            bad += 1
            print('MISS', key, c['pieces'], k, lo, hi, mp.nstr(truth[k], 20), 'cert', c['cert'])
    if c['pieces'] == 256:
        wmax = max(h - l for l, h in c['va'])
        print('ok' if True else '', key, 'w256=%.3e' % wmax, 'truth=', [mp.nstr(x, 8) for x in truth])
print('misses', bad)
