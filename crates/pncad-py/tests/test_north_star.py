"""Verification for the north-star audit's YES rows (LIB-U10).

`docs/guide/north-star-audit.md` claims that a handful of demo-tour
scenes are authorable through the Python bindings today. A claim like
that rots the moment the tour or the bindings move, so it is not left
as prose: every YES row is built here, in Python, and checked against
the same exact volume oracle the Rust scene asserts.

A row that stops being true fails this file. A gap that CLOSES makes
`test_the_named_gaps_are_still_gaps` fail, which is the intended
prompt to move a NO row to YES.
"""

import math
import unittest
from pathlib import Path

from pncad import (
    ArcSweep,
    BooleanOp,
    Doc,
    DocEdit,
    DocParam,
    Node,
    Open,
    ParamName,
    SketchPlane,
    Start,
    circle,
    circle_split,
    deg,
    evaluate,
    load,
    m,
    rad,
)


def slab(doc, x, y, z):
    """The axis-aligned box [x0,x1] x [y0,y1] x [z0,z1], in metres."""
    profile = doc.insert(
        Node.polygon(
            [
                (x[0] * m, y[0] * m),
                (x[1] * m, y[0] * m),
                (x[1] * m, y[1] * m),
                (x[0] * m, y[1] * m),
            ],
            elevation=z[0] * m,
        )
    )
    return doc.insert(Node.extrude(profile, (z[1] - z[0]) * m))


def volume_of(doc, node):
    ev = evaluate(doc)
    assert ev.succeeded(node), "the scene evaluated"
    body = ev.value(node).body()
    body.validate()
    return body.mass_properties().volume


class TestChute(unittest.TestCase):
    """Tour scene `chute` (demos/tour/src/bodies.rs): a C-channel
    revolved 270 degrees about the world y axis. One profile, one op,
    no booleans — the cleanest authorable scene in the tour."""

    def test_chute_matches_the_scene_oracle(self):
        poly = [
            (1.0, 0.0), (1.75, 0.0), (1.75, 0.625), (1.5625, 0.625),
            (1.5625, 0.1875), (1.1875, 0.1875), (1.1875, 0.625), (1.0, 0.625),
        ]
        doc = Doc()
        profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in poly]))
        axis = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0)))
        chute = doc.insert(Node.revolve(profile, axis, 270 * deg))

        expected = (1287.0 / 2048.0) * math.pi
        self.assertAlmostEqual(volume_of(doc, chute), expected, delta=1e-12)


class TestDie(unittest.TestCase):
    """Tour scene `die` (demos/tour/src/bool_bodies.rs): a cube less 21
    pip pockets, 21 sequential undeclared subtracts. Every cutter
    overshoots its face and is strictly interior in the other two axes,
    so no operand pair shares a plane and the undeclared lane is legal."""

    def test_die_matches_the_exact_dyadic_oracle(self):
        h, inn, out = 0.125, (0.875, 1.5), (-1.5, -0.875)
        n, z, p = -0.5, 0.0, 0.5
        doc = Doc()
        body = slab(doc, (-1, 1), (-1, 1), (-1, 1))

        faces = [
            ([(z, z)], lambda a, b: ((a - h, a + h), (b - h, b + h), inn)),
            ([(n, n), (n, z), (n, p), (p, n), (p, z), (p, p)],
             lambda a, b: ((a - h, a + h), (b - h, b + h), out)),
            ([(n, n), (p, p)], lambda a, b: (inn, (a - h, a + h), (b - h, b + h))),
            ([(n, n), (n, p), (p, n), (p, p), (z, z)],
             lambda a, b: (out, (a - h, a + h), (b - h, b + h))),
            ([(n, n), (z, z), (p, p)], lambda a, b: ((a - h, a + h), inn, (b - h, b + h))),
            ([(n, n), (n, p), (p, n), (p, p)],
             lambda a, b: ((a - h, a + h), out, (b - h, b + h))),
        ]

        pips = 0
        for centres, box in faces:
            for a, b in centres:
                cutter = slab(doc, *box(a, b))
                body = doc.insert(Node.boolean(BooleanOp.Subtract, body, cutter))
                pips += 1

        self.assertEqual(pips, 21)
        expected = 8.0 - pips * 0.25 * 0.25 * 0.125
        self.assertEqual(expected, 7.8359375)
        self.assertAlmostEqual(volume_of(doc, body), expected, delta=1e-12)


