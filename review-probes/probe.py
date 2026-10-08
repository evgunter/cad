import math, random, sys
from decimal import Decimal as D, getcontext
getcontext().prec = 60

def sub(a,b): return [a[i]-b[i] for i in range(3)]
def add(a,b): return [a[i]+b[i] for i in range(3)]
def mul(a,s): return [x*s for x in a]
def dot(a,b): return a[0]*b[0]+a[1]*b[1]+a[2]*b[2]
def cross(a,b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def norm(a): return math.sqrt(dot(a,a))
def unit(a): n=norm(a); return [x/n for x in a]

def deviation(axis, radius, center, tc, ta, R, r):
    # mirrors circle_torus.rs f64 arithmetic
    w0 = sub(center, tc); h0 = dot(w0, ta); w_perp = sub(w0, mul(ta, h0)); offset = norm(w_perp)
    off_centre = math.sqrt(h0**2 + (offset - R)**2)
    normal = mul(cross(ta, w_perp), 1.0/max(offset, R*0.5))
    normal = [x/max(offset,R*0.5) for x in cross(ta,w_perp)]
    s2 = norm(cross(axis, normal))**2
    eta = r*r*s2/(2.0*(R-r))
    X = r*r*s2 + 2.0*r*eta + eta*eta
    tilt = X/(r + math.sqrt(max(r*r - X, 0.0)))
    return off_centre + abs(radius - r) + tilt

def Dv(v): return [D(x) for x in v]
def dsqrt(x): return x.sqrt() if x>0 else D(0)
def oracle(center, axis, radius, u, tc, ta, R, r, n=2048):
    # exact-input high precision; circle points via rational param of angle
    c, a, uu = Dv(center), Dv(axis), Dv(u)
    na = dsqrt(sum(x*x for x in a)); a=[x/na for x in a]
    du = sum(uu[i]*a[i] for i in range(3)); uu=[uu[i]-a[i]*du for i in range(3)]
    nu = dsqrt(sum(x*x for x in uu)); uu=[x/nu for x in uu]
    v = [a[1]*uu[2]-a[2]*uu[1], a[2]*uu[0]-a[0]*uu[2], a[0]*uu[1]-a[1]*uu[0]]
    if globals().get('RAW'): v = Dv(cross(axis,u)); uu=Dv(u)
    T, A = Dv(tc), Dv(ta)
    nA = dsqrt(sum(x*x for x in A)); A=[x/nA for x in A]
    RR, rr, rho = D(R), D(r), D(radius)
    def dist(th):
        cs, sn = D(math.cos(th)), D(math.sin(th))
        p = [c[i] + rho*(uu[i]*cs + v[i]*sn) for i in range(3)]
        w = [p[i]-T[i] for i in range(3)]
        h = sum(w[i]*A[i] for i in range(3))
        q = [w[i]-A[i]*h for i in range(3)]
        rax = dsqrt(sum(x*x for x in q))
        return abs(dsqrt((rax-RR)**2 + h*h) - rr)
    best, bt = D(0), 0.0
    for i in range(n):
        th = 2*math.pi*i/n
        d = dist(th)
        if d > best: best, bt = d, th
    lo, hi = bt - 2*math.pi/n, bt + 2*math.pi/n
    for _ in range(40):
        m1, m2 = lo+(hi-lo)/3, hi-(hi-lo)/3
        if dist(m1) < dist(m2): lo = m1
        else: hi = m2
    return float(max(best, dist((lo+hi)/2)))

def rand_unit(rng):
    while True:
        v=[rng.uniform(-1,1) for _ in range(3)]; n=norm(v)
        if 1e-3<n<=1: return [x/n for x in v]

def rot(v, k, ang):  # rodrigues about unit k
    c,s = math.cos(ang), math.sin(ang)
    return add(add(mul(v,c), mul(cross(k,v),s)), mul(k, dot(k,v)*(1-c)))
