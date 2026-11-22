"""The certified range of one field: how far a parameter or a slot can
move before the document stops being PROVABLY the same build.

`RangeSearch` is the stepped search, `certified_range` runs it to its
budget, `admit_range` says whether a request can be searched at all,
and `RangeRefusal` is the typed no.

THE SCENE is a unit square extruded by the named parameter `depth`.
Straight geometry certifies over a macroscopic interval, so a side of
a seed that stays clear of `depth == 0` is proven in ONE replay — which
is what lets this file state its expectations as exact offsets rather
than as thresholds. The seed below is deliberately lopsided, so a
binding that swapped the two sides, or answered the kernel's own seed
where one was handed, reads a different number.

WHAT IS PINNED AND WHAT IS NOT. The answer's SHAPE is pinned at every
step count this file reads: three states per side (proven, ended,
undecided) that a pair of floats could not carry. How many replays a
side that reaches toward `depth == 0` needs is the driver's
certification width and is pinned nowhere here.
"""

import unittest

import pncad
from pncad import (
    CancelToken,
    Distribution,
    Doc,
    DocEdit,
    Formula,
    FreeVar,
    Node,
    QuantityOpMismatch,
    RangeField,
    RangeRefusal,
    RangeSearch,
    RangeSeed,
    VarName,
    admit_range,
    certified_range,
    deg,
    evaluate,
    m,
    mm,
)

SQUARE = [
    (Formula.length_in(0, m), Formula.length_in(0, m)),
    (Formula.length_in(1, m), Formula.length_in(0, m)),
    (Formula.length_in(1, m), Formula.length_in(1, m)),
    (Formula.length_in(0, m), Formula.length_in(1, m)),
]

#: The prism's depth and the seed every search below looks over, in
#: metres. ORACLES: every offset this file expects is one of these.
DEPTH = 1.0
SEED_LO = -0.5
SEED_HI = 0.25


def prism(other=None):
    """A document whose one solid is the unit square extruded by the
    parameter `depth`, and the extrude node.

    `other` is a distribution hung on a second parameter, `width`,
    that no node reads: a spread the search has to clear, and so a
    condition its answer has to name."""
    doc = Doc()
    doc.apply(DocEdit.declare_var(VarName("depth"), FreeVar.length(DEPTH * m)))
    if other is not None:
        doc.apply(DocEdit.declare_var(VarName("width"), FreeVar.length(2 * m, other)))
    outline = doc.insert(Node.polygon(SQUARE))
    solid = doc.insert(Node.extrude(outline, Formula.length_in(DEPTH, m)))
    doc.apply(DocEdit.set_param(solid, "distance", doc.parse_formula("depth")))
    return doc, solid


def seed():
    return RangeSeed(SEED_LO * m, SEED_HI * m)


DEPTH_FIELD = RangeField.param(VarName("depth"))


class TestTheFinishedSearch(unittest.TestCase):
    """A seed clear of zero is proven whole, one replay per side."""

    @classmethod
    def setUpClass(cls):
        cls.doc, cls.solid = prism(Distribution.normal(1 * mm))
        cls.search = RangeSearch(cls.doc, DEPTH_FIELD, seed())
        cls.words = []
        while True:
            cls.words.append(cls.search.step())
            if cls.words[-1] != "advanced":
                break
        cls.answer = cls.search.answer()

    def test_it_takes_one_replay_per_side_and_then_finishes(self):
        self.assertEqual(self.words, ["advanced", "advanced", "finished"])
        self.assertEqual(self.search.steps, 2)
        self.assertEqual(self.answer.steps, 2)

    def test_both_sides_reach_the_seeds_end(self):
        for name, side, end in (
            ("lo", self.answer.lo, SEED_LO),
            ("hi", self.answer.hi, SEED_HI),
        ):
            with self.subTest(side=name):
                self.assertTrue(side.reached_seed_end)
                self.assertEqual(side.proven_to.meters, end)
                self.assertIsNone(side.different)
                self.assertIsNone(side.not_decided)

    def test_the_proven_interval_is_the_seed(self):
        lo, hi = self.answer.proven_offsets
        self.assertEqual((lo.meters, hi.meters), (SEED_LO, SEED_HI))
        self.assertEqual(self.answer.seed, seed())
        self.assertEqual(
            (self.answer.seed.lo.meters, self.answer.seed.hi.meters),
            (SEED_LO, SEED_HI),
        )
        self.assertFalse(self.answer.nothing_proven)

    def test_the_absolute_interval_is_measured_from_the_nominal(self):
        lo, hi = self.answer.proven_interval
        self.assertEqual(self.answer.nominal.meters, DEPTH)
        self.assertEqual((lo.meters, hi.meters), (DEPTH + SEED_LO, DEPTH + SEED_HI))

    def test_the_answer_names_the_spread_it_pinned(self):
        self.assertEqual(self.answer.pinned, [self.doc.var(VarName("width"))])
        self.assertEqual(self.answer.field, DEPTH_FIELD)

    def test_no_other_spread_pins_nothing(self):
        doc, _ = prism()
        answer = certified_range(doc, DEPTH_FIELD, seed(), max_replays=2)
        self.assertEqual(answer.pinned, [])
        self.assertTrue(answer.lo.reached_seed_end)

    def test_the_slot_is_boxed_through_the_variable_it_reads(self):
        through_slot = certified_range(
            self.doc, RangeField.slot(self.solid, "distance"), seed(), max_replays=2
        )
        self.assertEqual(through_slot.proven_offsets, self.answer.proven_offsets)
        self.assertEqual(
            admit_range(self.doc, RangeField.slot(self.solid, "distance")).axis,
            self.doc.var(VarName("depth")),
        )
        # The two were asked through different fields, and the answer
        # says which.
        self.assertNotEqual(through_slot, self.answer)


