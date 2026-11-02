"""Authoring a measurement, and reading the web back (LIB-B-MEASURES).

The census family B-MEASURES chartered the AUTHORING half of
ERROR-DESIGN E3/E10: `MeasurePrimitive`'s three verbs and `AssertionRelation`
onto `Node.measure` / `Doc.measure` / `Node.assertion`, and the clearance
requirement `Node.clearance`. Arithmetic
over measured values is an ordinary formula over the measures' outputs:
there is no second arithmetic language. The READING
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

THE LANE, MEASURED, because it decides which clearance verdicts a Python
caller can ever see. A clearance is decided by the certified sweep, which
reads enclosures, and the binding evaluates at `f64` alone — a point
scalar. So every clearance here answers the third state naming the
certifying lane (`needs_certifying_lane`); a decided verdict and its
pair are not reachable from Python.

NOTHING HERE READS INSIDE A NAME. Every reference is a `(node, name)`
pair whose name is a materializer's opaque text handed straight back,
which is the whole point of the alphabet: selecting a face and
measuring it are the same vocabulary.
"""

import math
import unittest

import pncad
from pncad import (
    AssertionRelation,
    Doc,
    DocEdit,
    FreeVar,
    FreeValue,
    EditError,
    Frame,
    EntityKind,
    EvaluationError,
    Formula,
    GeomPred,
    MeasurePrimitive,
    NamePat,
    Node,
    PoseDef,
    VarName,
    Selector,
    SurfaceKind,
    circle,
    evaluate,
    load,
    m,
    mm,
)

from placing import against, placed, assemble_world_copies
from measuring import pinned

SQUARE = [
    (Formula.length_in(0, m), Formula.length_in(0, m)),
    (Formula.length_in(1, m), Formula.length_in(0, m)),
    (Formula.length_in(1, m), Formula.length_in(1, m)),
    (Formula.length_in(0, m), Formula.length_in(1, m)),
]


def in_world(doc, body, z=0.0):
    """`body`'s copy standing in the world, `z` up: two constructions
    are two spaces, and a measure across them reads their copies in the
    world's."""
    if z == 0.0:
        return doc.place(body, doc.world)
    return placed(doc, body, Frame.translation((0 * m, 0 * m, z * m)))


def prism(doc, height=1.0):
    """A 1 m x 1 m x `height` prism in its own coordinates."""
    outline = doc.insert(Node.polygon(SQUARE))
    return doc.insert(Node.extrude(outline, Formula.length_in(height, m)))


def slab(doc, elevation, height=1.0):
    """A 1 m x 1 m x `height` prism standing in the world with its base
    at `elevation`."""
    return in_world(doc, prism(doc, height), elevation)


def cylinder_body(doc, centre_x, radius, height=0.5):
    """A circular prism in its own coordinates, centred at `centre_x`."""
    outline = doc.insert(
        Node.profile(circle((centre_x * m, 0 * m), radius * m))
    )
    return doc.insert(Node.extrude(outline, Formula.length_in(height, m)))


def cylinder(doc, centre_x, radius, height=0.5):
    """A circular prism standing on the world xy plane, centred at
    `centre_x`."""
    return in_world(doc, cylinder_body(doc, centre_x, radius, height))


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


def two_faces():
    """A slab's bottom and top faces, as `(node, name)` references."""
    doc = Doc()
    node = slab(doc, 0.0)
    ev = evaluate(doc)
    return (node, face_at_height(ev, node, 0.0)), (node, face_at_height(ev, node, 1.0))


