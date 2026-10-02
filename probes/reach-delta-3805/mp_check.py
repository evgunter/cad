"""Extended-precision recheck of one fuzz case: the ellipse
c + major*u*cos t + minor*(n x u)*sin t against a sphere, every stored
f64 taken exactly. Prints the signed distance's extremes/sign changes."""
import sys
from mpmath import mp, mpf, cos, sin, sqrt, pi, findroot
mp.dps = 50
C = [mpf(x) for x in (-959.2639129088163, -367.02388074477653, 623.0471277842355)]
N = [mpf(x) for x in (-0.31903108529555385, -0.8958001297855095, -0.3094532179367431)]
U = [mpf(x) for x in (-0.1811201960417648, -0.2628702620852155, 0.9476785846989699)]
A, B = mpf(-3514.3154468033267), mpf(-2361.48048802387)
S = [mpf(x) for x in (-2850.858173756086, -510.3082903169221, 2988.8851976595456)]
R = mpf(39.170514333982986)
V = [N[1]*U[2]-N[2]*U[1], N[2]*U[0]-N[0]*U[2], N[0]*U[1]-N[1]*U[0]]
def P(t):
    c, s = cos(t), sin(t)
    return [C[i] + U[i]*A*c + V[i]*B*s for i in range(3)]
def d(t):
    p = P(t)
    return sqrt(sum((p[i]-S[i])**2 for i in range(3))) - R
t0, t1 = mpf(5.382086155163396), mpf(9.981586155095057)
mid = (t0+t1)/2
n = 4000
best = None
for k in range(n+1):
    t = mid - pi + 2*pi*k/n
    v = d(t)
    if best is None or abs(v) < abs(best[1]):
        best = (t, v)
print("coarse min |d|", best)
# refine signed extremum near best by golden section on d (minimise sign*d)
sg = 1 if best[1] > 0 else -1
lo, hi = best[0]-2*pi/n, best[0]+2*pi/n
g = (sqrt(5)-1)/2
for _ in range(200):
    x1 = hi-g*(hi-lo); x2 = lo+g*(hi-lo)
    if sg*d(x1) < sg*d(x2): hi = x2
    else: lo = x1
te = (lo+hi)/2
print("extremum t", te, "d", mp.nstr(d(te), 20))
