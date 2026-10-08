from probe import *
rng = random.Random(int(sys.argv[1]) if len(sys.argv)>1 else 1)
N = int(sys.argv[2]) if len(sys.argv)>2 else 20
rows=[]; viol=[]; served_off=[]
for eps in [1e-9, 1e-6, 1e-12]:
  for scale in [1e-3, 1.0, 1e3]:
    for off in [0.0, 1e3]:
      for i in range(N):
        ta = rand_unit(rng); R = scale*rng.uniform(1,2); r = R*rng.uniform(0.05,0.95)
        tc = mul(rand_unit(rng), off)
        e = unit(cross(ta, rand_unit(rng))); t = cross(ta, e)
        ph = rng.uniform(0, 2*math.pi)
        C = add(tc, mul(e, R)); n = t; u = add(mul(e, math.cos(ph)), mul(ta, math.sin(ph)))
        # budget split
        w = [rng.random() for _ in range(3)]; s=sum(w); w=[x/s for x in w]
        tot = eps*rng.uniform(0.6, 1.0)
        dc, dr, dt = [x*tot for x in w]
        C2 = add(C, mul(rand_unit(rng), dc))
        rho = r + rng.choice([-1,1])*dr
        # tilt: target tilt term dt ~ r s^2 R / 2(R-r) -> s
        sg = math.sqrt(max(2*dt*(R-r)/(r*R),0))
        k = unit(cross(n, rand_unit(rng)))  # axis in carrier plane -> tilts plane
        if rng.random()<0.5: k = unit(add(k, mul(n, rng.uniform(-1,1))))
        ang = math.asin(min(sg,1.0))
        n2 = rot(n, k, ang); u2 = rot(u, k, ang)
        dev = deviation(n2, rho, C2, tc, ta, R, r)
        tru = oracle(C2, n2, rho, u2, tc, ta, R, r, n=1024)
        rows.append((eps,scale,off,dev,tru))
        if tru > dev*(1+1e-9): viol.append((eps,scale,off,i,dev,tru))
        if dev <= eps and tru > eps: served_off.append((eps,scale,off,i,dev,tru, tru/eps))
print("cases", len(rows), "bound violations", len(viol), "served-off", len(served_off))
for v in viol[:10]: print("VIOL", v)
for v in sorted(served_off, key=lambda x:-x[-1])[:15]: print("OFF", v)
import statistics
for eps in [1e-9,1e-6,1e-12]:
    rs=[t/d for (e,_,_,d,t) in rows if e==eps and d>0]
    print(eps, "tru/dev min %.3f median %.3f max %.3f"%(min(rs), statistics.median(rs), max(rs)))
