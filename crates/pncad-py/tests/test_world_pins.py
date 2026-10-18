"""World pins, bundles and the loose copy, from Python.

A copy is placed by its placement's BUNDLE (D10, INTENT stage 3): a
mate equates a pose read off the copy with a pose of the space it
joins or of the world (`WorldRead`), and values set the freedoms it
leaves. `Doc.pin` is the "place part" gesture on a placement already
in the document: it pins the copy in the world at a pose; a mate
added to a bundle (`Doc.constrain`) places the copy that holds it on
its target; a constraint the rest of the bundle already fixes is the
placement's own refusal where it evaluates; and a copy whose bundle
pins less lives in a space of its own, which the product, the gate,
the pose door and STEP each refuse to compare with the world.

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
            resolver=self.ws,
        )

    def loose(self, ref):
        """An instance whose bundle is empty: a loose copy until a
        constraint pins it."""
        return Node.instantiate_part(ref, [])

    def pinned(self, doc, ref, pose=None):
        """An instance pinned in the world at `pose` by the gesture."""
        node = doc.insert(self.loose(ref))
        doc.pin(node, pose, resolver=self.ws)
        return node

    def test_a_pin_places_the_copy_at_its_pose(self):
        doc = Doc("py-pin-chain")
        post = doc.insert(Node.instantiate_part(self.post_ref))
        # An inserted instance with no bundle given is a loose copy
        # until something pins it.
        self.assertEqual(doc.bundle(post), [])
        self.assertEqual(evaluate(doc, resolver=self.ws).loose, [post])
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
        # A removed pin leaves the copy loose, never a refusal; an id the
        # bundle no longer holds is refused typed, naming its address.
        for value in placed.values:
            doc.apply(DocEdit.remove_constraint(value))
        doc.apply(DocEdit.remove_constraint(placed.mate))
        self.assertEqual(doc.bundle(post), [])
        self.assertEqual(evaluate(doc, resolver=self.ws).loose, [post])
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.remove_constraint(placed.mate))
        self.assertEqual(
            (caught.exception.variant, caught.exception.node, caught.exception.mate),
            ("unknown_constraint", post, placed.mate),
        )

    def test_remove_mate_takes_the_mate_and_the_values_naming_it_as_one_action(self):
        doc = Doc("py-remove-mate")
        post = self.pinned(doc, self.post_ref, lifted(1.0))
        shelf = doc.insert(self.loose(self.shelf_ref))
        seat, values = self.seat(doc, post, shelf)
        pin = doc.bundle(post)[0][0]
        # The door answers the values it took, in bundle order, and
        # leaves the copy loose: the mate went with them.
        self.assertEqual(doc.remove_mate(seat), values)
        self.assertEqual(doc.bundle(shelf), [])
        self.assertEqual(evaluate(doc, resolver=self.ws).loose, [shelf])
        # The other copy's bundle is untouched.
        self.assertEqual(next(ref for ref, _ in doc.bundle(post)), pin)
        # A mate no value names goes alone.
        bare = doc.constrain(
            shelf,
            Mate(
                PoseDef.plane(shelf, self.face(doc, shelf, CapEnd.Start)),
                PoseDef.flip(PoseDef.plane(post, self.face(doc, post, CapEnd.End))),
                ContactClass.Rest,
            ),
            resolver=self.ws,
        )
        self.assertEqual(doc.remove_mate(bare), [])
        self.assertEqual(doc.bundle(shelf), [])
        # An id the bundle no longer holds is refused, and touches
        # nothing.
        held = doc.bundle(post)
        with self.assertRaises(EditError) as caught:
            doc.remove_mate(bare)
        self.assertEqual(caught.exception.variant, "unknown_constraint")
        self.assertEqual(doc.bundle(post), held)
        # A value's address removes that value alone: it names no mate,
        # so nothing goes with it.
        value = held[1][0]
        self.assertEqual(doc.remove_mate(value), [])
        self.assertEqual(doc.bundle(post), [held[0], *held[2:]])
        # The world is no placement.
        with self.assertRaises(EditError) as caught:
            doc.remove_mate(MateRef(doc.world, pin.constraint))
        self.assertEqual(caught.exception.variant, "constraint_on_non_placement")

    def test_a_mate_places_the_copy_whose_bundle_holds_it(self):
        doc = Doc("py-pin-mate")
        post = self.pinned(doc, self.post_ref)
        shelf = doc.insert(self.loose(self.shelf_ref))
        self.assertEqual(evaluate(doc, resolver=self.ws).loose, [shelf])
        joint, values = self.seat(doc, post, shelf)
        self.assertEqual(joint.placement, shelf)
        self.assertEqual([ref for ref, _ in doc.bundle(shelf)], [joint, *values])
        self.assertEqual(doc.last_maintenance, [], "a value clears nothing")
        ev = evaluate(doc, resolver=self.ws)
        self.assertEqual(ev.loose, [])
        self.assertEqual(ev.space(shelf), "world")
        solved = solve_document(doc, resolver=self.ws)
        self.assertIsNone(solved.fault(shelf))
        for got, want in zip(
            solved.pose(shelf).origin,
            solved.placement(doc, shelf).origin,
            strict=True,
        ):
            # The post is pinned at the identity, so its space is the
            # world's.
            self.assertAlmostEqual(got.meters, want.meters, delta=1e-12)

    def test_a_loose_copy_lives_in_its_own_space(self):
        doc = Doc("py-pin-loose")
        post = self.pinned(doc, self.post_ref)
        shelf = doc.insert(self.loose(self.shelf_ref))
        solved = solve_document(doc, resolver=self.ws)
        self.assertIsNone(solved.fault(shelf), "a loose copy is not a refusal")
        with self.assertRaises(pncad.EvaluationError) as caught:
            solved.placement(doc, shelf)
        self.assertEqual(caught.exception.kind, "space_mismatch")
        ev = evaluate(doc, resolver=self.ws)
        self.assertEqual(ev.loose, [shelf])
        self.assertEqual(ev.space(shelf), shelf)
        self.assertEqual(ev.space(post), "world")
        with self.assertRaises(pncad.ExportError) as caught:
            ev.step_string(shelf)
        self.assertEqual(caught.exception.variant, "loose")
        self.assertEqual(caught.exception.copies, [shelf])

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
        cs = control.insert(self.loose(self.shelf_ref))
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
        shelf = doc.insert(self.loose(self.shelf_ref))
        self.seat(doc, post, shelf)
        poses = solve_document(doc, resolver=self.ws)
        self.assertLess(gap(mat(poses.placement(doc, post)), w_post), 1e-12)
        self.assertLess(gap(mat(poses.placement(doc, shelf)), mul(w_post, rel)), 1e-12)
        # A copy pinned, transitively, against the world is posed in
        # the world's coordinates.
        self.assertLess(gap(mat(poses.pose(shelf)), mul(w_post, rel)), 1e-12)

    def test_a_constraint_the_bundle_already_fixes_is_refused(self):
        """A post rests on the world's floor by its underside; a second
        plane mate, a side on one of the world's walls, fixes a normal
        the first already fixes, so the door refuses it, measuring
        nothing, naming both and the freedoms the two would leave."""
        doc = Doc("py-pin-overconstrained")
        post = doc.insert(self.loose(self.post_ref))

        def facing(normal):
            return PoseDef.plane(post, bench_scene.face_facing(self.ws, doc, post, normal))

        floor, wall_x = (WorldRead.through(role).flip() for role in ("xy", "yz"))
        bottom = doc.constrain(post, Mate(facing((0.0, 0.0, -1.0)), floor, ContactClass.Rest))
        with self.assertRaises(pncad.EditError) as caught:
            doc.constrain(post, Mate(facing((-1.0, 0.0, 0.0)), wall_x, ContactClass.Rest))
        fault = caught.exception.fault
        self.assertEqual(fault.variant, "mate_overconstrained")
        self.assertEqual(fault.placement, post)
        self.assertIn(bottom, fault.constraints)
        self.assertEqual(len(fault.constraints), 2)
        self.assertEqual(fault.free, 1)
        self.assertIn(pncad.OVERCONSTRAINED_RECOURSE, str(fault))

    def test_the_gate_checks_a_loose_copys_own_space(self):
        """`pncad.assemble` reaches the kernel's one gate, which checks
        a loose copy's space by itself: two posts seated at one spot
        under the shelf interpenetrate there, and the gate refuses
        though the world beside them certifies."""
        doc = Doc("py-pin-gate")
        p1 = doc.insert(self.loose(self.post_ref))
        shelf = doc.insert(self.loose(self.shelf_ref))
        p2 = doc.insert(self.loose(self.post_ref))
        self.seat(doc, p1, shelf)
        # The second post under the shelf at the first's seat.
        bench_scene.rest(
            doc,
            p2,
            (p2, self.face(doc, p2, CapEnd.End)),
            (shelf, self.face(doc, shelf, CapEnd.Start)),
            bench_scene.post_under_shelf(bench_scene.SEAT_A),
            resolver=self.ws,
        )
        self.pinned(doc, self.post_ref, Frame.translation((1 * m, 0 * m, 0 * m)))
        ev = evaluate(doc, resolver=self.ws)
        self.assertEqual(ev.loose, [p1])
        self.assertEqual((ev.space(shelf), ev.space(p2)), (p1, p1))
        with self.assertRaises(pncad.AssemblyError):
            pncad.assemble(doc, ev)

    def test_the_doors_work_off_the_main_thread(self):
        """Authoring, solving and reading on a worker thread answers
        what the main thread does."""

        def author():
            doc = Doc("py-pin-thread")
            post = self.pinned(doc, self.post_ref, lifted(3.0))
            shelf = doc.insert(self.loose(self.shelf_ref))
            self.seat(doc, post, shelf)
            solved = solve_document(doc, resolver=self.ws)
            return (
                len(doc.bundle(shelf)),
                [r.variant for r in doc.last_maintenance],
                evaluate(doc, resolver=self.ws).loose,
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
        self.assertEqual(said["worker"][:3], (4, [], []))


if __name__ == "__main__":
    unittest.main()
