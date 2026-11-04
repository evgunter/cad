import math, random
def form(R,r,rho,z): return ((rho-R)**2+z*z-r*r)*((rho+R)**2+z*z-r*r)
def dist(R,r,rho,z):
    m=math.hypot(rho-R,z)
    if R + r*(rho-R)/m >= 0: return abs(m-r)
    h=math.sqrt((r-R)*(r+R))
    return min(math.hypot(rho,z-h),math.hypot(rho,z+h))
def over(R,r,f,rlo,rhi,z0,z1,side_rho):
    zmin=0 if z0<=0<=z1 else min(z0*z0,z1*z1)
    lo,hi=(rlo+R)**2,(rhi+R)**2
    least=min(lo,hi)+zmin-r*r; most=max(lo,hi)+max(z0*z0,z1*z1)-r*r
    s=1 if R>0 else -1
    outer=((1+s)*least-(1-s)*most)*0.5
    if not outer>0: return None
    first=f/(r*outer); d=f/((r+max(r-first,0))*outer)
    rr = rhi if side_rho=='hi' else rlo
    if not r*rr-d*abs(R)>0: return None
    return d
random.seed(1); worst=0;hits=0
for i in range(400000):
    r=random.uniform(0.2,2); u=random.uniform(-0.98,0.98)
    if abs(u)<0.02: u=0.5
    R=r*u; h=math.sqrt((r-R)*(r+R))
    sc=10**random.uniform(-6,0)
    rlo=max(0,random.uniform(-0.2,1)*sc*random.choice([0,1])) if random.random()<0.5 else random.uniform(0,2*r)
    rhi=rlo+sc*random.random()*3
    zc=random.choice([h,-h,0,random.uniform(-2*r,2*r)])+sc*random.uniform(-1,1)
    dz=sc*random.random(); z0,z1=zc-dz,zc+dz
    n=10; f=0; far=0
    for a in range(n+1):
        for b in range(n+1):
            rho=rlo+(rhi-rlo)*a/n; z=z0+(z1-z0)*b/n
            f=max(f,abs(form(R,r,rho,z))); far=max(far,dist(R,r,rho,z))
    f=2*f+1e-300
    d=over(R,r,f,rlo,rhi,z0,z1,'hi')
    if d is None: continue
    if far>d*(1+1e-9)+1e-15:
        hits+=1
        if far/d>worst: worst=far/d; ex=(R,r,rlo,rhi,z0,z1,d,far, over(R,r,f,rlo,rhi,z0,z1,'lo'))
print(hits, worst); print(ex if hits else None)
