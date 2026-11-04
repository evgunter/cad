import json, sys
from mpmath import mp, mpf, sqrt, fabs
mp.dps = 40
def V(x): return [mpf(v) for v in x]
def dot(a,b): return a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
def sub(a,b): return [a[0]-b[0],a[1]-b[1],a[2]-b[2]]
def add(a,b): return [a[0]+b[0],a[1]+b[1],a[2]+b[2]]
def mul(a,s): return [a[0]*s,a[1]*s,a[2]*s]
def norm(a): return sqrt(dot(a,a))
def dist_fn(c):
    k=c['kind']; C=V(c['c']); A=V(c['a']); p1=mpf(c['p1']); p2=mpf(c['p2'])
    def ax(p):
        q=sub(p,C); h=dot(q,A); w=sub(q,mul(A,h)); return h,norm(w)
    if k=='plane': return lambda p: fabs(dot(sub(p,C),A))/norm(A)
    if k=='sphere': return lambda p: fabs(norm(sub(p,C))-p1)
    if k=='cylinder': return lambda p: fabs(ax(p)[1]-p1)
    if k=='cone':
        s,co=mp.sin(p1),mp.cos(p1)
        def f(p):
            h,r=ax(p); return fabs(r*co-fabs(h)*s)
        return f
    def f(p):
        h,r=ax(p); return fabs(sqrt((r-p1)**2+h*h)-p2)
    return f
def sup(c):
    D=dist_fn(c); O=V(c['o']); U=V(c['d']); t0,t1=V(c['span'])
    g=lambda t: D(add(O,mul(U,t)))
    n=1500
    ts=[t0+(t1-t0)*i/n for i in range(n+1)]
    vals=[g(t) for t in ts]
    best=max(vals)
    peaks=[i for i in range(1,n) if vals[i]>=vals[i-1] and vals[i]>=vals[i+1]]
    peaks.sort(key=lambda i:-vals[i])
    gr=(sqrt(5)-1)/2
    for i in peaks[:6]:
        a,b=ts[i-1],ts[i+1]
        for _ in range(70):
            x,y=b-gr*(b-a),a+gr*(b-a); fx,fy=g(x),g(y); best=max(best,fx,fy)
            if fx>fy: b=y
            else: a=x
    # multiscale near singular approaches: centre/apex foot, axis foot, apex-plane crossing, tips
    C=V(c['c']); A=V(c['a']); uu=dot(U,U)
    near=[dot(sub(C,O),U)/uu]
    q=sub(O,C); h0=dot(q,A); w=dot(U,A)
    if w!=0: near.append(-h0/w)
    qo=sub(q,mul(A,h0)); uo=sub(U,mul(A,w))
    if dot(uo,uo)>0: near.append(-dot(qo,uo)/dot(uo,uo))
    if c['kind']=='spindle':
        tip=sqrt(mpf(c['p2'])**2-mpf(c['p1'])**2)
        for sg in (1,-1):
            T=add(C,mul(A,sg*tip)); near.append(dot(sub(T,O),U)/uu)
    for t in near:
        for k in range(0,62):
            st=mpf(10)**-18*mpf(2)**k*(t1-t0)
            for tt in (t,t+st,t-st):
                if t0<=tt<=t1: best=max(best,g(tt))
    return best
bad=[]; stats={}
for line in open(sys.argv[1]):
    c=json.loads(line)
    s=sup(c); r=mpf(c['read']); z=mpf(c['zero']); e=mpf(c['esc'])
    reach=norm(V(c['o']))+norm(V(c['d']))*max(abs(mpf(x)) for x in c['span'])+3
    gap=s-r
    key=c['kind']; stats.setdefault(key,[0,0,0]); stats[key][0]+=1
    tags=[]
    if gap>1e-3*z+1e-14*reach: tags.append('READ_UNDER(gap=%.3e, sup=%.3e)'%(float(gap),float(s)))
    for nm in ('v64','vi'):
        v=c[nm]
        if s>1.2*e and v!='off': tags.append(f'{nm}:not-off({v}) sup={float(s):.3e}')
        if v=='off' and s<0.8*e: tags.append(f'{nm}:FALSE-OFF sup={float(s):.3e}')
        if v=='on' and s>1.2*z: tags.append(f'{nm}:on-past-zero sup={float(s):.3e}')
    if s>1.2*e: stats[key][1]+=1
    if s<0.8*z: stats[key][2]+=1
    if tags: bad.append((c['i'],c['kind'],tags))
print(stats)
for b in bad: print(b)
print(len(bad),'bad')