class TestAnAnswerReadEarly(unittest.TestCase):
    """The answer is readable after any step, and what it has not
    replayed is undecided ground — never a bound."""

    def setUp(self):
        self.doc, _ = prism()

    def test_before_any_step_nothing_is_proven_and_more_time_helps(self):
        answer = RangeSearch(self.doc, DEPTH_FIELD, seed()).answer()
        self.assertEqual(answer.steps, 0)
        self.assertTrue(answer.nothing_proven)
        self.assertEqual(
            tuple(q.meters for q in answer.proven_offsets), (0.0, 0.0)
        )
        for name, side in (("lo", answer.lo), ("hi", answer.hi)):
            with self.subTest(side=name):
                self.assertFalse(side.reached_seed_end)
                self.assertIsNone(side.different)
                self.assertEqual(side.not_decided.past.meters, 0.0)
                self.assertEqual(side.not_decided.reason, "unfinished")
                self.assertEqual(side.not_decided.replays, 0)
                self.assertIsNone(side.not_decided.budget)
                self.assertIsNone(side.not_decided.width)
                self.assertTrue(side.not_decided.more_time_helps)

    def test_one_step_decides_one_side_and_leaves_the_other_undecided(self):
        search = RangeSearch(self.doc, DEPTH_FIELD, seed())
        self.assertEqual(search.step(), "advanced")
        answer = search.answer()
        decided = [s for s in (answer.lo, answer.hi) if s.reached_seed_end]
        waiting = [s for s in (answer.lo, answer.hi) if s.not_decided is not None]
        self.assertEqual((len(decided), len(waiting)), (1, 1))
        self.assertTrue(waiting[0].not_decided.more_time_helps)
        self.assertFalse(answer.nothing_proven)

    def test_a_spent_replay_count_finishes_and_keeps_what_was_proven(self):
        search = RangeSearch(self.doc, DEPTH_FIELD, seed(), max_replays=1)
        self.assertEqual([search.step(), search.step()], ["advanced", "finished"])
        self.assertEqual(search.steps, 1)
        self.assertEqual(
            search.answer(), certified_range(self.doc, DEPTH_FIELD, seed(), max_replays=1)
        )

    def test_the_same_steps_give_the_same_answer(self):
        def after(steps):
            search = RangeSearch(self.doc, DEPTH_FIELD, seed())
            for _ in range(steps):
                search.step()
            return search.answer()

        for steps in (0, 1, 2):
            with self.subTest(steps=steps):
                first, second = after(steps), after(steps)
                self.assertEqual(first, second)
                self.assertEqual(hash(first), hash(second))
        # Equality reads the answer: one more step is another answer.
        self.assertNotEqual(after(1), after(2))


class TestTheSeed(unittest.TestCase):
    def setUp(self):
        self.doc, _ = prism()

    def test_the_kernels_seed_spans_zero_to_twice_the_value(self):
        chosen = RangeSeed.of(self.doc, DEPTH_FIELD)
        self.assertEqual((chosen.lo.meters, chosen.hi.meters), (-DEPTH, DEPTH))
        self.assertEqual(chosen, RangeSeed.symmetric(DEPTH * m))
        self.assertEqual(chosen.dimension, "length")
        # No seed handed is that seed, stated on the answer.
        self.assertEqual(RangeSearch(self.doc, DEPTH_FIELD).answer().seed, chosen)

    def test_a_seed_in_another_dimension_is_refused_at_the_door(self):
        for wrong in (RangeSeed(-0.5, 0.25), RangeSeed(-1 * deg, 1 * deg)):
            with self.subTest(seed=repr(wrong)):
                with self.assertRaises(QuantityOpMismatch) as caught:
                    admit_range(self.doc, DEPTH_FIELD, wrong)
                self.assertEqual(caught.exception.left, "length")
        with self.assertRaises(QuantityOpMismatch):
            RangeSeed(-1 * mm, 1 * deg)


