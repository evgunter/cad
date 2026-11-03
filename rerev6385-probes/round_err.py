import random, math
from fractions import Fraction as F
from decimal import Decimal, getcontext
getcontext().prec=60
def dsqrt(fr): return Decimal(fr.numerator)/Decimal(fr.denominator)
def exact_off(q,c,a_raw,r,cyl):
    q=[F(x) for x in q]; c=[F(x) for x in c]
    v=[q[i]-c[i] for i in range(3)]
    if cyl:
        # exact unit axis from the f64 normalized axis Round holds (as Round::cylinder stores a.normalize())
        a=[F(x) for x in a_raw]
        av=sum(a[i]*v[i] for i in range(3)); aa=sum(x*x for x in a)
        v=[v[i]-a[i]*av/aa for i in range(3)]
    n2=sum(x*x for x in v)
    return (dsqrt(n2).sqrt()) - Decimal(r)
def f64_off(q,c,a,r,cyl):
    v=[q[i]-c[i] for i in range(3)]
    if cyl:
        av=a[0]*v[0]+a[1]*v[1]+a[2]*v[2]
        v=[v[i]-a[i]*av for i in range(3)]
    return math.sqrt(v[0]*v[0]+v[1]*v[1]+v[2]*v[2]) - r
random.seed(1)
worst={}
for shift in [0,10,100,1000,1e4]:
  for r in [1e-3,1e-2,1,1e2,1e3]:
    for cyl in [True,False]:
      w=0
      for _ in range(300):
        a=[random.uniform(-1,1) for _ in range(3)]; na=math.sqrt(sum(x*x for x in a)); a=[x/na for x in a]
        # origin up to 1e4 along the axis from the point too
        base=[shift*random.uniform(-1,1) for _ in range(3)]
        # point on the carrier: pick a unit vector perpendicular to a
        t=[random.uniform(-1,1) for _ in range(3)]
        if cyl:
            ta=sum(t[i]*a[i] for i in range(3)); t=[t[i]-a[i]*ta for i in range(3)]
        nt=math.sqrt(sum(x*x for x in t)); t=[x/nt for x in t]
        along = random.uniform(-1,1)*shift if cyl else 0.0
        c=base
        q=[c[i]+t[i]*r+ (a[i]*along) for i in range(3)]
        e=abs(F(f64_off(q,c,a,r,cyl)) - F(str(exact_off(q,c,a,r,cyl))))
        w=max(w,float(e))
      worst[(shift,r,cyl)]=w
for k,v in worst.items(): print(k, "%.2e"%v)
