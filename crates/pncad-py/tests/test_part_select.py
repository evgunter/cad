"""The projection node, from Python (LIB-B-PART).

`Node.part` says ONE body out of a multi-body value — "the upper half
of that split", "instance 1 of that pattern" — and it is the node that
gives the two plural payloads a downstream door. The Python mirror of
`crates/editor-core/tests/corpus/part_select.rs` and the acceptance
rows in `crates/editor-core/tests/docm2_part.rs`, asserted through the
bound doors rather than restated as prose.

THREE DOORS, ONE SENTENCE. `PartSelect.split_half` /
`PartSelect.instance` are the selector pair; `Node.part` carries one;
and `Node.pattern` — bound HERE, not before — is what an instance
selection selects out of. That last one is the finding this unit's
scope sweep turned up before any code: exactly one node in the kernel
emits a plural `instances` payload, it was deliberately unbound
because nothing consumed one, and `Node.part` is the thing that
consumes one. Binding the selector without the source would have
shipped a door whose Instance half no Python caller could reach.

WHAT "A PROJECTION" MEANS, and what the rows below check. Nothing is
moved, re-stamped or recomputed: the half's body is the split's own,
so its mass IS the mass of the side `Value.split` hands back
(`TestTheHalfIsTheHalf`), the two halves unioned are the box again
(`test_the_halves_rejoin_into_the_box`), and the names pass through
verbatim, so a face read off the Part is a face of the split
(`test_the_names_pass_through`). The index is STRUCTURAL, so moving it
recomputes the Part and nothing upstream
(`test_moving_the_index_recomputes_the_part_alone`).

DELIBERATELY NOT ASSERTED HERE, so its absence is not read as
coverage: the `Arc`-identity claim (a Part's body is the same
allocation as the half's, not a copy of it) is `docm2_part.rs`'s A1
and has no Python spelling — `Body` is an opaque handle with no
identity door, and inventing one to test this would be a door built
for its own test. The mass and the name table are what cross, and
they are what is compared.
"""

import unittest

from pncad import (
    BooleanOp,
    Doc,
    DocEdit,
    DocParam,
    EditError,
    EvaluationError,
    Expr,
    Node,
    ParamName,
    PartSelect,
    PatternKind,
    PlaneRelation,
    SplitHalf,
    WrittenAngle,
    WrittenLength,
    evaluate,
    m,
    rad,
)

#: The box's footprint half-extent and height, metres — the corpus
#: document's own constants, so the dyadic pins below are its pins.
BOX_HALF = 1.0
BOX_H = 1.0
#: The tool plane's height: a cut at mid-height.
CUT_Z = 0.5
#: The pattern's instance count and its pitch along x, clear of the
#: box's 2 m width.
COUNT = 3
PITCH = 3.0


def box(doc, half=BOX_HALF, height=BOX_H):
    """The box [-half, half]^2 x [0, height], standing on the ground."""
    square = doc.insert(
        Node.polygon(
            [
                (Expr.written_length(WrittenLength.in_unit(-half, m)), Expr.written_length(WrittenLength.in_unit(-half, m))),
                (Expr.written_length(WrittenLength.in_unit(half, m)), Expr.written_length(WrittenLength.in_unit(-half, m))),
                (Expr.written_length(WrittenLength.in_unit(half, m)), Expr.written_length(WrittenLength.in_unit(half, m))),
                (Expr.written_length(WrittenLength.in_unit(-half, m)), Expr.written_length(WrittenLength.in_unit(half, m))),
            ],
            plane=doc.sketch_frame(),
        )
    )
    return doc.insert(Node.extrude(square, Expr.written_length(WrittenLength.in_unit(height, m))))


def split_at(doc, target, z=CUT_Z):
    """`target` cut by a horizontal plane at height `z`."""
    tool = doc.insert(Node.datum_plane((
        Expr.written_length(WrittenLength.in_unit(0, m)),
        Expr.written_length(WrittenLength.in_unit(0, m)),
        Expr.written_length(WrittenLength.in_unit(z, m)),
    ), (
        Expr.literal(0.0),
        Expr.literal(0.0),
        Expr.literal(1.0),
    )))
    return doc.insert(Node.split(target, tool))


