"""The §L4 typed-quantity surface.

Stdlib `unittest` on purpose: this box has no pip, and a scaffold that
can only be tested after installing a test runner is a scaffold that
does not get tested.
"""

import math
import unittest

import pncad
from pncad import (
    Angle,
    Count,
    DimensionError,
    Doc,
    FmtQuantityError,
    Length,
    cm,
    deg,
    inch,
    m,
    mm,
    pi_rad,
    rad,
)

LENGTH_UNITS = (mm, cm, m, inch)
ANGLE_UNITS = (deg, rad, pi_rad)


def reads_back_as(text):
    """The canonical value `text` parses to, through the REAL parser.

    `Doc.parse_expr` plus `Doc.eval` is the door the formatter's pin is
    stated against — "`parse(fmt(x, unit))` recovers `x` bit-exactly,
    where `parse` is the expression text parser's literal semantics"
    (`crates/quantity/src/fmt.rs`). Rust's own round-trip test cannot
    use it: `quantity` sits BELOW `editor-core`, so its fixture
    transliterates the parser's literal rule by hand
    (`crates/quantity/src/tests.rs::parse_back` — parse the decimal,
    multiply by the factor). Python is above both and can call the
    parser itself, so the pin is checked here against the thing it
    names rather than against a restatement of it.

    A document with no declared parameters is the right environment:
    formatter output is always a bare literal, and an env that could
    bind a name would let a fixture pass on something other than the
    literal it meant to check.
    """
    doc = Doc()
    value = doc.eval(doc.parse_expr(text))
    if isinstance(value, Length):
        return value.meters
    if isinstance(value, Angle):
        return value.radians
    raise AssertionError(f"{text!r} did not read back as a quantity: {value!r}")


class TestConstruction(unittest.TestCase):
    def test_scalar_times_unit_builds_a_quantity(self):
        # The headline §L4 spelling.
        self.assertIsInstance(25 * mm, Length)
        self.assertIsInstance(90 * deg, Angle)

    def test_canonical_units_are_metres_and_radians(self):
        self.assertEqual((25 * mm).meters, 0.025)
        self.assertEqual((1 * m).meters, 1.0)
        self.assertEqual((1 * cm).meters, 0.01)
        self.assertEqual((1 * inch).meters, 0.0254)
        self.assertEqual((1 * rad).radians, 1.0)
        self.assertAlmostEqual((180 * deg).radians, math.pi, places=15)

    def test_multiplication_commutes(self):
        self.assertEqual((25 * mm).meters, (mm * 25).meters)

    def test_round_trip_through_a_unit(self):
        self.assertAlmostEqual((25 * mm).in_unit(mm), 25.0, places=12)
        self.assertAlmostEqual((90 * deg).in_unit(deg), 90.0, places=12)


class TestArithmetic(unittest.TestCase):
    """Exactly `crates/quantity`'s infallible subset — no more."""

    def test_same_dimension_add_and_subtract(self):
        self.assertEqual(((2 * m) + (3 * m)).meters, 5.0)
        self.assertEqual(((3 * m) - (2 * m)).meters, 1.0)
        self.assertEqual((-(2 * m)).meters, -2.0)

    def test_scalar_scaling_and_division(self):
        self.assertEqual(((2 * m) * 3).meters, 6.0)
        self.assertEqual((3 * (2 * m)).meters, 6.0)
        self.assertEqual(((6 * m) / 3).meters, 2.0)

    def test_ordering(self):
        self.assertTrue((1 * m) < (2 * m))
        self.assertTrue((2 * m) >= (2 * m))
        self.assertEqual(1 * m, 100 * cm)


class TestTypedDimensionErrors(unittest.TestCase):
    """Rust refuses these at compile time; Python raises them, TYPED."""

    def test_length_plus_angle_raises_typed_error(self):
        with self.assertRaises(DimensionError) as caught:
            _ = (1 * m) + (1 * rad)
        err = caught.exception
        # The payload is ATTRIBUTES, not a parsed message (§L4).
        self.assertEqual(err.op, "+")
        self.assertEqual(err.left, "length")
        self.assertEqual(err.right, "angle")

    def test_length_plus_bare_number_names_the_scalar_dimension(self):
        with self.assertRaises(DimensionError) as caught:
            _ = (1 * m) + 1.0
        self.assertEqual(caught.exception.right, "scalar")

    def test_quantity_times_quantity_is_refused(self):
        # `quantity` has no `Mul<Self>`; an area type would be an
        # invention, so this is a refusal rather than a new dimension.
        with self.assertRaises(DimensionError) as caught:
            _ = (2 * m) * (3 * m)
        self.assertEqual(caught.exception.op, "*")

    def test_dimension_errors_are_pncad_errors(self):
        self.assertTrue(issubclass(DimensionError, pncad.PncadError))

    def test_a_foreign_operand_is_a_plain_type_error(self):
        # Not dimensionally wrong — genuinely undefined.
        with self.assertRaises(TypeError):
            _ = (1 * m) + "banana"


