import sys,struct,math
from decimal import Decimal as D, getcontext
getcontext().prec=70
def f(h): return struct.unpack('>d',bytes.fromhex(h))[0]
def d(h): return D(f(h))
def sincos(x):
    s=D(0);c=D(0);term=D(1);k=0
    # cos: sum (-1)^n x^2n/(2n)!, sin: sum (-1)^n x^(2n+1)/(2n+1)!
    t=D(1);n=0
    while True:
        if n%2==0: c+= t if (n//2)%2==0 else -t
        else: s+= t if (n//2)%2==0 else -t
        n+=1; t=t*x/n
        if abs(t)<D(10)**-65 and n>10: break
    return s,c
def sub(a,b): return [x-y for x,y in zip(a,b)]
def add(a,b): return [x+y for x,y in zip(a,b)]
def sc(a,k): return [x*k for x in a]
def dot(a,b): return sum(x*y for x,y in zip(a,b))
def cross(a,b): return [a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]
U=2.0**-53
stats={}
bad_bound=0;bad_enc=0;nP=0
for line in open(sys.argv[1]):
    w=line.split()
    if w[0]=='C':
        v=[d(x) for x in w[1:]]
        A=v[0:3];N=v[3:6];Ur=v[6:9];a=v[9];b=v[10];c0=v[11];s0=v[12]
        apex=v[13:16];cax=v[16:19];half=v[19];po=v[20:23];pn=v[23:26]
        V=cross(N,Ur)
        sa,ca=sincos(half)
        wn=dot(cax,cax).sqrt(); pnn=dot(pn,pn).sqrt()
        dec=int(math.floor(math.log10(float(a))))
        continue
    tag=w[1]; t=d(w[2]); p=[d(x) for x in w[3:6]]; err=[f(x) for x in w[6:9]]
    lo=[d(w[9]),d(w[11]),d(w[13])]; hi=[d(w[10]),d(w[12]),d(w[14])]
    s,c=sincos(t)
    E=add(A,add(sc(Ur,a*(c-c0)),sc(V,b*(s-s0))))
    nP+=1
    e=[abs(x-y) for x,y in zip(p,E)]
    if any(float(e[i])>err[i] for i in range(3)): bad_bound+=1; print("BOUND", dec, tag, [float(x) for x in e], err)
    if any(not(lo[i]<=E[i]<=hi[i]) for i in range(3)):
        bad_enc+=1; i=[j for j in range(3) if not(lo[j]<=E[j]<=hi[j])][0]; print("ENC", dec, tag, float(t), "c0s0",float(c0),float(s0),"coord",i,"E-lo",float(E[i]-lo[i]),"hi-E",float(hi[i]-E[i]),"width",float(hi[i]-lo[i]),"p-E",float(p[i]-E[i]))
    width=max(float(hi[i]-lo[i]) for i in range(3))
    # miss off the surfaces, of the f64 point
    pl=abs(dot(sub(p,po),pn))/pnn
    dd=sub(p,apex); h=dot(dd,cax)/wn; r=(dot(dd,dd)-h*h).sqrt()
    co=abs(r*ca-abs(h)*sa)
    own=1+float(dot(p,p).sqrt())
    m=max(float(pl),float(co))
    k=(dec,tag); S=stats.setdefault(k,[0,0.0,0.0,0.0,0.0])
    S[0]+=1; S[1]=max(S[1],m); S[2]=max(S[2],m/(U*own)); S[3]=max(S[3],m/(U*float(a))); S[4]=max(S[4],width)
print("points",nP,"running-bound violations",bad_bound,"interval-enclosure violations",bad_enc)
print("decade tag n worst_miss_m miss/(u*(1+|p|)) miss/(u*a) worst_interval_width")
for k in sorted(stats):
    S=stats[k]; print(f"1e{k[0]} {k[1]:8} {S[0]:4} {S[1]:.2e} {S[2]:7.2f} {S[3]:.2e} {S[4]:.2e}")
# Interval box against the exact plane and cone: is 0 within each residual's range over the box?
miss_pl=0; miss_co=0; n=0; worst=[]
for line in open(sys.argv[1]):
    w=line.split()
    if w[0]=='C':
        v=[d(x) for x in w[1:]]
        apex=v[13:16];cax=v[16:19];half=v[19];po=v[20:23];pn=v[23:26]
        sa,ca=sincos(half); wn=dot(cax,cax).sqrt(); pnn=dot(pn,pn).sqrt(); a=v[9]
        continue
    lo=[d(w[9]),d(w[11]),d(w[13])]; hi=[d(w[10]),d(w[12]),d(w[14])]
    mid=[(x+y)/2 for x,y in zip(lo,hi)]; hw=[(y-x)/2 for x,y in zip(lo,hi)]
    n+=1
    r0=dot(sub(mid,po),pn)/pnn; rad=sum(abs(pn[i])*hw[i] for i in range(3))/pnn
    if abs(r0)>rad: miss_pl+=1; worst.append(("plane",w[1],float(a),float(abs(r0)-rad),float(max(hw))))
    dd=sub(mid,apex); h=dot(dd,cax)/wn; r=(dot(dd,dd)-h*h).sqrt()
    co=r*ca-abs(h)*sa
    # gradient magnitude of r cos - |h| sin is 1; the box's reach is |hw|
    radc=dot(hw,hw).sqrt()
    if abs(co)>radc: miss_co+=1; worst.append(("cone",w[1],float(a),float(abs(co)-radc),float(max(hw))))
print("interval boxes",n,"excluding the exact plane",miss_pl,"excluding the exact cone",miss_co)
for x in worst[:10]: print(x)
