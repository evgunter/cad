"""Authoring a measurement, and reading the web back (LIB-B-MEASURES).

The census family B-MEASURES chartered the AUTHORING half of
ERROR-DESIGN E3/E10: `MeasureExpr`'s constructors, `MeasurePrimitive`'s
four verbs and `AssertionDir` onto `Node.measure` / `Node.assertion`,
with `MeasureNodeFault` as the refusal a caller dispatches on and
`MeasureUnavailableAt` as the one the fourth verb adds. The READING
half — `Value.measure` answering a `Measurement`, `Value.assertion` a
`Verdict` — already shipped, and it is what every row here reads the
authored web back through.

EVERY NUMBER IS AN ORACLE, RE-DERIVED FROM THE AUTHORING, never a
constant transcribed from `crates/editor-core/tests/m10_2_measure.rs`.
Two cylinders authored at x = ±0.30 make their walls' distance 0.60
because that is where the centres were written; two slabs authored 2 m
apart make their facing gap +2; a prism's opposed caps make the angle
between them a straight one; a coaxial bore and pin make the signed gap
exactly `r_bore - r_pin`. A row that read a previous run of the code
under test could not have gone red for any bug in the closed form.

THE LANE, MEASURED, because it decides which of the fourth verb's two
refusals a Python caller can ever see. `min_clearance` is answered by
an engine whose value is an ENCLOSURE, and the binding evaluates at
`f64` alone — a point scalar, with nowhere to put one. So
`MeasureUnavailableAt` is reachable here and is exercised below, while
the engine's own `MinClearanceRefusal` is not reachable at any feature
set: its one producer is the interval lane's `MinClearanceLane` impl,
and turning on `pncad-py`'s `interval` feature forwards a scalar the
binding still does not evaluate at. That refusal's tag and prose are
pinned in Rust instead (`src/tests.rs`).

NOTHING HERE READS INSIDE A NAME. Every reference is a `(node, name)`
pair whose name is a materializer's opaque text handed straight back,
which is the whole point of the alphabet: selecting a face and
measuring it are the same vocabulary.
"""

import math
import unittest

import pncad
from pncad import (
    AssertionDir,
    Doc,
    DocEdit,
    DocParam,
    DocParamValue,
    EditError,
    EntityKind,
    EvaluationError,
    Expr,
    GeomPred,
    MeasureExpr,
    MeasurePrimitive,
    NamePat,
    Node,
    ParamName,
    Selector,
    SurfaceKind,
    WrittenAngle,
    WrittenLength,
    circle,
    evaluate,
    load,
    m,
    mm,
    rad,
)

_0M = Expr.written_length(WrittenLength.in_unit(0, m))
_1M = Expr.written_length(WrittenLength.in_unit(1, m))
SQUARE = [(_0M, _0M), (_1M, _0M), (_1M, _1M), (_0M, _1M)]


def slab(doc, elevation, height=1.0):
    """A 1 m x 1 m x `height` prism whose base sits at `elevation`."""
    plane = doc.sketch_frame(elevation=Expr.written_length(WrittenLength.in_unit(elevation, m)))
    outline = doc.insert(Node.polygon(SQUARE, plane=plane))
    return doc.insert(Node.extrude(outline, Expr.written_length(WrittenLength.in_unit(height, m))))


def cylinder(doc, centre_x, radius, height=0.5):
    """A circular prism on the world xy plane, centred at `centre_x`."""
    outline = doc.insert(
        Node.profile(circle((centre_x * m, 0 * m), radius * m), doc.sketch_frame())
    )
    return doc.insert(Node.extrude(outline, Expr.written_length(WrittenLength.in_unit(height, m))))


def wall(ev, node):
    """One cylindrical wall of a circular prism, found the way a user
    finds it: ask the selection door for that node's cylinder faces.

    A circular extrude's wall is two faces sharing one cylinder
    carrier, and the closed forms read the CARRIER — so either answers
    for the prism, and the first in canonical order is taken.
    """
    found = ev.select_where(
        node,
        Selector.of(NamePat.of_kind(EntityKind.Face)),
        [GeomPred.surface_kind(SurfaceKind.Cylinder)],
    )
    assert found, "a circular extrude has a cylindrical wall"
    return sorted(found)[0]


def face_at_height(ev, node, z):
    """The one face of `node` whose carrier frame sits at height `z` —
    read off the evaluation rather than transcribed as a role name."""
    found = [
        f
        for f in ev.all_faces(node)
        if abs(ev.face_frame(node, f).origin[2].meters - z) < 1e-12
    ]
    assert len(found) == 1, f"one face at z = {z}, found {len(found)}"
    return found[0]


