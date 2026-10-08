from probe import *
import json
rng = random.Random(7)
fx=[]; meta=[]
def emit(eps, C, n, rho, u, tc, ta, R, r, kind):
    fx.append([eps]+C+n+[rho]+u+tc+ta+[R, r]); meta.append(kind)
for eps in [1e-9, 1e-6, 1e-12]:
  for scale in [1e-3, 1.0, 1e3]:
    for off in [0.0, 1e3]:
      for i in range(40):
        ta = rand_unit(rng); R = scale*rng.uniform(1,2)
        q = rng.choice([0.05,0.25,0.5,0.8,0.95,0.99, rng.uniform(0.05,0.99)])
        r = R*q
        tc = mul(rand_unit(rng), off)
        e = unit(cross(ta, rand_unit(rng))); t = unit(cross(ta, e))
        ph = rng.uniform(0, 2*math.pi)
        C = add(tc, mul(e, R)); n = t; u = add(mul(e, math.cos(ph)), mul(ta, math.sin(ph)))
        mode = i % 4
        if mode == 0:  # exact meridian
            emit(eps, C, n, r, u, tc, ta, R, r, "meridian"); continue
        if mode == 1:  # aligned: each 0.45 eps; radial out + vertical + radius + tilt
            dc, dr, dt = 0.45*eps, 0.45*eps, 0.45*eps
            dirc = unit(add(mul(e, rng.uniform(-1,1)), mul(ta, rng.uniform(-1,1))))
        elif mode == 2:  # each 0.3 eps (sum 0.9) random directions
            dc, dr, dt = 0.3*eps, 0.3*eps, 0.3*eps
            dirc = rand_unit(rng)
        else:  # random split total 1.2 eps
            w=[rng.random() for _ in range(3)]; s=sum(w); dc,dr,dt=[x/s*1.2*eps for x in w]
            dirc = rand_unit(rng)
        C2 = add(C, mul(dirc, dc)); rho = r + rng.choice([-1,1])*dr
        sg = math.sqrt(2*dt*(R-r)/(r*R))
        k = unit(cross(n, rand_unit(rng)))
        ang = math.asin(min(sg,1.0))
        n2 = rot(n, k, ang); u2 = rot(u, k, ang)
        emit(eps, C2, n2, rho, u2, tc, ta, R, r, "mixed%d"%mode)
for eps in [1e-9]:
  for q in [1.2, 1.5, 1.0+1e-9]:
    for i in range(8):
        ta=[0.0,0.0,1.0]; R=1.0; r=R*q; tc=[0.0,0.0,0.0]
        e=unit([math.cos(i),math.sin(i),0.0]); t=unit(cross(ta,e)); ph=0.3*i
        C=add(tc,mul(e,R)); u=add(mul(e,math.cos(ph)),mul(ta,math.sin(ph)))
        if i%2==0: emit(eps,C,t,r,u,tc,ta,R,r,"spindle-meridian"); continue
        k=unit(cross(t,[0.3,0.5,0.8])); ang=1e-3*(i)
        emit(eps,C,rot(t,k,ang),r,rot(u,k,ang),tc,ta,R,r,"spindle-tilt")
open('fx.txt','w').write("\n".join(" ".join(repr(x) for x in f) for f in fx))
json.dump(meta, open('meta.json','w'))
print(len(fx))
