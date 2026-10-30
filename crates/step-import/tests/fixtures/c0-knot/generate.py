#!/usr/bin/env python3
"""Hand-authored AP214 fixtures for the C0-knot split at the import door.

Every file but half_cone is the same prism, 2 x 1 in plan (metres), whose top is a
B-spline wall over x in [0, 2], y in [0, 1]. The bottom and the four
sides are planes; the top's front and back edges (y = 0, y = 1) are
B-spline curves on those planes, and its left and right edges are lines.

    ridge_u.step   the top is degree 2 in u (along x) with a double knot
                   at the ridge x = 1, where its two halves meet at a
                   crease; its front and back edges are the matching
                   iso curves, double knot included.
    ridge_v.step   the same locus with u and v swapped, so the knot is in
                   v and the face states same_sense .F.
    half_cone.step a half frustum of a cone whose wall is a rational
                   B-spline of two quarter-circle spans joined at a
                   double knot, smooth there; its rims are CIRCLE arcs
                   with no knot, so the split places its vertices on
                   them.
    seam_edge.step the top is one smooth quadratic span with no interior
                   knot; its FRONT edge states the same parabola split at
                   its midpoint, a double knot where the curve is smooth.
                   Only the edge carrier has a C0 knot.

Regenerate with `python3 generate.py <dir>`; the outputs are committed
so the test does not need a Python interpreter.
"""
import sys


class Step:
    def __init__(self):
        self.lines = []

    def add(self, text):
        self.lines.append(text)
        return len(self.lines)

    def pt(self, p):
        return self.add("CARTESIAN_POINT('',(%s))" % fmt(p))

    def dr(self, d):
        return self.add("DIRECTION('',(%s))" % fmt(d))

    def ax2(self, origin, axis, ref):
        return self.add("AXIS2_PLACEMENT_3D('',#%d,#%d,#%d)"
                        % (self.pt(origin), self.dr(axis), self.dr(ref)))


def num(c):
    t = "%.17g" % c
    if not any(ch in t for ch in ".eE"):
        t += "."
    return t


def fmt(v):
    return ",".join(num(c) for c in v)


def ints(v):
    return ",".join("%d" % c for c in v)


def build(name, top_profile, front_profile, back_profile, swap_uv):
    """`top_profile` is (degree, [(x, z)], mults, knots) along x; the
    edge profiles have the same shape and lie at y = 0 and y = 1."""
    s = Step()
    V = {}
    pos = {
        "A": (0, 0, 0), "B": (2, 0, 0), "C": (2, 1, 0), "D": (0, 1, 0),
        "E": (0, 0, 1), "F": (2, 0, 1), "G": (2, 1, 1), "H": (0, 1, 1),
    }
    for k in "ABCDEFGH":
        V[k] = s.add("VERTEX_POINT('',#%d)" % s.pt(pos[k]))

    E = {}

    def line_edge(a, b):
        pa, pb = pos[a], pos[b]
        d = tuple(pb[i] - pa[i] for i in range(3))
        c = s.add("LINE('',#%d,#%d)" % (s.pt(pa), s.add("VECTOR('',#%d,1.)" % s.dr(d))))
        E[(a, b)] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V[a], V[b], c))

    def spline_edge(a, b, profile, y):
        deg, xz, mults, knots = profile
        pts = ",".join("#%d" % s.pt((x, y, z)) for x, z in xz)
        c = s.add("B_SPLINE_CURVE_WITH_KNOTS('',%d,(%s),.UNSPECIFIED.,.F.,.F.,(%s),(%s),.UNSPECIFIED.)"
                  % (deg, pts, ints(mults), fmt(knots)))
        E[(a, b)] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V[a], V[b], c))

    for a, b in (("A", "B"), ("B", "C"), ("C", "D"), ("D", "A"),
                 ("A", "E"), ("B", "F"), ("C", "G"), ("D", "H"),
                 ("E", "H"), ("F", "G")):
        line_edge(a, b)
    spline_edge("E", "F", front_profile, 0.0)
    spline_edge("H", "G", back_profile, 1.0)

    def uses(loop):
        out = []
        for i, a in enumerate(loop):
            b = loop[(i + 1) % len(loop)]
            if (a, b) in E:
                out.append((E[(a, b)], "T"))
            else:
                out.append((E[(b, a)], "F"))
        return out

    faces = []

    def face(surf, loop, sense="T"):
        oes = [s.add("ORIENTED_EDGE('',*,*,#%d,.%s.)" % u) for u in uses(loop)]
        el = s.add("EDGE_LOOP('',(%s))" % ",".join("#%d" % i for i in oes))
        fb = s.add("FACE_OUTER_BOUND('',#%d,.T.)" % el)
        faces.append(s.add("ADVANCED_FACE('',(#%d),#%d,.%s.)" % (fb, surf, sense)))

    def plane(origin, axis, ref):
        return s.add("PLANE('',#%d)" % s.ax2(origin, axis, ref))

    deg, xz, mults, knots = top_profile
    rows = [["#%d" % s.pt((x, y, z)) for y in (0.0, 1.0)] for x, z in xz]
    if swap_uv:
        rows = [list(r) for r in zip(*rows, strict=True)]
        surf = s.add(
            "B_SPLINE_SURFACE_WITH_KNOTS('',1,%d,(%s),.UNSPECIFIED.,.F.,.F.,.F.,(2,2),(%s),(0.,1.),(%s),.UNSPECIFIED.)"
            % (deg, ",".join("(%s)" % ",".join(r) for r in rows), ints(mults), fmt(knots)))
    else:
        surf = s.add(
            "B_SPLINE_SURFACE_WITH_KNOTS('',%d,1,(%s),.UNSPECIFIED.,.F.,.F.,.F.,(%s),(2,2),(%s),(0.,1.),.UNSPECIFIED.)"
            % (deg, ",".join("(%s)" % ",".join(r) for r in rows), ints(mults), fmt(knots)))

    face(plane((0, 0, 0), (0, 0, -1), (1, 0, 0)), "ADCB")
    # S_u x S_v is +z with u along x, and -z with the chart transposed.
    face(surf, "EFGH", "F" if swap_uv else "T")
    face(plane((0, 0, 0), (0, -1, 0), (1, 0, 0)), "ABFE")
    face(plane((0, 1, 0), (0, 1, 0), (1, 0, 0)), "DHGC")
    face(plane((0, 0, 0), (-1, 0, 0), (0, 1, 0)), "AEHD")
    face(plane((2, 0, 0), (1, 0, 0), (0, 1, 0)), "BCGF")

    return finish(s, name, faces)


