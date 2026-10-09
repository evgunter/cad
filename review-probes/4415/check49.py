import sys, re, math
sys.path.insert(0,'/tmp/claude-0/probe')
from lump import *
from fractions import Fraction as F
L=[(0.,0.),(2.,0.),(2.,1.),(1.,1.),(1.,2.),(0.,2.)]
LP=[[(0.,0.),(2.,0.),(2.,1.),(0.,1.)],[(0.,1.),(1.,1.),(1.,2.),(0.,2.)]]
mir=lambda p: [(-x,y) for x,y in p][::-1]
def wedge(alpha):
    def at(t):
        t=math.radians(t); r=2.0/max(abs(math.cos(t)),abs(math.sin(t))); return (r*math.cos(t),r*math.sin(t))
    prof=[(0.,0.),at(0.0)]; c=45.0
    while c<alpha: prof.append(at(c)); c+=90.0
    prof.append(at(alpha))
    return prof,[[prof[0],prof[k],prof[k+1]] for k in range(1,len(prof)-1)],[0.,0.,1.]
P={'Ltop':(L,LP,[1.,1.,1.]),'Lbot':(L,LP,[1.,1.,0.]),'Lmirror':(mir(L),[mir(p) for p in LP],[-1.,1.,1.]),
 'notch307':([(0,0),(4,0),(4,2),(2,1),(0,2)],[[(0,0),(2,0),(2,1),(0,2)],[(2,0),(4,0),(4,2),(2,1)]],[2.,1.,1.]),
 'shallow200':([(0,0),(4,0),(4,1),(2,.6),(0,1)],[[(0,0),(2,0),(2,.6),(0,1)],[(2,0),(4,0),(4,1),(2,.6)]],[2.,.6,1.]),
 'convex':(L,LP,[2.,0.,1.]),
 'vee300':([(0,0),(4,0),(4,4),(2,.5),(0,4)],[[(0,0),(2,0),(2,.5),(0,4)],[(2,0),(4,0),(4,4),(2,.5)]],[2.,.5,1.]),
 'asym':([(0,0),(4,0),(4,3),(1,1),(0,1.5)],[[(0,0),(1,0),(1,1),(0,1.5)],[(1,0),(4,0),(4,3),(1,1)]],[1.,1.,1.]),
 'w345':wedge(345.0),'w60':wedge(60.0)}
def edges(prof,v):
    n=len(prof); i=[k for k,(x,y) in enumerate(prof) if abs(x-v[0])<1e-12 and abs(y-v[1])<1e-12][0]
    p,a,b=prof[i],prof[(i+n-1)%n],prof[(i+1)%n]
    vz=-1.0 if v[2]>0.5 else 1.0
    return [[a[0]-p[0],a[1]-p[1],0.0],[b[0]-p[0],b[1]-p[1],0.0],[0.0,0.0,vz]]
def pose2(e,a,d):
    e=unit(e); p1,p2=basis(e); al=2*math.pi*(a+0.25)/16
    return unit(add(add(sc(p1,math.cos(al)),sc(p2,math.sin(al))),sc(e,d)))
for line in open(sys.argv[1]):
    m_=re.match(r'(\S+) nt e(\d) a(\d+) d(\S+) (\w\w) (\w): ERR Escalated .*?lo: ([-\d.e]+), hi: ([-\d.e]+)',line)
    if not m_: continue
    name,e,a,d,order,op,lo,hi=m_.groups(); lo,hi=float(lo),float(hi)
    prof,pieces,v=P[name]
    m=pose2(edges(prof,v)[int(e)],int(a),float(d))
    vals=[]
    for pc in pieces:
        if op=='I':
            r=lump(pc,1.0,v,m)
        else:
            # prism piece outside the cube's near face: m.x <= m.v
            r=lump(pc,1.0,v,m,out=True)
        if r: vals.append(float(r[0]/r[1]))
    sl=1e-9*abs(lo); hit=[x for x in vals if lo-sl<=abs(x)*(1 if lo>0 else -1)<=hi+sl]
    print(f"{name} e{e} a{a} d{d} {order} {op} cert=[{lo:.10e},{hi:.10e}] exact={['%.10e'%x for x in vals]} {'IN' if hit else 'MISS'}")
