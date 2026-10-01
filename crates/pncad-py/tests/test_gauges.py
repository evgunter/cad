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
