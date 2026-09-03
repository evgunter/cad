"""Picking — the fourth door onto a name (LIB-B-PICKING).

`Evaluation.select` answers "which entities match this shape",
`face_frame` and its siblings answer "where is this named entity", and
`denotation` answers "how many does it denote". This file is the
door-level row for the fourth: **what is under this ray**.

THE ORACLES ARE THE OTHER DOORS. A pick is only worth having if it
answers in the alphabet the rest of the surface speaks, so almost every
assertion here is a cross-check against a door that already existed:
the picked name is one `all_faces` minted, the patch names ARE
`all_faces`' set, the boundary names ARE `all_edges`', and the mesh a
`NodePick` picks against is triangle-for-triangle the mesh
`Body.tessellate` draws at the same budget. Where the oracle is
arithmetic instead, it is arithmetic on a plane the document states —
a cap at z = 1 m, hit from z = 3 m, at t = 2.

NOTHING HERE READS INSIDE A NAME. Every name is an opaque text, compared
with other opaque texts and never parsed.
"""

import math
import unittest

import pncad
from pncad import (
    CapEnd,
    Doc,
    EntityKind,
    HitTestError,
    NamePat,
    Node,
    NodePick,
    NodePickError,
    PickHit,
    PncadError,
    Ray,
    SegPat,
    SegTag,
    Selector,
    deg,
    evaluate,
    m,
    mm,
)

#: The chordal budget every index in this file is built at. A cube's
#: faces are planar, so the tessellation is exact at any budget and the
#: arithmetic below does not depend on this number.
DELTA = 0.5 * mm


def square(doc, side=1.0, at=(0.0, 0.0)):
    """A `side`-metre square on the sketch plane, corner at `at`."""
    x, y = at
    return doc.insert(
        Node.polygon(
            [
                ((x + 0.0) * m, (y + 0.0) * m),
                ((x + side) * m, (y + 0.0) * m),
                ((x + side) * m, (y + side) * m),
                ((x + 0.0) * m, (y + side) * m),
            ],
            plane=doc.sketch_frame(),
        )
    )


def unit_cube(doc, at=(0.0, 0.0)):
    """A 1 m cube on the ground plane — z from 0 to 1."""
    return doc.insert(Node.extrude(square(doc, at=at), 1 * m))


def straight_down(x=0.5, y=0.5, z=3.0, scale=1.0):
    """A ray at `(x, y, z)` aimed at -Z, with `scale * 1 m` of length
    per unit of `t`."""
    return Ray((x * m, y * m, z * m), (0.0, 0.0, -scale))


def one(found):
    assert len(found) == 1, f"expected exactly one name, got {found}"
    return found[0]


def top_cap(ev, node):
    """The name of `node`'s top cap face — the selector door's answer,
    which is the oracle a pick is checked against."""
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


class TestTheRayAnswersAName(unittest.TestCase):
    """One cube, hit from straight above."""

    def setUp(self):
        self.doc = Doc()
        self.cube = unit_cube(self.doc)
        self.ev = evaluate(self.doc)
        self.pick = NodePick.build(self.ev, self.cube, 0, DELTA)

    def test_the_picked_face_is_the_one_the_selector_names(self):
        # The oracle: `select` says which name the top cap carries, and
        # the ray has to agree. Two doors, one alphabet.
        hit = self.ev.pick_face([self.pick], straight_down())
        self.assertIsInstance(hit, PickHit)
        self.assertEqual(hit.name, top_cap(self.ev, self.cube))

    def test_the_picked_name_is_a_materializers_own_text(self):
        hit = self.ev.pick_face([self.pick], straight_down())
        self.assertIn(hit.name, self.ev.all_faces(self.cube))

    def test_the_hit_carries_the_targets_own_pairing(self):
        hit = self.ev.pick_face([self.pick], straight_down())
        self.assertEqual(hit.node, self.cube)
        self.assertEqual(hit.body, 0)

    def test_t_and_the_point_agree_with_the_documents_own_arithmetic(self):
        # The cap's carrier is the plane z = 1 m (the document says so:
        # a 1 m extrude off the ground plane). A ray from z = 3 m
        # aimed at -Z with a unit direction therefore hits at t = 2,
        # and at the point directly below its origin.
        hit = self.ev.pick_face([self.pick], straight_down())
        self.assertAlmostEqual(hit.t, 2.0, places=12)
        px, py, pz = hit.point
        self.assertAlmostEqual(px.meters, 0.5, places=12)
        self.assertAlmostEqual(py.meters, 0.5, places=12)
        self.assertAlmostEqual(pz.meters, 1.0, places=12)

    def test_t_is_in_units_of_the_rays_own_direction(self):
        # Twice the direction, half the parameter, same point — the
        # documented meaning of `t`, and the reason it is a bare float
        # and `point` is the dimensioned answer.
        doubled = self.ev.pick_face([self.pick], straight_down(scale=2.0))
        self.assertAlmostEqual(doubled.t, 1.0, places=12)
        self.assertAlmostEqual(doubled.point[2].meters, 1.0, places=12)

    def test_a_pick_is_deterministic(self):
        first = self.ev.pick_face([self.pick], straight_down())
        again = self.ev.pick_face([self.pick], straight_down())
        self.assertEqual(first.name, again.name)
        # Bit-identical, not merely close: the whole chain is fixed
        # iteration order with a total tie-break and no hashing.
        self.assertEqual(first.t, again.t)

    def test_a_side_face_answers_a_different_name(self):
        # Aimed at the +X wall from outside, so the cap's name must NOT
        # be the answer.
        sideways = Ray((3 * m, 0.5 * m, 0.5 * m), (-1.0, 0.0, 0.0))
        hit = self.ev.pick_face([self.pick], sideways)
        self.assertNotEqual(hit.name, top_cap(self.ev, self.cube))
        self.assertIn(hit.name, self.ev.all_faces(self.cube))
        self.assertAlmostEqual(hit.point[0].meters, 1.0, places=12)


