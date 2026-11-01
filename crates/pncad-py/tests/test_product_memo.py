"""One evaluation, one gather: the product memoized on `Evaluation`.

`run_checks`, `product` and `product_named` all want the document's
product. A Python
`Evaluation` is the immutable (document, evaluation) pair captured at
`evaluate` and the product is a pure function of that pair and the
run's tolerance, so the first of those doors gathers and the rest
reuse: asking twice costs once.

WHAT THIS FILE CAN AND CANNOT SEE. The gather COUNT is not a Python
observable — the kernel's witness is a debug-only thread-local counter
and binding it would make a public door out of a profile-dependent
number — so the counts are pinned in Rust, on the default build path,
by `crate::tests::product_memo_rows` in `crates/pncad-py/src/tests.rs`,
against the very functions these doors call. What is asserted HERE is
everything a caller can actually observe: that the answers are
unchanged in either order, and that the pairing refusal every one of
these doors owes still arrives — which is the one thing a memo could
have silently dropped, because a memo reaches no gather to be refused
by.
"""

import unittest
from placing import assemble_world_copies

import pncad
from pncad import (
    Doc,
    Formula,
    Node,
    evaluate,
    m,
    product,
    product_named,
    run_checks,
)
from measuring import pinned


def slab(doc, x0, x1):
    """The axis-aligned box [x0,x1] x [0,1] x [0,1], in metres."""
    profile = doc.insert(
        Node.polygon(
            [
                (Formula.length_in(x0, m), Formula.length_in(0.0, m)),
                (Formula.length_in(x1, m), Formula.length_in(0.0, m)),
                (Formula.length_in(x1, m), Formula.length_in(1.0, m)),
                (Formula.length_in(x0, m), Formula.length_in(1.0, m)),
            ],
        )
    )
    return doc.insert(Node.extrude(profile, Formula.length_in(1.0, m)))


def one_box(seed="memo-one-box"):
    doc = Doc(seed)
    doc.set_product(doc.place(slab(doc, 0.0, 1.0), doc.world))
    return doc


class TestBothQuestionsOnOneEvaluation(unittest.TestCase):
    def test_the_order_the_doors_are_asked_in_changes_nothing(self):
        """The memo is filled by whichever door asks first, and the
        others read what the first left. Both orders, one document,
        the same answers."""
        doc = one_box()

        forward = evaluate(doc)
        report_first = run_checks(doc, forward)
        named_second = product_named(doc, forward)

        backward = evaluate(doc)
        named_first = product_named(doc, backward)
        report_second = run_checks(doc, backward)

        self.assertEqual(report_first.findings, report_second.findings)
        self.assertEqual(report_first.skipped, report_second.skipped)
        self.assertEqual(named_first[1], named_second[1])
        self.assertEqual(
            pinned(named_first[0]).volume,
            pinned(named_second[0]).volume,
        )
        self.assertEqual(
            pinned(product(doc, forward)).volume,
            pinned(named_first[0]).volume,
        )

    def test_two_evaluations_of_one_document_answer_the_same(self):
        """The memo is per-evaluation, and an evaluation is per-ask.
        Two of them are two memos and one answer."""
        doc = one_box("memo-two-evaluations")
        assemble_world_copies(doc)
        first = pinned(product(doc, evaluate(doc))).volume
        assemble_world_copies(doc)
        second = pinned(product(doc, evaluate(doc))).volume
        self.assertEqual(first, second)

    def test_a_document_with_an_empty_world_refuses_every_time(self):
        """A gather that refuses carries no product to keep, so nothing
        is memoized and the second ask refuses exactly as the first
        did — never a cached refusal, never a silent pass."""
        doc = Doc("memo-no-body-root")
        doc.insert(
            Node.polygon(
                [
                    (Formula.length_in(0.0, m), Formula.length_in(0.0, m)),
                    (Formula.length_in(1.0, m), Formula.length_in(0.0, m)),
                    (Formula.length_in(1.0, m), Formula.length_in(1.0, m)),
                ],
            )
        )
        ev = evaluate(doc)
        for _ in range(2):
            with self.assertRaises(pncad.ProductError) as caught:
                product(doc, ev)
            self.assertEqual(caught.exception.variant, "empty_product")


class TestTheMemoRefusesAMispairedDocument(unittest.TestCase):
    """DI3 at the memo. A door that answers from a memo reaches no
    gather, so the refusal the gather owed — an evaluation of ANOTHER
    document — is raised before the memo is consulted instead. Node
    ids are minted per document, so a foreign evaluation answers every
    lookup and produces a confident answer about other geometry: this
    is the refusal that stops it, and it is asked at every door that
    wants a product.
    """

    def setUp(self):
        self.doc = one_box("memo-paired")
        self.other = one_box("memo-paired-other")
        self.ev = evaluate(self.other)

    def test_run_checks_refuses_under_the_registrys_own_class(self):
        with self.assertRaises(pncad.ChecksError) as caught:
            run_checks(self.doc, self.ev)
        self.assertEqual(caught.exception.variant, "evaluation_of_another_document")

    def test_product_refuses(self):
        with self.assertRaises(pncad.ProductError) as caught:
            product(self.doc, self.ev)
        self.assertEqual(caught.exception.variant, "evaluation_of_another_document")

    def test_product_named_refuses(self):
        with self.assertRaises(pncad.ProductError) as caught:
            product_named(self.doc, self.ev)
        self.assertEqual(caught.exception.variant, "evaluation_of_another_document")

    def test_the_refusal_stands_after_the_evaluation_has_been_gathered(self):
        """The order that would catch a pairing check sited behind the
        memo: fill the memo through the document the evaluation IS of,
        then ask the same evaluation about another document. A memo
        consulted first would answer, in full, about the wrong
        recipe."""
        product(self.other, self.ev)
        with self.assertRaises(pncad.ProductError) as caught:
            product(self.doc, self.ev)
        self.assertEqual(caught.exception.variant, "evaluation_of_another_document")
        with self.assertRaises(pncad.ChecksError):
            run_checks(self.doc, self.ev)


if __name__ == "__main__":
    unittest.main()
