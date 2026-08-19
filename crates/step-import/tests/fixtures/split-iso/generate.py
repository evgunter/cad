#!/usr/bin/env python3
"""Hand-authored AP214 fixtures for the S28 import residual.

Shape: a D-prism (half-disc cross-section, x >= 0, radius R, height H)
with a rectangular notch milled into the curved side from the top --
the keyway.  The OUTER CYLINDRICAL FACE therefore has a **U-shaped
(notched) iso domain** in (u=theta, v=z): the bbox is
[-pi/2, pi/2] x [0, H] but the sub-rectangle [t1, t2] x [z1, H] is
NOT part of the face.

`notch.step`  -- the notched solid (8 faces)
`plain.step`  -- the same D-prism with no notch (4 faces), the control
"""
import math

R, RHO, H, Z1 = 10.0, 6.0, 20.0, 12.0
T1, T2 = -0.4, 0.4
XC = 3.0                      # the flat's offset from the axis
TC = math.acos(XC / R)        # the segment's half-angle (< pi/2, so the
                              # face's u-span 2*TC stays clear of the
                              # walk's half-period tie at exactly pi)


ROT = None   # set to a 3x3 row-major tuple to place the part obliquely


def rot(v):
    if ROT is None:
        return v
    return tuple(sum(ROT[i][j] * v[j] for j in range(3)) for i in range(3))


def P(r, t, z):
    return (r * math.cos(t), r * math.sin(t), z)


class Step:
    def __init__(self):
        self.lines = []
        self.n = 0

    def add(self, text):
        self.n += 1
        self.lines.append("#%d = %s;" % (self.n, text))
        return self.n

    def pt(self, p):
        return self.add("CARTESIAN_POINT('',(%s))" % fmt(rot(p)))

    def dr(self, d):
        return self.add("DIRECTION('',(%s))" % fmt(rot(d)))

    def ax2(self, origin, axis, ref):
        return self.add("AXIS2_PLACEMENT_3D('',#%d,#%d,#%d)"
                        % (self.pt(origin), self.dr(axis), self.dr(ref)))


def num(c):
    if abs(c) < 1e-12:
        c = 0.0
    t = "%.15g" % c
    if not any(ch in t for ch in ".eE"):
        t += "."
    return t


def fmt(v):
    return ",".join(num(c) for c in v)


