import sys, math
src = open(sys.argv[1]).read()
out = sys.argv[2]
R = 0.5
def arcs(angles):
    P=[]; W=[]
    on=lambda t:(R*math.cos(t),R*math.sin(t))
    P.append(on(angles[0])); W.append(1.0)
    for a,b in zip(angles,angles[1:]):
        c=math.cos((b-a)/2); m=(a+b)/2
        P.append((R/c*math.cos(m),R/c*math.sin(m))); W.append(c)
        P.append(on(b)); W.append(1.0)
    return P,W
def quad(angles):
    P,W=arcs(angles); n=len(angles)-1
    mult=[3]+[2]*(n-1)+[3]; kv=list(range(n+1))
    return 2,P,W,mult,kv
def cubic(angles):
    P,W=arcs(angles); n=len(angles)-1
    Q=[];V=[]
    for j in range(n):
        h=[(P[2*j+i][0]*W[2*j+i],P[2*j+i][1]*W[2*j+i],W[2*j+i]) for i in range(3)]
        e=[h[0],tuple((h[0][k]+2*h[1][k])/3 for k in range(3)),tuple((2*h[1][k]+h[2][k])/3 for k in range(3)),h[2]]
        if j>0: e=e[1:]
        for x in e: Q.append((x[0]/x[2],x[1]/x[2])); V.append(x[2])
    mult=[4]+[3]*(n-1)+[4]; kv=list(range(n+1))
    return 3,Q,V,mult,kv
def single(angles):
    P,W=arcs(angles); n=len(angles)-1
    h=[(P[i][0]*W[i],P[i][1]*W[i],W[i]) for i in range(3)]
    q1=tuple((h[0][k]+h[1][k])/2 for k in range(3)); q2=tuple((h[1][k]+h[2][k])/2 for k in range(3))
    P2=[P[0],(q1[0]/q1[2],q1[1]/q1[2]),(q2[0]/q2[2],q2[1]/q2[2])]+P[2:]
    W2=[W[0],q1[2],q2[2]]+W[2:]
    mult=[3,1]+[2]*(n-1)+[3]; kv=[0,0.5]+list(range(1,n+1))
    return 2,P2,W2,mult,kv
mode=sys.argv[3]
d=math.radians
ang={'q3':[0,d(120),d(240),d(360)],'uneven':[0,d(170),d(265),d(360)],'c4':[0,d(90),d(180),d(270),d(360)],'single':[0,d(120),d(240),d(360)]}[ {'quad':'q3','uneven':'uneven','cubic':'c4','single':'single'}[mode] ]
f={'quad':quad,'uneven':quad,'cubic':cubic,'single':single}[mode]
def ent(z, rid):
    deg,P,W,mult,kv=f(ang)
    pts=[]; extra=[]
    for i,(x,y) in enumerate(P):
        pid=rid*100+i
        extra.append(f"#{pid} = CARTESIAN_POINT('',({x!r},{y!r},{z!r}));")
        pts.append(f"#{pid}")
    e=f"( BOUNDED_CURVE() B_SPLINE_CURVE({deg},({','.join(pts)}),.UNSPECIFIED.,.T.,.F.) B_SPLINE_CURVE_WITH_KNOTS(({','.join(map(str,mult))}),({','.join(repr(float(k)) for k in kv)}),.UNSPECIFIED.) CURVE() GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_CURVE(({','.join(repr(w) for w in W)})) REPRESENTATION_ITEM('') )"
    return e, extra
lines=src.split('\n'); res=[]
for L in lines:
    if L.startswith('#24 = CIRCLE'):
        e,x=ent(1.0,9024); res+=x; res.append('#24 = '+e+';')
    elif L.startswith('#39 = CIRCLE'):
        e,x=ent(0.0,9039); res+=x; res.append('#39 = '+e+';')
    else: res.append(L)
open(out,'w').write('\n'.join(res))
