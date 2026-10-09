import math, itertools, sys
from fractions import Fraction as F
import mpmath as mp
mp.mp.dps=60
def dot(a,b): return a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
def cross(a,b): return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
def unit(a): s=1.0/math.sqrt(dot(a,a)); return [c*s for c in a]
def add(a,b): return [x+y for x,y in zip(a,b)]
def sc(a,k): return [c*k for c in a]
def basis(m):
    m=unit(m); seed=[0,0,1.0] if abs(m[2])<0.9 else [1.0,0,0]
    u=unit(cross(seed,m)); return u,cross(m,u)
def pose(edge,a,d):
    e=unit(edge); p1,p2=basis(e)
    al=2*math.pi*(a+0.25)/16
    return unit(add(add(sc(p1,math.cos(al)),sc(p2,math.sin(al))),sc(e,d)))
def lump(piece, h, v, m, zlo=0.0, out=False):
    # half-spaces n.x <= c, exact rationals
    u,w=basis(m)
    Q=lambda a:[F(x) for x in a]
    hs=[]
    n=len(piece)
    for i in range(n):
        (x0,y0),(x1,y1)=piece[i],piece[(i+1)%n]
        nn=[F(y1-y0),F(x0-x1),F(0)]   # outward for CCW
        hs.append((nn,dot(nn,[F(x0),F(y0),F(0)])))
    hs.append(([F(0),F(0),F(1)],F(zlo+h))); hs.append(([F(0),F(0),F(-1)],F(-zlo)))
    V=Q(v); M=Q(m); U=Q(u); W=Q(w)
    cuts=[(M,0)] if out else [(sc(M,-1),0),(M,4),(U,2),(sc(U,-1),2),(W,2),(sc(W,-1),2)]
    for nn,off in cuts:
        hs.append((nn,dot(nn,V)+off))
    def solve(A,b):
        # 3x3 Cramer
        det=lambda R:(R[0][0]*(R[1][1]*R[2][2]-R[1][2]*R[2][1])-R[0][1]*(R[1][0]*R[2][2]-R[1][2]*R[2][0])+R[0][2]*(R[1][0]*R[2][1]-R[1][1]*R[2][0]))
        D=det(A)
        if D==0: return None
        out=[]
        for k in range(3):
            R=[row[:] for row in A]
            for r in range(3): R[r][k]=b[r]
            out.append(det(R)/D)
        return out
    pts=set()
    for i,j,k in itertools.combinations(range(len(hs)),3):
        p=solve([hs[i][0],hs[j][0],hs[k][0]],[hs[i][1],hs[j][1],hs[k][1]])
        if p and all(dot(nn,p)<=c for nn,c in hs): pts.add(tuple(p))
    pts=list(pts)
    if len(pts)<4: return None
    c=[sum(p[i] for p in pts)/len(pts) for i in range(3)]
    vol=F(0); area=mp.mpf(0)
    for nn,cc in hs:
        f=[p for p in pts if dot(nn,p)==cc]
        if len(f)<3: continue
        fc=[sum(p[i] for p in f)/len(f) for i in range(3)]
        nm=[mp.mpf(x.numerator)/x.denominator for x in nn]
        e1=[mp.mpf((f[0][i]-fc[i]).numerator)/(f[0][i]-fc[i]).denominator for i in range(3)]
        if all(x==0 for x in e1): e1=[mp.mpf((f[1][i]-fc[i]).numerator)/(f[1][i]-fc[i]).denominator for i in range(3)]
        e2=cross(nm,e1)
        def ang(p):
            d=[mp.mpf((p[i]-fc[i]).numerator)/(p[i]-fc[i]).denominator for i in range(3)]
            return mp.atan2(dot(d,e2),dot(d,e1))
        f.sort(key=ang)
        for t in range(1,len(f)-1):
            a,b,cc2=f[0],f[t],f[t+1]
            ab=[b[i]-a[i] for i in range(3)]; ac=[cc2[i]-a[i] for i in range(3)]
            cr=cross(ab,ac)
            vol+=dot([a[i]-c[i] for i in range(3)],cr)/6
            area+=mp.sqrt(mp.mpf(dot(cr,cr).numerator)/dot(cr,cr).denominator)/2
    vol=abs(vol)
    return mp.mpf(vol.numerator)/vol.denominator, area
if __name__=="__main__":
    d=float(sys.argv[1]) if len(sys.argv)>1 else 1e-8
    m=pose([2.0,1.0,0.0],3,d)
    r=lump([(2,0),(4,0),(4,2),(2,1)],1.0,[2.0,1.0,1.0],m)
    if r: vol,ar=r; print("d",d,"V",mp.nstr(vol,12),"A",mp.nstr(ar,12),"V/A",mp.nstr(vol/ar,15))
    else: print("empty")
