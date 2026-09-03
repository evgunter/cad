"""Cooperative cancellation: the token, the partial answer, the thread.

Census family B-CANCEL (LIB-B-CANCEL). `evaluate` grew a `cancel=`
keyword taking a `CancelToken`, and a canceled run answers by BEING a
partial `Evaluation` — so `Evaluation.canceled` is bound beside it,
because a stop nobody can observe is not a stop.

**What is a PIN here and what is a MEASUREMENT.** Cancelling a run
already under way is a race by construction: the kernel checks the
token between nodes, and which node it stops at is whatever the
scheduler decided. So the contract is pinned on the DETERMINISTIC
arm — a token canceled BEFORE the run starts, where the check before
the first node fires every time and the prefix is exactly empty — and
the concurrent arm is pinned only on the invariants that hold whatever
the race decides. The one thing the concurrent arm does pin
categorically is that the evaluating thread RELEASES THE GIL, because
that is a yes/no about the binding rather than a threshold about the
schedule, and without it the token is decorative: a Python thread that
cannot run cannot set a flag.

The numbers behind the choice, measured on this box against the debug
extension (LIB-B-CANCEL, 2026-09-03), on the `stack(WIDE)` document
these tests use: 31 nodes, ~300 ms. A helper thread cancelling 20 ms
in stopped it 20/20 with a 7-node prefix; cancelling with no delay
stopped it 20/20 with a prefix of 0, the helper winning the start.
The SAME code against a build differing only in that it does not
release the GIL cancelled **0 of 20 at either delay** — the helper
could not execute a bytecode until the run had already finished — and
recorded 0 stamps in the window the GIL pin counts, against 139 with
the GIL released.

Those are UNGUARDED readings, quoted to explain the shape of the tests
and computed by none of them: a rate two boxes could measure
differently is not a guard. What is guarded is the 0-versus-many, and
the counterfactual above is what says the guard has teeth.
"""

import threading
import time
import unittest

from pncad import (
    BooleanOp,
    CancelToken,
    Doc,
    EvaluationError,
    Node,
    evaluate,
    m,
)


