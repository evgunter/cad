import struct, sys
from fractions import Fraction as F
from decimal import Decimal, getcontext
from collections import defaultdict
getcontext().prec = 70
def fx(h): return F(struct.unpack('>d', bytes.fromhex(h))[0])
def D(fr): return Decimal(fr.numerator)/Decimal(fr.denominator)
def sincos(x):
    x = D(x); s = Decimal(0); c = Decimal(0); term = Decimal(1); k = 0
    while abs(term) > Decimal(10)**-75 or k < 4:
        if k % 2 == 0: c += term if (k//2)%2==0 else -term
        else: s += term if (k//2)%2==0 else -term
        k += 1; term = term * x / k
    return s, c
s3 = defaultdict(lambda: [0]*9); s4 = defaultdict(lambda: [0]*8)
for line in open(sys.argv[1]):
    t = line.split()
    if not t or t[0] not in ('S3','S4'): continue
    k = (t[0], t[1], t[2], t[3]); e = Decimal(t[1])
    if t[0] == 'S3':
        cx,cy,ux,uy,sa,sb,t0,t1,ax,ay,bx,by,per,exc,dt = [fx(h) for h in t[4:19]]
        main, posed = int(t[19]), int(t[20])
        A = (cx-ax)*(by-ay) - (cy-ay)*(bx-ax) + sa*sb*dt
        tA = abs(D(A))/D(per)
        d = D(t1) - D(t0); sn,_ = sincos(t1-t0)
        tB = abs(D(sa*sb)*(d - sn))/D(per)
        s = s3[k]; s[0]+=1
        s[1]+= main==1; s[2]+= main==1 and tA<=e; s[3]+= main==1 and tB<=e
        s[4]+= posed==1; s[5]+= posed==1 and tB<=e
        s[6]+= main!=1 and tB>=10*e; s[7]+= posed!=1 and tB>=10*e
        err = abs(abs(D(exc))/D(per) - tB); s[8] = max(s[8], err)
    else:
        cx,cy,ux,uy,sa,sb,r,px,py = [fx(h) for h in t[4:13]]
        got = int(t[13])
        sn, cs = sincos(r)
        X = D(cx) + D(ux)*D(sa)*cs + D(-uy)*D(sb)*sn
        Y = D(cy) + D(uy)*D(sa)*cs + D(ux)*D(sb)*sn
        dist = ((X-D(px))**2 + (Y-D(py))**2).sqrt()
        s = s4[k]; s[0]+=1
        s[1]+= got==0; s[2]+= got==0 and dist>=10*e   # accepted though definitely apart
        s[3]+= got==1; s[4]+= got==1 and dist<=e      # refused though truly on
        s[5]+= got==2
        s[6]+= got==0 and dist>e
print("S3: site eps far ratio | n | main: positive, wrong vs exact-formula(<=eps), wrong vs true segment(<=eps) | re-posed: positive wrong | refuses though true>=10eps: main, re-posed | max |main margin - true|")
for k in sorted(s3, key=lambda k:(float(k[1]),float(k[2]),float(k[3]))):
    s=s3[k]; print(*k,'|',s[0],'|',s[1],s[2],s[3],'|',s[4],s[5],'|',s[6],s[7],'|','%.2e'%s[8])
print("S4: site eps far ratio | n | Zero(accept), of which true dist>=10eps, of which true dist>eps | Positive(refuse), of which true<=eps | Err")
for k in sorted(s4, key=lambda k:(float(k[1]),float(k[2]),float(k[3]))):
    s=s4[k]; print(*k,'|',s[0],'|',s[1],s[2],s[6],'|',s[3],s[4],'|',s[5])
