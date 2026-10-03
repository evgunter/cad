#!/usr/bin/env python3
"""Hand-authored AP214 fixtures for issue #653's import route.

Emits FOUR files into the directory given as argv[1], one per cell of a
2x2 grid.  Both dimensions are load-bearing and neither is optional:

                     axis-aligned          obliquely placed
    unsplit          plain_axis.step       plain_oblique.step
    split            split_axis.step       split_oblique.step

SHAPE.  A D-prism: half-disc cross-section (x >= XC, radius R, height
H), four faces -- bottom half-disc, top half-disc, the chord plane, and
the OUTER CYLINDRICAL FACE, whose iso domain in (u=theta, v=z) is the
swept rectangle [-TC, TC] x [0, H].

SPLIT.  In the `split_*` files the cylindrical face's vertical boundary
at u = -TC is stated as TWO collinear `EDGE_CURVE`s meeting at a vertex
at mid-height -- separate `LINE` carriers, adopted independently, which
is what every exporter emits when a vertex lands mid-side.  That is the
whole point of the family: the two sub-edges share no carrier identity,
so the walk cannot group them by comparing carriers and #653's sameness
test has to be geometric.

OBLIQUE.  `ROT` is a general rotation about an irrational-ish axis,
applied to every point and direction as they are written.  Axis-aligned
the two sub-edges' `atan2`s land on the same bits by luck and the mesh
comes out clean even on `main`; the oblique statement of the same part
is what turns the defect on.  The pair is the control.

Regenerate with `python3 generate.py <dir>`; the outputs are committed
so the test row does not need a Python interpreter.
"""
import math
import sys

R, H = 10.0, 20.0
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


def build(split=False):
    s = Step()
    # --- boilerplate ------------------------------------------------
    s.add("APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2)")
    s.add("APPLICATION_CONTEXT('core data for automotive mechanical design processes')")
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

    pos = {}   # vertex name -> 3-D point, for line directions
    if split:
        pos["M0"] = P(R, -TC, H / 2.0)
    pos["P0"] = P(R, -TC, 0)
    pos["P1"] = P(R, TC, 0)
    pos["P2"] = P(R, -TC, H)
    pos["P3"] = P(R, TC, H)

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

    face(plane((0, 0, 0), (0, 0, -1), (1, 0, 0)), [("e1", "F"), ("e2", "F")])
    face(plane((0, 0, H), (0, 0, 1), (1, 0, 0)), [("e3", "T"), ("e8", "T")])
    chord_e9 = [("e9a", "T"), ("e9b", "T")] if split else [("e9", "T")]
    cyl_e9 = [("e9b", "F"), ("e9a", "F")] if split else [("e9", "F")]
    face(plane((XC, 0, 0), (-1, 0, 0), (0, 0, 1)),
         [*chord_e9, ("e8", "F"), ("e10", "F"), ("e2", "T")])
    face(cyl(R), [("e1", "T"), ("e10", "T"), ("e3", "F"), *cyl_e9])

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
    open("%s/plain_%s.step" % (d, tag), "w").write(build())
    open("%s/split_%s.step" % (d, tag), "w").write(build(split=True))
print("ok")
