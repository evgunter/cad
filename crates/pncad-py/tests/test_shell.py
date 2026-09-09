"""`Node.shell` — the hollowing door, through the public Python doors
only, mirroring `crates/editor-core/tests/lib_g17_shell_node.rs`.

The document is the `cup` corpus document re-authored in Python: a
box `L × L × H` hollowed to a wall `t` with its top opened into a rim.
The oracle is derived, not measured (the Rust module's docs carry the
derivation): the cavity is `(L−2t) × (L−2t) × (H−t)`, so

    V = L²H − (L−2t)²(H−t)
    A = 2L² + 4LH + 4(L−2t)(H−t)

and every dimension here is dyadic, so both are asserted EXACTLY.

The rebuild is spelled twice, because the two spellings say
different things. `DocEdit.set_param` moves the wall and the height on
the LIVE document, which is what a caller with a cup in hand does; the
row after it re-authors a second document at the bumped values and
asks the SAME name text of it, which is what a caller keeping names
across documents does. Both land on the closed form.
"""

import unittest

from pncad import (
    CapEnd,
    Doc,
    DocEdit,
    EntityKind,
    EvaluationError,
    Expr,
    NamePat,
    Node,
    OpGroup,
    SegPat,
    SegTag,
    Selector,
    WrittenLength,
    evaluate,
    m,
)

L, H, T = 1.0, 1.0, 0.125
H_BUMPED, T_BUMPED = 1.5, 0.25


def closed_forms(side, h, t):
    inner = side - 2 * t
    return (
        side * side * h - inner * inner * (h - t),
        2 * side * side + 4 * side * h + 4 * inner * (h - t),
    )


def blank(doc, side, h):
    """A box `side × side × h` on the document's sketch frame."""
    square = doc.insert(
        Node.polygon(
            [
                (Expr.written_length(WrittenLength.in_unit(0, m)), Expr.written_length(WrittenLength.in_unit(0, m))),
                (Expr.written_length(WrittenLength.in_unit(side, m)), Expr.written_length(WrittenLength.in_unit(0, m))),
                (Expr.written_length(WrittenLength.in_unit(side, m)), Expr.written_length(WrittenLength.in_unit(side, m))),
                (Expr.written_length(WrittenLength.in_unit(0, m)), Expr.written_length(WrittenLength.in_unit(side, m))),
            ],
            plane=doc.sketch_frame(),
        )
    )
    return doc.insert(Node.extrude(square, Expr.written_length(WrittenLength.in_unit(h, m))))


def top_of(doc, box):
    """The box's top face by ROLE: the extrude's end cap."""
    faces = evaluate(doc).select(
        box,
        Selector.of(
            NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(CapEnd.End))
        ),
    )
    assert len(faces) == 1
    return faces[0]


def cup(doc, side=L, h=H, t=T):
    box = blank(doc, side, h)
    return box, doc.insert(Node.shell(box, Expr.written_length(WrittenLength.in_unit(t, m)), [top_of(doc, box)]))


def mass(doc, node):
    return evaluate(doc).value(node).body().mass_properties()


