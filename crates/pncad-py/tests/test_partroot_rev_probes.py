"""Review probes (lane `partroot-rev`) over PR 3459's Python surface.

Each row asserts what the review expects; a red row is a finding.
"""

import shutil
import tempfile
import unittest
from pathlib import Path

import bench_scene
import pncad
from bench_scene import POST_SEAT, SEAT_A, seat
from pncad import CapEnd, ContactClass, DocRef, Expr, Node, Workspace, evaluate, m
from pncad import solve_document
from test_assembly_author import BenchWorkspace
from test_assembly_eval import failures


class TestNestedCauseNamesItsDocument(unittest.TestCase):
    def test_the_cause_says_which_document_its_node_is_in(self):
        directory = Path(tempfile.mkdtemp())
        self.addCleanup(shutil.rmtree, directory, ignore_errors=True)
        store = Workspace(str(directory))
        boss = bench_scene.prism("rev-partroot-boss", 0.02, 0.02, 0.0)
        store.create(boss)
        bracket = pncad.Doc("rev-partroot-bracket")
        bracket.insert(Node.instantiate_part(DocRef(boss.id, pncad.content_pin(boss))))
        store.create(bracket)
        assembly = pncad.Doc("rev-partroot-assembly")
        instance = assembly.insert(
            Node.instantiate_part(DocRef(bracket.id, pncad.content_pin(bracket)))
        )
        refusal = failures(evaluate(assembly, resolver=store))[instance]
        cause = refusal.__cause__
        print("PY-PROBE str:", str(refusal))
        print("PY-PROBE cause:", repr(cause.node), str(cause))
        print("PY-PROBE cause.cause:", repr(cause.__cause__.node), str(cause.__cause__))
        print("PY-PROBE instance:", repr(instance))
        print("PY-PROBE attrs:", [a for a in dir(cause) if not a.startswith("_")])
        # Nothing on the cause says it is the bracket's node 0 rather
        # than the assembly's node 0.
        self.assertNotEqual(
            cause.node,
            instance,
            "the cause's node is indistinguishable from the assembly's own node",
        )


class TestMateErrorCause(BenchWorkspace):
    def test_a_raised_mate_error_carries_the_placer_refusal(self):
        doc = pncad.Doc("rev-placer-refused")
        post_a = doc.insert(Node.instantiate_part(self.post_ref))
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref))
        lifted = doc.insert(
            Node.transform(
                shelf_i,
                (Expr.length_in(0, m), Expr.length_in(0, m), Expr.length_in(0.25, m)),
                (Expr.literal(1e200), Expr.literal(0.0), Expr.literal(0.0)),
                Expr.literal(0.5 * pncad.rad),
            )
        )
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        mate = doc.insert(
            Node.mate(post_a, a_top, lifted, s_bottom, ContactClass.Rest, seat(POST_SEAT, SEAT_A))
        )
        solved = solve_document(doc, resolver=self.ws)
        fault = solved.fault(mate)
        print("PY-PROBE fault:", str(fault), "| cause:", str(fault.cause))
        with self.assertRaises(pncad.MateError) as caught:
            solved.placement(doc, shelf_i)
        err = caught.exception
        print("PY-PROBE MateError:", str(err), "| variant:", err.fault.variant)
        print("PY-PROBE __cause__:", repr(err.__cause__), "| suppress:", err.__suppress_context__)
        self.assertIsInstance(err.__cause__, pncad.EvaluationError)
        self.assertEqual(str(err.__cause__), str(fault.cause))


if __name__ == "__main__":
    unittest.main()
