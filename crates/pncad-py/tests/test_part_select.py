"""One body out of a multi-body value, from Python (LIB-B-PART).

A split's half is an output of the split, read as `doc.output(split,
port)` — "the upper half of that split" — and `Node.part` says
"instance 1 of that pattern", giving a pattern's plural payload a
downstream door. The Python mirror of
`crates/editor-core/tests/corpus/part_select.rs` and the acceptance
rows in `crates/editor-core/tests/docm2_part.rs`, asserted through the
bound doors rather than restated as prose.

WHAT A READ OF A HALF MEANS, and what the rows below check. Nothing is
moved, re-stamped or recomputed: a reader of the half reads the
split's own side, so a still transform of it weighs what the side
`Value.split` hands back weighs (`TestTheHalfIsItsPort`), the two
halves unioned are the box again
(`test_the_halves_rejoin_into_the_box`), and the names pass through
verbatim, so a face read through a port is a face of the split
(`test_the_names_pass_through`). A Part's index is STRUCTURAL, so
moving it recomputes the Part and nothing upstream
(`test_moving_the_index_recomputes_the_part_alone`).
"""

import unittest

from pncad import (
    BooleanOp,
    Doc,
    DocEdit,
    FreeVar,
    EditError,
    EvaluationError,
    Formula,
    Node,
    VarName,
    PartSelect,
    PatternKind,
    PlaneRelation,
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
                (Formula.length_in(-half, m), Formula.length_in(-half, m)),
                (Formula.length_in(half, m), Formula.length_in(-half, m)),
                (Formula.length_in(half, m), Formula.length_in(half, m)),
                (Formula.length_in(-half, m), Formula.length_in(half, m)),
            ],
            plane=doc.sketch_frame(),
        )
    )
    return doc.insert(Node.extrude(square, Formula.length_in(height, m)))


def split_at(doc, target, z=CUT_Z):
    """`target` cut by a horizontal plane at height `z`."""
    tool = doc.insert(Node.datum_plane((
        Formula.length_in(0, m),
        Formula.length_in(0, m),
        Formula.length_in(z, m),
    ), (
        Formula.literal(0.0),
        Formula.literal(0.0),
        Formula.literal(1.0),
    )))
    return doc.insert(Node.split(target, tool))


def pattern_of(doc, prototype, count=COUNT, pitch=PITCH):
    """`count` copies of `prototype` stepped `pitch` apart along x."""
    return doc.insert(
        Node.pattern(prototype, Formula.count(count), PatternKind.linear((
            Formula.literal(1.0),
            Formula.literal(0.0),
            Formula.literal(0.0),
        ), Formula.length_in(pitch, m)))
    )


