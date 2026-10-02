"""Checks the running bound itself (review of PR 3847): the f64 `lo`
against the factored extreme evaluated in 60-digit mpmath on the same
stored inputs, |lo - lo*| <= lo_error; and the bound's ratio to the old
noise charge."""
import sys, struct
import mpmath as mp
mp.mp.dps = 60
def fx(h): return struct.unpack('<d', struct.pack('<Q', int(h, 16)))[0]
worst = 0; worst_n = 0; viol = 0; seen = set()
for li, (a, b) in enumerate(zip(open(sys.argv[1]), open(sys.argv[2]))):
    w = [mp.mpf(fx(h)) for h in a.split()]
    key = tuple(a.split()[:16])
    if key in seen: continue
    seen.add(key)
    C, N, U, rho, S, r = w[0:3], w[3:6], w[6:9], w[9], w[10:13], w[13]
    V = [N[1]*U[2]-N[2]*U[1], N[2]*U[0]-N[0]*U[2], N[0]*U[1]-N[1]*U[0]]
    e = [C[k]-S[k] for k in range(3)]
    eu = sum(e[k]*U[k] for k in range(3)); ev = sum(e[k]*V[k] for k in range(3)); en = sum(e[k]*N[k] for k in range(3))
    off = mp.sqrt(eu**2+ev**2); dm = mp.sqrt((off-rho)**2+en**2)
    lo_star = (dm-r)*(dm+r)/(2*r)
    o = b.split()[-6:]
    noise, lo, le = float(o[2]), float(o[3]), float(o[4])
    err = abs(mp.mpf(lo) - lo_star)
    if le > 0:
        q = float(err/le); worst = max(worst, q)
    if err > le:
        viol += 1; print('BOUND-VIOLATED', li, float(err), le)
    worst_n = max(worst_n, le/noise)
print('poses', len(seen), 'violations', viol, 'max |lo-lo*|/lo_error %.3f' % worst, 'max lo_error/noise %.3f' % worst_n)
