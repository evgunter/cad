"""LIB-G18a review lane r1 — adopted probe rows.

Each row here is RED (or newly-covering) against `773d95af`, and each
names the claim it falsifies. They are written as ordinary suite rows so
the fix pass can adopt them; nothing here is lane-private machinery.

R1 and R2 falsify sentences that are asserted in THREE places each
(`crates/pncad-py/src/py/value.rs`, `crates/pncad-py/pncad.pyi`, and
`docs/guide/north-star-audit.md`'s G8 row) — fix the claim or fix the
code, but the sweep is all three sites, not one.
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
    Doc,
    ParamName,
    Workspace,
    evaluate,
    m,
)

from test_assembly_eval import (
    CORPUS,
    SHELF_THICKNESS,
    failures,
    manifest,
    opened,
    volumes,
)
from test_document import slab


def scratch(case):
    directory = Path(tempfile.mkdtemp()) / "bench"
    shutil.copytree(CORPUS, directory)
    case.addCleanup(shutil.rmtree, directory.parent, ignore_errors=True)
    return directory


class R1TheCountersDoNotAccountForPoisonedNodes(unittest.TestCase):
    """FALSIFIES: "`recomputed + reused` is the live node count either
    way" (`pncad.pyi` `Evaluation.recomputed`; `value.rs` same getter;
    audit G8 row "The two counters sum to the live node count, so they
    are evidence rather than a hint").

    `bookkeep` (`editor-core/src/eval/mod.rs:1211`) counts `Ok` and
    `Failed` and drops `Poisoned` on the floor. The suite's own
    `test_the_two_counters_account_for_every_live_node` evaluates ONLY
    with a resolver, where nothing is poisoned — so it cannot go red on
    this, which is the Q3 shape (a premise that excludes the failing
    mode)."""

    def test_the_counters_sum_to_the_live_node_count_when_a_node_poisons(self):
        _, docs = opened()
        evaluation = evaluate(docs["layout"])  # no resolver: node 1 poisons
        poisoned = [
            node
            for node in evaluation.order()
            if not evaluation.succeeded(node)
            and self._reason(evaluation, node) == "poisoned"
        ]
        self.assertTrue(poisoned, "the fixture must actually poison a node")
        self.assertEqual(
            evaluation.reused + evaluation.recomputed,
            len(evaluation.order()),
            "a poisoned node is in order() and in neither counter",
        )

    @staticmethod
    def _reason(evaluation, node):
        try:
            evaluation.value(node)
        except pncad.EvaluationError as refusal:
            return refusal.reason
        return None


class R2APriorDefeatsTheSeamsRefusals(unittest.TestCase):
    """FALSIFIES: "`None` -- the default -- is a kernel-only evaluation,
    in which every instantiate node refuses typed" (`pncad.pyi`
    `evaluate`; `value.rs` same docstring; and, upstream, the kernel's
    own `EvalOptions::resolver` doc).

    The memo's content key for `InstantiatePart` is (document id, pin,
    solved placement, interface) — it does NOT record whether a
    resolver was present or what the store held. So a `prior` answers
    where the seam would refuse, and the answer can be STALE: a body
    the store no longer holds any version of.

    This is the sharpest form of the brief's "can `reused` lie?": it
    does not miscount, but it silently converts a ratified refusal
    (A4's Cargo.lock pin gate, `PIN_MISMATCH_RECOURSE`) into a success.
    Whether the right fix is a docstring, a key change, or a design
    decision is the fix pass's call — but the behaviour must be pinned
    either way, and today nothing pins it."""

    def test_a_prior_does_not_evaluate_an_assembly_without_a_resolver(self):
        _, docs = opened()
        store, _ = opened()
        first = evaluate(docs["layout"], resolver=store)
        without = evaluate(docs["layout"], prior=first)
        self.assertNotEqual(
            failures(without),
            {},
            "resolver=None is documented to refuse at every instantiate node",
        )

    def test_a_moved_pin_still_refuses_when_a_prior_is_passed(self):
        directory = scratch(self)
        store, docs = opened(directory)
        first = evaluate(docs["layout"], resolver=store)
        docs["shelf"].apply(
            DocEdit.set_doc_param_value(
                ParamName("thickness"),
                DocParamValue.length(SHELF_THICKNESS * 1.5 * m),
            )
        )
        store.resave(docs["shelf"])
        moved = Workspace(str(directory))

        self.assertEqual(
            [r.kind for r in failures(evaluate(docs["layout"], resolver=moved)).values()],
            ["part_pin_mismatch"],
            "control: without a prior the pin gate fires",
        )
        with_prior = evaluate(docs["layout"], resolver=moved, prior=first)
        self.assertEqual(
            [r.kind for r in failures(with_prior).values()],
            ["part_pin_mismatch"],
            "a prior must not retarget the pin gate into a stale success",
        )

    def test_a_document_the_store_lost_still_refuses_when_a_prior_is_passed(self):
        directory = scratch(self)
        store, docs = opened(directory)
        first = evaluate(docs["layout"], resolver=store)
        os.remove(directory / f"{manifest()['post']}.pncad")
        gone = Workspace(str(directory))
        with_prior = evaluate(docs["layout"], resolver=gone, prior=first)
        self.assertNotEqual(
            failures(with_prior),
            {},
            "the referenced document is not in the store at any version",
        )


class R3ReuseIsKeyedByNodeIdNotByContentAlone(unittest.TestCase):
    """FALSIFIES: "a key is content, not position" (`pncad.pyi`
    `evaluate`; `test_a_prior_of_another_document_reuses_nothing_and_is
    _legal`'s docstring).

    `eval/mod.rs:1330` is `prior.nodes.get(&id)`: node IDENTITY selects
    the candidate and the content key only VALIDATES it. Identical
    content at a shifted id reuses nothing; identical content at the
    same id in a DIFFERENT document reuses everything. The suite's
    existing cross-document row passes by fixture accident, not by
    rule."""

    def test_identical_content_at_a_shifted_node_id_is_reused(self):
        a = Doc("g18a-r1-a")
        slab(a, (0 * m, 6 * m), (0 * m, 4 * m), (0 * m, 2 * m))
        prior = evaluate(a)

        b = Doc("g18a-r1-b")
        slab(b, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))  # shifts the ids
        slab(b, (0 * m, 6 * m), (0 * m, 4 * m), (0 * m, 2 * m))  # identical to a's
        again = evaluate(b, prior=prior)
        self.assertEqual(
            again.reused, 2, "the same content at a different id reuses nothing"
        )

    def test_a_different_document_does_reuse_when_the_ids_coincide(self):
        """The contrast, and it is GREEN today — recorded so the fix
        pass sees both halves of the rule at once."""
        a = Doc("g18a-r1-c")
        slab(a, (0 * m, 6 * m), (0 * m, 4 * m), (0 * m, 2 * m))
        prior = evaluate(a)
        c = Doc("g18a-r1-d")
        slab(c, (0 * m, 6 * m), (0 * m, 4 * m), (0 * m, 2 * m))
        self.assertEqual(evaluate(c, prior=prior).reused, 2)


class R4TheEpsilonSeamArmIsReachableFromPython(unittest.TestCase):
    """NEW COVERAGE, and it corrects a reachability claim.

    `test_assembly_eval.TestTheResolutionRefusals` says "Three of the
    four are reachable from Python", and the audit's G18 row lists all
    eight `part_*` tags as reachable "where a Python-buildable document
    can reach it (the cycle arm is not)". Measured: `part_epsilon_seam`
    is a FOURTH reachable arm and no Python row exercises it — a store
    is a directory, and a referenced document's recorded epsilon is
    outside its content pin, so editing it does not trip the pin gate.

    (`part_root_failed`, `part_product` and `part_depth_exceeded` were
    swept the same way and ARE pin-gated — an on-disk content edit
    moves the pin, so they refuse `part_pin_mismatch` first. The
    unreachability argument the PR makes for the cycle arm covers those
    three too; it does not cover this one.)"""

    def test_a_referenced_document_recording_another_epsilon_refuses_typed(self):
        directory = scratch(self)
        post = directory / f"{manifest()['post']}.pncad"
        text = post.read_text(encoding="utf-8")
        self.assertIn('"epsilon": 1e-9,', text)
        post.write_text(text.replace('"epsilon": 1e-9,', '"epsilon": 5e-9,'))

        _, docs = opened()
        refusals = failures(evaluate(docs["layout"], resolver=Workspace(str(directory))))
        self.assertEqual(
            sorted(r.kind for r in refusals.values()),
            ["part_epsilon_seam", "part_epsilon_seam"],
        )


if __name__ == "__main__":
    unittest.main()
