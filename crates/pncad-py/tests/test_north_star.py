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

import inspect
import math
import unittest
from pathlib import Path
from typing import ClassVar

from pncad import (
    ArcSide,
    ArcSweep,
    BooleanOp,
    Bulge,
    Center,
    Cmp,
    ContactClass,
    CurveKind,
    Doc,
    DocEdit,
    DocParam,
    DocParamValue,
    EditError,
    EntityKind,
    EvaluationError,
    Frame,
    GeomPred,
    NamePat,
    Node,
    Open,
    ParamName,
    PatternKind,
    PlaneRelation,
    Radius,
    SegPat,
    SegTag,
    Selector,
    SketchPlane,
    Start,
    SurfaceKind,
    Via,
    circle,
    circle_split,
    deg,
    evaluate,
    load,
    m,
    mm,
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


def projectbox(doc):
    """Tour scene `projectbox` (demos/tour/src/projectbox.rs): 15 ops
    over 16 boxes — cavity, six vent slots, four bosses, four pilot
    pockets. Shared by the volume-oracle row and the `cutaway` row,
    which splits exactly this body."""
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
    return body


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
        body = projectbox(doc)

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
    """Tour scenes `heatsink5/7/9`, still YES* — the FUSED body.

    The scene's whole body is the fins unioned INTO a base, and that
    last step is the residual G8 names: fusing an N-solid group into a
    base needs a multi-solid boolean operand the kernel does not have
    (`combine` takes two SINGLE-SOLID operands). So the fused body is
    still reproduced here by hand-authoring each fin, one union apiece.

    What is no longer hand-authored is the fin family itself —
    `TestHeatsinkFins` below says it as ONE node with a
    parameter-driven count, which is the structural half of the scene.
    """

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


class TestHeatsinkFins(unittest.TestCase):
    """The structural half of `heatsink5/7/9`, said structurally —
    the Python twin of corpus document `heat_sink_fins`.

    ONE `PlacedUnion(Linear)` node carries the whole fin family, its
    count bound to the document parameter `fins`, and 5 -> 7 -> 9 is
    ONE `set_doc_param` edit each. That is what G8 said could not be
    said: no pattern node, no structural-param edit.

    The BASE deliberately stays out. Fusing the group into it is the
    kernel's single-solid combine wall (`JoinDesync`: "operand A/B is
    not a single-solid body"), a kernel door that does not exist —
    reported, never worked around, so this document says exactly what
    the group node buys and no more.
    """

    #: `heat_sink`'s own constants: footprint 0.1875 x 0.75 at
    #: z = 0.1875, extruded 0.8125, pitch 0.3125 — leaving 0.125 of
    #: clear air between neighbours, which is the clearance the
    #: disjointness certificate needs.
    FIN_VOLUME = 0.1875 * 0.75 * 0.8125
    FIN_AREA = 2 * 0.140625 + 2 * (0.1875 + 0.75) * 0.8125

    def build(self):
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("fins"), DocParam.count(5)))
        profile = doc.insert(
            Node.polygon(
                [
                    (0.25 * m, 0.125 * m),
                    (0.4375 * m, 0.125 * m),
                    (0.4375 * m, 0.875 * m),
                    (0.25 * m, 0.875 * m),
                ],
                elevation=0.1875 * m,
            )
        )
        fin = doc.insert(Node.extrude(profile, 0.8125 * m))
        fins = doc.insert(
            Node.placed_union(
                fin, 5, PatternKind.linear((1.0, 0.0, 0.0), 0.3125 * m)
            )
        )
        doc.apply(DocEdit.bind_count_param(fins, ParamName("fins")))
        return doc, fins

    def test_one_param_edit_recounts_the_whole_fin_family(self):
        doc, fins = self.build()
        for count in (5, 7, 9):
            with self.subTest(fins=count):
                doc.apply(DocEdit.set_doc_param(ParamName("fins"), DocParam.count(count)))
                ev = evaluate(doc)
                self.assertTrue(ev.succeeded(fins))
                body = ev.value(fins).body()
                body.validate()
                mass = body.mass_properties()
                # The corpus pins, exactly: both oracles are dyadic, so
                # the comparison is `==`, not a tolerance.
                self.assertEqual(mass.volume, count * self.FIN_VOLUME)
                self.assertEqual(mass.surface_area, count * self.FIN_AREA)

    def test_the_fin_family_is_one_node_and_one_body(self):
        doc, fins = self.build()
        # A pattern node would answer `instances` here, which is
        # exactly what no boolean can consume.
        self.assertEqual(evaluate(doc).value(fins).kind, "body")


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
        / "crates" / "pncad" / "tests" / "plate_param.v18.pncad"
    )

    # Insert order: profile, plate, tab profile, tab, union, measure,
    # assertion. The union is index 4 and no longer the last insert —
    # the fixture gained the measurement pair so the READ doors below
    # have a document to read.
    UNION = 4
    MEASURE = 5
    ASSERTION = 6

    def plate(self):
        doc = load(self.FIXTURE.read_text(encoding="utf-8")).doc
        return doc, doc.order()[self.UNION]

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

    def test_the_value_door_moves_the_holes_and_keeps_the_declaration(self):
        """`set_doc_param_value` is the SAFE spelling of a value change.

        `set_doc_param` is create-or-replace: passing it a `DocParam`
        rebuilt from a dimension and a number replaces the declaration,
        and any distribution the parameter carried (ERROR-DESIGN E1/E2 —
        which Python cannot spell) is deleted with no refusal. The value
        door carries the declaration forward instead. Here it is doing
        the ordinary job as well: the same oracle, through the other
        door."""
        for r in (0.25, 0.4):
            with self.subTest(hole_r=r):
                doc, solid = self.plate()
                doc.apply(
                    DocEdit.set_doc_param_value(
                        ParamName("hole_r"), DocParamValue.length(r * m)
                    )
                )
                self.assertAlmostEqual(
                    volume_of(doc, solid), self.oracle(r), delta=1e-6
                )

    def test_the_value_door_refuses_typed(self):
        """Its two refusals, both typed: there is no declaration to
        carry forward, and a kind change is a redeclaration."""
        import pncad

        doc, _solid = self.plate()
        with self.assertRaises(pncad.EditError) as ctx:
            doc.apply(
                DocEdit.set_doc_param_value(
                    ParamName("never_declared"), DocParamValue.length(1 * m)
                )
            )
        self.assertEqual(ctx.exception.variant, "doc_param_not_declared")
        with self.assertRaises(pncad.EditError) as ctx:
            doc.apply(
                DocEdit.set_doc_param_value(
                    ParamName("hole_r"), DocParamValue.count(3)
                )
            )
        self.assertEqual(
            ctx.exception.variant, "doc_param_value_kind_mismatch"
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

    # The holes sit at x = 1.0 and x = 2.2 (see `plate_profile` in
    # crates/pncad/tests/all.rs), so the axis separation the fixture's
    # measure reports is exactly 1.2 — the number the SCENE was
    # authored with, not one read off a previous run.
    WEB_ORACLE = 2.2 - 1.0

    def test_a_measure_and_its_verdict_read_back_from_python(self):
        """The READ half of the measurement vocabulary (ERROR-DESIGN
        E3/E10), which is what the binding census says SHOULD ship:
        Python cannot author a measure (`B-MEASURES`), but it can read
        one — and its assertion's verdict — off any evaluation,
        including this document, which was authored elsewhere and
        crossed through the persistence door."""
        doc, _ = self.plate()
        ev = evaluate(doc)

        measurement = ev.value(doc.order()[self.MEASURE]).measure()
        self.assertEqual(measurement.dimension, "Length")
        # The scene's own oracle. The row this replaces asserted only
        # `value >= 0.0`, which the fixture satisfied VACUOUSLY: it was
        # measuring one hole's two wall halves against each other and
        # reporting 0.0. Pinning the number caught that.
        self.assertAlmostEqual(measurement.value, self.WEB_ORACLE, places=9)
        self.assertIsNotNone(measurement.length)
        self.assertAlmostEqual(
            measurement.length.in_unit(m), measurement.value, places=12
        )

        verdict = ev.value(doc.order()[self.ASSERTION]).assertion()
        self.assertEqual(verdict.status, "Holds")
        self.assertIs(verdict.holds, True)
        # The verdict reports the measure's OWN number, and its flag
        # agrees with the relation it claims to have decided.
        self.assertAlmostEqual(verdict.measured, measurement.value, places=12)
        self.assertIsNotNone(verdict.bound)
        self.assertGreaterEqual(verdict.measured, verdict.bound)
        self.assertIsNone(verdict.reason)

    def test_reading_a_verdict_changes_nothing(self):
        """E10's report-only rule, from the consumer's seat.

        The claim is not "reading is a pure getter" — that is true of
        every property and cannot fail. It is that the ASSERTION'S
        PRESENCE changes no outcome: delete it, and the document's
        product and its measured value are bit-identical. That is what
        a gating assertion would break, so this is the row that would
        go red if one ever appeared."""
        import pncad

        doc, solid = self.plate()
        ev = evaluate(doc)
        with_volume = ev.value(solid).body().mass_properties().volume
        with_web = ev.value(doc.order()[self.MEASURE]).measure().value
        verdict = ev.value(doc.order()[self.ASSERTION]).assertion()
        self.assertEqual(verdict.status, "Holds")

        # The SAME document with the assertion deleted. `Doc.apply`
        # edits in place, so this is a second load of the fixture.
        without, _ = self.plate()
        without.apply(DocEdit.delete_node(without.order()[self.ASSERTION]))
        ev2 = evaluate(without)
        self.assertEqual(
            ev2.value(solid).body().mass_properties().volume,
            with_volume,
            "an assertion is report-only: the product cannot see it",
        )
        self.assertEqual(
            ev2.value(without.order()[self.MEASURE]).measure().value,
            with_web,
            "an assertion is report-only: the measure cannot see it either",
        )

        # And asking a non-assertion for a verdict is a typed refusal,
        # not a guess.
        with self.assertRaises(pncad.EvaluationError) as ctx:
            ev.value(solid).assertion()
        self.assertEqual(ctx.exception.reason, "wrong_kind")
        with self.assertRaises(pncad.EvaluationError) as ctx:
            ev.value(solid).measure()
        self.assertEqual(ctx.exception.reason, "wrong_kind")


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
    """Tour scene `vase` (demos/tour/src/bodies.rs, row 4): a belly arc
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
            .arc_to(Via((1.3 * m, 0.8 * m), (0.5 * m, 2.0 * m)))
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
    """Tour scene `sheave` (demos/tour/src/bodies.rs, row 5): a grooved
    pulley — planes, cylinders, two cone shoulders and one torus
    groove — revolved fully about the world y axis. The Rust scene's
    own closed form is asserted verbatim.

    Its STRUCTURAL oracle is not: the scene also names its surface
    census (one torus, two cones), and this row does not count them.
    The volume is what it checks. (Both doors that would — the
    selector's `surface_kind` filter and the mesh's per-face patches —
    have since been bound, by LIB-PYSEL and LIB-G11; asserting a
    surface census here is work this row has not done, not work the
    surface cannot express.)"""

    def test_sheave_matches_the_scene_oracle(self):
        tip = Open.at((0.4 * m, 0 * m))
        for x, y in [(0.9, 0.0), (0.9, 0.25), (1.6, 0.25), (1.6, 0.0),
                     (2.0, 0.0), (2.1, 0.2)]:
            tip = tip.line_to((x * m, y * m))
        tip = tip.arc_to(Via((1.8 * m, 0.5 * m), (2.1 * m, 0.8 * m)))  # groove
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
    """Tour scene `bossplate` (demos/tour/src/bossplate.rs, row 17): a
    plate fused with a round boss whose rim is THREE arcs.

    The three-arc rim is the scene's point (the seam is three walls,
    not two), and it is `circle_split` — the declared-subdivision
    carrier — not the `circle` primitive, whose private lowering is
    two semicircles. The Rust scene's closed form is asserted
    verbatim, and its three-seam-arc census is not: the vertex count
    of the authored loop is checked instead, which is where the claim
    is actually made."""

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
    # `strict=True`: a `heights` of the wrong length is a caller error, not a
    # shorter loft. Without it this helper would silently skin fewer sections
    # than the profile list names and every assertion below would still pass,
    # on a solid nobody asked for.
    sections = [
        doc.insert(Node.polygon([(x * m, y * m) for x, y in pts], elevation=z * m))
        for pts, z in zip([PRISM_SQUARE, PRISM_TRAPEZOID, PRISM_SQUARE], heights, strict=True)
    ]
    return doc.insert(Node.loft(sections, 2))


class TestLoftPrism(unittest.TestCase):
    """Tour scene `loft_prism` (demos/tour/src/skinned.rs, row 18; the
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
    19): `loft_prism`'s OWN sections and height with only the middle
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
    """Tour scenes `silhouette` (row 31), `silhouette3` (row 32) and
    its three shadow stops (rows 33-35), demos/tour/src/letterforms.rs:
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

    def test_the_shadow_rows_read_row_32s_body(self):
        """Rows 33-35 flip because row 32's body is theirs: the
        shadows are a CAMERA, not a construction.

        What this row shows, exactly: each shadow stop resolves to the
        SAME node id, so re-reading it yields row 32's oracle. It is
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


