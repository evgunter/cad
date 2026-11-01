"""World pins, bundles and target spaces, from Python.

A copy is placed into its placement's TARGET space by its BUNDLE (D10,
INTENT stage 3): a mate equates a pose read off the copy with a pose of
the target space, read off geometry there, and values on the target
space's axes set the freedoms the mates leave. Every bundle pins its
copy and is written whole (`Doc.set_constraints`, `Placed`). A copy is
pinned in the world at a pose by the values that put it there; a
constraint the rest of the bundle already fixes is refused at the
door; a constraint taken away is written zero again, so the copy stays
pinned; and a copy placed into a space other than the world has no
world pose.

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
    Coord,
    Doc,
    DocEdit,
    EditError,
    Frame,
    Mate,
    MateRef,
    MateValue,
    Node,
    Placed,
    PoseDef,
    Workspace,
    evaluate,
    m,
    rad,
    solve_document,
)

import bench_scene
from placing import moves, assemble_world_copies


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
        """An instance pinned in the world at `pose` (the identity when
        `None`) by the values that put it there."""
        node = doc.insert(self.stood(doc, ref))
        if pose is not None:
            doc.set_constraints(node, Placed.unmated(moves(doc, pose)))
        return node

    def test_a_pin_places_the_copy_at_its_pose(self):
        doc = Doc("py-pin-chain")
        post = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
        # An inserted instance with no bundle given stands by the
        # identity bundle: no mate, six values.
        self.assertEqual(len(doc.bundle(post)), 6)
        pin = Placed.unmated([MateValue(Coord.slide("x"), 5 * m), MateValue(Coord.slide("z"), 2 * m)])
        written = doc.set_constraints(post, pin)
        bundle = doc.bundle(post)
        self.assertEqual([ref for ref, _ in bundle], written)
        # No mate: the values state the copy's whole motion on the
        # world's axes, the author's first, then a zero on each freedom
        # left unnamed.
        self.assertEqual(
            [held.coord.variant for _, held in bundle],
            ["slide_x", "slide_z", "slide_y", "spin_x", "spin_y", "spin_z"],
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
        # Every bundle pins its copy: a value taken away is written
        # zero again, so the copy keeps its place on the other axes.
        doc.apply(DocEdit.remove_constraint(written[0]))
        at = solve_document(doc, resolver=self.ws).placement(doc, post).origin
        for got, want in zip(at, (0.0, 0.0, 2.0), strict=True):
            self.assertAlmostEqual(got.meters, want, delta=1e-12)
        # An id the bundle no longer holds is refused typed, naming its
        # address.
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.remove_constraint(written[0]))
        self.assertEqual(
            (caught.exception.variant, caught.exception.node, caught.exception.mate),
            ("unknown_constraint", post, written[0]),
        )

    def test_remove_mate_keeps_the_copy_pinned(self):
        """`Doc.remove_mate` writes the bundle whole again without the
        mate: the values the bundle then leaves are kept, each freedom
        no value sets is written zero, and the copy stays pinned. An id
        the bundle does not hold, and the world, refuse in their own
        words."""
        doc = Doc("py-remove-mate")
        post = self.pinned(doc, self.post_ref, lifted(1.0))
        shelf = doc.insert(self.stood(doc, self.shelf_ref))
        seat, _ = self.seat(doc, post, shelf)
        pin = doc.bundle(post)[0][0]
        written = doc.remove_mate(seat)
        held = doc.bundle(shelf)
        self.assertFalse(any(isinstance(c, Mate) for _, c in held))
        self.assertEqual(len(held), 6)
        self.assertTrue(set(written) <= {ref for ref, _ in held})
        self.assertIsNone(solve_document(doc, resolver=self.ws).fault(shelf))
        # An id the bundle no longer holds is refused, and touches
        # nothing.
        with self.assertRaises(EditError) as caught:
            doc.remove_mate(seat)
        self.assertEqual(caught.exception.variant, "unknown_constraint")
        self.assertEqual(doc.bundle(shelf), held)
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

    def test_the_at_rest_check_reads_each_assemble_by_itself(self):
        """The at-rest check reads each `⊔`'s census between its own
        members, whether or not the product reads it: two posts seated
        at one spot under a shelf in a space of their own, gathered by a
        `⊔` there, interpenetrate, and the check reports their overlap
        as one loud interference finding under that `⊔`, though the
        world's product beside them has nothing to report."""
        doc = Doc("py-pin-gate")
        self.pinned(doc, self.post_ref, Frame.translation((1 * m, 0 * m, 0 * m)))
        assemble_world_copies(doc)
        aside = doc.new_space()
        shelf = doc.insert(Node.instantiate_part(self.shelf_ref, aside))
        posts = []
        for _ in range(2):
            post = doc.insert(Node.instantiate_part(self.post_ref, aside))
            bench_scene.rest(
                doc,
                post,
                (post, self.face(doc, post, CapEnd.End)),
                (shelf, self.face(doc, shelf, CapEnd.Start)),
                bench_scene.post_under_shelf(bench_scene.SEAT_A),
            )
            posts.append(post)
        self.assertEqual({doc.space(p) for p in posts}, {doc.output(aside)})
        # One post under the shelf: nothing clashes anywhere.
        def at_rest():
            report = pncad.run_checks(doc, evaluate(doc, resolver=self.ws))
            return [f for f in report.findings if f.check == pncad.CheckId.AtRest]

        doc.assemble([shelf, posts[0]])
        self.assertEqual(at_rest(), [])
        clash = doc.assemble([shelf, *posts])
        (finding,) = at_rest()
        self.assertEqual(finding.census, doc.output(clash), "the `⊔` that decided it")
        self.assertEqual(finding.evidence.variant, "interference")
        self.assertTrue(finding.evidence.interference.loud)

    def test_a_seat_charts_on_the_space_not_on_the_body_it_rests_on(self):
        """Against an independent 4x4 composition: a post pinned to the
        world at a turned pose, and the shelf seated on its top. The
        seat's values chart on the world's axes carried onto the mated
        normal by the least turn, never on the post's own frame, so turning and sliding
        the post moves the shelf only along that normal — by the
        post's rise — and the shelf's pose is the seat it has on a post
        standing at the identity, lifted."""

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
        lifted_rel = mul(rz(0.0, (0, 0, w_post[2][3])), rel)
        self.assertLess(gap(mat(poses.placement(doc, shelf)), lifted_rel), 1e-12)
        # A copy placed into the world is posed in the world's
        # coordinates.
        self.assertLess(gap(mat(poses.pose(shelf)), lifted_rel), 1e-12)

    def test_a_constraint_the_bundle_already_fixes_is_refused(self):
        """A post rests on a base's top by its underside; a second
        plane mate, a side on one of the base's sides, fixes a normal
        the first already fixes, so the door refuses the bundle,
        measuring nothing, naming both and the freedoms the two would
        leave."""
        doc = Doc("py-pin-overconstrained")
        post = doc.insert(self.stood(doc, self.post_ref))
        base = self.pinned(doc, self.post_ref)

        def facing(node, normal):
            return PoseDef.plane(node, bench_scene.face_facing(self.ws, doc, node, normal))

        floor = PoseDef.flip(facing(base, (0.0, 0.0, 1.0)))
        wall_x = PoseDef.flip(facing(base, (1.0, 0.0, 0.0)))
        with self.assertRaises(pncad.EditError) as caught:
            doc.set_constraints(
                post,
                [
                    Mate(facing(post, (0.0, 0.0, -1.0)), floor),
                    Mate(facing(post, (-1.0, 0.0, 0.0)), wall_x),
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
