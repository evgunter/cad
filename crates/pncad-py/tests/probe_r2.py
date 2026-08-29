"""Reviewer probe rows for LIB-G18a (lane lib-g18a-r2), against 773d95af.

Named probe_* so unittest discovery skips it; run directly:

    PYTHONPATH=target/python-stage python3 crates/pncad-py/tests/probe_r2.py

Two adopted findings, each demonstrated by execution:

R2-P1 (behavior, undocumented): a `prior=` taken before a resave
    fully reuses and serves the pinned old part body where a fresh
    evaluation refuses `part_pin_mismatch`. The memo key is the
    instantiate node's content (id + pin + placement), so the value is
    content-certified — but the pin GATE is bypassed: the resolver is
    never asked, so "a pin that moved refuses" holds only for
    evaluations that actually cross the seam. Nothing at the Python
    surface states this consequence.

R2-P2 (doc claim, false): the new `reused`/`recomputed` docstrings
    (py/value.rs getters, pncad.pyi) claim `recomputed + reused` "is
    the live node count either way". Poisoned nodes are counted by
    neither (kernel `bookkeep` skips `NodeResult::Poisoned`), so on
    any refusal path the sum undershoots `len(order())`:
    no-resolver layout reads 0 + 2 = 2 against 3 nodes in `order()`.
    The kernel's own field doc is honest ("how many nodes actually ran
    their op"); the overclaim is this unit's. The committed
    `test_the_two_counters_account_for_every_live_node` only runs
    all-success documents, so it cannot catch this.
"""

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


def failures(evaluation):
    out = {}
    for node in evaluation.order():
        if not evaluation.succeeded(node):
            try:
                evaluation.value(node)
            except pncad.EvaluationError as refusal:
                out[node] = refusal
    return out


def volumes(evaluation, node):
    return [b.mass_properties().volume for b in evaluation.value(node).bodies()]


def scratch():
    directory = Path(tempfile.mkdtemp()) / "bench"
    shutil.copytree(CORPUS, directory)
    return directory


class R2P1MemoBypassesThePinGate(unittest.TestCase):
    """Green: pins the MEASURED behavior so the fix pass can decide
    whether to keep it (and document it) or change it."""

    def test_a_prior_serves_where_a_fresh_evaluation_refuses(self):
        directory = scratch()
        store, docs = opened(directory)
        first = evaluate(docs["layout"], resolver=store)
        self.assertEqual(failures(first), {})
        old_shelf = volumes(first, first.order()[2])

        docs["shelf"].apply(
            DocEdit.set_doc_param_value(
                ParamName("thickness"), DocParamValue.length(0.06 * m)
            )
        )
        store.resave(docs["shelf"])
        moved = Workspace(str(directory))

        fresh = evaluate(docs["layout"], resolver=moved)
        self.assertEqual(
            [r.kind for r in failures(fresh).values()], ["part_pin_mismatch"]
        )

        memoized = evaluate(docs["layout"], resolver=moved, prior=first)
        self.assertEqual(failures(memoized), {}, "the memo never asks the seam")
        self.assertEqual((memoized.reused, memoized.recomputed), (3, 0))
        self.assertEqual(
            volumes(memoized, memoized.order()[2]),
            old_shelf,
            "the pinned old body is served, not the resaved one",
        )


class R2P2TheCounterSumClaim(unittest.TestCase):
    """The docstrings' claim, asserted verbatim — and it is red on any
    refusal path, so it rides as expectedFailure: poisoned nodes are
    counted by neither counter."""

    @unittest.expectedFailure
    def test_the_sum_is_the_live_node_count_on_a_refusal_path(self):
        _, docs = opened()
        refusing = evaluate(docs["layout"])  # no resolver: 2 fail, 1 poisoned
        self.assertEqual(
            refusing.reused + refusing.recomputed, len(refusing.order())
        )

    def test_what_the_sum_actually_is(self):
        _, docs = opened()
        refusing = evaluate(docs["layout"])
        poisoned = sum(
            1 for r in failures(refusing).values() if r.reason == "poisoned"
        )
        self.assertEqual(
            refusing.reused + refusing.recomputed,
            len(refusing.order()) - poisoned,
            "the sum is nodes that RAN or were reused; poisoned count as neither",
        )


if __name__ == "__main__":
    unittest.main(verbosity=2)