def slab(doc, x, y, z):
    """Insert the axis-aligned box [x0,x1] x [y0,y1] x [z0,z1]."""
    x0, x1 = x
    y0, y1 = y
    z0, z1 = z
    profile = doc.insert(
        Node.polygon(
            [(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
            plane=doc.sketch_frame(elevation=z0),
        )
    )
    return doc.insert(Node.extrude(profile, z1 - z0))


def stack(count):
    """A left-leaning union of `count` overlapping boxes.

    Deliberately serial: each union takes the previous one, so the
    schedule is a CHAIN. Python evaluates on the sequential path
    (`EvalOptions.parallel` is false and is one of the fields with no
    Python spelling), so the cancel check falls between nodes and the
    completed results are a prefix of `order()` — which is what the
    concurrent test below reads.

    Every box is offset in y and z as well as x, so no two faces are
    coincident: coplanar operands are the C4 undeclared-contact
    refusal, and a document that FAILS nodes would confuse a stopped
    run with a broken one. `count` boxes give `4 * count - 1` nodes,
    all of which succeed.
    """
    doc = Doc()
    fused = None
    for i in range(count):
        dy, dz = 0.37 * i, 0.23 * i
        box = slab(
            doc,
            (i * 1.0 * m, (i + 3.0) * m),
            (dy * m, (3.0 + dy) * m),
            (dz * m, (3.0 + dz) * m),
        )
        fused = (
            box
            if fused is None
            else doc.insert(Node.boolean(BooleanOp.Union, fused, box))
        )
    return doc, fused


#: The deterministic arm's document: small, because a pre-canceled run
#: evaluates nothing and the full runs it is compared against should
#: stay cheap.
SMALL = 3

#: The concurrent arm's document, sized by measurement: 31 nodes take
#: ~300 ms against the debug extension on this box, which leaves a
#: helper thread a wide window to be scheduled in. Nothing asserted
#: below depends on that duration — see the module docstring.
WIDE = 8


class TestCancelToken(unittest.TestCase):
    """The token on its own: a flag with two states and no reset."""

    def test_a_fresh_token_is_not_canceled(self):
        self.assertFalse(CancelToken().canceled)

    def test_cancel_sets_it_and_is_idempotent(self):
        token = CancelToken()
        token.cancel()
        self.assertTrue(token.canceled)
        token.cancel()
        self.assertTrue(token.canceled)

    def test_there_is_no_way_back(self):
        """One-way by construction: no `reset`, no settable attribute.

        A reusable token would let two runs disagree about what it
        meant, so the class simply does not spell the reverse — and
        `canceled` is a read-only property, not a field a caller can
        put False back into.
        """
        token = CancelToken()
        token.cancel()
        self.assertFalse(hasattr(token, "reset"))
        with self.assertRaises(AttributeError):
            token.canceled = False

    def test_the_repr_says_which_state_it_is_in(self):
        token = CancelToken()
        self.assertEqual(repr(token), "CancelToken(canceled=False)")
        token.cancel()
        self.assertEqual(repr(token), "CancelToken(canceled=True)")


class TestCanceledEvaluation(unittest.TestCase):
    """The deterministic arm — a token canceled before the run starts.

    The kernel reads the flag BEFORE each node, so a pre-canceled
    token stops the run at the very first check: zero nodes ran, and
    every assertion below is exact rather than bounded.
    """

    def setUp(self):
        self.doc, self.fused = stack(SMALL)
        self.token = CancelToken()
        self.token.cancel()
        self.canceled = evaluate(self.doc, cancel=self.token)

    def test_the_run_reports_itself_canceled(self):
        self.assertTrue(self.canceled.canceled)

    def test_a_run_with_no_token_is_never_canceled(self):
        """The default is a fresh token nobody holds — so the outcome
        of an ordinary Python evaluation is a constant, which is
        exactly why `EvalOutcome` needed no Python shape until this
        keyword existed."""
        self.assertFalse(evaluate(self.doc).canceled)

    def test_a_token_that_was_never_set_leaves_the_run_complete(self):
        self.assertFalse(evaluate(self.doc, cancel=CancelToken()).canceled)

    def test_the_order_is_still_the_whole_document(self):
        """Order is DATA, not schedule: the full deterministic order
        survives cancelation, which is what makes the prefix legible
        as a prefix rather than as an arbitrary subset."""
        complete = evaluate(self.doc)
        self.assertEqual(self.canceled.order(), complete.order())
        self.assertEqual(len(self.canceled.order()), 4 * SMALL - 1)

    def test_nothing_ran_and_nothing_was_reused(self):
        self.assertEqual(self.canceled.recomputed, 0)
        self.assertEqual(self.canceled.reused, 0)

    def test_every_node_reports_no_value(self):
        for node in self.canceled.order():
            self.assertFalse(self.canceled.succeeded(node))

    def test_an_unreached_node_raises_the_standing_ladder_reason(self):
        """`node_not_evaluated`, spelled as `ReadbackError` and
        `HitTestError` spell it — NOT `unknown_node`, which is the
        different fact that the document has no such id.

        This arm was unreachable before `cancel=`: with no way to stop
        a run, a live node always had a result, and the door said "no
        such node" for both states because only one could arise.
        """
        node = self.canceled.order()[0]
        with self.assertRaises(EvaluationError) as caught:
            self.canceled.value(node)
        self.assertEqual(caught.exception.reason, "node_not_evaluated")
        self.assertEqual(caught.exception.node, node)
        self.assertIsNone(caught.exception.kind)
        self.assertIsNone(caught.exception.through)

    def test_a_foreign_id_still_raises_unknown_node(self):
        """The two no-entry states stay apart, which is the whole
        point of splitting them: an id from ANOTHER document is not a
        node this run failed to reach."""
        other, _ = stack(SMALL + 2)
        stranger = evaluate(other).order()[-1]
        self.assertNotIn(stranger, self.canceled.order())
        with self.assertRaises(EvaluationError) as caught:
            self.canceled.value(stranger)
        self.assertEqual(caught.exception.reason, "unknown_node")

    def test_cancelation_is_not_failure(self):
        """A canceled run is a PARTIAL answer, never a failed one: no
        node is marked failed or poisoned by the cancelation, so a
        caller branching on `node_failed` never sees a stop reported
        as a defect."""
        for node in self.canceled.order():
            with self.assertRaises(EvaluationError) as caught:
                self.canceled.value(node)
            self.assertEqual(caught.exception.reason, "node_not_evaluated")

    def test_the_token_and_the_run_answer_different_questions(self):
        """`token.canceled` says the flag is set; `evaluation.canceled`
        says a run OBSERVED it. They differ exactly when a run finished
        before the flag was set — which is spellable in one line."""
        late = CancelToken()
        complete = evaluate(self.doc, cancel=late)
        late.cancel()
        self.assertTrue(late.canceled)
        self.assertFalse(complete.canceled)

    def test_one_token_can_stop_several_runs(self):
        """The token is a handle onto one flag, not a per-run object:
        a caller holding one can abandon a whole batch."""
        second, _ = stack(SMALL)
        self.assertTrue(evaluate(second, cancel=self.token).canceled)

    def test_a_canceled_run_is_a_legal_prior(self):
        """The memo certifies by content key, and a canceled run
        simply has fewer entries to certify against — so passing one
        as `prior=` reuses whatever it did finish and re-runs the
        rest, rather than refusing or serving a hole."""
        complete = evaluate(self.doc, prior=self.canceled)
        self.assertFalse(complete.canceled)
        self.assertEqual(complete.reused, 0)
        self.assertEqual(complete.recomputed, len(complete.order()))
        for node in complete.order():
            self.assertTrue(complete.succeeded(node))


class TestCancelingARunUnderWay(unittest.TestCase):
    """The concurrent arm. Everything here is either an invariant that
    holds whatever the race decides, or the yes/no about the GIL."""

    def test_the_evaluating_thread_releases_the_gil(self):
        """**The pin that makes the token more than decoration.**

        A helper thread stamps the clock every millisecond. If the
        evaluating thread held the GIL, the helper could not execute a
        single bytecode while the call was in flight, so no stamp
        could land inside the call's own window — and the ones taken
        in the switch-interval gap around the call sit within
        microseconds of its edges. Counting only stamps in the MIDDLE
        HALF of the window therefore reads exactly zero under a held
        GIL and dozens under a released one: a categorical difference,
        not a threshold on the schedule.
        """
        doc, _ = stack(WIDE)
        stop = threading.Event()
        stamps = []

        def sample():
            while not stop.is_set():
                stamps.append(time.perf_counter())
                time.sleep(0.001)

        helper = threading.Thread(target=sample, daemon=True)
        helper.start()
        try:
            # Let the helper reach its loop before the window opens.
            time.sleep(0.02)
            start = time.perf_counter()
            run = evaluate(doc)
            end = time.perf_counter()
        finally:
            stop.set()
            helper.join()

        span = end - start
        lo, hi = start + span / 4, end - span / 4
        inside = [s for s in stamps if lo <= s <= hi]
        self.assertFalse(run.canceled)
        self.assertGreater(
            len(inside),
            0,
            "no other Python thread ran during evaluate: the GIL was held "
            "for the kernel call, so `cancel=` cannot be set by anyone",
        )

    def test_editing_the_document_mid_run_is_refused_not_raced(self):
        """**What releasing the GIL does NOT open**, and the reason it
        is safe to release at all.

        `evaluate` borrows `doc` for the whole call. Another thread
        that can now run is also a thread that could try to EDIT the
        recipe being evaluated — so the borrow is what stands between
        a cooperative stop and a data race, and pyo3 enforces it:
        a mutating door needs `&mut`, the borrow flag is already
        taken, and the call raises `RuntimeError("Already borrowed")`.

        Loud, typed, and on the EDITING side — the evaluation is
        untouched and still answers for every node. Measured here
        rather than reasoned, because the reasoning is what named the
        wrong door the first time this was written down.
        """
        doc, _ = stack(WIDE)
        outcome = []

        def edit():
            time.sleep(0.02)
            try:
                doc.insert(
                    Node.polygon(
                        [(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m)],
                        plane=doc.sketch_frame(elevation=0 * m),
                    )
                )
                outcome.append(None)
            except BaseException as refused:  # noqa: BLE001 — the point
                outcome.append(refused)

        helper = threading.Thread(target=edit, daemon=True)
        helper.start()
        run = evaluate(doc)
        helper.join()

        self.assertEqual(len(outcome), 1)
        refused = outcome[0]
        self.assertIsInstance(
            refused,
            RuntimeError,
            "a mutation landed on a document under evaluation instead of "
            "being refused — the borrow that makes the GIL release safe is gone",
        )
        self.assertIn("borrow", str(refused).lower())
        # The run is unharmed: the refusal is the editor's, not its.
        self.assertFalse(run.canceled)
        for node in run.order():
            self.assertTrue(run.succeeded(node))

    def test_a_thread_cancelling_mid_run_yields_a_legal_partial(self):
        """The invariants, whichever way the race falls.

        Either the run completed — in which case every node has a
        value and `canceled` is False — or it was stopped, in which
        case the results are a PREFIX of the full order: some initial
        run of nodes succeeded and everything after it is unreached.
        Never a hole in the middle, never a node failed by the stop.
        """
        doc, _ = stack(WIDE)
        token = CancelToken()
        launched = threading.Event()

        def stopper():
            launched.wait()
            time.sleep(0.02)
            token.cancel()

        helper = threading.Thread(target=stopper, daemon=True)
        helper.start()
        launched.set()
        run = evaluate(doc, cancel=token)
        helper.join()

        order = run.order()
        self.assertEqual(len(order), 4 * WIDE - 1)
        reached = [run.succeeded(node) for node in order]
        if not run.canceled:
            self.assertTrue(all(reached))
            return
        # A prefix: once it stops reaching nodes it never starts again.
        self.assertEqual(sorted(reached, reverse=True), reached)
        self.assertEqual(run.recomputed, sum(reached))
        for node, ok in zip(order, reached, strict=True):
            if ok:
                continue
            with self.assertRaises(EvaluationError) as caught:
                run.value(node)
            self.assertEqual(caught.exception.reason, "node_not_evaluated")


if __name__ == "__main__":
    unittest.main()
