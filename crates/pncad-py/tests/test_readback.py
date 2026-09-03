"""The geometry read-back doors (LIB-B-READBACK).

`crate::select`'s third invariant is **a name answers with values,
never keys**, and until this unit Python had the first half only:
`Evaluation.all_faces` handed back names and nothing asked one where
it sat. These are the door-level rows for the four that now do —
`face_frame`, `edge_frame`, `vertex_position`, `denotation` — and for
the vocabulary they answer in.

`test_assembly_author.py::TestBenchLayout` is the scene-scale row: the
audit's row 47, where an instance's cap frame is read on the layout
evaluation and checked against the placement arithmetic. What is here
is the behavior of the doors themselves, and above all their
REFUSALS: a read-back that cannot answer says which invariant broke,
in the kernel's own words, and never guesses.

NOTHING HERE READS INSIDE A NAME. Every name is a materializer's
opaque text handed straight back, which is the whole point of the
doors: the alphabet does not change between asking for a name and
asking where it is.
"""

import math
import unittest

from pncad import (
    CapEnd,
    Denotation,
    Doc,
    EntityKind,
    NamePat,
    Node,
    Pose,
    PncadError,
    ReadbackError,
    SegPat,
    SegTag,
    Selector,
    evaluate,
    m,
)


def unit_cube(doc):
    """A 1 m cube on the ground plane, rooted at the origin."""
    square = doc.insert(
        Node.polygon([(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)], plane=doc.sketch_frame())
    )
    return doc.insert(Node.extrude(square, 1 * m))


def one(found):
    assert len(found) == 1, f"expected exactly one name, got {found}"
    return found[0]


def top_cap(ev, node):
    return one(
        ev.select(
            node,
            Selector.of(
                NamePat.of_kind(EntityKind.Face).seg(
                    SegPat.tag(SegTag.Cap).side(CapEnd.Top)
                )
            ),
        )
    )


class TestTheDoorsAnswerWithValues(unittest.TestCase):
    """The cube, asked where its own geometry is."""

    def setUp(self):
        self.doc = Doc()
        self.cube = unit_cube(self.doc)
        self.ev = evaluate(self.doc)

    def test_a_cap_face_answers_with_its_carriers_plane(self):
        pose = self.ev.face_frame(self.cube, top_cap(self.ev, self.cube))
        # The top cap's carrier is the plane z = 1, and the axis is
        # the CHART's normal — a direction, dimensionless — while the
        # origin is a POSITION and crosses dimensioned.
        self.assertAlmostEqual(pose.origin[2].meters, 1.0, delta=1e-12)
        self.assertAlmostEqual(abs(pose.axis[2]), 1.0, delta=1e-12)
        self.assertIsInstance(pose, Pose)

    def test_a_planes_triad_is_right_handed_and_complete(self):
        pose = self.ev.face_frame(self.cube, top_cap(self.ev, self.cube))
        # A plane's convention fixes a `u_ref`, so `v_ref` exists too
        # and is `axis x u_ref` — the door computes it rather than
        # making the caller redo the cross product.
        self.assertIsNotNone(pose.u_ref)
        axis, u_ref, v_ref = pose.axis, pose.u_ref, pose.v_ref
        cross = (
            axis[1] * u_ref[2] - axis[2] * u_ref[1],
            axis[2] * u_ref[0] - axis[0] * u_ref[2],
            axis[0] * u_ref[1] - axis[1] * u_ref[0],
        )
        for got, want in zip(v_ref, cross, strict=True):
            self.assertAlmostEqual(got, want, delta=1e-15)

    def test_a_straight_edge_has_no_distinguished_perpendicular(self):
        """Rule 3, crossing: where the stored geometry fixes no
        convention the door says so rather than inventing one. A line
        has a direction and no reference perpendicular, so `u_ref` is
        `None` — and `v_ref` with it."""
        edge = self.ev.all_edges(self.cube)[0]
        pose = self.ev.edge_frame(self.cube, edge)
        self.assertIsNone(pose.u_ref)
        self.assertIsNone(pose.v_ref)
        # The axis is still there: a line HAS a direction.
        length = math.sqrt(sum(c * c for c in pose.axis))
        self.assertAlmostEqual(length, 1.0, delta=1e-12)

    def test_a_vertex_answers_with_a_dimensioned_position(self):
        """A cube's eight corners are the eight coordinates of
        {0, 1}^3, read back one name at a time and never transcribed
        as literals."""
        corners = {
            tuple(round(c.meters, 12) for c in self.ev.vertex_position(self.cube, v))
            for v in self.ev.all_vertices(self.cube)
        }
        self.assertEqual(
            corners,
            {(x, y, z) for x in (0.0, 1.0) for y in (0.0, 1.0) for z in (0.0, 1.0)},
        )

    def test_a_pose_carries_no_equality(self):
        """Comparing coordinates is a tolerance question, and an
        exact-bit `==` would be a decided predicate wearing an
        operator. Two reads of the SAME face are distinct objects and
        compare unequal; the components are what a caller compares."""
        cap = top_cap(self.ev, self.cube)
        first = self.ev.face_frame(self.cube, cap)
        second = self.ev.face_frame(self.cube, cap)
        self.assertNotEqual(first, second)
        self.assertAlmostEqual(
            first.origin[2].meters, second.origin[2].meters, delta=0.0
        )

    def test_a_name_says_how_it_denotes_before_it_is_read(self):
        cap = top_cap(self.ev, self.cube)
        denotation = self.ev.denotation(self.cube, cap)
        self.assertIsInstance(denotation, Denotation)
        self.assertFalse(denotation.tied)
        self.assertEqual(denotation.candidates, 1)
        self.assertEqual(denotation, self.ev.denotation(self.cube, cap))


