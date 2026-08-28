import math,random
PI=math.pi
def rot(v,a):
    x,z=v; return (x*math.cos(a)-z*math.sin(a), x*math.sin(a)+z*math.cos(a))
class T:
    def __init__(s,p,t): s.p=p; s.t=t
    def arc(s,ring,turn):
        n = (-s.t[1], s.t[0]) if turn>=0 else (s.t[1],-s.t[0])
        c = (s.p[0]+ring*n[0], s.p[1]+ring*n[1])
        radial = ((s.p[0]-c[0])/ring, (s.p[1]-c[1])/ring)
        adv = rot(radial,turn)
        return dict(center=c,ring=ring), T((c[0]+ring*adv[0], c[1]+ring*adv[1]), rot(s.t,turn))
def run(t1,t2,r1,r2):
    root=T((0.0,0.0),(0.0,1.0))
    a,f = root.arc(r1,t1*PI/180.0)
    u,fl_ = f.arc(r2,t2*PI/180.0)
    C=u['center']; P=fl_.p
    w0=P[0]-C[0]; w2=P[1]-C[1]
    return math.sqrt(w0*w0+w2*w2)-r2
print("authored (22,170,5.0,1.1):", repr(run(22.0,170.0,5.0,1.1)))
# sensitivity: perturb the turn of the second arc
exact=0; n=0
for k in range(-25,26):
    d=run(22.0,170.0+k*0.001,5.0,1.1); n+=1
    if d==0.0: exact+=1
print("perturbing turn2 by +-0.025deg in 0.001 steps: exact-zero %d/%d"%(exact,n))
exact=0;n=0
for k in range(-25,26):
    d=run(22.0,170.0,5.0,1.1+k*1e-6); n+=1
    if d==0.0: exact+=1
print("perturbing ring2 by +-2.5e-5: exact-zero %d/%d"%(exact,n))
exact=0;n=0
random.seed(1)
for _ in range(2000):
    d=run(random.uniform(5,40),random.uniform(100,200),random.uniform(1,8),random.uniform(0.5,3)); n+=1
    if d==0.0: exact+=1
print("random turtles: exact-zero %d/%d"%(exact,n))