# ------------------------------------------------------------------
# The rows LIB-PYBUNDLE unblocked: G4 (fillet), G6 (split), G7 (rigid
# placement) and G9 (multi-loop profiles). Each rebuilds the scene
# from the same authored numbers as the Rust source and asserts the
# scene's own oracle.
# ------------------------------------------------------------------


def loop_of(points):
    """A closed polygonal loop through `points`, in metres — the
    PATHS spelling of `demos/tour/src/paths.rs::path_polygon`."""
    chain = Open.at((points[0][0] * m, points[0][1] * m))
    for x, y in points[1:]:
        chain = chain.line_to((x * m, y * m))
    return chain.line_to(Start)


class TestPlate(unittest.TestCase):
    """Tour scene `plate` (demos/tour/src/bodies.rs, row 3): a
    6 x 3 x 0.6 slab with two r = 0.7 through-holes — genus 2, and the
    first multi-loop profile Python can say.

    Oracle, derived (the Rust scene asserts none — the tour holds it
    to the generic ladder): the loops are a rectangle and two disjoint
    circles, so the area is 6*3 - 2*pi*0.7^2 and the prism is that
    times 0.6."""

    def test_plate_matches_the_derived_closed_form(self):
        doc = Doc()
        sketch = doc.insert(
            Node.profile(
                [
                    loop_of([(-3, -1.5), (3, -1.5), (3, 1.5), (-3, 1.5)]),
                    circle((-1.5 * m, 0 * m), 0.7 * m),
                    circle((1.5 * m, 0 * m), 0.7 * m),
                ]
            )
        )
        plate = doc.insert(Node.extrude(sketch, 0.6 * m))
        expected = 0.6 * (6.0 * 3.0 - 2.0 * math.pi * 0.7 * 0.7)
        self.assertAlmostEqual(volume_of(doc, plate), expected, delta=1e-12)

    def test_the_loop_set_is_validated_kernel_side(self):
        """Nothing about the loop SET is pre-checked at the boundary.
        Two disjoint circles are not an outline and its hole, and the
        refusal is the kernel's own profile validation reaching Python
        through the edit door's replay probe — typed, at `insert`."""
        with self.assertRaises(EditError) as caught:
            Doc().insert(
                Node.profile(
                    [circle((0 * m, 0 * m), 1 * m), circle((5 * m, 0 * m), 1 * m)]
                )
            )
        self.assertEqual(caught.exception.variant, "profile_program_refused")


class TestAz(unittest.TestCase):
    """Tour scene `az` (demos/tour/src/az.rs, row 36): the A prism and
    the Z prism intersected. The A's counter is a true inner loop, so
    the scene needed multi-loop profiles; its yz/zx-style frames came
    with G3.

    The scene's own exact oracle: 880383/327680."""

    A_OUTLINE: ClassVar = [
        (0.0, 0.0), (0.625, 0.0), (0.8125, 1.0), (1.1875, 1.0),
        (1.375, 0.0), (2.0, 0.0), (1.125, 2.5), (0.875, 2.5),
    ]
    A_COUNTER: ClassVar = [(0.90625, 1.4375), (1.09375, 1.4375), (1.0, 2.0)]
    Z_OUTLINE: ClassVar = [
        (-0.0625, 0.0), (2.5625, 0.0), (2.5625, 0.4375), (0.6875, 0.4375),
        (2.5625, 1.5625), (2.5625, 2.0), (-0.0625, 2.0), (-0.0625, 1.5625),
        (1.8125, 1.5625), (-0.0625, 0.4375),
    ]

    def test_az_matches_the_scene_oracle(self):
        doc = Doc()
        a_plane = SketchPlane.from_frame(
            (0 * m, 0 * m, -0.0625 * m), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)
        )
        z_plane = SketchPlane.from_frame(
            (-0.0625 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)
        )
        a = doc.insert(
            Node.extrude(
                doc.insert(
                    Node.profile(
                        [loop_of(self.A_OUTLINE), loop_of(self.A_COUNTER)],
                        plane=a_plane,
                    )
                ),
                2.125 * m,
            )
        )
        z = doc.insert(
            Node.extrude(
                doc.insert(Node.profile(loop_of(self.Z_OUTLINE), plane=z_plane)),
                2.125 * m,
            )
        )
        az = doc.insert(Node.boolean(BooleanOp.Intersect, a, z))
        # demos/tour/src/az.rs::V_AZ, at the scene's own 1e-9 gate.
        self.assertAlmostEqual(volume_of(doc, az), 880383.0 / 327680.0, delta=1e-9)


class TestDiefillet(unittest.TestCase):
    """Tour scene `diefillet`, the `blank` stop (row 8): a unit cube
    with all twelve edges filleted at r = 0.12.

    The selection is not "every edge" — there is no such spelling —
    it is the twelve names `all_edges` materialized off THIS
    evaluation, stored into the recipe and frozen there.

    The scene's own closed form (demos/tour/src/diefillet.rs::
    blank_volume, and crates/sweep/tests/m5_pr12_die.rs) at its own
    relative 1e-9 gate."""

    L, R = 1.0, 0.12

    def build(self):
        doc = Doc()
        sq = doc.insert(
            Node.polygon(
                [
                    (0 * m, 0 * m), (self.L * m, 0 * m),
                    (self.L * m, self.L * m), (0 * m, self.L * m),
                ]
            )
        )
        cube = doc.insert(Node.extrude(sq, self.L * m))
        return doc, cube

    def test_diefillet_matches_the_scene_oracle(self):
        doc, cube = self.build()
        edges = evaluate(doc).all_edges(cube)
        self.assertEqual(len(edges), 12)
        blank = doc.insert(Node.fillet(cube, self.R * m, edges))

        core = self.L - 2.0 * self.R
        want = (
            core ** 3
            + 6.0 * self.R * core ** 2
            + 12.0 * (math.pi * self.R * self.R / 4.0) * core
            + (4.0 / 3.0) * math.pi * self.R ** 3
        )
        self.assertAlmostEqual(volume_of(doc, blank), want, delta=1e-9 * want)

    def test_an_empty_selection_is_the_kernels_refusal(self):
        """Not pre-checked at the boundary: the fillet node itself
        refuses an empty selection, typed, at evaluate."""
        doc, cube = self.build()
        nothing = doc.insert(Node.fillet(cube, self.R * m, []))
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(nothing)
        self.assertEqual(caught.exception.kind, "fillet_selection_empty")

    def test_a_name_is_carried_not_composed(self):
        """The text is a TOKEN. Something that is not a name at all is
        a boundary ValueError — there is no name grammar in Python to
        half-parse."""
        _doc, cube = self.build()
        with self.assertRaises(ValueError):
            Node.fillet(cube, self.R * m, ["the top edge"])

    def test_the_selection_is_canonical_whatever_order_it_arrives_in(self):
        """`Node.fillet` goes through Rust's one construction door, so
        two recipes that select the same edges are bit-identical."""
        doc, cube = self.build()
        edges = evaluate(doc).all_edges(cube)
        # `bit_eq` compares identity as well as recipe, so the two
        # spellings are authored as the SAME part — which is what the
        # labelled constructor says. The claim under test is about the
        # SELECTION being canonical, not about two parts colliding.
        forward = Doc(label="canonical-fillet-selection")
        backward = Doc(label="canonical-fillet-selection")
        for target, order in ((forward, edges), (backward, list(reversed(edges)))):
            sq = target.insert(
                Node.polygon(
                    [
                        (0 * m, 0 * m), (self.L * m, 0 * m),
                        (self.L * m, self.L * m), (0 * m, self.L * m),
                    ]
                )
            )
            solid = target.insert(Node.extrude(sq, self.L * m))
            target.insert(Node.fillet(solid, self.R * m, order))
        self.assertTrue(forward.bit_eq(backward))


class TestDiechamfer(unittest.TestCase):
    """Tour scenes `spacer` (row 2) and `diechamfer` (rows 11, 12),
    through the recipe door LIB-G16 opened: a unit cube with all
    twelve edges CHAMFERED at an equal setback.

    What this row is evidence of is the audit's own complaint. Before
    `Node.chamfer` the scenes had to take the body out of the
    document, call `chamfer_edges` beside it with ARENA keys, and hand
    back something the document could not name. Here the selection is
    the twelve names `all_edges` materialized off this evaluation —
    the same text `Node.fillet` takes — and the result is a node with
    a stable name of its own.

    The oracle is derived, not measured: a cube of side L chamfered at
    setback d is the cube cut by twelve edge planes and eight corner
    patches, which integrates to

        V = L**3 - 6*L*d**2 + (16/3)*d**3

    (`crates/editor-core/tests/lib_g16_chamfer_node.rs` carries the
    derivation and meters the surface area too)."""

    # `demos/tour/src/diefillet.rs`: L and the blend size the
    # chamfered pair sets as its SETBACK, so this IS the scene.
    L, D = 1.0, 0.12

    def build(self):
        doc = Doc()
        sq = doc.insert(
            Node.polygon(
                [
                    (0 * m, 0 * m), (self.L * m, 0 * m),
                    (self.L * m, self.L * m), (0 * m, self.L * m),
                ]
            )
        )
        cube = doc.insert(Node.extrude(sq, self.L * m))
        return doc, cube

    def test_the_chamfered_cube_matches_the_derived_closed_form(self):
        doc, cube = self.build()
        edges = evaluate(doc).all_edges(cube)
        self.assertEqual(len(edges), 12)
        blank = doc.insert(Node.chamfer(cube, self.D * m, edges))
        want = self.L ** 3 - 6.0 * self.L * self.D ** 2 + (16.0 / 3.0) * self.D ** 3
        self.assertAlmostEqual(volume_of(doc, blank), want, delta=1e-9 * want)

    def test_the_chamfer_removes_more_than_the_fillet_of_the_same_size(self):
        """The twin recipes differ in one node kind, and the geometry
        says so: the flat strip cuts the corner the rolling ball rides
        around."""
        doc, cube = self.build()
        edges = evaluate(doc).all_edges(cube)
        ch = doc.insert(Node.chamfer(cube, self.D * m, edges))
        fi = doc.insert(Node.fillet(cube, self.D * m, edges))
        self.assertLess(volume_of(doc, ch), volume_of(doc, fi))

    def test_the_chamfered_body_carries_names_of_its_own(self):
        """The half `chamfer_edges` beside a document could never
        give: the result is a NODE, so its faces have stable names and
        a downstream selection can reach them."""
        doc, cube = self.build()
        edges = evaluate(doc).all_edges(cube)
        blank = doc.insert(Node.chamfer(cube, self.D * m, edges))
        ev = evaluate(doc)
        faces = ev.all_faces(blank)
        # 6 supports + 12 strips + 8 corner patches.
        self.assertEqual(len(faces), 26)
        self.assertEqual(len(set(faces)), 26, "every face is named once")
        # Euler on the same body: 8 triangular corner patches give 24
        # vertices, so V - E + F = 2 puts E at 48. Every one of them
        # is named and distinct, which is what makes a downstream
        # selection possible at all.
        edges_out = ev.all_edges(blank)
        self.assertEqual(len(edges_out), 48)
        self.assertEqual(len(set(edges_out)), 48, "every edge is named once")

    def test_an_empty_selection_is_the_kernels_refusal_naming_the_chamfer(self):
        """The refusal is the chamfer's, not the fillet's — one shared
        ladder, but the tag says which verb asked."""
        doc, cube = self.build()
        nothing = doc.insert(Node.chamfer(cube, self.D * m, []))
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(nothing)
        self.assertEqual(caught.exception.kind, "chamfer_selection_empty")

    def test_a_name_is_carried_not_composed(self):
        _doc, cube = self.build()
        with self.assertRaises(ValueError):
            Node.chamfer(cube, self.D * m, ["the top edge"])

    def test_the_selection_is_canonical_whatever_order_it_arrives_in(self):
        doc, cube = self.build()
        edges = evaluate(doc).all_edges(cube)
        forward = Doc(label="canonical-chamfer-selection")
        backward = Doc(label="canonical-chamfer-selection")
        for target, order in ((forward, edges), (backward, list(reversed(edges)))):
            sq = target.insert(
                Node.polygon(
                    [
                        (0 * m, 0 * m), (self.L * m, 0 * m),
                        (self.L * m, self.L * m), (0 * m, self.L * m),
                    ]
                )
            )
            solid = target.insert(Node.extrude(sq, self.L * m))
            target.insert(Node.chamfer(solid, self.D * m, order))
        self.assertTrue(forward.bit_eq(backward))


