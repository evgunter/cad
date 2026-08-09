"""The §L4 typed-quantity surface.

Stdlib `unittest` on purpose: this box has no pip, and a scaffold that
can only be tested after installing a test runner is a scaffold that
does not get tested.
"""

import math
import unittest

import pncad
from pncad import Angle, Count, DimensionError, Length, cm, deg, inch, m, mm, rad


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
