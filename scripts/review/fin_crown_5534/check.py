"""Out-of-tree checks of the fin crown review dump: exact triangle-pair
intersections, winding numbers, signed volume, admesh, manifold3d oracle."""
import json, sys, math, subprocess, os, struct, itertools
from fractions import Fraction as F
import numpy as np

def sub(a,b): return (a[0]-b[0],a[1]-b[1],a[2]-b[2])
def dot(a,b): return a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
def cross(a,b): return (a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0])

def clip(poly, n, a):
    """Keep points p of the polygon with n.(p-a) >= 0 (exact)."""
    out=[]
    m=len(poly)
    if m==0: return out
    d=[dot(n,sub(p,a)) for p in poly]
    if m==1: return poly if d[0]>=0 else []
    if m==2:
        p,q=poly; dp,dq=d
        if dp>=0 and dq>=0: return poly
        if dp<0 and dq<0: return []
        x=tuple(p[k]+dp/(dp-dq)*(q[k]-p[k]) for k in range(3))
        keep=p if dp>=0 else q
        return [keep] if keep==x else [keep,x]
    for i in range(m):
        p,q=poly[i],poly[(i+1)%m]
        dp,dq=d[i],d[(i+1)%m]
        if dp>=0: out.append(p)
        if (dp>0 and dq<0) or (dp<0 and dq>0):
            s=dp/(dp-dq)
            out.append(tuple(p[k]+s*(q[k]-p[k]) for k in range(3)))
    # dedupe consecutive
    res=[]
    for p in out:
        if not res or res[-1]!=p: res.append(p)
    if len(res)>1 and res[0]==res[-1]: res.pop()
    return res

def tri_inter(A,B):
    """A ∩ B as the vertex list of a convex set (exact)."""
    n=cross(sub(B[1],B[0]),sub(B[2],B[0]))
    poly=list(A)
    poly=clip(poly,n,B[0]); poly=clip(poly,tuple(-x for x in n),B[0])
    for i in range(3):
        a,b,c=B[i],B[(i+1)%3],B[(i+2)%3]
        m=cross(n,sub(b,a))
        if dot(m,sub(c,a))<0: m=tuple(-x for x in m)
        poly=clip(poly,m,a)
        if not poly: return []
    return poly

def in_hull(p, S):
    if len(S)==0: return False
    if len(S)==1: return p==S[0]
    if len(S)==2:
        a,b=S
        if cross(sub(p,a),sub(b,a))!=(0,0,0): return False
        t=dot(sub(p,a),sub(b,a)); return 0<=t<=dot(sub(b,a),sub(b,a))
    return True  # identical triangles flagged separately

def selfx(tris):
    T=[tuple(tuple(F(c) for c in v) for v in t) for t in tris]
    lo=np.array([[min(v[k] for v in t) for k in range(3)] for t in tris]); hi=np.array([[max(v[k] for v in t) for k in range(3)] for t in tris])
    bad=[]
    for i in range(len(T)):
        cand=np.nonzero(np.all(lo[i+1:]<=hi[i]+1e-12,axis=1)&np.all(hi[i+1:]>=lo[i]-1e-12,axis=1))[0]+i+1
        for j in cand:
            A,B=T[i],T[j]
            S=[v for v in A if v in B]
            if len(S)==3: bad.append((i,j,'duplicate')); continue
            I=tri_inter(A,B)
            extra=[p for p in I if not in_hull(p,S)]
            if extra:
                bad.append((int(i),int(j),len(S),[[float(c) for c in p] for p in extra][:3]))
    return bad

def solid_angles(P, tris):
    a=tris[:,0]-P; b=tris[:,1]-P; c=tris[:,2]-P
    la,lb,lc=[np.linalg.norm(x,axis=1) for x in (a,b,c)]
    num=np.einsum('ij,ij->i',a,np.cross(b,c))
    den=la*lb*lc+np.einsum('ij,ij->i',a,b)*lc+np.einsum('ij,ij->i',a,c)*lb+np.einsum('ij,ij->i',b,c)*la
    return 2*np.arctan2(num,den)

def windings(tris, rng, nrand=20000):
    t=np.array(tris,dtype=float)
    seen={}
    nrm=np.cross(t[:,1]-t[:,0],t[:,2]-t[:,0]); nl=np.linalg.norm(nrm,axis=1); 
    pts=[]
    for i in range(len(t)):
        if nl[i]==0: continue
        n=nrm[i]/nl[i]
        for w in [(1/3,1/3,1/3),(0.8,0.1,0.1),(0.1,0.8,0.1),(0.1,0.1,0.8),(0.98,0.01,0.01),(0.01,0.98,0.01),(0.01,0.01,0.98),(0.49,0.49,0.02),(0.02,0.49,0.49),(0.49,0.02,0.49)]:
            c=w[0]*t[i,0]+w[1]*t[i,1]+w[2]*t[i,2]
            for d in (-1e-6,-1e-8,1e-8,1e-6):
                pts.append(c+d*n)
    lo=t.reshape(-1,3).min(0); hi=t.reshape(-1,3).max(0)
    pts.extend(lo+(hi-lo)*rng.random((nrand,3)))
    for P in pts:
        w=solid_angles(P,t).sum()/(4*math.pi)
        r=round(w)
        if abs(w-r)>1e-3: r=('frac',round(w,3))
        seen[r]=seen.get(r,0)+1
    return seen

def signed_volume(tris):
    return float(sum(dot(tuple(F(c) for c in a),cross(tuple(F(c) for c in b),tuple(F(c) for c in cc))) for a,b,cc in tris)/6)

def admesh(tris, path):
    with open(path,'wb') as f:
        f.write(b'\0'*80); f.write(struct.pack('<I',len(tris)))
        for tr in tris:
            a,b,c=[np.array(v) for v in tr]; n=np.cross(b-a,c-a); l=np.linalg.norm(n); n=n/l if l else n
            f.write(struct.pack('<12fH',*n,*a,*b,*c,0))
    r=subprocess.run(['admesh','--exact','--tolerance=0',path],capture_output=True,text=True).stdout
    keys=['Number of parts','Degenerate facets','Edges fixed','Facets removed','Facets added','Facets reversed','Backwards edges','Normals fixed','Volume']
    d={}
    for line in r.splitlines():
        for k in keys:
            if line.strip().startswith(k): d[k]=line.split(':')[-1].strip()
    return d

if __name__=='__main__':
    path=sys.argv[1]; mode=sys.argv[2] if len(sys.argv)>2 else 'all'
    rng=np.random.default_rng(1)
    for line in open(path):
        r=json.loads(line)
        if r['kind'] not in mode.split(',') and mode!='all': continue
        tris=r['tris']
        if not tris: 
            print(json.dumps({k:r[k] for k in r if k!='tris'})); continue
        res={k:r[k] for k in r if k!='tris'}
        res['ntris']=len(tris)
        res['selfx']=selfx(tris)
        res['wind']=windings(tris,rng, nrand=int(os.environ.get('NRAND','4000')))
        res['meshvol']=signed_volume(tris)
        res['admesh']=admesh(tris,'/tmp/probe.stl')
        print(json.dumps(res),flush=True)