class TestProjectbox(unittest.TestCase):
    """Tour scene `projectbox` (demos/tour/src/projectbox.rs): the
    longest boolean chain in the tour, 15 ops over 16 boxes. Its own
    design rule — no two operand planes coincide anywhere in the chain,
    every offset in 1/16 steps — is exactly what makes it authorable
    without a declaration door."""

    def test_projectbox_matches_the_exact_dyadic_oracle(self):
        doc = Doc()
        body = slab(doc, (0, 3), (0, 2), (0, 1.5))
        body = doc.insert(
            Node.boolean(BooleanOp.Subtract, body, slab(doc, (0.25, 2.75), (0.25, 1.75), (0.25, 2.0)))
        )
        for x in [(0.5, 0.875), (1.3125, 1.6875), (2.125, 2.5)]:
            for y in [(-0.25, 0.5), (1.5, 2.25)]:
                body = doc.insert(
                    Node.boolean(BooleanOp.Subtract, body, slab(doc, x, y, (0.5, 1.25)))
                )
        bx = [(0.4375, 0.8125), (2.1875, 2.5625)]
        by = [(0.4375, 0.8125), (1.1875, 1.5625)]
        for x in bx:
            for y in by:
                body = doc.insert(
                    Node.boolean(BooleanOp.Union, body, slab(doc, x, y, (0.1875, 0.875)))
                )
        for x in bx:
            for y in by:
                px = (x[0] + 0.09375, x[1] - 0.09375)
                py = (y[0] + 0.09375, y[1] - 0.09375)
                body = doc.insert(
                    Node.boolean(BooleanOp.Subtract, body, slab(doc, px, py, (0.5625, 1.0625)))
                )

        # The scene's own running oracle, term for term:
        #   9 - 2.5*1.5*1.25                     the cavity
        #   - 6 * 0.375*0.25*0.75                the vent slots
        #   + 4 * 0.375*0.375*0.625              the bosses
        #   - 4 * 0.1875*0.1875*0.3125           the pilot pockets
        expected = (
            9.0
            - 2.5 * 1.5 * 1.25
            - 6 * 0.375 * 0.25 * 0.75
            + 4 * 0.375 * 0.375 * 0.625
            - 4 * 0.1875 * 0.1875 * 0.3125
        )
        self.assertEqual(expected, 4.1982421875)
        self.assertAlmostEqual(volume_of(doc, body), expected, delta=1e-12)


class TestHeatsink(unittest.TestCase):
    """Tour scenes `heatsink5/7/9` — authorable only as YES*: the BODY
    is reproducible by hand-authoring each fin, but the scene's actual
    point (one recipe, a LinearPattern node, a structural-param count
    edit 5->7->9, memoized recompute) is unreachable. There is no
    pattern node and no parameter edit in the Python surface."""

    def build(self, fins):
        doc = Doc()
        body = slab(doc, (0, 3), (0, 1), (0, 0.25))
        for i in range(fins):
            dx = i * 0.3125
            fin = slab(doc, (0.25 + dx, 0.4375 + dx), (0.125, 0.875), (0.1875, 1.0))
            body = doc.insert(Node.boolean(BooleanOp.Union, body, fin))
        return volume_of(doc, body)

    def test_each_fin_count_matches_the_scene_oracle(self):
        for fins in (5, 7, 9):
            with self.subTest(fins=fins):
                self.assertAlmostEqual(
                    self.build(fins), 0.75 + fins * 0.10546875, delta=1e-12
                )


