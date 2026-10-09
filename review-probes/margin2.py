import numpy as np

def quad(f,a,b,epsabs=0,epsrel=0):
    import numpy as np
    n=2000; h=(b-a)/n
    x=np.linspace(a,b,n+1); y=np.array([f(t) for t in x])
    return (h/3*(y[0]+y[-1]+4*y[1:-1:2].sum()+2*y[2:-1:2].sum()),0)
def run(k, t0, s_list, label):
    a,b=float(k),1.0
    P=lambda t: np.array([a*np.cos(t), b*np.sin(t), 0.0])
    dP=lambda t: np.array([-a*np.sin(t), b*np.cos(t), 0.0])
    speed=lambda t: np.linalg.norm(dP(t))
    axis=np.array([0,0,1.0]); c=np.zeros(3)
    site=P(t0); that=dP(t0)/speed(t0)
    u=site-c; radial=u-axis*axis.dot(u); n=np.cross(axis,radial)
    sinpsi=abs(n.dot(that))/np.linalg.norm(radial)
    out=[]
    for s in s_list:
        # find t1 with arc s
        lo,hi=t0,t0+np.pi/2
        for _ in range(80):
            m=(lo+hi)/2
            if quad(speed,t0,m,epsabs=1e-15,epsrel=1e-14)[0]<s: lo=m
            else: hi=m
        p=P(lo)
        old=n.dot(p-c)/np.linalg.norm(radial)
        new=n.dot(p-site)/abs(n.dot(that))
        out.append((s,old/s,new/s))
    print(f"{label}: k={k} t0={t0:.4f} sinpsi={sinpsi:.4f}  "+"  ".join(f"s={s:.0e}: old/s={o:.4f} new/s={nw:.6f}" for s,o,nw in out))
    # off-curve sensitivity: displace p normal to the curve by delta
    N=np.array([-that[1],that[0],0.0])
    print(f"   d(new)/d(normal offset) = {abs(n.dot(N))/abs(n.dot(that)):.2f}   d(old)/d(offset) = {abs(n.dot(N))/np.linalg.norm(radial):.3f}")
for k in [1,6,10,60]:
    run(k, np.pi/4 if k>1 else 0.7, [1e-9,1e-3,0.1], "flank t=pi/4")
for k in [6,10,60]:
    run(k, 0.0, [1e-9,1e-3,0.1], "major vertex")
    run(k, np.pi/2, [1e-9,1e-3,0.1], "minor vertex")
    # the steepest-ψ site: sinψ minimal at tan t = sqrt(a/b)... scan
    ts=np.linspace(0.01,np.pi/2-0.01,20000)
    sp=[abs(np.cross([0,0,1],[k*np.cos(t),np.sin(t),0]).dot(np.array([-k*np.sin(t),np.cos(t),0])))/(np.hypot(k*np.cos(t),np.sin(t))*np.hypot(k*np.sin(t),np.cos(t))) for t in ts]
    i=int(np.argmin(sp)); print(f"   k={k}: min sinpsi={sp[i]:.4f} at t={ts[i]:.4f}, 2k/(k^2+1)={2*k/(k*k+1):.4f}")
    run(k, ts[i], [1e-9,1e-3,0.1], "min-psi site")
