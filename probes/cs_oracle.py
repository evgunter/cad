"""Independent oracle for the circle x sphere probe (review of PR 3847).
Exact rational coefficients of R(t) = |C(t) - s|^2 - r^2 for the circle
C(t) = c + rho (u cos t + (n x u) sin t) on the STORED f64 frame (no
orthonormality assumed), roots by the tangent half-angle quartic in
60-digit mpmath.  Compares each certified root to the true one and each
certified Miss to the true count."""
import sys, struct
from fractions import Fraction as F
import mpmath as mp
mp.mp.dps = 60
def fx(h): return struct.unpack('<d', struct.pack('<Q', int(h, 16)))[0]
def true_roots(c, n, u, rho, s, r):
    C=[F(x) for x in c]; N=[F(x) for x in n]; U=[F(x) for x in u]; S=[F(x) for x in s]
    rho=F(rho); r=F(r)
    V=[N[1]*U[2]-N[2]*U[1], N[2]*U[0]-N[0]*U[2], N[0]*U[1]-N[1]*U[0]]
    e=[C[k]-S[k] for k in range(3)]
    dot=lambda a,b: sum(a[k]*b[k] for k in range(3))
    # |e + rho(u cos + v sin)|^2 - r^2 = A + B cos + Cs sin + D cos^2 + E sin^2 + G cos sin
    A=dot(e,e)-r*r; B=2*rho*dot(e,U); Cs=2*rho*dot(e,V)
    D=rho*rho*dot(U,U); E=rho*rho*dot(V,V); G=2*rho*rho*dot(U,V)
    # t = tan(x/2): cos=(1-t^2)/(1+t^2), sin=2t/(1+t^2); multiply by (1+t^2)^2
    # coefficients of polynomial in t (degree 4)
    p=[F(0)]*5
    def add(poly, k):
        for i,a in enumerate(poly): p[i]+=k*a
    one=[1,0,2,0,1]          # (1+t^2)^2
    cos_=[1,0,0,0,-1]        # (1-t^2)(1+t^2)
    sin_=[0,2,0,2,0]         # 2t(1+t^2)
    cc=[1,0,-2,0,1]          # (1-t^2)^2
    ss=[0,0,4,0,0]           # 4t^2
    cs=[0,2,0,-2,0]          # 2t(1-t^2)
    add(one,A); add(cos_,B); add(sin_,Cs); add(cc,D); add(ss,E); add(cs,G)
    coeffs=[mp.mpf(p[i].numerator)/p[i].denominator for i in range(4,-1,-1)]
    rts=mp.polyroots(coeffs, maxsteps=400, extraprec=400) if coeffs[0]!=0 else mp.polyroots(coeffs[1:],maxsteps=400,extraprec=400)
    real=[]
    for t in rts:
        if abs(mp.im(t)) <= mp.mpf(10)**-40*max(1,abs(t)):
            real.append(2*mp.atan(mp.re(t)))
    def R(x): return A+B*mp.cos(x)+Cs*mp.sin(x)+D*mp.cos(x)**2+E*mp.sin(x)**2+G*mp.cos(x)*mp.sin(x)
    return real, R
def main(inp, outp):
    rows=[]
    bad=0; stats={}; mx={}
    for li,(a,b) in enumerate(zip(open(inp),open(outp))):
        w=[fx(h) for h in a.split()]
        c,n,u,rho,s,r,eps=w[0:3],w[3:6],w[6:9],w[9],w[10:13],w[13],w[16]
        o=b.split(); kind=o[0]
        key=(eps,kind[0]); stats[key]=stats.get(key,0)+1
        if kind[0] not in 'CM': continue
        real, R=true_roots(c,n,u,rho,s,r)
        if kind.startswith('C'):
            ths=[fx(o[1]),fx(o[2])][:int(kind[1])]
            for th in ths:
                if not real:
                    print('CERT-NO-TRUE-ROOT', li, eps, th); bad+=1; continue
                d=min(abs(mp.atan2(mp.sin(th-x),mp.cos(th-x))) for x in real)
                arc=rho*d
                mx[eps]=max(mx.get(eps,0),float(arc/eps))
                if arc>eps:
                    print('ROOT-OFF', li, 'eps',eps,'arc_err %.3e'%float(arc), 'ratio %.2f'%float(arc/eps), 'rho',rho,'r',r); bad+=1
            if len(real)!=2: print('COUNT', li, eps, len(real)); bad+=1
        elif kind=='M':
            if len(real)>0:
                print('MISS-ON-CROSSING', li, eps, len(real)); bad+=1
    print('verdicts', sorted(stats.items())); print('violations', bad); print('max certified arc error over eps', {k: round(v, 4) for k, v in mx.items()})
if __name__=='__main__': main(sys.argv[1], sys.argv[2])
