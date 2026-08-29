"""R1 review probes for LIB-G18a: the resolver and memo parameters.

Each row here is a QUESTION the unit's own suite does not ask. Rows
marked RED-AS-WRITTEN record behaviour the reviewer believes is wrong
or under-documented; rows marked PIN record behaviour that is correct
and was simply unpinned.
"""

import os
import shutil
import tempfile
import unittest
from pathlib import Path

import pncad
from pncad import (
    DocEdit,
    DocParamValue,
    DocRef,
    ParamName,
    Workspace,
    evaluate,
    m,
)

CORPUS = Path(__file__).resolve().parent / "corpus" / "bench"


def manifest():
    text = (CORPUS / "MANIFEST").read_text(encoding="utf-8")
    return dict(line.split() for line in text.splitlines() if line.strip())


def opened(directory=CORPUS):
    store = Workspace(str(directory))
    docs = {
        label: store.resolve(DocRef(ident, store.current_pin(ident)))
        for label, ident in manifest().items()
    }
    return store, docs


def kinds(evaluation):
    out = []
    for node in evaluation.order():
        if not evaluation.succeeded(node):
            try:
                evaluation.value(node)
            except pncad.EvaluationError as refusal:
                out.append(refusal.kind)
    return sorted(out)


def volumes(evaluation, node):
    return [b.mass_properties().volume for b in evaluation.value(node).bodies()]


class ProbeCase(unittest.TestCase):
    def scratch(self):
        directory = Path(tempfile.mkdtemp()) / "bench"
        shutil.copytree(CORPUS, directory)
        self.addCleanup(shutil.rmtree, directory.parent, ignore_errors=True)
        return directory


class TestTheMemoVersusTheSeamGate(ProbeCase):
    """Can `reused` lie? The unit asserts a memo hit never asks the
    seam and calls that sharing evidence. The other half of that
    sentence is that a memo hit never asks the seam's GATES either."""

    def test_a_prior_makes_a_moved_pin_stop_refusing(self):
        """RED-AS-WRITTEN. `test_a_pin_that_moved_refuses_rather_than
        _retargeting` pins that a resaved part refuses `part_pin_mismatch`.
        Pass a prior from before the resave and the same evaluation
        succeeds instead, reporting the STALE body as a full reuse."""
        directory = self.scratch()
        store, docs = opened(directory)
        before = evaluate(docs["layout"], resolver=store)
        self.assertEqual(kinds(before), [])

        docs["shelf"].apply(
            DocEdit.set_doc_param_value(
                ParamName("thickness"), DocParamValue.length(0.04 * 1.5 * m)
            )
        )
        store.resave(docs["shelf"])
        moved = Workspace(str(directory))

        without_prior = evaluate(docs["layout"], resolver=moved)
        self.assertEqual(kinds(without_prior), ["part_pin_mismatch"])

        with_prior = evaluate(docs["layout"], resolver=moved, prior=before)
        # The reviewer's claim: the gate is skipped and nothing says so.
        self.assertEqual(
            kinds(with_prior), ["part_pin_mismatch"],
            "a prior must not let a moved pin evaluate silently",
        )

    def test_a_prior_makes_a_missing_document_stop_refusing(self):
        """RED-AS-WRITTEN. Delete the part the store must supply. With
        no prior the instance refuses `part_unresolved`; with a prior
        the evaluation succeeds and crosses the seam zero times."""
        directory = self.scratch()
        store, docs = opened()
        before = evaluate(docs["layout"], resolver=store)

        os.remove(directory / f"{manifest()['post']}.pncad")
        gone = Workspace(str(directory))

        self.assertEqual(
            kinds(evaluate(docs["layout"], resolver=gone)),
            ["part_unresolved", "part_unresolved"],
        )
        with_prior = evaluate(docs["layout"], resolver=gone, prior=before)
        self.assertEqual(
            kinds(with_prior), ["part_unresolved", "part_unresolved"],
            "a prior must not resurrect a document the store no longer holds",
        )

    def test_a_prior_lets_an_assembly_evaluate_with_no_resolver_at_all(self):
        """PIN of the shape, stated positively. This one the unit
        asserts on purpose (`test_a_memo_hit_never_asks_the_seam`);
        recorded here so the class is visible as a class."""
        store, docs = opened()
        before = evaluate(docs["layout"], resolver=store)
        after = evaluate(docs["layout"], prior=before)
        self.assertEqual((after.reused, after.recomputed), (3, 0))
        self.assertEqual(after.part_evaluations, 0)
        self.assertEqual(kinds(after), [])