class DieScene:
    """The 21-pip die construction rows 9 and 10 share (a mixin, not a
    TestCase): one re-charted ball, twenty-one `Node.transform`
    placements whose pole rides the face normal, the twenty-one balls
    fused into a single tool, ONE subtract."""

    L, PIP_R, PIP_H, PIP_D = 1.0, 0.09, 0.05, 0.22

    # (pip count, face normal, the two in-face axes, rotation carrying
    # +z to that normal) — demos/tour/src/diefillet.rs::placements.
    FACES: ClassVar = [
        (1, (0, 0, 1), (1, 0, 0), (0, 1, 0), (0.0, 0.0, 1.0), 0.0),
        (6, (0, 0, -1), (1, 0, 0), (0, 1, 0), (1.0, 0.0, 0.0), math.pi),
        (2, (1, 0, 0), (0, 1, 0), (0, 0, 1), (0.0, 1.0, 0.0), math.pi / 2),
        (5, (-1, 0, 0), (0, 1, 0), (0, 0, 1), (0.0, 1.0, 0.0), -math.pi / 2),
        (3, (0, 1, 0), (0, 0, 1), (1, 0, 0), (1.0, 0.0, 0.0), -math.pi / 2),
        (4, (0, -1, 0), (0, 0, 1), (1, 0, 0), (1.0, 0.0, 0.0), math.pi / 2),
    ]

    def layout(self, n):
        d = self.PIP_D
        diag = [(-d, -d), (d, d)]
        anti = [(-d, d), (d, -d)]
        return {
            1: [(0.0, 0.0)],
            2: diag,
            3: [*diag, (0.0, 0.0)],
            4: diag + anti,
            5: diag + anti + [(0.0, 0.0)],
            6: diag + anti + [(-d, 0.0), (d, 0.0)],
        }[n]

    def ball(self, doc):
        """A radius-PIP_R sphere at the origin, pole along +z: the
        half-disc revolved fully.

        The CHART is not the scene's. Both the scene
        (`demos/tour/src/diefillet.rs::ball`) and the oracle test
        (`sweep/tests/m5_pr12_die.rs::ball_at`) revolve ONE bulge-1
        semicircular arc; that chart refuses through the document
        layer (`kind == "naming"` — a meridian running pole to pole
        gives the revolve emitter a two-vertex all-on-axis loop it
        cannot name). This is the corpus `die_pips` workaround
        instead: two quarter arcs, so no meridian runs pole to pole.
        Same sphere, differently charted — the volume oracle is
        untouched by it, and the re-chart is what makes the scene
        reachable at all."""
        plane = SketchPlane.from_frame(
            (0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0)
        )
        half = (
            Open.at((0 * m, -self.PIP_R * m))
            .arc_to(Bulge((self.PIP_R * m, 0 * m), math.tan(math.pi / 8)))
            .arc_continue((0 * m, self.PIP_R * m))
            .line_to(Start)
        )
        sketch = doc.insert(Node.profile(half, plane=plane))
        axis = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0)))
        return doc.insert(Node.revolve(sketch, axis, (2.0 * math.pi) * rad))

    def pipped_die(self, doc):
        """The pipped cube: cube ∖ (21 fused balls), one subtract."""
        sq = doc.insert(
            Node.polygon(
                [
                    (0 * m, 0 * m), (self.L * m, 0 * m),
                    (self.L * m, self.L * m), (0 * m, self.L * m),
                ]
            )
        )
        cube = doc.insert(Node.extrude(sq, self.L * m))
        origin_ball = self.ball(doc)

        placed = []
        for n, nrm, ex, ey, axis, angle in self.FACES:
            # The ball centre sits PIP_R - PIP_H proud of the face, so
            # it dips exactly PIP_H in: an interpenetration, never a
            # tangency the kernel would refuse.
            base = [0.5 + nrm[i] * (0.5 + self.PIP_R - self.PIP_H) for i in range(3)]
            for u, w in self.layout(n):
                c = [base[i] + ex[i] * u + ey[i] * w for i in range(3)]
                placed.append(
                    doc.insert(
                        Node.transform(
                            origin_ball,
                            (c[0] * m, c[1] * m, c[2] * m),
                            axis,
                            angle * rad,
                        )
                    )
                )
        assert len(placed) == 21

        tool = placed[0]
        for pip in placed[1:]:
            tool = doc.insert(Node.boolean(BooleanOp.Union, tool, pip))
        return doc.insert(Node.boolean(BooleanOp.Subtract, cube, tool))


class TestDiepips(DieScene, unittest.TestCase):
    """Tour scene `diepips` (row 9): twenty-one spherical dimples on
    the six faces of a unit cube, cut in ONE group operation.

    The scene's STRUCTURE transfers whole — see `DieScene`. Its ball
    is RE-CHARTED (`DieScene.ball` states why and what it dodges).
    The scene's oracle is crates/sweep/tests/m5_pr12_die.rs: the cube
    less twenty-one spherical caps."""

    def test_diepips_matches_the_scene_oracle(self):
        doc = Doc()
        die = self.pipped_die(doc)

        cap = math.pi * self.PIP_H ** 2 * (3.0 * self.PIP_R - self.PIP_H) / 3.0
        want = self.L ** 3 - 21.0 * cap
        self.assertAlmostEqual(volume_of(doc, die), want, delta=1e-9 * want)


class TestDiecomposed(DieScene, unittest.TestCase):
    """Tour scene `diecomposed` (row 10): the pipped cube filleted IN
    PLACE, twice — the twelve box edges at r = 0.12, then all 21 pip
    rims at r = 0.02 — one body carrying the blank's blends, the pip
    cavities, and the rim torus bands.

    This row was YES\\* until LIB-PYSEL closed G13: the two blends
    need the box edges and the pip rims SEPARATED, and with no
    selector bound the only Python route read inside the opaque name
    text — representation-dependence, not a selector (the ordinal-28
    ruling). Now the scene says what the Rust scene says: the SAME
    two geometric filters `lib_sel1_geoselect.rs` runs — carrier kind
    `Line` for the box edges, `Plane`/`Sphere` adjacency for the
    rims — executed by `select_where`, with no name text read.

    The oracle is the closed form the Rust scene meters
    (crates/sweep/tests/m6_surgery.rs, and the tour note's printed
    V = 0.952915 m³): Steiner blank − 21·(cap + rim-torus extra),
    the extra derived by Pappus below."""

    DIE_R = 0.12  # the box-edge blend radius
    RIM_R = 0.02  # the pip-rim blend radius

    def blank_volume(self):
        """The blank's Steiner closed form: shrunk core + 6 slabs +
        12 quarter-cylinders + 8 sphere octants."""
        core = self.L - 2.0 * self.DIE_R
        return (
            core ** 3
            + 6.0 * self.DIE_R * core ** 2
            + 12.0 * (math.pi * self.DIE_R ** 2 / 4.0) * core
            + (4.0 / 3.0) * math.pi * self.DIE_R ** 3
        )

    @staticmethod
    def spherical_cap(r, h):
        """A height-`h` cap off a radius-`r` ball."""
        return math.pi * h * h * (3.0 * r - h) / 3.0

    def rim_fillet_extra(self):
        """The material one rim-torus fillet removes beyond the pip
        cap itself — the Pappus derivation `m6_surgery.rs` documents,
        ported verbatim: first moments of the curvilinear triangle
        between the old rim, the rolling ball's plane tangency, and
        its pip-ball tangency, times 2π."""
        big_r, h, r = self.PIP_R, self.PIP_H, self.RIM_R
        d = big_r - h
        s = math.sqrt((big_r + r) ** 2 - (d + r) ** 2)
        rho_rim = math.sqrt(big_r * big_r - d * d)
        k = big_r / (big_r + r)
        t_s = (s * k, d - (d + r) * k)

        def tri_m(a, b, c):
            area2 = (b[0] - a[0]) * (c[1] - a[1]) - (c[0] - a[0]) * (b[1] - a[1])
            return abs(area2) / 2.0 * ((a[0] + b[0] + c[0]) / 3.0)

        def seg_m(c, rad, p, q):
            a0 = math.atan2(p[1] - c[1], p[0] - c[0])
            a1 = math.atan2(q[1] - c[1], q[0] - c[0])
            if a1 - a0 > math.pi:
                a1 -= 2.0 * math.pi
            elif a0 - a1 > math.pi:
                a1 += 2.0 * math.pi
            lo, hi = min(a0, a1), max(a0, a1)
            sector = (
                c[0] * rad * rad * (hi - lo) / 2.0
                + rad ** 3 * (math.sin(hi) - math.sin(lo)) / 3.0
            )
            return sector - tri_m(c, p, q)

        p_rim = (rho_rim, 0.0)
        t_p = (s, 0.0)
        moment = (
            tri_m(p_rim, t_p, t_s)
            - seg_m((s, -r), r, t_p, t_s)
            - seg_m((0.0, d), big_r, p_rim, t_s)
        )
        return 2.0 * math.pi * moment

    def test_diecomposed_matches_the_scene_oracle(self):
        doc = Doc()
        die = self.pipped_die(doc)
        edges = Selector.of(NamePat.of_kind(EntityKind.Edge))

        # Blend 1 — the box edges, said by CARRIER KIND: of the pipped
        # cube's 96 edges (12 box lines + 21 pips × 4 circular arcs),
        # exactly the twelve lines. Materialized off THIS evaluation,
        # stored, frozen — and never read as text.
        straight = evaluate(doc).select_where(
            die, edges, [GeomPred.curve_kind(CurveKind.Line)]
        )
        self.assertEqual(len(straight), 12)
        blank = doc.insert(Node.fillet(die, self.DIE_R * m, straight))

        # Blend 2 — the pip rims, said by ADJACENT KINDS: the edges
        # whose two faces are a plane (the shrunk cap) and a sphere
        # (the pip cavity), unordered. A full revolve's band is two
        # half-faces, so each rim is two arcs: 21 × 2 = 42 names. The
        # cavity meridians (sphere on BOTH sides — no dihedral wedge,
        # unfilletable at any radius) match neither filter, exactly as
        # in the Rust scene.
        rims = evaluate(doc).select_where(
            blank,
            edges,
            [GeomPred.adjacent_kinds(SurfaceKind.Plane, SurfaceKind.Sphere)],
        )
        self.assertEqual(len(rims), 42)
        composed = doc.insert(Node.fillet(blank, self.RIM_R * m, rims))

        want = self.blank_volume() - 21.0 * (
            self.spherical_cap(self.PIP_R, self.PIP_H) + self.rim_fillet_extra()
        )
        # The tour note prints V to six figures; the derivation must
        # land on the same number, or it is not the scene's oracle.
        self.assertEqual(round(want, 6), 0.952915)
        self.assertAlmostEqual(volume_of(doc, composed), want, delta=1e-9 * want)


