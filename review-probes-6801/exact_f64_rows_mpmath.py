import math
from mpmath import mp, mpf, sqrt, cos, sin, tan, atan
mp.dps=50
def lever(h,w):
    a=h/2.0; c=math.cos(a)
    return math.sin(a)*math.sqrt(math.tan(a)**2+16.0/27.0)/((1.0+min(w,1.0))*(1.0+min(c,1.0)))
def column(r,h,ends,normal):
    c=math.cos(h/2.0); pts=[(r,0.0)]; w=[ends[0]]
    for k in range(len(normal)):
        base=h*k; mid=base+h/2.0
        pts.append((r/c*math.cos(mid), r/c*math.sin(mid))); w.append(normal[k]*math.sqrt(ends[k]*ends[k+1]))
        pts.append((r*math.cos(base+h), r*math.sin(base+h))); w.append(ends[k+1])
    return pts,w
def rq(p,wt,s):
    b=[(1-s)**2*wt[0],2*s*(1-s)*wt[1],s**2*wt[2]]; d=sum(b)
    return [sum(b[i]*p[i][k] for i in range(3))/d for k in range(2)]
def supf(f,lo,hi,n=400):
    xs=[lo+(hi-lo)*i/n for i in range(n+1)]; v=[f(x) for x in xs]; i=max(range(n+1),key=lambda j:v[j])
    a,b=xs[max(i-1,0)],xs[min(i+1,n)]
    for _ in range(100):
        m1,m2=a+(b-a)*mpf('0.381966011250105'),a+(b-a)*mpf('0.618033988749895')
        if f(m1)<f(m2): a=m1
        else: b=m2
    return max(v[i],f((a+b)/2))
def exact_miss(r,h,ends,normal):
    pts,wt=column(r,h,ends,normal); P=[(mpf(x),mpf(y)) for x,y in pts]; W=[mpf(x) for x in wt]
    R=mpf(r); H=mpf(h); worst=0
    for k in range(len(normal)):
        p,w=P[2*k:2*k+3],W[2*k:2*k+3]
        lam=sqrt(w[0]/w[2])
        def miss(t):
            phi=t-(k+mpf(1)/2)*H
            s=mpf(1)/2+tan(phi/2)/(2*tan(H/4))
            s=lam*s/(lam*s+1-s)
            b=rq(p,w,s); return sqrt((b[0]-R*cos(t))**2+(b[1]-R*sin(t))**2)
        worst=max(worst,supf(miss,k*H,(k+1)*H))
    return worst
z=1e-9
for deg,r,side,ends,env in [(179.9,1e3,1.0,[1.0,1.0],4.999399766115475e-10),(178.0,1e3,1.0,[1.0,1.0],4.999477388795411e-10),
                            (179.5,1e3,-1.0,[1.0,1.0],4.999392481432831e-10),(179.9,1e3,1.0,[1e-3,1.0],4.999399766115475e-10),
                            (175.0,1e3,1.0,[1.0,1.0],5.000175930210703e-10),(179.9,1.0,-1.0,[1.0,1.0],5.000000041356262e-10)]:
    h=math.radians(deg); c=math.cos(h/2.0)
    d=side*min(0.5*z/(r*lever(h,c)),0.5*c)
    m=exact_miss(r,h,ends,[c+d])
    print(deg,r,side,ends,"env",env,"exact miss",mp.nstr(m,12),"env-miss",mp.nstr(mpf(env)-m,4))