class TestTheResolverSnapshot(ProbeCase):
    """Claim 3: the resolver is a SNAPSHOT of the id -> path scan,
    taken at the `evaluate` call. What does a mutation through the same
    Python object do to it?"""

    def test_a_resave_through_the_same_object_is_seen_by_a_later_evaluate(self):
        """PIN. The unit's own pin-mismatch test builds a FRESH
        `Workspace` after `resave` rather than reusing the one it
        resaved through. This row asks whether that was necessary."""
        directory = self.scratch()
        store, docs = opened(directory)
        docs["shelf"].apply(
            DocEdit.set_doc_param_value(
                ParamName("thickness"), DocParamValue.length(0.04 * 1.5 * m)
            )
        )
        store.resave(docs["shelf"])
        self.assertEqual(
            kinds(evaluate(docs["layout"], resolver=store)),
            ["part_pin_mismatch"],
            "the store the resave went through must see its own write",
        )

    def test_the_snapshot_is_taken_at_the_evaluate_call_not_before(self):
        """PIN. A document created after the `Workspace` was built, but
        before `evaluate`, IS visible -- so the snapshot point is the
        call, not the construction."""
        directory = self.scratch()
        names = manifest()
        os.remove(directory / f"{names['post']}.pncad")
        gone = Workspace(str(directory))
        _, whole = opened()

        self.assertEqual(
            kinds(evaluate(whole["layout"], resolver=gone)),
            ["part_unresolved", "part_unresolved"],
        )
        gone.create(whole["post"])
        self.assertEqual(
            kinds(evaluate(whole["layout"], resolver=gone)), [],
            "a create before evaluate is inside the snapshot",
        )


class TestTheCrossDocumentPrior(ProbeCase):
    """Claim 2: `prior` from a DIFFERENT document. The stub says a key
    is content, not position, and the unit tests only the case where
    NOTHING coincides."""

    def test_a_prior_from_a_sibling_assembly_over_the_same_parts(self):
        """The layout and the stand instantiate the SAME two parts.
        Records what actually happens rather than asserting a belief."""
        store, docs = opened()
        layout = evaluate(docs["layout"], resolver=store)
        stand = evaluate(docs["stand"], resolver=store, prior=layout)
        self.assertEqual(kinds(stand), [], "no refusal either way")
        self.assertEqual(
            stand.reused + stand.recomputed, len(stand.order()),
            "the counters still account for every live node",
        )
        material = sorted(v for n in stand.order() for v in volumes(stand, n))
        self.assertEqual(len(material), 3, f"reused={stand.reused} {material}")


class TestTheCorpusStalenessHoleIsWiderThanDisclosed(unittest.TestCase):
    """Claim 1. The disclosed hole is 'a STRUCTURAL change with the
    constants left alone'. These rows measure two more."""

    def test_a_placement_oracle_was_available_and_unused(self):
        """The committed suite's every oracle is a VOLUME, and a volume
        is invariant under every seat, mate and pattern OFFSET the
        scene declares -- so no row goes red if `SEAT_A`, `SEAT_B` or
        `POST_SEAT` are re-derived, none of which the constants guard
        reads. This row shows the cheapest oracle that WOULD have: the
        patterned posts occupy four distinct boxes, readable through
        `tessellate`, which the binding already exposes."""
        from pncad import mm

        store, docs = opened()
        layout = evaluate(docs["layout"], resolver=store)
        pattern = layout.order()[1]
        boxes = set()
        for body in layout.value(pattern).bodies():
            mesh = body.tessellate(5 * mm)
            xs = [round(p[0].meters, 9) for p in mesh.positions]
            ys = [round(p[1].meters, 9) for p in mesh.positions]
            boxes.add((min(xs), min(ys)))
        self.assertEqual(
            len(boxes), 4,
            "the four patterned posts are at four distinct origins, and "
            "nothing in the committed suite reads that",
        )


if __name__ == "__main__":
    unittest.main()
