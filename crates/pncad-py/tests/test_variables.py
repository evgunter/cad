"""A variable is an identity, and its name is held beside it.

`Doc.var` answers the `Var` a name holds, and every slot that reads the
variable holds that identity, not the text: a rename moves the name and
keeps the handle, and a delete leaves the readers in place, each
refusing at evaluation by its own tag.
"""

import unittest

from pncad import (
    Doc,
    DocEdit,
    FreeVar,
    FreeValue,
    EditError,
    EvaluationError,
    Expr,
    VarName,
    Var,
    VarDecl,
    evaluate,
    m,
)

from test_slot_edits import blank


def driven():
    """A box whose height reads the variable `height`, and the box."""
    doc = Doc("variables-row-13")
    doc.apply(DocEdit.declare_var(VarName("height"), FreeVar.length(2 * m)))
    box = blank(doc)
    doc.apply(DocEdit.set_param(box, "distance", doc.parse_formula("height")))
    return doc, box


def volume(doc, box):
    return evaluate(doc).value(box).body().mass_properties().volume


class TestAVarIsAnIdentity(unittest.TestCase):
    def test_a_handle_survives_a_rename(self):
        doc, box = driven()
        var = doc.var(VarName("height"))
        self.assertIsInstance(var, Var)
        self.assertEqual(len(var.hex), 16)
        before = volume(doc, box)

        doc.apply(DocEdit.rename_var(var, VarName("tall")))

        self.assertEqual(doc.var_name(var), VarName("tall"))
        self.assertEqual(doc.var(VarName("tall")), var)
        self.assertIsNone(doc.var(VarName("height")))
        self.assertEqual(list(doc.vars), [var])
        # The slot reads the identity, so the new name is what reads
        # back and the geometry is the same bits.
        self.assertEqual(volume(doc, box), before)
        # Addressed by the handle, the variable moves under its new name.
        doc.apply(DocEdit.set_var_value(var, FreeValue.length(3 * m)))
        self.assertAlmostEqual(volume(doc, box), before * 1.5)

    def test_a_rename_onto_a_held_name_refuses(self):
        doc, _ = driven()
        doc.apply(DocEdit.declare_var(VarName("width"), FreeVar.length(1 * m)))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.rename_var(VarName("height"), VarName("width")))
        self.assertEqual(caught.exception.variant, "var_name_taken")
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.rename_var(VarName("height"), VarName("height")))
        self.assertEqual(caught.exception.variant, "var_name_unchanged")

    def test_a_delete_leaves_its_readers_unresolved(self):
        doc, box = driven()
        var = doc.var(VarName("height"))

        doc.apply(DocEdit.delete_var(var))

        self.assertIsNone(doc.var_name(var))
        self.assertEqual(doc.vars, {})
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(box)
        self.assertEqual(caught.exception.reason, "node_failed")
        self.assertEqual(caught.exception.kind, "expr")
        self.assertEqual(caught.exception.inner_kind, "unresolved_var")
        # A new declare of the same name is a new variable, and the old
        # reader does not read it.
        doc.apply(DocEdit.declare_var(VarName("height"), FreeVar.length(2 * m)))
        self.assertNotEqual(doc.var(VarName("height")), var)
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(box)
        self.assertEqual(caught.exception.inner_kind, "unresolved_var")

    def test_a_delete_of_a_deleted_variable_refuses(self):
        doc, _ = driven()
        var = doc.var(VarName("height"))
        doc.apply(DocEdit.delete_var(var))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.delete_var(var))
        self.assertEqual(caught.exception.variant, "unknown_var")


class TestADefinedVariable(unittest.TestCase):
    """A variable defined by an expression over others: its value is the
    expression's, and it moves when what it reads moves."""

    def defined(self):
        doc = Doc("variables-defined")
        doc.apply(DocEdit.declare_var(VarName("base"), FreeVar.length(1 * m)))
        doc.apply(
            DocEdit.declare_var(VarName("height"), doc.parse_formula("base * 2.0"))
        )
        box = blank(doc)
        doc.apply(DocEdit.set_param(box, "distance", doc.parse_formula("height")))
        return doc, box

    def test_a_definition_reads_its_inputs(self):
        doc, box = self.defined()
        height = doc.var(VarName("height"))
        self.assertNotIn(height, doc.vars, "a defined variable is no free one")
        self.assertEqual(doc.unparse(doc.definition(height)), "base * 2.0")
        # Stored, so read back by id: the read-only `Expr`, not the
        # authored `Formula` the edit carried.
        self.assertIsInstance(doc.definition(height), Expr)
        self.assertEqual(doc.definition(height).dimension, "length")
        self.assertIsNone(doc.definition(doc.var(VarName("base"))))
        before = volume(doc, box)
        doc.apply(DocEdit.set_var_value(VarName("base"), FreeValue.length(2 * m)))
        self.assertAlmostEqual(volume(doc, box), before * 2.0)

    def test_a_definition_is_spelled_by_a_var_decl_and_back(self):
        doc, _ = self.defined()
        height = doc.var(VarName("height"))
        decl = VarDecl.defined(doc.parse_formula("base + base"))
        self.assertIsNotNone(decl.expr)
        self.assertIsNone(decl.value)
        doc.apply(DocEdit.define_var(height, decl))
        self.assertEqual(doc.var(VarName("height")), height, "the same identity")
        doc.apply(DocEdit.define_var(height, VarDecl.free(FreeVar.length(3 * m))))
        self.assertIsNone(doc.definition(height))
        self.assertIn(height, doc.vars)

    def test_a_definition_refuses_typed(self):
        doc, _ = self.defined()
        with self.assertRaises(EditError) as caught:
            doc.apply(
                DocEdit.define_var(VarName("base"), doc.parse_formula("height * 0.5"))
            )
        self.assertEqual(caught.exception.variant, "definition_cycle")
        with self.assertRaises(EditError) as caught:
            doc.apply(
                DocEdit.set_var_value(VarName("height"), FreeValue.length(1 * m))
            )
        self.assertEqual(caught.exception.variant, "not_a_free_var")
        # Read where `missing` is declared, written where it is not.
        elsewhere = Doc("variables-elsewhere")
        elsewhere.apply(
            DocEdit.declare_var(VarName("missing"), FreeVar.length(1 * m))
        )
        with self.assertRaises(EditError) as caught:
            doc.apply(
                DocEdit.declare_var(
                    VarName("depth"), elsewhere.parse_formula("missing * 2.0")
                )
            )
        self.assertEqual(caught.exception.variant, "definition_unknown_var_name")
        self.assertEqual(
            caught.exception.param,
            "depth",
            "a definition's faults name the variable defined",
        )

    def test_a_defined_variable_is_listed_by_name(self):
        doc, _ = self.defined()
        self.assertEqual(list(doc.params), [VarName("base")])
        self.assertEqual(list(doc.definitions), [VarName("height")])
        self.assertEqual(
            doc.unparse(doc.definitions[VarName("height")]), "base * 2.0"
        )


if __name__ == "__main__":
    unittest.main()
