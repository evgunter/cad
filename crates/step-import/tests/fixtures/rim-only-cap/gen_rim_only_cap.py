#!/usr/bin/env python3
"""Hand-authored AP214 fixtures for the RIM-ONLY SPHERE CAP.  Writes
TWO files into the directory given as argv[1]: rimonly2.step and
rimonly1.step.  (Adapted from ../halfcap/gen_halfcap.py.)

The solid is the cap of an R = 10 mm sphere above latitude B = 0.5 rad
(argv[2] overrides B), closed by its base disc: two faces sharing one
rim circle, the north pole in the INTERIOR of the sphere face.  The
sphere face's loop is the rim and nothing else -- no meridian edge, no
pole vertex.

  * rimonly2.step: the rim as two half arcs, 2 V / 2 E / 2 F;
  * rimonly1.step: the rim as ONE closed circle edge, 1 V / 1 E / 2 F.

Both import as a solid that passes every tier and measures the closed
form the generator prints; what `mesh::tessellate` answers for the
sphere face is the row in ../../meridian_free_cap.rs.

THESE FILES' DISPOSITION IS EXPECTED TO CHANGE, BY DESIGN.  A pole
inside a face is a vertex of it, so the face stated here is not one the
kernel keeps: work/exch/import-normalizes-the-rim-only-cap.md is the
unit that makes import re-mint it in the seamed form.  When it lands,
the two corpus rows in ../../tier_gate.rs (their edge and vertex
census) and the "adopted as stated" assertions in
../../meridian_free_cap.rs move with it -- that is the normalization
working, not a regression.  The files themselves stay as they are: they
are the input that unit normalizes.
"""
import math
import sys

R = 10.0            # mm
def num(c):
    if abs(c) < 1e-12:
        c = 0.0
    t = "%.15g" % c
    if not any(ch in t for ch in ".eE"):
        t += "."
    return t


def fmt(v):
    return ",".join(num(c) for c in v)


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
        return self.add("AXIS2_PLACEMENT_3D('',#%d,#%d,#%d)"
                        % (self.pt(origin), self.dr(axis), self.dr(ref)))


def build(kind):
    h = R * math.sin(B)
    rc = R * math.cos(B)
    pos = {"A": (rc, 0.0, h)}
    if kind == "rimonly2":
        pos["B"] = (-rc, 0.0, h)

    s = Step()
    s.add("APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2)")
    s.add("APPLICATION_CONTEXT('core data for automotive mechanical design processes')")
    s.add("SHAPE_DEFINITION_REPRESENTATION(#4,#10)")
    s.add("PRODUCT_DEFINITION_SHAPE('','',#5)")
    s.add("PRODUCT_DEFINITION('design','',#6,#9)")
    s.add("PRODUCT_DEFINITION_FORMATION('','',#7)")
    s.add("PRODUCT('rim-only-cap','rim-only-cap','',(#8))")
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

    def rim_circle():
        return s.add("CIRCLE('',#%d,%s)" % (s.ax2((0, 0, h), (0, 0, 1), (1, 0, 0)), num(rc)))

    if kind == "rimonly2":
        E["r1"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["B"], rim_circle()))
        E["r2"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["B"], V["A"], rim_circle()))
        rim_fwd = [("r1", "T"), ("r2", "T")]
        rim_rev = [("r2", "F"), ("r1", "F")]
    else:
        E["r"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["A"], rim_circle()))
        rim_fwd = [("r", "T")]
        rim_rev = [("r", "F")]

    faces = []
    _all_oriented = []

    def face(surf_id, loop):
        _all_oriented.extend(loop)
        oes = [s.add("ORIENTED_EDGE('',*,*,#%d,.%s.)" % (E[nm], o)) for nm, o in loop]
        el = s.add("EDGE_LOOP('',(%s))" % ",".join("#%d" % i for i in oes))
        fb = s.add("FACE_OUTER_BOUND('',#%d,.T.)" % el)
        faces.append(s.add("ADVANCED_FACE('',(#%d),#%d,.T.)" % (fb, surf_id)))

    sph = s.add("SPHERICAL_SURFACE('',#%d,%s)"
                % (s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0)), num(R)))
    # rim +u: interior toward the north pole.
    face(sph, rim_fwd)
    base = s.add("PLANE('',#%d)" % s.ax2((0, 0, h), (0, 0, -1), (1, 0, 0)))
    face(base, rim_rev)

    # --- self-check: every edge traversed exactly twice, once each way
    use = {}
    for (nm, o) in _all_oriented:
        use.setdefault(nm, []).append(o)
    bad = [(nm, v) for nm, v in use.items() if sorted(v) != ["F", "T"]]
    unused = [nm for nm in E if nm not in use]
    assert not bad, ("mis-paired edges", bad)
    assert not unused, ("unused edges", unused)

    s.lines[shell_slot] = "#%d = CLOSED_SHELL('',(%s));" % (
        shell, ",".join("#%d" % f for f in faces))
    s.lines[msb_slot] = "#%d = MANIFOLD_SOLID_BREP('',#%d);" % (msb, shell)

    lu = s.add("( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT(.MILLI.,.METRE.) )")
    pau = s.add("( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) )")
    sau = s.add("( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() )")
    unc = s.add("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-07),#%d,"
                "'distance_accuracy_value','confusion accuracy')" % lu)
    ctx = s.add("( GEOMETRIC_REPRESENTATION_CONTEXT(3) "
                "GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#%d)) "
                "GLOBAL_UNIT_ASSIGNED_CONTEXT((#%d,#%d,#%d)) "
                "REPRESENTATION_CONTEXT('Context #1','3D Context with UNIT and UNCERTAINTY') )"
                % (unc, lu, pau, sau))
    s.add("PRODUCT_RELATED_PRODUCT_CATEGORY('part',$,(#7))")
    s.lines[absr_slot] = "#10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#%d,#%d),#%d);" % (world, msb, ctx)

    head = """ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('rim-only sphere cap'),'2;1');
FILE_NAME('rim-only-cap','2026-08-29T00:00:00',('hand'),('hand'),'hand','hand','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));
ENDSEC;
DATA;
"""
    nv, ne, nf = len(V), len(E), len(faces)
    print("%-16s V=%d E=%d F=%d  V-E+F=%d" % (kind, nv, ne, nf, nv - ne + nf))
    return head + "\n".join(s.lines) + "\nENDSEC;\nEND-ISO-10303-21;\n"


B = float(sys.argv[2]) if len(sys.argv) > 2 else 0.5
hm = R * (1.0 - math.sin(B))
print("exact volume = %.9e m^3 ; sphere face area = %.9e m^2"
      % (math.pi * hm * hm * (3 * R - hm) / 3 * 1e-9, 2 * math.pi * R * hm * 1e-6))
d = sys.argv[1]
for k in ("rimonly2", "rimonly1"):
    open("%s/%s.step" % (d, k), "w").write(build(k))
print("ok")
