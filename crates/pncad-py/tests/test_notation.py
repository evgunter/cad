"""Authored notation, from Python (LIB-B-NOTATION).

A `Length` erases. `25 * mm` is canonical metres and the `mm` is gone
at the multiply, which is what the kernel below wants and what makes
the arithmetic closed — and it is wrong for exactly one thing:
recording what a person TYPED, so a document reads back the way it was
written. `WrittenLength` and `WrittenAngle` are that record, and
`DocParam.written_length` / `written_angle` are the doors that put one
into a document.

THE MEASUREMENT THIS FILE EXISTS TO PIN. Before this unit,
`DocParam.length(25 * mm)` saved `"display_unit": "m"` — the canonical
row, whatever the caller wrote. `test_the_saved_row_names_the_authored_
unit` is that byte, now reading `mm`, and
`test_the_erasing_door_still_erases` is the other half: `length` is
NOT deprecated and still records the canonical row, because a caller
who has a number and no notation should say so rather than pick one.

WHY TWO TYPES AND NOT A UNIT ON `Length`. `quantity::written`'s module
docs rule it: there is no answer to what notation the sum of a
millimetre and an inch is written in, so an authored quantity has no
arithmetic — and `Length` is the type every node constructor takes and
every expression evaluates to, arithmetic and all.
`test_the_authored_pair_has_no_arithmetic` holds that line from
Python.

THE MIS-DIMENSIONED PAIRING IS UNREPRESENTABLE, NOT REFUSED, at the
authoring door: a `WrittenLength` holds a `LengthUnit`, so
`written_length` cannot mint an angle-in-millimetres and there is no
refusal to test. What IS reachable from Python is the same fault
arriving out of a FILE, where the document invariant is checked by the
shared save/load validator — `TestAMisDimensionedRowRefusesAtLoad`,
two arms, the off-table symbol refusing earlier and differently. The
static half of the unrepresentability claim is `ty_fixtures/illegal.py`.

DELIBERATELY NOT ASSERTED HERE. Node-slot literals: `Node.extrude(25 *
mm)` records the canonical row too, and always has. That is not this
family's charter — which is parameter-scoped — and it cannot be fixed
at these doors; it is filed as
`work/lib/node-slot-literals-erase-the-authored-notation.md`.
"""

import json
import unittest

from pncad import (
    AngleUnit,
    Doc,
    DocEdit,
    DocParam,
    DocParamValue,
    Length,
    LengthUnit,
    ParamName,
    PersistError,
    WrittenAngle,
    WrittenLength,
    cm,
    deg,
    inch,
    load,
    m,
    mm,
    pi_rad,
    rad,
)

WIDTH = ParamName("width")


def saved_params(doc):
    """The `params` map of `doc`'s saved snapshot, as parsed JSON.

    The file is the point of this whole family, so the assertions read
    the BYTES rather than the in-memory value wherever the claim is
    about what a document records. The header line carries the id and
    is not JSON, hence the split.
    """
    return json.loads(doc.save().split("\n", 1)[1])["snapshot"]["params"]


def doc_with(name, param):
    doc = Doc()
    doc.apply(DocEdit.set_doc_param(name, param))
    return doc