class TestAMissIsNotAFailure(unittest.TestCase):
    """`Ok(None)` crosses as `None`, and errors are never flattened
    into it."""

    def setUp(self):
        self.doc = Doc()
        self.cube = unit_cube(self.doc)
        self.ev = evaluate(self.doc)
        self.pick = NodePick.build(self.ev, self.cube, 0, DELTA)

    def test_a_ray_beside_the_body_misses(self):
        beside = straight_down(x=5.0, y=5.0)
        self.assertIsNone(self.ev.pick_face([self.pick], beside))

    def test_a_ray_aimed_away_misses(self):
        # `t >= 0` only: the body is BEHIND this ray's origin.
        away = Ray((0.5 * m, 0.5 * m, 3 * m), (0.0, 0.0, 1.0))
        self.assertIsNone(self.ev.pick_face([self.pick], away))

    def test_no_targets_at_all_is_a_miss(self):
        self.assertIsNone(self.ev.pick_face([], straight_down()))

    def test_a_poisoned_ray_misses_rather_than_refusing(self):
        # A non-finite ray is legal input and fail-safe: it can only
        # LOSE constraints in the slab test, so it prunes nothing, every
        # exact test misses, and the answer is the typed miss. It is
        # emphatically not a refusal.
        nan = float("nan")
        for ray in (
            Ray((nan * m, 0.5 * m, 3 * m), (0.0, 0.0, -1.0)),
            Ray((0.5 * m, 0.5 * m, 3 * m), (nan, nan, nan)),
            Ray((0.5 * m, 0.5 * m, math.inf * m), (0.0, 0.0, -1.0)),
        ):
            with self.subTest(ray=repr(ray)):
                self.assertIsNone(self.ev.pick_face([self.pick], ray))


class TestTheRayValue(unittest.TestCase):
    """`Ray` carries what it was given, and normalizes nothing."""

    def test_origin_is_dimensioned_and_direction_is_not(self):
        ray = Ray((1 * m, 2 * m, 3 * m), (0.0, 0.0, -2.0))
        self.assertEqual([q.meters for q in ray.origin], [1.0, 2.0, 3.0])
        self.assertEqual(ray.direction, (0.0, 0.0, -2.0))

    def test_the_direction_is_not_normalized(self):
        # Silently normalizing would make `t` mean something the caller
        # did not write, so the value keeps the length it was handed.
        ray = Ray((0 * m, 0 * m, 0 * m), (3.0, 4.0, 0.0))
        self.assertEqual(ray.direction, (3.0, 4.0, 0.0))

    def test_the_repr_says_the_origin_is_metres(self):
        ray = Ray((1 * m, 0 * m, 0 * m), (0.0, 0.0, -1.0))
        self.assertIn(") m", repr(ray))


