#!/usr/bin/env python3
"""Hand-authored AP214 fixture for issue 896's pole guard: a sphere
face whose boundary EXCLUDES the pole while two of its declared
vertices sit within the default tolerance band of it.  Writes
polefrustum.step into the directory given as argv[1].

The solid is the halfcap family's sibling (same R = 10 mm sphere,
same base latitude B = 0.5 rad, same y = 0 cut keeping y >= 0) with
the cap TRUNCATED at latitude T = pi/2 - t, t = 9e-8 rad -- a half
spherical frustum.  Four faces:

  * the spherical half-band, chart domain [0, pi] x [B, T]: base rim
    half-circle at latitude B, meridian arc up the +x side, TOP rim
    half-circle at latitude T (radius R*sin(t) ~= 9e-7 mm), meridian
    arc down the -x side.  The north pole is NOT on the boundary and
    carries no vertex: an UNDECLARED chart pole.
  * the base half-disk at z = R sin B (outward -z);
  * the top half-disk at z = R sin T (outward +z) -- the nano flat
    that replaces the pole;
  * the cut half-plane in y = 0 (outward -y).

The two top-rim vertices C (u=0) and D (u=pi) sit at chord distance
2R sin(t/2) ~= R*t = 9e-7 mm = 0.9e-9 m from the pole -- INSIDE the
default eps = 1e-9 m band -- while their mutual separation is
2R sin t ~= 1.8e-9 m, OUTSIDE it.  The walk never sees any of this:
the import door refuses both twins at every suite band (measured --
span escalation at 1e-9, zero-span at 1e-6, and at 1e-12, where the
spans certify, the near-tangent rim/sphere contact refuses at
adoption).  The step-import row `poleguard.rs` holds the route
argument; `tier_gate.rs` pins the three bands cell by cell.
poleband_eps12.step is the same band form with t = 9e-11 rad
(vertex 0.9e-12 m from the pole), so the 1e-12 band also gets a
fixture whose near-pole feature is INSIDE it.
"""
import math
import sys

R = 10.0            # mm
B = 0.5             # base latitude, rad
T_OFF = 9.0e-8      # polar offset t of the top rim, rad
T = math.pi / 2.0 - T_OFF


def num(c):
    if abs(c) < 1e-12:
        c = 0.0
    t = "%.17g" % c
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