class TestTheVerbVocabulary(unittest.TestCase):
    """The three primitives as VALUES: what each one says about itself
    before it is a node."""

    def test_each_verb_names_itself_and_its_dimension(self):
        a, b = two_faces()
        rows = [
            (MeasurePrimitive.distance(a, b), "distance", "length"),
            (MeasurePrimitive.angle(a, b), "angle", "angle"),
            (MeasurePrimitive.gap(a, b), "gap", "length"),
        ]
        for prim, verb, dimension in rows:
            with self.subTest(verb=verb):
                self.assertEqual(prim.verb, verb)
                self.assertEqual(prim.dimension, dimension)

    def test_a_gaps_pair_keeps_its_roles_and_is_not_re_sorted(self):
        """C5's formulas are asymmetric in the mating roles, so the
        order is authored data rather than a set."""
        a, b = two_faces()
        self.assertEqual(MeasurePrimitive.gap(a, b).refs, (a, b))
        self.assertEqual(MeasurePrimitive.gap(b, a).refs, (b, a))
        self.assertNotEqual(MeasurePrimitive.gap(a, b), MeasurePrimitive.gap(b, a))

    def test_primitives_are_values_comparable_and_hashable(self):
        a, b = two_faces()
        one = MeasurePrimitive.distance(a, b)
        same = MeasurePrimitive.distance(a, b)
        self.assertEqual(one, same)
        self.assertEqual(hash(one), hash(same))
        # A verb is part of the value: two primitives over the same
        # references measuring different things are different values.
        self.assertNotEqual(one, MeasurePrimitive.angle(a, b))
        self.assertEqual(len({one, same, MeasurePrimitive.angle(a, b)}), 2)
        self.assertEqual(repr(one), "MeasurePrimitive.distance(…)")

    def test_a_name_that_is_not_a_name_refuses_at_the_constructor(self):
        (node, _), _ = two_faces()
        with self.assertRaises(ValueError):
            MeasurePrimitive.distance((node, "the top face"), (node, "the other"))

    def test_the_three_relations_keep_the_kernels_symbols(self):
        self.assertEqual(AssertionRelation.AtLeast.symbol, ">=")
        self.assertEqual(AssertionRelation.AtMost.symbol, "<=")
        self.assertEqual(AssertionRelation.Equal.symbol, "=")
        self.assertEqual(AssertionRelation.AtLeast, AssertionRelation.AtLeast)
        self.assertNotEqual(AssertionRelation.AtLeast, AssertionRelation.AtMost)
        self.assertNotEqual(AssertionRelation.Equal, AssertionRelation.AtLeast)


class TestTheArithmeticIsAFormula(unittest.TestCase):
    """A measurement with arithmetic is measures and a formula over
    their outputs: `Doc.measure` records the measures as one action, and
    the arithmetic is the document's own expression language."""

    def test_doc_measure_records_one_measure_per_primitive(self):
        doc = Doc()
        node = slab(doc, 0.0, height=2.0)
        ev = evaluate(doc)
        a = (node, face_at_height(ev, node, 0.0))
        b = (node, face_at_height(ev, node, 2.0))
        recorded = doc.measure(
            [MeasurePrimitive.distance(a, b), MeasurePrimitive.angle(a, b)]
        )
        self.assertEqual(len(recorded.measures), 2)
        self.assertEqual(
            [doc.output(m) for m in recorded.measures], recorded.outputs
        )
        self.assertEqual([v.kind for v in recorded.outputs], ["length", "angle"])
        self.assertTrue(all(doc.node_kind(m) == "measure" for m in recorded.measures))

    def test_the_lattice_is_the_formulas(self):
        """The arithmetic over a measured value is checked by the
        formula's own rules: a length plus an angle refuses at the text
        door, as any document expression does."""
        doc = Doc()
        node = slab(doc, 0.0)
        ev = evaluate(doc)
        a = (node, face_at_height(ev, node, 0.0))
        b = (node, face_at_height(ev, node, 1.0))
        recorded = doc.measure([MeasurePrimitive.distance(a, b)])
        doc.apply(DocEdit.rename_var(recorded.outputs[0], VarName("depth")))
        self.assertEqual(doc.parse_formula("depth - 1 mm").dimension, "length")
        with self.assertRaises(pncad.ParseError):
            doc.parse_formula("depth + 1 rad")