def measured(doc, node):
    """Evaluate and read the measurement `node` answered."""
    return evaluate(doc).value(node).measure()


def verdict(doc, node):
    """Evaluate and read the verdict `node` answered."""
    return evaluate(doc).value(node).assertion()


class TestTheVerbVocabulary(unittest.TestCase):
    """The four primitives as VALUES: what each one says about itself
    before any document exists."""

    def test_each_verb_names_itself_and_its_dimension(self):
        rows = [
            (MeasurePrimitive.distance(0, 1), "distance", "length"),
            (MeasurePrimitive.angle(0, 1), "angle", "angle"),
            (MeasurePrimitive.min_clearance(0, 1), "min_clearance", "length"),
            (MeasurePrimitive.gap(0, 1), "gap", "length"),
        ]
        for prim, verb, dimension in rows:
            with self.subTest(verb=verb):
                self.assertEqual(prim.verb, verb)
                self.assertEqual(prim.dimension, dimension)

    def test_a_gaps_pair_keeps_its_roles_and_is_not_re_sorted(self):
        """C5's formulas are asymmetric in the mating roles, so the
        order is authored data rather than a set."""
        self.assertEqual(MeasurePrimitive.gap(6, 7).refs, (6, 7))
        self.assertEqual(MeasurePrimitive.gap(7, 6).refs, (7, 6))
        self.assertNotEqual(MeasurePrimitive.gap(6, 7), MeasurePrimitive.gap(7, 6))

    def test_primitives_are_values_comparable_and_hashable(self):
        one = MeasurePrimitive.distance(0, 1)
        same = MeasurePrimitive.distance(0, 1)
        self.assertEqual(one, same)
        self.assertEqual(hash(one), hash(same))
        # A verb is part of the value: two primitives over the same
        # references measuring different things are different values.
        self.assertNotEqual(one, MeasurePrimitive.angle(0, 1))
        self.assertEqual(len({one, same, MeasurePrimitive.angle(0, 1)}), 2)
        self.assertEqual(repr(one), "MeasurePrimitive.distance(0, 1)")

    def test_a_negative_index_is_not_a_position(self):
        """An index is a POSITION in the reference list, so a negative
        one is not a value the argument can hold — refused at the call,
        never wrapped round to the end."""
        with self.assertRaises(OverflowError):
            MeasurePrimitive.distance(-1, 0)

    def test_the_two_directions_keep_the_kernels_symbols(self):
        self.assertEqual(AssertionDir.AtLeast.symbol, ">=")
        self.assertEqual(AssertionDir.AtMost.symbol, "<=")
        self.assertEqual(AssertionDir.AtLeast, AssertionDir.AtLeast)
        self.assertNotEqual(AssertionDir.AtLeast, AssertionDir.AtMost)


