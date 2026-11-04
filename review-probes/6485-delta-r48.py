import mpmath as mp, math
mp.mp.dps = 50
eps_run = 1e-12
C = [-2.4039241239609534e-11,0.0,1.0]; N=[1.0399662681036463e-14,0.0,1.0]; U=[1.0,0.0,-1.0399662681036463e-14]; R=48.07848247921907
al = 1.55
def cross(a,b): return [a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0]]
def dot(a,b): return sum(x*y for x,y in zip(a,b))
def coeffs(num, sin, cos, sqrt):
    Cn=[num(x) for x in C]; Nn=[num(x) for x in N]; Un=[num(x) for x in U]; r=num(R); a_=num(al)
    # code normalizes? frame stored as is; v_ref = axis x u_ref
    V = cross(Nn, Un)
    sa, ca = sin(a_), cos(a_)
    cc, ss = ca*ca, sa*sa
    ax=[num(0),num(0),num(1)]
    perp = lambda x: [x[i]-ax[i]*dot(ax,x) for i in range(3)]
    d = Cn
    e, up, vp = perp(d), perp(Un), perp(V)
    aa, bb = r*r*dot(up,up), r*r*dot(vp,vp)
    half=num(0.5); two=num(2)
    wall=[dot(e,e)+(aa+bb)*half, two*r*dot(e,up), two*r*dot(e,vp), (aa-bb)*half, r*r*dot(up,vp)]
    h0,hu,hv = dot(d,ax), r*dot(Un,ax), r*dot(V,ax)
    height=[h0*h0+(hu*hu+hv*hv)*half, two*h0*hu, two*h0*hv, (hu*hu-hv*hv)*half, hu*hv]
    return [cc*wall[k]-ss*height[k] for k in range(5)], wall, height, cc, ss
f64c, w, hgt, cc, ss = coeffs(float, math.sin, math.cos, math.sqrt)
exc, *_ = coeffs(mp.mpf, mp.sin, mp.cos, mp.sqrt)
for k in range(5): print(k, "f64", f64c[k], "exact", mp.nstr(exc[k], 12), "err", float(abs(f64c[k]-exc[k])))
print("cc*wall0", cc*w[0], "ss*h0", ss*hgt[0])
sup_exact = abs(exc[0]) + mp.hypot(exc[1],exc[2]) + mp.hypot(exc[3],exc[4])
print("exact form sup bound (m^2):", mp.nstr(sup_exact, 8))
# exact distance sup
Cn=[mp.mpf(x) for x in C]; Nn=[mp.mpf(x) for x in N]; Un=[mp.mpf(x) for x in U]; V=cross(Nn,Un)
def dist(t):
    p=[Cn[i]+R*(mp.cos(t)*Un[i]+mp.sin(t)*V[i]) for i in range(3)]
    rho=mp.sqrt(p[0]**2+p[1]**2); return abs(rho*mp.cos(al)-abs(p[2])*mp.sin(al))
best=max(dist(2*mp.pi*i/2000) for i in range(2000))
print("50-digit sup distance (m):", mp.nstr(best, 8))
print("rounding charge 8.557e-12 m^2 -> envelope 4.28e-12 m; charge / actual f64 max coeff err:", 8.557412351930129e-12/max(float(abs(f64c[k]-exc[k])) for k in range(5)))
