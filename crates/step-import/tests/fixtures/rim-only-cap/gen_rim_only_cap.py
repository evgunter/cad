#!/usr/bin/env python3
"""Hand-authored AP214 fixtures for a CHART SINGULARITY THAT IS NOT A
VERTEX.  Writes six files into the directory given as argv[1].
(Adapted from ../halfcap/gen_halfcap.py.)

Three are the cap of an R = 10 mm sphere above latitude B = 0.5 rad
(argv[2] overrides B), closed by its base disc: two faces sharing one
rim circle.

  * rimonly2.step: the rim as two half arcs and nothing else, the
    north pole in the INTERIOR of the sphere face, 2 V / 2 E / 2 F;
  * rimonly1.step: the rim as ONE closed circle edge, 1 V / 1 E / 2 F;
  * seamed.step: Open CASCADE's statement -- the closed rim plus one
    seam meridian from the rim vertex to a pole vertex of valence 1,
    used twice by the sphere face, 2 V / 2 E / 2 F;
  * spur.step: the same seam stopping short of the pole, at latitude
    1.2 rad, so the pole is still INTERIOR to the face, 2 V / 2 E / 2 F.

Two more state a singular point inside a face another way:

  * tilted.step: the cap of the same sphere cut by a plane tilted
    0.3 rad off the axis at distance 6 mm from the centre (cap height
    4 mm), the north pole interior to the cap face, the rim one closed
    circle, 1 V / 1 E / 2 F;
  * conecap.step: a cone of rim radius 5 mm and half-angle 0.5 rad,
    its apex interior to the lateral face (no seam generator), closed
    by its base disc, 1 V / 1 E / 2 F.

A chart singularity inside a face is a vertex of it, so import re-mints
each into the pole form (`NormalizationKind::ChartSingularityVertex`);
the rows reading them are ../../meridian_free_cap.rs and
../../tier_gate.rs.
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
    # The rim circle: centre, axis, reference direction, radius.
    rim = ((0.0, 0.0, h), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0), rc)
    if kind == "tilted":
        n = (math.sin(TILT), 0.0, math.cos(TILT))
        ref = (math.cos(TILT), 0.0, -math.sin(TILT))
        rim = (tuple(TILT_D * c for c in n), n, ref, math.sqrt(R * R - TILT_D * TILT_D))
    if kind == "conecap":
        rim = ((0.0, 0.0, 0.0), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0), CONE_R)
    centre, normal, ref, radius = rim
    pos = {"A": tuple(c + radius * r for c, r in zip(centre, ref, strict=True))}
    if kind == "rimonly2":
        pos["B"] = (-rc, 0.0, h)
    if kind == "seamed":
        pos["N"] = (0.0, 0.0, R)
    if kind == "spur":
        pos["N"] = (R * math.cos(SPUR), 0.0, R * math.sin(SPUR))

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
        return s.add("CIRCLE('',#%d,%s)" % (s.ax2(centre, normal, ref), num(radius)))

    if kind == "rimonly2":
        E["r1"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["B"], rim_circle()))
        E["r2"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["B"], V["A"], rim_circle()))
        rim_fwd = [("r1", "T"), ("r2", "T")]
        rim_rev = [("r2", "F"), ("r1", "F")]
    else:
        E["r"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["A"], rim_circle()))
        rim_fwd = [("r", "T")]
        rim_rev = [("r", "F")]
    seam = []
    if kind in ("seamed", "spur"):
        mc = s.add("CIRCLE('',#%d,%s)" % (s.ax2((0, 0, 0), (0, -1, 0), (1, 0, 0)), num(R)))
        E["m"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["N"], mc))
        seam = [("m", "T"), ("m", "F")]

    faces = []
    _all_oriented = []

    def face(surf_id, loop):
        _all_oriented.extend(loop)
        oes = [s.add("ORIENTED_EDGE('',*,*,#%d,.%s.)" % (E[nm], o)) for nm, o in loop]
        el = s.add("EDGE_LOOP('',(%s))" % ",".join("#%d" % i for i in oes))
        fb = s.add("FACE_OUTER_BOUND('',#%d,.T.)" % el)
        faces.append(s.add("ADVANCED_FACE('',(#%d),#%d,.T.)" % (fb, surf_id)))

    if kind == "conecap":
        # The apex lies below the rim, at z = -CONE_R / tan(CONE_A); the
        # lateral face runs from it up to the rim, which its CCW
        # boundary traverses in -u.
        cone = s.add("CONICAL_SURFACE('',#%d,%s,%s)"
                     % (s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0)), num(CONE_R), num(CONE_A)))
        face(cone, rim_rev)
        base = s.add("PLANE('',#%d)" % s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0)))
        face(base, rim_fwd)
    else:
        sph = s.add("SPHERICAL_SURFACE('',#%d,%s)"
                    % (s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0)), num(R)))
        # rim +u about its own normal: interior toward the north pole
        # (then the seam up and back).
        face(sph, [*rim_fwd, *seam])
        base = s.add("PLANE('',#%d)" % s.ax2(centre, tuple(-c for c in normal), ref))
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
SPUR = 1.2  # the spur's tip latitude (rad)
TILT, TILT_D = 0.3, 6.0  # the tilted cap's plane: tilt (rad), distance (mm)
CONE_R, CONE_A = 5.0, 0.5  # the cone cap's rim radius (mm), half-angle (rad)
hm = R * (1.0 - math.sin(B))
print("exact volume = %.9e m^3 ; sphere face area = %.9e m^2"
      % (math.pi * hm * hm * (3 * R - hm) / 3 * 1e-9, 2 * math.pi * R * hm * 1e-6))
ht = R - TILT_D
print("tilted: exact volume = %.9e m^3" % (math.pi * ht * ht * (3 * R - ht) / 3 * 1e-9))
print("conecap: exact volume = %.9e m^3"
      % (math.pi * CONE_R ** 2 * (CONE_R / math.tan(CONE_A)) / 3 * 1e-9))
d = sys.argv[1]
for k in ("rimonly2", "rimonly1", "seamed", "spur", "tilted", "conecap"):
    open("%s/%s.step" % (d, k), "w").write(build(k))
print("ok")