class TestTheExpressionLanguage(unittest.TestCase):
    """`MeasureExpr` before any node: the arithmetic, its dimension and
    the refusal a mis-dimensioned tree earns AT CONSTRUCTION."""

    def test_the_dimension_rides_the_expression(self):
        length = MeasureExpr.primitive(MeasurePrimitive.distance(0, 1))
        angle = MeasureExpr.primitive(MeasurePrimitive.angle(0, 1))
        self.assertEqual(length.dimension, "length")
        self.assertEqual(angle.dimension, "angle")
        self.assertEqual(MeasureExpr.neg(angle).dimension, "angle")
        self.assertEqual(MeasureExpr.add(length, length).dimension, "length")

    def test_a_document_expression_enters_as_a_leaf(self):
        """The only door inward is the TEXT one: `Doc.parse_expr` runs
        the checking parser, and a `MeasureExpr` leaf carries what it
        answered."""
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("pad"), DocParam.length(1 * mm)))
        leaf = MeasureExpr.value(doc.parse_expr("pad"))
        self.assertEqual(leaf.dimension, "length")
        # A value leaf holds no primitive: it reaches out of the
        # document's arithmetic, never into geometry.
        self.assertEqual(leaf.primitives, [])

    def test_the_f1_lattice_refuses_at_the_constructor(self):
        """A mis-dimensioned tree does not exist to be handed around.

        The refusal is `LiteralError` carrying the kernel's own
        mismatch tag — the same refusal a document expression earns,
        because this language asks `Expr`'s constructors rather than
        restating the table. `value` is `None`: the door refuses over
        two operands' DIMENSIONS and has no single number to name.
        """
        length = MeasureExpr.primitive(MeasurePrimitive.distance(0, 1))
        angle = MeasureExpr.primitive(MeasurePrimitive.angle(0, 1))
        with self.assertRaises(pncad.LiteralError) as caught:
            MeasureExpr.add(length, angle)
        self.assertEqual(caught.exception.kind, "mismatch")
        self.assertIsNone(caught.exception.value)

    def test_the_product_and_quotient_rules_are_the_kernels(self):
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("half"), DocParam.scalar(0.5)))
        length = MeasureExpr.primitive(MeasurePrimitive.distance(0, 1))
        scalar = MeasureExpr.value(doc.parse_expr("half"))
        self.assertEqual(MeasureExpr.mul(length, scalar).dimension, "length")
        self.assertEqual(MeasureExpr.div(length, scalar).dimension, "length")
        # Two lengths multiplied is not a length, and this language has
        # no area: the F1 rule refuses rather than inventing one.
        with self.assertRaises(pncad.LiteralError) as caught:
            MeasureExpr.mul(length, length)
        self.assertEqual(caught.exception.kind, "mul_needs_scalar")
        with self.assertRaises(pncad.LiteralError) as caught:
            MeasureExpr.div(scalar, length)
        self.assertEqual(caught.exception.kind, "div_needs_scalar_divisor")

    def test_min_and_max_take_the_same_dimension(self):
        a = MeasureExpr.primitive(MeasurePrimitive.distance(0, 1))
        b = MeasureExpr.primitive(MeasurePrimitive.gap(0, 1))
        self.assertEqual(MeasureExpr.min(a, b).dimension, "length")
        self.assertEqual(MeasureExpr.max(a, b).dimension, "length")
        with self.assertRaises(pncad.LiteralError):
            MeasureExpr.min(a, MeasureExpr.primitive(MeasurePrimitive.angle(0, 1)))

    def test_the_primitives_come_back_in_pre_order(self):
        """One order, two consumers: the construction door's bounds
        check runs over it and the evaluation reads the leaves back in
        it, so a caller inspecting a tree sees the same order both."""
        first = MeasurePrimitive.distance(0, 1)
        second = MeasurePrimitive.gap(2, 3)
        tree = MeasureExpr.sub(
            MeasureExpr.primitive(first), MeasureExpr.primitive(second)
        )
        self.assertEqual(tree.primitives, [first, second])
        self.assertEqual(
            MeasureExpr.sub(
                MeasureExpr.primitive(second), MeasureExpr.primitive(first)
            ).primitives,
            [second, first],
        )


