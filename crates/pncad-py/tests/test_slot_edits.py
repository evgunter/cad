"""The two edits that address a document by NAME after it is built.

`DocEdit.set_param` moves a continuous slot's expression on a live
node — the door a constructor's literal was waiting for — and
`DocEdit.rebind` repairs a stored stable name. Both are addressed the
way a refusal answers: a slot by its own word, a name by its own text,
so a caller retries at the address it was refused at without
translating anything.

The rows here are the doors' own: one success per door, read back off
a fresh evaluation as a number that moved, and one refusal per tag
each door can provoke — including the two the boundary decides before
the kernel sees an edit.
"""

import unittest

from pncad import (
    CapEnd,
    Doc,
    DocEdit,
    DocParam,
    DocParamValue,
    EditError,
    EntityKind,
    NamePat,
    Node,
    ParamName,
    PatternKind,
    SegPat,
    SegTag,
    Selector,
    evaluate,
    m,
)

# A box L x L x H hollowed to a wall T with ONE face opened: the cup
# corpus document, whose closed form is exact at every dyadic value.
L, H, T = 1.0, 2.0, 0.25


def blank(doc, side=L, height=H):
    """A box `side x side x height` on the document's sketch frame."""
    square = doc.insert(
        Node.polygon(
            [(0 * m, 0 * m), (side * m, 0 * m), (side * m, side * m), (0 * m, side * m)],
            plane=doc.sketch_frame(),
        )
    )
    return doc.insert(Node.extrude(square, height * m))


FACES = NamePat.of_kind(EntityKind.Face)


def named(doc, node, pattern):
    """The faces of `node` a role pattern selects, off a fresh
    evaluation — names are read, never composed."""
    return evaluate(doc).select(node, Selector.of(pattern))


def top_of(doc, box):
    """The box's top face by ROLE: the extrude's end cap."""
    return named(doc, box, FACES.seg(SegPat.tag(SegTag.Cap).side(CapEnd.End)))[0]


def walls_of(doc, box):
    """The box's four lateral faces, by role."""
    return named(doc, box, FACES.seg(SegPat.tag(SegTag.Lateral)))


def volume(doc, node):
    return evaluate(doc).value(node).body().mass_properties().volume