def finish(s, name, faces):
    shell = s.add("CLOSED_SHELL('',(%s))" % ",".join("#%d" % f for f in faces))
    msb = s.add("MANIFOLD_SOLID_BREP('%s',#%d)" % (name, shell))
    world = s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0))
    lu = s.add("( LENGTH_UNIT() NAMED_UNIT(*) SI_UNIT($,.METRE.) )")
    pau = s.add("( NAMED_UNIT(*) PLANE_ANGLE_UNIT() SI_UNIT($,.RADIAN.) )")
    sau = s.add("( NAMED_UNIT(*) SI_UNIT($,.STERADIAN.) SOLID_ANGLE_UNIT() )")
    unc = s.add("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.E-09),#%d,"
                "'distance_accuracy_value','confusion accuracy')" % lu)
    ctx = s.add("( GEOMETRIC_REPRESENTATION_CONTEXT(3) "
                "GLOBAL_UNCERTAINTY_ASSIGNED_CONTEXT((#%d)) "
                "GLOBAL_UNIT_ASSIGNED_CONTEXT((#%d,#%d,#%d)) "
                "REPRESENTATION_CONTEXT('Context #1','3D Context with UNIT and UNCERTAINTY') )"
                % (unc, lu, pau, sau))
    rep = s.add("ADVANCED_BREP_SHAPE_REPRESENTATION('',(#%d,#%d),#%d)" % (world, msb, ctx))
    app = s.add("APPLICATION_CONTEXT('core data for automotive mechanical design processes')")
    s.add("APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#%d)" % app)
    pc = s.add("PRODUCT_CONTEXT('',#%d,'mechanical')" % app)
    prod = s.add("PRODUCT('%s','%s','',(#%d))" % (name, name, pc))
    pdf = s.add("PRODUCT_DEFINITION_FORMATION('','',#%d)" % prod)
    pdc = s.add("PRODUCT_DEFINITION_CONTEXT('part definition',#%d,'design')" % app)
    pd = s.add("PRODUCT_DEFINITION('design','',#%d,#%d)" % (pdf, pdc))
    pds = s.add("PRODUCT_DEFINITION_SHAPE('','',#%d)" % pd)
    s.add("SHAPE_DEFINITION_REPRESENTATION(#%d,#%d)" % (pds, rep))

    head = """ISO-10303-21;
HEADER;
FILE_DESCRIPTION(('C0-knot split probe'),'2;1');
FILE_NAME('%s','2026-10-29T00:00:00',('hand'),('hand'),'hand','hand','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));
ENDSEC;
DATA;
""" % name
    body = "\n".join("#%d = %s;" % (i + 1, t) for i, t in enumerate(s.lines))
    return head + body + "\nENDSEC;\nEND-ISO-10303-21;\n"

