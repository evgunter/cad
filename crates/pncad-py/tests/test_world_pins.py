"""World pins, bundles and target spaces, from Python.

A copy is placed into its placement's TARGET space by its BUNDLE (D10,
INTENT stage 3): a mate equates a pose read off the copy with a pose of
the target space or of the world (`WorldRead`), and values set the
freedoms it leaves. Every bundle pins its copy and is written whole
(`Doc.set_constraints`). `Doc.pin` is the "place part" gesture on a
placement already in the document: it writes the bundle that pins the
copy in the world at a pose; a constraint the rest of the bundle
already fixes, and a bundle that leaves a freedom unwritten, are
refused at the door; and a copy placed into a space other than the
world has no world pose.

The doors are exercised on a worker `threading.Thread` too: a caller
that authors and solves off the main thread reads the same document
and the same poses.
"""

import math
import shutil
import tempfile
import threading
import unittest

import pncad
from pncad import (
    CapEnd,
    ContactClass,
    Doc,
    DocEdit,
    EditError,
    Frame,
    Mate,
    MateRef,
    Node,
    PoseDef,
    Workspace,
    WorldRead,
    evaluate,
    m,
    rad,
    solve_document,
)

import bench_scene


def lifted(z):
    return Frame.translation((0 * m, 0 * m, z * m))


