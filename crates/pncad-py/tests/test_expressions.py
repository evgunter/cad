"""The expression READ side (LIB-B-EXPR-READ).

A recipe slot is not always a number. `width / 2.0 - margin` is an
ordinary thing for one to hold, and until this unit Python could
declare the parameters that expression reads and had no way to ask
what it was WORTH — the façade's own words for the absence are "a
panel that shows a slot before editing it needs this", and
`Expr.literal_value` answers only for a bare literal.

Three doors close it, all on `Doc` because all three read a
per-document table: `parse_expr` reads the declared DIMENSIONS,
`eval` and `eval_count` the bound VALUES. These are the door-level
rows for them and, above all, for their REFUSALS — a text that is not
an expression says WHERE it stopped being one, and a value that
cannot be computed says which parameter is missing rather than
displaying a blank.

The oracles here are real arithmetic on real documents. Nothing
asserts a message; every refusal is checked by its stable tag and its
payload, which is the contract.
"""

import math
import unittest

from pncad import (
    Angle,
    ArcSide,
    Doc,
    DocEdit,
    DocParam,
    DocParamValue,
    EvalError,
    Expr,
    Length,
    ParamName,
    ParseError,
    PncadError,
    Radius,
    deg,
    m,
    mm,
    rad,
)


def plate(width=0.1 * m, margin=3 * mm, holes=4):
    """A document declaring the three parameters these rows read.

    Deliberately a bare parameter table rather than a modelled part:
    what is under test is the expression layer, and geometry would
    make every row depend on the evaluator that is NOT this one.
    """
    doc = Doc("expression-rows")
    doc.apply(DocEdit.set_doc_param(ParamName("width"), DocParam.length(width)))
    doc.apply(DocEdit.set_doc_param(ParamName("margin"), DocParam.length(margin)))
    doc.apply(DocEdit.set_doc_param(ParamName("holes"), DocParam.count(holes)))
    return doc


class TestTheTextDoorBuildsCheckedTrees(unittest.TestCase):
    def setUp(self):
        self.doc = plate()

    def test_an_expression_knows_what_it_measures(self):
        self.assertEqual(self.doc.parse_expr("width").dimension, "length")
        self.assertEqual(self.doc.parse_expr("30 deg").dimension, "angle")
        self.assertEqual(self.doc.parse_expr("holes").dimension, "count")
        self.assertEqual(self.doc.parse_expr("2.5").dimension, "scalar")

    def test_the_declarations_come_from_the_document(self):
        """A bare identifier is a reference to THIS document's
        parameter, carrying the dimension it was declared with."""
        self.assertEqual(self.doc.parse_expr("width").dimension, "length")
        self.assertEqual(self.doc.parse_expr("holes").dimension, "count")
        # The same text against a document that declares nothing.
        with self.assertRaises(ParseError) as caught:
            Doc("empty").parse_expr("width")
        self.assertEqual(caught.exception.variant, "unknown_param")
        self.assertEqual(caught.exception.name, "width")

    def test_the_whole_algebra_is_reachable_through_one_call(self):
        """The parser runs every smart constructor, so one door
        reaches the operators, the functions and the unit suffixes."""
        for source in [
            "width / 2.0 - margin",
            "min(width, 2.0 * margin)",
            "max(width, margin)",
            "sin(30 deg) * width",
            "atan2(width, width)",
            "scalar(holes) * margin",
            "-width",
            "(width + margin) / 2.0",
        ]:
            with self.subTest(source=source):
                self.assertIsInstance(self.doc.parse_expr(source), Expr)

    def test_a_tree_reads_back_as_text_that_parses_to_itself(self):
        """`unparse` is `parse_expr`'s inverse — the door outward, and
        the round trip is what makes it one."""
        for source in ["width / 2.0 - margin", "sin(30 deg)", "holes + 1"]:
            with self.subTest(source=source):
                once = self.doc.parse_expr(source)
                twice = self.doc.parse_expr(once.text)
                self.assertEqual(once, twice)
                self.assertEqual(once.text, twice.text)

    def test_the_text_is_a_rendering_and_not_the_original_string(self):
        """Whitespace and redundant parentheses are the parser's to
        normalise, which is why the docstring calls `text` a rendering
        — a panel that expects its own bytes back is wrong about the
        door."""
        spaced = self.doc.parse_expr("  width   /   2.0  ")
        self.assertEqual(spaced.text, self.doc.parse_expr("width / 2.0").text)

    def test_an_expression_names_the_parameters_it_reads(self):
        """Sorted, deduplicated, and the fact a consumer needs to know
        when a value it displayed has gone stale."""
        expr = self.doc.parse_expr("width / 2.0 - margin + width")
        self.assertEqual([p.name for p in expr.params], ["margin", "width"])
        self.assertEqual(self.doc.parse_expr("1 m + 2 m").params, [])

    def test_a_bare_literal_answers_its_number_and_nothing_else_does(self):
        """`literal_value` is the narrow door, and the case it does
        NOT answer is the one that made the evaluator's absence
        bite."""
        self.assertEqual(self.doc.parse_expr("25 mm").literal_value, 0.025)
        self.assertIsNone(self.doc.parse_expr("width / 2.0").literal_value)
        # A count literal answers None: handing an exact integer back
        # as a float is the implicit promotion the language refuses.
        self.assertIsNone(self.doc.parse_expr("4").literal_value)

    def test_equality_is_the_trees_and_a_tree_is_unhashable(self):
        self.assertEqual(
            self.doc.parse_expr("width + margin"),
            self.doc.parse_expr("width + margin"),
        )
        self.assertNotEqual(
            self.doc.parse_expr("width + margin"),
            self.doc.parse_expr("margin + width"),
        )
        with self.assertRaises(TypeError):
            {self.doc.parse_expr("width")}