def pattern_of(doc, prototype, count=COUNT, pitch=PITCH):
    """`count` copies of `prototype` stepped `pitch` apart along x."""
    return doc.insert(
        Node.pattern(prototype, Expr.count(count), PatternKind.linear((
            Expr.literal(1.0),
            Expr.literal(0.0),
            Expr.literal(0.0),
        ), Expr.written_length(WrittenLength.in_unit(pitch, m))))
    )


def mass_of(ev, node):
    body = ev.value(node).body()
    body.validate()
    return body.mass_properties()


def refusal(testcase, doc, node):
    """The typed refusal `node` evaluates to, as its stable tag."""
    with testcase.assertRaises(EvaluationError) as caught:
        evaluate(doc).value(node)
    return caught.exception.kind


class TestTheHalfIsTheHalf(unittest.TestCase):
    """A split's half, selected — the corpus document's first half.

    The oracle is the split's OWN value: `Value.split` hands back the
    two sides, and a Part of a half must weigh exactly what that side
    weighs. Nothing here transcribes a number a Part is then checked
    against; the two readings of the same body are compared.
    """

    def build(self):
        doc = Doc()
        cube = box(doc)
        split = split_at(doc, cube)
        above = doc.insert(Node.part(split, PartSelect.split_half(SplitHalf.Above)))
        below = doc.insert(Node.part(split, PartSelect.split_half(SplitHalf.Below)))
        return doc, split, above, below

    def test_each_half_weighs_what_the_split_says_it_weighs(self):
        doc, split, above, below = self.build()
        ev = evaluate(doc)
        self.assertEqual(ev.value(split).kind, "split")
        upper, lower = ev.value(split).split()
        for part, side, name in ((above, upper, "above"), (below, lower, "below")):
            with self.subTest(half=name):
                self.assertIsNotNone(side)
                self.assertEqual(ev.value(part).kind, "body")
                self.assertEqual(
                    mass_of(ev, part).volume, side.mass_properties().volume
                )
                self.assertEqual(
                    mass_of(ev, part).surface_area,
                    side.mass_properties().surface_area,
                )

    def test_the_two_halves_are_the_box_cut_where_the_plane_is(self):
        """The dyadic pin, stated once: a 2 x 2 x 1 box cut at 0.5 is
        two 2 x 2 x 0.5 slabs. Both oracles are exact binary fractions,
        so the comparison is `==` rather than a tolerance."""
        doc, _, above, below = self.build()
        ev = evaluate(doc)
        for part in (above, below):
            mass = mass_of(ev, part)
            self.assertEqual(mass.volume, 2.0 * 2.0 * 0.5)
            self.assertEqual(mass.surface_area, 2 * 2.0 * 2.0 + 4 * 2.0 * 0.5)

    def test_the_halves_rejoin_into_the_box(self):
        """The document's own statement that a Part IS the half:
        nothing was moved, re-stamped or lost on the way through, so
        unioning the two back together is the box again.

        The two halves REST on each other across the section, which is
        a declared contact — detected through `find_flush_candidates`
        and declared through `Doc.declare_all`, the same protocol any
        touching union goes through. That the detector finds the pair
        AT ALL across two Parts is itself the pass-through claim: the
        faces it matches are the split's, carried verbatim.

        The inventory says the same thing a second way. Five findings:
        ONE `SameOpposite` — the section, where the halves face each
        other — and four `SameOriented`, the box's four walls, each cut
        into two coplanar pieces facing the same way out. Only the
        first is a rest contact to declare.
        """
        doc, _, above, below = self.build()
        ev = evaluate(doc)
        findings = ev.find_flush_candidates(above, below)
        self.assertEqual(len(findings), 5)
        section = [f for f in findings if f.relation == PlaneRelation.SameOpposite]
        self.assertEqual(len(section), 1, "the one section face pair")
        whole = doc.insert(
            Node.boolean(BooleanOp.Union, above, below, declare=doc.declare_all(section))
        )
        mass = mass_of(evaluate(doc), whole)
        self.assertEqual(mass.volume, 2.0 * 2.0 * 1.0)
        self.assertEqual(mass.surface_area, 2 * 2.0 * 2.0 + 4 * 2.0 * 1.0)

    def test_the_names_pass_through(self):
        """A Part's name table is the input's, RESTRICTED to the
        selected body and otherwise verbatim — so every face name the
        Part answers with is a face name of the split, unchanged as
        text, and the two halves' name sets are disjoint."""
        doc, split, above, below = self.build()
        ev = evaluate(doc)
        whole = set(ev.all_faces(split))
        upper, lower = set(ev.all_faces(above)), set(ev.all_faces(below))
        self.assertTrue(upper <= whole)
        self.assertTrue(lower <= whole)
        self.assertEqual(upper & lower, set())
        # A cut box is a 6-faced slab per side: four walls, the
        # original face, and the section.
        self.assertEqual(len(upper), 6)
        self.assertEqual(len(lower), 6)


