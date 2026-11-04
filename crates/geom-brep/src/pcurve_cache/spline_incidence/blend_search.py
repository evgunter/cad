import random, sys
from fractions import Fraction as F
u = 2.0**-53
random.seed(int(sys.argv[1]) if len(sys.argv)>1 else 1)
worst=0; arg=None
for _ in range(600000):
    lo=random.random()*0.5; hi=lo+random.random()*0.5+1e-3; x0=lo+(hi-lo)*random.random()
    t=(x0-lo)/(hi-lo); T=(F(x0)-F(lo))/(F(hi)-F(lo))
    # x and y of opposite sign, near-cancelling result to make |r| small
    a=random.uniform(-1,1); b=random.uniform(-1,1)
    r=a+(b-a)*t; R=F(a)+(F(b)-F(a))*T
    err=abs(F(r)-R); span=abs(b-a)
    bound=u*(span*t+abs(r))+span*3*u*t
    if bound>0 and float(err/F(bound))>worst: worst=float(err/F(bound)); arg=(a,b,lo,hi,x0)
print("worst single-blend error/bound", worst, arg)