class TestTheTextDoorRefusesTyped(unittest.TestCase):
    def setUp(self):
        self.doc = plate()

    def refusal(self, source):
        with self.assertRaises(ParseError) as caught:
            self.doc.parse_expr(source)
        return caught.exception

    def test_the_class_is_a_pncad_error(self):
        self.assertTrue(issubclass(ParseError, PncadError))

    def test_every_refusal_says_where_it_stopped(self):
        """`pos` is on every arm, because for a parser the position IS
        the recourse."""
        for source in [
            "1 m $ 2",
            "1 m +",
            "(1 m 2 m)",
            "1 m 2 m",
            "1 furlong",
            "hypot(1, 2)",
            "sin(1 rad, 2 rad)",
            "height",
            "1 m + 1 rad",
        ]:
            with self.subTest(source=source):
                err = self.refusal(source)
                self.assertIsInstance(err.pos, int)
                self.assertGreaterEqual(err.pos, 0)
                self.assertLessEqual(err.pos, len(source))

    def test_the_arms_carry_their_own_payload(self):
        outside = self.refusal("1 m $ 2")
        self.assertEqual(outside.variant, "unexpected_char")
        self.assertEqual(outside.char, "$")

        ends = self.refusal("1 m +")
        self.assertEqual(ends.variant, "unexpected_end")
        self.assertIsNotNone(ends.expected)

        trailing = self.refusal("1 m 2 m")
        self.assertEqual(trailing.variant, "trailing_input")
        self.assertIsNotNone(trailing.found)

        unit = self.refusal("1 furlong")
        self.assertEqual(unit.variant, "unknown_unit")
        self.assertEqual(unit.symbol, "furlong")

        unknown = self.refusal("hypot(1, 2)")
        self.assertEqual(unknown.variant, "unknown_function")
        self.assertEqual(unknown.name, "hypot")

        arity = self.refusal("sin(1 rad, 2 rad)")
        self.assertEqual(arity.variant, "wrong_arity")
        self.assertEqual(arity.name, "sin")
        self.assertEqual(arity.arity, 1)
        self.assertEqual(arity.given, 2)

        param = self.refusal("height")
        self.assertEqual(param.variant, "unknown_param")
        self.assertEqual(param.name, "height")

    def test_a_dimension_mismatch_refuses_here_with_its_position(self):
        """The routing decision this unit made, executed.

        The text door runs every smart constructor, so an
        ill-dimensioned reduction is caught DURING the parse — and it
        arrives as `ParseError` rather than `LiteralError` because
        `pos` is the fact that says where to edit and `LiteralError`
        has nowhere to put it. The inner refusal is not lost: its own
        tag rides as `kind`."""
        err = self.refusal("1 m + 1 rad")
        self.assertEqual(err.variant, "dimension")
        self.assertEqual(err.kind, "mismatch")
        self.assertGreater(err.pos, 0)

        # The other reductions the checker refuses, each keeping its
        # own inner tag.
        for source, kind in [
            ("1 m * 1 m", "mul_needs_scalar"),
            ("1 m / 1 rad", "div_needs_scalar_divisor"),
            ("sin(1 m)", "trig_needs_angle"),
            ("scalar(1 m)", "not_count"),
            ("width / holes", "count_needs_explicit_promotion"),
        ]:
            with self.subTest(source=source):
                refused = self.refusal(source)
                self.assertEqual(refused.variant, "dimension")
                self.assertEqual(refused.kind, kind)

    def test_every_payload_field_is_present_on_every_arm(self):
        """`getattr` never raises, so a caller reads the payload
        without first branching on `variant` — the shape
        `ReadbackError` and `AssemblyError` already have."""
        fields = [
            "variant",
            "pos",
            "char",
            "expected",
            "found",
            "text",
            "symbol",
            "name",
            "arity",
            "given",
            "kind",
        ]
        for source in ["1 m $ 2", "height", "1 m + 1 rad", "sin(1 rad, 2 rad)"]:
            err = self.refusal(source)
            for field in fields:
                with self.subTest(source=source, field=field):
                    self.assertTrue(hasattr(err, field))