class TestTheClosedForms(unittest.TestCase):
    """The four verbs against oracles the authoring fixes."""

    def test_the_distance_between_two_walls_is_the_axis_separation(self):
        doc = Doc()
        offset = 0.30
        left = cylinder(doc, -offset, 0.2)
        right = cylinder(doc, offset, 0.2)
        ev = evaluate(doc)
        node = doc.insert(
            Node.measure(MeasurePrimitive.distance((left, wall(ev, left)), (right, wall(ev, right))))
        )
        answer = measured(doc, node)
        self.assertEqual(answer.dimension, "Length")
        # The centres were written at x = ±offset; nothing else decides
        # this number.
        self.assertAlmostEqual(answer.value, 2 * offset, places=12)
        # A Length measure answers through the typed quantity too.
        self.assertAlmostEqual(answer.length.meters, 2 * offset, places=12)

    def test_a_distance_reads_two_axes_read_off_the_walls(self):
        """A reference may be a pose read off a cell: the axes of two
        parallel walls read their separation, which is what an
        assertion `= 0` over an unproven coaxiality says. The
        references come back as they were passed."""
        doc = Doc()
        offset = 0.30
        left = cylinder(doc, -offset, 0.2)
        right = cylinder(doc, offset, 0.2)
        ev = evaluate(doc)
        a = PoseDef.axis(left, wall(ev, left))
        b = PoseDef.axis(right, wall(ev, right))
        primitive = MeasurePrimitive.distance(a, b)
        self.assertIsInstance(primitive.refs[0], PoseDef)
        node = doc.insert(Node.measure(primitive))
        self.assertAlmostEqual(measured(doc, node).value, 2 * offset, places=12)

    def test_the_angle_between_opposed_caps_is_a_straight_angle(self):
        """An extruded prism's caps are parallel with OPPOSED chart
        normals, so the angle between their carriers is pi. The oracle
        is the authoring, not the codomain: `0 <= a <= pi` would be
        satisfied by every possible answer."""
        doc = Doc()
        prism = slab(doc, 0.0, height=0.4)
        ev = evaluate(doc)
        node = doc.insert(
            Node.measure(MeasurePrimitive.angle((prism, face_at_height(ev, prism, 0.0)), (prism, face_at_height(ev, prism, 0.4))))
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
                    Node.measure(MeasurePrimitive.gap(outer, inner))
                )
                self.assertAlmostEqual(measured(doc, node).value, air, places=12)

    def test_the_gap_sign_convention_walks_all_three_regimes_on_two_coaxial_rods(self):
        """Two solid rods on ONE axis, so the axis offset is zero. Their
        walls are both convex, a nested pair, so the signed gap is how
        far the inner wall stands out of the outer rod's material,
        exactly `r_inner - r_outer`: all three signs from one document
        shape."""
        for outer_r, inner_r in ((0.51, 0.50), (0.50, 0.50), (0.49, 0.50)):
            with self.subTest(outer=outer_r, inner=inner_r):
                doc = Doc()
                outer = cylinder(doc, 0.0, outer_r)
                inner = cylinder(doc, 0.0, inner_r)
                ev = evaluate(doc)
                node = doc.insert(
                    Node.measure(MeasurePrimitive.gap((outer, wall(ev, outer)), (inner, wall(ev, inner))))
                )
                answer = measured(doc, node)
                self.assertEqual(answer.dimension, "Length")
                self.assertAlmostEqual(answer.value, inner_r - outer_r, places=12)

    def test_the_authors_own_arithmetic_spells_the_web(self):
        """The worked example's shape: the web between two holes is
        `distance(wall, wall) - 2 * r`, spelled as the author's
        arithmetic rather than hidden inside a primitive."""
        doc = Doc()
        offset, radius = 0.30, 0.2
        doc.apply(
            DocEdit.declare_var(VarName("hole_r"), FreeVar.length(radius * m))
        )
        left = cylinder(doc, -offset, radius)
        right = cylinder(doc, offset, radius)
        ev = evaluate(doc)
        recorded = doc.measure(
            [MeasurePrimitive.distance((left, wall(ev, left)), (right, wall(ev, right)))]
        )
        doc.apply(DocEdit.rename_var(recorded.outputs[0], VarName("span")))
        web = doc.parse_formula("span - (hole_r + hole_r)")
        # One measure, the distance; the arithmetic is a formula over
        # its output that no node holds.
        self.assertEqual(len(recorded.measures), 1)
        self.assertEqual(len(recorded.outputs), 1)
        self.assertAlmostEqual(
            measured(doc, recorded.measures[0]).value, 2 * offset, places=12
        )
        # 0.60 between the axes, less both radii.
        self.assertAlmostEqual(
            evaluate(doc).reading(web).value,
            2 * offset - 2 * radius,
            places=12,
        )

    def test_a_measure_reads_the_placed_carrier(self):
        """The reference names the node its carrier is READ AT, and
        that is what makes a measure report moved geometry.

        Two copies of ONE prism, the second `travel` along x: each
        names the prism's corner under its own placement, and the
        distance between the corner read at one copy and the same
        corner read at the other is exactly the translation. The same
        reference read at one site measures against itself: zero.
        """
        doc = Doc()
        travel = 100.0
        body = prism(doc)
        here = in_world(doc, body)
        moved = placed(doc, body, Frame.translation((travel * m, 0 * m, 0 * m)))
        ev = evaluate(doc)
        corner = sorted(ev.all_vertices(moved))[0]
        at = [c.meters for c in ev.vertex_position(moved, corner)]
        same = [
            v
            for v in ev.all_vertices(here)
            if max(
                abs(c.meters - w)
                for c, w in zip(ev.vertex_position(here, v), (at[0] - travel, at[1], at[2]), strict=True)
            )
            < 1e-9
        ]
        self.assertEqual(len(same), 1, "each copy holds the prism's corner once")
        measure = doc.insert(
            Node.measure(MeasurePrimitive.distance((here, same[0]), (moved, corner)))
        )
        self.assertAlmostEqual(measured(doc, measure).value, travel, places=9)
        # The same name read at ONE site measures against itself: zero,
        # which is what makes the number above the translation and not
        # an artefact of which vertex was picked.
        authored = doc.insert(
            Node.measure(MeasurePrimitive.distance((here, same[0]), (here, same[0])))
        )
        self.assertAlmostEqual(measured(doc, authored).value, 0.0, places=12)