class TestNearestAndTheTieBreak(unittest.TestCase):
    """Several targets, one answer, and the documented order."""

    def test_the_nearest_body_wins(self):
        doc = Doc()
        lower = unit_cube(doc)
        upper = doc.insert(
            Node.transform(lower, (0 * m, 0 * m, 2 * m), (0.0, 0.0, 1.0), 0 * deg)
        )
        ev = evaluate(doc)
        picks = [
            NodePick.build(ev, lower, 0, DELTA),
            NodePick.build(ev, upper, 0, DELTA),
        ]
        hit = ev.pick_face(picks, straight_down(z=10.0))
        # The upper cube's top cap is at z = 3 m, the lower one's at
        # z = 1 m; from z = 10 m the upper is nearer whichever order the
        # targets were offered in.
        self.assertEqual(hit.node, upper)
        self.assertAlmostEqual(hit.point[2].meters, 3.0, places=12)
        self.assertEqual(ev.pick_face(list(reversed(picks)), straight_down(z=10.0)).node, upper)

    def test_an_exact_tie_resolves_to_the_earlier_target(self):
        # Two nodes drawing the SAME cube in the same place: the ray
        # hits both at the same `t`, and the documented tie-break is
        # position in `targets`. Not chance — the same call answers the
        # other way when the list is reversed.
        doc = Doc()
        first = unit_cube(doc)
        second = doc.insert(Node.extrude(square(doc), 1 * m))
        ev = evaluate(doc)
        a = NodePick.build(ev, first, 0, DELTA)
        b = NodePick.build(ev, second, 0, DELTA)
        self.assertEqual(ev.pick_face([a, b], straight_down()).node, first)
        self.assertEqual(ev.pick_face([b, a], straight_down()).node, second)


class TestThePairingIsTrueByConstruction(unittest.TestCase):
    """What a `NodePick` knows about itself, and the mesh it carries."""

    def setUp(self):
        self.doc = Doc()
        self.cube = unit_cube(self.doc)
        self.ev = evaluate(self.doc)
        self.pick = NodePick.build(self.ev, self.cube, 0, DELTA)

    def test_the_index_names_the_pair_it_was_built_from(self):
        self.assertEqual(self.pick.node, self.cube)
        self.assertEqual(self.pick.body, 0)

    def test_what_is_drawn_is_what_is_picked(self):
        # The oracle: tessellating the same body at the same budget
        # through the ordinary door must give the same mesh, triangle
        # for triangle. One tessellation, one source of truth.
        drawn = self.ev.value(self.cube).body().tessellate(DELTA)
        self.assertEqual(self.pick.mesh.patch_count, drawn.patch_count)
        self.assertEqual(self.pick.mesh.triangles, drawn.triangles)

    def test_the_mesh_handle_is_stable_across_reads(self):
        self.assertEqual(
            self.pick.mesh.triangle_count, self.pick.mesh.triangle_count
        )

    def test_a_cube_has_six_patches(self):
        self.assertEqual(self.pick.mesh.patch_count, 6)

    def test_the_repr_says_the_pair(self):
        self.assertIn("patches", repr(self.pick))


class TestThePatchInversion(unittest.TestCase):
    """The door a display consumer needs: patch index in, name out."""

    def setUp(self):
        self.doc = Doc()
        self.cube = unit_cube(self.doc)
        self.ev = evaluate(self.doc)
        self.pick = NodePick.build(self.ev, self.cube, 0, DELTA)

    def test_the_patch_names_are_exactly_the_face_names(self):
        # The oracle: `all_faces` is the face-name materializer, and a
        # patch is a face. Same set, same alphabet, and one entry per
        # patch so the positions line up with `Mesh.patch(i)`.
        names = self.pick.patch_names(self.ev)
        self.assertEqual(len(names), self.pick.mesh.patch_count)
        self.assertTrue(all(isinstance(n, str) for n in names))
        self.assertEqual(set(names), set(self.ev.all_faces(self.cube)))

    def test_the_boundary_names_are_exactly_the_edge_names(self):
        names = self.pick.boundary_names(self.ev)
        self.assertTrue(all(isinstance(n, str) for n in names))
        self.assertEqual(set(names), set(self.ev.all_edges(self.cube)))

    def test_a_patch_name_is_the_name_a_pick_answers(self):
        # The two inversions have to agree: the face a ray hits is one
        # of the faces the patch list names.
        hit = self.ev.pick_face([self.pick], straight_down())
        self.assertIn(hit.name, self.pick.patch_names(self.ev))

    def test_the_slots_are_names_and_never_none(self):
        # The loud arm rides IN a slot (a `HitTestError` value), so a
        # slot is never `None` and a missing name is never silence.
        for entry in self.pick.patch_names(self.ev) + self.pick.boundary_names(
            self.ev
        ):
            self.assertNotIsInstance(entry, type(None))
            self.assertIsInstance(entry, (str, HitTestError))


