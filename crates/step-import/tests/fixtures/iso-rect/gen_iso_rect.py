#!/usr/bin/env python3
"""Hand-authored AP214 fixtures for the ISO-RECTANGLE PREDICATE
(S58 / #649).  Writes FOUR files into the directory given as argv[1]:
cross.step, rect.step, tee.step and xsplit.step.

The rule these fixtures are about, in the vocabulary of the fix they
were committed with: `geom_brep::props`' `props_rim_level` -- EVERY
RIM SITS AT ONE OF THE FACE'S TWO EXTREME v-LEVELS.  A domain that
satisfies it has constant u-measure w(v) = du across the whole open
v-interval, which is the premise `area = r*du*(hi-lo)` integrates.
The group-sum arithmetic quoted below is the rule that STOOD IN for
this one before the fix; it is kept because slipping past it is
exactly what these fixtures are for.

`cross.step` -- an annular-sector stack whose OUTER and INNER
cylindrical faces both have a PLUS/CROSS-shaped iso domain in
(u=theta, v=z):

    layer 1: u in [-UC, UC],  z in [0,  Z1]
    layer 2: u in [-UO, UO],  z in [Z1, Z2]     <- the arms
    layer 3: u in [-UC, UC],  z in [Z2, H ]

with UC = UO/2, i.e. the u-extent is U = 2*UO and the ARM WIDTH
(the column) is 2*UC = UO = U/2 exactly.  Every rim group then sums
to U/2:  bottom U/2; the two z=Z1 rims (UO-UC)+(UO-UC) = U/2; the two
z=Z2 rims likewise; top U/2.  That agreement is why the old span-SUM
rule accepted it and `mass_properties` returned a 19%-low volume at
pad = 0.0.  Under the level rule what refuses it is the four INTERIOR
rims, at z = Z1 and z = Z2, sitting at neither extreme.

`rect.step`   -- the CONTROL: the same annular sector with no arms
                 (u in [-UC,UC] over the full height), a genuine
                 iso-rectangle of identical du and identical v extent.
                 It must still import and still measure EXACTLY.
`tee.step`    -- the NEGATIVE CONTROL: arms on one side only in v
                 (layer 3 absent), so the group sums are U/2, U/2, U.
                 This one was ALREADY refused before the fix, by the
                 sum rule; the level rule refuses it on its interior
                 z = Z1 rims instead.  Only the REASON moved.
`xsplit.step` -- the second door, and the more alarming one: the same
                 solid as `cross`, with each cylindrical wall authored
                 as THREE RECTANGULAR sub-faces meeting on interior
                 meridians at u = +-UC over [Z1, Z2].  All three
                 reference ONE cylinder surface entity per radius, so
                 the importer gives them the same SurfaceKey.  Every
                 face is then a genuine iso-rectangle: the body
                 imports, passes tier 3 and measures EXACTLY -- and
                 the PUBLIC `Body::merge_coplanar_faces()` fuses each
                 wall back into `cross`'s plus without moving any
                 geometry.  Four inline `kind == "xsplit"` branches
                 below build it: its contour, the extra interior
                 meridian edges, the three sub-faces per wall, and the
                 two `kind != "xsplit"` guards that suppress the
                 single-face walls.
"""
import math
import sys

R, RI, H = 10.0, 6.0, 20.0          # mm
Z1, Z2 = 6.0, 14.0
UO, UC = 1.0, 0.5                   # UC = UO/2  <== the whole hypothesis