class TestTheTypedRefusal(unittest.TestCase):
    """Each arm reached from Python, with its payload on the attribute
    the class documents and `None` on the rest."""

    PAYLOAD = ("lo", "hi", "analyzed", "param", "node", "slot", "cause", "varying")

    def setUp(self):
        self.doc, self.solid = prism()

    def refused(self, call, variant, **carried):
        with self.assertRaises(RangeRefusal) as caught:
            call()
        error = caught.exception
        self.assertIsInstance(error, pncad.PncadError)
        self.assertEqual(error.variant, variant)
        for name in self.PAYLOAD:
            with self.subTest(variant=variant, attribute=name):
                self.assertEqual(getattr(error, name), carried.get(name))
        return error

    def test_a_seed_that_excludes_the_current_value(self):
        self.refused(
            lambda: RangeSearch(self.doc, DEPTH_FIELD, RangeSeed(0.1 * m, 0.2 * m)),
            "seed_not_a_bracket",
            lo=0.1,
            hi=0.2,
        )

    def test_a_name_the_document_does_not_hold(self):
        self.refused(
            lambda: admit_range(self.doc, RangeField.param(VarName("nope"))),
            "not_a_continuous_param",
            param="nope",
        )

    def test_a_slot_the_node_does_not_carry(self):
        error = self.refused(
            lambda: certified_range(
                self.doc, RangeField.slot(self.solid, "radius"), max_replays=1
            ),
            "unknown_slot",
            node=self.solid,
            slot="radius",
        )
        self.assertIn("radius", str(error))

    def test_a_word_outside_the_slot_alphabet_is_not_a_field(self):
        with self.assertRaises(ValueError):
            RangeField.slot(self.solid, "no_such_word")

    def test_a_nominal_build_canceled_before_it_starts(self):
        token = CancelToken()
        token.cancel()
        self.refused(
            lambda: RangeSearch(self.doc, DEPTH_FIELD, seed(), cancel=token),
            "stopped",
        )

    def test_a_cancelled_step_keeps_the_answer_and_a_later_step_continues(self):
        search = RangeSearch(self.doc, DEPTH_FIELD, seed())
        self.assertEqual(search.step(), "advanced")
        before = search.answer()
        token = CancelToken()
        token.cancel()
        self.assertEqual(search.step(cancel=token), "stopped")
        self.assertEqual(search.steps, 1)
        self.assertEqual(search.answer(), before)
        self.assertEqual(search.step(), "advanced", "one stop per step: the search resumes")
        self.assertEqual(search.steps, 2)

    def test_the_nominal_builds_cancel_does_not_reach_the_steps(self):
        token = CancelToken()
        search = RangeSearch(self.doc, DEPTH_FIELD, seed(), cancel=token)
        token.cancel()
        self.assertEqual(search.step(), "advanced")


class TestAdmission(unittest.TestCase):
    def setUp(self):
        self.doc, self.solid = prism()

    def test_it_answers_the_axis_and_its_value(self):
        admission = admit_range(self.doc, DEPTH_FIELD, seed())
        self.assertEqual(admission.axis, self.doc.var(VarName("depth")))
        self.assertEqual(admission.nominal.meters, DEPTH)

    def test_a_build_that_failed_refuses_without_building_again(self):
        # A depth of zero is a prism that does not build.
        self.doc.apply(
            DocEdit.set_var_value(VarName("depth"), pncad.FreeValue.length(0 * m))
        )
        built = evaluate(self.doc)
        admit_range(self.doc, DEPTH_FIELD)
        with self.assertRaises(RangeRefusal) as caught:
            admit_range(self.doc, DEPTH_FIELD, built=built)
        self.assertEqual(caught.exception.variant, "witness_does_not_build")
        self.assertEqual(caught.exception.node, self.solid)
        self.assertIsInstance(caught.exception.cause, str)

    def test_an_evaluation_of_another_document_is_not_this_ones_build(self):
        other, _ = prism(Distribution.normal(1 * mm))
        with self.assertRaises(ValueError):
            admit_range(self.doc, DEPTH_FIELD, built=evaluate(other))


if __name__ == "__main__":
    unittest.main()