class TestPlateParam(unittest.TestCase):
    """Audit gap G10, CLOSED (R1-PARAMS): named document parameters.

    The corpus' parametric flagship `plate_param` — a plate whose two
    hole radii are ONE `DocParam` — driven from Python: the
    `set_doc_param` edit is authored here with the bound
    `ParamName`/`DocParam` vocabulary, and the result is checked
    against the same analytic oracle the Rust acceptance rows assert
    (`crates/editor-core/tests/switch_plate_param.rs`).

    Stated honestly: the DOCUMENT arrives through the persistence
    door, because plate_param's profile (three loops, two of them
    circles) is still behind gaps G1 and G9. The fixture cannot rot —
    `crates/pncad/tests/all.rs` re-authors the scene façade-only and
    pins the saved text line for line (all but the snapshot's epsilon
    line, which CI's tolerance sweep varies by design)."""

    FIXTURE = (
        Path(__file__).resolve().parents[3]
        / "crates" / "pncad" / "tests" / "plate_param.v6.pncad"
    )

    def plate(self):
        doc = load(self.FIXTURE.read_text(encoding="utf-8")).doc
        return doc, doc.order()[-1]  # the union is the last insert

    # Plate + tab − their overlap − two cylinders of radius r: the
    # closed form the Rust rows assert, tab included.
    @staticmethod
    def oracle(r):
        return 4.0 * 2.0 * 0.5 + 1.0 * 0.75 * 0.25 - 0.5 * 0.25 * 0.25 \
            - 2.0 * math.pi * r * r * 0.5

    def test_one_param_edit_moves_both_holes_to_the_scene_oracle(self):
        for r in (0.25, 0.4):
            with self.subTest(hole_r=r):
                doc, solid = self.plate()
                doc.apply(
                    DocEdit.set_doc_param(
                        ParamName("hole_r"), DocParam.length(r * m)
                    )
                )
                self.assertAlmostEqual(
                    volume_of(doc, solid), self.oracle(r), delta=1e-6
                )

    def test_the_edit_is_legal_at_rest_and_replay_refuses_r_zero(self):
        """The acceptance suite's deliberate asymmetry: `set_doc_param`
        itself applies cleanly even for a refusing value — the refusal
        belongs to REPLAY, which names the profile node."""
        doc, solid = self.plate()
        doc.apply(
            DocEdit.set_doc_param(ParamName("hole_r"), DocParam.length(0 * m))
        )
        ev = evaluate(doc)
        self.assertFalse(ev.succeeded(solid))
        profile = doc.order()[0]  # plate_param's profile is inserted first
        import pncad

        with self.assertRaises(pncad.EvaluationError) as ctx:
            ev.value(profile)
        self.assertEqual(ctx.exception.reason, "node_failed")
        self.assertEqual(ctx.exception.kind, "profile_replay")


# ------------------------------------------------------------------
# The rows G1 unblocked (LIB-PYG1). Each rebuilds the scene from the
# PATHS lattice — the same verbs, the same authored numbers as the
# Rust source — and asserts the scene's own oracle.
# ------------------------------------------------------------------


def y_axis(doc):
    return doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0)))


class TestBracket(unittest.TestCase):
    """Tour scene `bracket` (demos/tour/src/bodies.rs, row 1): an L
    outline with one r = 0.5 inner fillet, extruded 0.75.

    The Rust scene asserts no closed form — the tour's generic ladder
    (validate, tessellate, mesh-vs-mass-properties) is all it gets —
    so the oracle here is derived and stated: the L's area is 5, and
    rounding the reflex corner ADDS the region between the corner and
    the arc, r^2 - pi*r^2/4.

    `toward` rather than `angle(PI)`: only the ratio of the components
    carries meaning, so the unit ray is stored verbatim and the two
    trim vertices are exact — `sin(PI)` is 1.22e-16, and it would
    perturb both by an ulp."""

    def test_bracket_matches_the_derived_closed_form(self):
        outline = (
            Open.at((0 * m, 0 * m))
            .line_to((3 * m, 0 * m))
            .line_to((3 * m, 1 * m))
            .toward(-1.0, 0.0)  # west, exactly
            .fillet(0.5 * m)
            .toward(0.0, 1.0)  # north, exactly
            .to((1 * m, 3 * m))  # the filleted side ends at its far vertex
            .line_to((0 * m, 3 * m))
            .line_to(Start)
        )
        # Five sharp corners plus the arc's two tangent points; the
        # virtual corner at (1, 1) is never a vertex.
        self.assertEqual(outline.vertex_count, 7)

        doc = Doc()
        bracket = doc.insert(
            Node.extrude(doc.insert(Node.profile(outline)), 0.75 * m)
        )
        expected = 0.75 * (5.25 - math.pi / 16.0)
        self.assertAlmostEqual(volume_of(doc, bracket), expected, delta=1e-12)


