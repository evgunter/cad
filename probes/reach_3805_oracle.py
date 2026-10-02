"""High-precision oracle (stdlib decimal, 60 digits) for the pinned
clearance counterexample: the least TRUE distance of the ellipse from the
sphere, minimised over theta near the minor vertex, from the stored f64
data read exactly. Never the kernel."""
from decimal import Decimal as D, getcontext
getcontext().prec = 60
def sin_cos(x):
    # reduce by 2pi
    pi = D('3.14159265358979323846264338327950288419716939937510582097494')
    tau = 2 * pi
    x = x - tau * int(x / tau)
    s, c, term, n = D(0), D(0), D(1), 0
    # cos and sin by series
    c = D(0); s = D(0); t = D(1); k = 0
    while True:
        tc = t
        c += tc if k % 4 == 0 else (-tc if k % 4 == 2 else 0)
        s += tc if k % 4 == 1 else (-tc if k % 4 == 3 else 0)
        k += 1
        t = t * x / k
        if abs(t) < D('1e-70') and k > 10:
            break
    return s, c
def cross(a, b):
    return (a[1]*b[2]-a[2]*b[1], a[2]*b[0]-a[0]*b[2], a[0]*b[1]-a[1]*b[0])
def run(center, axis, major, minor, u_ref, sc, r, theta0):
    f = lambda v: tuple(D(x) for x in v)
    center, axis, u_ref, sc = f(center), f(axis), f(u_ref), f(sc)
    major, minor, r = D(major), D(minor), D(r)
    v_ref = cross(axis, u_ref)
    def dist(th):
        s, c = sin_cos(th)
        p = [center[i] + u_ref[i]*major*c + v_ref[i]*minor*s for i in range(3)]
        return sum((p[i]-sc[i])**2 for i in range(3)).sqrt() - r
    lo, hi = D(theta0) - D('0.01'), D(theta0) + D('0.01')
    g = (D(5).sqrt() - 1) / 2
    for _ in range(200):
        a = hi - g*(hi-lo); b = lo + g*(hi-lo)
        if dist(a) < dist(b): hi = b
        else: lo = a
    return dist((lo+hi)/2)
pi = 3.141592653589793
print("case 168, eps 1e-12, least true distance:",
  run((-0.4805481181680724, -0.4410169206690735, -0.8889582859866558),
      (-0.9621201415661211, -0.09095689596824941, 0.2570052066955227),
      4.846323757498574, 0.20752839706206302,
      (0.0877135584784833, 0.7893034347374652, 0.6077058659998946),
      (-0.4269724200467884, -0.5670491346043792, -0.7329974017510368),
      2.4660736247312856e-5, 3*pi/2))
