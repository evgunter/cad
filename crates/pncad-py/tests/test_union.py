"""The union, the intersect and the subtract, and the edit that
rewrites a member list.

`Node.union` and `Node.intersect` fold a member argument — a list of
reads, or one read of a whole family — and `Node.subtract` is the one
pair node, `from_` cut by `tool`. The member list is DATA, and
`DocEdit.set_members` is what makes that mean something: the list is
rewritten on the live node rather than by re-authoring a chain.

The Python side of `crates/editor-core/tests/docm3_union.rs`. The
load-bearing rows are the same: the fold equals the chain over the same
members in the same order; any count is a fold (one member is that
body, none the typed empty body); a read listed twice glues, so
`[a, a]` is `a` and `a - a` is empty; and a node with no list refuses
`set_members_on_non_list`, typed with the id that provoked it.
"""

import json
import math
import unittest

from pncad import (
    Doc,
    DocEdit,
    EditError,
    Formula,
    Node,
    PatternKind,
    evaluate,
    load,
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
# One box alone, and the overlap of A and B, which A minus B leaves the
# rest of: 8 - 2.625.
BOX_VOLUME = 8.0
AB_COMMON = 2.625
A_MINUS_B = BOX_VOLUME - AB_COMMON


def slab(doc, box):
    """The axis-aligned box, in metres, as an extruded rectangle."""
    (x0, x1), (y0, y1), (z0, z1) = box
    profile = doc.insert(
        Node.polygon(
            [
                (Formula.length_in(x0, m), Formula.length_in(y0, m)),
                (Formula.length_in(x1, m), Formula.length_in(y0, m)),
                (Formula.length_in(x1, m), Formula.length_in(y1, m)),
                (Formula.length_in(x0, m), Formula.length_in(y1, m)),
            ],
            plane=doc.sketch_frame(elevation=Formula.length_in(z0, m)),
        )
    )
    return doc.insert(Node.extrude(profile, Formula.length_in(z1 - z0, m)))


def measured(doc, node):
    """The node's mass properties, through a fresh evaluation."""
    ev = evaluate(doc)
    body = ev.value(node).body()
    body.validate()
    return body.mass_properties()


def assert_volume(case, props, expected):
    """Mass properties are ENCLOSURES, so a closed form is required to
    lie inside the certified pad rather than to match a float."""
    case.assertLessEqual(abs(props.volume - expected), props.volume_pad + 1e-9)


def assert_empty(case, doc, node):
    """The typed empty body: a boolean value that denotes no body."""
    value = evaluate(doc).value(node)
    case.assertEqual(value.kind, "boolean")
    case.assertEqual(value.bodies(), [])


class TestTheUnion(unittest.TestCase):
    def members(self, doc):
        return [slab(doc, box) for box in (A, B, C)]

    def test_the_fold_is_the_chain_over_the_same_members(self):
        """One node against the chain it replaces: `union([a, b, c])`
        folds left in the LIST's order, so the body it produces is the
        one `union([union([a, b]), c])` produces — and both are the
        closed form the three boxes' inclusion-exclusion gives."""
        doc = Doc()
        a, b, c = self.members(doc)
        folded = doc.insert(Node.union([a, b, c]))
        chained = doc.insert(Node.union([doc.insert(Node.union([a, b])), c]))
        self.assertEqual(doc.node_kind(folded), "union")

        fold = measured(doc, folded)
        chain = measured(doc, chained)
        assert_volume(self, fold, ABC_VOLUME)
        assert_volume(self, chain, ABC_VOLUME)
        self.assertLess(fold.volume_pad, 1e-9)
        self.assertAlmostEqual(fold.volume, chain.volume, delta=1e-12)

    def test_a_list_of_one_is_that_body(self):
        """Any count is a union: one member builds that member's body,
        accepted at `insert` and evaluated without a boolean."""
        doc = Doc()
        a, _b, _c = self.members(doc)
        alone = doc.insert(Node.union([a]))
        self.assertEqual(doc.node_kind(alone), "union")
        assert_volume(self, measured(doc, alone), BOX_VOLUME)

    def test_an_empty_list_is_the_typed_empty_body(self):
        """No members is the typed empty body, for a union and an
        intersect alike — not a refusal."""
        doc = Doc()
        nothing = doc.insert(Node.union([]))
        common = doc.insert(Node.intersect([]))
        assert_empty(self, doc, nothing)
        assert_empty(self, doc, common)

    def test_a_member_repeated_glues(self):
        """A read listed twice is the same material twice: `[a, b, a]`
        is `[a, b]`, and `[a, a]` is `a`."""
        doc = Doc()
        a, b, _c = self.members(doc)
        repeated = doc.insert(Node.union([a, b, a]))
        twice = doc.insert(Node.union([a, a]))
        assert_volume(self, measured(doc, repeated), AB_VOLUME)
        assert_volume(self, measured(doc, twice), BOX_VOLUME)

    def test_a_file_repeating_a_member_loads_and_glues(self):
        """The load door holds a repeated member to the same rule as
        the edit door: a file whose union lists one member twice loads,
        and the union is the union of the distinct members."""
        doc = Doc()
        a, b, _c = self.members(doc)
        union = doc.insert(Node.union([a, b]))
        header, body = doc.save().split("\n", 1)
        wire = json.loads(body)
        (members,) = [
            node["Union"]["members"]["Spelled"]
            for node in wire["snapshot"]["nodes"].values()
            if "Union" in node
        ]
        members.append(members[0])
        loaded = load(f"{header}\n{json.dumps(wire)}").doc
        assert_volume(self, measured(loaded, union), AB_VOLUME)

    def test_one_read_of_a_family_is_its_members(self):
        """A single operand, not a list, reads a family whole: a
        pattern's two copies of `A`, stepped by B's offset, union to
        the same body as `[a, b]`. A family read beside a single read
        in one list is the kernel's kind refusal at `insert`."""
        doc = Doc()
        a, _b, _c = self.members(doc)
        family = doc.insert(
            Node.pattern(
                a,
                Formula.count(2),
                PatternKind.linear(
                    (Formula.literal(1.0), Formula.literal(0.5), Formula.literal(0.25)),
                    Formula.length_in(math.sqrt(1.3125), m),
                ),
            )
        )
        folded = doc.insert(Node.union(family))
        self.assertAlmostEqual(measured(doc, folded).volume, AB_VOLUME, delta=1e-9)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.union([family, a]))
        self.assertEqual(caught.exception.variant, "slot_var_kind")

    def test_a_family_spelled_by_index_is_the_family_read_whole(self):
        """`family[i]` reads one member of the family: the union of the
        members spelled by index builds and names as the family read
        whole, and a member read by index sits at any body seat."""
        doc = Doc()
        a, _b, _c = self.members(doc)
        family = doc.insert(
            Node.pattern(
                a,
                Formula.count(2),
                PatternKind.linear(
                    (Formula.literal(1.0), Formula.literal(0.5), Formula.literal(0.25)),
                    Formula.length_in(math.sqrt(1.3125), m),
                ),
            )
        )
        whole = doc.insert(Node.union(family))
        spelled = doc.insert(Node.union([family[0], family[1]]))
        self.assertAlmostEqual(measured(doc, spelled).volume, AB_VOLUME, delta=1e-9)
        ev = evaluate(doc)
        self.assertEqual(len(ev.all_faces(whole)), len(ev.all_faces(spelled)))
        second = doc.insert(Node.subtract(family[1], a))
        assert_volume(self, measured(doc, second), A_MINUS_B)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.subtract(a[0], family[1]))
        self.assertEqual(caught.exception.variant, "slot_var_kind")