class TestTheClosedForms(unittest.TestCase):
    """The four verbs against oracles the authoring fixes."""

    def test_the_distance_between_two_walls_is_the_axis_separation(self):
        doc = Doc()
        offset = 0.30
        left = cylinder(doc, -offset, 0.2)
        right = cylinder(doc, offset, 0.2)
        ev = evaluate(doc)
        node = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [(left, wall(ev, left)), (right, wall(ev, right))],
            )
        )
        answer = measured(doc, node)
        self.assertEqual(answer.dimension, "Length")
        # The centres were written at x = ±offset; nothing else decides
        # this number.
        self.assertAlmostEqual(answer.value, 2 * offset, places=12)
        # A Length measure answers through the typed quantity too.
        self.assertAlmostEqual(answer.length.meters, 2 * offset, places=12)

    def test_the_angle_between_opposed_caps_is_a_straight_angle(self):
        """An extruded prism's caps are parallel with OPPOSED chart
        normals, so the angle between their carriers is pi. The oracle
        is the authoring, not the codomain: `0 <= a <= pi` would be
        satisfied by every possible answer."""
        doc = Doc()
        prism = slab(doc, 0.0, height=0.4)
        ev = evaluate(doc)
        node = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.angle(0, 1)),
                [
                    (prism, face_at_height(ev, prism, 0.0)),
                    (prism, face_at_height(ev, prism, 0.4)),
                ],
            )
        )
        answer = measured(doc, node)
        self.assertEqual(answer.dimension, "Angle")
        self.assertAlmostEqual(answer.value, math.pi, places=12)
        # An angle is not a length, so the typed-quantity column is
        # empty rather than holding radians relabelled as metres.
        self.assertIsNone(answer.length)

    def test_a_plane_gap_over_disjoint_slabs_is_positive_both_ways(self):
        """A gap is a MATERIAL separation and its sign says so: 2 m of
        air between two slabs reads +2 whichever way round the roles
        go, never a negative "interference"."""
        doc = Doc()
        air = 2.0
        lower = slab(doc, 0.0)
        upper = slab(doc, 1.0 + air)
        ev = evaluate(doc)
        facing = [
            (lower, face_at_height(ev, lower, 1.0)),
            (upper, face_at_height(ev, upper, 1.0 + air)),
        ]
        for outer, inner in (facing, list(reversed(facing))):
            with self.subTest(outer=outer[0]):
                node = doc.insert(
                    Node.measure(
                        MeasureExpr.primitive(MeasurePrimitive.gap(0, 1)),
                        [outer, inner],
                    )
                )
                self.assertAlmostEqual(measured(doc, node).value, air, places=12)

    def test_the_gap_sign_convention_walks_all_three_regimes(self):
        """A bore and a pin on ONE axis, so the axis offset is zero and
        the signed gap is exactly `r_bore - r_pin` — clearance, contact
        and interference from one document shape."""
        for bore_r, pin_r in ((0.51, 0.50), (0.50, 0.50), (0.49, 0.50)):
            with self.subTest(bore=bore_r, pin=pin_r):
                doc = Doc()
                bore = cylinder(doc, 0.0, bore_r)
                pin = cylinder(doc, 0.0, pin_r)
                ev = evaluate(doc)
                node = doc.insert(
                    Node.measure(
                        MeasureExpr.primitive(MeasurePrimitive.gap(0, 1)),
                        [(bore, wall(ev, bore)), (pin, wall(ev, pin))],
                    )
                )
                answer = measured(doc, node)
                self.assertEqual(answer.dimension, "Length")
                self.assertAlmostEqual(answer.value, bore_r - pin_r, places=12)

    def test_the_authors_own_arithmetic_spells_the_web(self):
        """The worked example's shape: the web between two holes is
        `distance(wall, wall) - 2 * r`, spelled as the author's
        arithmetic rather than hidden inside a primitive."""
        doc = Doc()
        offset, radius = 0.30, 0.2
        doc.apply(
            DocEdit.set_doc_param(ParamName("hole_r"), DocParam.length(radius * m))
        )
        left = cylinder(doc, -offset, radius)
        right = cylinder(doc, offset, radius)
        ev = evaluate(doc)
        r = MeasureExpr.value(doc.parse_expr("hole_r"))
        web = MeasureExpr.sub(
            MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
            MeasureExpr.add(r, r),
        )
        node = doc.insert(
            Node.measure(web, [(left, wall(ev, left)), (right, wall(ev, right))])
        )
        # 0.60 between the axes, less both radii.
        self.assertAlmostEqual(
            measured(doc, node).value, 2 * offset - 2 * radius, places=12
        )

    def test_a_measure_reads_the_placed_carrier(self):
        """The reference names the node its carrier is READ AT, and
        that is what makes a measure report moved geometry.

        ONE vertex name, measured against ITSELF at two reading sites:
        a rigid transform is identity-preserving, so it mints no name
        and the only thing that differs between the two references is
        which node they resolve at. The distance between them is
        exactly the translation, and reading both at the minting node
        would be zero.
        """
        doc = Doc()
        travel = 100.0
        prism = slab(doc, 0.0)
        moved = doc.insert(
            Node.transform(prism, (
                Expr.written_length(WrittenLength.in_unit(travel, m)),
                Expr.written_length(WrittenLength.in_unit(0, m)),
                Expr.written_length(WrittenLength.in_unit(0, m)),
            ), (
                Expr.literal(0.0),
                Expr.literal(0.0),
                Expr.literal(1.0),
            ), Expr.written_angle(WrittenAngle.in_unit(0, rad)))
        )
        ev = evaluate(doc)
        corner = sorted(ev.all_vertices(moved))[0]
        self.assertIn(corner, ev.all_vertices(prism), "a transform mints no name")
        placed = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [(prism, corner), (moved, corner)],
            )
        )
        self.assertAlmostEqual(measured(doc, placed).value, travel, places=9)
        # The same name read at ONE site measures against itself: zero,
        # which is what makes the number above the translation and not
        # an artefact of which vertex was picked.
        authored = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [(prism, corner), (prism, corner)],
            )
        )
        self.assertAlmostEqual(measured(doc, authored).value, 0.0, places=12)