def build_half_cone(name):
    """A half frustum of a cone about z, radius 1 at z = 0 and 1/2 at
    z = 1, cut by the plane x = 0. Its wall is a rational B-spline of
    two quarter-circle spans joined at a double knot on the +x meridian;
    its rims are CIRCLE arcs, which carry no knot."""
    s = Step()
    r2 = 2 ** -0.5
    pos = {"A": (0, -1, 0), "B": (0, 1, 0), "C": (0, 0.5, 1), "D": (0, -0.5, 1)}
    V = {k: s.add("VERTEX_POINT('',#%d)" % s.pt(p)) for k, p in pos.items()}
    E = {}

    def line_edge(n, a, b):
        pa, pb = pos[a], pos[b]
        d = tuple(pb[i] - pa[i] for i in range(3))
        c = s.add("LINE('',#%d,#%d)" % (s.pt(pa), s.add("VECTOR('',#%d,1.)" % s.dr(d))))
        E[n] = (s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V[a], V[b], c)), a, b)

    def arc_edge(n, a, b, z, r):
        c = s.add("CIRCLE('',#%d,%s)" % (s.ax2((0, 0, z), (0, 0, 1), (1, 0, 0)), num(r)))
        E[n] = (s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V[a], V[b], c)), a, b)

    arc_edge("bot", "A", "B", 0.0, 1.0)
    arc_edge("top", "D", "C", 1.0, 0.5)
    line_edge("ab", "A", "B")
    line_edge("dc", "D", "C")
    line_edge("ad", "A", "D")
    line_edge("bc", "B", "C")

    faces = []

    def face(surf, loop, sense="T"):
        # `loop` is [(edge name, from vertex)]: each edge walked from there.
        oes = []
        for n, frm in loop:
            eid, a, _ = E[n]
            oes.append(s.add("ORIENTED_EDGE('',*,*,#%d,.%s.)" % (eid, "T" if frm == a else "F")))
        el = s.add("EDGE_LOOP('',(%s))" % ",".join("#%d" % i for i in oes))
        fb = s.add("FACE_OUTER_BOUND('',#%d,.T.)" % el)
        faces.append(s.add("ADVANCED_FACE('',(#%d),#%d,.%s.)" % (fb, surf, sense)))

    def plane(origin, axis, ref):
        return s.add("PLANE('',#%d)" % s.ax2(origin, axis, ref))

    ring = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1)]
    rows = [["#%d" % s.pt((x * r, y * r, z)) for r, z in ((1.0, 0.0), (0.5, 1.0))] for x, y in ring]
    w = [1.0, r2, 1.0, r2, 1.0]
    weights = ",".join("(%s)" % ",".join(num(wi) for _ in range(2)) for wi in w)
    surf = s.add(
        "( BOUNDED_SURFACE() B_SPLINE_SURFACE(2,1,(%s),.UNSPECIFIED.,.F.,.F.,.F.) "
        "B_SPLINE_SURFACE_WITH_KNOTS((3,2,3),(2,2),(0.,1.,2.),(0.,1.),.UNSPECIFIED.) "
        "GEOMETRIC_REPRESENTATION_ITEM() RATIONAL_B_SPLINE_SURFACE((%s)) "
        "REPRESENTATION_ITEM('') SURFACE() )"
        % (",".join("(%s)" % ",".join(r) for r in rows), weights))

    face(surf, [("bot", "A"), ("bc", "B"), ("top", "C"), ("ad", "D")])
    face(plane((0, 0, 0), (0, 0, -1), (1, 0, 0)), [("bot", "B"), ("ab", "A")])
    face(plane((0, 0, 1), (0, 0, 1), (1, 0, 0)), [("top", "D"), ("dc", "C")])
    face(plane((0, 0, 0), (-1, 0, 0), (0, 1, 0)), [("ad", "A"), ("dc", "D"), ("bc", "C"), ("ab", "B")])
    return finish(s, name, faces)


# Two quadratic halves meeting at a crease at x = 1: the left half
# rises with slope +0.2 into the ridge, the right falls with -0.2.
RIDGE = (2, [(0, 1), (0.5, 1.4), (1, 1.5), (1.5, 1.4), (2, 1)], (3, 2, 3), (0, 1, 2))
# One quadratic arch, and the same arch split at its midpoint by de
# Casteljau (every control point a dyadic rational, so exact).
ARCH = (2, [(0, 1), (1, 1.5), (2, 1)], (3, 3), (0, 1))
ARCH_SPLIT = (2, [(0, 1), (0.5, 1.25), (1, 1.25), (1.5, 1.25), (2, 1)], (3, 2, 3), (0, 0.5, 1))

d = sys.argv[1]
for fname, args in (
    ("ridge_u", (RIDGE, RIDGE, RIDGE, False)),
    ("ridge_v", (RIDGE, RIDGE, RIDGE, True)),
    ("seam_edge", (ARCH, ARCH_SPLIT, ARCH, False)),
):
    with open("%s/%s.step" % (d, fname), "w") as f:
        f.write(build(fname, *args))
with open("%s/half_cone.step" % d, "w") as f:
    f.write(build_half_cone("half_cone"))
print("ok")