class TestTheClearanceRequirement(unittest.TestCase):
    """`Node.clearance`: a requirement with no value, and the third
    state the `f64` scalar answers it with."""

    def seats(self):
        doc = Doc()
        left = cylinder(doc, -0.30, 0.2)
        right = cylinder(doc, 0.30, 0.2)
        ev = evaluate(doc)
        return doc, (left, wall(ev, left)), (right, wall(ev, right))

    def test_at_f64_every_bound_shape_answers_the_third_state(self):
        """E10's third state used for what it is for: the requirement
        is recorded, the run cannot answer it, and the refusal names
        the lane that can. NOT a poisoning — the node evaluates."""
        doc, a, b = self.seats()
        f = doc.parse_formula
        shapes = {
            "at_least": dict(at_least=f("1 mm")),
            "at_most": dict(at_most=f("1 m")),
            "band": dict(at_least=f("1 mm"), at_most=f("1 m")),
            "equal": dict(equal=f("0.2 m")),
        }
        for shape, bounds in shapes.items():
            with self.subTest(shape=shape):
                node = doc.insert(Node.clearance(a, b, **bounds))
                self.assertEqual(doc.node_kind(node), "assertion")
                answer = verdict(doc, node)
                self.assertEqual(answer.status, "Refused")
                self.assertIsNone(answer.holds)
                self.assertEqual(answer.refusal, "needs_certifying_lane")
                self.assertIn("interval lane", answer.reason)
                # No clearance value exists, and no pair decided it.
                self.assertIsNone(answer.measured)
                self.assertIsNone(answer.bound)
                self.assertIsNone(answer.pair)

    def test_a_bound_that_is_not_a_length_refuses_at_the_door(self):
        """The bounds are lengths: an angle refuses at insert, never a
        comparison of radians with metres."""
        doc, a, b = self.seats()
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.clearance(a, b, at_least=doc.parse_formula("1 deg")))
        self.assertEqual(caught.exception.variant, "payload_var_kind")

    def test_a_bounds_combination_outside_the_four_shapes_refuses(self):
        doc, a, b = self.seats()
        c = doc.parse_formula("1 mm")
        for bounds in (
            dict(),
            dict(equal=c, at_least=c),
            dict(equal=c, at_most=c),
            dict(equal=c, at_least=c, at_most=c),
        ):
            with self.subTest(bounds=sorted(bounds)):
                with self.assertRaises(ValueError):
                    Node.clearance(a, b, **bounds)

    def test_it_refuses_typed_on_an_edge_seat(self):
        """A seat's entity kind is the selection's face scope, and an
        edge names no faces at all: a seat reads a body or a face, so
        the insert refuses by the seat's kind."""
        doc = Doc()
        left = slab(doc, 0.0)
        right = slab(doc, 4.0)
        ev = evaluate(doc)
        with self.assertRaises(EditError) as caught:
            doc.insert(
                Node.clearance(
                    (left, ev.all_edges(left)[0]),
                    (right, ev.all_edges(right)[0]),
                    at_least=doc.parse_formula("1 mm"),
                )
            )
        self.assertEqual(caught.exception.variant, "slot_var_kind")