class TestTheFourthVerb(unittest.TestCase):
    """`min_clearance`, and the typed absence a point scalar answers
    it with."""

    def clearance_document(self):
        doc = Doc()
        left = cylinder(doc, -0.30, 0.2)
        right = cylinder(doc, 0.30, 0.2)
        ev = evaluate(doc)
        node = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.min_clearance(0, 1)),
                [(left, wall(ev, left)), (right, wall(ev, right))],
            )
        )
        return doc, node

    def test_the_measure_node_evaluates_and_has_no_value(self):
        """The absence is a VALUE and not a failure — the node built,
        and said what it could not say."""
        doc, node = self.clearance_document()
        self.assertEqual(doc.node_kind(node), "measure")
        value = evaluate(doc).value(node)
        self.assertEqual(value.kind, "measure")
        with self.assertRaises(pncad.MeasureUnavailableAt) as caught:
            value.measure()
        refusal = caught.exception
        self.assertEqual(refusal.variant, "needs_enclosure")
        self.assertEqual(refusal.verb, "min_clearance")
        self.assertEqual(refusal.scalar, "f64")
        # The recourse is IN the refusal: it names the door that can
        # answer rather than handing back a worse number.
        self.assertEqual(refusal.door, "clearance::min_separation")
        self.assertIn(refusal.door, str(refusal))

    def test_it_is_not_the_analysis_lanes_refusal_of_the_same_name(self):
        """Two classes one word apart, and a caller catching one must
        not catch the other: `MeasureUnavailable` is the analysis lane
        refusing to price a mass over a band."""
        self.assertFalse(
            issubclass(pncad.MeasureUnavailableAt, pncad.MeasureUnavailable)
        )
        self.assertFalse(
            issubclass(pncad.MeasureUnavailable, pncad.MeasureUnavailableAt)
        )
        for cls in (pncad.MeasureUnavailableAt, pncad.MeasureUnavailable):
            self.assertTrue(issubclass(cls, pncad.PncadError))

    def test_an_assertion_over_it_reports_the_third_state(self):
        """E10's third state used for what it is for: the requirement
        is recorded, the run cannot answer it, and neither half is
        hidden. NOT a poisoning — the measure did not fail."""
        doc, node = self.clearance_document()
        assertion = doc.insert(
            Node.assertion(node, AssertionDir.AtLeast, doc.parse_expr("1 mm"))
        )
        answer = verdict(doc, assertion)
        self.assertEqual(answer.status, "Unevaluated")
        self.assertIsNone(answer.holds)
        self.assertIsNone(answer.measured)
        self.assertIsNone(answer.bound)
        self.assertIn("min_clearance", answer.reason)
        self.assertIn("clearance::min_separation", answer.reason)

    def test_it_refuses_typed_on_an_edge_reference(self):
        """A reference's entity kind is the selection's face scope, and
        an edge names no faces at all."""
        doc = Doc()
        left = slab(doc, 0.0)
        right = slab(doc, 4.0)
        ev = evaluate(doc)
        node = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.min_clearance(0, 1)),
                [(left, ev.all_edges(left)[0]), (right, ev.all_edges(right)[0])],
            )
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(node)
        self.assertEqual(caught.exception.kind, "measure_selection_kind")