class TestTheInstanceIsTheInstance(unittest.TestCase):
    """A pattern's instance, selected — the corpus document's second
    half, and the reason `Node.pattern` binds here."""

    def build(self):
        doc = Doc()
        cube = box(doc)
        family = pattern_of(doc, cube)
        return doc, cube, family

    def test_the_patterns_value_is_plural_and_the_parts_is_not(self):
        doc, _, family = self.build()
        middle = doc.insert(Node.part(family, PartSelect.instance(Expr.count(1))))
        ev = evaluate(doc)
        self.assertEqual(ev.value(family).kind, "instances")
        self.assertEqual(len(ev.value(family).bodies()), COUNT)
        self.assertEqual(ev.value(middle).kind, "body")

    def test_each_instance_weighs_what_the_pattern_says_it_weighs(self):
        doc, _, family = self.build()
        parts = [
            doc.insert(Node.part(family, PartSelect.instance(Expr.count(i))))
            for i in range(COUNT)
        ]
        ev = evaluate(doc)
        bodies = ev.value(family).bodies()
        for i, part in enumerate(parts):
            with self.subTest(instance=i):
                self.assertEqual(
                    mass_of(ev, part).volume, bodies[i].mass_properties().volume
                )
        # A rigid step moves a body and changes neither oracle, so
        # every instance weighs the prototype's own dyadic mass.
        for part in parts:
            self.assertEqual(mass_of(ev, part).volume, 2.0 * 2.0 * 1.0)

    def test_an_instance_is_an_ordinary_operand(self):
        """What the projection BUYS: the plural value feeds nothing,
        and one body out of it feeds everything. The middle copy is
        lifted by a transform, exactly as the corpus document lifts
        it."""
        doc, _, family = self.build()
        middle = doc.insert(Node.part(family, PartSelect.instance(Expr.count(1))))
        lifted = doc.insert(
            Node.transform(middle, (
                Expr.written_length(WrittenLength.in_unit(0, m)),
                Expr.written_length(WrittenLength.in_unit(0, m)),
                Expr.written_length(WrittenLength.in_unit(2, m)),
            ), (
                Expr.literal(0.0),
                Expr.literal(0.0),
                Expr.literal(1.0),
            ), Expr.written_angle(WrittenAngle.in_unit(0, rad)))
        )
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(lifted))
        self.assertEqual(mass_of(ev, lifted).volume, 2.0 * 2.0 * 1.0)


class TestTheSelectorAndTheValueMustAgree(unittest.TestCase):
    """A half against a split, an index against a pattern's instances,
    and any other pairing refuses `wrong_operand` at evaluation.

    Not at construction: which value a node id carries is not known
    until the document runs, so the door takes the pairing and the
    evaluator judges it. All four crossings, as `docm2_part.rs`'s A4
    asserts them.
    """

    def test_all_four_mismatches_refuse(self):
        doc = Doc()
        cube = box(doc)
        split = split_at(doc, cube)
        family = pattern_of(doc, cube)
        half = PartSelect.split_half(SplitHalf.Above)
        index = PartSelect.instance(Expr.count(0))
        for of, select, label in (
            (family, half, "a half of a pattern"),
            (split, index, "an index of a split"),
            (cube, half, "a half of a plain body"),
            (cube, index, "an index of a plain body"),
        ):
            with self.subTest(case=label):
                node = doc.insert(Node.part(of, select))
                self.assertEqual(refusal(self, doc, node), "wrong_operand")


