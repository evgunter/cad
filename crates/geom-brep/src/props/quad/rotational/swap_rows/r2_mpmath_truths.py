# Independent truths for r2_projected_integrals_hold_high_precision_truths:
# a NURBS evaluated by de Boor in mpmath at 40 digits, u' = (x y' - y x')/(x^2+y^2)
# from the curve's own derivative, integrated by tanh-sinh per knot span.
import mpmath as mp
mp.mp.dps = 40
def polar(d, rho, z):
    a = mp.radians(d); rho = mp.mpf(float(rho)); return [rho*mp.cos(a), rho*mp.sin(a), mp.mpf(z)]
F = mp.mpf
chords = [
 ([0,0,0,0,1,1,1,1],3,[[1,0,0],[1,1.2,0.3],[-0.5,1.3,0.9],[-1.1,0.2,1.4]],[1,0.7,1.3,1]),
 ([0,0,0,0,1,1,1,1],3,[polar(150,1.0,0),polar(200,'1.5620499',0.3),polar(260,'1.3928388',0.9),polar(320,'1.1180340',1.4)],[1,0.7,1.3,1]),
 ([0,0,0,F(1)/3,F(2)/3,1,1,1],2,[polar(0,1.0,0),polar(100,1.3,0.4),polar(220,1.2,0.8),polar(340,1.25,1.2),polar(470,1.0,1.6)],[1,0.8,1.1,0.9,1]),
]
def basis(knots, p, i, t):
    if p == 0:
        return F(1) if (knots[i] <= t < knots[i+1]) or (t == knots[-1] and knots[i] < t <= knots[i+1]) else F(0)
    a = F(0); b = F(0)
    if knots[i+p] != knots[i]: a = (t-knots[i])/(knots[i+p]-knots[i])*basis(knots,p-1,i,t)
    if knots[i+p+1] != knots[i+1]: b = (knots[i+p+1]-t)/(knots[i+p+1]-knots[i+1])*basis(knots,p-1,i+1,t)
    return a+b
def hom(c, t):
    knots, p, P, w = c
    # f64 control values, as the Rust test passes them
    out = [F(0)]*4
    for i in range(len(P)):
        N = basis(knots, p, i, t)
        for k in range(3): out[k] += N*F(float(P[i][k]))*F(float(w[i]))
        out[3] += N*F(float(w[i]))
    return out
r, voff, vsign, anc = F(0.8), F(0.3), -1, F(0.25)
res = []
for c in chords:
    knots = [F(float(k)) for k in c[0]]
    cc = (knots, c[1], c[2], c[3])
    def f(t, k):
        B = hom(cc, t); dB = mp.diff(lambda s: hom(cc, s)[0], t), mp.diff(lambda s: hom(cc, s)[1], t)
        du = (B[0]*dB[1]-B[1]*dB[0])/(B[0]**2+B[1]**2)
        v = voff + vsign*B[2]/B[3]
        return [-r*r*(v-anc)*du, -r*(v-anc)*du, du][k]
    brk = sorted(set([F(0), F(1), F(0.37)] + [k for k in knots if 0 < k < 1]))
    row = []
    for k in range(3):
        row.append(sum(mp.quad(lambda t: f(t, k), [brk[i], brk[i+1]]) for i in range(len(brk)-1)))
    res.append(row)
print("[" + ", ".join("[" + ", ".join(mp.nstr(x, 20) for x in row) + "]" for row in res) + "]")