class TestTheAssertion(unittest.TestCase):
    """A recorded requirement, its three states, and the parameter edit
    that moves it."""

    def web_document(self, bound_mm):
        """Two cylinders 0.6 m apart, a distance measure over their
        walls, and an assertion whose bound is a PARAMETER."""
        doc = Doc()
        doc.apply(
            DocEdit.set_doc_param(ParamName("bound"), DocParam.length(bound_mm * mm))
        )
        left = cylinder(doc, -0.30, 0.2)
        right = cylinder(doc, 0.30, 0.2)
        ev = evaluate(doc)
        measure = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [(left, wall(ev, left)), (right, wall(ev, right))],
            )
        )
        return doc, measure

    def test_one_parameter_edit_flips_the_verdict(self):
        """The bound is an `Expr`, so it reaches a document parameter —
        which is what makes a recorded requirement re-decidable without
        re-authoring the node."""
        doc, measure = self.web_document(500.0)
        assertion = doc.insert(
            Node.assertion(measure, AssertionDir.AtLeast, doc.parse_expr("bound"))
        )
        self.assertEqual(doc.node_kind(assertion), "assertion")

        holds = verdict(doc, assertion)
        self.assertEqual(holds.status, "Holds")
        self.assertTrue(holds.holds)
        # Both numbers on a decided verdict: 0.6 m of separation
        # against a 0.5 m bound.
        self.assertAlmostEqual(holds.measured, 0.6, places=12)
        self.assertAlmostEqual(holds.bound, 0.5, places=12)

        doc.apply(
            DocEdit.set_doc_param_value(
                ParamName("bound"), DocParamValue.length(700 * mm)
            )
        )
        violated = verdict(doc, assertion)
        self.assertEqual(violated.status, "Violated")
        self.assertFalse(violated.holds)
        self.assertAlmostEqual(violated.measured, 0.6, places=12)
        self.assertAlmostEqual(violated.bound, 0.7, places=12)
        self.assertLess(violated.measured, violated.bound)

    def test_the_other_direction_gates_the_other_way(self):
        doc, measure = self.web_document(700.0)
        assertion = doc.insert(
            Node.assertion(measure, AssertionDir.AtMost, doc.parse_expr("bound"))
        )
        self.assertEqual(verdict(doc, assertion).status, "Holds")
        doc.apply(
            DocEdit.set_doc_param_value(
                ParamName("bound"), DocParamValue.length(500 * mm)
            )
        )
        self.assertEqual(verdict(doc, assertion).status, "Violated")

    def test_a_verdict_is_report_only(self):
        """A `Violated` assertion changes no downstream outcome: the
        same document with and without it saves the same recipe for
        every other node, and the assertion denotes no body."""
        doc, measure = self.web_document(700.0)
        without = doc.save()
        assertion = doc.insert(
            Node.assertion(measure, AssertionDir.AtLeast, doc.parse_expr("bound"))
        )
        self.assertEqual(verdict(doc, assertion).status, "Violated")
        # Every node that existed before the assertion still evaluates
        # to the value it did, and the measure is untouched.
        self.assertAlmostEqual(measured(doc, measure).value, 0.6, places=12)
        self.assertIn('"Measure"', without)
        self.assertNotIn('"Assertion"', without)
        self.assertIn('"Assertion"', doc.save())

    def test_an_assertion_over_a_failed_measure_is_poisoned(self):
        """The DAG edge is what composes: a verdict about a
        measurement that did not happen would be a verdict about
        nothing, so the assertion is poisoned rather than
        `Unevaluated`."""
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("s"), DocParam.scalar(0.0)))
        # `13 m / s` with `s` bound to zero — the DIVISION is the
        # measure language's, so each leaf evaluates fine and the
        # measure arithmetic is what goes non-finite.
        over_zero = MeasureExpr.div(
            MeasureExpr.value(doc.parse_expr("13 m")),
            MeasureExpr.value(doc.parse_expr("s")),
        )
        measure = doc.insert(Node.measure(over_zero, []))
        assertion = doc.insert(
            Node.assertion(measure, AssertionDir.AtLeast, doc.parse_expr("1 m"))
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(measure)
        self.assertEqual(caught.exception.kind, "measure_non_finite")
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(assertion)
        self.assertEqual(caught.exception.kind, "measure_non_finite")
        self.assertEqual(caught.exception.through, measure)


class TestTheRefusals(unittest.TestCase):
    """Every door that says no, reached from Python."""

    def one_face(self):
        doc = Doc()
        node = slab(doc, 0.0)
        return doc, node, face_at_height(evaluate(doc), node, 0.0)

    def test_an_index_past_the_end_refuses_at_the_constructor(self):
        """`MeasureNodeFault`'s one arm, raised where it is written —
        the timing the construction door buys over the `Doc.apply`
        after it."""
        _doc, node, face = self.one_face()
        with self.assertRaises(pncad.MeasureNodeFault) as caught:
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [(node, face)],
            )
        fault = caught.exception
        self.assertEqual(fault.variant, "ref_index_out_of_range")
        self.assertEqual(fault.verb, "distance")
        self.assertEqual(fault.index, 1)
        self.assertEqual(fault.refs, 1)
        # The verb is the ONE vocabulary: what the fault names is what
        # the primitive calls itself.
        self.assertEqual(fault.verb, MeasurePrimitive.distance(0, 1).verb)

    def test_a_reference_list_that_is_long_enough_is_accepted(self):
        """The mirror of the row above — an index the list DOES carry
        is not refused, so the check is about the bound and not about
        indices in general."""
        doc, node, face = self.one_face()
        top = face_at_height(evaluate(doc), node, 1.0)
        built = Node.measure(
            MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
            [(node, face), (node, top)],
        )
        self.assertEqual(doc.node_kind(doc.insert(built)), "measure")

    def test_an_unread_reference_is_carried_and_not_bounded_against(self):
        """A reference no primitive indexes is carried data. The bounds
        check runs over the indices the EXPRESSION reads, so a longer
        list is legal."""
        doc, node, face = self.one_face()
        top = face_at_height(evaluate(doc), node, 1.0)
        built = Node.measure(
            MeasureExpr.primitive(MeasurePrimitive.gap(1, 0)),
            [(node, face), (node, top), (node, face)],
        )
        self.assertEqual(doc.node_kind(doc.insert(built)), "measure")

    def test_a_name_that_is_not_a_name_refuses_at_the_boundary(self):
        _doc, node, _ = self.one_face()
        with self.assertRaises(ValueError):
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [(node, "the top face"), (node, "the other one")],
            )

    def test_an_assertion_must_reference_a_measure(self):
        doc, node, _ = self.one_face()
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.assertion(node, AssertionDir.AtLeast, doc.parse_expr("1 m")))
        self.assertEqual(caught.exception.variant, "assertion_target")

    def test_an_assertion_compares_like_with_like_or_not_at_all(self):
        """The bound's dimension is the MEASURE's, and the edit door is
        where that is checked because it is the door that holds the
        document."""
        doc = Doc()
        node = slab(doc, 0.0)
        ev = evaluate(doc)
        measure = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.angle(0, 1)),
                [
                    (node, face_at_height(ev, node, 0.0)),
                    (node, face_at_height(ev, node, 1.0)),
                ],
            )
        )
        with self.assertRaises(EditError) as caught:
            doc.insert(
                Node.assertion(measure, AssertionDir.AtMost, doc.parse_expr("1 m"))
            )
        self.assertEqual(caught.exception.variant, "assertion_dimension")
        # And the matching dimension is accepted, so the row above is
        # about the mismatch rather than about assertions on angles.
        doc.insert(Node.assertion(measure, AssertionDir.AtMost, doc.parse_expr("4 rad")))

    def test_deleting_a_referenced_node_is_refused(self):
        """A measure CONSUMES the values it names, so its references
        are recipe edges — the one place this node kind departs from
        the `Declare`/`Mate` name carve-out."""
        doc = Doc()
        node = slab(doc, 0.0)
        ev = evaluate(doc)
        doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.gap(0, 1)),
                [
                    (node, face_at_height(ev, node, 0.0)),
                    (node, face_at_height(ev, node, 1.0)),
                ],
            )
        )
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.delete_node(node))
        self.assertEqual(caught.exception.variant, "delete_would_dangle")

    def test_a_carrier_pair_with_no_closed_form_refuses_naming_the_pair(self):
        """A whole BODY has no carrier, and the refusal names the pair
        class rather than guessing an arm."""
        doc = Doc()
        left = slab(doc, 0.0)
        right = slab(doc, 4.0)
        ev = evaluate(doc)
        node = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [(left, ev.all_bodies(left)[0]), (right, ev.all_bodies(right)[0])],
            )
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(node).measure()
        self.assertEqual(caught.exception.kind, "measure_unsupported")

    def test_a_gap_between_non_parallel_planes_refuses(self):
        """C5's plane arm is about a SEPARATION along a shared normal,
        so a cap and a side wall have no gap to report — the closed
        form refuses rather than projecting one onto an axis nobody
        named."""
        doc = Doc()
        node = slab(doc, 0.0)
        ev = evaluate(doc)
        bottom = face_at_height(ev, node, 0.0)
        top = face_at_height(ev, node, 1.0)
        side = next(f for f in ev.all_faces(node) if f not in (bottom, top))
        measure = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.gap(0, 1)),
                [(node, bottom), (node, side)],
            )
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(measure).measure()
        self.assertEqual(caught.exception.kind, "measure_not_parallel")

    def test_a_reference_that_does_not_resolve_at_its_site_refuses(self):
        """N5's typed vocabulary, on the measurement channel: a
        well-formed name the reading site's own table does not carry
        names nothing THERE, and the measure says so rather than
        measuring what is left."""
        doc = Doc()
        here = slab(doc, 0.0)
        elsewhere = slab(doc, 4.0)
        ev = evaluate(doc)
        alien = face_at_height(ev, here, 0.0)
        measure = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [(elsewhere, alien), (elsewhere, face_at_height(ev, elsewhere, 4.0))],
            )
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(measure)
        self.assertEqual(caught.exception.kind, "measure_ref_resolve")

    def test_the_load_door_re_checks_the_reference_indices(self):
        """`Node.measure` cannot mint an out-of-range index, so the
        only way one reaches a document is a hand-edited file — and the
        load door runs the SAME check, refusing with the fault's own
        prose.

        MEASURED, and reported: it arrives as `PersistError` with
        `variant == "snapshot"`, so the fault is in the message and not
        in a branchable tag. The edit door's `measure_malformed` is
        unreachable from Python for the same reason the construction
        door exists.
        """
        doc = Doc()
        node = slab(doc, 0.0)
        ev = evaluate(doc)
        doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [
                    (node, face_at_height(ev, node, 0.0)),
                    (node, face_at_height(ev, node, 1.0)),
                ],
            )
        )
        tampered = doc.save().replace('"b": 1', '"b": 7')
        self.assertNotEqual(tampered, doc.save(), "the tamper found its slot")
        with self.assertRaises(pncad.PersistError) as caught:
            load(tampered)
        self.assertEqual(caught.exception.variant, "snapshot")
        self.assertIn("reads reference 7", str(caught.exception))

    def test_a_value_that_is_not_a_measure_says_so(self):
        doc = Doc()
        node = slab(doc, 0.0)
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(node).measure()
        # A door that refuses the CALL carries its word on `reason`;
        # `kind` is the tag of a node that FAILED, and this one did not.
        self.assertEqual(caught.exception.reason, "wrong_kind")
        self.assertIsNone(caught.exception.kind)