class TestCup(unittest.TestCase):
    def test_the_cup_is_exactly_its_closed_form(self):
        doc = Doc()
        _box, hollow = cup(doc)
        body = evaluate(doc).value(hollow).body()
        body.validate()
        want_v, want_a = closed_forms(L, H, T)
        props = body.mass_properties()
        self.assertEqual(props.volume, want_v)
        self.assertEqual(props.surface_area, want_a)
        # Five outer faces, the rim annulus, five cavity faces.
        self.assertEqual(len(evaluate(doc).all_faces(hollow)), 11)

    def test_an_empty_open_list_is_the_sealed_hollow(self):
        doc = Doc()
        box = blank(doc, L, H)
        sealed = doc.insert(Node.shell(box, Expr.written_length(WrittenLength.in_unit(T, m)), []))
        body = evaluate(doc).value(sealed).body()
        body.validate()
        inner = L - 2 * T
        self.assertEqual(body.mass_properties().volume, L * L * H - inner * inner * (H - 2 * T))
        # Two complete boxes: no rim was minted.
        ev = evaluate(doc)
        self.assertEqual(len(ev.all_faces(sealed)), 12)
        rims = ev.select(
            sealed, Selector.of(NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Rim)))
        )
        self.assertEqual(rims, [])

    def test_the_rim_inner_and_outer_names_resolve(self):
        doc = Doc()
        box, hollow = cup(doc)
        ev = evaluate(doc)
        faces = NamePat.of_kind(EntityKind.Face)
        rim = ev.select(hollow, Selector.of(faces.seg(SegPat.tag(SegTag.Rim))))
        self.assertEqual(len(rim), 1, "one designated chart, one rim")
        inner = ev.select(hollow, Selector.of(faces.seg(SegPat.tag(SegTag.Inner))))
        self.assertEqual(len(inner), 5, "the cavity floor and its four walls")
        outer = ev.select(hollow, Selector.of(faces.seg(SegPat.tag(SegTag.FromTarget))))
        self.assertEqual(len(outer), 5, "the bottom and the four sides carried through")
        # The shell's three roles group as the shell's; the survivors
        # speak as the carried-through shape.
        by_group = ev.select(hollow, Selector.of(faces.seg(SegPat.group(OpGroup.Shell))))
        self.assertEqual(sorted(by_group), sorted(rim + inner))
        # The rim wraps the top's own name, and the top's own name is
        # gone from the cup: what a selector says for the mouth is the
        # rim, never the carried-through top.
        top = top_of(doc, box)
        wrapped = ev.select(hollow, Selector.of(faces.seg(SegPat.tag(SegTag.Rim).of([NamePat.any()]))))
        self.assertEqual(wrapped, rim)
        self.assertIn(top, rim[0])
        self.assertTrue(all(top not in name for name in outer))
        for name in rim + inner + outer:
            self.assertEqual(ev.resolve(name).status, "resolved")

    def test_the_live_document_rebuilds_through_the_slot_edits(self):
        """The cup at bumped values without re-authoring anything: two
        `set_param` edits move the extrude's `distance` and the
        shell's `shell_thickness`, and the SAME document lands on the
        bumped closed form with its names still resolving."""
        doc = Doc()
        box, hollow = cup(doc)
        self.assertEqual(mass(doc, hollow).volume, closed_forms(L, H, T)[0])
        names = evaluate(doc).select(
            hollow,
            Selector.of(NamePat.of_kind(EntityKind.Face).seg(SegPat.group(OpGroup.Shell))),
        )

        doc.apply(DocEdit.set_param(box, "distance", doc.parse_expr(f"{H_BUMPED} m")))
        doc.apply(
            DocEdit.set_param(hollow, "shell_thickness", doc.parse_expr(f"{T_BUMPED} m"))
        )

        ev = evaluate(doc)
        props = ev.value(hollow).body().mass_properties()
        want_v, want_a = closed_forms(L, H_BUMPED, T_BUMPED)
        self.assertEqual(props.volume, want_v)
        self.assertEqual(props.surface_area, want_a)
        for name in names:
            self.assertEqual(ev.resolve(name).status, "resolved", name)

    def test_the_same_recipe_at_bumped_values_answers_the_same_names(self):
        doc = Doc()
        _box, hollow = cup(doc)
        ev = evaluate(doc)
        faces = NamePat.of_kind(EntityKind.Face)
        names = ev.select(hollow, Selector.of(faces.seg(SegPat.group(OpGroup.Shell))))
        # The same recipe at the bumped height and wall: node ids and
        # therefore name text coincide, and every name still resolves.
        bumped = Doc()
        _box2, hollow2 = cup(bumped, L, H_BUMPED, T_BUMPED)
        ev2 = evaluate(bumped)
        for name in names:
            self.assertEqual(ev2.resolve(name).status, "resolved", name)
        props = ev2.value(hollow2).body().mass_properties()
        want_v, want_a = closed_forms(L, H_BUMPED, T_BUMPED)
        self.assertEqual(props.volume, want_v)
        self.assertEqual(props.surface_area, want_a)