class TestTheEvaluatorAnswersValues(unittest.TestCase):
    def setUp(self):
        self.doc = plate()

    def test_a_length_expression_answers_a_length(self):
        """Dimensioned out — the crossing rule, not the kernel's
        unit-erased float."""
        value = self.doc.eval(self.doc.parse_expr("width / 2.0 - margin"))
        self.assertIsInstance(value, Length)
        # The oracle is the arithmetic itself: 0.1/2 - 0.003, in
        # metres, with no rounding step anywhere in the evaluator.
        self.assertEqual(value.in_unit(m), 0.1 / 2.0 - 0.003)

    def test_an_angle_expression_answers_an_angle(self):
        value = self.doc.eval(self.doc.parse_expr("atan2(1 m, 1 m)"))
        self.assertIsInstance(value, Angle)
        self.assertAlmostEqual(value.in_unit(rad), math.pi / 4)
        self.assertAlmostEqual(value.in_unit(deg), 45.0)

    def test_a_dimensionless_expression_answers_a_bare_float(self):
        value = self.doc.eval(self.doc.parse_expr("sin(30 deg)"))
        self.assertIsInstance(value, float)
        self.assertAlmostEqual(value, 0.5)

    def test_a_count_expression_answers_an_exact_integer(self):
        self.assertEqual(self.doc.eval_count(self.doc.parse_expr("holes")), 4)
        self.assertEqual(self.doc.eval_count(self.doc.parse_expr("holes * 3")), 12)
        self.assertEqual(self.doc.eval_count(self.doc.parse_expr("-holes")), -4)

    def test_the_value_follows_the_document_and_not_the_expression(self):
        """The point of the whole family: one expression, and its
        value moves when the parameter does."""
        expr = self.doc.parse_expr("width / 2.0")
        self.assertEqual(self.doc.eval(expr).in_unit(m), 0.05)
        self.doc.apply(
            DocEdit.set_doc_param_value(
                ParamName("width"), DocParamValue.length(0.2 * m)
            )
        )
        self.assertEqual(self.doc.eval(expr).in_unit(m), 0.1)

    def test_an_expression_parsed_elsewhere_evaluates_here(self):
        """An `Expr` is a plain value carrying the dimensions its refs
        were declared with, so it travels between documents that agree
        about them."""
        other = plate(width=0.4 * m, margin=1 * mm, holes=2)
        expr = self.doc.parse_expr("width - margin")
        self.assertEqual(other.eval(expr).in_unit(m), 0.4 - 0.001)

    def test_a_count_promotes_only_where_the_expression_says_so(self):
        promoted = self.doc.parse_expr("scalar(holes) * margin")
        self.assertEqual(promoted.dimension, "length")
        self.assertAlmostEqual(self.doc.eval(promoted).in_unit(mm), 12.0)

    def test_evaluating_changes_nothing(self):
        before = self.doc.save()
        self.doc.eval(self.doc.parse_expr("width / 2.0"))
        self.doc.eval_count(self.doc.parse_expr("holes"))
        self.assertEqual(self.doc.save(), before)


