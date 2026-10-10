"""A slot holds a variable (INTENT-LITERALS PR C; the spec's §8 row 15).

A value written at a slot is a variable of its own, anonymous: the
edit mints it, and `Doc.slot` reads it back as a `Var`. A named `Var`
passed to two slots is one variable both read, which is how two slots
share a value; an unnamed one has exactly one reader, so a second
refuses until it is named (VR2). Two values typed alike are two
variables. A slot takes a `Var`, a `Formula`, or a value, written or
bare (Q9).
"""

import unittest

from pncad import (
    Doc,
    DocEdit,
    EditError,
    EvalError,
    Formula,
    FreeVar,
    Node,
    Var,
    VarName,
    WrittenLength,
    evaluate,
    m,
    mm,
)

from test_slot_edits import blank


def square(doc):
    """A unit square on the document's sketch frame."""
    return doc.insert(
        Node.polygon(
            [(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)],
            plane=doc.sketch_frame(),
        )
    )


def volume(doc, node):
    return evaluate(doc).value(node).body().mass_properties().volume


class TestASlotHoldsAVariable(unittest.TestCase):
    def test_a_written_value_mints_one_anonymous_variable(self):
        doc = Doc("slot-variables-written")
        profile = square(doc)
        before = len(doc.vars)
        box = doc.insert(Node.extrude(profile, WrittenLength.in_unit(250.0, mm)))
        self.assertEqual(len(doc.vars), before + 1, "one variable, the depth's own")
        var = doc.slot(box, "distance")
        self.assertIsInstance(var, Var)
        self.assertIn(var, doc.vars)
        self.assertIsNone(doc.var_name(var), "it is anonymous")
        self.assertAlmostEqual(volume(doc, box), 0.25)

    def test_one_var_passed_to_two_slots_is_read_by_both(self):
        doc = Doc("slot-variables-shared")
        doc.apply(DocEdit.declare_var(VarName("depth"), FreeVar.length(0.5 * m)))
        depth = doc.var(VarName("depth"))
        a = doc.insert(Node.extrude(square(doc), depth))
        b = doc.insert(Node.extrude(square(doc), depth))
        self.assertEqual(doc.slot(a, "distance"), depth)
        self.assertEqual(doc.slot(a, "distance"), doc.slot(b, "distance"))

    def test_an_unnamed_var_passed_twice_refuses_until_it_is_named(self):
        doc = Doc("slot-variables-unnamed-twice")
        a = doc.insert(Node.extrude(square(doc), 0.5 * m))
        depth = doc.slot(a, "distance")
        self.assertIsNone(doc.var_name(depth), "the premise: it is unnamed")
        profile = square(doc)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.extrude(profile, depth))
        self.assertEqual(caught.exception.variant, "shared_var_needs_name")
        self.assertIsNone(caught.exception.param, "it has no name to speak")
        doc.apply(DocEdit.rename_var(depth, VarName("depth")))
        b = doc.insert(Node.extrude(profile, depth))
        self.assertEqual(doc.slot(b, "distance"), depth)
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.rename_var(depth, None))
        self.assertEqual(caught.exception.variant, "shared_var_needs_name")
        self.assertEqual(caught.exception.param, "depth")

    def test_two_values_typed_alike_are_two_variables(self):
        doc = Doc("slot-variables-typed")
        a = doc.insert(Node.extrude(square(doc), 0.5 * m))
        b = doc.insert(Node.extrude(square(doc), 0.5 * m))
        self.assertNotEqual(doc.slot(a, "distance"), doc.slot(b, "distance"))

    def test_a_slot_takes_every_form_q9_names(self):
        doc = Doc("slot-variables-forms")
        for depth in (
            0.5 * m,
            WrittenLength.in_unit(500.0, mm),
            Formula.length_in(0.5, m),
        ):
            box = doc.insert(Node.extrude(square(doc), depth))
            self.assertAlmostEqual(volume(doc, box), 0.5)
        # A bare `float` is a dimensionless number, which a length slot
        # refuses at the door, naming both dimensions.
        with self.assertRaises(EditError) as caught:
            Node.extrude(square(doc), 0.5)
        self.assertEqual(caught.exception.variant, "slot_dimension_mismatch")

    def test_a_slots_value_is_its_variable_evaluated(self):
        doc = Doc("slot-variables-eval")
        doc.apply(DocEdit.declare_var(VarName("w"), FreeVar.length(0.25 * m)))
        typed = doc.insert(Node.extrude(square(doc), 0.5 * m))
        defined = doc.insert(
            Node.extrude(square(doc), doc.parse_formula("w * 3.0"))
        )
        self.assertAlmostEqual(doc.eval(doc.slot(typed, "distance")).meters, 0.5)
        self.assertAlmostEqual(doc.eval(doc.slot(defined, "distance")).meters, 0.75)
        stranger = Doc("slot-variables-stranger")
        foreign = doc.slot(typed, "distance")
        self.assertNotIn(foreign, stranger.vars)
        with self.assertRaises(EvalError) as caught:
            stranger.eval(foreign)
        self.assertEqual(caught.exception.variant, "unresolved_var")

    def test_a_slot_of_a_node_the_document_does_not_hold_is_none(self):
        doc = Doc("slot-variables-none")
        box = blank(doc)
        self.assertIsNotNone(doc.slot(box, "distance"))
        self.assertIsNone(doc.slot(box, "radius"))


if __name__ == "__main__":
    unittest.main()
