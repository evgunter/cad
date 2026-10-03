# Independent check of PR #3978's touch spreads against the true section,
# for true margins mu in [0, zero] and the bound's m = escalate = 10*zero.
from mpmath import mp, mpf, sqrt, cos, pi
mp.dps=50
import random
random.seed(1)
zero=mpf('1e-9'); m=10*zero
worst={}
def rec(k,ratio,info):
    if ratio>worst.get(k,(0,None))[0]: worst[k]=(ratio,info)
for it in range(4000):
    mu=zero*mpf(random.random())
    s=mpf(10)**random.uniform(-3,3)
    # sphere-sphere outside / inside, either labelling
    r1=s*mpf(random.uniform(0.01,2)); r2=s*mpf(random.uniform(0.01,2))
    for kind in ('out','in'):
        if kind=='out': dd=r1+r2-mu
        else:
            dd=abs(r1-r2)+mu
        x=(dd**2+r1**2-r2**2)/(2*dd)
        rad2=r1**2-x**2
        if rad2<0: continue
        spread=sqrt(r1*r2*m*2/dd)+m
        rec('ss-'+kind+('-Fsmall' if r1<r2 else '-Fbig'), sqrt(rad2)/spread,(float(r1),float(r2),float(dd)))
    # sphere-plane
    rho=s*mpf(random.uniform(0.01,2)); rad=sqrt(mu*(2*rho-mu))
    rec('sp', rad/(sqrt(m*(2*rho+m))+m),float(rho))
    # sphere-cylinder outside/inside: sample section points
    rc=s*mpf(random.uniform(0.01,2)); rho=s*mpf(random.uniform(0.01,2))
    for kind in ('out','in'):
        if kind=='out': e=rc+rho-mu
        else:
            e=rc-rho+mu
            if e<=0: continue
        S=rho**2-(e-rc)**2
        spread=sqrt(m*(2*rho+m)*(1+rc/e))+m
        best=0
        for j in range(200):
            th=2*pi*j/200
            t2=S-2*e*rc*(1-cos(th))
            if t2<0: continue
            d=sqrt(t2+2*rc**2*(1-cos(th)))
            best=max(best,d)
        rec('sc-'+kind,best/spread,(float(rc),float(rho),float(e)))
for k,v in sorted(worst.items()): print(k, float(v[0]), v[1])