class TestTheEvaluatorRefusesTyped(unittest.TestCase):
    def setUp(self):
        self.doc = plate()

    def test_the_class_is_a_pncad_error(self):
        self.assertTrue(issubclass(EvalError, PncadError))

    def test_a_parameter_with_no_binding_names_itself(self):
        """The absence the façade calls out: a slot whose value cannot
        be computed says which parameter is missing rather than
        displaying a blank."""
        expr = self.doc.parse_expr("width / 2.0")
        with self.assertRaises(EvalError) as caught:
            Doc("empty").eval(expr)
        self.assertEqual(caught.exception.variant, "unknown_param")
        self.assertEqual(caught.exception.name, "width")

    def test_a_redeclared_parameter_says_both_dimensions(self):
        """The expression's reference recorded a length; this document
        declares the same name as a count."""
        expr = self.doc.parse_expr("width")
        counts = Doc("counts")
        counts.apply(DocEdit.set_doc_param(ParamName("width"), DocParam.count(3)))
        with self.assertRaises(EvalError) as caught:
            counts.eval(expr)
        self.assertEqual(caught.exception.variant, "param_dimension_mismatch")
        self.assertEqual(caught.exception.name, "width")
        self.assertEqual(caught.exception.expected, "length")
        self.assertEqual(caught.exception.found, "count")

    def test_the_two_doors_cannot_be_confused(self):
        """Counts are exact and continuous values are not, so each
        door refuses the other's expression by name."""
        with self.assertRaises(EvalError) as continuous:
            self.doc.eval(self.doc.parse_expr("holes"))
        self.assertEqual(
            continuous.exception.variant, "count_expr_in_continuous_eval"
        )

        with self.assertRaises(EvalError) as exact:
            self.doc.eval_count(self.doc.parse_expr("width"))
        self.assertEqual(exact.exception.variant, "continuous_expr_in_count_eval")
        self.assertEqual(exact.exception.found, "length")

    def test_a_count_that_cannot_promote_exactly_refuses(self):
        with self.assertRaises(EvalError) as caught:
            self.doc.eval(self.doc.parse_expr("scalar(9999999999)"))
        self.assertEqual(caught.exception.variant, "count_to_scalar_out_of_range")
        self.assertEqual(caught.exception.count, 9999999999)

    def test_a_pole_is_caught_on_the_value_and_not_at_the_operation(self):
        """Division by zero is NOT a refusal in the expression layer —
        the evaluator has no branches to hide it behind — so the
        poison flows through the arithmetic and is refused at the
        boundary, on the finished value."""
        pole = self.doc.parse_expr("width / 0.0")
        with self.assertRaises(EvalError) as caught:
            self.doc.eval(pole)
        self.assertEqual(caught.exception.variant, "non_finite_result")

    def test_every_payload_field_is_present_on_every_arm(self):
        fields = ["variant", "name", "expected", "found", "count"]
        cases = [
            (Doc("empty").eval, self.doc.parse_expr("width")),
            (self.doc.eval, self.doc.parse_expr("holes")),
            (self.doc.eval_count, self.doc.parse_expr("width")),
            (self.doc.eval, self.doc.parse_expr("width / 0.0")),
        ]
        for door, expr in cases:
            with self.assertRaises(EvalError) as caught:
                door(expr)
            for field in fields:
                with self.subTest(expr=expr.text, field=field):
                    self.assertTrue(hasattr(caught.exception, field))


class TestTheAuthoringHalfIsStillClosed(unittest.TestCase):
    """G1's residue, at the doors this unit deliberately did not open.

    The READ side crossing does not make a parametric radius sayable,
    and saying so here — beside the doors that work — is what keeps
    the two halves from being confused for each other.
    """

    def test_an_arc_radius_is_a_length_and_not_an_expression(self):
        radius = plate().parse_expr("width / 2.0")
        self.assertEqual(radius.dimension, "length")
        with self.assertRaises(TypeError):
            Radius(radius, ArcSide.Left)

    def test_a_parameter_is_a_number_and_not_an_expression(self):
        derived = plate().parse_expr("width / 2.0")
        with self.assertRaises(TypeError):
            DocParam.length(derived)


if __name__ == "__main__":
    unittest.main()