class TestEnumeratingAWholeNode(unittest.TestCase):
    """`build_all`, and the states it keeps apart."""

    def test_a_solid_enumerates_to_one_body(self):
        doc = Doc()
        cube = unit_cube(doc)
        ev = evaluate(doc)
        picks = NodePick.build_all(ev, cube, DELTA)
        self.assertEqual([p.body for p in picks], [0])
        self.assertEqual(picks[0].node, cube)

    def test_a_split_enumerates_to_both_halves(self):
        # The case `build_all` exists for: a caller cannot ask a node
        # how many bodies it has, and the indices are the gather's, not
        # a dense range the caller may assume.
        doc = Doc()
        cube = unit_cube(doc)
        knife = doc.insert(
            Node.datum_plane((0.5 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0))
        )
        cut = doc.insert(Node.split(cube, knife))
        ev = evaluate(doc)
        picks = NodePick.build_all(ev, cut, DELTA)
        self.assertEqual(len(picks), 2)
        self.assertEqual(sorted(p.body for p in picks), [0, 1])
        # Each half is pickable on its own terms, and a ray finds the
        # half it passes through.
        left = ev.pick_face(picks, straight_down(x=0.25))
        right = ev.pick_face(picks, straight_down(x=0.75))
        self.assertEqual(left.node, cut)
        self.assertEqual(right.node, cut)
        self.assertNotEqual(left.body, right.body)


class TestTheIndexRefusesTyped(unittest.TestCase):
    """`NodePickError`, arm by arm."""

    def setUp(self):
        self.doc = Doc()
        self.cube = unit_cube(self.doc)
        self.ev = evaluate(self.doc)

    def refusal(self, call):
        with self.assertRaises(NodePickError) as caught:
            call()
        return caught.exception

    def test_a_node_that_never_draws_refuses_not_a_body(self):
        doc = Doc()
        datum = doc.insert(
            Node.datum_plane((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0))
        )
        ev = evaluate(doc)
        err = self.refusal(lambda: NodePick.build(ev, datum, 0, DELTA))
        self.assertEqual(err.variant, "not_a_body")
        self.assertEqual(err.node, datum)
        # `build_all` refuses the same way rather than answering empty:
        # "never draws" is not "draws nothing today".
        self.assertEqual(
            self.refusal(lambda: NodePick.build_all(ev, datum, DELTA)).variant,
            "not_a_body",
        )

    def test_a_body_index_the_value_does_not_have_refuses(self):
        err = self.refusal(lambda: NodePick.build(self.ev, self.cube, 3, DELTA))
        self.assertEqual(err.variant, "no_such_body")
        self.assertEqual(err.node, self.cube)
        self.assertEqual(err.body, 3)

    def test_a_node_this_evaluation_never_ran_refuses_the_standing_ladder(self):
        # Authored AFTER the evaluation was taken, so the run has no
        # result for it — the same word `ReadbackError` uses.
        later = unit_cube(self.doc, at=(5.0, 5.0))
        err = self.refusal(lambda: NodePick.build(self.ev, later, 0, DELTA))
        self.assertEqual(err.variant, "node_not_evaluated")
        self.assertEqual(err.node, later)

    def test_a_tessellation_refusal_arrives_under_the_tessellators_own_tag(self):
        # The forwarding rule: the chordal budget is the tessellator's
        # to judge, and its refusal crosses under its own word rather
        # than a wrapper's.
        err = self.refusal(lambda: NodePick.build(self.ev, self.cube, 0, 0 * m))
        self.assertEqual(err.variant, "invalid_chordal_tolerance")
        self.assertIsNone(err.node)

    def test_every_arm_carries_every_attribute(self):
        # `None` where an arm does not apply, never absent: a caller
        # reads the payload without first branching on `variant`.
        doc = Doc()
        datum = doc.insert(
            Node.datum_plane((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0))
        )
        ev = evaluate(doc)
        for call in (
            lambda: NodePick.build(ev, datum, 0, DELTA),
            lambda: NodePick.build(self.ev, self.cube, 7, DELTA),
            lambda: NodePick.build(self.ev, self.cube, 0, -1 * m),
        ):
            err = self.refusal(call)
            with self.subTest(variant=err.variant):
                for field in ("variant", "node", "through", "kind", "body"):
                    self.assertTrue(hasattr(err, field), field)

    def test_the_refusal_is_a_pncad_error(self):
        err = self.refusal(lambda: NodePick.build(self.ev, self.cube, 9, DELTA))
        self.assertIsInstance(err, PncadError)
        # The message is the kernel's own prose, not a Debug dump.
        self.assertNotIn(" { ", str(err))