class TestTheAuthoredPair(unittest.TestCase):
    """`WrittenLength` / `WrittenAngle` themselves, before any document."""

    def test_in_unit_multiplies_and_remembers(self):
        written = WrittenLength.in_unit(25.0, mm)
        self.assertEqual(written.meters, 0.025)
        self.assertEqual(written.unit, mm)
        self.assertEqual(written.length, 25 * mm)

    def test_canonical_in_records_a_notation_without_multiplying(self):
        # The door for a value arrived at by computing: the arithmetic
        # has happened, the number is canonical, and this says which
        # notation to record it in.
        computed = (20 * mm) + (5 * mm)
        written = WrittenLength.canonical_in(computed, mm)
        self.assertEqual(written.meters, 0.025)
        self.assertEqual(written.unit, mm)
        self.assertEqual(written, WrittenLength.in_unit(25.0, mm))

    def test_the_notation_is_never_absent(self):
        # There is no "canonical, notation unknown" state: metres are
        # a notation said out loud, not the lack of one.
        self.assertEqual(WrittenLength.in_unit(0.025, m).unit, m)
        self.assertEqual(WrittenAngle.in_unit(1.0, rad).unit, rad)

    def test_the_angle_mirror(self):
        written = WrittenAngle.in_unit(90.0, deg)
        self.assertAlmostEqual(written.radians, 1.5707963267948966)
        self.assertEqual(written.unit, deg)
        self.assertEqual(written.angle, 90 * deg)
        self.assertEqual(
            WrittenAngle.canonical_in(90 * deg, deg),
            written,
        )

    def test_equality_compares_both_halves(self):
        # The same magnitude authored in two units is two authorings —
        # the opposite of the stored literal this feeds, where the
        # display unit is excluded from expression identity.
        self.assertEqual(WrittenLength.in_unit(25.0, mm).meters,
                         WrittenLength.in_unit(0.025, m).meters)
        self.assertNotEqual(WrittenLength.in_unit(25.0, mm),
                            WrittenLength.in_unit(0.025, m))
        self.assertNotEqual(WrittenAngle.in_unit(90.0, deg),
                            WrittenAngle.in_unit(0.5, pi_rad))

    def test_equal_authorings_hash_together(self):
        pair = {
            WrittenLength.in_unit(25.0, mm): "a",
            WrittenLength.in_unit(25.0, mm): "b",
            WrittenLength.in_unit(0.025, m): "c",
        }
        self.assertEqual(len(pair), 2)
        self.assertEqual(pair[WrittenLength.in_unit(25.0, mm)], "b")

    def test_the_two_spellings_of_zero_are_one_authoring(self):
        # `DocParam`'s already-settled shape, not `Length`'s: the
        # equality is IEEE, so the hash folds to match it.
        self.assertEqual(WrittenLength.in_unit(-0.0, mm),
                         WrittenLength.in_unit(0.0, mm))
        self.assertEqual(hash(WrittenLength.in_unit(-0.0, mm)),
                         hash(WrittenLength.in_unit(0.0, mm)))

    def test_the_authored_pair_has_no_arithmetic(self):
        # The line `quantity::written` draws: there is no answer to
        # what notation the sum of a millimetre and an inch is written
        # in, so the type refuses to have one rather than inventing it.
        a = WrittenLength.in_unit(25.0, mm)
        b = WrittenLength.in_unit(1.0, inch)
        with self.assertRaises(TypeError):
            a + b
        with self.assertRaises(TypeError):
            a * 2.0
        with self.assertRaises(TypeError):
            _ = -a
        # Computing on the `Length` inside is the door that IS open.
        self.assertIsInstance(a.length + b.length, Length)

    def test_the_repr_shows_both_halves(self):
        self.assertEqual(repr(WrittenLength.in_unit(25.0, mm)),
                         "WrittenLength(0.025 m in mm)")
        self.assertEqual(repr(WrittenAngle.in_unit(0.0, deg)),
                         "WrittenAngle(0 rad in deg)")


class TestTheUnitsCompare(unittest.TestCase):
    """Equality on `LengthUnit` / `AngleUnit`.

    Not ornamental: a unit is now something a caller gets BACK, and
    without these the fallback is identity — a unit read off a value
    compares unequal to the `mm` it was written in.
    """

    def test_a_unit_read_back_equals_the_constant(self):
        self.assertEqual(WrittenLength.in_unit(25.0, mm).unit, mm)
        self.assertEqual(WrittenAngle.in_unit(90.0, deg).unit, deg)

    def test_different_rows_are_different_units(self):
        self.assertNotEqual(WrittenLength.in_unit(25.0, mm).unit, cm)
        self.assertNotEqual(WrittenAngle.in_unit(1.0, rad).unit, pi_rad)

    def test_a_unit_is_usable_as_a_dict_key(self):
        tally = {}
        for written in (WrittenLength.in_unit(25.0, mm),
                        WrittenLength.in_unit(3.0, mm),
                        WrittenLength.in_unit(1.0, m)):
            tally[written.unit] = tally.get(written.unit, 0) + 1
        self.assertEqual(tally, {mm: 2, m: 1})

    def test_the_types_do_not_cross(self):
        self.assertIsInstance(WrittenLength.in_unit(1.0, m).unit, LengthUnit)
        self.assertIsInstance(WrittenAngle.in_unit(1.0, rad).unit, AngleUnit)