class TestRefusals(unittest.TestCase):
    def refusal(self, doc, node):
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(node)
        return caught.exception

    def test_an_unresolvable_open_name_refuses_typed(self):
        # A name the target never minted, read off a sibling document
        # whose recipe has the same node ids and one more wall: a
        # pentagon prism's fifth wall names an entity the square box's
        # extrude never had, so it resolves to nothing there.
        pentagon = Doc()
        five = pentagon.insert(
            Node.polygon(
                [
                    (Expr.written_length(WrittenLength.in_unit(0, m)), Expr.written_length(WrittenLength.in_unit(0, m))),
                    (Expr.written_length(WrittenLength.in_unit(1, m)), Expr.written_length(WrittenLength.in_unit(0, m))),
                    (Expr.written_length(WrittenLength.in_unit(1.5, m)), Expr.written_length(WrittenLength.in_unit(0.5, m))),
                    (Expr.written_length(WrittenLength.in_unit(1, m)), Expr.written_length(WrittenLength.in_unit(1, m))),
                    (Expr.written_length(WrittenLength.in_unit(0, m)), Expr.written_length(WrittenLength.in_unit(1, m))),
                ],
                plane=pentagon.sketch_frame(),
            )
        )
        prism = pentagon.insert(Node.extrude(five, Expr.written_length(WrittenLength.in_unit(H, m))))
        walls = evaluate(pentagon).select(
            prism,
            Selector.of(NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Lateral))),
        )
        self.assertEqual(len(walls), 5)
        doc = Doc()
        box = blank(doc, L, H)
        self.assertEqual(
            len(set(walls) - set(evaluate(doc).all_faces(box))), 1, "one wall the box lacks"
        )
        ghost = sorted(set(walls) - set(evaluate(doc).all_faces(box)))[0]
        node = doc.insert(Node.shell(box, Expr.written_length(WrittenLength.in_unit(T, m)), [ghost]))
        self.assertEqual(self.refusal(doc, node).kind, "shell_open_resolve")

    def test_an_edge_in_the_open_list_refuses_typed(self):
        doc = Doc()
        box = blank(doc, L, H)
        edge = evaluate(doc).all_edges(box)[0]
        node = doc.insert(Node.shell(box, Expr.written_length(WrittenLength.in_unit(T, m)), [edge]))
        self.assertEqual(self.refusal(doc, node).kind, "shell_open_kind")

    def test_a_non_positive_wall_is_the_kernels_refusal(self):
        doc = Doc()
        box = blank(doc, L, H)
        node = doc.insert(Node.shell(box, Expr.written_length(WrittenLength.in_unit(-0.125, m)), [top_of(doc, box)]))
        refusal = self.refusal(doc, node)
        self.assertEqual(refusal.kind, "shell")
        self.assertIn("not certifiably positive", str(refusal))

    def test_a_name_is_carried_not_composed(self):
        doc = Doc()
        box = blank(doc, L, H)
        with self.assertRaises(ValueError):
            Node.shell(box, Expr.written_length(WrittenLength.in_unit(T, m)), ["the top face"])

    def test_the_designation_order_is_kept_and_a_repeat_keeps_its_first(self):
        doc = Doc()
        box = blank(doc, L, H)
        ev = evaluate(doc)
        top = top_of(doc, box)
        bottom = ev.select(
            box,
            Selector.of(
                NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(CapEnd.Start))
            ),
        )[0]
        forward = Doc(label="shell-order")
        backward = Doc(label="shell-order")
        doubled = Doc(label="shell-order")
        for target, order in (
            (forward, [top, bottom]),
            (backward, [bottom, top]),
            (doubled, [top, bottom, top]),
        ):
            target.insert(Node.shell(blank(target, L, H), Expr.written_length(WrittenLength.in_unit(T, m)), order))
        # Order is meaning, so the two orders are two documents; a
        # repeat keeps its first occurrence, so the doubled list is the
        # forward one bit for bit.
        self.assertFalse(forward.bit_eq(backward))
        self.assertTrue(forward.bit_eq(doubled))


if __name__ == "__main__":
    unittest.main()
