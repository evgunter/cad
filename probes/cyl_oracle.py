"""Oracle for the circle x cylinder corpus (review of PR 3847): exact
rational |w - (w.a)a/|a|^2|^2 - R^2 along C(t) on the stored frame,
roots by the half-angle polynomial (degree 4) in 60-digit mpmath.
Reports, for rows whose bits differ between main and head, each lane's
worst certified arc error over eps."""
import sys, struct
from fractions import Fraction as F
import mpmath as mp
mp.mp.dps = 60
fx = lambda h: struct.unpack('<d', struct.pack('<Q', int(h, 16)))[0]
def roots(w):
    C=[F(x) for x in w[0:3]]; N=[F(x) for x in w[3:6]]; U=[F(x) for x in w[6:9]]; rho=F(w[9])
    O=[F(x) for x in w[10:13]]; R=F(w[13]); A=[F(x) for x in w[17:20]]
    V=[N[1]*U[2]-N[2]*U[1], N[2]*U[0]-N[0]*U[2], N[0]*U[1]-N[1]*U[0]]
    dot=lambda a,b: sum(a[k]*b[k] for k in range(3)); aa=dot(A,A)
    e=[C[k]-O[k] for k in range(3)]
    # w(t) = e + rho(U cos + V sin); f = w.w - (w.A)^2/aa - R^2
    def q(p, r_):  # bilinear form p.r - (p.A)(r.A)/aa
        return dot(p, r_) - dot(p, A)*dot(r_, A)/aa
    k0=q(e,e)-R*R; kc=2*rho*q(e,U); ks=2*rho*q(e,V); kcc=rho*rho*q(U,U); kss=rho*rho*q(V,V); kcs=2*rho*rho*q(U,V)
    p=[F(0)]*5
    def add(poly,k):
        for i,a in enumerate(poly): p[i]+=k*a
    add([1,0,2,0,1],k0); add([1,0,0,0,-1],kc); add([0,2,0,2,0],ks); add([1,0,-2,0,1],kcc); add([0,0,4,0,0],kss); add([0,2,0,-2,0],kcs)
    co=[mp.mpf(p[i].numerator)/p[i].denominator for i in range(4,-1,-1)]
    while co and co[0]==0: co=co[1:]
    rs=mp.polyroots(co,maxsteps=400,extraprec=400)
    return [2*mp.atan(mp.re(t)) for t in rs if abs(mp.im(t))<=mp.mpf(10)**-40*max(1,abs(t))]
def worst(w, toks):
    if not toks or toks[0][0]!='C': return None
    real=roots(w); rho=w[9]; eps=w[16]
    if not real: return float('inf')
    return max(float(rho*min(abs(mp.atan2(mp.sin(fx(t)-x),mp.cos(fx(t)-x))) for x in real))/eps for t in toks[1:])
inp, mn, hd = sys.argv[1:4]
wm=wh=0; cnt=0; over_m=over_h=0
for a,b,c in zip(open(inp),open(mn),open(hd)):
    m=b.split('|')[0].split(); h=c.split('|')[0].split()
    if m==h: continue
    w=[fx(x) for x in a.split()]
    x=worst(w,m); y=worst(w,h); cnt+=1
    if x is not None: wm=max(wm,x); over_m+= x>1
    if y is not None: wh=max(wh,y); over_h+= y>1
print('differing rows',cnt,'main worst arc/eps %.3g (over: %d)'%(wm,over_m),'head worst arc/eps %.3g (over: %d)'%(wh,over_h))