class TestVase(unittest.TestCase):
    """Tour scene `vase` (demos/tour/src/bodies.rs, row 3): a belly arc
    bound by a via point, revolved fully about the world y axis.

    Oracle, derived (the Rust scene asserts none): the belly runs on
    the circle of radius 1.3 centred at (0, 0.8) — on the axis — so
    the solid of revolution is pi * integral of x(y)^2 dy, which comes
    out at exactly 2.939 pi."""

    def test_vase_matches_the_derived_closed_form(self):
        outline = (
            Open.at((0 * m, 0 * m))
            .line_to((1.2 * m, 0 * m))
            .line_to((1.2 * m, 0.3 * m))
            .arc_via((1.3 * m, 0.8 * m), (0.5 * m, 2.0 * m))
            .line_to((0.9 * m, 2.5 * m))
            .line_to((0 * m, 2.5 * m))
            .line_to(Start)
        )
        doc = Doc()
        vase = doc.insert(
            Node.revolve(doc.insert(Node.profile(outline)), y_axis(doc), 360 * deg)
        )
        self.assertAlmostEqual(volume_of(doc, vase), 2.939 * math.pi, delta=1e-12)


class TestSheave(unittest.TestCase):
    """Tour scene `sheave` (demos/tour/src/bodies.rs, row 4): a grooved
    pulley — planes, cylinders, two cone shoulders and one torus
    groove — revolved fully about the world y axis. The Rust scene's
    own closed form is asserted verbatim.

    Its STRUCTURAL oracle is not: the scene also names its surface
    census (one torus, two cones), and counting surface kinds from
    Python needs tessellation or a selector, both still gaps (G11 and
    the selector surface). The volume is what this row can check."""

    def test_sheave_matches_the_scene_oracle(self):
        tip = Open.at((0.4 * m, 0 * m))
        for x, y in [(0.9, 0.0), (0.9, 0.25), (1.6, 0.25), (1.6, 0.0),
                     (2.0, 0.0), (2.1, 0.2)]:
            tip = tip.line_to((x * m, y * m))
        tip = tip.arc_via((1.8 * m, 0.5 * m), (2.1 * m, 0.8 * m))  # r = 0.3 groove
        for x, y in [(2.0, 1.0), (1.6, 1.0), (1.6, 0.75), (0.9, 0.75),
                     (0.9, 1.0), (0.4, 1.0)]:
            tip = tip.line_to((x * m, y * m))
        outline = tip.line_to(Start)

        doc = Doc()
        sheave = doc.insert(
            Node.revolve(doc.insert(Node.profile(outline)), y_axis(doc), 360 * deg)
        )
        expected = 2.0 * (1997.0 / 1200.0) * math.pi - 0.189 * math.pi * math.pi
        volume = volume_of(doc, sheave)
        self.assertLess(abs((volume - expected) / expected), 1e-12)


class TestBossplate(unittest.TestCase):
    """Tour scene `bossplate` (demos/tour/src/bossplate.rs, row 12): a
    plate fused with a round boss whose rim is THREE arcs.

    The three-arc rim is the scene's point (the seam is three walls,
    not two), and it is `circle_split` — the declared-subdivision
    carrier — not the `circle` primitive, whose private lowering is
    two semicircles. The Rust scene's closed form is asserted
    verbatim; its three-seam-arc census needs tessellation, which is
    still a gap here (G11)."""

    def test_bossplate_matches_the_scene_oracle(self):
        doc = Doc()
        plate_outline = (
            Open.at((0 * m, 0 * m))
            .line_to((4 * m, 0 * m))
            .line_to((4 * m, 4 * m))
            .line_to((0 * m, 4 * m))
            .line_to(Start)
        )
        plate = doc.insert(
            Node.extrude(doc.insert(Node.profile(plate_outline)), 1.0 * m)
        )
        boss_outline = circle_split((2 * m, 2 * m), 0.5 * m, 3, 0 * rad)
        self.assertEqual(boss_outline.vertex_count, 3, "three arcs, three walls")
        boss = doc.insert(
            Node.extrude(
                doc.insert(Node.profile(boss_outline, elevation=0.4 * m)), 1.2 * m
            )
        )
        fused = doc.insert(Node.boolean(BooleanOp.Union, plate, boss))

        expected = 16.0 + math.pi * 0.25 * 0.6
        self.assertAlmostEqual(volume_of(doc, fused), expected, delta=1e-6)


