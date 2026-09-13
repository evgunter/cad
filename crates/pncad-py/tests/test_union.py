"""The n-ary union node and the edit that rewrites its members.

`Node.union` is the fold over a member LIST — the third union door
beside `Node.boolean`, which is the BINARY operation over two named
operand slots, and `Node.placed_union`, whose members are one
prototype under a placement rule. This is the one whose membership is
DATA, and `DocEdit.set_members` is what makes that mean something: the
list is rewritten on the live node rather than by re-authoring a chain
of booleans.

The Python side of `crates/editor-core/tests/docm3_union.rs`. The
load-bearing rows are the same: the fold equals the boolean chain over
the same members in the same order, and the list-input door's three
refusals — `set_members_on_non_list`, `too_few_members`,
`duplicate_input` — arrive typed with the ids and counts that provoked
them.
"""

import unittest

from pncad import (
    BooleanOp,
    Doc,
    DocEdit,
    EditError,
    Expr,
    Node,
    evaluate,
    m,
)

# Three boxes of side 2, offset so that every pair overlaps in a
# volume and NO two faces are coplanar: a flush pair would be an
# undeclared contact, which is a different subject with its own door
# (`declare=`), and this scene is about the fold.
A = ((0.0, 2.0), (0.0, 2.0), (0.0, 2.0))
B = ((1.0, 3.0), (0.5, 2.5), (0.25, 2.25))
C = ((0.75, 2.75), (1.0, 3.0), (0.5, 2.5))

# Inclusion-exclusion over the three, by hand: 3 * 8 - (|AB| + |AC| +
# |BC|) + |ABC| = 24 - (2.625 + 1.875 + 4.59375) + 1.5.
ABC_VOLUME = 16.40625
# The same sum for the two-member list `set_members` rewrites to:
# 8 + 8 - 2.625.
AB_VOLUME = 13.375


def slab(doc, box):
    """The axis-aligned box, in metres, as an extruded rectangle."""
    (x0, x1), (y0, y1), (z0, z1) = box
    profile = doc.insert(
        Node.polygon(
            [
                (Expr.length_in(x0, m), Expr.length_in(y0, m)),
                (Expr.length_in(x1, m), Expr.length_in(y0, m)),
                (Expr.length_in(x1, m), Expr.length_in(y1, m)),
                (Expr.length_in(x0, m), Expr.length_in(y1, m)),
            ],
            plane=doc.sketch_frame(elevation=Expr.length_in(z0, m)),
        )
    )
    return doc.insert(Node.extrude(profile, Expr.length_in(z1 - z0, m)))


def measured(doc, node):
    """The node's mass properties, through a fresh evaluation."""
    ev = evaluate(doc)
    body = ev.value(node).body()
    body.validate()
    return body.mass_properties()


class TestTheNaryUnion(unittest.TestCase):
    def members(self, doc):
        return [slab(doc, box) for box in (A, B, C)]

    def test_the_fold_is_the_boolean_chain_over_the_same_members(self):
        """One node against the chain it replaces: `union([a, b, c])`
        folds left in the LIST's order, so the body it produces is the
        one `boolean(boolean(a, b), c)` produces — and both are the
        closed form the three boxes' inclusion-exclusion gives."""
        doc = Doc()
        a, b, c = self.members(doc)
        folded = doc.insert(Node.union([a, b, c]))
        chained = doc.insert(
            Node.boolean(
                BooleanOp.Union,
                doc.insert(Node.boolean(BooleanOp.Union, a, b)),
                c,
            )
        )
        self.assertEqual(doc.node_kind(folded), "union")

        fold = measured(doc, folded)
        chain = measured(doc, chained)
        # Mass properties are ENCLOSURES, so the closed form is
        # required to lie inside the certified pad rather than to
        # match a float.
        self.assertLessEqual(abs(fold.volume - ABC_VOLUME), fold.volume_pad + 1e-9)
        self.assertLessEqual(abs(chain.volume - ABC_VOLUME), chain.volume_pad + 1e-9)
        self.assertLess(fold.volume_pad, 1e-9)
        self.assertAlmostEqual(fold.volume, chain.volume, delta=1e-12)

    def test_a_list_of_one_is_not_a_union(self):
        """The floor is the node's own, held at the edit door: a
        one-member list refuses at `insert` and says how many it
        found, which is the number a caller would otherwise have to
        count for itself."""
        doc = Doc()
        a, _b, _c = self.members(doc)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.union([a]))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "too_few_members")
        self.assertEqual(refusal.count, 1)
        self.assertIsNone(refusal.input)
        self.assertIsNone(refusal.slot)

    def test_a_member_repeated_is_refused_and_named(self):
        """Members are pairwise DISTINCT, and the refusal carries the
        repeat rather than the whole list: the caller is told which id
        to drop."""
        doc = Doc()
        a, b, _c = self.members(doc)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.union([a, b, a]))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "duplicate_input")
        self.assertEqual(refusal.input, a)
        self.assertIsNone(refusal.count)


class TestSetMembers(unittest.TestCase):
    """`DocEdit.set_members` — the one edit that changes a live node's
    inputs, and the three refusals it carries."""

    def build(self):
        doc = Doc()
        a, b, c = (slab(doc, box) for box in (A, B, C))
        return doc, a, b, c, doc.insert(Node.union([a, b, c]))

    def test_the_membership_is_the_list_as_stated(self):
        """Dropping a member is the whole list restated without it —
        no positional spelling, no per-entry arm — and the node's
        value follows immediately."""
        doc, a, b, _c, union = self.build()
        self.assertLessEqual(
            abs(measured(doc, union).volume - ABC_VOLUME),
            measured(doc, union).volume_pad + 1e-9,
        )

        doc.apply(DocEdit.set_members(union, [a, b]))
        props = measured(doc, union)
        self.assertLessEqual(abs(props.volume - AB_VOLUME), props.volume_pad + 1e-9)
        self.assertEqual(doc.node_kind(union), "union")

    def test_a_node_with_no_list_refuses(self):
        """A boolean's operands are two NAMED slots, not a list, so
        there is nothing here for this edit to replace."""
        doc, a, b, _c, _union = self.build()
        pair = doc.insert(Node.boolean(BooleanOp.Union, a, b))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_members(pair, [a, b]))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "set_members_on_non_list")
        self.assertEqual(refusal.node, pair)
        self.assertIsNone(refusal.input)
        self.assertIsNone(refusal.count)

    def test_a_list_under_the_floor_refuses(self):
        """The floor `insert` holds is re-held of the REWRITTEN node,
        so the edit cannot leave a union with one member."""
        doc, a, _b, _c, union = self.build()
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_members(union, [a]))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "too_few_members")
        self.assertEqual(refusal.node, union)
        self.assertEqual(refusal.count, 1)

    def test_a_repeated_member_refuses(self):
        """Pairwise distinctness, re-checked the same way and naming
        the same repeat."""
        doc, a, b, _c, union = self.build()
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_members(union, [a, b, a]))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "duplicate_input")
        self.assertEqual(refusal.node, union)
        self.assertEqual(refusal.input, a)
        self.assertIsNone(refusal.count)


if __name__ == "__main__":
    unittest.main()
