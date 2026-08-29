#!/usr/bin/env python3
"""Hand-authored AP214 fixtures for the SPHERE POLAR EXTENT (issue
723).  Writes TWO files into the directory given as argv[1]:
halfcap.step and halfcap_nosplit.step.

Re-derived from the issue's text (the original artifacts died with
their machine): literally HALF OF A SPHERICAL CAP -- the cap of a
R = 10 mm sphere above latitude B = 0.5 rad, cut in half by the
plane y = 0, keeping y >= 0.  Three faces:

  * the spherical half-cap, chart domain [0, pi] x [B, pi/2]: one rim
    half-circle at latitude B and ONE MERIDIAN GREAT-CIRCLE ARC from
    (u=0, v=B) over the north pole down to (u=pi, v=B) -- the arc
    whose latitude reaches +1 in its INTERIOR;
  * the base half-disk in the plane z = R sin B (outward -z);
  * the cut half-plane face in y = 0 (outward -y), bounded by the
    meridian arc and the base diameter.

`halfcap.step` splits the meridian arc by one ORDINARY vertex at
t = 1.0 rad (3 V / 4 E / 3 F, chi = 2).  That single vertex is what
made the wrong number reachable: with it, the endpoint fold saw
latitudes {sin B, sin 1} and the sphere face was ACCEPTED at
R^2*pi*(sin 1 - sin B) -- volume -47.187% through the import door at
tier-3 green.  `halfcap_nosplit.step` is the identical solid with the
arc as ONE edge (2 V / 3 E / 3 F, chi = 2); its endpoint latitudes
coincide, so the endpoint fold refused it degenerate.  Under the
span-derived extent both certify the same exact numbers:

    volume = pi*h^2*(3R - h)/6,  h = R*(1 - sin B)   (~3.5182e-7 m^3)
    sphere face area = R^2*pi*(1 - sin B)
"""
import math
import sys

R = 10.0            # mm
B = 0.5             # base latitude, rad
SPLIT = 1.0         # the ordinary split vertex's latitude parameter


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
    # Vertices on the meridian great circle in the xz-plane,
    # parameterized by latitude t on the +x side (axis (0,-1,0),
    # ref (1,0,0)): P(t) = (R cos t, 0, R sin t).
    pos = {
        "A": (rc, 0.0, h),                                    # (u=0, v=B)
        "B": (-rc, 0.0, h),                                   # (u=pi, v=B)
        "C": (R * math.cos(SPLIT), 0.0, R * math.sin(SPLIT)),
    }

    s = Step()
    s.add("APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2)")
    s.add("APPLICATION_CONTEXT('core data for automotive mechanical design processes')")
    s.add("SHAPE_DEFINITION_REPRESENTATION(#4,#10)")
    s.add("PRODUCT_DEFINITION_SHAPE('','',#5)")
    s.add("PRODUCT_DEFINITION('design','',#6,#9)")
    s.add("PRODUCT_DEFINITION_FORMATION('','',#7)")
    s.add("PRODUCT('hc','hc','',(#8))")
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
    # The rim half-circle at latitude B: A -> B CCW about +z (through
    # +y), azimuth 0 -> pi.
    rim_c = s.add("CIRCLE('',#%d,%s)" % (s.ax2((0, 0, h), (0, 0, 1), (1, 0, 0)), num(rc)))
    E["rim"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["B"], rim_c))

    # The meridian great circle in the plane y = 0 (its parameter is
    # the latitude on the +x side; the pole sits at t = pi/2, INSIDE
    # every arc authored below).
    def meridian_circle():
        return s.add("CIRCLE('',#%d,%s)"
                     % (s.ax2((0, 0, 0), (0, -1, 0), (1, 0, 0)), num(R)))

    if kind == "halfcap":
        E["m1"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["C"], meridian_circle()))
        E["m2"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["C"], V["B"], meridian_circle()))
        mer_fwd = [("m1", "T"), ("m2", "T")]          # A -> C -> B
        mer_rev = [("m2", "F"), ("m1", "F")]          # B -> C -> A
    elif kind == "halfcap_nosplit":
        del V["C"]
        E["m"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["B"], meridian_circle()))
        mer_fwd = [("m", "T")]
        mer_rev = [("m", "F")]
    else:
        raise ValueError(kind)

    # The base diameter: A -> B along -x at z = h.
    line = s.add("LINE('',#%d,#%d)"
                 % (s.pt(pos["A"]), s.add("VECTOR('',#%d,1.)" % s.dr((-1, 0, 0)))))
    E["d"] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V["A"], V["B"], line))

    faces = []
    _all_oriented = []

    def face(surf_id, loop):
        _all_oriented.extend(loop)
        oes = [s.add("ORIENTED_EDGE('',*,*,#%d,.%s.)" % (E[nm], o)) for nm, o in loop]
        el = s.add("EDGE_LOOP('',(%s))" % ",".join("#%d" % i for i in oes))
        fb = s.add("FACE_OUTER_BOUND('',#%d,.T.)" % el)
        faces.append(s.add("ADVANCED_FACE('',(#%d),#%d,.T.)" % (fb, surf_id)))

    # Sphere face: outward = the sphere normal, CCW in the chart --
    # rim +u (A -> B through +y), meridian back over the pole.
    sph = s.add("SPHERICAL_SURFACE('',#%d,%s)"
                % (s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0)), num(R)))
    face(sph, [("rim", "T"), *mer_rev])

    # Base half-disk at z = h, outward -z (material above): CCW about
    # -z is rim reversed (B -> A through +y), then the diameter A -> B.
    base = s.add("PLANE('',#%d)" % s.ax2((0, 0, h), (0, 0, -1), (1, 0, 0)))
    face(base, [("rim", "F"), ("d", "T")])

    # Cut half-plane at y = 0, outward -y (material at y > 0): CCW
    # about -y is the meridian A -> pole -> B, then the diameter back.
    cut = s.add("PLANE('',#%d)" % s.ax2((0, 0, 0), (0, -1, 0), (1, 0, 0)))
    face(cut, [*mer_fwd, ("d", "F")])

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
FILE_DESCRIPTION(('sphere polar extent probe'),'2;1');
FILE_NAME('hc','2026-08-29T00:00:00',('hand'),('hand'),'hand','hand','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));
ENDSEC;
DATA;
"""
    nv, ne, nf = len(V), len(E), len(faces)
    print("%-16s V=%d E=%d F=%d  V-E+F=%d" % (kind, nv, ne, nf, nv - ne + nf))
    return head + "\n".join(s.lines) + "\nENDSEC;\nEND-ISO-10303-21;\n"


hm = R * (1.0 - math.sin(B))
print("exact volume  = %.9e mm^3 = %.9e m^3"
      % (math.pi * hm * hm * (3 * R - hm) / 6,
         math.pi * hm * hm * (3 * R - hm) / 6 * 1e-9))
d = sys.argv[1]
for k in ("halfcap", "halfcap_nosplit"):
    open("%s/%s.step" % (d, k), "w").write(build(k))
print("ok")