class TestTheAssertion(unittest.TestCase):
    """A recorded requirement, its three states, and the parameter edit
    that moves it."""

    def web_document(self, bound_mm):
        """Two cylinders 0.6 m apart, a distance measure over their
        walls, and an assertion whose bound is a PARAMETER."""
        doc = Doc()
        doc.apply(
            DocEdit.declare_var(VarName("bound"), FreeVar.length(bound_mm * mm))
        )
        left = cylinder(doc, -0.30, 0.2)
        right = cylinder(doc, 0.30, 0.2)
        ev = evaluate(doc)
        measure = doc.insert(
            Node.measure(MeasurePrimitive.distance((left, wall(ev, left)), (right, wall(ev, right))))
        )
        return doc, measure

    def test_one_parameter_edit_flips_the_verdict(self):
        """The bound is a `Formula`, so it reaches a document parameter —
        which is what makes a recorded requirement re-decidable without
        re-authoring the node."""
        doc, measure = self.web_document(500.0)
        assertion = doc.insert(
            Node.assertion(doc.output(measure), AssertionRelation.AtLeast, doc.parse_formula("bound"))
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
            DocEdit.set_var_value(
                VarName("bound"), FreeValue.length(700 * mm)
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
            Node.assertion(doc.output(measure), AssertionRelation.AtMost, doc.parse_formula("bound"))
        )
        self.assertEqual(verdict(doc, assertion).status, "Holds")
        doc.apply(
            DocEdit.set_var_value(
                VarName("bound"), FreeValue.length(500 * mm)
            )
        )
        self.assertEqual(verdict(doc, assertion).status, "Violated")

    def test_equal_holds_only_at_the_bound(self):
        """`relation=AssertionRelation.Equal`: the 0.6 m web equals a
        600 mm bound and is violated by 500 mm and 700 mm alike."""
        doc, measure = self.web_document(600.0)
        assertion = doc.insert(
            Node.assertion(
                doc.output(measure),
                relation=AssertionRelation.Equal,
                bound=doc.parse_formula("bound"),
            )
        )
        self.assertEqual(verdict(doc, assertion).status, "Holds")
        for off in (500, 700):
            doc.apply(DocEdit.set_var_value(VarName("bound"), FreeValue.length(off * mm)))
            self.assertEqual(verdict(doc, assertion).status, "Violated", off)

    def test_a_verdict_is_report_only(self):
        """A `Violated` assertion changes no downstream outcome: the
        same document with and without it saves the same recipe for
        every other node, and the assertion denotes no body."""
        doc, measure = self.web_document(700.0)
        without = doc.save()
        assertion = doc.insert(
            Node.assertion(doc.output(measure), AssertionRelation.AtLeast, doc.parse_formula("bound"))
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
        node = slab(doc, 0.0)
        ev = evaluate(doc)
        bottom = face_at_height(ev, node, 0.0)
        side = next(f for f in ev.all_faces(node) if f not in (bottom, face_at_height(ev, node, 1.0)))
        # A cap and a side wall are not parallel, so the gap fails.
        measure = doc.insert(Node.measure(MeasurePrimitive.gap((node, bottom), (node, side))))
        assertion = doc.insert(
            Node.assertion(doc.output(measure), AssertionRelation.AtLeast, doc.parse_formula("1 m"))
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(assertion)
        self.assertEqual(caught.exception.kind, "measure_not_parallel")
        self.assertEqual(caught.exception.through, measure)

    def test_an_assertion_over_a_non_finite_value_fails_itself(self):
        """Arithmetic over measured values is a formula the assertion
        reads, so a value that goes non-finite fails the assertion's own
        payload — no measure failed, and nothing is poisoned."""
        doc = Doc()
        doc.apply(DocEdit.declare_var(VarName("s"), FreeVar.scalar(0.0)))
        # `13 m / s` with `s` bound to zero.
        over_zero = doc.parse_formula("13 m / s")
        assertion = doc.insert(
            Node.assertion(over_zero, AssertionRelation.AtLeast, doc.parse_formula("1 m"))
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(assertion)
        self.assertEqual(caught.exception.kind, "payload_expr")


class TestTheRefusals(unittest.TestCase):
    """Every door that says no, reached from Python."""

    def one_face(self):
        doc = Doc()
        node = slab(doc, 0.0)
        return doc, node, face_at_height(evaluate(doc), node, 0.0)

    def test_a_measure_over_two_references_is_accepted(self):
        doc, node, face = self.one_face()
        top = face_at_height(evaluate(doc), node, 1.0)
        built = Node.measure(MeasurePrimitive.distance((node, face), (node, top)))
        self.assertEqual(doc.node_kind(doc.insert(built)), "measure")

    def test_a_name_that_is_not_a_name_refuses_at_the_boundary(self):
        _doc, node, _ = self.one_face()
        with self.assertRaises(ValueError):
            Node.measure(MeasurePrimitive.distance((node, "the top face"), (node, "the other one")))

    def test_an_assertion_must_read_a_scalar(self):
        doc, node, _ = self.one_face()
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.assertion(doc.output(node), AssertionRelation.AtLeast, doc.parse_formula("1 m")))
        self.assertEqual(caught.exception.variant, "payload_var_kind")

    def test_a_construction_cannot_read_a_measured_value(self):
        """A measure's output is observed (D10): only an assertion reads
        it, and a construction's slot reading it refuses at the door."""
        doc = Doc()
        node = prism(doc)
        ev = evaluate(doc)
        face, top = face_at_height(ev, node, 0.0), face_at_height(ev, node, 1.0)
        measure = doc.insert(
            Node.measure(MeasurePrimitive.distance((node, face), (node, top)))
        )
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_param(node, "distance", doc.output(measure)))
        self.assertEqual(caught.exception.variant, "construction_reads_observed")
        self.assertEqual(caught.exception.node, node)

    def test_an_assertion_compares_like_with_like_or_not_at_all(self):
        """The bound's dimension is the MEASURE's, and the edit door is
        where that is checked because it is the door that holds the
        document."""
        doc = Doc()
        node = slab(doc, 0.0)
        ev = evaluate(doc)
        measure = doc.insert(
            Node.measure(MeasurePrimitive.angle((node, face_at_height(ev, node, 0.0)), (node, face_at_height(ev, node, 1.0))))
        )
        with self.assertRaises(EditError) as caught:
            doc.insert(
                Node.assertion(doc.output(measure), AssertionRelation.AtMost, doc.parse_formula("1 m"))
            )
        self.assertEqual(caught.exception.variant, "assertion_dimension")
        # And the matching dimension is accepted, so the row above is
        # about the mismatch rather than about assertions on angles.
        doc.insert(Node.assertion(doc.output(measure), AssertionRelation.AtMost, doc.parse_formula("4 rad")))

    def test_deleting_a_referenced_node_strands_the_measure(self):
        """A measure CONSUMES the values it names, so its references
        are selections that read their body — and, like every read,
        deleting that body is accepted and reported on the measure
        twice: its selections read an output the delete took, and name
        entities it minted. It refuses at evaluation until re-pointed."""
        doc = Doc()
        node = slab(doc, 0.0)
        ev = evaluate(doc)
        measure = doc.insert(
            Node.measure(MeasurePrimitive.gap((node, face_at_height(ev, node, 0.0)), (node, face_at_height(ev, node, 1.0))))
        )
        doc.apply(DocEdit.delete_node(node))
        self.assertEqual(
            {
                (m.variant, m.node)
                for m in doc.last_maintenance
                if m.variant != "anonymous_var_removed"
            },
            {("stranded_read", measure), ("stranded_selection", measure)},
        )
        with self.assertRaises(EvaluationError):
            evaluate(doc).value(measure)

    def test_a_carrier_pair_with_no_closed_form_refuses_naming_the_pair(self):
        """A plane against a cylinder has no v1 closed form, and the
        refusal names the pair class rather than guessing an arm."""
        doc = Doc()
        block = slab(doc, 2.0)
        post = cylinder(doc, 0.0, 0.2)
        ev = evaluate(doc)
        node = doc.insert(
            Node.measure(MeasurePrimitive.distance((block, face_at_height(ev, block, 2.0)), (post, wall(ev, post))))
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(node).measure()
        self.assertEqual(caught.exception.kind, "measure_unsupported")

    def test_a_whole_body_is_no_distance_reference(self):
        """A `distance` reads a face, an edge or a vertex, so a whole
        body refuses at insert by the seat's kind."""
        doc = Doc()
        left = slab(doc, 0.0)
        right = slab(doc, 4.0)
        ev = evaluate(doc)
        with self.assertRaises(EditError) as caught:
            doc.insert(
                Node.measure(MeasurePrimitive.distance((left, ev.all_bodies(left)[0]), (right, ev.all_bodies(right)[0])))
            )
        self.assertEqual(caught.exception.variant, "slot_var_kind")

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
            Node.measure(MeasurePrimitive.gap((node, bottom), (node, side)))
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
            Node.measure(MeasurePrimitive.distance((elsewhere, alien), (elsewhere, face_at_height(ev, elsewhere, 4.0))))
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(measure)
        self.assertEqual(caught.exception.kind, "select_resolve")

    def test_the_load_door_refuses_a_construction_reading_a_measured_value(self):
        """The edit door refuses a construction reading a measure's
        output (`construction_reads_observed`), so the only way one
        reaches a document is a hand-edited file — and the load door
        runs the same rule, refusing in the snapshot's own words.

        It arrives as `PersistError` with `variant == "snapshot"`, the
        rule in the message.
        """
        import json

        doc = Doc()
        node = slab(doc, 0.0)
        ev = evaluate(doc)
        measure = doc.insert(
            Node.measure(MeasurePrimitive.distance((node, face_at_height(ev, node, 0.0)), (node, face_at_height(ev, node, 1.0))))
        )
        slab(doc, 4.0)
        output = doc.output(measure)
        self.assertIsNotNone(output)
        header, body = doc.save().split("\n", 1)
        wire = json.loads(body)
        nodes = wire["snapshot"]["nodes"]
        extrudes = sorted(
            (key for key, held in nodes.items() if "Extrude" in held),
            key=lambda key: int(key.split(":")[0]),
        )
        # The later slab's depth is re-pointed at the measured value.
        nodes[extrudes[-1]]["Extrude"]["distance"] = output.hex
        tampered = header + "\n" + json.dumps(wire, indent=2) + "\n"
        with self.assertRaises(pncad.PersistError) as caught:
            load(tampered)
        self.assertEqual(caught.exception.variant, "snapshot")
        self.assertIn("a measured value, which only an assertion reads", str(caught.exception))

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
        doc.apply(DocEdit.declare_var(VarName("bound"), FreeVar.length(1 * mm)))
        left = cylinder(doc, -0.30, 0.2)
        right = cylinder(doc, 0.30, 0.2)
        ev = evaluate(doc)
        recorded = doc.measure(
            [MeasurePrimitive.distance((left, wall(ev, left)), (right, wall(ev, right)))]
        )
        doc.apply(DocEdit.rename_var(recorded.outputs[0], VarName("span")))
        assertion = doc.insert(
            Node.assertion(
                doc.parse_formula("span - bound"),
                AssertionRelation.AtLeast,
                doc.parse_formula("0.1 m"),
            )
        )
        return doc, recorded.measures[0], assertion

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

    def test_a_measure_over_an_unplaced_body_leaves_the_product_alone(self):
        """The ordinary case, and the one the worked example is in:
        the measure's references sit UNDER the placed union, the
        measure places nothing, and the gathered solid is the same
        one."""
        doc = Doc()
        left = cylinder_body(doc, -0.30, 0.2)
        right = against(
            doc,
            left, cylinder_body(doc, 0.30, 0.2), Frame.translation((0 * m, 0 * m, 0 * m))
        )
        placed = doc.place(doc.insert(Node.union([left, right])), doc.world)
        placements = doc.placements()
        self.assertIn(placed, placements)
        assemble_world_copies(doc)
        before = pinned(pncad.product(doc, evaluate(doc))).volume
        ev = evaluate(doc)
        doc.insert(
            Node.measure(MeasurePrimitive.distance((left, wall(ev, left)), (right, wall(ev, right))))
        )
        self.assertEqual(doc.placements(), placements)
        self.assertAlmostEqual(
            pinned(pncad.product(doc, evaluate(doc))).volume,
            before,
            places=12,
        )

    def test_a_measure_over_the_placed_bodies_takes_nothing(self):
        """A measure reads the bodies it measures and places nothing,
        so measuring the two placed bodies leaves the world as it was:
        the product is still both, whatever reads them."""
        doc = Doc()
        left = cylinder(doc, -0.30, 0.2)
        right = cylinder(doc, 0.30, 0.2)
        placed = doc.placements()
        self.assertEqual(placed, [left, right])
        assemble_world_copies(doc)
        before = pinned(pncad.product(doc, evaluate(doc))).volume
        self.assertGreater(before, 0.0)
        ev = evaluate(doc)
        doc.insert(
            Node.measure(MeasurePrimitive.distance((left, wall(ev, left)), (right, wall(ev, right))))
        )
        self.assertEqual(doc.placements(), placed)
        self.assertEqual(
            pinned(pncad.product(doc, evaluate(doc))).volume, before
        )


if __name__ == "__main__":
    unittest.main()
