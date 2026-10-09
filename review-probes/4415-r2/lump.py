import math, sys
from fractions import Fraction as F
def dot(a,b): return a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
def cross(a,b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def add(a,b): return [a[i]+b[i] for i in range(3)]
def sub(a,b): return [a[i]-b[i] for i in range(3)]
def sc(a,s): return [c*s for c in a]
def unit(a): return sc(a, 1.0/math.sqrt(dot(a,a)))
def basis(m):
    m=unit(m); seed=[0.0,0.0,1.0] if abs(m[2])<0.9 else [1.0,0.0,0.0]
    u=unit(cross(seed,m)); return u, cross(m,u)
def pose(a,d):
    e=unit([2.0,1.0,0.0]); p1,p2=basis(e)
    al=math.tau*(a+0.25)/16.0
    base=add(sc(p1,math.cos(al)),sc(p2,math.sin(al)))
    m=unit(unit(add(base,sc(e,d)))); u,w=basis(m); return u,w,m
def clip(faces,n,dd):
    kept=[];cap=[]
    for f in faces:
        g=[]
        for i in range(len(f)):
            p,q=f[i],f[(i+1)%len(f)]
            sp,sq=dot(n,p)-dd,dot(n,q)-dd
            if sp<=0: g.append(p)
            if sp*sq<0:
                x=add(p,sc(sub(q,p),sp/(sp-sq))); g.append(x); cap.append(x)
            if sp==0: cap.append(p)
        if len(g)>=3: kept.append(g)
    if len(cap)>=3:
        # order cap by angle (float for ordering only)
        c=[sum(float(p[i]) for p in cap)/len(cap) for i in range(3)]
        nf=[float(x) for x in n]; e1,e2=basis(nf)
        cap.sort(key=lambda x: math.atan2(dot([float(x[i])-c[i] for i in range(3)],e2), dot([float(x[i])-c[i] for i in range(3)],e1)))
        # dedupe
        dd2=[]
        for p in cap:
            if not dd2 or p!=dd2[-1]: dd2.append(p)
        cap=dd2
        if dot(cross(sub(cap[1],cap[0]),sub(cap[2],cap[0])),n)<0: cap.reverse()
        kept.append(cap)
    return kept
def lump(a,d):
    u,w,m=pose(a,d)
    U,W,M=[[F(x) for x in v] for v in (u,w,m)]
    v=[F(2),F(1),F(1)]
    piece=[(2,0),(4,0),(4,2),(2,1)]
    bot=[[F(x),F(y),F(0)] for x,y in piece]; top=[[F(x),F(y),F(1)] for x,y in piece]
    faces=[bot[::-1],top]+[[bot[i],bot[(i+1)%4],top[(i+1)%4],top[i]] for i in range(4)]
    # cube: v + U*a + W*b + M*c, a,b in [-2,2], c in [0,4]: half-spaces in exact arithmetic
    cuts=[(sc(M,-1),-dot(M,v)),(M,dot(M,v)+4),(U,dot(U,v)+2),(sc(U,-1),-dot(U,v)+2),(W,dot(W,v)+2),(sc(W,-1),-dot(W,v)+2)]
    for n,dd in cuts: faces=clip(faces,n,dd)
    V=F(0); A=0.0
    for f in faces:
        for i in range(1,len(f)-1):
            V+=dot(f[0],cross(f[i],f[i+1]))/6
            c=cross(sub(f[i],f[0]),sub(f[i+1],f[0])); A+=math.sqrt(float(dot(c,c)))/2
    return V, A
for a,d,lo,hi in [(3,1e-8,3.2887162002985347e-9,3.288716346922386e-9),(3,2e-8,6.577432405102582e-9,6.57743255172645e-9),(3,3e-8,9.866148530882808e-9,9.866148718183271e-9)]:
    V,A=lump(a,d); va=float(V)/A
    print(f"a{a} d{d:g}: exact-plane V/A={va:.12e}  cert=[{lo:.12e},{hi:.12e}] inside={lo<=va<=hi} rel-off={(va-(lo+hi)/2)/va:.2e}")