class TestTheRefusalsAreTyped(unittest.TestCase):
    """The family's own two refusals, reached from Python for the first
    time — `empty_half` and `instance_out_of_range`, the two tags
    `crates/pncad-py/src/tags.rs` has carried since DOCM-2 with no
    caller able to construct either."""

    def test_a_half_with_no_material_refuses(self):
        """A tool plane that misses the box entirely: the empty side's
        Part refuses, and the side WITH material still evaluates — the
        refusal is the selection's, not the split's."""
        doc = Doc()
        cube = box(doc)
        split = split_at(doc, cube, z=2.0)
        above = doc.insert(Node.part(split, PartSelect.split_half(SplitHalf.Above)))
        below = doc.insert(Node.part(split, PartSelect.split_half(SplitHalf.Below)))
        self.assertEqual(refusal(self, doc, above), "empty_half")
        self.assertTrue(evaluate(doc).succeeded(below))

    def test_an_index_outside_the_count_refuses(self):
        """At the count and below zero, one refusal: neither is
        wrapped nor clamped, because either would hand back a body the
        author did not name."""
        doc = Doc()
        family = pattern_of(doc, box(doc))
        for index in (COUNT, -1):
            with self.subTest(index=index):
                node = doc.insert(Node.part(family, PartSelect.instance(Expr.count(index))))
                self.assertEqual(refusal(self, doc, node), "instance_out_of_range")

    def test_lowering_the_count_under_a_live_index_refuses(self):
        """The index is judged against the count as it stands NOW, so
        an edit upstream of the Part can invalidate it — and says so
        rather than quietly selecting a neighbour."""
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("n"), DocParam.count(COUNT)))
        family = pattern_of(doc, box(doc))
        doc.apply(DocEdit.bind_count_param(family, ParamName("n")))
        live = doc.insert(Node.part(family, PartSelect.instance(Expr.count(2))))
        self.assertTrue(evaluate(doc).succeeded(live))

        doc.apply(DocEdit.set_doc_param(ParamName("n"), DocParam.count(2)))
        self.assertTrue(evaluate(doc).succeeded(family), "the pattern is Ok at two")
        self.assertEqual(refusal(self, doc, live), "instance_out_of_range")