class TestAParameterRemembersItsNotation(unittest.TestCase):
    def test_the_saved_row_names_the_authored_unit(self):
        # The byte this family exists for. Before this unit the same
        # authoring saved `"display_unit": "m"`.
        doc = doc_with(WIDTH, DocParam.written_length(WrittenLength.in_unit(25.0, mm)))
        row = saved_params(doc)["width"]["Continuous"]
        self.assertEqual(row["display_unit"], "mm")
        self.assertEqual(row["dim"], "Length")
        self.assertEqual(row["value"], 0.025)

    def test_metres_stay_metres(self):
        doc = doc_with(WIDTH, DocParam.written_length(WrittenLength.in_unit(2.0, m)))
        row = saved_params(doc)["width"]["Continuous"]
        self.assertEqual(row["display_unit"], "m")
        self.assertEqual(row["value"], 2.0)

    def test_an_angle_in_degrees(self):
        spin = ParamName("spin")
        doc = doc_with(spin, DocParam.written_angle(WrittenAngle.in_unit(90.0, deg)))
        row = saved_params(doc)["spin"]["Continuous"]
        self.assertEqual(row["display_unit"], "deg")
        self.assertEqual(row["dim"], "Angle")
        self.assertAlmostEqual(row["value"], 1.5707963267948966)

    def test_the_erasing_door_still_erases(self):
        # `length` is not deprecated by `written_length` and does not
        # guess: a caller with a number and no notation says so, and
        # the document records the canonical row.
        doc = doc_with(WIDTH, DocParam.length(25 * mm))
        self.assertEqual(saved_params(doc)["width"]["Continuous"]["display_unit"], "m")

    def test_the_notation_survives_a_save_and_load(self):
        doc = doc_with(WIDTH, DocParam.written_length(WrittenLength.in_unit(25.0, mm)))
        back = load(doc.save()).doc
        self.assertEqual(back.params[WIDTH].unit, "mm")
        self.assertEqual(back.params[WIDTH],
                         DocParam.written_length(WrittenLength.in_unit(25.0, mm)))

    def test_two_notations_of_one_magnitude_are_two_parameters(self):
        # `DocParam`'s equality includes the notation, which is what
        # makes the round trip above a real check rather than a
        # comparison of numbers.
        self.assertNotEqual(
            DocParam.written_length(WrittenLength.in_unit(25.0, mm)),
            DocParam.written_length(WrittenLength.in_unit(0.025, m)),
        )


class TestTheReadDoor(unittest.TestCase):
    """`Doc.params` and `DocParam.unit` — the half that had no door."""

    def test_the_map_answers_every_declared_parameter(self):
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(
            WIDTH, DocParam.written_length(WrittenLength.in_unit(25.0, mm))))
        doc.apply(DocEdit.set_doc_param(
            ParamName("holes"), DocParam.count(4)))
        self.assertEqual(sorted(n.name for n in doc.params),
                         ["holes", "width"])
        self.assertEqual(doc.params[WIDTH].unit, "mm")

    def test_an_empty_document_has_no_parameters(self):
        self.assertEqual(Doc().params, {})

    def test_the_map_is_a_snapshot(self):
        doc = doc_with(WIDTH, DocParam.written_length(WrittenLength.in_unit(25.0, mm)))
        taken = doc.params
        taken.clear()
        self.assertEqual(doc.params[WIDTH].unit, "mm")

    def test_a_scalar_names_the_dimensionless_row(self):
        # The empty symbol, which reads as the absence it is — and is
        # NOT the absence a count reports.
        doc = doc_with(ParamName("k"), DocParam.scalar(2.0))
        self.assertEqual(doc.params[ParamName("k")].unit, "")

    def test_a_count_has_no_notation_at_all(self):
        doc = doc_with(ParamName("holes"), DocParam.count(4))
        self.assertIsNone(doc.params[ParamName("holes")].unit)


