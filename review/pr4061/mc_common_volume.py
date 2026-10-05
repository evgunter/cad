import math, random
def sub(a,b): return [a[i]-b[i] for i in range(3)]
def dot(a,b): return sum(a[i]*b[i] for i in range(3))
def cross(a,b): return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
def unit(v): l=math.sqrt(dot(v,v)); return [x/l for x in v]
def frame(m,psi):
    m=unit(m); seed=[0,0,1.] if abs(m[2])<0.9 else [1.,0,0]
    u0=unit(cross(seed,m)); w0=cross(m,u0); c,s=math.cos(psi),math.sin(psi)
    u=[c*u0[i]+s*w0[i] for i in range(3)]; w=cross(m,u); return [u,w,m]
def wedge(alpha):
    def at(t):
        t=math.radians(t); r=2/max(abs(math.cos(t)),abs(math.sin(t))); return (r*math.cos(t),r*math.sin(t))
    prof=[(0,0),at(0)]; c=45
    while c<alpha: prof.append(at(c)); c+=90
    prof.append(at(alpha))
    return [[prof[0],prof[k],prof[k+1]] for k in range(1,len(prof)-1)],[0,0,1.]
def poly_prism(poly):
    P=[]
    for i in range(len(poly)):
        p,q=poly[i],poly[(i+1)%len(poly)]; out=[q[1]-p[1],p[0]-q[0],0]; P.append((out,out[0]*p[0]+out[1]*p[1]))
    return P+[([0,0,1.],1.),([0,0,-1.],0.)]
def run(aa,ab,m,psi,N=300000):
    pa,va=wedge(aa); pb,vb=wedge(ab); f=frame(m,psi)
    turn=lambda n:[f[0][t]*n[0]+f[1][t]*n[1]+f[2][t]*n[2] for t in range(3)]
    A=[poly_prism(p) for p in pa]
    B=[[(turn(n), d-dot(n,vb)+dot(turn(n),va)) for n,d in poly_prism(p)] for p in pb]
    ins=lambda P,x: all(dot(n,x)<=d for n,d in P)
    random.seed(1); ka=kab=0
    for _ in range(N):
        x=[random.uniform(-2,2),random.uniform(-2,2),random.uniform(0,1)]
        if any(ins(P,x) for P in A):
            ka+=1
            if any(ins(P,x) for P in B): kab+=1
    return 16*ka/N,16*kab/N
print("a vol, common (MC):", run(343,330,[1,1,0],0.7), "oracle common 5.0916, kernel I 4.3178")
