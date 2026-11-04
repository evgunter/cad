# Exact Bézier extraction of a homogeneous row (Boehm in rationals), against
# the f64 cut's points and their tracked bounds (point_rounding).
import json, struct, sys
from fractions import Fraction as F
def fb(s): return F(struct.unpack('<d', struct.pack('<Q', int(s)))[0])
def fl(s): return struct.unpack('<d', struct.pack('<Q', int(s)))[0]
def insert(knots, p, P, x):
    k = max(i for i in range(len(knots)-1) if knots[i] <= x and x < knots[i+1]) if x < knots[-1] else None
    Q = []
    for i in range(len(P)+1):
        if i <= k-p: Q.append(P[i])
        elif i > k: Q.append(P[i-1])
        else:
            a = (x - knots[i])/(knots[i+p]-knots[i])
            Q.append([ (1-a)*P[i-1][c] + a*P[i][c] for c in range(4)])
    return sorted(knots+[x]), Q
worst=0; worst_case=None; n=0
for line in open(sys.argv[1]):
    d=json.loads(line); p=d['pv']; nv=d['nv']
    kv=[fb(x) for x in d['kv']]; c=[fb(x) for x in d['c']]
    cp=[[fb(x) for x in q] for q in d['cp']]; w=[fb(x) for x in d['w']]
    for i in range(2):
        row=[]
        for j in range(nv):
            q=cp[i*nv+j]; ww=w[i*nv+j]
            row.append([(q[0]-c[0])*ww,(q[1]-c[1])*ww,(q[2]-c[2])*ww,ww])
        knots=list(kv)
        vals=sorted(set(knots))
        for v in vals[1:-1]:
            m=knots.count(v)
            for _ in range(p-m): knots,row=insert(knots,p,row,v)
        # cells j: points j*p .. j*p+p
        for ci,cell in enumerate(d['cells']):
            pts=cell['pts']  # row-major in u: (pu+1)*(pv+1) with pu=1 -> index a*(pv+1)+b
            for b in range(p+1):
                got,bound=pts[i*(p+1)+b]
                H=row[ci*p+b]
                for k in range(3):
                    ex=H[k]/H[3]
                    dr=abs(F(fl(got[k]))-ex); bd=F(fl(bound[k])); n+=1
                    if bd>0:
                        r=float(dr/bd)
                        if r>worst: worst=r; worst_case=(d['case'],ci,b,k,float(dr),float(bd))
                    elif dr>0: print("ZERO BOUND drift", d['case'])
print("points",n,"worst drift/bound",worst,worst_case)
