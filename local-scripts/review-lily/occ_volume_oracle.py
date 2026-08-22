# FreeCAD headless: import each lily STEP, report OCC volume (mm^3 internal),
# compare against MY independently derived closed forms (m^3 -> mm^3).
import math
import os
import sys
import Part

def torus_seg(R, r, deg):
    return math.radians(deg) * R * math.pi * r**2

def zone_frustum(r, a, b, rho, h):
    zone = math.pi * (r**2*(a+b) - (a**3+b**3)/3)
    R2 = math.sqrt(r**2 - b**2)
    return zone + math.pi*h*(R2**2 + R2*rho + rho**2)/3

def seg_area(c, h):
    R = (c**2/4 + h**2)/(2*h)
    return R**2*math.acos((R-h)/R) - (R-h)*c/2

def crescent(c, wo, wi, t):
    return (seg_area(c, wo) - seg_area(c, wi)) * t

expect = {
    'lily_stem':     torus_seg(5.0, 0.060, 22.0),
    'lily_arch':     torus_seg(1.1, 0.052, 170.0),
    'lily_pedicel':  torus_seg(0.42, 0.032, 130.0),
    'lily_lantern':  zone_frustum(0.44, 0.40, 0.36, 0.09, 0.16),
    'lily_lantern2': zone_frustum(0.30, 0.27, 0.24, 0.05, 0.15),
    'lily_leaf_a':   crescent(1.45, 0.16, 0.035, 0.026),
    'lily_leaf_b':   crescent(1.25, 0.14, 0.030, 0.024),
    'lily_leaf_c':   crescent(0.95, 0.115, 0.025, 0.022),
}

d = sys.argv[-1] if os.path.isdir(sys.argv[-1]) else os.environ['LILY_DIR']
worst = 0.0
for name, vm3 in sorted(expect.items()):
    p = os.path.join(d, name + '.step')
    shape = Part.Shape()
    shape.read(p)
    solids = shape.Solids
    print(name, 'solids=%d' % len(solids), 'valid=%s' % all(s.isValid() for s in solids))
    v = sum(s.Volume for s in solids)  # mm^3 (FreeCAD internal)
    ref = vm3 * 1e9
    rel = abs(v - ref) / ref
    worst = max(worst, rel)
    print('  occ=%r ref=%r rel=%.3e' % (v, ref, rel))
print('WORST_REL=%.3e' % worst)