class TestIntersectAndSubtract(unittest.TestCase):
    def test_the_intersect_keeps_the_common_material(self):
        """`intersect([a, b])` is the overlap; `[a, a]` is `a`."""
        doc = Doc()
        a, b = slab(doc, A), slab(doc, B)
        common = doc.insert(Node.intersect([a, b]))
        twice = doc.insert(Node.intersect([a, a]))
        self.assertEqual(doc.node_kind(common), "intersect")
        assert_volume(self, measured(doc, common), AB_COMMON)
        assert_volume(self, measured(doc, twice), BOX_VOLUME)

    def test_the_subtract_cuts_tool_from_from_(self):
        """`subtract(a, b)` is `a` less the overlap — the operand order
        is the meaning — and `subtract(a, a)` is the typed empty body."""
        doc = Doc()
        a, b = slab(doc, A), slab(doc, B)
        cut = doc.insert(Node.subtract(a, b))
        spelled = doc.insert(Node.subtract(from_=a, tool=b))
        itself = doc.insert(Node.subtract(a, a))
        self.assertEqual(doc.node_kind(cut), "subtract")
        assert_volume(self, measured(doc, cut), A_MINUS_B)
        assert_volume(self, measured(doc, spelled), A_MINUS_B)
        assert_empty(self, doc, itself)


class TestSetMembers(unittest.TestCase):
    """`DocEdit.set_members` — the one edit that changes a live node's
    inputs."""

    def build(self):
        doc = Doc()
        a, b, c = (slab(doc, box) for box in (A, B, C))
        return doc, a, b, c, doc.insert(Node.union([a, b, c]))

    def test_the_membership_is_the_list_as_stated(self):
        """Dropping a member is the whole list restated without it —
        no positional spelling, no per-entry arm — and the node's
        value follows immediately."""
        doc, a, b, _c, union = self.build()
        assert_volume(self, measured(doc, union), ABC_VOLUME)

        doc.apply(DocEdit.set_members(union, [a, b]))
        assert_volume(self, measured(doc, union), AB_VOLUME)
        self.assertEqual(doc.node_kind(union), "union")

    def test_a_node_with_no_list_refuses(self):
        """A subtract's operands are two NAMED slots, not a list, so
        there is nothing here for this edit to replace."""
        doc, a, b, _c, _union = self.build()
        pair = doc.insert(Node.subtract(a, b))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_members(pair, [a, b]))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "set_members_on_non_list")
        self.assertEqual(refusal.node, pair)
        self.assertIsNone(refusal.input)
        self.assertIsNone(refusal.count)

    def test_a_list_of_one_is_accepted(self):
        """The rewritten node is held to the rules `insert` holds, and
        one member is a union: the edit is accepted and the node is
        that member's body."""
        doc, a, _b, _c, union = self.build()
        doc.apply(DocEdit.set_members(union, [a]))
        assert_volume(self, measured(doc, union), BOX_VOLUME)

    def test_a_repeated_member_is_accepted_and_glues(self):
        """A repeat is re-held the same way and glues: `[a, b, a]` is
        the union of `a` and `b`."""
        doc, a, b, _c, union = self.build()
        doc.apply(DocEdit.set_members(union, [a, b, a]))
        assert_volume(self, measured(doc, union), AB_VOLUME)


if __name__ == "__main__":
    unittest.main()