class TestDiechamferDie(DieScene, unittest.TestCase):
    """Tour scene `diechamfer`, the die stop (row 12): the pipped cube's
    twelve box edges CHAMFERED in place, the 21 pip cavities carried
    through as sharp rings.

    This row is what the scene's finding 2 asked for. The Rust scene
    has to say "the twelve box edges" as a hand-rolled carrier-kind
    loop over the kernel body, because `select_where` answers stable
    NAMES and `chamfer_edges` takes arena KEYS. Here the same
    `select_where` call feeds `Node.chamfer` directly — the identical
    line `TestDiecomposed` feeds `Node.fillet`, one verb over.

    The pip rims stay sharp on purpose: the chamfer's v1 door is
    plane-plane only, so a plane-sphere rim would refuse
    `ChamferArmUnsupported`. That is the door's scope, not an
    omission, and the scene says so."""

    DIE_D = 0.12  # the box-edge SETBACK, the filleted die's radius

    def test_the_box_edges_chamfer_through_select_where(self):
        doc = Doc()
        die = self.pipped_die(doc)
        edges = Selector.of(NamePat.of_kind(EntityKind.Edge))
        straight = evaluate(doc).select_where(
            die, edges, [GeomPred.curve_kind(CurveKind.Line)]
        )
        self.assertEqual(len(straight), 12)
        chamfered = doc.insert(Node.chamfer(die, self.DIE_D * m, straight))

        # It evaluates, and it is not the fillet: at the same size the
        # flat strip cuts the corner the ball rides around.
        filleted = doc.insert(Node.fillet(die, self.DIE_D * m, straight))
        self.assertLess(volume_of(doc, chamfered), volume_of(doc, filleted))

        # The scene's own census for the chamfered BOX, plus the 21
        # pip cavities carried through: the box contributes 26 faces
        # (6 shrunk supports + 12 strips + 8 corner patches) and each
        # pip cavity is a sphere in two half-faces.
        faces = evaluate(doc).all_faces(chamfered)
        self.assertEqual(len(faces), 26 + 21 * 2)


class TestTiltedcut(unittest.TestCase):
    """Tour scene `tiltedcut` (demos/tour/src/curvedcut.rs, row 16): a
    r = 1, h = 2.5 cylinder cut by a plane through its mid-height,
    tilted 0.3 rad. Both halves are bodies.

    The scene's own oracle is a BRACKET, not an equality: each half's
    exact volume pi*r^2*h/2 must lie inside the certified enclosure
    [v - pad, v + pad] the mass-properties door answers with."""

    R, H, PHI = 1.0, 2.5, 0.3

    def test_both_halves_bracket_the_exact_half_volume(self):
        doc = Doc()
        disc = doc.insert(Node.profile(circle((0 * m, 0 * m), self.R * m)))
        cylinder = doc.insert(Node.extrude(disc, self.H * m))
        plane = doc.insert(
            Node.datum_plane(
                (0 * m, 0 * m, (self.H / 2.0) * m),
                (math.sin(self.PHI), 0.0, math.cos(self.PHI)),
            )
        )
        cut = doc.insert(Node.split(cylinder, plane))

        above, below = evaluate(doc).value(cut).split()
        exact = math.pi * self.R * self.R * self.H / 2.0
        for name, half in (("above", above), ("below", below)):
            with self.subTest(half=name):
                self.assertIsNotNone(half)
                half.validate()
                props = half.mass_properties()
                self.assertLessEqual(props.volume - props.volume_pad, exact)
                self.assertLessEqual(exact, props.volume + props.volume_pad)

    def test_a_split_value_is_not_a_body(self):
        """The value's KIND is the honest one: a split denotes two
        sides, so `body()` refuses rather than picking one."""
        doc = Doc()
        disc = doc.insert(Node.profile(circle((0 * m, 0 * m), self.R * m)))
        cylinder = doc.insert(Node.extrude(disc, self.H * m))
        plane = doc.insert(
            Node.datum_plane((0 * m, 0 * m, 1 * m), (0.0, 0.0, 1.0))
        )
        cut = doc.insert(Node.split(cylinder, plane))
        value = evaluate(doc).value(cut)
        self.assertEqual(value.kind, "split")
        with self.assertRaises(EvaluationError):
            value.body()


class TestRocker(unittest.TestCase):
    """Tour scene `rocker` (row 7): a plate whose every corner is a
    fillet — five between the hub circle, the boss circle and the three
    straight sides, plus the eye slot's arc-by-arc tip.

    G12's row, and the last one the PATHS surface owed. Two of the
    outline's five corners arrive ON a carrier the fillet verb itself
    authors (`fillet_arc` with the `Center` mode); two DEPART one the
    verb re-authors from the tip's own bits (`arc_fillet` with the
    `Radius` mode) and arrive straight; the keel knee is the
    line-by-line seam. Not one corner is written down — every one is
    DERIVED from the two carriers.

    Oracle, the scene's own and exact: the eye is a HOLE, so the
    rocker's volume is the outline's prism less the eye's, and the
    solid's census is the tour's (26 vertices, 39 edges, 15 faces —
    genus 1). A far-pocket S8 pick or a lost seam vertex moves the
    census; a corner off its carriers moves the volume identity."""

    HUB_C, HUB_R = (0 * m, 0 * m), 2.5
    BOSS_C, BOSS_R = (7 * m, 0 * m), 1.5
    BLEND, KNEE, EYE = 0.5 * m, 0.5 * m, 0.25 * m
    DEPTH = 0.5 * m

    def outline(self):
        return (
            Open.at((5.05 * m, -1.6 * m))
            .toward(2.1, 0.8)
            .fillet_arc(self.BLEND, Center(self.BOSS_C, ArcSweep.Ccw, (8.5 * m, 0 * m)))
            .arc_fillet(Radius(self.BOSS_R * m, ArcSide.Left), self.BLEND)
            .at((4.05 * m, 1.35 * m))
            .toward(-4.1, 0.3)
            .fillet_arc(self.BLEND, Center(self.HUB_C, ArcSweep.Ccw, (-2.5 * m, 0 * m)))
            .arc_fillet(Radius(self.HUB_R * m, ArcSide.Left), self.BLEND)
            .at((3.0 * m, -1.75 * m))
            .toward(2.0, -0.5)
            .fillet(self.KNEE)
            .to(Start)
        )

    def eye(self):
        tip = math.sqrt(0.75)
        return (
            Open.arc_fillet_arc(
                Center((-0.5 * m, 0 * m), ArcSweep.Ccw, (0 * m, -tip * m)),
                self.EYE,
                Center((0.5 * m, 0 * m), ArcSweep.Ccw, Start),
            )
        )

    def prism(self, doc, loops):
        return doc.insert(Node.extrude(doc.insert(Node.profile(loops)), self.DEPTH))

    def test_rocker_matches_the_scene_oracle(self):
        doc = Doc()
        rocker = self.prism(doc, [self.outline(), self.eye()])
        plain = self.prism(doc, [self.outline()])
        slot = self.prism(doc, [self.eye()])
        self.assertAlmostEqual(
            volume_of(doc, rocker),
            volume_of(doc, plain) - volume_of(doc, slot),
            delta=1e-12,
        )
        ev = evaluate(doc)
        census = (
            len(ev.all_vertices(rocker)),
            len(ev.all_edges(rocker)),
            len(ev.all_faces(rocker)),
        )
        self.assertEqual(census, (26, 39, 15))

    def test_the_outline_is_ten_vertices_and_no_authored_corner(self):
        """The LB5 topology, positively: the hub arc is ONE segment,
        so the outline carries ten vertices — five fillet arcs, each
        with a trim point ahead of it — and the seam sits on the keel,
        where `.to(Start)` retrims the entry anchor away."""
        self.assertEqual(self.outline().vertex_count, 10)
        self.assertEqual(self.outline().step_count, 12)


class TestTable(unittest.TestCase):
    """Corpus scene `corner_table` (row 30): the corner-aligned
    four-leg table, authored through the DETECT/DECLARE protocol from
    Python (LIB-PYG5, G5) exactly as the Rust corpus authors it
    (`editor-core/tests/corpus/table.rs`): per leg, evaluate the
    document so far, `find_flush_candidates` between the accumulated
    body and the new leg, INSPECT the findings (the counts below are
    that inspection), `Doc.declare_all`, and wire the Declare id into
    the union. Nothing is fused; nothing parses a name.

    Exact oracles, derived as the corpus derives them (dyadic):
    volume = top 4·3·0.25 = 3, plus per leg 0.5·0.5·1.125 = 0.28125
    minus the slab overlap 0.5·0.5·0.125 = 0.03125 ⇒ +0.25 each ⇒
    4.0. Area = 27.5 + 4·2.75 − 4·0.625 (interior boundary removed
    per union) − 4·0.125 (double-counted coplanar wall overlap) =
    35.5. The finding inventory per leg is 2, 4, 5, 7: two corner
    wall planes each, plus every earlier leg's floor plane, plus one
    inner-wall plane per same-side earlier leg — all flush walls, the
    merge-stage SameOriented flavor."""

    def test_table_builds_through_detect_declare_and_matches_the_oracles(self):
        doc = Doc()
        top = slab(doc, (0, 4), (0, 3), (1, 1.25))
        legs = [
            ((0.0, 0.5), (0.0, 0.5)),
            ((3.5, 4.0), (0.0, 0.5)),
            ((3.5, 4.0), (2.5, 3.0)),
            ((0.0, 0.5), (2.5, 3.0)),
        ]
        acc = top
        for i, (x, y) in enumerate(legs):
            leg = slab(doc, x, y, (0, 1.125))
            # The protocol: evaluate, detect, INSPECT, declare —
            # findings pass through the author's hands as values.
            ev = evaluate(doc)
            findings = ev.find_flush_candidates(acc, leg)
            self.assertEqual(len(findings), [2, 4, 5, 7][i])
            for f in findings:
                self.assertEqual(f.relation, PlaneRelation.SameOriented)
                self.assertEqual(f.class_, ContactClass.Rest)
            decl = doc.declare_all(findings)
            acc = doc.insert(Node.boolean(BooleanOp.Union, acc, leg, declare=decl))
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(acc))
        body = ev.value(acc).body()
        body.validate()
        props = body.mass_properties()
        # Dyadic scene: both oracles hold EXACTLY, as the Rust corpus
        # pins them (4.0 / 35.5).
        self.assertEqual(props.volume, 4.0)
        self.assertEqual(props.surface_area, 35.5)