def build():
    h = R * math.sin(B)
    rc = R * math.cos(B)
    zt = R * math.sin(T)
    rho = R * math.cos(T)          # ~= R * T_OFF = 9e-7 mm
    # Vertices on the meridian great circle in the xz-plane,
    # parameterized by latitude on the +x side: P(t) = (R cos t, 0,
    # R sin t); the -x side is latitude pi - t on the same circle.
    pos = {
        "A": (rc, 0.0, h),         # (u=0,  v=B)
        "B": (-rc, 0.0, h),        # (u=pi, v=B)
        "C": (rho, 0.0, zt),       # (u=0,  v=T)
        "D": (-rho, 0.0, zt),      # (u=pi, v=T)
    }

    s = Step()
    s.add("APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2)")
    s.add("APPLICATION_CONTEXT('core data for automotive mechanical design processes')")
    s.add("SHAPE_DEFINITION_REPRESENTATION(#4,#10)")
    s.add("PRODUCT_DEFINITION_SHAPE('','',#5)")
    s.add("PRODUCT_DEFINITION('design','',#6,#9)")
    s.add("PRODUCT_DEFINITION_FORMATION('','',#7)")
    s.add("PRODUCT('pf','pf','',(#8))")
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
    # Rims: CCW about +z (azimuth 0 -> pi through +y).
    rim_c = s.add("CIRCLE('',#%d,%s)" % (s.ax2((0, 0, h), (0, 0, 1), (1, 0, 0)), num(rc)))
    E["rim"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["B"], rim_c))
    trim_c = s.add("CIRCLE('',#%d,%s)" % (s.ax2((0, 0, zt), (0, 0, 1), (1, 0, 0)), num(rho)))
    E["trim"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["C"], V["D"], trim_c))

    # Meridian arcs on the great circle in y = 0 (axis (0,-1,0), ref
    # (1,0,0): parameter IS the +x-side latitude, so both arcs run
    # forward): A -> C at latitudes B -> T, D -> B at pi-T -> pi-B.
    def meridian_circle():
        return s.add("CIRCLE('',#%d,%s)"
                     % (s.ax2((0, 0, 0), (0, -1, 0), (1, 0, 0)), num(R)))

    E["m1"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["C"], meridian_circle()))
    E["m2"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["D"], V["B"], meridian_circle()))

    # Diameters, both along -x: base A -> B, top C -> D.
    base_line = s.add("LINE('',#%d,#%d)"
                      % (s.pt(pos["A"]), s.add("VECTOR('',#%d,1.)" % s.dr((-1, 0, 0)))))
    E["d"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["B"], base_line))
    top_line = s.add("LINE('',#%d,#%d)"
                     % (s.pt(pos["C"]), s.add("VECTOR('',#%d,1.)" % s.dr((-1, 0, 0)))))
    E["dt"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["C"], V["D"], top_line))

    faces = []
    _all_oriented = []

    def face(surf_id, loop):
        _all_oriented.extend(loop)
        oes = [s.add("ORIENTED_EDGE('',*,*,#%d,.%s.)" % (E[nm], o)) for nm, o in loop]
        el = s.add("EDGE_LOOP('',(%s))" % ",".join("#%d" % i for i in oes))
        fb = s.add("FACE_OUTER_BOUND('',#%d,.T.)" % el)
        faces.append(s.add("ADVANCED_FACE('',(#%d),#%d,.T.)" % (fb, surf_id)))

    # Sphere half-band: CCW in the chart seen from outside -- base rim
    # +u (A -> B through +y), up the -x meridian (B -> D), top rim -u
    # (D -> C), down the +x meridian (C -> A).
    sph = s.add("SPHERICAL_SURFACE('',#%d,%s)"
                % (s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0)), num(R)))
    face(sph, [("rim", "T"), ("m2", "F"), ("trim", "F"), ("m1", "F")])

    # Base half-disk at z = h, outward -z: rim reversed, then the base
    # diameter A -> B.
    base = s.add("PLANE('',#%d)" % s.ax2((0, 0, h), (0, 0, -1), (1, 0, 0)))
    face(base, [("rim", "F"), ("d", "T")])

    # Top half-disk at z = zt, outward +z: top rim C -> D through +y,
    # then the top diameter back.
    top = s.add("PLANE('',#%d)" % s.ax2((0, 0, zt), (0, 0, 1), (1, 0, 0)))
    face(top, [("trim", "T"), ("dt", "F")])

    # Cut half-plane at y = 0, outward -y: up the +x meridian, top
    # diameter C -> D, down the -x meridian, base diameter back.
    cut = s.add("PLANE('',#%d)" % s.ax2((0, 0, 0), (0, -1, 0), (1, 0, 0)))
    face(cut, [("m1", "T"), ("dt", "T"), ("m2", "T"), ("d", "F")])

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
FILE_DESCRIPTION(('undeclared-pole guard probe'),'2;1');
FILE_NAME('pf','2026-09-01T00:00:00',('hand'),('hand'),'hand','hand','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));
ENDSEC;
DATA;
"""
    nv, ne, nf = len(V), len(E), len(faces)
    print("polefrustum      V=%d E=%d F=%d  V-E+F=%d" % (nv, ne, nf, nv - ne + nf))
    d_pole = 2.0 * R * math.sin(T_OFF / 2.0)
    sep = 2.0 * rho
    print("junction-to-pole chord  = %.6e mm = %.6e m" % (d_pole, d_pole * 1e-3))
    print("C-D separation          = %.6e mm = %.6e m" % (sep, sep * 1e-3))
    return head + "\n".join(s.lines) + "\nENDSEC;\nEND-ISO-10303-21;\n"


def build_full(t_off, label):
    """The FULL frustum, seam-authored: two vertices, three edges (two
    full circles and one seam meridian), three faces.  No straight
    nano edge: the half frustum's 1.8e-9 m top diameter LINE certifies
    its parameter span in METRES and lands in the K band's
    indeterminate zone (ParamSpan / interval_span_forward escalation),
    refusing the whole solid at default eps -- a circle's span is
    ANGULAR, so the full-circle authoring is the route that can reach
    the mesh walk at all."""
    t_full = math.pi / 2.0 - t_off
    h = R * math.sin(B)
    rc = R * math.cos(B)
    zt = R * math.sin(t_full)
    rho = R * math.cos(t_full)
    pos = {
        "A": (rc, 0.0, h),         # base rim at u=0
        "C": (rho, 0.0, zt),       # top rim at u=0
    }

    s = Step()
    s.add("APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2)")
    s.add("APPLICATION_CONTEXT('core data for automotive mechanical design processes')")
    s.add("SHAPE_DEFINITION_REPRESENTATION(#4,#10)")
    s.add("PRODUCT_DEFINITION_SHAPE('','',#5)")
    s.add("PRODUCT_DEFINITION('design','',#6,#9)")
    s.add("PRODUCT_DEFINITION_FORMATION('','',#7)")
    s.add("PRODUCT('pb','pb','',(#8))")
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
    # Full rims, CCW about +z, closed at their u=0 vertex.
    rim_c = s.add("CIRCLE('',#%d,%s)" % (s.ax2((0, 0, h), (0, 0, 1), (1, 0, 0)), num(rc)))
    E["rim"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["A"], rim_c))
    trim_c = s.add("CIRCLE('',#%d,%s)" % (s.ax2((0, 0, zt), (0, 0, 1), (1, 0, 0)), num(rho)))
    E["trim"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["C"], V["C"], trim_c))
    # The seam meridian A -> C at u=0, on the y=0 great circle
    # (parameter = +x-side latitude, forward).
    seam_c = s.add("CIRCLE('',#%d,%s)"
                   % (s.ax2((0, 0, 0), (0, -1, 0), (1, 0, 0)), num(R)))
    E["seam"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["C"], seam_c))

    faces = []
    _all_oriented = []

    def face(surf_id, loop):
        _all_oriented.extend(loop)
        oes = [s.add("ORIENTED_EDGE('',*,*,#%d,.%s.)" % (E[nm], o)) for nm, o in loop]
        el = s.add("EDGE_LOOP('',(%s))" % ",".join("#%d" % i for i in oes))
        fb = s.add("FACE_OUTER_BOUND('',#%d,.T.)" % el)
        faces.append(s.add("ADVANCED_FACE('',(#%d),#%d,.T.)" % (fb, surf_id)))

    # Sphere band, ONE loop with the seam walked both ways: base rim
    # +u (A -> A), seam up (A -> C), top rim -u (C -> C), seam down
    # (C -> A).
    sph = s.add("SPHERICAL_SURFACE('',#%d,%s)"
                % (s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0)), num(R)))
    face(sph, [("rim", "T"), ("seam", "T"), ("trim", "F"), ("seam", "F")])

    # Base full disk at z = h, outward -z: rim reversed is CCW about -z.
    base = s.add("PLANE('',#%d)" % s.ax2((0, 0, h), (0, 0, -1), (1, 0, 0)))
    face(base, [("rim", "F")])

    # Top nano disk at z = zt, outward +z: trim forward is CCW about +z.
    top = s.add("PLANE('',#%d)" % s.ax2((0, 0, zt), (0, 0, 1), (1, 0, 0)))
    face(top, [("trim", "T")])

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
FILE_DESCRIPTION(('undeclared-pole guard probe, seam-authored'),'2;1');
FILE_NAME('pb','2026-09-01T00:00:00',('hand'),('hand'),'hand','hand','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));
ENDSEC;
DATA;
"""
    nv, ne, nf = len(V), len(E), len(faces)
    print("%-16s V=%d E=%d F=%d  V-E+F=%d" % (label, nv, ne, nf, nv - ne + nf))
    d_pole = 2.0 * R * math.sin(t_off / 2.0)
    print("  vertex-to-pole chord = %.6e mm = %.6e m" % (d_pole, d_pole * 1e-3))
    return head + "\n".join(s.lines) + "\nENDSEC;\nEND-ISO-10303-21;\n"


d = sys.argv[1]
open("%s/polefrustum.step" % d, "w").write(build())
open("%s/poleband.step" % d, "w").write(build_full(T_OFF, "poleband"))
open("%s/poleband_eps12.step" % d, "w").write(build_full(9.0e-11, "poleband_eps12"))
print("ok")