# ------------------------------------------------------------------
# The rows LIB-PYG23A unblocked: G3 (non-xy sketch planes) entirely,
# and G2's LOFT half. Each rebuilds the scene from the same authored
# numbers as the Rust source and asserts the scene's own oracle.
# ------------------------------------------------------------------

# demos/tour/src/skinned.rs::PRISM_SQUARE / PRISM_TRAPEZOID, verbatim.
PRISM_SQUARE = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
PRISM_TRAPEZOID = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)]


def prism_loft(doc, heights):
    """The square/trapezoid/square stack both loft scenes share, at
    `heights`, skinned at v-degree 2.

    There is no placement argument on `Node.loft` and none is wanted:
    each section rides its OWN profile's sketch plane, so the spacing
    IS the three elevations — which is exactly how the Rust scenes
    differ from one another (`lofted_at_z`)."""
    sections = [
        doc.insert(Node.polygon([(x * m, y * m) for x, y in pts], elevation=z * m))
        for pts, z in zip([PRISM_SQUARE, PRISM_TRAPEZOID, PRISM_SQUARE], heights)
    ]
    return doc.insert(Node.loft(sections, 2))


class TestLoftPrism(unittest.TestCase):
    """Tour scene `loft_prism` (demos/tour/src/skinned.rs, row 13; the
    document twin is editor-core/tests/corpus/loft_prism.rs): three
    polyline quad sections — squares at z = 0 and z = 2, a trapezoid at
    z = 1 — skinned at v-degree 2. The middle section is not an affine
    image of the squares, so the four walls are genuinely curved."""

    def test_loft_prism_matches_the_derived_closed_form(self):
        # The scene's own derivation: the degree-2 skin through
        # sections at (0, 1/2, 1) is the quadratic Lagrange
        # interpolant, corner paths S + lambda(v)*D with
        # lambda = 4v(1-v), z = 2v exactly, each slice a trapezoid of
        # area 4 + 2*d*lambda (d = 0.375) -> V = 8 + 8d/3 = 9 exactly.
        doc = Doc()
        prism = prism_loft(doc, [0.0, 1.0, 2.0])

        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(prism), "the loft evaluated")
        body = ev.value(prism).body()
        body.validate()
        props = body.mass_properties()
        # The Rust acceptance row's bracket (sweep/tests/
        # m6_loft_body.rs): a quadrature enclosure is not a closed
        # form, so 9 must lie inside the CERTIFIED pad, and the pad
        # must stay tight.
        self.assertLessEqual(abs(props.volume - 9.0), props.volume_pad + 1e-9)
        self.assertLess(props.volume_pad, 1e-6, "the exact per-span lane is tight")

    def test_the_v_degree_is_a_count_the_kernel_checks(self):
        """`1 <= v_degree <= len(profiles) - 1` is the kernel's rule,
        not the binding's: nothing is pre-checked here, so degree 3
        through three sections refuses at evaluation."""
        doc = Doc()
        sections = [
            doc.insert(Node.polygon([(x * m, y * m) for x, y in PRISM_SQUARE], elevation=z * m))
            for z in (0.0, 1.0, 2.0)
        ]
        overdegree = doc.insert(Node.loft(sections, 3))
        ev = evaluate(doc)
        self.assertFalse(ev.succeeded(overdegree), "degree 3 needs four sections")


