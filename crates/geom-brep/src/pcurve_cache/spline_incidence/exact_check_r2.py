import json, struct, sys
from fractions import Fraction as F
def fb(s): return F(struct.unpack('<d', struct.pack('<Q', int(s)))[0])
def basis(knots, p, n, t):
    # Cox-de Boor, exact; returns N_i,p(t) for i in 0..n-1; t in [k_p, k_n]
    m = len(knots)
    # find span
    if t == knots[n]:
        k = n - 1
        while knots[k] == knots[k+1]: k -= 1
    else:
        k = max(i for i in range(p, n) if knots[i] <= t)
    N = [F(0)]*(p+1); N[0] = F(1)
    left = [F(0)]*(p+1); right=[F(0)]*(p+1)
    for j in range(1, p+1):
        left[j] = t - knots[k+1-j]; right[j] = knots[k+j] - t
        saved = F(0)
        for r in range(j):
            tmp = N[r] / (right[r+1] + left[j-r])
            N[r] = saved + right[r+1]*tmp
            saved = left[j-r]*tmp
        N[j] = saved
    out = [F(0)]*n
    for r in range(p+1): out[k-p+r] = N[r]
    return out
bad = 0; tot = 0
for line in open(sys.argv[1]):
    d = json.loads(line)
    ku = [fb(x) for x in d['ku']]; kv = [fb(x) for x in d['kv']]
    nu, nv, pu, pv = d['nu'], d['nv'], d['pu'], d['pv']
    cp = [[fb(c) for c in p] for p in d['cp']]; w = [fb(x) for x in d['w']]
    u, v = fb(d['u']), fb(d['v']); q = [fb(x) for x in d['q']]
    Nu = basis(ku, pu, nu, u); Nv = basis(kv, pv, nv, v)
    num = [F(0)]*3; den = F(0)
    for i in range(nu):
        if Nu[i] == 0: continue
        for j in range(nv):
            if Nv[j] == 0: continue
            c = Nu[i]*Nv[j]*w[i*nv+j]
            den += c
            for k in range(3): num[k] += c*cp[i*nv+j][k]
    P = [x/den for x in num]
    d2 = sum((P[k]-q[k])**2 for k in range(3))
    fl = fb(d["floor"]); tot += 1
    if tot <= 12: print(d["case"], "floor", float(fl), "exact", float(d2)**0.5, "esc", float(fb(d["esc"])))
    if fl*fl > d2:
        bad += 1
        print(f"FALSE OFF case {d['case']}: floor {float(fl):.3e} > exact dist {float(d2)**0.5:.3e} (escalate {float(fb(d['esc'])):.0e}) deg ({pu},{pv})")
print("checked", tot, "false offs", bad)