class TestTheDocumentCarriesIt(unittest.TestCase):
    """The measurement vocabulary on the wire, and the read door that
    answers for it with no evaluation in hand."""

    def authored(self):
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("bound"), DocParam.length(1 * mm)))
        left = cylinder(doc, -0.30, 0.2)
        right = cylinder(doc, 0.30, 0.2)
        ev = evaluate(doc)
        web = MeasureExpr.sub(
            MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
            MeasureExpr.value(doc.parse_expr("bound")),
        )
        measure = doc.insert(
            Node.measure(web, [(left, wall(ev, left)), (right, wall(ev, right))])
        )
        assertion = doc.insert(
            Node.assertion(measure, AssertionDir.AtLeast, doc.parse_expr("0.1 m"))
        )
        return doc, measure, assertion

    def test_a_document_carrying_a_measure_saves_and_loads(self):
        doc, measure, assertion = self.authored()
        before = measured(doc, measure).value
        back = load(doc.save()).doc
        self.assertEqual(back.node_kind(measure), "measure")
        self.assertEqual(back.node_kind(assertion), "assertion")
        self.assertAlmostEqual(measured(back, measure).value, before, places=12)
        self.assertEqual(verdict(back, assertion).status, "Holds")
        # A round trip is the identity on the recipe's bits.
        self.assertTrue(back.bit_eq(doc))

    def test_node_kind_answers_the_two_new_words(self):
        """`Doc.node_kind` has spoken these two words since it was
        written, for nodes no Python caller could author. They are
        answerable with no evaluation in hand at all."""
        doc, measure, assertion = self.authored()
        self.assertEqual(doc.node_kind(measure), "measure")
        self.assertEqual(doc.node_kind(assertion), "assertion")

    def test_a_measure_denotes_no_body_and_a_verdict_is_not_one_either(self):
        """Neither value is a body, which is what "report only" means
        at the value level: no operation in the vocabulary accepts
        either as an operand."""
        doc, measure, assertion = self.authored()
        ev = evaluate(doc)
        self.assertEqual(ev.value(measure).kind, "measure")
        self.assertEqual(ev.value(assertion).kind, "assertion")

    def test_a_measure_over_a_non_root_leaves_the_product_alone(self):
        """The ordinary case, and the one the worked example is in:
        the measure's references sit UNDER something else, so the
        measure is a fresh sink, the product root above them is
        untouched, and the gathered solid is the same one."""
        doc = Doc()
        left = cylinder(doc, -0.30, 0.2)
        right = cylinder(doc, 0.30, 0.2)
        union = doc.insert(Node.boolean(pncad.BooleanOp.Union, left, right))
        before = pncad.product(doc, evaluate(doc)).mass_properties().volume
        ev = evaluate(doc)
        doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [(left, wall(ev, left)), (right, wall(ev, right))],
            )
        )
        self.assertIn(union, doc.roots)
        self.assertAlmostEqual(
            pncad.product(doc, evaluate(doc)).mass_properties().volume,
            before,
            places=12,
        )

    def test_a_measure_over_the_product_roots_takes_them(self):
        """**Measured, and reported as a finding rather than asserted
        to be right.**

        A measure's references are recipe EDGES, so a measure over the
        nodes that are currently product roots is a sink consuming
        them: D-3's tip transfer moves the root onto the measure, and
        a measure denotes no body — so the document that had a solid
        product now has none. `DocEdit.set_roots` cannot restore one:
        listing the bodies alone leaves the measure uncovered, and
        listing the measure beside them is an ancestor pair.

        This row exists so the behaviour is visible and so a kernel
        change to it goes red HERE with the argument in hand, not so
        the behaviour is preserved.
        """
        doc = Doc()
        left = cylinder(doc, -0.30, 0.2)
        right = cylinder(doc, 0.30, 0.2)
        self.assertEqual(doc.roots, [left, right])
        self.assertGreater(
            pncad.product(doc, evaluate(doc)).mass_properties().volume, 0.0
        )
        ev = evaluate(doc)
        measure = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
                [(left, wall(ev, left)), (right, wall(ev, right))],
            )
        )
        self.assertEqual(doc.roots, [measure])
        with self.assertRaises(pncad.ProductError) as caught:
            pncad.product(doc, evaluate(doc))
        self.assertEqual(caught.exception.variant, "no_body_roots")
        # And neither root list D-2 would accept puts a body back.
        for roots, refusal in (
            ([left, right], "root_uncovered"),
            ([left, right, measure], "root_ancestor"),
        ):
            with self.subTest(roots=roots):
                fresh = load(doc.save()).doc
                with self.assertRaises(EditError) as caught:
                    fresh.apply(DocEdit.set_roots(roots))
                self.assertEqual(caught.exception.variant, refusal)


if __name__ == "__main__":
    unittest.main()
