#!/usr/bin/env python3
"""Review probe for MESH-12 (issue 1588, measurement 2): half of a
spherical cap whose RIM ROW is stated as TWO on-sphere circle arcs at
latitudes v1 and v1 + dv, junction vertex at the mean latitude.  Writes
two_level_rim_f<F>.step into the directory given as argv[1], one file
per rim gap factor F, where R * dv = F * EPS metres.

Topology (chi = 2): vertices A (u=0, on rim 1), J (u=pi/2, mean
latitude), B (u=pi, on rim 2), P (the north pole); edges rim1 A->J,
rim2 J->B, m1 A->P and m2 P->B on the y=0 meridian great circle
(split AT the pole so no arc crosses it), and the diameter line A->B.
Faces: the sphere face, the base plane through A, B and the y axis
direction (each rim is off it by at most R*dv*cos(v)/2), and the cut
plane y = 0.

The point under review: the two rims are exactly on the sphere and
props' `props_rim_level` would admit them for F < 1; whether the
import door lets the body reach that decide, or refuses first at the
pcurve re-mint's `pcurve_loop_continuity`, is what the step-import
row reads off these files.
"""

import math
import sys

R = 10.0  # mm
V1 = 0.5  # lower rim latitude, rad
EPS_M = 1.0e-9  # the default band, metres
FACTORS = [0.5, 1.0, 1.5]


def num(c):
    if abs(c) < 1e-15:
        c = 0.0
    t = "%.17g" % c
    if not any(ch in t for ch in ".eE"):
        t += "."
    return t


def fmt(v):
    return ",".join(num(c) for c in v)


def unit(v):
    n = math.sqrt(sum(c * c for c in v))
    return tuple(c / n for c in v)


def cross(a, b):
    return (
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    )


class Step:
    def __init__(self):
        self.lines, self.n = [], 0

    def add(self, text):
        self.n += 1
        self.lines.append("#%d = %s;" % (self.n, text))
        return self.n

    def pt(self, p):
        return self.add("CARTESIAN_POINT('',(%s))" % fmt(p))

    def dr(self, d):
        return self.add("DIRECTION('',(%s))" % fmt(d))

    def ax2(self, origin, axis, ref):
        return self.add(
            "AXIS2_PLACEMENT_3D('',#%d,#%d,#%d)"
            % (self.pt(origin), self.dr(axis), self.dr(ref))
        )


