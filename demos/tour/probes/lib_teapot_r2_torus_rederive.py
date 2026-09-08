# Independent numeric re-derivation of TestTorusvessel.boundary(t):
# the solid of revolution bounded by foot cylinder + torus band + neck,
# moved inward by t. Volume by pi * integral rho(y)^2 dy, area by
# 2 pi * integral rho ds, both by composite Simpson over each run, plus the
# flat annuli/caps. Nothing here is copied from the row's formula.
import math
R_FOOT=5/64; R_BAND=9/64; R_NECK=7/64; Y_FOOT=4/64; Y_SHOULDER=12/64; Y_MOUTH=24/64
R_TUBE=5/64; A_HALF=4/64; H_TUBE=8/64; R_BELLIED=6/64; WALL=1/128

def simpson(f,a,b,n=200000):
    if n%2: n+=1
    h=(b-a)/n; s=f(a)+f(b)
    for i in range(1,n):
        s+= (4 if i%2 else 2)*f(a+i*h)
    return s*h/3

def rederive(t):
    R=R_BELLIED; r=R_TUBE-t; a=A_HALF-t
    foot=R_FOOT-t; neck=R_NECK-t
    # band: rho(u) = R + sqrt(r^2-u^2), u in [-a,a]; arc length element ds = r/sqrt(r^2-u^2) du
    rho=lambda u: R+math.sqrt(max(r*r-u*u,0.0))
    v_band=math.pi*simpson(lambda u: rho(u)**2, -a, a)
    a_band=2*math.pi*simpson(lambda u: rho(u)*r/math.sqrt(r*r-u*u), -a*(1-1e-12), a*(1-1e-12))  # avoid the endpoint singularity
    # endpoint singularity: integrate by theta instead: u=r sin th, ds=r dth, rho=R+r cos th
    th0=math.asin(a/r)
    a_band=2*math.pi*simpson(lambda th:(R+r*math.cos(th))*r, -th0, th0)
    junction=R+math.sqrt(r*r-a*a)
    V=math.pi*foot*foot*Y_FOOT + v_band + math.pi*neck*neck*(Y_MOUTH-Y_SHOULDER)
    A=(math.pi*foot*foot + 2*math.pi*foot*Y_FOOT + math.pi*(junction**2-foot**2) + a_band
       + math.pi*(junction**2-neck**2) + 2*math.pi*neck*(Y_MOUTH-Y_SHOULDER) + math.pi*neck*neck)
    return V,A

def row_boundary(t):
    big_r=R_BELLIED; r,a=R_TUBE-t,A_HALF-t; theta0=math.asin(a/r)
    foot,neck=R_FOOT-t,R_NECK-t
    junction=big_r+math.sqrt(r*r-a*a)
    v_band=math.pi*(2*a*big_r*big_r+2*big_r*(a*math.sqrt(r*r-a*a)+r*r*theta0)+2*r*r*a-2*a**3/3)
    volume=math.pi*foot*foot*Y_FOOT+v_band+math.pi*neck*neck*(Y_MOUTH-Y_SHOULDER)
    a_band=4*math.pi*r*(big_r*theta0+a)
    area=(math.pi*foot*foot+2*math.pi*foot*Y_FOOT+math.pi*(junction*junction-foot*foot)+a_band
          +math.pi*(junction*junction-neck*neck)+2*math.pi*neck*(Y_MOUTH-Y_SHOULDER)+math.pi*neck*neck)
    return volume,area

for t in (0.0, WALL):
    V,A=rederive(t); v,a=row_boundary(t)
    print(f"t={t}: rederived V={V:.15g} A={A:.15g}; row V={v:.15g} A={a:.15g}; relerr V={(V-v)/v:.2e} A={(A-a)/a:.2e}")
v0,a0=row_boundary(0.0); v1,a1=row_boundary(WALL)
print("wall V", v0-v1, "sum A", a0+a1, "capacity L", v1*1000)
# sanity: the heights are consistent (band spans Y_FOOT..Y_SHOULDER = 2a at t=0)
print("2*A_HALF == Y_SHOULDER-Y_FOOT:", 2*A_HALF == Y_SHOULDER-Y_FOOT, "H_TUBE centre == mid:", H_TUBE==(Y_FOOT+Y_SHOULDER)/2)
