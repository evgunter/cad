import json, sys
import mpmath as mp
mp.mp.dps = 40
def basis(knots, p, i, t):
    # Cox-de Boor value and derivative, right-continuous; t strictly inside spans in quad
    def N(i, p, t):
        if p == 0:
            return mp.mpf(1) if knots[i] <= t < knots[i+1] else mp.mpf(0)
        a = 0 if knots[i+p] == knots[i] else (t-knots[i])/(knots[i+p]-knots[i])*N(i,p-1,t)
        b = 0 if knots[i+p+1] == knots[i+1] else (knots[i+p+1]-t)/(knots[i+p+1]-knots[i+1])*N(i+1,p-1,t)
        return a+b
    def dN(i,p,t):
        a = 0 if knots[i+p]==knots[i] else p/(knots[i+p]-knots[i])*N(i,p-1,t)
        b = 0 if knots[i+p+1]==knots[i+1] else p/(knots[i+p+1]-knots[i+1])*N(i+1,p-1,t)
        return a-b
    return N(i,p,t), dN(i,p,t)
seen = {}
bad = 0
for line in open(sys.argv[1]):
    d = json.loads(line)
    key = d['name']
    if key not in seen:
        K = [mp.mpf(x) for x in d['knots']]; p = d['degree']
        P = [[mp.mpf(c) for c in q] for q in d['pts']]; W = [mp.mpf(x) for x in d['w']]
        R = [mp.mpf(c) for c in d['ref']]
        def integrand(t, comp):
            A=[0,0,0]; dA=[0,0,0]; w=0; dw=0
            for i in range(len(P)):
                n, dn = basis(K,p,i,t)
                w += n*W[i]; dw += dn*W[i]
                for c in range(3):
                    A[c]+=n*W[i]*P[i][c]; dA[c]+=dn*W[i]*P[i][c]
            X=[A[c]/w - R[c] for c in range(3)]
            dX=[(dA[c]*w - A[c]*dw)/w**2 for c in range(3)]
            cr=[X[1]*dX[2]-X[2]*dX[1], X[2]*dX[0]-X[0]*dX[2], X[0]*dX[1]-X[1]*dX[0]]
            return cr[comp]/2
        t0, t1 = mp.mpf(d['t'][0]), mp.mpf(d['t'][1])
        brk = sorted(set([t0,t1]+[k for k in K if t0<k<t1]))
        tr = [mp.quad(lambda t: integrand(t,c), brk) for c in range(3)]
        seen[key] = tr
    tr = seen[key]
    for c in range(3):
        lo, hi = map(float, d["va"][c])
        ok = mp.mpf(lo) <= tr[c] <= mp.mpf(hi)
        if not ok:
            bad += 1
            print("FAIL", key, d['pieces'], c, lo, hi, mp.nstr(tr[c], 20))
    print("%-36s p=%-4d z=[%.17g, %.17g] truth=%s width=%.3g" % (key, d['pieces'], float(d["va"][2][0]), float(d["va"][2][1]), mp.nstr(tr[2],18), float(d["va"][2][1])-float(d["va"][2][0])))
print("bad", bad)
