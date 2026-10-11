import sys, re, math, itertools
def sub(a,b): return (a[0]-b[0],a[1]-b[1],a[2]-b[2])
def cross(a,b): return (a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0])
def dot(a,b): return a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
def hull_planes(pts):
    planes=[]
    for i,j,k in itertools.combinations(range(len(pts)),3):
        n=cross(sub(pts[j],pts[i]),sub(pts[k],pts[i]))
        nn=math.sqrt(dot(n,n))
        if nn<1e-9: continue
        n=(n[0]/nn,n[1]/nn,n[2]/nn); d=dot(n,pts[i])
        s=[dot(n,p)-d for p in pts]
        if all(v<=1e-9 for v in s): pass
        elif all(v>=-1e-9 for v in s): n=(-n[0],-n[1],-n[2]); d=-d
        else: continue
        if not any(abs(dot(n,m)-1)<1e-12 and abs(d-e)<1e-12 for m,e in planes):
            planes.append((n,d))
    return planes
def make_A(planes):
    def A(x):
        if abs(x)>=1: return 0.0
        h=math.sqrt(1-x*x); ylo,yhi=-h,h
        tops=[];bots=[]
        for (n,d) in planes:
            c0=d-n[0]*x
            if abs(n[2])<1e-14:
                if abs(n[1])<1e-15:
                    if c0<0: return 0.0
                elif n[1]>0: yhi=min(yhi,c0/n[1])
                else: ylo=max(ylo,c0/n[1])
                continue
            f=(c0/n[2], -n[1]/n[2])  # z = a + b y
            (tops if n[2]>0 else bots).append(f)
        if yhi<=ylo: return 0.0
        cand={ylo,yhi}
        for grp in (tops,bots):
            for f,g in itertools.combinations(grp,2):
                if f[1]!=g[1]:
                    y=(g[0]-f[0])/(f[1]-g[1])
                    if ylo<y<yhi: cand.add(y)
        ys=sorted(cand)
        gv=lambda y: min(a+b*y for a,b in tops)-max(a+b*y for a,b in bots)
        tot=0.0
        for y0,y1 in zip(ys,ys[1:]):
            # g linear on [y0,y1]; evaluate at interior-anchored ends via midpoint-active pieces
            ym=(y0+y1)/2
            ta=min(tops,key=lambda f:f[0]+f[1]*ym); ba=max(bots,key=lambda f:f[0]+f[1]*ym)
            g0=ta[0]+ta[1]*y0-(ba[0]+ba[1]*y0); g1=ta[0]+ta[1]*y1-(ba[0]+ba[1]*y1)
            L=y1-y0
            if g0>=0 and g1>=0: tot+=(g0+g1)/2*L
            elif g0>0: tot+=g0*(g0/(g0-g1)*L)/2
            elif g1>0: tot+=g1*(g1/(g1-g0)*L)/2
        return tot
    return A
# Gauss-Legendre 10
GL=[(-0.9739065285171717,0.0666713443086881),(-0.8650633666889845,0.1494513491505806),(-0.6794095682990244,0.2190863625159820),(-0.4333953941292472,0.2692667193099963),(-0.1488743389816312,0.2955242247147529)]
GL=GL+[(-x,w) for x,w in reversed(GL)]
def gl(f,a,b):
    m=(a+b)/2;r=(b-a)/2
    return r*sum(w*f(m+r*x) for x,w in GL)
def adapt(f,a,b,whole,tol,depth=0):
    m=(a+b)/2; l=gl(f,a,m); r=gl(f,m,b)
    if abs(l+r-whole)<tol or depth>40: return l+r
    return adapt(f,a,m,l,tol/1.6,depth+1)+adapt(f,m,b,r,tol/1.6,depth+1)
worst=0
for line in open(sys.argv[1]):
    if not line.startswith('PROBE'): continue
    vi=float(re.search(r' vi=(\S+)',line).group(1))
    pts=[tuple(map(float,p.split(','))) for p in re.search(r'pts=(\S+)',line).group(1).split(';')]
    planes=hull_planes(pts)+[((0,0,1.0),8.0),((0,0,-1.0),8.0)]
    A=make_A(planes)
    f=lambda t: A(math.sin(t))*math.cos(t)
    # pre-split at many points for robustness
    edges=[-math.pi/2+i*math.pi/64 for i in range(65)]
    v=sum(adapt(f,a,b,gl(f,a,b),1e-13) for a,b in zip(edges,edges[1:]))
    err=abs(v-vi); worst=max(worst,err)
    tag=line.split(' nverts')[0]
    print(f"{err:.3e} faces={len(planes)-2} vi={vi:.12f} mine={v:.12f} {tag}")
print("WORST",worst)
