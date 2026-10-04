import numpy as np, math
r=0.5; TILT=0.3
def G(y,p): # ∫ sqrt(p²-y²) dy
    y=max(-p,min(p,y)); return 0.5*(y*math.sqrt(max(p*p-y*y,0))+p*p*math.asin(y/p))
def quad_area(Y,Z,p):
    # area of disc radius p at origin ∩ {y<=Y, z<=Z}
    if Y<=-p or Z<=-p: return 0.0
    Ym=min(Y,p)
    a=math.sqrt(max(p*p-Z*Z,0.0)) if abs(Z)<p else 0.0
    if Z>=p: return 2*(G(Ym,p)-G(-p,p))
    # integrand: [min(Z,s)+s]_+
    tot=0.0
    if Z>=0:
        # |y|>a : 2s ; |y|<a : Z+s
        segs=[(-p,-a,'2s'),(-a,a,'Zs'),(a,p,'2s')]
    else:
        segs=[(-a,a,'Zs')]
    for lo,hi,k in segs:
        hi2=min(hi,Ym)
        if hi2<=lo: continue
        if k=='2s': tot+=2*(G(hi2,p)-G(lo,p))
        else: tot+=Z*(hi2-lo)+(G(hi2,p)-G(lo,p))
    return tot
def rect_area(y0,y1,z0,z1,p):
    if y1<=y0 or z1<=z0: return 0.0
    return quad_area(y1,z1,p)-quad_area(y0,z1,p)-quad_area(y1,z0,p)+quad_area(y0,z0,p)
xg,wg=np.polynomial.legendre.leggauss(10); xk,wk=np.polynomial.legendre.leggauss(21)
def adapt(f,a,b,tol,depth=60):
    c,h=(a+b)/2,(b-a)/2
    g=h*sum(w*f(c+h*x) for x,w in zip(xg,wg)); k=h*sum(w*f(c+h*x) for x,w in zip(xk,wk))
    if abs(g-k)<=tol or depth==0: return k
    return adapt(f,a,c,tol/2,depth-1)+adapt(f,c,b,tol/2,depth-1)
def vol(big,c,cut):
    z0,z1=(0.0,1.0) if cut else (-1.0,1.0)
    def sl(x):
        w2=r*r-x*x; p2=big*big-(x-c[0])**2
        if w2<=0 or p2<=0: return 0.0
        w=math.sqrt(w2); top=z1
        if cut: top=min(z1,0.5-x*math.tan(TILT))
        return rect_area(-w-c[1],w-c[1],z0-c[2],top-c[2],math.sqrt(p2))
    lo=max(-r,c[0]-big); hi=min(r,c[0]+big)
    return adapt(sl,lo,hi,1e-15)
for args in [(0.3,[0.7,0.1,-0.2],False),(0.8,[0.2,0.1,0.05],False),(0.2,[0.5,0.0,0.35],True),(0.1,[0.45,0.0,0.3],True)]:
    print(args, repr(vol(*args)))
print("check whole ball inside", vol(0.1,[0.0,0.0,0.2],True)-4/3*math.pi*1e-3)