def build(factor):
    dv = factor * EPS_M / (R * 1e-3)  # radians
    v2 = V1 + dv
    vm = V1 + 0.5 * dv
    pos = {
        "A": (R * math.cos(V1), 0.0, R * math.sin(V1)),
        "J": (0.0, R * math.cos(vm), R * math.sin(vm)),
        "B": (-R * math.cos(v2), 0.0, R * math.sin(v2)),
        "P": (0.0, 0.0, R),
    }

    s = Step()
    s.add(
        "APPLICATION_PROTOCOL_DEFINITION('international standard',"
        "'automotive_design',2000,#2)"
    )
    s.add("APPLICATION_CONTEXT('core data for automotive mechanical design processes')")
    s.add("SHAPE_DEFINITION_REPRESENTATION(#4,#10)")
    s.add("PRODUCT_DEFINITION_SHAPE('','',#5)")
    s.add("PRODUCT_DEFINITION('design','',#6,#9)")
    s.add("PRODUCT_DEFINITION_FORMATION('','',#7)")
    s.add("PRODUCT('tl','tl','',(#8))")
    s.add("PRODUCT_CONTEXT('',#2,'mechanical')")
    s.add("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design')")
    s.n += 1
    absr_slot = len(s.lines)
    s.lines.append(None)
    world = s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0))
    assert world == 14
    s.n += 1
    msb_slot = len(s.lines)
    s.lines.append(None)
    msb = 15
    s.n += 1
    shell_slot = len(s.lines)
    s.lines.append(None)
    shell = 16

    V = {nm: s.add("VERTEX_POINT('',#%d)" % s.pt(p)) for nm, p in pos.items()}

    E = {}

    def rim_circle(v):
        return s.add(
            "CIRCLE('',#%d,%s)"
            % (s.ax2((0, 0, R * math.sin(v)), (0, 0, 1), (1, 0, 0)), num(R * math.cos(v)))
        )

    E["rim1"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["J"], rim_circle(V1)))
    E["rim2"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["J"], V["B"], rim_circle(v2)))

    def meridian_circle():
        return s.add(
            "CIRCLE('',#%d,%s)" % (s.ax2((0, 0, 0), (0, -1, 0), (1, 0, 0)), num(R))
        )

    E["m1"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["P"], meridian_circle()))
    E["m2"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["P"], V["B"], meridian_circle()))

    ab = tuple(b - a for a, b in zip(pos["A"], pos["B"], strict=True))
    d_dir = unit(ab)
    line = s.add(
        "LINE('',#%d,#%d)"
        % (s.pt(pos["A"]), s.add("VECTOR('',#%d,1.)" % s.dr(d_dir)))
    )
    E["d"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["B"], line))

    faces = []
    _all_oriented = []

    def face(surf_id, loop):
        _all_oriented.extend(loop)
        oes = [s.add("ORIENTED_EDGE('',*,*,#%d,.%s.)" % (E[nm], o)) for nm, o in loop]
        el = s.add("EDGE_LOOP('',(%s))" % ",".join("#%d" % i for i in oes))
        fb = s.add("FACE_OUTER_BOUND('',#%d,.T.)" % el)
        faces.append(s.add("ADVANCED_FACE('',(#%d),#%d,.T.)" % (fb, surf_id)))

    sph = s.add(
        "SPHERICAL_SURFACE('',#%d,%s)" % (s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0)), num(R))
    )
    face(sph, [("rim1", "T"), ("rim2", "T"), ("m2", "F"), ("m1", "F")])

    # Base plane through A and B, containing the y direction; outward
    # is the -z-ish side (material above).
    n = unit(cross(ab, (0.0, 1.0, 0.0)))
    if n[2] > 0:
        n = tuple(-c for c in n)
    ref = unit(cross((0.0, 1.0, 0.0), n))
    base = s.add("PLANE('',#%d)" % s.ax2(pos["A"], n, ref))
    face(base, [("rim2", "F"), ("rim1", "F"), ("d", "T")])

    cut = s.add("PLANE('',#%d)" % s.ax2((0, 0, 0), (0, -1, 0), (1, 0, 0)))
    face(cut, [("m1", "T"), ("m2", "T"), ("d", "F")])

    use = {}
    for nm, o in _all_oriented:
        use.setdefault(nm, []).append(o)
    bad = [(nm, v) for nm, v in use.items() if sorted(v) != ["F", "T"]]
    unused = [nm for nm in E if nm not in use]
    assert not bad, ("mis-paired edges", bad)
    assert not unused, ("unused edges", unused)

    s.lines[shell_slot] = "#%d = CLOSED_SHELL('',(%s));" % (
        shell,
        ",".join("#%d" % f for f in faces),
    )
    s.lines[msb_slot] = "#%d = MANIFOLD_SOLID_BREP('',#%d);" % (msb, shell)

    lu = s.add("( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) )")
    pau = s.add("( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) )")
    sau = s.add("( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() )")
    unc = s.add(
        "UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#%d,"
        "'distance_accuracy_value','confusion accuracy')" % lu
    )
    ctx = s.add(
        "( GEOMETRIC_REPRESENTATION_CONTEXT(3) "
        "GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#%d)) "
        "GLOBAL_UNIT_ASSIGNED_CONTEXT((#%d,#%d,#%d)) "
        "REPRESENTATION_CONTEXT('Context #1','3D Context with UNIT and UNCERTAINTY') )"
        % (unc, lu, pau, sau)
    )
    s.add("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#7))")
    s.lines[absr_slot] = (
        "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#%d,#%d),#%d);" % (world, msb, ctx)
    )

    head = """ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('two-level rim row probe'),'2;1');
FILE_NAME('tl','2026-09-03T00:00:00',('hand'),('hand'),'hand','hand','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));
ENDSEC;
DATA;
"""
    print(
        "f=%g  dv=%.6e rad  R*dv=%.6e m  junction-to-rim=%.6e m"
        % (factor, dv, dv * R * 1e-3, 0.5 * dv * R * 1e-3)
    )
    return head + "\n".join(s.lines) + "\nENDSEC;\nEND-ISO-10303-21;\n"


def main():
    out = sys.argv[1]
    for f in FACTORS:
        name = "two_level_rim_f%s.step" % ("%g" % f).replace(".", "p")
        with open("%s/%s" % (out, name), "w", encoding="utf-8") as fh:
            fh.write(build(f))


if __name__ == "__main__":
    main()
