#!/usr/bin/env python3
"""Independent closed forms for the 8 lily bodies, derived from the
authored parameters in demos/tour/src/lily.rs ONLY. Formula validity is
cross-checked by adaptive-ish Simpson numeric integration."""
import math

def simpson(f, a, b, n=200000):
    h = (b - a) / n
    s = f(a) + f(b)
    for i in range(1, n):
        s += f(a + i * h) * (4 if i % 2 else 2)
    return s * h / 3

def torus_seg(R, r, theta_deg):
    # Pappus for a partial angle: centroid path length theta*R x disc area
    return math.radians(theta_deg) * R * math.pi * r**2

def zone_plus_frustum(globe, top, mouth, lip_r, lip_drop):
    r, a, b = globe, top, mouth
    zone = math.pi * (r**2 * (a + b) - (a**3 + b**3) / 3)
    R2 = math.sqrt(r**2 - b**2)
    frustum = math.pi * lip_drop * (R2**2 + R2 * lip_r + lip_r**2) / 3
    def x2(z):
        if z >= -b:
            return r**2 - z**2
        s = (-b - z) / lip_drop
        return ((1 - s) * R2 + s * lip_r) ** 2
    num = math.pi * (simpson(x2, -b - lip_drop, -b) + simpson(x2, -b, a))
    return zone + frustum, num

def segment_area(c, h):
    R = (c**2 / 4 + h**2) / (2 * h)
    return R**2 * math.acos((R - h) / R) - (R - h) * c / 2

def crescent(clen, w_out, w_in, thick):
    A = segment_area(clen, w_out) - segment_area(clen, w_in)
    def arc(c, h):
        R = (c**2/4 + h**2)/(2*h)
        yc = h - R
        return lambda x: yc + math.sqrt(max(R**2 - (x - c/2)**2, 0.0))
    yo, yi = arc(clen, w_out), arc(clen, w_in)
    Anum = simpson(lambda x: yo(x) - yi(x), 0.0, clen)
    return A * thick, Anum * thick

bodies = {}
bodies['lily_stem']    = torus_seg(5.0, 0.060, 22.0)
bodies['lily_arch']    = torus_seg(1.1, 0.052, 170.0)
bodies['lily_pedicel'] = torus_seg(0.42, 0.032, 130.0)
l1, l1n = zone_plus_frustum(0.44, 0.40, 0.36, 0.09, 0.16)
l2, l2n = zone_plus_frustum(0.30, 0.27, 0.24, 0.05, 0.15)
bodies['lily_lantern'], bodies['lily_lantern2'] = l1, l2
la, lan = crescent(1.45, 0.16, 0.035, 0.026)
lb, lbn = crescent(1.25, 0.14, 0.030, 0.024)
lc, lcn = crescent(0.95, 0.115, 0.025, 0.022)
bodies['lily_leaf_a'], bodies['lily_leaf_b'], bodies['lily_leaf_c'] = la, lb, lc

print("closed-form vs numeric cross-checks (rel):")
for name, cf, num in [('lantern', l1, l1n), ('lantern2', l2, l2n),
                      ('leaf_a', la, lan), ('leaf_b', lb, lbn), ('leaf_c', lc, lcn)]:
    print(f"  {name}: {(cf-num)/cf:.2e}")

print("\nindependent volumes:")
for k, v in bodies.items():
    print(f"  {k:15s} {v!r}")

print("\nfixture sidecar: lantern1 =", repr(l1), " mm3 =", repr(l1 * 1e9))
print("  sidecar says 0.36225803729804673 /", "362258037.2980467")

# ---- finding 14: near-tangency of the conical pucker ----
def near_tangency(globe, top, mouth, lip_r, lip_drop, tag):
    r, b = globe, mouth
    rm = math.sqrt(r**2 - b**2)
    P = (rm, -b)
    Q = (lip_r, -b - lip_drop)
    d = (Q[0]-P[0], Q[1]-P[1])
    dl = math.hypot(*d)
    dist = abs(P[0]*d[1] - P[1]*d[0]) / dl          # centre->carrier line
    tang = (-P[1]/r, P[0]/r)
    dot = abs(d[0]*tang[0] + d[1]*tang[1]) / dl
    ang = math.degrees(math.acos(min(dot, 1.0)))
    print(f"  {tag}: departure angle = {ang:.4g} deg, r - dist(line) = {r-dist:.4g}")

print("\nfinding 14 (near-tangency):")
near_tangency(0.44, 0.40, 0.36, 0.09, 0.16, "lantern1")
near_tangency(0.30, 0.27, 0.24, 0.05, 0.15, "lantern2")