class TestTheValueDoorLeavesTheNotationAlone(unittest.TestCase):
    """The unit rides with the DECLARATION, not with the number."""

    def test_a_value_edit_keeps_the_authored_unit(self):
        doc = doc_with(WIDTH, DocParam.written_length(WrittenLength.in_unit(25.0, mm)))
        doc.apply(DocEdit.set_doc_param_value(WIDTH, DocParamValue.length(30 * mm)))
        row = saved_params(doc)["width"]["Continuous"]
        self.assertEqual(row["display_unit"], "mm")
        self.assertEqual(row["value"], 0.03)

    def test_the_create_or_replace_door_restates_it(self):
        # `set_doc_param` is create-or-replace, so it redeclares the
        # notation along with everything else. That is not a bug in
        # the value door; it is why the value door exists.
        doc = doc_with(WIDTH, DocParam.written_length(WrittenLength.in_unit(25.0, mm)))
        doc.apply(DocEdit.set_doc_param(WIDTH, DocParam.length(30 * mm)))
        self.assertEqual(saved_params(doc)["width"]["Continuous"]["display_unit"], "m")


class TestTheTextDoorAlreadyCarriesIt(unittest.TestCase):
    """`Expr::written_length` needs no binding of its own.

    `Doc.parse_expr` is the checking parser: it does the one multiply
    and calls `literal_with_unit`, so the notation crosses on the way
    in and `Expr.text` reads it back. Binding a written-literal
    constructor would be a second spelling of one grammar, which
    `py/expr.rs` already rules out for the whole builder set.
    """

    def test_a_parsed_literal_reads_back_in_the_unit_it_was_written_in(self):
        doc = Doc()
        for source, canonical in (("25 mm", 0.025), ("2 m", 2.0)):
            with self.subTest(source=source):
                expr = doc.parse_expr(source)
                self.assertEqual(expr.text, source)
                self.assertEqual(expr.literal_value, canonical)

    def test_the_two_spellings_of_a_quarter_turn_stay_apart(self):
        doc = Doc()
        by_deg = doc.parse_expr("90 deg")
        by_pi = doc.parse_expr("0.5 pi rad")
        self.assertEqual(by_deg.text, "90 deg")
        self.assertEqual(by_pi.text, "0.5 pi rad")
        self.assertAlmostEqual(by_deg.literal_value, by_pi.literal_value)


class TestAMisDimensionedRowRefusesAtLoad(unittest.TestCase):
    """The pairing the authoring doors cannot mint, arriving from a file.

    `written_length` is total — a `WrittenLength` holds a length unit,
    so the dimension cannot disagree — and the `DocParam` payload is
    not reachable from Python at all. The one way in is a hand-edited
    file, and the shared save/load validator is where the document
    invariant is checked.
    """

    def tampered(self, symbol):
        doc = doc_with(ParamName("spin"),
                       DocParam.written_angle(WrittenAngle.in_unit(90.0, deg)))
        header, body = doc.save().split("\n", 1)
        parsed = json.loads(body)
        parsed["snapshot"]["params"]["spin"]["Continuous"]["display_unit"] = symbol
        return header + "\n" + json.dumps(parsed)

    def test_a_length_unit_on_an_angle_parameter(self):
        with self.assertRaises(PersistError) as raised:
            load(self.tampered("mm"))
        refusal = raised.exception
        self.assertEqual(refusal.variant, "display_unit")
        self.assertIn("spin", str(refusal))
        # The arm's three fields, as payload rather than as prose: the
        # parameter, what the unit measures, and what it was declared.
        self.assertEqual(refusal.name, "spin")
        self.assertEqual(refusal.unit, "length")
        self.assertEqual(refusal.declared, "angle")
        # This arm wraps no refusal of another layer.
        self.assertIsNone(refusal.inner_variant)

    def test_an_off_table_symbol_refuses_earlier_and_differently(self):
        # A different fault: the token is not a row of the table at
        # all, so it never reaches the dimension walk.
        with self.assertRaises(PersistError) as raised:
            load(self.tampered("furlong"))
        self.assertEqual(raised.exception.variant, "unreadable")


if __name__ == "__main__":
    unittest.main()