class TestThePickRefusesTyped(unittest.TestCase):
    """`HitTestError` — the standing ladder, up front."""

    def test_a_target_from_another_run_refuses_rather_than_answering(self):
        # The staleness a pick consumer actually meets: an index built
        # against a later evaluation, offered to an earlier one. The
        # mesh cannot belong to this run, so the door refuses instead of
        # inverting against a table that is not there.
        doc = Doc()
        cube = unit_cube(doc)
        before = evaluate(doc)
        later_node = doc.insert(
            Node.transform(cube, (0 * m, 0 * m, 2 * m), (0.0, 0.0, 1.0), 0 * deg)
        )
        after = evaluate(doc)
        stale = NodePick.build(after, later_node, 0, DELTA)
        with self.assertRaises(HitTestError) as caught:
            before.pick_face([stale], straight_down(z=10.0))
        err = caught.exception
        self.assertEqual(err.variant, "node_not_evaluated")
        self.assertEqual(err.node, later_node)
        for field in ("variant", "node", "through", "kind", "body"):
            self.assertTrue(hasattr(err, field), field)

    def test_a_standing_refusal_is_never_flattened_into_a_miss(self):
        # The same call with a ray that hits nothing still REFUSES:
        # target standing is checked up front, before any geometry.
        doc = Doc()
        cube = unit_cube(doc)
        before = evaluate(doc)
        later_node = doc.insert(
            Node.transform(cube, (0 * m, 0 * m, 2 * m), (0.0, 0.0, 1.0), 0 * deg)
        )
        after = evaluate(doc)
        stale = NodePick.build(after, later_node, 0, DELTA)
        with self.assertRaises(HitTestError):
            before.pick_face([stale], straight_down(x=50.0))

    def test_the_two_classes_are_not_the_same_class(self):
        # Nothing to pick against, and a pick that could not answer, are
        # different stages and a caller must be able to catch one
        # without the other.
        self.assertIsNot(HitTestError, NodePickError)
        self.assertTrue(issubclass(HitTestError, PncadError))
        self.assertTrue(issubclass(NodePickError, PncadError))


class TestThePickedNameIsUsable(unittest.TestCase):
    """A pick is a selection: the text goes straight back into the
    document layer, unread."""

    def test_a_picked_face_narrows_a_selector_the_same_way(self):
        doc = Doc()
        cube = unit_cube(doc)
        ev = evaluate(doc)
        pick = NodePick.build(ev, cube, 0, DELTA)
        hit = ev.pick_face([pick], straight_down())
        # The selector door, asked for the same face by SHAPE, answers
        # the identical text — so a caller can start from either end.
        self.assertEqual(
            hit.name,
            one(
                ev.select(
                    cube,
                    Selector.of(
                        NamePat.of_kind(EntityKind.Face).seg(
                            SegPat.tag(SegTag.Cap).side(CapEnd.Top)
                        )
                    ),
                )
            ),
        )

    def test_a_picked_face_reads_back_where_it_sits(self):
        # Picking and read-back are two doors on one name: the face the
        # ray hit reads back as the plane the ray hit it on.
        doc = Doc()
        cube = unit_cube(doc)
        ev = evaluate(doc)
        pick = NodePick.build(ev, cube, 0, DELTA)
        hit = ev.pick_face([pick], straight_down())
        pose = ev.face_frame(cube, hit.name)
        self.assertAlmostEqual(pose.origin[2].meters, 1.0, places=12)
        self.assertAlmostEqual(abs(pose.axis[2]), 1.0, places=12)


class TestTheModuleExportsThem(unittest.TestCase):
    """The five names that crossed identically are on the module."""

    def test_the_picking_vocabulary_is_top_level(self):
        for name in (
            "Ray",
            "PickHit",
            "NodePick",
            "NodePickError",
            "HitTestError",
        ):
            with self.subTest(name=name):
                self.assertTrue(hasattr(pncad, name), name)


if __name__ == "__main__":
    unittest.main()