class TestCrosslapGlued(unittest.TestCase):
    """Tour scene `crosslap` (row 37): the two notched beams MATED.
    Undeclared, the mate refuses at the coincidence door — since
    register R3 as the typed MENU (`kind == "undeclared_contact"`,
    the candidate declaration attached). The recourse the menu names
    is executed: detect, INSPECT (the joint's mate is the
    resting-contact class — the notch floor/ceiling and the four
    crossing walls, all `SameOpposite`), declare, and the SAME union
    glues through the declared-REST zip at the scene's exact oracle
    2·(BEAM_VOL − NOTCH_VOL) = 1.875 (`demos/tour/src/crosslap.rs`
    asserts the same both ways).

    The inspection step EARNS ITS KEEP here, and honestly: the
    detector also reports the beams' coplanar exteriors (bottoms at
    z=0, tops at z=0.5 — `SameOriented`, the merge-stage flavor), and
    declaring the BOTTOM pairs trips a document-layer naming-emitter
    wall (`kind == "naming"`) after the kernel glues fine — a
    measured residue pinned below, not hidden. The scene's statement
    (the mate) needs none of those pairs; its oracle holds exactly."""

    def beams(self, doc):
        beam_a = doc.insert(
            Node.boolean(
                BooleanOp.Subtract,
                slab(doc, (0, 4), (1.75, 2.25), (0, 0.5)),
                slab(doc, (1.75, 2.25), (1.5, 2.5), (0.25, 0.75)),
            )
        )
        beam_b = doc.insert(
            Node.boolean(
                BooleanOp.Subtract,
                slab(doc, (1.75, 2.25), (0, 4), (0, 0.5)),
                slab(doc, (1.5, 2.5), (1.75, 2.25), (-0.25, 0.25)),
            )
        )
        return beam_a, beam_b

    def test_the_mate_refuses_undeclared_then_glues_declared(self):
        doc = Doc()
        beam_a, beam_b = self.beams(doc)
        naive = doc.insert(Node.boolean(BooleanOp.Union, beam_a, beam_b))
        ev = evaluate(doc)
        self.assertFalse(ev.succeeded(naive))
        with self.assertRaises(EvaluationError) as caught:
            ev.value(naive)
        self.assertEqual(caught.exception.kind, "undeclared_contact")
        menu = caught.exception.finding
        self.assertIsNotNone(menu)

        # The menu's declare arm, executed: the detector reports the
        # joint's whole flush inventory, the menu's own finding among
        # them. The INSPECTION narrows to the mate itself — the
        # resting-contact class, a typed field, no name ever read:
        # the notch floor/ceiling and the four crossing walls.
        findings = ev.find_flush_candidates(beam_a, beam_b)
        self.assertIn(menu, findings)
        mate = [f for f in findings if f.relation == PlaneRelation.SameOpposite]
        self.assertEqual(len(mate), 5)
        decl = doc.declare_all(mate)
        glued = doc.insert(
            Node.boolean(BooleanOp.Union, beam_a, beam_b, declare=decl)
        )
        # 2·(4·0.5·0.5 − 0.5·0.5·0.25) = 1.875, exactly (dyadic).
        self.assertEqual(volume_of(doc, glued), 1.875)

    def test_the_merge_stage_bottom_declaration_hits_the_naming_wall(self):
        """The measured residue, pinned so its fall is loud: declare
        the detector's FULL inventory — mate plus the merge-stage
        `SameOriented` exteriors — and the kernel glues, but the
        boolean node still fails in the document layer's NAMING
        emitter (the bottom-plane pairs: one beam-A face merging with
        one of beam B's two coplanar bottom halves). When this test
        fails with the union succeeding, the wall has fallen — flip
        this scene's declaration back to the whole inventory and drop
        the inspection narrowing above."""
        doc = Doc()
        beam_a, beam_b = self.beams(doc)
        ev = evaluate(doc)
        findings = ev.find_flush_candidates(beam_a, beam_b)
        self.assertEqual(len(findings), 9)
        decl = doc.declare_all(findings)
        glued = doc.insert(
            Node.boolean(BooleanOp.Union, beam_a, beam_b, declare=decl)
        )
        ev = evaluate(doc)
        self.assertFalse(ev.succeeded(glued))
        with self.assertRaises(EvaluationError) as caught:
            ev.value(glued)
        self.assertEqual(caught.exception.kind, "naming")


class TestCrosslapExploded(unittest.TestCase):
    """Tour scene `crosslap_exploded` (row 38): two notched beams, the
    second LIFTED clear so the joint reads. The lift was the whole
    reason this row was YES* — hand-authoring beam B a quarter-metre
    up said the same body and lost the placement — and
    `Node.transform` is what makes it the scene's own statement.

    The scene's oracle, per beam: BEAM_VOL - NOTCH_VOL = 0.9375,
    exactly (crates/topo/tests/crosslap_rest.rs asserts equality)."""

    def test_the_lift_is_a_placement_and_preserves_the_beam(self):
        doc = Doc()
        beam_a = doc.insert(
            Node.boolean(
                BooleanOp.Subtract,
                slab(doc, (0, 4), (1.75, 2.25), (0, 0.5)),
                slab(doc, (1.75, 2.25), (1.5, 2.5), (0.25, 0.75)),
            )
        )
        beam_b = doc.insert(
            Node.boolean(
                BooleanOp.Subtract,
                slab(doc, (1.75, 2.25), (0, 4), (0, 0.5)),
                slab(doc, (1.5, 2.5), (1.75, 2.25), (-0.25, 0.25)),
            )
        )
        # A pure translation still names an axis: the evaluator
        # normalizes it, and a zero-length one refuses rather than
        # being read as "no rotation".
        lifted = doc.insert(
            Node.transform(beam_b, (0 * m, 0 * m, 1.25 * m), (0.0, 0.0, 1.0), 0 * rad)
        )
        expected = 4.0 * 0.5 * 0.5 - 0.5 * 0.5 * 0.25
        self.assertEqual(expected, 0.9375)
        self.assertAlmostEqual(volume_of(doc, beam_a), expected, delta=1e-12)
        self.assertAlmostEqual(volume_of(doc, lifted), expected, delta=1e-12)

    def test_a_degenerate_rotation_axis_refuses(self):
        doc = Doc()
        box = slab(doc, (0, 1), (0, 1), (0, 1))
        bad = doc.insert(
            Node.transform(box, (0 * m, 0 * m, 1 * m), (0.0, 0.0, 0.0), 0 * rad)
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(bad)
        self.assertEqual(caught.exception.kind, "degenerate_direction")


# ------------------------------------------------------------------
# The rows the ROSTER RE-CUT added. The audit's table had drifted 13
# stops behind the tour (`crates/pncad/tests/all.rs::
# the_north_star_audit_has_a_row_for_every_tour_stop` is what now
# stops that happening); four of those stops graded YES, and these are
# their oracles.
#
# Each is held to whatever its scene actually pins. Two of the four
# scenes carry a closed form and are checked against it; the other two
# carry census and band pins instead, and those are what is asserted —
# never a number invented here to have one.
# ------------------------------------------------------------------


class TestHollowring(unittest.TestCase):
    """Tour scene `hollowring` (demos/tour/src/ring.rs, row 25): an
    annulus revolved a full turn — a tube bent into a closed circle,
    hollow all the way round, in ONE revolve of a HOLED profile.

    This is the easiest row in the tour to be sure of, because the
    Rust scene settles it itself: `ring::through_the_document` builds
    the same ring as a three-node recipe — a two-loop `Profile`, a
    `Datum::Axis`, a full `Revolve` — and the scene asserts the plain
    door and the recipe agree on volume EXACTLY, not merely on census.
    Those three nodes are exactly what Python binds, so this rebuilds
    that recipe and checks the scene's own oracles.

    The torus closed forms, outer minus bore:
    V = 2π²R(rₒ² − rᵢ²) and A = 4π²R(rₒ + rᵢ). A body that had quietly
    built as a plain torus misses the volume by the bore and the area
    by the inner wall."""

    R: ClassVar[float] = 0.30
    RO: ClassVar[float] = 0.07
    RI: ClassVar[float] = 0.05

    def ring(self, doc):
        centre = (self.R * m, 0 * m)
        profile = doc.insert(
            Node.profile(
                [circle(centre, self.RO * m), circle(centre, self.RI * m)]
            )
        )
        axis = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0)))
        return doc.insert(Node.revolve(profile, axis, 360 * deg))

    def test_hollowring_matches_the_torus_closed_forms(self):
        doc = Doc()
        ring = self.ring(doc)
        body = evaluate(doc).value(ring).body()
        body.validate()
        props = body.mass_properties()
        v_want = 2 * math.pi**2 * self.R * (self.RO**2 - self.RI**2)
        a_want = 4 * math.pi**2 * self.R * (self.RO + self.RI)
        self.assertLess(abs(props.volume - v_want) / v_want, 1e-12)
        self.assertLess(abs(props.surface_area - a_want) / a_want, 1e-12)
        # A plain torus of the outer radius would measure this much
        # more — the assertion above is discriminating, not a
        # coincidence of scale.
        v_solid = 2 * math.pi**2 * self.R * self.RO**2
        self.assertGreater(v_solid - v_want, 0.4 * v_want)

    def test_the_census_is_the_scenes_absolute_pin(self):
        """Two shells, each a two-arc profile fully revolved: 2
        half-tube walls, 2 seam meridians, 2 full-period rims, 2
        vertices — so the solid carries twice that. The scene pins
        (4, 8, 4) absolutely; a face appearing or vanishing moves it.

        What the Python row CANNOT see, stated rather than skipped:
        the shell decomposition itself. `all_bodies` answers one body
        (the ring is one solid), and there is no per-shell door in the
        bindings, so the scene's `classify_shells` reading of the
        cavity's own negated volume has no Python form. The census and
        the closed forms are what crosses."""
        doc = Doc()
        ring = self.ring(doc)
        ev = evaluate(doc)
        self.assertEqual(len(ev.all_vertices(ring)), 4)
        self.assertEqual(len(ev.all_edges(ring)), 8)
        self.assertEqual(len(ev.all_faces(ring)), 4)
        self.assertEqual(len(ev.all_bodies(ring)), 1)


