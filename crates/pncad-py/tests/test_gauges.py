"""Gauges, instance offsets and the unplaced group, from Python.

The Python mirror of `crates/editor-core/tests/p2_gauges.rs`
(ASSEMBLY.md A11 (2)): an instance names its gauge — the world by
default — and may carry an offset in it; a gauge names its parent; a
placing mate places its FIRST operand's group on its second's and
clears the first's root offset; the compound door copies the second's
gauge to the first's group before mating; and a group nothing places
lives in its own space, which the product, the gate, a measure and STEP
each refuse to compare with the world.

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
    Node,
    Placement,
    Workspace,
    evaluate,
    groups,
    m,
    rad,
    root_of,
    solve_document,
)

import bench_scene


def lifted(z):
    return Placement.literal(Frame.translation((0 * m, 0 * m, z * m)))


class Gauges(unittest.TestCase):
    def setUp(self):
        self.dir = tempfile.mkdtemp()
        self.addCleanup(shutil.rmtree, self.dir, True)
        self.ws = Workspace(self.dir)
        (_, self.post_ref), (_, self.shelf_ref) = bench_scene.parts(self.ws)

    def face(self, doc, node, side):
        return bench_scene.instance_face(self.ws, doc, node, side)

    def seat(self, doc, post, shelf):
        """"Mate the shelf to the post": the shelf's underside on the
        post's top — the stand's own first seat."""
        return Node.mate(
            shelf,
            self.face(doc, shelf, CapEnd.Start),
            post,
            self.face(doc, post, CapEnd.End),
            ContactClass.Rest,
            bench_scene.seat(*bench_scene.STAND_SEATS[0]),
        )

    def test_a_gauge_under_a_gauge_places_by_the_composed_chain(self):
        doc = Doc("py-gauge-chain")
        outer = doc.insert(Node.gauge(lifted(2.0)))
        inner = doc.insert(
            Node.gauge(Placement.literal(Frame.translation((5 * m, 0 * m, 0 * m))), outer)
        )
        post = doc.insert(Node.instantiate_part(self.post_ref))
        self.assertIsNone(doc.gauge(post), "an inserted instance stands on the world")
        self.assertEqual(doc.offset(post), Placement.identity())
        doc.apply(DocEdit.set_gauge(post, inner))
        self.assertEqual(doc.gauge(post), inner)
        self.assertEqual(doc.gauge(inner), outer)
        self.assertEqual(doc.node_kind(outer), "gauge")
        at = solve_document(doc, resolver=self.ws).placement(doc, post).origin
        self.assertEqual(tuple(c.meters for c in at), (5.0, 0.0, 2.0))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_gauge(outer, inner))
        self.assertEqual(caught.exception.variant, "gauge_cycle")
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_offset(outer, Placement.identity()))
        self.assertEqual(caught.exception.variant, "offset_on_non_instance")

    def test_promote_moves_an_offset_onto_a_gauge_and_fold_undoes_it(self):
        doc = Doc("py-gauge-promote")
        g = doc.insert(Node.gauge(lifted(1.0)))
        post = doc.insert(Node.instantiate_part(self.post_ref))
        doc.apply(DocEdit.set_gauge(post, g))
        doc.apply(DocEdit.set_offset(post, lifted(2.0)))
        before = solve_document(doc, resolver=self.ws).placement(doc, post).origin
        k = doc.apply(DocEdit.promote(post))
        self.assertEqual(doc.node_kind(k), "gauge")
        self.assertEqual(doc.gauge(k), g, "the gauge sits on the instance's gauge")
        self.assertEqual(doc.gauge(post), k)
        self.assertEqual(doc.offset(post), Placement.identity())
        after = solve_document(doc, resolver=self.ws).placement(doc, post).origin
        self.assertEqual(after, before, "nothing moves")
        self.assertIsNone(doc.apply(DocEdit.fold(k)))
        self.assertEqual(doc.gauge(post), g)
        self.assertEqual(doc.offset(post), lifted(2.0), "the gauge's steps in front")
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.fold(post))
        self.assertEqual(
            (caught.exception.variant, caught.exception.node),
            ("fold_on_non_gauge", post),
        )
        doc.apply(DocEdit.set_offset(post, None))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.promote(post))
        self.assertEqual(caught.exception.variant, "promote_without_offset")

    def test_the_mate_door_clears_the_first_operands_root_offset(self):
        doc = Doc("py-gauge-door")
        post = doc.insert(Node.instantiate_part(self.post_ref))
        shelf = doc.insert(Node.instantiate_part(self.shelf_ref))
        doc.insert(self.seat(doc, post, shelf), resolver=self.ws)
        [cleared] = doc.last_maintenance
        self.assertEqual(
            (cleared.variant, cleared.node, cleared.offset),
            ("offset_cleared", shelf, Placement.identity()),
        )
        self.assertIsNone(doc.offset(shelf))
        self.assertEqual(root_of(doc, shelf), post)

    def test_the_compound_door_regauges_the_first_operands_group(self):
        doc = Doc("py-gauge-compound")
        g = doc.insert(Node.gauge(lifted(1.0)))
        post = doc.insert(Node.instantiate_part(self.post_ref))
        doc.apply(DocEdit.set_gauge(post, g))
        shelf = doc.insert(Node.instantiate_part(self.shelf_ref))
        mate = doc.regauge_then_mate(self.seat(doc, post, shelf), resolver=self.ws)
        self.assertEqual(doc.gauge(shelf), g)
        self.assertEqual(groups(doc), [[post, shelf]])
        self.assertEqual(
            solve_document(doc, resolver=self.ws).role(mate), pncad.MateRole.Determining
        )

    def test_an_unplaced_group_lives_in_its_own_space(self):
        doc = Doc("py-gauge-unplaced")
        post = doc.insert(Node.instantiate_part(self.post_ref))
        shelf = doc.insert(Node.instantiate_part(self.shelf_ref))
        doc.apply(
            DocEdit.set_offset(
                shelf, Placement.literal(Frame.translation((2 * m, 0 * m, 0 * m)))
            )
        )
        doc.apply(DocEdit.set_offset(shelf, None))
        solved = solve_document(doc, resolver=self.ws)
        self.assertEqual(solved.unplaced(shelf), "no_offset")
        self.assertIsNone(solved.unplaced(post))
        with self.assertRaises(pncad.EvaluationError) as caught:
            solved.placement(doc, shelf)
        self.assertEqual(caught.exception.kind, "unplaced")
        self.assertIn(pncad.UNPLACED_RECOURSE, str(caught.exception))
        ev = evaluate(doc, resolver=self.ws)
        self.assertEqual(ev.unplaced(shelf), (shelf, "no_offset"))
        self.assertIsNone(ev.unplaced(post))
        with self.assertRaises(pncad.ExportError) as caught:
            ev.step_string(shelf)
        self.assertEqual(caught.exception.variant, "unplaced")
        self.assertEqual(caught.exception.parts, [(shelf, shelf, "no_offset")])

    def test_a_world_pose_is_the_gauge_chain_and_root_offset_onto_the_solve(self):
        """`A ∘ F ∘ B` through Python, against an independent 4x4
        composition: a post on a gauge under a turned gauge, at a turned
        offset, and the shelf seated on it."""

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

        control = Doc("py-afb-control")
        cp = control.insert(Node.instantiate_part(self.post_ref))
        cs = control.insert(Node.instantiate_part(self.shelf_ref))
        control.insert(self.seat(control, cp, cs), resolver=self.ws)
        rel = mat(solve_document(control, resolver=self.ws).placement(control, cs))

        doc = Doc("py-afb")
        outer = doc.insert(
            Node.gauge(
                Placement.literal(
                    Frame.rotate_then_translate((0.0, 0.0, 1.0), 0.25 * rad, (0 * m, 0 * m, 2 * m))
                )
            )
        )
        inner = doc.insert(
            Node.gauge(Placement.literal(Frame.translation((5 * m, 0 * m, 0 * m))), outer)
        )
        post = doc.insert(Node.instantiate_part(self.post_ref))
        shelf = doc.insert(Node.instantiate_part(self.shelf_ref))
        doc.apply(DocEdit.set_gauge(post, inner))
        doc.apply(DocEdit.set_gauge(shelf, inner))
        doc.apply(
            DocEdit.set_offset(
                post,
                Placement.literal(
                    Frame.rotate_then_translate((0.0, 0.0, 1.0), 0.3 * rad, (1 * m, 2 * m, 0 * m))
                ),
            )
        )
        doc.insert(self.seat(doc, post, shelf), resolver=self.ws)
        poses = solve_document(doc, resolver=self.ws)
        w_post = mul(mul(rz(0.25, (0, 0, 2)), rz(0.0, (5, 0, 0))), rz(0.3, (1, 2, 0)))
        self.assertLess(gap(mat(poses.placement(doc, post)), w_post), 1e-12)
        self.assertLess(gap(mat(poses.placement(doc, shelf)), mul(w_post, rel)), 1e-12)

    def test_the_compound_door_refuses_a_mate_that_would_start_placing(self):
        doc = Doc("py-gauge-compound-refuses")
        g = doc.insert(Node.gauge(lifted(1.0)))
        post = doc.insert(Node.instantiate_part(self.post_ref))
        doc.apply(DocEdit.set_gauge(post, g))
        other = doc.insert(Node.instantiate_part(self.post_ref))
        doc.apply(DocEdit.set_gauge(other, g))
        doc.apply(
            DocEdit.set_offset(other, Placement.literal(Frame.translation((2 * m, 0 * m, 0 * m))))
        )
        shelf = doc.insert(Node.instantiate_part(self.shelf_ref))
        declaring = doc.insert(self.seat(doc, other, shelf), resolver=self.ws)
        self.assertEqual(
            solve_document(doc, resolver=self.ws).role(declaring), pncad.MateRole.Declaring
        )
        with self.assertRaises(EditError) as caught:
            doc.regauge_then_mate(self.seat(doc, post, shelf), resolver=self.ws)
        self.assertEqual(caught.exception.variant, "would_start_placing")
        self.assertEqual(caught.exception.node, declaring)
        self.assertIsNone(doc.gauge(shelf), "the refused action leaves the document untouched")

    def test_the_gate_checks_an_unplaced_groups_own_space(self):
        """`pncad.assemble` reaches the kernel's one gate, which checks
        an unplaced group inside its own space: two posts seated at one
        spot under the shelf interpenetrate there, and the gate refuses
        though the world beside them certifies."""
        doc = Doc("py-gauge-gate")
        p1 = doc.insert(Node.instantiate_part(self.post_ref))
        shelf = doc.insert(Node.instantiate_part(self.shelf_ref))
        p2 = doc.insert(Node.instantiate_part(self.post_ref))
        seat_a = bench_scene.STAND_SEATS[0]
        doc.insert(self.seat(doc, p1, shelf), resolver=self.ws)
        doc.insert(
            Node.mate(
                p2,
                self.face(doc, p2, CapEnd.End),
                shelf,
                self.face(doc, shelf, CapEnd.Start),
                ContactClass.Rest,
                bench_scene.seat(seat_a[1], seat_a[0]),
            ),
            resolver=self.ws,
        )
        lone = doc.insert(Node.instantiate_part(self.post_ref))
        doc.apply(
            DocEdit.set_offset(lone, Placement.literal(Frame.translation((1 * m, 0 * m, 0 * m))))
        )
        g = doc.insert(Node.gauge(Placement.identity()))
        for node in (p1, shelf, p2):
            doc.apply(DocEdit.set_gauge(node, g))
        doc.apply(DocEdit.delete_node(g))
        ev = evaluate(doc, resolver=self.ws)
        self.assertIsNotNone(ev.unplaced(shelf))
        with self.assertRaises(pncad.AssemblyError):
            pncad.assemble(doc, ev)

    def test_the_doors_work_off_the_main_thread(self):
        """Authoring, solving and reading on a worker thread answers
        what the main thread does."""

        def author():
            doc = Doc("py-gauge-thread")
            g = doc.insert(Node.gauge(lifted(3.0)))
            post = doc.insert(Node.instantiate_part(self.post_ref))
            shelf = doc.insert(Node.instantiate_part(self.shelf_ref))
            doc.apply(DocEdit.set_gauge(post, g))
            mate = doc.regauge_then_mate(self.seat(doc, post, shelf), resolver=self.ws)
            solved = solve_document(doc, resolver=self.ws)
            return (
                doc.gauge(shelf),
                doc.offset(shelf),
                [r.variant for r in doc.last_maintenance],
                solved.role(mate),
                tuple(c.meters for c in solved.placement(doc, shelf).origin),
            )

        said = {}

        def worker():
            said["worker"] = author()

        thread = threading.Thread(target=worker)
        thread.start()
        thread.join()
        self.assertEqual(said["worker"], author())
        self.assertEqual(said["worker"][2], ["offset_cleared"])


if __name__ == "__main__":
    unittest.main()