def P(r, t, z):
    return (r * math.cos(t), r * math.sin(t), z)


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
    # ---- the (u, z) lattice ------------------------------------------
    U = {"A": -UO, "B": -UC, "C": UC, "D": UO}
    if kind == "cross":
        Z = {"0": 0.0, "1": Z1, "2": Z2, "3": H}
        # (azimuth code, z code) pairs on the plus contour, CCW
        contour = [("B", "0"), ("C", "0"), ("C", "1"), ("D", "1"),
                   ("D", "2"), ("C", "2"), ("C", "3"), ("B", "3"),
                   ("B", "2"), ("A", "2"), ("A", "1"), ("B", "1")]
    elif kind == "rect":
        Z = {"0": 0.0, "3": H}
        contour = [("B", "0"), ("C", "0"), ("C", "3"), ("B", "3")]
    elif kind == "xsplit":
        Z = {"0": 0.0, "1": Z1, "2": Z2, "3": H}
        contour = [("B", "0"), ("C", "0"), ("C", "1"), ("D", "1"),
                   ("D", "2"), ("C", "2"), ("C", "3"), ("B", "3"),
                   ("B", "2"), ("A", "2"), ("A", "1"), ("B", "1")]
    elif kind == "tee":
        Z = {"0": 0.0, "1": Z1, "3": H}
        contour = [("B", "0"), ("C", "0"), ("C", "1"), ("D", "1"),
                   ("D", "3"), ("A", "3"), ("A", "1"), ("B", "1")]
    else:
        raise ValueError(kind)

    s = Step()
    s.add("APPLICATION_PROTOCOL_DEFINITION('international standard','automotive_design',2000,#2)")
    s.add("APPLICATION_CONTEXT('core data for automotive mechanical design processes')")
    s.add("SHAPE_DEFINITION_REPRESENTATION(#4,#10)")
    s.add("PRODUCT_DEFINITION_SHAPE('','',#5)")
    s.add("PRODUCT_DEFINITION('design','',#6,#9)")
    s.add("PRODUCT_DEFINITION_FORMATION('','',#7)")
    s.add("PRODUCT('xs','xs','',(#8))")
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

    # ---- vertices ----------------------------------------------------
    pos, V = {}, {}
    for side, r in (("O", R), ("I", RI)):
        for (a, z) in contour:
            nm = side + a + z
            pos[nm] = P(r, U[a], Z[z])
            V[nm] = s.add("VERTEX_POINT('',#%d)" % s.pt(pos[nm]))

    # ---- edges -------------------------------------------------------
    E = {}

    def line_edge(nm, a, b):
        pa, pb = pos[a], pos[b]
        d = [pb[i] - pa[i] for i in range(3)]
        L = math.sqrt(sum(c * c for c in d))
        d = [c / L for c in d]
        c = s.add("LINE('',#%d,#%d)" % (s.pt(pa), s.add("VECTOR('',#%d,1.)" % s.dr(d))))
        E[nm] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V[a], V[b], c))

    def arc_edge(nm, a, b, r, z):
        c = s.add("CIRCLE('',#%d,%s)" % (s.ax2((0, 0, z), (0, 0, 1), (1, 0, 0)), num(r)))
        E[nm] = s.add("EDGE_CURVE('',#%d,#%d,#%d,.T.)" % (V[a], V[b], c))

    # the contour's own edges, per side: consecutive pairs.  A pair that
    # shares the z code is an ARC (rim); one that shares the azimuth code
    # is a LINE (meridian).  Stored in increasing u / increasing z.
    rims, mers = [], []
    n = len(contour)
    for i in range(n):
        (a0, z0), (a1, z1) = contour[i], contour[(i + 1) % n]
        if z0 == z1:
            lo, hi = (a0, a1) if U[a0] < U[a1] else (a1, a0)
            rims.append((lo, hi, z0))
        else:
            lo, hi = (z0, z1) if Z[z0] < Z[z1] else (z1, z0)
            mers.append((a0, lo, hi))
    for side, r in (("O", R), ("I", RI)):
        for (lo, hi, z) in rims:
            arc_edge("a%s%s%s%s" % (side, lo, hi, z), side + lo + z, side + hi + z, r, Z[z])
        for (a, lo, hi) in mers:
            line_edge("v%s%s%s%s" % (side, a, lo, hi), side + a + lo, side + a + hi)
    if kind == "xsplit":
        # the interior meridians at u = +-UC over [Z1, Z2] -- the shared
        # edges the three rectangular sub-faces meet on, and the ones
        # `merge_coplanar_faces` would kill.
        for side in ("O", "I"):
            for a in ("B", "C"):
                line_edge("v%s%s12" % (side, a), side + a + "1", side + a + "2")

    # radial lines I -> O at every contour vertex
    for (a, z) in contour:
        line_edge("r%s%s" % (a, z), "I" + a + z, "O" + a + z)

    # ---- faces -------------------------------------------------------
    faces = []
    _all_oriented = []

    def face(surf_id, loop, sense=True):
        _all_oriented.extend(loop)
        oes = [s.add("ORIENTED_EDGE('',*,*,#%d,.%s.)" % (E[nm], o)) for nm, o in loop]
        el = s.add("EDGE_LOOP('',(%s))" % ",".join("#%d" % i for i in oes))
        fb = s.add("FACE_OUTER_BOUND('',#%d,.T.)" % el)
        faces.append(s.add("ADVANCED_FACE('',(#%d),#%d,.%s.)"
                           % (fb, surf_id, "T" if sense else "F")))

    def plane(origin, axis, ref):
        return s.add("PLANE('',#%d)" % s.ax2(origin, axis, ref))

    def cyl(r):
        return s.add("CYLINDRICAL_SURFACE('',#%d,%s)"
                     % (s.ax2((0, 0, 0), (0, 0, 1), (1, 0, 0)), num(r)))

    def tang(t):
        return (-math.sin(t), math.cos(t), 0.0)

    def radial(t):
        return (math.cos(t), math.sin(t), 0.0)

    def arc(side, lo, hi, z, fwd):
        return ("a%s%s%s%s" % (side, lo, hi, z), "T" if fwd else "F")

    def mer(side, a, lo, hi, fwd):
        return ("v%s%s%s%s" % (side, a, lo, hi), "T" if fwd else "F")

    def rad(a, z, fwd):   # fwd = I -> O
        return ("r%s%s" % (a, z), "T" if fwd else "F")

    if kind == "xsplit":
        # ONE cylinder surface entity per radius, referenced by all three
        # sub-faces, so the importer gives them the SAME SurfaceKey (the
        # merge ladder's structural rung (a)).
        so, si = cyl(R), cyl(RI)
        face(so, [("aOBC0", "T"), ("vOC01", "T"), ("vOC12", "T"), ("vOC23", "T"),
                  ("aOBC3", "F"), ("vOB23", "F"), ("vOB12", "F"), ("vOB01", "F")])
        face(so, [("aOCD1", "T"), ("vOD12", "T"), ("aOCD2", "F"), ("vOC12", "F")])
        face(so, [("aOAB1", "T"), ("vOB12", "T"), ("aOAB2", "F"), ("vOA12", "F")])
        face(si, [("vIB01", "T"), ("vIB12", "T"), ("vIB23", "T"), ("aIBC3", "T"),
                  ("vIC23", "F"), ("vIC12", "F"), ("vIC01", "F"), ("aIBC0", "F")],
             sense=False)
        face(si, [("vIC12", "T"), ("aICD2", "T"), ("vID12", "F"), ("aICD1", "F")],
             sense=False)
        face(si, [("vIA12", "T"), ("aIAB2", "T"), ("vIB12", "F"), ("aIAB1", "F")],
             sense=False)
    else:
      _skip = True
    # F: OUTER cylinder -- the contour, CCW in (u,v) (outward = +radial
    # = the chart normal, so the interior-left rule gives CCW).
    loop = []
    for i in range(n):
        (a0, z0), (a1, z1) = contour[i], contour[(i + 1) % n]
        if z0 == z1:
            loop.append(arc("O", *((a0, a1) if U[a0] < U[a1] else (a1, a0)),
                            z0, U[a0] < U[a1]))
        else:
            loop.append(mer("O", a0, *((z0, z1) if Z[z0] < Z[z1] else (z1, z0)),
                            Z[z0] < Z[z1]))
    if kind != "xsplit":
        face(cyl(R), loop)

    # F: INNER cylinder -- outward = -radial = -chart normal, so the
    # traversal is the REVERSE of the outer contour.
    loop = []
    for i in range(n, 0, -1):
        (a0, z0), (a1, z1) = contour[i % n], contour[i - 1]
        if z0 == z1:
            loop.append(arc("I", *((a0, a1) if U[a0] < U[a1] else (a1, a0)),
                            z0, U[a0] < U[a1]))
        else:
            loop.append(mer("I", a0, *((z0, z1) if Z[z0] < Z[z1] else (z1, z0)),
                            Z[z0] < Z[z1]))
    if kind != "xsplit":
        face(cyl(RI), loop, sense=False)

    # planar annular-sector faces at constant z: (lo, hi, zcode, up?)
    # `up` = the material is BELOW, so the outward normal is +z.
    shelves = []
    for (lo, hi, z) in rims:
        # material is above a rim whose face-interior lies at larger z
        i = [k for k in range(n) if contour[k] == (lo, z) or contour[k] == (hi, z)]
        # decide from the contour direction: the rim runs +u  <=>  the
        # face interior is above  <=>  the shelf's outward normal is -z.
        runs_plus = any(contour[k] == (lo, z) and contour[(k + 1) % n] == (hi, z)
                        for k in range(n))
        shelves.append((lo, hi, z, not runs_plus))
    for (lo, hi, z, up) in shelves:
        ax = (0, 0, 1) if up else (0, 0, -1)
        if up:   # CCW seen from above: outer arc +u, inner arc -u
            loop = [arc("O", lo, hi, z, True), rad(hi, z, False),
                    arc("I", lo, hi, z, False), rad(lo, z, True)]
        else:    # CW seen from above
            loop = [arc("O", lo, hi, z, False), rad(lo, z, False),
                    arc("I", lo, hi, z, True), rad(hi, z, True)]
        face(plane((0, 0, Z[z]), ax, (1, 0, 0)), loop)

    # planar radial faces at constant u, one per meridian
    for (a, lo, hi) in mers:
        t = U[a]
        # the meridian runs +z  <=>  interior is at LARGER u  <=>  the
        # face's outward normal is -tangential.
        runs_plus = any(contour[k] == (a, lo) and contour[(k + 1) % n] == (a, hi)
                        for k in range(n))
        # CCW in (u,v) on the OUTER wall: the meridian at the LARGER-u
        # side runs +v and the interior is at smaller u, so runs_plus
        # <=> outward = +tangential.
        outward = tang(t) if runs_plus else tuple(-c for c in tang(t))
        if runs_plus:       # outward = +tang: CW in (r, z)
            loop = [mer("I", a, lo, hi, True), rad(a, hi, True),
                    mer("O", a, lo, hi, False), rad(a, lo, False)]
        else:               # outward = -tang: CCW in (r, z)
            loop = [rad(a, lo, True), mer("O", a, lo, hi, True),
                    rad(a, hi, False), mer("I", a, lo, hi, False)]
        face(plane(P(0, t, 0), outward, (0, 0, 1)), loop)

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
FILE_DESCRIPTION(('du-sum-rule probe'),'2;1');
FILE_NAME('xs','2026-08-19T00:00:00',('hand'),('hand'),'hand','hand','');
FILE_SCHEMA(('AUTOMOTIVE_DESIGN { 1 0 10303 214 1 1 1 1 }'));
ENDSEC;
DATA;
"""
    nv, ne, nf = len(V), len(E), len(faces)
    print("%-6s V=%d E=%d F=%d  V-E+F=%d" % (kind, nv, ne, nf, nv - ne + nf))
    return head + "\n".join(s.lines) + "\nENDSEC;\nEND-ISO-10303-21;\n"


d = sys.argv[1]
for k in ("cross", "rect", "tee", "xsplit"):
    open("%s/%s.step" % (d, k), "w").write(build(k))
print("ok")
