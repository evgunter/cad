"""World pins, bundles and the loose copy, from Python.

A copy is placed by its placement's BUNDLE (D10, INTENT stage 3): an
instance pinned to the world at a pose (`Mate.world_pin`) stands there,
its pose a `Placement` chain; a mate added to a bundle (`Doc.constrain`)
places the copy that holds it on its target; a constraint the rest of
the bundle already fixes is refused at the door; and a copy whose
bundle pins less lives in a space of its own, which the product, the
gate, the pose door and STEP each refuse to compare with the world.

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
    Node,
    Placement,
    Workspace,
    evaluate,
    m,
    rad,
    solve_document,
)

import bench_scene


def lifted(z):
    return Placement.literal(Frame.translation((0 * m, 0 * m, z * m)))


class WorldPins(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, self.dir, True)
        self.ws = Workspace(self.dir)
        (_, self.post_ref), (_, self.shelf_ref) = bench_scene.parts(self.ws)

    def face(self, doc, node, side):
        return bench_scene.instance_face(self.ws, doc, node, side)

    def seat(self, doc, post, shelf):
        """"Mate the shelf to the post": the shelf's underside on the
        post's top — the stand's own first seat, a constraint of the
        shelf's bundle."""
        return Mate(
            shelf,
            self.face(doc, shelf, CapEnd.Start),
            post,
            self.face(doc, post, CapEnd.End),
            ContactClass.Rest,
            bench_scene.seat(*bench_scene.STAND_SEATS[0]),
        )

    def loose(self, ref):
        """An instance whose bundle is empty: a loose copy until a
        constraint pins it."""
        return Node.instantiate_part(ref, [])

    def test_a_world_pin_places_by_its_composed_chain(self):
        doc = Doc("py-pin-chain")
        chain = lifted(2.0).compose(
            Placement.literal(Frame.translation((5 * m, 0 * m, 0 * m)))
        )
        post = doc.insert(Node.instantiate_part(self.post_ref, [Mate.world_pin(chain)]))
        [(pin, held)] = doc.bundle(post)
        self.assertEqual(pin.placement, post)
        self.assertEqual((held.a, held.b), ("world", "copy"))
        self.assertEqual(held, Mate.world_pin(chain))
        self.assertEqual(doc.node_kind(doc.world), "world")
        self.assertNotIn(doc.world, doc.order(), "the world is not counted among the nodes")
        self.assertEqual(doc.node_count, len(doc.order()))
        self.assertNotIn(
            doc.world, evaluate(doc, resolver=self.ws).order(), "the world is scheduled nowhere"
        )
        at = solve_document(doc, resolver=self.ws).placement(doc, post).origin
        self.assertEqual(tuple(c.meters for c in at), (5.0, 0.0, 2.0))
        # An inserted instance with no bundle given stands where its
        # part's world does: pinned to the world at the identity.
        origin = doc.insert(Node.instantiate_part(self.post_ref))
        [(_, default)] = doc.bundle(origin)
        self.assertEqual(default, Mate.world_pin())
        # The world is no placement, and no edit deletes it.
        with self.assertRaises(EditError) as caught:
            doc.bundle(doc.world)
        self.assertEqual(caught.exception.variant, "constraint_on_non_placement")
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.delete_node(doc.world))
        self.assertEqual(caught.exception.variant, "world_is_undeletable")
        # A removed pin leaves the copy loose, never a refusal; an id the
        # bundle no longer holds is refused typed, naming its address.
        doc.apply(DocEdit.remove_constraint(pin))
        self.assertEqual(doc.bundle(post), [])
        self.assertEqual(evaluate(doc, resolver=self.ws).loose, [post])
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.remove_constraint(pin))
        self.assertEqual(
            (caught.exception.variant, caught.exception.node, caught.exception.mate),
            ("unknown_constraint", post, pin),
        )

    def test_a_mate_places_the_copy_whose_bundle_holds_it(self):
        doc = Doc("py-pin-mate")
        post = doc.insert(Node.instantiate_part(self.post_ref))
        shelf = doc.insert(self.loose(self.shelf_ref))
        self.assertEqual(evaluate(doc, resolver=self.ws).loose, [shelf])
        joint = doc.constrain(shelf, self.seat(doc, post, shelf), resolver=self.ws)
        self.assertEqual(joint.placement, shelf)
        self.assertEqual([ref for ref, _ in doc.bundle(shelf)], [joint])
        self.assertEqual(doc.last_maintenance, [], "a mate clears nothing")
        ev = evaluate(doc, resolver=self.ws)
        self.assertEqual(ev.loose, [])
        self.assertEqual(ev.space(shelf), "world")
        solved = solve_document(doc, resolver=self.ws)
        self.assertIsNone(solved.fault(shelf))
        self.assertEqual(
            solved.pose(shelf),
            solved.placement(doc, shelf),
            "the post is pinned at the identity, so its space is the world's",
        )

    def test_a_loose_copy_lives_in_its_own_space(self):
        doc = Doc("py-pin-loose")
        post = doc.insert(Node.instantiate_part(self.post_ref))
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

    def test_a_world_pose_is_the_pin_chain_onto_the_solve(self):
        """`P ∘ B` through Python, against an independent 4x4
        composition: a post pinned to the world by a turned chain, and
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
        cp = control.insert(Node.instantiate_part(self.post_ref))
        cs = control.insert(self.loose(self.shelf_ref))
        control.constrain(cs, self.seat(control, cp, cs), resolver=self.ws)
        rel = mat(solve_document(control, resolver=self.ws).placement(control, cs))

        def turned(t, v):
            return Placement.literal(
                Frame.rotate_then_translate(
                    (0.0, 0.0, 1.0), t * rad, (v[0] * m, v[1] * m, v[2] * m)
                )
            )

        chain = turned(0.25, (0, 0, 2)).compose(turned(0.0, (5, 0, 0))).compose(
            turned(0.3, (1, 2, 0))
        )
        doc = Doc("py-pb")
        post = doc.insert(Node.instantiate_part(self.post_ref, [Mate.world_pin(chain)]))
        shelf = doc.insert(self.loose(self.shelf_ref))
        doc.constrain(shelf, self.seat(doc, post, shelf), resolver=self.ws)
        poses = solve_document(doc, resolver=self.ws)
        w_post = mul(mul(rz(0.25, (0, 0, 2)), rz(0.0, (5, 0, 0))), rz(0.3, (1, 2, 0)))
        self.assertLess(gap(mat(poses.placement(doc, post)), w_post), 1e-12)
        self.assertLess(gap(mat(poses.placement(doc, shelf)), mul(w_post, rel)), 1e-12)
        self.assertLess(gap(mat(poses.pose(shelf)), rel), 1e-12, "the pose is in the post's space")

    def test_a_constraint_the_bundle_already_fixes_is_refused(self):
        doc = Doc("py-pin-overconstrained")
        post = doc.insert(Node.instantiate_part(self.post_ref))
        shelf = doc.insert(self.loose(self.shelf_ref))
        held = doc.constrain(shelf, self.seat(doc, post, shelf), resolver=self.ws)
        with self.assertRaises(EditError) as caught:
            doc.constrain(shelf, self.seat(doc, post, shelf), resolver=self.ws)
        err = caught.exception
        self.assertEqual((err.variant, err.inner_variant), ("mate_refused", "mate_overconstrained"))
        # The fault names the placement, and each constraint whose
        # removal leaves the bundle as fixed as it was: the one added.
        self.assertEqual(err.fault.placement, shelf)
        self.assertTrue(err.fault.constraints)
        for named in err.fault.constraints:
            self.assertEqual(named.placement, shelf)
            self.assertNotEqual(named, held)
        self.assertIn(pncad.OVERCONSTRAINED_RECOURSE, str(err))
        self.assertEqual(
            [ref for ref, _ in doc.bundle(shelf)], [held], "the refused edit leaves the bundle"
        )

    def test_the_gate_checks_a_loose_copys_own_space(self):
        """`pncad.assemble` reaches the kernel's one gate, which checks
        a loose copy's space by itself: two posts seated at one spot
        under the shelf interpenetrate there, and the gate refuses
        though the world beside them certifies."""
        doc = Doc("py-pin-gate")
        p1 = doc.insert(self.loose(self.post_ref))
        shelf = doc.insert(self.loose(self.shelf_ref))
        p2 = doc.insert(self.loose(self.post_ref))
        seat_a = bench_scene.STAND_SEATS[0]
        doc.constrain(shelf, self.seat(doc, p1, shelf), resolver=self.ws)
        doc.constrain(
            p2,
            Mate(
                p2,
                self.face(doc, p2, CapEnd.End),
                shelf,
                self.face(doc, shelf, CapEnd.Start),
                ContactClass.Rest,
                bench_scene.seat(seat_a[1], seat_a[0]),
            ),
            resolver=self.ws,
        )
        doc.insert(
            Node.instantiate_part(
                self.post_ref,
                [Mate.world_pin(Placement.literal(Frame.translation((1 * m, 0 * m, 0 * m))))],
            )
        )
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
            post = doc.insert(
                Node.instantiate_part(self.post_ref, [Mate.world_pin(lifted(3.0))])
            )
            shelf = doc.insert(self.loose(self.shelf_ref))
            doc.constrain(shelf, self.seat(doc, post, shelf), resolver=self.ws)
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
        self.assertEqual(said["worker"][:3], (1, [], []))


if __name__ == "__main__":
    unittest.main()