class TestTheIndexIsStructural(unittest.TestCase):
    """`SlotId::Instance` from Python: its own door, because an index
    is not a count.

    `DocEdit.bind_instance_param` is `bind_count_param`'s sibling and
    the pair is the kernel's own distinction crossing — a panel that
    spelled an index "count" would be lying about it. Neither door
    reaches the other's slot, which is what the two refusals below
    say.
    """

    def build(self):
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("which"), DocParam.count(0)))
        family = pattern_of(doc, box(doc))
        chosen = doc.insert(Node.part(family, PartSelect.instance(Expr.count(0))))
        doc.apply(DocEdit.bind_instance_param(chosen, ParamName("which")))
        return doc, family, chosen

    def test_one_param_edit_moves_which_instance_is_selected(self):
        """The payoff: which copy a downstream consumer sees is a named
        number, and moving it is one `set_doc_param`."""
        doc, family, chosen = self.build()
        for which in (0, 1, 2):
            with self.subTest(which=which):
                doc.apply(
                    DocEdit.set_doc_param(ParamName("which"), DocParam.count(which))
                )
                ev = evaluate(doc)
                self.assertEqual(
                    mass_of(ev, chosen).volume,
                    ev.value(family).bodies()[which].mass_properties().volume,
                )
        # And the bound index is judged the same way a literal one is.
        doc.apply(DocEdit.set_doc_param(ParamName("which"), DocParam.count(COUNT)))
        self.assertEqual(refusal(self, doc, chosen), "instance_out_of_range")

    def test_moving_the_index_recomputes_the_part_alone(self):
        """A projection moves nothing upstream of itself, so an index
        edit recomputes exactly one node — the memo's own statement
        that a Part is a projection."""
        doc, _, _chosen = self.build()
        first = evaluate(doc)
        doc.apply(DocEdit.set_doc_param(ParamName("which"), DocParam.count(1)))
        again = evaluate(doc, prior=first)
        self.assertEqual(again.recomputed, 1)
        self.assertEqual(again.reused, len(doc) - 1)

    def test_neither_slot_door_reaches_the_others_slot(self):
        doc, family, chosen = self.build()
        with self.assertRaises(EditError) as no_count:
            doc.apply(DocEdit.bind_count_param(chosen, ParamName("which")))
        self.assertEqual(no_count.exception.variant, "unknown_slot")
        with self.assertRaises(EditError) as no_instance:
            doc.apply(DocEdit.bind_instance_param(family, ParamName("which")))
        self.assertEqual(no_instance.exception.variant, "unknown_slot")

    def test_a_half_selection_carries_no_index_slot(self):
        """A Part is one node with two shapes, and only one of them
        has a slot: which HALF is not a number to bind."""
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("which"), DocParam.count(0)))
        split = split_at(doc, box(doc))
        above = doc.insert(Node.part(split, PartSelect.split_half(SplitHalf.Above)))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.bind_instance_param(above, ParamName("which")))
        self.assertEqual(caught.exception.variant, "unknown_slot")


class TestThePatternDoor(unittest.TestCase):
    """`Node.pattern` itself: the replication rule vocabulary over an
    UNFUSED family, beside `Node.placed_union`'s fused one."""

    def test_the_count_is_the_structural_slot(self):
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("n"), DocParam.count(2)))
        family = pattern_of(doc, box(doc), count=2)
        doc.apply(DocEdit.bind_count_param(family, ParamName("n")))
        for n in (2, 3, 5):
            with self.subTest(count=n):
                doc.apply(DocEdit.set_doc_param(ParamName("n"), DocParam.count(n)))
                self.assertEqual(len(evaluate(doc).value(family).bodies()), n)

    def test_a_count_below_one_refuses(self):
        doc = Doc()
        family = pattern_of(doc, box(doc), count=0)
        self.assertEqual(refusal(self, doc, family), "non_positive_count")

    def test_an_explicit_rule_refuses_at_insert(self):
        """The explicit rule carries its OWN placements, so pairing it
        with a count is two answers to one question — refused at the
        edit door, where `Node.placed_union` refuses it too."""
        doc = Doc()
        prototype = box(doc)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.pattern(prototype, Expr.count(2), PatternKind.explicit([])))
        self.assertEqual(caught.exception.variant, "placement_rule_mismatch")


class TestTheReadSide(unittest.TestCase):
    """What a reader sees: `Doc.node_kind` answers the constructor's
    own word for both new nodes, and a Part is a DAG edge the document
    refuses to delete out from under."""

    def test_node_kind_answers_part_and_pattern(self):
        doc = Doc()
        cube = box(doc)
        split = split_at(doc, cube)
        family = pattern_of(doc, cube)
        half = doc.insert(Node.part(split, PartSelect.split_half(SplitHalf.Below)))
        one = doc.insert(Node.part(family, PartSelect.instance(Expr.count(0))))
        self.assertEqual(doc.node_kind(family), "pattern")
        self.assertEqual(doc.node_kind(half), "part")
        self.assertEqual(doc.node_kind(one), "part")
        self.assertEqual(doc.node_kind(split), "split")

    def test_the_selected_value_is_a_dag_input(self):
        doc = Doc()
        family = pattern_of(doc, box(doc))
        doc.insert(Node.part(family, PartSelect.instance(Expr.count(0))))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.delete_node(family))
        self.assertEqual(caught.exception.variant, "delete_would_dangle")


if __name__ == "__main__":
    unittest.main()
