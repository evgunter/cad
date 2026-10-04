"""Exact-rational oracle for the settled-residue result bodies (delta-2,
PR 3977). Reads RES| dumps; for every body computes, in Fractions of the
stored f64 values:
  fan    : sum over faces of (anchor . A_face)/3, anchor = first loop point
           (the lane's re-derivation, from vertex points)
  fanlo/hi: the same, the anchor ranging over every loop point of each face
  proj   : sum over faces of ((o.n)(n.A)/(n.n))/3 (main's planar formula)
  delta  : max distance of a loop point from its face's stored plane
and the backstop margin of each op under each model."""
import sys
from fractions import Fraction as F

def vec(s): return tuple(F(float(x)) for x in s.split(','))
def sub(a,b): return tuple(x-y for x,y in zip(a,b))
def dot(a,b): return sum(x*y for x,y in zip(a,b))
def cross(a,b): return (a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0])
def add(a,b): return tuple(x+y for x,y in zip(a,b))

def loop_va(pts, ref):
    acc=(F(0),)*3
    for i in range(len(pts)):
        a=sub(pts[i],ref); b=sub(pts[(i+1)%len(pts)],ref)
        acc=add(acc,cross(a,b))
    return tuple(x/2 for x in acc)

def face_terms(o,n,loops):
    va=(F(0),)*3
    for l in loops: va=add(va,loop_va(l,l[0]))
    allp=[p for l in loops for p in l]
    fan=[dot(a,va) for a in allp]
    proj=dot(o,n)*dot(n,va)/dot(n,n)
    nn=dot(n,n)
    import math
    delta=max(abs(dot(sub(p,o),n)) for p in allp)/F(math.sqrt(float(nn)))
    area=math.sqrt(float(dot(va,va)))
    return fan[0], min(fan), max(fan), proj, float(delta), area

bodies={}; verdict={}; vols={}; hdr=None
for line in open(sys.argv[1]):
    f=line.rstrip('\n').split('|')
    if len(f)>1 and f[1].startswith('eps='): hdr=line.strip(); continue
    if 'FACE' in f:
        i=f.index('FACE'); tag='|'.join(f[1:i])
        if f[i+2]=='nonplanar': continue
        loops=[[vec(p) for p in l.split(';')] for l in f[i+4].split('#')]
        bodies.setdefault(tag,[]).append((vec(f[i+2]),vec(f[i+3]),loops))
    elif 'VERDICT' in f:
        i=f.index('VERDICT'); verdict['|'.join(f[1:i])]=f[i+1]
    elif 'OPS' in f:
        i=f.index('OPS'); vols['|'.join(f[1:i])]=(F(float(f[i+1][3:])),F(float(f[i+2][3:])))

def measure(tag):
    t=[face_terms(*fc) for fc in bodies[tag]]
    s=lambda k: sum(x[k] for x in t)/3
    return dict(fan=s(0),fanlo=s(1),fanhi=s(2),proj=s(3),delta=max(x[4] for x in t),
                dA=sum(x[4]*x[5] for x in t))

print(hdr)
blk=measure('block')
for rt in sorted({k.rsplit('|',1)[0] for k in verdict}):
    tool=measure(rt+'|tool')
    for op,bound,sign in [('union','A+B',-1),('intersect','B',-1),('AminusB','A-B',+1)]:
        k=rt+'|'+op
        if k not in bodies: continue
        r=measure(k)
        out=[]
        for model in ['fan','fanlo','fanhi','proj']:
            b={'A+B':blk[model]+tool[model],'B':tool[model],'A-B':blk[model]-tool[model]}[bound]
            if model in ('fanlo','fanhi'):
                b={'A+B':blk['fan']+tool['fan'],'B':tool['fan'],'A-B':blk['fan']-tool['fan']}[bound]
            m=sign*(r[model]-b)   # >= 0 holds; < 0 crosses
            out.append(f"{model}={float(m):+.2e}")
        print(f"{k:28s} verdict={verdict[k]:8s} margin(>=0 holds): {' '.join(out)}  maxdelta={r['delta']:.1e} sum(delta*A)={r['dA']:.1e}")
