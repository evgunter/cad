import numpy as np, math
yc, rc = 2.45, 0.1
def prim(y, b):
    y = min(y, b)
    return 0.5*(y*math.sqrt(max(b*b-y*y,0.0)) + b*b*math.asin(y/b))
def inner(z):
    w = math.sqrt(max(0.25-z*z,0.0)); b = 2.0+w
    h = math.sqrt(max(rc*rc-z*z,0.0))
    lo, hi = yc-h, yc+h
    if lo >= b: return 0.0
    return 2.0*(prim(hi,b)-prim(lo,b))
# kink: yc + h(z) = b(z)
f = lambda z: yc+math.sqrt(rc*rc-z*z) - 2.0 - math.sqrt(0.25-z*z)
a, c = 0.0, 0.0999999
for _ in range(200):
    m=0.5*(a+c)
    if f(a)*f(m) <= 0: c=m
    else: a=m
zk = 0.5*(a+c)
xg, wg = np.polynomial.legendre.leggauss(200)
def gl(fn, lo, hi, sine=False):
    s=0.0
    for x, w in zip(xg, wg):
        if sine:
            # map theta in [lo,hi] to z = rc*sin(theta)
            th = 0.5*(hi-lo)*x+0.5*(hi+lo)
            s += w*fn(rc*math.sin(th))*rc*math.cos(th)*0.5*(hi-lo)
        else:
            t=0.5*(hi-lo)*x+0.5*(hi+lo); s+=w*fn(t)*0.5*(hi-lo)
    return s
thk = math.asin(zk/rc)
# by symmetry 2x over z>=0; split [0, zk] and [zk, 0.1] in theta
parts=[]
for n in [2,4,8]:
    tot=0.0
    edges=[0.0]+[thk*i/n for i in range(1,n)]+[thk]+[thk+(math.pi/2-thk)*i/n for i in range(1,n)]+[math.pi/2]
    for lo,hi in zip(edges[:-1],edges[1:]):
        tot+=gl(inner, lo, hi, sine=True)
    parts.append(2*tot)
print("kink z", zk, "int", parts)
vd = 2*math.pi**2*2*0.25
print("sub", [vd-p for p in parts])