class TestNonuniformLoft(unittest.TestCase):
    """Tour scene `nonuniform_loft` (demos/tour/src/skinned.rs, row
    14): `loft_prism`'s OWN sections and height with only the middle
    placement moved — z = 0, 0.15, 2. The degree-2 skin interpolates
    through the crowded spacing and overshoots.

    READ-BACK RESIDUE, stated plainly (spec deliverable 3, MEASURED):
    the scene's actual subject is the v-parameterization the skin
    CHOSE, and the Rust scene asks the kernel for it
    (`sweep::loft_parameters`, LIB-U5) rather than deriving it. That
    door is NOT reachable from Python and binding it is not cheap: it
    takes `&[Section]` and `&[Affine3]` — kernel-level values with no
    Python vocabulary — and the document layer cannot supply them
    either, because a Loft node evaluates to a `Body` and drops
    `LoftGeometry::section_params` on the way out. So this row asserts
    the VOLUME oracle and pins `t` as the tour's own constant; the
    read-back stays a named residue, the m3 precedent from LIB-PYG1."""

    # demos/tour/src/skinned.rs::NONUNIFORM_T — the middle section's
    # v-parameter at this spacing, 3*sqrt(29)/(3*sqrt(29) + sqrt(5701)),
    # which the Rust scene pins against `loft_parameters`.
    NONUNIFORM_T = 0.1762536890990181

    def test_nonuniform_loft_matches_the_derived_closed_form(self):
        # V = 4H + dH/(3t(1-t)) = 8 + 0.25/(t(1-t)), H = 2, d = 0.375.
        t = self.NONUNIFORM_T
        expected = 8.0 + 0.25 / (t * (1.0 - t))
        self.assertAlmostEqual(expected, 9.721901523222, delta=1e-11)

        doc = Doc()
        loft = prism_loft(doc, [0.0, 0.15, 2.0])
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(loft), "the non-uniform loft evaluated")
        body = ev.value(loft).body()
        body.validate()
        props = body.mass_properties()
        # The scene's claim is "quadrature agrees at pad ~1e-13".
        self.assertLessEqual(abs(props.volume - expected), props.volume_pad + 1e-9)
        self.assertLess(props.volume_pad, 1e-6)

    def test_the_two_lofts_are_a_minimal_pair(self):
        """The scenes' whole point: same sections, same degree, same
        height — only the middle placement moves, and the volume
        moves with it."""
        doc = Doc()
        prism = prism_loft(doc, [0.0, 1.0, 2.0])
        skewed = prism_loft(doc, [0.0, 0.15, 2.0])
        ev = evaluate(doc)
        prism_v = ev.value(prism).body().mass_properties().volume
        skewed_v = ev.value(skewed).body().mass_properties().volume
        self.assertGreater(skewed_v, prism_v, "the crowded spacing overshoots")


# The letterform silhouette family (demos/tour/src/letterforms.rs).
# DECOUPLED variants: every cross-operand-coincident plane pair offset
# by 1/16, which is the tour's own no-shared-carrier design rule.
H_DECOUPLED = [
    (0.0, 0.0), (0.5, 0.0), (0.5, 1.25), (1.5, 1.25), (1.5, 0.0625),
    (2.0, 0.0625), (2.0, 2.9375), (1.5625, 2.9375), (1.5625, 1.75),
    (0.4375, 1.75), (0.4375, 3.0), (0.0, 3.0),
]
T_DECOUPLED = [
    (1.1875, 0.125), (1.8125, 0.125), (1.8125, 2.625), (3.25, 2.625),
    (3.25, 3.125), (-0.25, 3.125), (-0.25, 2.5625), (1.1875, 2.5625),
]
# (z, x), counterclockwise; the right-opening notch makes the C.
C_LETTER = [
    (0.1875, -0.0625), (3.0625, -0.0625), (3.0625, 2.0625),
    (2.4375, 2.0625), (2.4375, 0.375), (0.8125, 0.375),
    (0.8125, 2.0625), (0.1875, 2.0625),
]
V_2WAY = 4.5078125
V_3WAY = 2.798095703125


def letter(doc, poly, plane, distance):
    """One letterform prism: a polygon on `plane`, extruded along that
    plane's NORMAL — which is what makes the family a G3 scene. The
    normal is u x v, so the yz frame extrudes +x and the zx frame +y,
    exactly as the captions say."""
    sketch = doc.insert(Node.polygon([(a * m, b * m) for a, b in poly], plane=plane))
    return doc.insert(Node.extrude(sketch, distance * m))


def silhouette3(doc):
    """The 3-way solid, and the 2-way it is built from — ONE
    construction, because the Rust scenes are one too."""
    h = letter(doc, H_DECOUPLED, SketchPlane.from_frame(
        (0 * m, 0 * m, -0.25 * m), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)), 3.5)
    t = letter(doc, T_DECOUPLED, SketchPlane.from_frame(
        (-0.25 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)), 2.5)
    c = letter(doc, C_LETTER, SketchPlane.from_frame(
        (0 * m, -0.5 * m, 0 * m), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0)), 4.0)
    two = doc.insert(Node.boolean(BooleanOp.Intersect, h, t))
    three = doc.insert(Node.boolean(BooleanOp.Intersect, two, c))
    return two, three