def still(doc, read):
    """A transform of `read` that moves nothing: a node holding the
    body a read of it hands on, names verbatim."""
    return doc.insert(
        Node.transform(read, (
            Formula.length_in(0, m),
            Formula.length_in(0, m),
            Formula.length_in(0, m),
        ), (
            Formula.literal(0.0),
            Formula.literal(0.0),
            Formula.literal(1.0),
        ), Formula.angle_in(0, rad))
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


class TestTheHalfIsItsPort(unittest.TestCase):
    """A split's half, read by port — the corpus document's first half.

    The oracle is the split's OWN value: `Value.split` hands back the
    two sides, and a reader of a port must hold exactly what that side
    weighs. Nothing here transcribes a number a read is then checked
    against; the two readings of the same body are compared.
    """

    def build(self):
        doc = Doc()
        cube = box(doc)
        split = split_at(doc, cube)
        above = still(doc, doc.output(split, 0))
        below = still(doc, doc.output(split, 1))
        return doc, split, above, below

    def test_each_half_weighs_what_the_split_says_it_weighs(self):
        doc, split, above, below = self.build()
        ev = evaluate(doc)
        self.assertEqual(ev.value(split).kind, "split")
        upper, lower = ev.value(split).split()
        for read, side, name in ((above, upper, "above"), (below, lower, "below")):
            with self.subTest(half=name):
                self.assertIsNotNone(side)
                self.assertEqual(ev.value(read).kind, "body")
                self.assertEqual(
                    mass_of(ev, read).volume, side.mass_properties().volume
                )
                self.assertEqual(
                    mass_of(ev, read).surface_area,
                    side.mass_properties().surface_area,
                )

    def test_the_two_halves_are_the_box_cut_where_the_plane_is(self):
        """The dyadic pin, stated once: a 2 x 2 x 1 box cut at 0.5 is
        two 2 x 2 x 0.5 slabs. Both oracles are exact binary fractions,
        so the comparison is `==` rather than a tolerance."""
        doc, _, above, below = self.build()
        ev = evaluate(doc)
        for read in (above, below):
            mass = mass_of(ev, read)
            self.assertEqual(mass.volume, 2.0 * 2.0 * 0.5)
            self.assertEqual(mass.surface_area, 2 * 2.0 * 2.0 + 4 * 2.0 * 0.5)

    def test_the_halves_rejoin_into_the_box(self):
        """The document's own statement that a port IS the half:
        nothing was moved, re-stamped or lost on the way through, so
        unioning the two back together is the box again.

        The two halves REST on each other across the section, which is
        a declared contact — detected through `find_flush_candidates`
        over the split itself, whose value holds both halves, and
        declared on the union. Both sides are sited at the split, and
        each is read in the half whose table holds it.
        """
        doc = Doc()
        split = split_at(doc, box(doc))
        ev = evaluate(doc)
        findings = ev.find_flush_candidates(split, split)
        section = [f for f in findings if f.relation == PlaneRelation.SameOpposite]
        self.assertTrue(section, "the section face pair")
        whole = doc.insert(
            Node.boolean(
                BooleanOp.Union,
                doc.output(split, 0),
                doc.output(split, 1),
                declare=section[:1],
            )
        )
        mass = mass_of(evaluate(doc), whole)
        self.assertEqual(mass.volume, 2.0 * 2.0 * 1.0)
        self.assertEqual(mass.surface_area, 2 * 2.0 * 2.0 + 4 * 2.0 * 1.0)

    def test_the_names_pass_through(self):
        """A read of a half hands on the split's rows for that half and
        otherwise verbatim — so every face name a reader answers with
        is a face name of the split, unchanged as text, and the two
        halves' name sets are disjoint."""
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
        middle = doc.insert(Node.part(family, PartSelect.instance(Formula.count(1))))
        ev = evaluate(doc)
        self.assertEqual(ev.value(family).kind, "instances")
        self.assertEqual(len(ev.value(family).bodies()), COUNT)
        self.assertEqual(ev.value(middle).kind, "body")

    def test_each_instance_weighs_what_the_pattern_says_it_weighs(self):
        doc, _, family = self.build()
        parts = [
            doc.insert(Node.part(family, PartSelect.instance(Formula.count(i))))
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
        middle = doc.insert(Node.part(family, PartSelect.instance(Formula.count(1))))
        lifted = doc.insert(
            Node.transform(middle, (
                Formula.length_in(0, m),
                Formula.length_in(0, m),
                Formula.length_in(2, m),
            ), (
                Formula.literal(0.0),
                Formula.literal(0.0),
                Formula.literal(1.0),
            ), Formula.angle_in(0, rad))
        )
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(lifted))
        self.assertEqual(mass_of(ev, lifted).volume, 2.0 * 2.0 * 1.0)


class TestAPartReadsCopies(unittest.TestCase):
    """A part reads a pattern's copies and nothing else, refused at
    insert: a plain body and a split's half are one body each
    (`slot_var_kind`), and a split named alone is two
    (`ambiguous_output`)."""

    def test_every_mismatch_refuses_at_insert(self):
        doc = Doc()
        cube = box(doc)
        split = split_at(doc, cube)
        index = PartSelect.instance(Formula.count(0))
        for of, label, tag in (
            (cube, "a plain body", "slot_var_kind"),
            (doc.output(split, 0), "a split's half", "slot_var_kind"),
            (split, "a split named alone", "ambiguous_output"),
        ):
            with self.subTest(case=label):
                with self.assertRaises(EditError) as caught:
                    doc.insert(Node.part(of, index))
                self.assertEqual(caught.exception.variant, tag)


class TestTheRefusalsAreTyped(unittest.TestCase):
    """The family's own two refusals, reached from Python for the first
    time — `empty_half` and `instance_out_of_range`, the two tags
    `crates/pncad-py/src/tags.rs` has carried since DOCM-2 with no
    caller able to construct either."""

    def test_a_half_with_no_material_refuses(self):
        """A tool plane that misses the box entirely: a reader of the
        empty side refuses, and a reader of the side WITH material still
        evaluates — the refusal is the read's, not the split's."""
        doc = Doc()
        cube = box(doc)
        split = split_at(doc, cube, z=2.0)
        above = still(doc, doc.output(split, 0))
        below = still(doc, doc.output(split, 1))
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
                node = doc.insert(Node.part(family, PartSelect.instance(Formula.count(index))))
                self.assertEqual(refusal(self, doc, node), "instance_out_of_range")

    def test_lowering_the_count_under_a_live_index_refuses(self):
        """The index is judged against the count as it stands NOW, so
        an edit upstream of the Part can invalidate it — and says so
        rather than quietly selecting a neighbour."""
        doc = Doc()
        doc.apply(DocEdit.declare_var(VarName("n"), FreeVar.count(COUNT)))
        family = pattern_of(doc, box(doc))
        doc.apply(DocEdit.bind_count_param(family, VarName("n")))
        live = doc.insert(Node.part(family, PartSelect.instance(Formula.count(2))))
        self.assertTrue(evaluate(doc).succeeded(live))

        doc.apply(DocEdit.define_var(VarName("n"), FreeVar.count(2)))
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
        doc.apply(DocEdit.declare_var(VarName("which"), FreeVar.count(0)))
        family = pattern_of(doc, box(doc))
        chosen = doc.insert(Node.part(family, PartSelect.instance(Formula.count(0))))
        doc.apply(DocEdit.bind_instance_param(chosen, VarName("which")))
        return doc, family, chosen

    def test_one_param_edit_moves_which_instance_is_selected(self):
        """The payoff: which copy a downstream consumer sees is a named
        number, and moving it is one `define_var`."""
        doc, family, chosen = self.build()
        for which in (0, 1, 2):
            with self.subTest(which=which):
                doc.apply(
                    DocEdit.define_var(VarName("which"), FreeVar.count(which))
                )
                ev = evaluate(doc)
                self.assertEqual(
                    mass_of(ev, chosen).volume,
                    ev.value(family).bodies()[which].mass_properties().volume,
                )
        # And the bound index is judged the same way a literal one is.
        doc.apply(DocEdit.define_var(VarName("which"), FreeVar.count(COUNT)))
        self.assertEqual(refusal(self, doc, chosen), "instance_out_of_range")

    def test_moving_the_index_recomputes_the_part_alone(self):
        """A projection moves nothing upstream of itself, so an index
        edit recomputes exactly one node — the memo's own statement
        that a Part is a projection."""
        doc, _, _chosen = self.build()
        first = evaluate(doc)
        doc.apply(DocEdit.define_var(VarName("which"), FreeVar.count(1)))
        again = evaluate(doc, prior=first)
        self.assertEqual(again.recomputed, 1)
        self.assertEqual(again.reused, len(doc) - 1)

    def test_neither_slot_door_reaches_the_others_slot(self):
        doc, family, chosen = self.build()
        with self.assertRaises(EditError) as no_count:
            doc.apply(DocEdit.bind_count_param(chosen, VarName("which")))
        self.assertEqual(no_count.exception.variant, "unknown_slot")
        with self.assertRaises(EditError) as no_instance:
            doc.apply(DocEdit.bind_instance_param(family, VarName("which")))
        self.assertEqual(no_instance.exception.variant, "unknown_slot")


class TestThePatternDoor(unittest.TestCase):
    """`Node.pattern` itself: the replication rule vocabulary over an
    UNFUSED family, beside `Node.placed_union`'s fused one."""

    def test_the_count_is_the_structural_slot(self):
        doc = Doc()
        doc.apply(DocEdit.declare_var(VarName("n"), FreeVar.count(2)))
        family = pattern_of(doc, box(doc), count=2)
        doc.apply(DocEdit.bind_count_param(family, VarName("n")))
        for n in (2, 3, 5):
            with self.subTest(count=n):
                doc.apply(DocEdit.define_var(VarName("n"), FreeVar.count(n)))
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
            doc.insert(Node.pattern(prototype, Formula.count(2), PatternKind.explicit([])))
        self.assertEqual(caught.exception.variant, "placement_rule_mismatch")
        self.assertEqual(caught.exception.inner_variant, "listed_on_pattern")


class TestTheReadSide(unittest.TestCase):
    """What a reader sees: `Doc.node_kind` answers the constructor's
    own word for both nodes, and a Part's read strands typed when what
    it reads is deleted."""

    def test_node_kind_answers_part_and_pattern(self):
        doc = Doc()
        cube = box(doc)
        family = pattern_of(doc, cube)
        one = doc.insert(Node.part(family, PartSelect.instance(Formula.count(0))))
        self.assertEqual(doc.node_kind(family), "pattern")
        self.assertEqual(doc.node_kind(one), "part")

    def test_the_selected_value_is_a_read(self):
        """Deleting the pattern a part reads is accepted, says the read
        it strands, and leaves the part refusing until re-pointed."""
        doc = Doc()
        family = pattern_of(doc, box(doc))
        one = doc.insert(Node.part(family, PartSelect.instance(Formula.count(0))))
        doc.apply(DocEdit.delete_node(family))
        stranded = [m for m in doc.last_maintenance if m.variant == "stranded_read"]
        self.assertEqual([m.node for m in stranded], [one])
        self.assertEqual(refusal(self, doc, one), "unresolved_read")


if __name__ == "__main__":
    unittest.main()
