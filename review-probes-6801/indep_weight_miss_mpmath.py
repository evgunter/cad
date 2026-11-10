# Independent: build two rational quadratics at held end weights (e0,e2), same control
# points (circle sub-arc of half-angle a, radius R), interior weights w*sqrt(e0e2) vs c*sqrt(e0e2);
# measure sup_s |B_w(s) - B_c(s)| directly (no Mobius, no closed form) and compare to the paid bound.
from mpmath import mp, mpf, cos, sin, sqrt, tan, pi, linspace
import random
mp.dps=50
def pt(P,W,s):
    b=[(1-s)**2*W[0],2*s*(1-s)*W[1],s**2*W[2]]; d=sum(b)
    return [sum(b[i]*P[i][k] for i in range(3))/d for k in range(2)]
def paid(R,a,w):
    c=cos(a); return R*abs(w-c)*sin(a)*sqrt(tan(a)**2+mpf(16)/27)/((1+min(w,1))*(1+min(c,1)))
def supmiss(R,a,w,e0,e2,n=400):
    c=cos(a); P=[(R*c,-R*sin(a)),(R/c,0),(R*c,R*sin(a))]
    g=sqrt(e0*e2)
    f=lambda s: (lambda p,q: sqrt((p[0]-q[0])**2+(p[1]-q[1])**2))(pt(P,[e0,w*g,e2],s),pt(P,[e0,c*g,e2],s))
    xs=[mpf(i)/n for i in range(n+1)]; v=[f(x) for x in xs]; i=max(range(n+1),key=lambda j:v[j])
    lo,hi=xs[max(i-1,0)],xs[min(i+1,n)]
    for _ in range(120):
        m1=lo+(hi-lo)*mpf('0.381966011250105'); m2=lo+(hi-lo)*mpf('0.618033988749895')
        if f(m1)<f(m2): lo=m1
        else: hi=m2
    return max(v[i],f((lo+hi)/2))
rng=random.Random(6801); lowest=(mpf(9),None)
for it in range(1500):
    k=it%6
    if k==0: a=mpf(10)**rng.uniform(-6,-1)
    elif k==1: a=pi/2-mpf(10)**rng.uniform(-9,-1)
    else: a=mpf(rng.uniform(1e-3,1.5707))
    c=cos(a)
    m=it%5
    if m==0: w=c*mpf(10)**rng.uniform(-8,0)       # w far below c, near 0
    elif m==1: w=c+mpf(10)**rng.uniform(-3,2)      # w>c, possibly >1, up to 100
    elif m==2: w=c*(1+mpf(rng.choice([-1,1]))*mpf(10)**rng.uniform(-12,-1))
    elif m==3: w=mpf(1)+mpf(10)**rng.uniform(-6,1) # w>1
    else: w=c-c*mpf(rng.uniform(0,1))
    if w<=0: continue
    R=mpf(10)**rng.uniform(-6,3)
    lam2=mpf(10)**rng.uniform(-12,12)
    e0,e2=lam2,mpf(1)
    s_=supmiss(R,a,w,e0,e2,n=200)
    if s_==0: continue
    r=paid(R,a,w)/s_
    if r<lowest[0]: lowest=(r,(float(a*360/pi),float(c),float(w),float(R),float(lam2)))
    if r<1: print("VIOLATION",mp.nstr(r,8),float(a*360/pi),float(w),float(R),float(lam2))
print("lowest paid/sup",mp.nstr(lowest[0],10),lowest[1])
