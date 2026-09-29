"""Review probes for PR 3482 (partprod-rev): a poisoned part root two
documents down, and the gather's ProductError fields for a poisoned
root."""

import shutil
import tempfile
import unittest
from pathlib import Path

import pncad
from pncad import DocRef, Expr, Node, Workspace, evaluate, m


def failures(evaluation):
    out = {}
    for node in evaluation.order():
        if not evaluation.succeeded(node):
            try:
                evaluation.value(node)
            except pncad.EvaluationError as e:
                out[node] = e
    return out


def broken(name):
    """A part whose extrude refuses and whose one root is a transform over it."""
    part = pncad.Doc(name)
    profile = part.insert(
        Node.polygon(
            [
                (Expr.length_in(0, m), Expr.length_in(0, m)),
                (Expr.length_in(0.02, m), Expr.length_in(0, m)),
                (Expr.length_in(0.02, m), Expr.length_in(0.02, m)),
                (Expr.length_in(0, m), Expr.length_in(0.02, m)),
            ],
            plane=part.sketch_frame(elevation=Expr.length_in(0, m)),
        )
    )
    extrude = part.insert(Node.extrude(profile, Expr.length_in(0.0, m)))
    root = moved(part, extrude)
    return part, extrude, root


def moved(doc, node):
    return doc.insert(
        Node.transform(
            node,
            (Expr.length_in(0.01, m), Expr.length_in(0, m), Expr.length_in(0, m)),
            (Expr.literal(0.0), Expr.literal(0.0), Expr.literal(1.0)),
            Expr.literal(0.0 * pncad.rad),
        )
    )


class TestPoisonedTwoDeep(unittest.TestCase):
    def setUp(self):
        directory = Path(tempfile.mkdtemp())
        self.addCleanup(shutil.rmtree, directory, ignore_errors=True)
        self.store = Workspace(str(directory))
        self.broken, self.extrude, self.broken_root = broken("probe-py-broken")
        self.store.create(self.broken)
        self.broken_ref = DocRef(self.broken.id, pncad.content_pin(self.broken))
        bracket = pncad.Doc("probe-py-bracket")
        self.inner = bracket.insert(Node.instantiate_part(self.broken_ref))
        self.bracket_root = moved(bracket, self.inner)
        self.store.create(bracket)
        self.bracket_ref = DocRef(bracket.id, pncad.content_pin(bracket))
        self.assembly = pncad.Doc("probe-py-assembly")
        self.instance = self.assembly.insert(Node.instantiate_part(self.bracket_ref))

    def test_the_cause_chain_is_typed_to_the_failing_node(self):
        refusal = failures(evaluate(self.assembly, resolver=self.store))[self.instance]
        print("PROBE", refusal.kind, "|", refusal)
        self.assertEqual(refusal.kind, "part_root_poisoned")
        self.assertIsNone(refusal.document)
        mid = refusal.__cause__
        self.assertIsInstance(mid, pncad.EvaluationError)
        print("PROBE", mid.kind, mid.node, mid.document, "|", mid)
        self.assertEqual(mid.kind, "part_root_poisoned")
        self.assertEqual((mid.node, mid.document), (self.inner, self.bracket_ref))
        last = mid.__cause__
        self.assertIsInstance(last, pncad.EvaluationError)
        print("PROBE", last.kind, last.node, last.document, "|", last)
        self.assertEqual(last.kind, "extrude")
        self.assertEqual((last.node, last.document), (self.extrude, self.broken_ref))
        self.assertIsNone(last.__cause__)

    def test_the_gathers_refusal_names_root_and_through(self):
        ev = evaluate(self.broken)
        with self.assertRaises(pncad.ProductError) as caught:
            pncad.product(self.broken, ev)
        e = caught.exception
        print("PROBE product", e.variant, e.node, e.through, "|", e)
        self.assertEqual(e.variant, "root_poisoned")
        self.assertEqual(e.node, self.broken_root)
        self.assertEqual(e.through, self.extrude)


if __name__ == "__main__":
    unittest.main()