def build(notched, split=False):
    s = Step()
    out = []
    # --- boilerplate ------------------------------------------------
    s.add("APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2)")
    s.add("APPLICATION_CONTEXT('core data for automotive mechanical design processes')")
    sdr = s.n + 1
    s.add("SHAPE_DEFINITION_REPRESENTATION(#4,#10)")
    s.add("PRODUCT_DEFINITION_SHAPE('','',#5)")
    s.add("PRODUCT_DEFINITION('design','',#6,#9)")
    s.add("PRODUCT_DEFINITION_FORMATION('','',#7)")
    s.add("PRODUCT('s28','s28','',(#8))")
    s.add("PRODUCT_CONTEXT('',#2,'mechanical')")
    s.add("PRODUCT_DEFINITION_CONTEXT('part definition',#2,'design')")
    assert s.n == 9
    # #10 = ADVANCED_BREP_SHAPE_REPRESENTATION('',(#11,#15),#ctx) -- patched later
    s.n += 1
    absr_slot = len(s.lines)
    s.lines.append(None)
    assert s.n == 10
    world = s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0))   # #11..#14
    assert world == 14
    s.n += 1                                          # #15 MANIFOLD_SOLID_BREP
    msb_slot = len(s.lines)
    s.lines.append(None)
    msb = 15
    s.n += 1                                          # #16 CLOSED_SHELL
    shell_slot = len(s.lines)
    s.lines.append(None)
    shell = 16

    # --- vertices ---------------------------------------------------
    V = {}
    def vtx(name, p):
        V[name] = s.add("VERTEX_POINT('',#%d)" % s.pt(p))

    if split:
        vtx("M0", P(R, -TC, H / 2.0))
    vtx("P0", P(R, -TC, 0))
    vtx("P1", P(R, TC, 0))
    vtx("P2", P(R, -TC, H))
    vtx("P3", P(R, TC, H))
    if notched:
        vtx("A1", P(R, T1, H));   vtx("A2", P(R, T2, H))
        vtx("B1", P(R, T1, Z1));  vtx("B2", P(R, T2, Z1))
        vtx("C1", P(RHO, T1, H)); vtx("C2", P(RHO, T2, H))
        vtx("D1", P(RHO, T1, Z1)); vtx("D2", P(RHO, T2, Z1))

    pos = {}   # vertex name -> 3-D point, for line directions
    if split:
        pos["M0"] = P(R, -TC, H / 2.0)
    pos["P0"] = P(R, -TC, 0); pos["P1"] = P(R, TC, 0)
    pos["P2"] = P(R, -TC, H); pos["P3"] = P(R, TC, H)
    if notched:
        pos["A1"] = P(R, T1, H); pos["A2"] = P(R, T2, H)
        pos["B1"] = P(R, T1, Z1); pos["B2"] = P(R, T2, Z1)
        pos["C1"] = P(RHO, T1, H); pos["C2"] = P(RHO, T2, H)
        pos["D1"] = P(RHO, T1, Z1); pos["D2"] = P(RHO, T2, Z1)

    # --- edges ------------------------------------------------------
    E = {}
    def line_edge(name, a, b):
        pa, pb = pos[a], pos[b]
        d = [pb[i] - pa[i] for i in range(3)]
        L = math.sqrt(sum(c * c for c in d))
        d = [c / L for c in d]
        c = s.add("LINE('',#%d,#%d)" % (s.pt(pa), s.add("VECTOR('',#%d,1.)" % s.dr(d))))
        E[name] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V[a], V[b], c))

    def arc_edge(name, a, b, r, z):
        c = s.add("CIRCLE('',#%d,%s)" % (s.ax2((0, 0, z), (0, 0, 1), (1, 0, 0)), num(r)))
        E[name] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V[a], V[b], c))

    if notched:
        arc_edge("e1", "P0", "P1", R, 0)
        line_edge("e2", "P1", "P0")
        arc_edge("e3", "P2", "A1", R, H)
        line_edge("e4", "A1", "C1")
        arc_edge("e5", "C1", "C2", RHO, H)
        line_edge("e6", "C2", "A2")
        arc_edge("e7", "A2", "P3", R, H)
        line_edge("e8", "P3", "P2")
        line_edge("e9", "P0", "P2")
        line_edge("e10", "P1", "P3")
        line_edge("e11", "A1", "B1")
        arc_edge("e12", "B1", "B2", R, Z1)
        line_edge("e13", "A2", "B2")
        line_edge("e14", "B1", "D1")
        line_edge("e15", "B2", "D2")
        arc_edge("e16", "D1", "D2", RHO, Z1)
        line_edge("e17", "C1", "D1")
        line_edge("e18", "C2", "D2")
    else:
        arc_edge("e1", "P0", "P1", R, 0)
        line_edge("e2", "P1", "P0")
        arc_edge("e3", "P2", "P3", R, H)
        line_edge("e8", "P3", "P2")
        if split:
            line_edge("e9a", "P0", "M0")
            line_edge("e9b", "M0", "P2")
        else:
            line_edge("e9", "P0", "P2")
        line_edge("e10", "P1", "P3")

    # --- faces ------------------------------------------------------
    faces = []
    def face(surf_id, loop):
        oes = [s.add("ORIENTED_EDGE('',*,*,#%d,.%s.)" % (E[n], o)) for n, o in loop]
        el = s.add("EDGE_LOOP('',(%s))" % ",".join("#%d" % i for i in oes))
        fb = s.add("FACE_OUTER_BOUND('',#%d,.T.)" % el)
        faces.append(s.add("ADVANCED_FACE('',(#%d),#%d,.T.)" % (fb, surf_id)))

    def plane(origin, axis, ref):
        return s.add("PLANE('',#%d)" % s.ax2(origin, axis, ref))

    def cyl(r):
        return s.add("CYLINDRICAL_SURFACE('',#%d,%s)" % (s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0)), num(r)))

    tang = lambda t: (-math.sin(t), math.cos(t), 0.0)

    if notched:
        # F1 bottom half-disc, outward normal -z
        face(plane((0, 0, 0), (0, 0, -1), (1, 0, 0)),
             [("e1", "F"), ("e2", "F")])
        # F2 top face (half-disc minus the notch's annular sector), +z
        face(plane((0, 0, H), (0, 0, 1), (1, 0, 0)),
             [("e3", "T"), ("e4", "T"), ("e5", "T"), ("e6", "T"), ("e7", "T"), ("e8", "T")])
        # F3 chord plane x = 0, outward normal -x
        face(plane((XC, 0, 0), (-1, 0, 0), (0, 0, 1)),
             [("e9", "T"), ("e8", "F"), ("e10", "F"), ("e2", "T")])
        # F4 THE OUTER CYLINDER -- U-shaped iso domain
        face(cyl(R),
             [("e1", "T"), ("e10", "T"), ("e7", "F"), ("e13", "T"),
              ("e12", "F"), ("e11", "F"), ("e3", "F"), ("e9", "F")])
        # F5 notch floor z = Z1, outward normal +z
        face(plane((0, 0, Z1), (0, 0, 1), (1, 0, 0)),
             [("e12", "T"), ("e15", "T"), ("e16", "F"), ("e14", "F")])
        # F6 notch wall at theta = T1, outward normal +tangential(T1)
        face(plane((0, 0, 0), tang(T1), (0, 0, 1)),
             [("e17", "F"), ("e4", "F"), ("e11", "T"), ("e14", "T")])
        # F7 notch wall at theta = T2, outward normal -tangential(T2)
        face(plane((0, 0, 0), [-c for c in tang(T2)], (0, 0, 1)),
             [("e15", "F"), ("e13", "F"), ("e6", "F"), ("e18", "T")])
        # F8 notch inner cylinder r = RHO, outward normal +radial
        face(cyl(RHO),
             [("e16", "T"), ("e18", "F"), ("e5", "F"), ("e17", "T")])
    else:
        face(plane((0, 0, 0), (0, 0, -1), (1, 0, 0)), [("e1", "F"), ("e2", "F")])
        face(plane((0, 0, H), (0, 0, 1), (1, 0, 0)), [("e3", "T"), ("e8", "T")])
        chord_e9 = [("e9a", "T"), ("e9b", "T")] if split else [("e9", "T")]
        cyl_e9 = [("e9b", "F"), ("e9a", "F")] if split else [("e9", "F")]
        face(plane((XC, 0, 0), (-1, 0, 0), (0, 0, 1)),
             chord_e9 + [("e8", "F"), ("e10", "F"), ("e2", "T")])
        face(cyl(R), [("e1", "T"), ("e10", "T"), ("e3", "F")] + cyl_e9)

    s.lines[shell_slot] = "#%d = CLOSED_SHELL('',(%s));" % (
        shell, ",".join("#%d" % f for f in faces))
    s.lines[msb_slot] = "#%d = MANIFOLD_SOLID_BREP('',#%d);" % (msb, shell)

    # --- units / context --------------------------------------------
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
FILE_DESCRIPTION(('S28 import residual probe'),'2;1');
FILE_NAME('s28','2026-08-19T00:00:00',('hand'),('hand'),'hand','hand','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));
ENDSEC;
DATA;
"""
    return head + "\n".join(s.lines) + "\nENDSEC;\nEND-ISO-10303-21;\n"


import sys

d = sys.argv[1]
th = 1.0 / 3.0
ax = (0.3178, 0.9412, -0.1109)
n = math.sqrt(sum(c * c for c in ax))
ax = tuple(c / n for c in ax)
c, sn = math.cos(th), math.sin(th)
x, y, z = ax
ROTM = (
    (c + x * x * (1 - c), x * y * (1 - c) - z * sn, x * z * (1 - c) + y * sn),
    (y * x * (1 - c) + z * sn, c + y * y * (1 - c), y * z * (1 - c) - x * sn),
    (z * x * (1 - c) - y * sn, z * y * (1 - c) + x * sn, c + z * z * (1 - c)),
)

for tag, R_ in (("axis", None), ("oblique", ROTM)):
    globals()["ROT"] = R_
    open("%s/plain_%s.step" % (d, tag), "w").write(build(False))
    open("%s/split_%s.step" % (d, tag), "w").write(build(False, split=True))
print("ok")