class TestTheDoorsRefuseTyped(unittest.TestCase):
    """Every refusal reachable from a document Python can author, one
    row per arm, asserted on the `variant` tag and the payload.

    WHICH ARMS ARE NOT HERE, measured rather than assumed:
    `dangling_entity`, `dangling_geometry` and `no_such_body` are
    kernel-bug arms — a stale handle, a live entity naming geometry
    the body no longer has, an emission that disagrees with its own
    value — and nothing a caller can author reaches one. The two
    dangling lanes are separate tags because they are separate facts;
    their texts are pinned in the Rust tag suite, which is the only
    place either arm can be constructed.
    `no_carrier` needs an edge carrying M3 null-edge scaffolding,
    which is a transient state no evaluated value is in.
    `no_canonical_frame` needs a NURBS carrier, which no bound node
    produces today. `node_failed` and `node_poisoned` are the
    evaluation ladder and are reached through `Evaluation.value`'s own
    rows rather than duplicated here. Each is bound and tagged; a row
    appears when some other door reaches it.
    """

    def setUp(self):
        self.doc = Doc()
        self.cube = unit_cube(self.doc)
        self.ev = evaluate(self.doc)

    def refusal(self, call):
        with self.assertRaises(ReadbackError) as caught:
            call()
        return caught.exception

    def test_the_class_is_a_pncad_error(self):
        self.assertTrue(issubclass(ReadbackError, PncadError))

    def test_a_face_door_handed_an_edge_name_says_which_kind_it_reads(self):
        edge = self.ev.all_edges(self.cube)[0]
        err = self.refusal(lambda: self.ev.face_frame(self.cube, edge))
        self.assertEqual(err.variant, "wrong_kind")
        # The two kinds are VALUES, not prose: a caller dispatches on
        # them without reading the message.
        self.assertEqual(err.wanted, EntityKind.Face)
        self.assertEqual(err.found, EntityKind.Edge)
        self.assertIn("kind mismatch", str(err))

    def test_a_vertex_door_handed_a_face_name_refuses_the_same_way(self):
        err = self.refusal(
            lambda: self.ev.vertex_position(self.cube, top_cap(self.ev, self.cube))
        )
        self.assertEqual(err.variant, "wrong_kind")
        self.assertEqual(err.wanted, EntityKind.Vertex)
        self.assertEqual(err.found, EntityKind.Face)

    def test_a_whole_body_has_no_single_frame_and_says_which_to_ask(self):
        """A body name is NOT a kind mismatch: the door refuses one
        rung earlier, because "a body has no single frame" is a
        different fact from "this door reads faces" — and the message
        carries the recourse, which is to ask about a face, an edge or
        a vertex of it."""
        body = self.ev.all_bodies(self.cube)[0]
        err = self.refusal(lambda: self.ev.face_frame(self.cube, body))
        self.assertEqual(err.variant, "whole_body")
        self.assertIsNone(err.wanted)
        self.assertIn("faces, edges, or vertices", str(err))

    def test_a_name_from_another_node_is_stale_not_silent(self):
        """The staleness the freeze exists to make visible. A stable
        name carries the node that MINTED it, so a cap name of one
        extrude asked of another answers nothing — refused, never
        matched to the sibling's identically-shaped cap."""
        second = unit_cube(self.doc)
        ev = evaluate(self.doc)
        first_cap = top_cap(ev, self.cube)
        err = self.refusal(lambda: ev.face_frame(second, first_cap))
        self.assertEqual(err.variant, "no_such_name")
        self.assertIn("stale", str(err))

    def test_the_same_recipe_mints_the_same_names_in_any_document(self):
        """Not a defect and worth pinning: a name is a function of the
        RECIPE — the minting node and the role path — not of the
        document object it was read from. Two documents built by the
        same calls answer to each other's names, and the read-back
        doors answer both. What a name is scoped to is the node, which
        the row above shows by breaking it."""
        twin = Doc()
        twin_cube = unit_cube(twin)
        twin_ev = evaluate(twin)
        cap = top_cap(twin_ev, twin_cube)
        self.assertEqual(cap, top_cap(self.ev, self.cube))
        here = self.ev.face_frame(self.cube, cap).origin
        there = twin_ev.face_frame(twin_cube, cap).origin
        for a, b in zip(here, there, strict=True):
            self.assertEqual(a.meters, b.meters)

    def test_a_node_this_run_did_not_produce_names_itself(self):
        cap = top_cap(self.ev, self.cube)
        # The node id is real and the name is well formed; what is
        # missing is a RESULT for that node in THIS evaluation, so the
        # refusal names the NODE rather than the name — the question
        # is which run you meant, not which face.
        empty = Doc()
        err = self.refusal(lambda: evaluate(empty).face_frame(self.cube, cap))
        self.assertEqual(err.variant, "node_not_evaluated")
        self.assertEqual(err.node, self.cube)

    def test_every_payload_field_is_present_on_every_arm(self):
        """The `getattr` contract: a caller reads the payload without
        first branching on `variant`, and an arm that does not carry a
        field answers `None` rather than raising."""
        edge = self.ev.all_edges(self.cube)[0]
        err = self.refusal(lambda: self.ev.face_frame(self.cube, edge))
        for field in (
            "node",
            "through",
            "candidates",
            "index",
            "payload",
            "carrier",
        ):
            self.assertIsNone(getattr(err, field), field)
        self.assertIsNotNone(err.wanted)

    def test_text_that_is_no_name_at_all_is_a_boundary_refusal(self):
        """Not a `ReadbackError`: there is no kernel refusal to
        forward, so this is the same boundary `ValueError` every other
        door raises for a string where a name belongs."""
        with self.assertRaises(ValueError):
            self.ev.face_frame(self.cube, "not a name")

    def test_the_denotation_door_refuses_a_stale_name_too(self):
        second = unit_cube(self.doc)
        ev = evaluate(self.doc)
        err = self.refusal(lambda: ev.denotation(second, top_cap(ev, self.cube)))
        self.assertEqual(err.variant, "no_such_name")


if __name__ == "__main__":
    unittest.main()
