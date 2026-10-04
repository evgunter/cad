import numpy as np
rng=np.random.default_rng(7)
def rot():
    q=rng.normal(size=4);q/=np.linalg.norm(q);a,b,c,d=q
    return np.array([[a*a+b*b-c*c-d*d,2*(b*c-a*d),2*(b*d+a*c)],[2*(b*c+a*d),a*a-b*b+c*c-d*d,2*(c*d-a*b)],[2*(b*d-a*c),2*(c*d+a*b),a*a-b*b-c*c+d*d]])
def frame(o,ax,r,cen,R):  # copy of cs_transverse_frame
    foot=o+ax*np.dot(cen-o,ax); off=cen-foot; d=np.linalg.norm(off)
    m=abs(R)-abs(r)-d
    return (cen,off/d) if m<0 else (foot,ax)
def loops(foot,a,e1,r,d,R,n=20000):
    e2=np.cross(a,e1); out=[]
    h2=lambda t:R*R-r*r-d*d+2*r*d*np.cos(t)
    def pts(t,up):
        h=np.sqrt(np.maximum(h2(t),0))
        P=foot+np.outer(np.cos(t),e1)*r+np.outer(np.sin(t),e2)*r+np.outer(up*h,a)
        return P
    if h2(np.pi)>0:
        t=np.linspace(0,2*np.pi,n,endpoint=False)
        out=[pts(t,1.0),pts(t[::-1],-1.0)]
    else:
        t0=np.arccos((r*r+d*d-R*R)/(2*r*d)); t=np.linspace(-t0,t0,n)[1:-1]
        out=[np.vstack([pts(t,1.0),pts(t[::-1],-1.0)])]
    return out
worst=[];bad=0;N=0
cases=[]
for k in range(4000):
    r=10**rng.uniform(-1,0.5); d=r*10**rng.uniform(-3,0.6)
    kind=k%5
    if kind==0: R=r+d-(r*d)*10**rng.uniform(-7,-1)        # just short (one loop)
    elif kind==1: R=r+d+(r*d)*10**rng.uniform(-7,-1)      # just past
    elif kind==2: R=r*(1+rng.uniform(-1e-3,1e-3))          # R ~ r
    elif kind==3: d=r*(1+rng.uniform(-1e-4,1e-4)); R=rng.uniform(1e-3,2*r+d)  # centre near wall
    else: R=rng.uniform(abs(r-d),r+d+2*r)
    if not (abs(r-d)<R) or abs(R-(r+d))<1e-12: continue
    Q=rot(); a=Q[:,2]; e1=Q[:,0]; foot=rng.normal(size=3)
    cen=foot+e1*d; o=foot+a*rng.normal()
    rs=r*(1 if rng.random()<.8 else -1); Rs=R*(1 if rng.random()<.8 else -1)
    c,ax=frame(o,a,rs,cen,Rs)
    for L in loops(foot,a,e1,r,d,R):
        T=np.roll(L,-1,0)-np.roll(L,1,0); T/=np.linalg.norm(T,axis=1)[:,None]
        s=np.einsum('j,ij->i',ax,np.cross(L-c,T))
        u=L-c; u=u-np.outer(u@ax,ax); v=np.roll(u,-1,0)
        turn=np.sum(np.arctan2(np.cross(u,v)@ax,np.einsum('ij,ij->i',u,v)))
        N+=1
        ok=(np.all(s>0) or np.all(s<0)) and abs(abs(turn)-2*np.pi)<1e-6
        if not ok: bad+=1; print("BAD",kind,r,d,R,s.min(),s.max(),turn)
        worst.append((np.abs(s).min()/max(r,d,abs(R)),kind,r,d,R))
print("loops",N,"bad",bad)
worst.sort(); print("smallest normalised |sense|:",worst[:3])
