"""Independent slice-integral oracle: volume of  {n.p >= s}  intersected
with the lens  ball(0,1) ∩ ball((0,1.4,0),0.8), via 1D integral along n of
closed-form disc∩halfplane areas.  For y >= y_r the lens is ball1, below it
ball2 (radical plane y_r)."""
import math, sys
def seg(R2, v0):  # area of disc radius sqrt(R2) on side signed-dist >= 0, centre at v0
    if R2 <= 0: return 0.0
    R = math.sqrt(R2)
    if v0 >= R: return math.pi*R2
    if v0 <= -R: return 0.0
    return R2*math.acos(-v0/R) + v0*math.sqrt(R2-v0*v0)
def lens_cap(n, s, r1=1.0, r2=0.8, d=1.4):
    yr = (d*d + r1*r1 - r2*r2)/(2*d)
    ny = n[1]; g = math.sqrt(max(0.0, 1-ny*ny))
    def area(t):
        tot = 0.0
        for (cy, r, sgn) in ((0.0, r1, +1), (d, r2, -1)):
            h = t - ny*cy            # plane n.p = t, distance from centre
            R2 = r*r - h*h
            if R2 <= 0: continue
            cyp = cy + ny*h          # centre of slice disc: c + n*h ; its y
            v0 = sgn*(cyp - yr)/g    # signed in-plane distance into the side
            tot += seg(R2, v0)
        return tot
    a, b = s, r1          # the lens lies inside ball1: n.p <= r1
    # composite 5-point Gauss-Legendre
    xs = [0.0, -0.5384693101056831, 0.5384693101056831, -0.9061798459386640, 0.9061798459386640]
    ws = [0.5688888888888889, 0.4786286704993665, 0.4786286704993665, 0.2369268850561891, 0.2369268850561891]
    N = 40000; hstep = (b-a)/N; tot = 0.0
    for k in range(N):
        m = a + (k+0.5)*hstep
        tot += sum(w*area(m + x*hstep/2) for x, w in zip(xs, ws))*hstep/2
    return tot
def dirv(lat, az):
    la, z = math.radians(lat), math.radians(az)
    return (math.cos(la)*math.cos(z), math.sin(la), math.cos(la)*math.sin(z))
if __name__ == "__main__":
    for tilt, az, h in [(20,90,0.015),(26,90,0.015),(25,90,0.012)]:
        n = dirv(90-tilt, az); s = 1-h
        print(tilt, az, h, repr(lens_cap(n, s)), "cap", math.pi*h*h*(3-h)/3)