class TestSilhouette(unittest.TestCase):
    """Tour scenes `silhouette` (row 22), `silhouette3` (row 23) and
    its three shadow stops (rows 24-26), demos/tour/src/letterforms.rs:
    one solid whose orthographic shadows are an H (down z), a T (down
    x) and a C (down y). The H is an xy sketch extruded +z, the T a yz
    sketch extruded +x, the C a zx sketch extruded +y — the audit's G3
    scene family, and the tour's first intersect-of-intersect.

    ROW SHARING, mirrored honestly: the three shadow stops are the
    SAME body as `silhouette3` viewed down a different axis
    (`three.body.clone()` in the Rust), so they are one construction
    here too. The sharing is true BY CONSTRUCTION — one node id, read
    three times — not by a discriminating assertion; there is no body
    identity surface in Python that could make it one."""

    def test_silhouette_matches_the_scene_oracle(self):
        doc = Doc()
        two, _ = silhouette3(doc)
        self.assertAlmostEqual(volume_of(doc, two), V_2WAY, delta=1e-9)

    def test_silhouette3_matches_the_scene_oracle(self):
        doc = Doc()
        _, three = silhouette3(doc)
        self.assertAlmostEqual(volume_of(doc, three), V_3WAY, delta=1e-9)

    def test_the_shadow_rows_read_row_23s_body(self):
        """Rows 24-26 flip because row 23's body is theirs: the
        shadows are a CAMERA, not a construction.

        What this row shows, exactly: each shadow stop resolves to the
        SAME node id, so re-reading it yields row 23's oracle. It is
        not a discriminating check — one node read three times cannot
        disagree with itself — and it is not meant to be. The sharing
        is a property of the construction above (one `three`, exactly
        as the Rust scene clones one body); this row pins that the
        thing being shared is the oracle-bearing body."""
        doc = Doc()
        _, three = silhouette3(doc)
        ev = evaluate(doc)
        shadows = {axis: three for axis in ("z", "x", "y")}
        self.assertEqual(set(shadows.values()), {three}, "one node, three stops")
        for axis, node in shadows.items():
            with self.subTest(shadow=axis):
                volume = ev.value(node).body().mass_properties().volume
                self.assertAlmostEqual(volume, V_3WAY, delta=1e-9)