class TestKlein(unittest.TestCase):
    """Tour scene `klein` (demos/tour/src/klein.rs, row 15): the
    non-orientable stop, as the honest 3-D stand-in — a thin
    3-manifold whose midsurface is the classic immersed Klein bottle.
    Three bodies, three revolves, NO boolean and NO fillet_edges.

    The bulb is one FULL revolve of one meridian band, and that band
    is the reason this row is interesting: it walks the neck down,
    blends, flares, turns through the wide rim, comes back up the
    inner tube and closes — `.toward`/`.fillet`/`.to`/`.tangent`/
    `.tangent_arc_to`/`.line` — and every one of those verbs is on the
    bound lattice, in an order the lattice admits. The two elbows are
    a two-loop (annular) profile revolved PARTIALLY about a datum axis
    at a NEGATIVE angle.

    Oracles: the elbows carry a Pappus closed form the scene asserts
    (the annulus area times the spine length, exactly, because the
    centroid is ON the spine). The bulb carries none, so this row
    asserts the scene's own discriminating pin instead — twelve faces,
    of which exactly four are cylinders: the neck wall and the inner
    tube wall are the SAME cylinder about the SAME axis, and the
    revolve's cosurface merge is a run-ADJACENCY decision, so each of
    the four runs keeps its own face."""

    R: ClassVar[float] = 0.25
    WALL: ClassVar[float] = 0.05
    ALPHA: ClassVar[float] = 30.0 * math.pi / 180.0
    ZTOP: ClassVar[float] = 3.0
    ZNECK: ClassVar[float] = 2.5
    RF: ClassVar[float] = 0.30
    RRIM: ClassVar[float] = 0.80
    RLOOP: ClassVar[float] = 1.20
    SWEEP_OVER: ClassVar[float] = 1.5 * math.pi
    SWEEP_IN: ClassVar[float] = 0.5 * math.pi

    def meridian(self):
        """The band's derived geometry, in sketch coordinates
        (radius, height) — the scene's own `meridian_at`, at the
        bottle's own proportions."""
        half = self.WALL / 2.0
        sa, ca = math.sin(self.ALPHA), math.cos(self.ALPHA)
        gx = self.R + self.RRIM * (1.0 + ca)
        gz = self.ZNECK - self.RRIM * (1.0 + ca) * ca / sa
        rim_z = gz - self.RRIM * sa
        return {
            "ro": self.R + half,
            "ri": self.R - half,
            "dir": (sa, -ca),
            "g_out": (gx + half * ca, gz + half * sa),
            "g_in": (gx - half * ca, gz - half * sa),
            "h_from_in": (self.R + half, rim_z),
            "h_from_out": (self.R - half, rim_z),
            "rim_z": rim_z,
            "z_tube": self.ZTOP - 2.0 * self.RLOOP,
        }

    def band(self, md):
        half = self.WALL / 2.0

        def p(xy):
            return (xy[0] * m, xy[1] * m)

        return (
            Open.at((md["ri"] * m, self.ZTOP * m))
            .toward(0.0, -1.0)
            .fillet((self.RF + half) * m)
            .toward(md["dir"][0], md["dir"][1])
            .to(p(md["g_in"]))
            .tangent()
            .tangent_arc_to(p(md["h_from_in"]))
            .tangent()
            .line((md["z_tube"] - md["rim_z"]) * m)
            .line_to((md["ri"] * m, md["z_tube"] * m))
            .line_to(p(md["h_from_out"]))
            .tangent()
            .tangent_arc_to(p(md["g_out"]))
            .tangent()
            .fillet((self.RF - half) * m)
            .toward(0.0, 1.0)
            .to((md["ro"] * m, self.ZTOP * m))
            .line_to(Start)
        )

    def bottle(self, doc):
        """The three bodies, in surface order: bulb, then the loop's
        two arcs."""
        md = self.meridian()
        # The bulb's sketch is the xz half-plane and its axis is the
        # plane's own +v, which is world +z.
        bulb_plane = SketchPlane.from_frame(
            (0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0)
        )
        band = doc.insert(Node.profile(self.band(md), plane=bulb_plane))
        axis = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0)))
        bulb = doc.insert(Node.revolve(band, axis, 2 * math.pi * rad))

        half = self.WALL / 2.0

        def elbow(z0, sweep):
            # HORIZONTAL sketch at the elbow's own end: the only frame
            # in which the annular section and the elbow axis are in
            # one plane, which is what a revolve needs. The angle is
            # negative because the axis is -y (the scene's own note).
            plane = SketchPlane.from_frame(
                (0 * m, 0 * m, z0 * m), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)
            )
            annulus = doc.insert(
                Node.profile(
                    [
                        circle((0 * m, 0 * m), (self.R + half) * m),
                        circle((0 * m, 0 * m), (self.R - half) * m),
                    ],
                    plane=plane,
                )
            )
            ax = doc.insert(
                Node.datum_axis(
                    (self.RLOOP * m, 0 * m, z0 * m), (0.0, -1.0, 0.0)
                )
            )
            return doc.insert(Node.revolve(annulus, ax, (-sweep) * rad))

        return bulb, elbow(self.ZTOP, self.SWEEP_OVER), elbow(
            md["z_tube"], self.SWEEP_IN
        )

    def test_the_two_elbows_match_the_scenes_pappus_oracle(self):
        doc = Doc()
        _, over, into = self.bottle(doc)
        ev = evaluate(doc)
        ring = math.pi * (
            (self.R + self.WALL / 2.0) ** 2 - (self.R - self.WALL / 2.0) ** 2
        )
        for node, sweep in ((over, self.SWEEP_OVER), (into, self.SWEEP_IN)):
            body = ev.value(node).body()
            body.validate()
            want = ring * sweep * self.RLOOP
            self.assertAlmostEqual(
                body.mass_properties().volume, want, delta=1e-12
            )

    def test_the_bulb_is_the_scenes_twelve_faces_four_of_them_cylinders(self):
        doc = Doc()
        bulb, _, _ = self.bottle(doc)
        ev = evaluate(doc)
        ev.value(bulb).body().validate()
        self.assertEqual(len(ev.all_faces(bulb)), 12)
        faces = Selector.of(NamePat.of_kind(EntityKind.Face))
        for kind, count, what in (
            (SurfaceKind.Cylinder, 4, "neck + inner tube, two walls each"),
            (SurfaceKind.Torus, 4, "the two blends and the rim, walled"),
            (SurfaceKind.Plane, 2, "the two annular rims"),
            (SurfaceKind.Cone, 2, "the flare, walled"),
        ):
            self.assertEqual(
                len(ev.select_where(bulb, faces, [GeomPred.surface_kind(kind)])),
                count,
                what,
            )


class TestBudfillet(unittest.TestCase):
    """Tour scene `budfillet` (demos/tour/src/bud.rs, row 14): the
    calochortus bud as a bored solid of revolution — sphere zone,
    conical pucker, lip disk, bore — with three latitude rims rolled
    through three different arms of the CURVED-support fillet family
    (sphere×cone, cone×plane, cylinder×plane).

    Two things had to be true for this row, and both are executed
    here. The document layer's `Node.fillet` reaches the curved arms
    unchanged, and the rims can be named the way the scene names
    them — BY DESCRIPTION. The Rust scene scans its own arena through
    two back-pointers because a directly revolved body has no
    selector; from Python the description IS the selector,
    `select_where(adjacent_kinds(...))`, plus a `datum_distance`
    station where the description is ambiguous: `(Cylinder, Plane)`
    names the bore's base rim and its top rim both.

    The scene's grain is per DISJOINT SET, not per rim — the mouth and
    the lip share the pucker cone, so they cannot roll in one call —
    and this row follows the same two calls the scene's recourse
    names. No closed form exists for the body, so the oracles are the
    scene's own: the census before and after, one torus band per rim,
    and the volume drop inside the scene's Pappus bracket."""

    BORE: ClassVar[float] = 0.2
    GLOBE: ClassVar[float] = 1.0
    MOUTH: ClassVar[tuple] = (0.8, 0.6)
    LIP_R: ClassVar[float] = 0.35
    TOP: ClassVar[float] = 0.75
    ROLL: ClassVar[float] = 0.05

    def sharp(self, doc):
        meridian = (
            Open.at((self.BORE * m, 0 * m))
            .line_to((self.GLOBE * m, 0 * m))
            .arc_to(
                Center(
                    c=(0 * m, 0 * m),
                    winding=ArcSweep.Ccw,
                    p=(self.MOUTH[0] * m, self.MOUTH[1] * m),
                )
            )
            .line_to((self.LIP_R * m, self.TOP * m))
            .line_to((self.BORE * m, self.TOP * m))
            .line_to(Start)
        )
        profile = doc.insert(Node.profile(meridian))
        axis = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0)))
        return doc.insert(Node.revolve(profile, axis, 2 * math.pi * rad))

    def test_three_curved_rims_roll_in_the_scenes_two_calls(self):
        doc = Doc()
        sharp = self.sharp(doc)
        base_plane = doc.insert(
            Node.datum_plane((0 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0))
        )
        ev = evaluate(doc)
        # The unfilleted twin, built explicitly so the comparison
        # below is between two bodies and not against a remembered
        # number (the scene's own discipline).
        self.assertEqual(len(ev.all_vertices(sharp)), 5)
        self.assertEqual(len(ev.all_edges(sharp)), 10)
        self.assertEqual(len(ev.all_faces(sharp)), 5)
        sharp_volume = ev.value(sharp).body().mass_properties().volume

        edges = Selector.of(NamePat.of_kind(EntityKind.Edge))
        mouth = ev.select_where(
            sharp,
            edges,
            [GeomPred.adjacent_kinds(SurfaceKind.Sphere, SurfaceKind.Cone)],
        )
        self.assertEqual(len(mouth), 1, "the description names one rim")
        first = doc.insert(Node.fillet(sharp, self.ROLL * m, mouth))

        ev = evaluate(doc)
        lip = ev.select_where(
            first,
            edges,
            [GeomPred.adjacent_kinds(SurfaceKind.Cone, SurfaceKind.Plane)],
        )
        self.assertEqual(len(lip), 1)
        # `(Cylinder, Plane)` is AMBIGUOUS at the bore — it names the
        # base rim and the top rim both — so the axial station picks
        # the one the scene rolls, exactly as its `rim_station` sort
        # does kernel-side.
        both = ev.select_where(
            first,
            edges,
            [GeomPred.adjacent_kinds(SurfaceKind.Cylinder, SurfaceKind.Plane)],
        )
        self.assertEqual(len(both), 2)
        base = ev.select_where(
            first,
            edges,
            [
                GeomPred.adjacent_kinds(SurfaceKind.Cylinder, SurfaceKind.Plane),
                GeomPred.datum_distance(base_plane, Cmp.Approx, 0 * m),
            ],
        )
        self.assertEqual(len(base), 1)

        rolled = doc.insert(Node.fillet(first, self.ROLL * m, lip + base))
        ev = evaluate(doc)
        body = ev.value(rolled).body()
        body.validate()

        # Proof 1: three annulus bands, each (+1 vertex, +2 edges,
        # +1 face) over the sharp bud.
        self.assertEqual(len(ev.all_vertices(rolled)), 8)
        self.assertEqual(len(ev.all_edges(rolled)), 16)
        self.assertEqual(len(ev.all_faces(rolled)), 8)

        # Proof 2: the band faces exist and are TORI — three of them,
        # over the two calls. A silhouette that did not move cannot
        # say this; three new revolution walls can only be there or
        # not.
        faces = Selector.of(NamePat.of_kind(EntityKind.Face))
        self.assertEqual(
            len(
                ev.select_where(
                    rolled, faces, [GeomPred.surface_kind(SurfaceKind.Torus)]
                )
            ),
            3,
        )

        # Proof 4: mass properties move, against the twin, inside the
        # scene's own bracket — a convex rim's roll REMOVES material,
        # and no more than Pappus over the three rims' own radii.
        removed = sharp_volume - body.mass_properties().volume
        cap = sum(
            2 * math.pi * r * self.ROLL**2
            for r in (self.MOUTH[0], self.LIP_R, self.BORE)
        )
        self.assertGreater(removed, 0.0)
        self.assertLess(removed, cap)