class TestTheContinuousSlotEdit(unittest.TestCase):
    """`DocEdit.set_param` — a continuous slot's expression, after the
    constructor that minted it."""

    def cup(self, doc):
        box = blank(doc)
        return box, doc.insert(Node.shell(box, T * m, [top_of(doc, box)]))

    def test_a_minted_literal_moves_and_the_body_follows(self):
        """The constructors take numbers, so a node arrives holding
        literals. This is the door that moves one afterwards, and the
        evidence is the solid: the same recipe, re-measured."""
        doc = Doc()
        box = blank(doc)
        # L x L x H.
        self.assertEqual(volume(doc, box), L * L * H)
        doc.apply(DocEdit.set_param(box, "distance", doc.parse_expr("3 m")))
        self.assertEqual(volume(doc, box), L * L * 3.0)

    def test_a_slot_driven_by_a_document_parameter_moves_with_it(self):
        """What the door buys beyond a new number: a slot holding a
        PARAMETER REFERENCE is a named, editable quantity from then
        on, exactly as `bind_count_param` makes a structural one. One
        `set_doc_param_value` afterwards moves the wall."""
        doc = Doc()
        _box, hollow = self.cup(doc)
        inner = L - 2 * T
        self.assertEqual(volume(doc, hollow), L * L * H - inner * inner * (H - T))

        doc.apply(DocEdit.set_doc_param(ParamName("wall"), DocParam.length(T * m)))
        doc.apply(DocEdit.set_param(hollow, "shell_thickness", doc.parse_expr("wall")))
        self.assertEqual(volume(doc, hollow), L * L * H - inner * inner * (H - T))

        doc.apply(DocEdit.set_doc_param_value(ParamName("wall"), DocParamValue.length(0.375 * m)))
        thick = L - 2 * 0.375
        self.assertEqual(
            volume(doc, hollow), L * L * H - thick * thick * (H - 0.375)
        )

    def test_the_word_a_refusal_answers_with_is_the_word_the_door_takes(self):
        """The round trip this door is spelled for: a slot read off
        `EditError.slot` is handed straight back, with no translation
        and no second vocabulary between the two."""
        doc = Doc()
        box, hollow = self.cup(doc)
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_param(box, "shell_thickness", doc.parse_expr("1 m")))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "unknown_slot")
        self.assertEqual(refusal.node, box)
        self.assertEqual(refusal.slot, "shell_thickness")
        # The same word, at the node that does carry the slot.
        doc.apply(DocEdit.set_param(hollow, refusal.slot, doc.parse_expr("0.375 m")))
        thick = L - 2 * 0.375
        self.assertEqual(
            volume(doc, hollow), L * L * H - thick * thick * (H - 0.375)
        )

    def test_a_foreign_node_refuses_before_any_slot_is_looked_up(self):
        doc = Doc()
        blank(doc)
        other = Doc(label="a-longer-recipe")
        for _ in range(3):
            stray = blank(other)
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_param(stray, "distance", doc.parse_expr("1 m")))
        self.assertEqual(caught.exception.variant, "unknown_node")
        self.assertEqual(caught.exception.node, stray)
        self.assertIsNone(caught.exception.slot)

    def test_an_expression_of_the_wrong_dimension_carries_the_pair(self):
        """A slot's dimension is the slot's, and the refusal says both
        halves in the alphabet `Expr.dimension` answers in."""
        doc = Doc()
        box = blank(doc)
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_param(box, "distance", doc.parse_expr("1 rad")))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "slot_dimension_mismatch")
        self.assertEqual(refusal.slot, "distance")
        self.assertEqual(refusal.expected, "length")
        self.assertEqual(refusal.found, "angle")

    def test_a_structural_slot_is_the_other_doors(self):
        """The structural/continuous divide is unlosable in the edit
        stream, so this door refuses at a Count-typed slot rather than
        quietly crossing it — whatever node it was aimed at."""
        doc = Doc()
        box = blank(doc)
        pattern = doc.insert(
            Node.pattern(box, 3, PatternKind.linear((1.0, 0.0, 0.0), 2 * m))
        )
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_param(pattern, "count", doc.parse_expr("4")))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "structural_slot_needs_structural_edit")
        self.assertEqual(refusal.slot, "count")
        self.assertIsNone(refusal.node)
        # And the door that IS this slot's still works on the node.
        doc.apply(DocEdit.set_doc_param(ParamName("copies"), DocParam.count(4)))
        doc.apply(DocEdit.bind_count_param(pattern, ParamName("copies")))

    def test_a_parameter_reference_is_checked_at_the_edit_door(self):
        """An expression parsed against one document and applied to
        another names a parameter the target does not declare — the
        continuous door runs the same reference check the structural
        one does."""
        declared = Doc(label="declares-the-wall")
        blank(declared)
        declared.apply(DocEdit.set_doc_param(ParamName("wall"), DocParam.length(T * m)))
        expr = declared.parse_expr("wall")

        doc = Doc(label="declares-nothing")
        box = blank(doc)
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.set_param(box, "distance", expr))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "unknown_doc_param")
        self.assertEqual(refusal.param, "wall")
        self.assertEqual(refusal.slot, "distance")
        self.assertEqual(refusal.node, box)

    def test_a_word_outside_the_alphabet_never_becomes_an_edit(self):
        """A slot is a NAME, and text that is not one of the names is
        a boundary `ValueError` — the refusal class a string where a
        name belongs already raises, with no kernel refusal to
        forward."""
        doc = Doc()
        box = blank(doc)
        for junk in ["", "Distance", "origin", "distance ", "count_x"]:
            with self.assertRaises(ValueError, msg=junk):
                DocEdit.set_param(box, junk, doc.parse_expr("1 m"))

    def test_a_profile_programs_expression_is_not_addressable_by_word(self):
        """`profile` is a word of the alphabet with no slot to read
        back: the rest of that address is a loop index, a step index
        and which argument, none of which the word carries. It refuses
        in its own sentence rather than as a misspelling."""
        doc = Doc()
        box = blank(doc)
        with self.assertRaises(ValueError) as caught:
            DocEdit.set_param(box, "profile", doc.parse_expr("1 m"))
        self.assertIn("profile program", str(caught.exception))