class WorldPins(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, self.dir, True)
        self.ws = Workspace(self.dir)
        (_, self.post_ref), (_, self.shelf_ref) = bench_scene.parts(self.ws)

    def face(self, doc, node, side):
        return bench_scene.instance_face(self.ws, doc, node, side)

    def seat(self, doc, post, shelf):
        """"Mate the shelf to the post": the shelf's underside resting
        on the post's top where the stand's first seat puts it — a
        constraint of the shelf's bundle, with its values. Answers the
        mate's address and its values'."""
        return bench_scene.rest(
            doc,
            shelf,
            (shelf, self.face(doc, shelf, CapEnd.Start)),
            (post, self.face(doc, post, CapEnd.End)),
            bench_scene.SHELF_ON_POST,
        )

    def stood(self, doc, ref):
        """An instance placed into `doc`'s world by the identity bundle,
        standing where its part's world stands until a bundle written in
        its place moves it."""
        return Node.instantiate_part(ref, doc.world)

    def pinned(self, doc, ref, pose=None):
        """An instance pinned in the world at `pose` by the gesture."""
        node = doc.insert(self.stood(doc, ref))
        doc.pin(node, pose, resolver=self.ws)
        return node

    def test_a_pin_places_the_copy_at_its_pose(self):
        doc = Doc("py-pin-chain")
        post = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
        # An inserted instance with no bundle given stands by the
        # identity bundle: no mate, six values.
        self.assertEqual(len(doc.bundle(post)), 6)
        placed = doc.pin(post, Frame.translation((5 * m, 0 * m, 2 * m)), resolver=self.ws)
        self.assertEqual((placed.placement, placed.mate.placement), (post, post))
        self.assertEqual(placed.by, "planar_face")
        bundle = doc.bundle(post)
        self.assertEqual([ref for ref, _ in bundle], [placed.mate, *placed.values])
        # The mate's target reads the world, inside the mate; each
        # value sets one freedom it leaves.
        self.assertIsInstance(bundle[0][1].to, WorldRead)
        self.assertEqual(bundle[0][1].to.kind, "plane")
        self.assertEqual(
            [held.coord.variant for _, held in bundle[1:]], ["slide_x", "slide_y", "spin"]
        )
        self.assertEqual(doc.node_kind(doc.world), "world")
        self.assertNotIn(doc.world, doc.order(), "the world is not counted among the nodes")
        self.assertEqual(doc.node_count, len(doc.order()))
        self.assertNotIn(
            doc.world, evaluate(doc, resolver=self.ws).order(), "the world is scheduled nowhere"
        )
        at = solve_document(doc, resolver=self.ws).placement(doc, post).origin
        for got, want in zip(at, (5.0, 0.0, 2.0), strict=True):
            self.assertAlmostEqual(got.meters, want, delta=1e-12)
        # The world is no placement, and no edit deletes it.
        with self.assertRaises(EditError) as caught:
            doc.bundle(doc.world)
        self.assertEqual(caught.exception.variant, "constraint_on_non_placement")
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.delete_node(doc.world))
        self.assertEqual(caught.exception.variant, "world_is_undeletable")
        # A mate a value names is not removed from under it: the value
        # goes first.
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.remove_constraint(placed.mate))
        self.assertEqual(caught.exception.variant, "mate_named_by_value")
        # Every bundle pins its copy: a value removed would leave its
        # freedom unwritten, and is refused.
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.remove_constraint(placed.values[0]))
        self.assertEqual(caught.exception.inner_variant, "mate_unpinned")
        # A pin written again replaces the bundle whole; an id the
        # bundle no longer holds is refused typed, naming its address.
        doc.pin(post, resolver=self.ws)
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.remove_constraint(placed.mate))
        self.assertEqual(
            (caught.exception.variant, caught.exception.node, caught.exception.mate),
            ("unknown_constraint", post, placed.mate),
        )

    def test_remove_mate_refuses_what_would_leave_a_freedom_unwritten(self):
        """`Doc.remove_mate` takes a mate and the values naming it as one
        action; every bundle pins its copy, so taking them refuses and
        touches nothing, and so does a value taken alone. An id the
        bundle does not hold, and the world, refuse in their own
        words."""
        doc = Doc("py-remove-mate")
        post = self.pinned(doc, self.post_ref, lifted(1.0))
        shelf = doc.insert(self.stood(doc, self.shelf_ref))
        seat, values = self.seat(doc, post, shelf)
        pin = doc.bundle(post)[0][0]
        held = doc.bundle(shelf)
        for taken in (seat, values[0]):
            with self.assertRaises(EditError) as caught:
                doc.remove_mate(taken)
            self.assertEqual(caught.exception.inner_variant, "mate_unpinned")
            self.assertEqual(doc.bundle(shelf), held)
        # An id the bundle no longer holds is refused, and touches
        # nothing.
        self.seat(doc, post, shelf)
        with self.assertRaises(EditError) as caught:
            doc.remove_mate(seat)
        self.assertEqual(caught.exception.variant, "unknown_constraint")
        # The world is no placement.
        with self.assertRaises(EditError) as caught:
            doc.remove_mate(MateRef(doc.world, pin.constraint))
        self.assertEqual(caught.exception.variant, "constraint_on_non_placement")

    def test_a_mate_places_the_copy_whose_bundle_holds_it(self):
        doc = Doc("py-pin-mate")
        post = self.pinned(doc, self.post_ref)
        shelf = doc.insert(self.stood(doc, self.shelf_ref))
        joint, values = self.seat(doc, post, shelf)
        self.assertEqual(joint.placement, shelf)
        self.assertEqual([ref for ref, _ in doc.bundle(shelf)], [joint, *values])
        # The seat is written over the identity bundle, whose six zeros
        # were anonymous variables read by nothing else: each retires.
        self.assertEqual(
            [r.variant for r in doc.last_maintenance], ["anonymous_var_removed"] * 6
        )
        self.assertEqual(doc.space(shelf), doc.output(doc.world))
        solved = solve_document(doc, resolver=self.ws)
        self.assertIsNone(solved.fault(shelf))
        for got, want in zip(
            solved.pose(shelf).origin,
            solved.placement(doc, shelf).origin,
            strict=True,
        ):
            # Placed into the world, so its pose is in the world's
            # coordinates.
            self.assertAlmostEqual(got.meters, want.meters, delta=1e-12)

    def test_a_copy_placed_into_another_space_has_no_world_pose(self):
        doc = Doc("py-pin-aside")
        post = self.pinned(doc, self.post_ref)
        aside = doc.new_space()
        shelf = doc.insert(Node.instantiate_part(self.shelf_ref, aside))
        solved = solve_document(doc, resolver=self.ws)
        self.assertIsNone(solved.fault(shelf), "a copy in another space is not a refusal")
        with self.assertRaises(pncad.EvaluationError) as caught:
            solved.placement(doc, shelf)
        self.assertEqual(caught.exception.kind, "space_mismatch")
        self.assertEqual(doc.space(shelf), doc.output(aside))
        self.assertEqual(doc.space(post), doc.output(doc.world))

    def test_a_world_pose_is_the_pin_onto_the_solve(self):
        """`P ∘ B` through Python, against an independent 4x4
        composition: a post pinned to the world at a turned pose, and
        the shelf seated on it."""

        def mat(f):
            c = f.columns
            o = [x.meters for x in f.origin]
            return [[c[0][i], c[1][i], c[2][i], o[i]] for i in range(3)] + [[0, 0, 0, 1]]

        def mul(a, b):
            return [[sum(a[i][k] * b[k][j] for k in range(4)) for j in range(4)] for i in range(4)]

        def rz(t, v):
            c, s = math.cos(t), math.sin(t)
            return [[c, -s, 0, v[0]], [s, c, 0, v[1]], [0, 0, 1, v[2]], [0, 0, 0, 1]]

        def gap(a, b):
            return max(abs(a[i][j] - b[i][j]) for i in range(3) for j in range(4))

        control = Doc("py-pb-control")
        cp = self.pinned(control, self.post_ref)
        cs = control.insert(self.stood(control, self.shelf_ref))
        self.seat(control, cp, cs)
        rel = mat(solve_document(control, resolver=self.ws).placement(control, cs))

        w_post = mul(mul(rz(0.25, (0, 0, 2)), rz(0.0, (5, 0, 0))), rz(0.3, (1, 2, 0)))
        turned = Frame.rotate_then_translate(
            (0.0, 0.0, 1.0),
            0.55 * rad,
            tuple(w_post[i][3] * m for i in range(3)),
        )
        doc = Doc("py-pb")
        post = self.pinned(doc, self.post_ref, turned)
        shelf = doc.insert(self.stood(doc, self.shelf_ref))
        self.seat(doc, post, shelf)
        poses = solve_document(doc, resolver=self.ws)
        self.assertLess(gap(mat(poses.placement(doc, post)), w_post), 1e-12)
        self.assertLess(gap(mat(poses.placement(doc, shelf)), mul(w_post, rel)), 1e-12)
        # A copy placed into the world is posed in the world's
        # coordinates.
        self.assertLess(gap(mat(poses.pose(shelf)), mul(w_post, rel)), 1e-12)

    def test_a_constraint_the_bundle_already_fixes_is_refused(self):
        """A post rests on the world's floor by its underside; a second
        plane mate, a side on one of the world's walls, fixes a normal
        the first already fixes, so the door refuses the bundle,
        measuring nothing, naming both and the freedoms the two would
        leave."""
        doc = Doc("py-pin-overconstrained")
        post = doc.insert(self.stood(doc, self.post_ref))

        def facing(normal):
            return PoseDef.plane(post, bench_scene.face_facing(self.ws, doc, post, normal))

        floor, wall_x = (WorldRead.through(role).flip() for role in ("xy", "yz"))
        with self.assertRaises(pncad.EditError) as caught:
            doc.set_constraints(
                post,
                [
                    Mate(facing((0.0, 0.0, -1.0)), floor, ContactClass.Rest),
                    Mate(facing((-1.0, 0.0, 0.0)), wall_x, ContactClass.Rest),
                ],
            )
        fault = caught.exception.fault
        self.assertEqual(fault.variant, "mate_overconstrained")
        self.assertEqual(fault.placement, post)
        self.assertEqual(len(fault.constraints), 2)
        self.assertEqual(fault.free, 1)
        self.assertIn(pncad.OVERCONSTRAINED_RECOURSE, str(fault))

    def test_the_doors_work_off_the_main_thread(self):
        """Authoring, solving and reading on a worker thread answers
        what the main thread does."""

        def author():
            doc = Doc("py-pin-thread")
            post = self.pinned(doc, self.post_ref, lifted(3.0))
            shelf = doc.insert(self.stood(doc, self.shelf_ref))
            self.seat(doc, post, shelf)
            solved = solve_document(doc, resolver=self.ws)
            return (
                len(doc.bundle(shelf)),
                [r.variant for r in doc.last_maintenance],
                doc.space(shelf) == doc.output(doc.world),
                tuple(c.meters for c in solved.placement(doc, shelf).origin),
            )

        said = {}

        def worker():
            said["worker"] = author()

        thread = threading.Thread(target=worker)
        thread.start()
        thread.join()
        self.assertEqual(said["worker"], author())
        # The shelf's bundle is its mate and its three values.
        self.assertEqual(said["worker"][:3], (4, ["anonymous_var_removed"] * 6, True))


if __name__ == "__main__":
    unittest.main()