class TestTwopeg(unittest.TestCase):
    """Tour scenes `twopeg_apart` (row 40, YES) and `twopeg` (row 39,
    NO — G19), demos/tour/src/twopeg.rs: two plates that locate on
    each other three ways at once — the mating plane, and each peg's
    wall against its own bore's wall.

    The two PARTS are ordinary work: plate P is a plate with two pegs
    unioned on (transverse curved booleans, `bossplate`'s lane), plate
    Q the same plate with two through-bores subtracted, and the apart
    framing lifts Q by a rigid transform. All of that is bound, which
    is why `twopeg_apart` is a YES row.

    The MATE is not, and the reason is G19, pinned below. Its three
    declared contacts are one planar `Rest` and two CYLINDRICAL ones,
    and Python can say only the planar third."""

    PLATE: ClassVar[tuple] = (6.0, 4.0, 1.0)
    PEG_R: ClassVar[float] = 0.5
    PEG_X: ClassVar[tuple] = (2.0, 4.0)
    PEG_Y: ClassVar[float] = 2.0
    ENGAGE: ClassVar[float] = 1.0

    def plate(self, doc, z0):
        x, y, _ = self.PLATE
        outline = (
            Open.at((0 * m, 0 * m))
            .line_to((x * m, 0 * m))
            .line_to((x * m, y * m))
            .line_to((0 * m, y * m))
            .line_to(Start)
        )
        profile = doc.insert(Node.profile(outline, elevation=z0 * m))
        return doc.insert(Node.extrude(profile, self.PLATE[2] * m))

    def peg(self, doc, cx, z0, h):
        """The radius-0.5 rim as THREE 120° arcs of one carrier —
        `circle_split`, as the scene writes it, because the split
        count is part of what the seam looks like."""
        rim = circle_split((cx * m, self.PEG_Y * m), self.PEG_R * m, 3, 0 * deg)
        profile = doc.insert(Node.profile(rim, elevation=z0 * m))
        return doc.insert(Node.extrude(profile, h * m))

    def parts(self, doc):
        plain = self.PLATE[0] * self.PLATE[1] * self.PLATE[2]
        stub = math.pi * self.PEG_R**2 * self.ENGAGE
        p = self.plate(doc, 0.0)
        for cx in self.PEG_X:
            boss = self.peg(
                doc, cx, 0.4, self.PLATE[2] - 0.4 + self.ENGAGE
            )
            p = doc.insert(Node.boolean(BooleanOp.Union, p, boss))
        q = self.plate(doc, self.PLATE[2])
        for cx in self.PEG_X:
            cutter = self.peg(doc, cx, self.PLATE[2] - 0.2, self.PLATE[2] + 0.4)
            q = doc.insert(Node.boolean(BooleanOp.Subtract, q, cutter))
        return p, q, plain + 2 * stub, plain - 2 * stub

    def test_twopeg_apart_is_two_parts_and_a_rigid_lift(self):
        doc = Doc()
        p, q, v_p, v_q = self.parts(doc)
        lifted = doc.insert(
            Node.transform(q, (0 * m, 0 * m, 1.6 * m), (0.0, 0.0, 1.0), 0 * deg)
        )
        ev = evaluate(doc)
        for node, want in ((p, v_p), (q, v_q), (lifted, v_q)):
            body = ev.value(node).body()
            body.validate()
            self.assertAlmostEqual(
                body.mass_properties().volume, want, delta=1e-12
            )

    def test_the_mate_has_no_python_path_because_the_detector_is_planar(self):
        """G19, pinned as the audit's NO rows are.

        Three assertions, and together they are the gap: the detector
        reports only PLANAR pairs on this scene's two parts (one
        `SameOpposite` — the mating plane — and six `SameOriented`
        merge-stage walls, and NOT the four cylindrical patches the
        scene declares); declaring every one of them still refuses;
        and the refusal is the CURVED-face arm, which is precisely
        what the scene says a cylindrical declaration unlocks.

        The day the detector grows its curved arm this test fails —
        with the union succeeding at the scene's exactly-additive
        oracle, 2·6·4·1 = 48 — and row 39 is promoted."""
        doc = Doc()
        p, q, _, _ = self.parts(doc)
        ev = evaluate(doc)
        findings = ev.find_flush_candidates(p, q)
        self.assertEqual(len(findings), 7)
        self.assertEqual(
            sum(1 for f in findings if f.relation == PlaneRelation.SameOpposite), 1
        )
        self.assertTrue(all(f.class_ == ContactClass.Rest for f in findings))

        declared = doc.insert(
            Node.boolean(
                BooleanOp.Union, p, q, declare=doc.declare_all(findings)
            )
        )
        ev = evaluate(doc)
        self.assertFalse(ev.succeeded(declared))
        with self.assertRaises(EvaluationError) as caught:
            ev.value(declared)
        self.assertEqual(caught.exception.kind, "boolean")
        self.assertIn("curved face", str(caught.exception))

    def test_a_finding_is_the_only_route_to_a_declaration_and_is_planar(self):
        """The other half of G19, on the smallest shape that shows it.

        A solid cylinder standing inside a block's bore of the SAME
        radius is a cylindrical `Rest` and nothing else: the block
        overshoots both ways, so no plane of one coincides with a
        plane of the other. The detector reports NOTHING, because its
        probe answers `None` for any curved carrier — and a
        `FlushFinding` cannot be built by hand, so there is no second
        route to the declaration."""
        import pncad

        doc = Doc()
        peg_p = doc.insert(Node.profile(circle((0 * m, 0 * m), 1 * m), elevation=0 * m))
        peg = doc.insert(Node.extrude(peg_p, 1 * m))
        block_outline = (
            Open.at((-3 * m, -3 * m))
            .line_to((3 * m, -3 * m))
            .line_to((3 * m, 3 * m))
            .line_to((-3 * m, 3 * m))
            .line_to(Start)
        )
        block_p = doc.insert(
            Node.profile(
                [block_outline, circle((0 * m, 0 * m), 1 * m)], elevation=-1 * m
            )
        )
        block = doc.insert(Node.extrude(block_p, 3 * m))
        ev = evaluate(doc)
        self.assertEqual(ev.find_flush_candidates(peg, block), [])
        with self.assertRaises(TypeError):
            pncad.FlushFinding()


# ------------------------------------------------------------------
# The gap LIB-G11 closed: G11, the ladder's steps 4 and 5.
#
# G11 was never a STOP's blocker — it is the one gap this page
# anchors to the generic ladder every scene is held to (*author →
# validate → measure → tessellate → cross-check → export*), which is
# why closing it moves no mark. What it buys is that the ladder now
# runs whole from Python, so this class runs it whole on two scenes
# the page already grades YES, and cross-checks each against the
# oracle its row asserts.
#
# The mesh measure is the CALLER's own divergence-theorem sum over
# the bound triangles (`test_mesh.mesh_signed_volume`), and closure
# is decided on shared position INDICES
# (`test_mesh.unmatched_half_edges`) — two computations that touch no
# kernel measure, which is what makes agreeing with one evidence.
# ------------------------------------------------------------------

from test_mesh import mesh_signed_volume, unmatched_half_edges  # noqa: E402


class TestMeshCrossCheck(unittest.TestCase):
    def test_chute_meshes_to_its_own_exact_volume(self):
        """Row 6's scene, all six rungs. `chute` is the page's own
        cleanest YES — one profile, one revolve, no booleans — and it
        is CURVED, so the mesh measure approaches the exact one from
        inside rather than reproducing it."""
        poly = [
            (1.0, 0.0), (1.75, 0.0), (1.75, 0.625), (1.5625, 0.625),
            (1.5625, 0.1875), (1.1875, 0.1875), (1.1875, 0.625), (1.0, 0.625),
        ]
        doc = Doc()
        profile = doc.insert(Node.polygon([(x * m, y * m) for x, y in poly]))
        axis = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0)))
        chute = doc.insert(Node.revolve(profile, axis, 270 * deg))

        body = evaluate(doc).value(chute).body()
        body.validate()
        exact = body.mass_properties().volume
        self.assertAlmostEqual(exact, (1287 / 2048) * math.pi, delta=1e-12)

        # Measured: 1.2e-4 relative at 0.5 mm, 2.4e-5 at 0.1 mm —
        # first order in δ, as the chordal certificate says. The pin
        # is one significant figure clear of the measurement.
        mesh = body.tessellate(0.1 * mm)
        self.assertEqual(unmatched_half_edges(mesh), [])
        measured = mesh_signed_volume(mesh)
        self.assertGreater(measured, 0.0, "the winding is outward")
        self.assertLess(abs(measured - exact) / exact, 1e-4)

    def test_the_letterform_prism_meshes_exactly(self):
        """Row 32's `T`: every face is planar, so the triangulation is
        EXACT and the two measures agree at rounding level. The scene's
        dyadic oracle is asserted of both."""
        t_plane = SketchPlane.from_frame(
            (-0.25 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)
        )
        letter = [
            (1.1875, 0.125), (1.8125, 0.125), (1.8125, 2.625), (3.25, 2.625),
            (3.25, 3.125), (-0.25, 3.125), (-0.25, 2.5625), (1.1875, 2.5625),
        ]
        doc = Doc()
        sketch = doc.insert(
            Node.polygon([(a * m, b * m) for a, b in letter], plane=t_plane)
        )
        prism = doc.insert(Node.extrude(sketch, 2.5 * m))

        body = evaluate(doc).value(prism).body()
        body.validate()
        self.assertAlmostEqual(
            body.mass_properties().volume, 8.505859375, delta=1e-12
        )

        mesh = body.tessellate(1 * mm)
        self.assertEqual(unmatched_half_edges(mesh), [])
        self.assertLess(abs(mesh_signed_volume(mesh) - 8.505859375), 1e-12)

    def test_the_mesh_and_the_stl_agree_facet_for_facet(self):
        """Step 6 for the mesh half: the binary file's declared facet
        count is the mesh's own, so what was exported is what was
        cross-checked."""
        doc = Doc()
        profile = doc.insert(
            Node.polygon(
                [(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)]
            )
        )
        cube = doc.insert(Node.extrude(profile, 1 * m))
        mesh = evaluate(doc).value(cube).body().tessellate(1 * mm)
        data = mesh.to_stl_binary(header="pncad north-star audit")
        self.assertEqual(
            int.from_bytes(data[80:84], "little"), mesh.triangle_count
        )
        self.assertEqual(
            mesh.to_stl_ascii(solid_name="cube").count("facet normal"),
            mesh.triangle_count,
        )