class TestTheNameRepair(unittest.TestCase):
    """`DocEdit.rebind` — the one name repair, and its five typed
    refusals."""

    def cup(self, doc):
        """A cup whose mouth is the top face: one document site
        references that name, which is what a repair rewrites."""
        box = blank(doc)
        top = top_of(doc, box)
        return box, top, doc.insert(Node.shell(box, T * m, [top]))

    def test_the_repair_rewrites_the_site_and_the_body_follows(self):
        """A shell's open list is a name-carrying payload, so saying
        what the mouth now denotes moves the mouth. The two closed
        forms are the cavity opened at the top and at a wall."""
        doc = Doc()
        _box, top, hollow = self.cup(doc)
        inner = L - 2 * T
        self.assertEqual(volume(doc, hollow), L * L * H - inner * inner * (H - T))

        wall = walls_of(doc, _box)[0]
        doc.apply(DocEdit.rebind(top, wall))
        # The cavity now reaches the opened wall: it is walled on one
        # side in y and on both in x and z.
        self.assertEqual(
            volume(doc, hollow), L * L * H - (L - 2 * T) * (L - T) * (H - 2 * T)
        )

    def test_a_name_rebound_to_itself_is_a_recorded_no_op(self):
        doc = Doc()
        _box, top, _hollow = self.cup(doc)
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.rebind(top, top))
        self.assertEqual(caught.exception.variant, "rebind_identity")
        self.assertEqual(caught.exception.name, top)

    def test_a_repair_cannot_cross_entity_kinds(self):
        """A reference's KIND is part of its type: a face reference
        cannot come to denote an edge, and the refusal names both
        kinds rather than the name it refused."""
        doc = Doc()
        box, top, _hollow = self.cup(doc)
        edge = evaluate(doc).all_edges(box)[0]
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.rebind(top, edge))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "rebind_kind_mismatch")
        self.assertEqual(refusal.from_kind, EntityKind.Face)
        self.assertEqual(refusal.to_kind, EntityKind.Edge)
        self.assertIsNone(refusal.name)

    def test_a_target_whose_node_is_gone_refuses(self):
        """The selection must denote something the recipe still has —
        node existence NOW, at the best-diagnostics door."""
        doc = Doc()
        _box, top, _hollow = self.cup(doc)
        spare = blank(doc)
        stranded = top_of(doc, spare)
        doc.apply(DocEdit.delete_node(spare))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.rebind(top, stranded))
        self.assertEqual(caught.exception.variant, "rebind_target_missing_node")
        self.assertEqual(caught.exception.name, stranded)

    def test_a_source_this_document_never_minted_is_a_typo(self):
        """Ids are monotone and never reused, so a name whose node is
        at or above the mint counter is foreign. A
        deleted-but-once-lived node is the repair case and is
        allowed — that is the distinction this refusal keeps."""
        doc = Doc()
        _box, top, _hollow = self.cup(doc)
        longer = Doc(label="a-longer-recipe")
        for _ in range(4):
            far = blank(longer)
        foreign = top_of(longer, far)
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.rebind(foreign, top))
        self.assertEqual(caught.exception.variant, "rebind_unknown_name")
        self.assertEqual(caught.exception.name, foreign)

    def test_a_source_nothing_references_has_nothing_to_repair(self):
        """A GUI's selection is not document state, so there is no
        repair to record for a name no site stores."""
        doc = Doc()
        box, _top, _hollow = self.cup(doc)
        unreferenced, other = walls_of(doc, box)[:2]
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.rebind(unreferenced, other))
        self.assertEqual(caught.exception.variant, "rebind_no_references")
        self.assertEqual(caught.exception.name, unreferenced)

    def test_a_name_is_carried_not_composed(self):
        """Both halves take the name's own text, and text that is not
        a name at all is the boundary `ValueError` every other
        name-taking door raises."""
        doc = Doc()
        _box, top, _hollow = self.cup(doc)
        with self.assertRaises(ValueError):
            DocEdit.rebind("the top face", top)
        with self.assertRaises(ValueError):
            DocEdit.rebind(top, "the other one")


if __name__ == "__main__":
    unittest.main()
