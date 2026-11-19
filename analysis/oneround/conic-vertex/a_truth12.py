import struct, sys
from fractions import Fraction as F
from decimal import Decimal, getcontext
from collections import defaultdict
getcontext().prec = 80
def fx(h): return F(struct.unpack('>d', bytes.fromhex(h))[0])
def D(fr): return Decimal(fr.numerator)/Decimal(fr.denominator)
def sq(fr): return D(fr).sqrt()
stat = defaultdict(lambda: [0,0,0,0,0,0,0])  # n, main1, mainwrong, vn1, vnwrong, vx1, vxwrong
gap = defaultdict(lambda:[0,0,0])
for line in open(sys.argv[1]):
    t = line.split()
    site, eps, far, ratio = t[0], t[1], t[2], t[3]
    x = [fx(h) for h in t[4:-3]]
    verd = [int(v) for v in t[-3:]]
    c = x[0:3]; u = x[3:6]; a = x[6]; b = x[7]
    v = [-u[1], u[0], F(0)]
    dot = lambda p,q: sum(i*j for i,j in zip(p,q))
    if site == 'S1':
        p = x[8:11]; q = x[11:14]
        al = [q[i]-p[i] for i in range(3)]
        m = [-al[1], al[0], F(0)]
        num = abs(D(dot(m,[c[i]-p[i] for i in range(3)]))) - sq((a*dot(m,u))**2 + (b*dot(m,v))**2)
        truth = num / sq(dot(m,m))
    else:
        hc = x[8:11]; hu = x[11:14]; r = x[14]
        hv = [-hu[1], hu[0], F(0)]
        w = [hc[i]-c[i] for i in range(3)]
        cx, cy = dot(w,u)/a, dot(w,v)/b
        ax, ay = r*dot(hu,u)/a, r*dot(hu,v)/b
        bx, by = r*dot(hv,u)/a, r*dot(hv,v)/b
        aa, bb, ab = ax*ax+ay*ay, bx*bx+by*by, ax*bx+ay*by
        spread = sq((aa-bb)**2 + 4*ab*ab)
        sig = ((D(aa+bb)+spread)/2).sqrt()
        cen = sq(cx*cx+cy*cy)
        lever = D(min(a,b))
        truth = max((1-cen-sig)*lever, (cen-sig-1)*lever)
    e = Decimal(eps)
    k = (site, eps, far, ratio)
    s = stat[k]; s[0]+=1
    for i in range(3):
        if verd[i]:
            s[1+2*i]+=1
            if truth <= e: s[2+2*i]+=1
            if truth < 10*e: gap[k][i]+=1
print("site eps far ratio | n | main: clear wrong(truth<=eps) ingap(<10eps) | vertex-naive: clear wrong ingap | vertex-exact: clear wrong ingap")
for k in sorted(stat, key=lambda k:(k[0],float(k[1]),float(k[2]),float(k[3]))):
    s=stat[k]; g=gap[k]
    print(*k, '|', s[0], '|', s[1], s[2], g[0], '|', s[3], s[4], g[1], '|', s[5], s[6], g[2])