class TestNamedGapsAreStillGaps(unittest.TestCase):
    """The NO rows' gaps, asserted as absences.

    If one of these fails, a door has been BUILT — good news, and the
    signal to promote the corresponding rows in the audit from NO to
    YES."""

    def test_the_bound_vocabulary_is_exactly_this(self):
        self.assertEqual(
            sorted(n for n in dir(Node) if not n.startswith("_")),
            [
                "boolean", "chamfer", "datum_axis", "datum_plane", "declare",
                "extrude", "fillet", "instantiate_part", "loft", "mate",
                "placed_union",
                "placed_union_at", "polygon", "profile", "revolve", "split",
                "transform",
            ],
        )
        self.assertEqual(
            sorted(n for n in dir(DocEdit) if not n.startswith("_")),
            [
                "bind_count_param", "delete_node", "insert_node",
                "set_doc_param", "set_doc_param_value", "set_placement",
                "set_roots", "set_tolerance", "update_reference",
            ],
        )

    def test_the_named_gaps_are_still_gaps(self):
        import pncad

        # `ParamName`/`DocParam` left this list when G10 closed
        # (R1-PARAMS) — `TestPlateParam` above is the positive form.
        # `Selector`/`select_where` left it when G13 closed
        # (LIB-PYSEL) — `TestDiecomposed` and
        # `test_the_selector_surface_narrows_without_reading_names`
        # are the positive forms; `select`/`select_where` are
        # `Evaluation` METHODS (the materializer posture `all_edges`
        # set), so the module-level absence below is shape, not gap —
        # and `find_flush_candidates` joined them when G5 closed
        # (LIB-PYG5): the detector is an `Evaluation` method too
        # (`TestTable`/`TestCrosslapGlued` are the positive forms).
        # `StableName` stays: a name is CARRIED as text, never
        # composed, so there is no name type and no name grammar —
        # a `FlushFinding`'s pair crosses as the same opaque texts.
        for door in [
            # `Mesh`/`TessellateError` left this list when G11 closed
            # (LIB-G11) — `TestMeshCrossCheck` below is the positive
            # form, and `tests/test_mesh.py` is the door's own suite.
            # `tessellate` and the two STL writers stay, as `select`
            # and `find_flush_candidates` do: they are METHODS on the
            # value they take (`Body.tessellate`, `Mesh.to_stl_ascii`,
            # `Mesh.to_stl_binary`), so the module-level absence below
            # is shape, not gap.
            "tessellate", "write_ascii", "write_binary",
            "select", "select_where",                 # methods, not module doors
            "find_flush_candidates",                  # method, not a module door
            "StableName",                             # names stay text
            # `Workspace`, `ContentPin`, `DocRef` and
            # `random_document_id` LEFT this list when G15 closed
            # (LIB-G15) — the positive form is `tests/test_workspace.py`,
            # which opens a directory of documents, resolves a
            # reference, and watches a moved pin refuse. What the row's
            # sentence still cannot say is the half below: two
            # documents a workspace accepts side by side, and no way to
            # assemble them.
            # G1's residue: no Expr door, so a profile step's argument
            # cannot be a named parameter. It is ALSO a naming
            # decision — the expression layer's genuine
            # dimension-mismatch arms already reach Python through
            # `load` (as `PersistError`/`parse`, issue #694), and
            # binding the operator builders would give them a second
            # route with `LiteralError` the nearest class while
            # `DimensionError` means the quantity boundary. Whoever
            # binds it decides which class those arms raise.
            "Expr",
            # G18 LEFT this list at LIB-G18b, the series' second half:
            # `Alignment`, `MateFrame`, `MatePrimitive`, `AxisSense`,
            # `assemble`, `Assembly`, `AssemblyError`,
            # `solve_document`, `update_references`, `mixed_pins` and
            # `Workspace.update_to_store` are all bound, and the
            # positive form is `tests/test_assembly_author.py`, which
            # authors the tour's two bench documents from nothing —
            # two part documents into a store, instances of them, the
            # mates that seat one on the other, the solve, the gather
            # and the A5 gate. `update_to_store` is a Workspace METHOD
            # rather than a module door, which is why it is not tested
            # for here.
            # G17: the shipped kernel verb with no node. Absent as a
            # MODULE door too — the tour reaches it as
            # `pncad::topo::shell`, and it does not cross.
            #
            # `chamfer_edges` stays here for a DIFFERENT reason now
            # (LIB-G16): the recipe door crossed as `Node.chamfer`, the
            # plain-body kernel verb did not, exactly as `fillet_edges`
            # has never crossed beside `Node.fillet`. That is the
            # binding shape, not a gap.
            "chamfer_edges", "shell", "shell_open",
        ]:
            with self.subTest(door=door):
                self.assertFalse(hasattr(pncad, door), f"{door} is now bound")

        # G18's structural door was a SIGNATURE and not a name, and
        # it is the half that CLOSED: `evaluate` takes the seam and
        # the memo, both keyword-only, so an `InstantiatePart` node
        # has a store to resolve its reference against. Pinned here as
        # the shape — `tests/test_assembly_eval.py` is what evaluates
        # the tour's own assembly documents through it — and by the
        # arity that stays refused, since a door named at the call
        # cannot be passed by position.
        self.assertEqual(
            list(inspect.signature(evaluate).parameters), ["doc", "resolver", "prior"]
        )
        with self.assertRaises(TypeError):
            evaluate(Doc(), None)

        # `circle` left this list when G1 closed (LIB-PYG1): it is a
        # profile PRIMITIVE, `pncad.circle`, not a node kind, and the
        # positive form is `TestBossplate` plus `tests/test_paths.py`.
        # `loft` left it when LIB-PYG23A closed G2's loft half; the
        # positive form is `TestLoftPrism`/`TestNonuniformLoft`.
        # `fillet`, `split` and `transform` left it when LIB-PYBUNDLE
        # closed G4/G6/G7 — the positive forms are `TestDiefillet`,
        # `TestTiltedcut` and `TestCrosslapExploded`/`TestDiepips`.
        # `declare` left it when LIB-PYG5 closed G5 — the positive
        # forms are `TestTable` and `TestCrosslapGlued`.
        # `sweep` and `tube` STAY: `wire_sweep` refuses unconditionally
        # (SWEEP_FRONTIER, the path-composition lane banked past M6),
        # and no `Node::Tube` exists at all. `pattern` stays for the
        # measured reason below — and note what is NOT in this list:
        # `placed_union`/`placed_union_at` left it when LIB-PYPU bound
        # the group boolean, whose value is an ordinary body.
        #
        # `chamfer` LEFT this list at LIB-G16: `Node::Chamfer` is a
        # recipe node now (schema v16), so `Node.chamfer` binds it —
        # `Node.fillet`'s twin, same frozen text selection. `shell`
        # and `shell_open` (G17) stay, and stay for the reason G16 no
        # longer has: the kernel verb ships (#1048) and `Node` has no
        # variant for it, so the scene that uses it has no document.
        #
        # `instantiate_part` and `mate` LEFT this list at LIB-G18b,
        # and `set_placement` with them — it was never a `Node` at
        # all, it is `DocEdit.set_placement`, which is where the A11
        # rule that placement is the CLUSTER's puts it.
        for node_kind in [
            "sweep", "tube", "pattern",
            "shell", "shell_open",
        ]:
            with self.subTest(node=node_kind):
                self.assertFalse(hasattr(Node, node_kind), f"Node.{node_kind} exists")

    def test_the_selector_surface_narrows_without_reading_names(self):
        """G13, CLOSED (LIB-PYSEL) — the flip of the absence this test
        used to pin.

        A selection could always be TAKEN (the four whole-kind
        materializers) and carried; what could not cross was anything
        that NARROWS one. Now `Evaluation.select` (role-path shape)
        and `Evaluation.select_where` (geometry: carrier kind,
        adjacency, datum distance) narrow — and the name text STAYS
        opaque by contract (the ordinal-28 ruling): both doors answer
        in the same alphabet the materializers speak, so nothing here
        reads inside a string. `TestDiecomposed` is the scene-scale
        positive form; this is the pocket-cube miniature the absence
        form used."""
        doc = Doc()
        cube = slab(doc, (0, 1), (0, 1), (0, 1))
        ev = evaluate(doc)
        self.assertEqual(len(ev.all_edges(cube)), 12)
        self.assertEqual(len(ev.all_faces(cube)), 6)
        self.assertEqual(len(ev.all_vertices(cube)), 8)
        self.assertEqual(len(ev.all_bodies(cube)), 1)

        pip = slab(doc, (0.4, 0.6), (0.4, 0.6), (0.9, 1.2))
        pipped = doc.insert(Node.boolean(BooleanOp.Subtract, cube, pip))
        ev = evaluate(doc)
        every_edge = ev.all_edges(pipped)
        self.assertEqual(len(every_edge), 24)

        edges = Selector.of(NamePat.of_kind(EntityKind.Edge))
        # An empty conjunction is exactly `select`, which on the
        # whole-kind pattern is exactly the materializer: three doors,
        # one answer, name for name.
        self.assertEqual(ev.select(pipped, edges), every_edge)
        self.assertEqual(ev.select_where(pipped, edges, []), every_edge)

        # Structural narrowing: the pocket's own edges came from
        # operand B of the subtraction — its 4 walls and 4 floor
        # edges. The 4 edges of the OPENING are `Seam` (minted where
        # the cap crosses a pocket wall, belonging to neither operand
        # alone), and the cube kept its 12: 8 + 4 + 12 = 24.
        from_b = Selector.of(
            NamePat.of_kind(EntityKind.Edge).seg(SegPat.tag(SegTag.FromB))
        )
        pocket = ev.select(pipped, from_b)
        self.assertEqual(len(pocket), 8)
        seam = Selector.of(
            NamePat.of_kind(EntityKind.Edge).seg(SegPat.tag(SegTag.Seam))
        )
        self.assertEqual(len(ev.select(pipped, seam)), 4)
        # ...and `matches` classifies the SAME materialized texts the
        # binding answered with, so the narrowing is checkable without
        # a second evaluation — or a single string read.
        self.assertEqual([n for n in every_edge if from_b.matches(n)], pocket)

        # Geometric narrowing: every edge of a box-minus-box is a
        # line, so the exact atom keeps all 24 — total, no refusal —
        # while an atom no edge satisfies keeps none.
        self.assertEqual(
            ev.select_where(
                pipped, edges, [GeomPred.curve_kind(CurveKind.Line)]
            ),
            every_edge,
        )
        self.assertEqual(
            ev.select_where(
                pipped,
                edges,
                [GeomPred.adjacent_kinds(SurfaceKind.Plane, SurfaceKind.Sphere)],
            ),
            [],
        )

    def test_a_split_whose_section_re_enters_one_face_now_names(self):
        """G14, CLOSED (LIB-G14) — and the diagnosis this test used to
        carry was WRONG, which is the more useful half of the finding.

        The name said "through boolean-minted faces"; measurement
        (cad-work/g14-survey.md) found TWO disjoint M4-era walls, and
        the one this scene hits has nothing to do with booleans. A
        split ACROSS boolean-minted faces named fine all along. What
        refused was a section line that re-enters ONE operand face —
        an inner loop, or any non-convex face — because
        `RoleSeg::SectionEdge{side, face}` names a chord only by the
        face it crosses, so a face crossed twice would mint one name
        twice. The walls read as one refusal because `NamingError` had
        no `Display` (#380).

        Both are gone: the chords become an N2 TIE (A2, ratified on
        #512), and a tied upstream entry PROPAGATES instead of
        refusing the whole op (B1)."""
        doc = Doc()
        box = slab(doc, (0, 3), (0, 2), (0, 1.5))
        cavity = slab(doc, (0.25, 0.75), (0.25, 0.75), (0.25, 2.0))
        hollow = doc.insert(Node.boolean(BooleanOp.Subtract, box, cavity))

        clear = doc.insert(Node.datum_plane((2.5 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0)))
        through = doc.insert(Node.datum_plane((0.5 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0)))
        ok = doc.insert(Node.split(hollow, clear))
        was_refused = doc.insert(Node.split(hollow, through))

        ev = evaluate(doc)
        for node in (ok, was_refused):
            above, below = ev.value(node).split()
            self.assertIsNotNone(above)
            self.assertIsNotNone(below)

    def test_the_cutaway_scene_has_a_document_spelling(self):
        """Tour scene `cutaway` (demos/tour/src/cutaway.rs), audit row
        31 — the row LIB-G14 flips.

        The scene runs `topo::split` KERNEL-level on the 15-op boolean
        project box with a tilted plane (normal (0.75, 0.1875, 1) — no
        axis alignment, crossing cavity floor, bosses and vents), then
        moves the halves apart. The geometry always worked; what did
        not exist was the DOCUMENT spelling, because `Node.split` on
        that boolean refused at name emission. Here is that spelling,
        with the scene's exact plane, and the names it now mints."""
        doc = Doc()
        box = projectbox(doc)
        tool = doc.insert(
            Node.datum_plane((1.5 * m, 1.0 * m, 0.75 * m), (0.75, 0.1875, 1.0))
        )
        cut = doc.insert(Node.split(box, tool))

        ev = evaluate(doc)
        above, below = ev.value(cut).split()
        self.assertIsNotNone(above)
        self.assertIsNotNone(below)

        # The split vocabulary is reachable from Python over the cut,
        # which is what "names end to end" means here: the emitter is
        # TOTAL, not merely non-refusing.
        def count(kind, tag):
            return len(
                ev.select(cut, Selector.of(NamePat.of_kind(kind).seg(SegPat.tag(tag))))
            )

        self.assertEqual(count(EntityKind.Body, SegTag.SplitBody), 2)
        self.assertEqual(count(EntityKind.Face, SegTag.SectionFace), 8)
        self.assertEqual(count(EntityKind.Edge, SegTag.SectionEdge), 32)
        self.assertEqual(count(EntityKind.Face, SegTag.SplitFragment), 32)
        self.assertEqual(count(EntityKind.Edge, SegTag.SplitFragment), 48)

    def test_the_rocker_outline_is_authorable(self):
        """G12, CLOSED — the flip of the absence this test used to pin.

        The wall was PATHS-DESIGN §2b's third: a STRAIGHT arrival off
        an ARC departure was refused, so the rocker's arc-to-line
        corners could not migrate to the lattice in Rust either.
        `TestRocker` is the scene-scale positive form. What remains
        here is the smallest statement of the §2c axiom that replaced
        the wall: a fillet knows only the tangent ray its directed
        point defines, so there is no carrier for its arrival to be
        keyed on and NO spelling refusal — the same `.at().toward()`
        pair completes the arrival whatever the departure rode."""
        loop = (
            Open.at((0 * m, 0 * m))
            .toward(1.0, 0.0)
            .fillet(0.5 * m)
            .at((3 * m, 3 * m))
            .toward(0.0, 1.0)
            .line(1 * m)
            .line_to(Start)
        )
        self.assertEqual(loop.vertex_count, 4)

    def test_a_plural_payload_cannot_feed_a_boolean(self):
        """Why `Node.pattern` stays unbound, measured rather than
        assumed — and what was built INSTEAD.

        A boolean's operand door refuses a plural payload: a split's
        two halves refuse below, and a `Pattern` node's `Instances`
        would refuse for the same reason. Binding the pattern node
        therefore still flips no row, so `Node.pattern` stays absent.

        What closes the replication half of G8 is a node whose value
        is SINGULAR: `PlacedUnion` fuses its placements and answers an
        ordinary `body`, which every downstream door consumes with no
        new arms. The contrast is the assertion below — same document,
        one payload a boolean cannot take and one it can."""
        self.assertFalse(hasattr(Node, "pattern"))

        doc = Doc()
        grouped = doc.insert(
            Node.placed_union_at(
                slab(doc, (0, 1), (0, 1), (0, 1)),
                [
                    Frame.translation((0 * m, 0 * m, 0 * m)),
                    Frame.translation((4 * m, 0 * m, 0 * m)),
                ],
            )
        )
        self.assertEqual(evaluate(doc).value(grouped).kind, "body")

        box = slab(doc, (0, 1), (0, 1), (0, 1))
        other = slab(doc, (2, 3), (0, 1), (0, 1))
        plane = doc.insert(Node.datum_plane((0 * m, 0 * m, 0.5 * m), (0.0, 0.0, 1.0)))
        halves = doc.insert(Node.split(box, plane))
        fused = doc.insert(Node.boolean(BooleanOp.Union, halves, other))
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(fused)
        self.assertEqual(caught.exception.kind, "wrong_operand")

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