class TestTheSketchPlaneVocabulary(unittest.TestCase):
    """G3's door itself, apart from any scene."""

    def test_the_named_planes_are_the_cyclic_frames(self):
        """`xy`/`yz`/`zx` are sugar for one `from_frame` spelling each,
        in the cyclic order x->y->z->x — the convention the tour's
        captions speak. Pinned through the repr, which prints the
        frame the plane actually carries; the Rust rows
        (crates/profile/tests/sketch_plane.rs) pin what it does to
        points."""
        origin = (0 * m, 0 * m, 0 * m)
        for named, u, v in [
            (SketchPlane.xy(), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)),
            (SketchPlane.yz(), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)),
            (SketchPlane.zx(), (0.0, 0.0, 1.0), (1.0, 0.0, 0.0)),
        ]:
            with self.subTest(plane=repr(named)):
                self.assertEqual(
                    repr(named), repr(SketchPlane.from_frame(origin, u, v))
                )

    def test_a_rigid_frame_does_not_change_the_measure(self):
        """The same sketch on each named plane extrudes to the same
        SOLID measure — a rigid frame moves a body, it does not
        reshape one. (Which face of the world it lands on is the
        silhouette family's job to pin: that scene's oracle only comes
        out if the T really is on yz and the C really is on zx.)"""
        rect = [(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (0.0, 1.0)]
        for plane in (SketchPlane.xy(), SketchPlane.yz(), SketchPlane.zx()):
            with self.subTest(plane=repr(plane)):
                doc = Doc()
                prism = letter(doc, rect, plane, 3.0)
                self.assertAlmostEqual(volume_of(doc, prism), 6.0, delta=1e-12)

    def test_plane_and_elevation_are_mutually_exclusive(self):
        """Two spellings of one thing: naming the plane twice is a
        boundary TypeError, on both doors, rather than a silent
        preference."""
        with self.assertRaises(TypeError):
            Node.polygon(
                [(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m)],
                elevation=1 * m,
                plane=SketchPlane.yz(),
            )
        with self.assertRaises(TypeError):
            Node.profile(
                circle((0 * m, 0 * m), 1 * m),
                elevation=1 * m,
                plane=SketchPlane.yz(),
            )

    def test_rigidity_is_an_unchecked_convention(self):
        """The Rust contract, verbatim: a non-rigid frame is a
        well-defined SKEWED sketch, not a refusal. The binding adds no
        orthogonality predicate — it would be a check the kernel does
        not make."""
        skewed = SketchPlane.from_frame(
            (0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0), (1.0, 1.0, 0.0)
        )
        doc = Doc()
        prism = letter(doc, [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], skewed, 1.0)
        # A sheared prism, not a cube: the sketch square lands as a
        # parallelogram (unit area, since det[u v n] = 1) swept 1 up
        # the frame's normal. Well-defined geometry either way.
        self.assertAlmostEqual(volume_of(doc, prism), 1.0, delta=1e-12)


class TestNamedGapsAreStillGaps(unittest.TestCase):
    """The NO rows' gaps, asserted as absences.

    If one of these fails, a door has been BUILT — good news, and the
    signal to promote the corresponding rows in the audit from NO to
    YES."""

    def test_the_bound_vocabulary_is_exactly_this(self):
        self.assertEqual(
            sorted(n for n in dir(Node) if not n.startswith("_")),
            ["boolean", "datum_axis", "extrude", "loft", "polygon", "profile", "revolve"],
        )
        self.assertEqual(
            sorted(n for n in dir(DocEdit) if not n.startswith("_")),
            ["delete_node", "insert_node", "set_doc_param", "set_tolerance"],
        )

    def test_the_named_gaps_are_still_gaps(self):
        import pncad

        # `ParamName`/`DocParam` left this list when G10 closed
        # (R1-PARAMS) — `TestPlateParam` above is the positive form.
        for door in [
            "tessellate", "Mesh", "write_stl",        # mesh + STL
            "select", "select_where", "Selector",     # selectors
            "StableName", "find_flush_candidates",    # names, detect/declare
        ]:
            with self.subTest(door=door):
                self.assertFalse(hasattr(pncad, door), f"{door} is now bound")

        # `circle` left this list when G1 closed (LIB-PYG1): it is a
        # profile PRIMITIVE, `pncad.circle`, not a node kind, and the
        # positive form is `TestBossplate` plus `tests/test_paths.py`.
        # `loft` left it when LIB-PYG23A closed G2's loft half; the
        # positive form is `TestLoftPrism`/`TestNonuniformLoft`.
        # `sweep` and `tube` STAY: `wire_sweep` refuses unconditionally
        # (SWEEP_FRONTIER, the path-composition lane banked past M6),
        # and no `Node::Tube` exists at all.
        for node_kind in ["fillet", "sweep", "tube", "pattern",
                          "transform", "split"]:
            with self.subTest(node=node_kind):
                self.assertFalse(hasattr(Node, node_kind), f"Node.{node_kind} exists")

    def test_a_profile_is_still_exactly_one_loop(self):
        """G9: holes have no door. `Node.profile` takes ONE loop, so a
        list of them is a boundary refusal, not a silently dropped
        second loop."""
        outer = circle((0 * m, 0 * m), 2 * m)
        inner = circle((0 * m, 0 * m), 1 * m)
        with self.assertRaises(TypeError):
            Node.profile([outer, inner])

    def test_the_plane_argument_is_a_sketch_plane_not_a_name(self):
        """G3 is closed, but the door takes the VALUE, not a string:
        `plane=` is a `SketchPlane`, so a stringly-typed spelling is a
        boundary refusal rather than a guess at what "yz" meant."""
        with self.assertRaises(TypeError):
            Node.profile(circle((0 * m, 0 * m), 1 * m), plane="yz")

    def test_a_swept_solid_is_still_out_of_reach(self):
        """G2's remaining half, positively: there is no `Node.sweep`
        to call, and the reason is not an unbound door — `wire_sweep`
        refuses unconditionally, so binding one would flip no row."""
        self.assertFalse(hasattr(Node, "sweep"))
        self.assertFalse(hasattr(Node, "tube"))


if __name__ == "__main__":
    unittest.main()