class TestDisplayFormatter(unittest.TestCase):
    """LIB-B-FORMAT: `Length.format` / `Angle.format`, the D6 display
    formatter, and the pin that is the whole reason it exists.

    The oracle throughout is `Doc.parse_expr` + `Doc.eval` — doors that
    already ship — rather than a table of expected strings. A string
    table would pin what the formatter happens to print today; the pin
    the module actually makes is a relationship between the two doors,
    and only one of them is the formatter.
    """

    def test_the_headline_spellings(self):
        # The four strings the design docs use, so a reader can see
        # what the door answers before reading the pin below.
        self.assertEqual((25 * mm).format(mm), "25 mm")
        self.assertEqual((0.25 * m).format(m), "0.25 m")
        self.assertEqual((1 * inch).format(inch), "1 in")
        self.assertEqual((90 * deg).format(deg), "90 deg")
        # A bare integral form, not `250.0` — the stripped spelling
        # parses to the identical f64 and matches the authored shape.
        self.assertEqual((0.25 * m).format(mm), "250 mm")
        self.assertEqual((-0.25 * m).format(mm), "-250 mm")
        self.assertEqual((0 * mm).format(mm), "0 mm")

    def test_the_suffix_is_the_units_own_symbol(self):
        # `symbol` is the oracle, not a literal: `inch` is bound under
        # a name Python could spell and the TABLE says `in`, so a
        # hand-written expectation here would be pinning the binding's
        # rename rather than the formatter's suffix.
        for unit in LENGTH_UNITS:
            self.assertTrue((1 * unit).format(unit).endswith(" " + unit.symbol))
        for unit in ANGLE_UNITS:
            self.assertTrue((1 * unit).format(unit).endswith(" " + unit.symbol))
        # Including the two-word one, which is the case a naive
        # `split()` on the text would get wrong.
        self.assertEqual(pi_rad.symbol, "pi rad")
        self.assertEqual((1 * pi_rad).format(pi_rad), "1 pi rad")

    def test_formatted_text_reads_back_to_the_exact_same_bits(self):
        """THE PIN, checked through the parser it is stated against.

        `parse(fmt(x, u)) == x` bit-exactly, for every unit and for
        values authored in the unit, arrived at by arithmetic, and
        taken from the awkward end of the range. Bit equality, not
        `assertAlmostEqual`: the module's claim is about the exact
        f64, and a tolerance would pass on a formatter that quietly
        rounded.
        """
        for unit in LENGTH_UNITS:
            for magnitude in (0.0, 1.0, 25.0, -3.5, 1e-9, 1e9, 0.1, 2.0 / 3.0):
                value = magnitude * unit
                text = value.format(unit)
                self.assertEqual(
                    reads_back_as(text).hex(),
                    value.meters.hex(),
                    f"{text!r} did not read back to the length it came from",
                )
        for unit in ANGLE_UNITS:
            for magnitude in (0.0, 1.0, 90.0, -0.25, 1e-9, 2.0 / 3.0):
                value = magnitude * unit
                text = value.format(unit)
                self.assertEqual(
                    reads_back_as(text).hex(),
                    value.radians.hex(),
                    f"{text!r} did not read back to the angle it came from",
                )

    def test_a_length_computed_from_others_still_reads_back_exactly(self):
        # The case the pin exists for: not a number a person typed,
        # but one the arithmetic produced, which is where a naive
        # `x / factor` rendering loses bits.
        for value in (
            (80 * mm) - (3 * mm) / 7.0,
            (1 * inch) + (1 * mm),
            ((2 * m) / 3.0) * 1.7,
        ):
            for unit in LENGTH_UNITS:
                text = value.format(unit)
                self.assertEqual(reads_back_as(text).hex(), value.meters.hex(), text)

    def test_in_unit_is_the_door_this_one_replaces_and_does_not_round_trip(self):
        """The charter's claim, as an assertion rather than as prose.

        The census charters B-FORMAT as making digit-and-symbol choice
        stop being "hand-work Python redoes beside `Length.in_unit`'s
        bare float". The hand-work is `f"{x.in_unit(u)} {u.symbol}"`,
        and here is a value where it loses bits and `format` does not
        — so the family is closed against a measured difference rather
        than against a preference.
        """
        # A value with no preimage in millimetres: multiplication by
        # the mm factor steps over it, so no decimal number of
        # millimetres reads back to these bits. Found the way
        # `crates/quantity/src/tests.rs` finds one — walk quotients up
        # from 1024 until `d * factor` jumps two ulps — and pinned as
        # a constant so the test states a fact instead of searching
        # for one.
        skipped = 1.0240000000000047
        value = skipped * m
        self.assertEqual(value.meters.hex(), skipped.hex())

        by_hand = f"{value.in_unit(mm)} {mm.symbol}"
        self.assertEqual(by_hand, "1024.0000000000048 mm")
        self.assertNotEqual(
            reads_back_as(by_hand).hex(),
            value.meters.hex(),
            "this value was chosen because the hand-written form loses "
            "bits; if it stopped doing so, pick another",
        )

        # `format` keeps the bits — by falling back to metres, which is
        # the trade the module docs rule on: never a wrong bit,
        # sometimes a canonical suffix.
        text = value.format(mm)
        self.assertEqual(text, "1.0240000000000047 m")
        self.assertEqual(reads_back_as(text).hex(), value.meters.hex())

    def test_the_canonical_fallback_is_visible_in_the_text(self):
        # A caller who needs to know WHICH unit they got reads the
        # suffix; nothing about the call promises the asked-for one.
        # A value authored in the unit always keeps it.
        self.assertTrue((25 * mm).format(mm).endswith(" mm"))
        self.assertTrue((1.0240000000000047 * m).format(mm).endswith(" m"))

    def test_the_same_angle_reads_back_in_the_notation_it_is_asked_for(self):
        # The half-turn row's whole reason. A quarter turn said two
        # ways is the same canonical radians to within an ulp and
        # neither spelling is exact, so nothing but the unit asked for
        # distinguishes them — which is why the unit is a per-call
        # argument rather than derived from the number.
        in_pi = 0.5 * pi_rad
        in_deg = 90 * deg
        self.assertLess(abs(in_pi.radians - in_deg.radians), 1e-15)
        self.assertEqual(in_pi.format(pi_rad), "0.5 pi rad")
        self.assertEqual(in_deg.format(deg), "90 deg")

    def test_a_non_finite_quantity_has_no_display_form(self):
        for poison in (float("nan"), float("inf"), float("-inf")):
            with self.assertRaises(FmtQuantityError) as caught:
                (poison * mm).format(mm)
            err = caught.exception
            self.assertEqual(err.variant, "non_finite")
            self.assertTrue(math.isnan(err.value) or math.isinf(err.value))
            with self.assertRaises(FmtQuantityError):
                (poison * deg).format(deg)

    def test_the_refusal_is_in_the_pncad_hierarchy_and_carries_prose(self):
        with self.assertRaises(FmtQuantityError) as caught:
            (float("inf") * m).format(m)
        self.assertTrue(issubclass(FmtQuantityError, pncad.PncadError))
        # The message is the kernel's own Display, and it points where
        # the fix is (upstream) rather than repeating the tag.
        self.assertIn("no display form", str(caught.exception))
        self.assertNotIn("NonFinite", str(caught.exception))

    def test_poison_is_constructible_which_is_why_the_refusal_is_typed(self):
        # `crates/quantity/src/lib.rs` says the newtypes refuse no
        # float; this is that sentence, from the Python side. If a
        # constructor ever starts refusing, this goes red and the
        # refusal class above needs revisiting.
        self.assertTrue(math.isnan((float("nan") * mm).meters))
        self.assertTrue(math.isinf((float("inf") * deg).radians))

    def test_format_will_not_take_the_other_dimensions_unit(self):
        # The reason the door hangs off the quantity rather than
        # arriving as a free `fmt_length(float, unit)`: the receiver
        # already knows what it measures, so the mis-pairing has no
        # spelling. `ty` pins the static half in the fixtures.
        with self.assertRaises(TypeError):
            (1 * m).format(deg)
        with self.assertRaises(TypeError):
            (1 * rad).format(mm)

    def test_signed_zero_displays_apart_while_comparing_equal(self):
        """A pin on a relationship, not a preference — see the banked
        item `the-quantity-boundary-compares-and-hashes-as-if-poison-
        and-signed-zero-cannot-arrive`.

        `format` is the door that is RIGHT here: its pin is about the
        exact bits, and `-0.0` and `0.0` are different bits. `==` is
        looser and `__hash__` disagrees with `==` — which is the
        defect, recorded rather than fixed in this unit, and pinned as
        it stands so a fix goes red instead of silent.
        """
        below, above = -0.0 * m, 0.0 * m
        self.assertEqual(below, above)
        self.assertEqual(below.format(m), "-0 m")
        self.assertEqual(above.format(m), "0 m")
        self.assertEqual(reads_back_as("-0 m").hex(), (-0.0).hex())
        # The data-model violation, as it stands today.
        self.assertNotEqual(hash(below), hash(above))

    def test_comparing_a_non_finite_quantity_raises_today(self):
        """Also pinned as it stands, and also banked, not fixed.

        `==` goes through the same `partial_cmp` the orderings do, so
        two NaN lengths RAISE where a bare `float` answers `False`.
        The comment at that arm used to claim it was unreachable; this
        is the counter-example, and the pin means whoever fixes it has
        to come here and say so.
        """
        poison = float("nan") * mm
        with self.assertRaises(ValueError):
            poison == poison  # noqa: B015
        with self.assertRaises(ValueError):
            poison < (1 * m)  # noqa: B015
        # A bare float, for contrast — the answer the quantity does
        # not give.
        self.assertFalse(float("nan") == float("nan"))


class TestCount(unittest.TestCase):
    def test_count_has_no_arithmetic(self):
        # Mirrors `quantity::Count`, which implements none: D4's
        # checked count algebra lives in the expression layer.
        two = Count(2)
        self.assertEqual(two.value, 2)
        with self.assertRaises(TypeError):
            _ = two + two


if __name__ == "__main__":
    unittest.main()
