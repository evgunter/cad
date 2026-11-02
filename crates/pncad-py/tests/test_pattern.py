"""Patterns are index variables, from Python (D10 `[repetition]`).

`k = index(N)` is a variable (`VarDecl.index`); a placement whose values
read it is evaluated once per value of `k`, so its copy is a family of
`N` members. The pattern writers (`Node.linear_pattern`, `Node.grid`,
`Node.circular_pattern`, `Node.bolt_circle`) author that placement and
nothing else; one member is read `row[i]` (`row[i, j]` for a grid), the
family whole is `Node.union(row)`, and `Doc.recognise` reads a placement
back as the row or ring it is.

The Python mirror of `crates/editor-core/tests/lib_placedunion.rs` and
the member rows of `crates/editor-core/tests/docm2_part.rs`, asserted
through the bound doors.
"""

import unittest

from pncad import (
    Doc,
    DocEdit,
    EditError,
    EntityKind,
    Formula,
    Frame,
    FreeVar,
    NamePat,
    Node,
    PoseDef,
    RoleRead,
    SegPat,
    SegTag,
    Selector,
    VarDecl,
    VarName,
    deg,
    evaluate,
    m,
)

from placing import against, own_frame, placed
from measuring import pinned
from test_part_select import box, mass_of, refusal, still

#: A row's member count and its pitch along x, clear of the box's 2 m
#: width.
COUNT = 3
PITCH = 3.0


def index(doc, name, count, within=None):
    """`name = index(count)`, declared in `doc`; answers its name."""
    k = VarName(name)
    doc.apply(DocEdit.declare_var(k, VarDecl.index(count, within)))
    return k


def row_of(doc, prototype, count=COUNT, pitch=PITCH, name="k"):
    """`count` copies of `prototype` in its own space, member `k` slid
    `k` pitches along x."""
    k = index(doc, name, count)
    return doc.insert(
        Node.linear_pattern(k, prototype, doc.space(prototype), [("x", Formula.length_in(pitch, m))])
    )


def slab(doc, x, y, z):
    """The box [x0,x1] x [y0,y1] x [0, z1 - z0] in its own coordinates,
    in metres."""
    profile = doc.insert(
        Node.polygon(
            [
                (Formula.length_in(x[0], m), Formula.length_in(y[0], m)),
                (Formula.length_in(x[1], m), Formula.length_in(y[0], m)),
                (Formula.length_in(x[1], m), Formula.length_in(y[1], m)),
                (Formula.length_in(x[0], m), Formula.length_in(y[1], m)),
            ],
        )
    )
    return doc.insert(Node.extrude(profile, Formula.length_in(z[1] - z[0], m)))


def z_axis(doc, block):
    """The z axis through the origin of the block `slab(doc, (2, 3),
    (-0.5, 0.5), ...)` draws, read off its geometry: its frame at its
    corner `(2, -0.5, 0)`, its z slid back to the origin."""
    frame = own_frame(doc, block, (2.0, -0.5, 0.0), (3.0, -0.5, 0.0))
    return PoseDef.slide(
        PoseDef.slide(RoleRead(frame, "z"), RoleRead(frame, "x"), -2 * m),
        RoleRead(frame, "y"),
        0.5 * m,
    )


# The heat-sink fin, constant for constant with the corpus document
# `heat_sink_fins`: footprint 0.1875 x 0.75, extruded 0.8125, stepped
# 0.3125 apart.
FIN_PITCH = 0.3125
FIN_VOLUME = 0.1875 * 0.75 * 0.8125
FIN_AREA = 2 * 0.140625 + 2 * (0.1875 + 0.75) * 0.8125


def fin_only(doc):
    """The fin prototype alone."""
    return slab(doc, (0.25, 0.4375), (0.125, 0.875), (0.1875, 1.0))


def fins(doc, count=5):
    """The fin row and its union."""
    row = row_of(doc, fin_only(doc), count=count, pitch=FIN_PITCH)
    return row, doc.insert(Node.union(row))


class TestTheRowIsAFamily(unittest.TestCase):
    """A row's value is its members, and its union one body."""

    def test_the_rows_value_is_plural_and_a_members_is_not(self):
        doc = Doc()
        family = row_of(doc, box(doc))
        middle = still(doc, family[1])
        ev = evaluate(doc)
        self.assertEqual(ev.value(family).kind, "instances")
        self.assertEqual(len(ev.value(family).bodies()), COUNT)
        self.assertEqual(
            [at for at, _ in ev.value(family).members()], [[i] for i in range(COUNT)]
        )
        self.assertEqual(len(ev.value(middle).bodies()), 1)

    def test_each_member_weighs_what_the_row_says_it_weighs(self):
        doc = Doc()
        family = row_of(doc, box(doc))
        reads = [still(doc, family[i]) for i in range(COUNT)]
        ev = evaluate(doc)
        bodies = ev.value(family).bodies()
        for i, read in enumerate(reads):
            with self.subTest(member=i):
                self.assertEqual(mass_of(ev, read).volume, pinned(bodies[i]).volume)
                # A rigid step moves a body and changes neither oracle.
                self.assertEqual(mass_of(ev, read).volume, 2.0 * 2.0 * 1.0)

    def test_a_member_is_an_ordinary_operand(self):
        """The middle member, lifted by a placement."""
        doc = Doc()
        family = row_of(doc, box(doc))
        lifted = placed(doc, family[1], Frame.translation((0 * m, 0 * m, 2 * m)))
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(lifted))
        self.assertEqual(mass_of(ev, lifted).volume, 2.0 * 2.0 * 1.0)

    def test_the_fin_union_is_one_body(self):
        doc = Doc()
        _row, group = fins(doc)
        mass = mass_of(evaluate(doc), group)
        self.assertEqual(mass.volume, 5 * FIN_VOLUME)
        self.assertEqual(mass.surface_area, 5 * FIN_AREA)

    def test_the_fin_union_equals_the_placement_union_chain(self):
        """The row's union and the chain it replaces — each later fin a
        copy placed against the first, `i` pitches along x — are the
        same solid on both oracles and in their census."""
        chain_doc = Doc()
        fin = fin_only(chain_doc)
        acc = fin
        for i in range(1, 5):
            copy = against(chain_doc, fin, fin, Frame.translation((i * FIN_PITCH * m, 0 * m, 0 * m)))
            acc = chain_doc.insert(Node.union([acc, copy]))
        row_doc = Doc()
        _row, group = fins(row_doc)
        chain_ev, row_ev = evaluate(chain_doc), evaluate(row_doc)
        chain, grouped = mass_of(chain_ev, acc), mass_of(row_ev, group)
        self.assertEqual(chain.volume, grouped.volume)
        self.assertEqual(chain.surface_area, grouped.surface_area)

        def census(ev, node):
            return (len(ev.all_faces(node)), len(ev.all_edges(node)), len(ev.all_vertices(node)))

        self.assertEqual(census(chain_ev, acc), census(row_ev, group))

    def test_every_member_is_one_member_segment_deep(self):
        """Every member's faces sit under exactly ONE member segment,
        whatever the count."""
        doc = Doc()
        row, _group = fins(doc)
        ev = evaluate(doc)
        names = ev.all_faces(row)
        # Six faces per fin, five fins, all distinct.
        self.assertEqual(len(names), 30)
        self.assertEqual(len(set(names)), 30)
        one_deep = Selector.of(NamePat.of_kind(EntityKind.Face).path([SegPat.tag(SegTag.Member)]))
        self.assertEqual(sorted(ev.select(row, one_deep)), sorted(names))
        two_deep = Selector.of(
            NamePat.of_kind(EntityKind.Face).path([SegPat.tag(SegTag.Member), SegPat.any()])
        )
        self.assertEqual(ev.select(row, two_deep), [])


class TestAMemberReadReadsAFamily(unittest.TestCase):
    """`xs[i]` reads a family and nothing else, refused at insert: a
    plain body and a split's half are one body each (`slot_var_kind`),
    and a split named alone is two (`ambiguous_output`)."""

    def test_every_mismatch_refuses_at_insert(self):
        doc = Doc()
        cube = box(doc)
        split = doc.insert(
            Node.split(cube, PoseDef.plane(cube, sorted(evaluate(doc).all_faces(cube))[0]))
        )
        for of, label, tag in (
            (cube, "a plain body", "slot_var_kind"),
            (doc.output(split, 0), "a split's half", "slot_var_kind"),
            (split, "a split named alone", "ambiguous_output"),
        ):
            with self.subTest(case=label):
                with self.assertRaises(EditError) as caught:
                    doc.insert(Node.union([of[0]]))
                self.assertEqual(caught.exception.variant, tag)


class TestTheRefusalsAreTyped(unittest.TestCase):
    """The family's own refusals, by their tags."""

    def test_a_member_outside_the_count_refuses(self):
        """At the count and below zero, one refusal: neither is
        wrapped nor clamped."""
        doc = Doc()
        family = row_of(doc, box(doc))
        for at in (COUNT, -1):
            with self.subTest(member=at):
                node = still(doc, family[at])
                self.assertEqual(refusal(self, doc, node), "member_out_of_range")

    def test_lowering_the_count_under_a_live_read_refuses(self):
        """A member is judged against the count as it stands NOW, so an
        edit of the index's count can invalidate a read of it — and says
        so rather than quietly reading a neighbour."""
        doc = Doc()
        doc.apply(DocEdit.declare_var(VarName("n"), FreeVar.count(COUNT)))
        family = row_of(doc, box(doc), count=doc.parse_formula("n"))
        live = still(doc, family[2])
        self.assertTrue(evaluate(doc).succeeded(live))
        doc.apply(DocEdit.define_var(VarName("n"), FreeVar.count(2)))
        self.assertTrue(evaluate(doc).succeeded(family), "the row is Ok at two")
        self.assertEqual(refusal(self, doc, live), "member_out_of_range")

    def test_an_index_is_declared_as_a_count_within_an_index(self):
        """`declare_var` refuses an index whose count is no count, or
        which is declared within a variable that is no index."""
        doc = Doc()
        doc.apply(DocEdit.declare_var(VarName("n"), FreeVar.count(2)))
        for decl, inner, found in (
            (VarDecl.index(Formula.length_in(2, m)), "count_not_count", "length"),
            (VarDecl.index(2, within=VarName("n")), "within_no_index", None),
        ):
            with self.subTest(case=inner):
                with self.assertRaises(EditError) as caught:
                    doc.apply(DocEdit.declare_var(VarName("q"), decl))
                self.assertEqual(caught.exception.variant, "index")
                self.assertEqual(caught.exception.inner_variant, inner)
                self.assertEqual(caught.exception.param, "q")
                self.assertEqual(caught.exception.found, found)

    def test_two_unrelated_indices_meet_in_no_reader(self):
        doc = Doc()
        cube = box(doc)
        k, j = index(doc, "k", 2), index(doc, "j", 2)
        with self.assertRaises(EditError) as caught:
            doc.insert(
                Node.grid(
                    (k, "x", Formula.length_in(PITCH, m)),
                    (j, "y", Formula.length_in(PITCH, m)),
                    cube,
                    doc.space(cube),
                )
            )
        self.assertEqual(caught.exception.variant, "indices_unrelated")

    def test_a_pitch_is_a_length(self):
        doc = Doc()
        cube = box(doc)
        k = index(doc, "k", 2)
        with self.assertRaises(ValueError):
            Node.linear_pattern(k, cube, doc.space(cube), [("x", Formula.angle_in(90, deg))])
        with self.assertRaises(ValueError):
            Node.linear_pattern(k, cube, doc.space(cube), [("w", Formula.length_in(1, m))])


class TestTheCountIsTheIndexs(unittest.TestCase):
    """The count is the index's definition, so one `define_var` of a
    parameter it reads recounts the family, and a member read at a
    count variable is moved by one edit too."""

    def test_one_param_edit_recounts_the_family(self):
        doc = Doc()
        doc.apply(DocEdit.declare_var(VarName("n"), FreeVar.count(2)))
        family = row_of(doc, box(doc), count=doc.parse_formula("n"))
        for n in (2, 3, 5, 0):
            with self.subTest(count=n):
                doc.apply(DocEdit.define_var(VarName("n"), FreeVar.count(n)))
                self.assertEqual(len(evaluate(doc).value(family).bodies()), n)

    def test_one_param_edit_recounts_the_fins(self):
        doc = Doc()
        doc.apply(DocEdit.declare_var(VarName("fins"), FreeVar.count(5)))
        row = row_of(doc, fin_only(doc), count=doc.parse_formula("fins"), pitch=FIN_PITCH)
        group = doc.insert(Node.union(row))
        for count in (5, 7, 9):
            with self.subTest(fins=count):
                doc.apply(DocEdit.define_var(VarName("fins"), FreeVar.count(count)))
                mass = mass_of(evaluate(doc), group)
                self.assertEqual(mass.volume, count * FIN_VOLUME)
                self.assertEqual(mass.surface_area, count * FIN_AREA)

    def test_one_param_edit_moves_which_member_is_read(self):
        doc = Doc()
        doc.apply(DocEdit.declare_var(VarName("which"), FreeVar.count(0)))
        family = row_of(doc, box(doc))
        chosen = still(doc, family[doc.parse_formula("which")])
        for which in (0, 1, 2):
            with self.subTest(which=which):
                doc.apply(DocEdit.define_var(VarName("which"), FreeVar.count(which)))
                ev = evaluate(doc)
                self.assertEqual(
                    mass_of(ev, chosen).volume,
                    pinned(ev.value(family).bodies()[which]).volume,
                )
        doc.apply(DocEdit.define_var(VarName("which"), FreeVar.count(COUNT)))
        self.assertEqual(refusal(self, doc, chosen), "member_out_of_range")


class TestTheGridAndTheRing(unittest.TestCase):
    def test_a_grid_is_indexed_outer_then_inner(self):
        doc = Doc()
        cube = box(doc)
        i = index(doc, "i", 2)
        j = index(doc, "j", 3, within=i)
        grid = doc.insert(
            Node.grid(
                (i, "x", Formula.length_in(PITCH, m)),
                (j, "y", Formula.length_in(PITCH, m)),
                cube,
                doc.space(cube),
            )
        )
        corner = still(doc, grid[1, 2])
        ev = evaluate(doc)
        self.assertEqual(
            [at for at, _ in ev.value(grid).members()],
            [[a, b] for a in range(2) for b in range(3)],
        )
        self.assertEqual(mass_of(ev, corner).volume, 4.0)
        self.assertEqual(len(ev.value(grid).bodies()), 6)

    def test_a_ring_places_around_an_axis(self):
        doc = Doc()
        block = slab(doc, (2, 3), (-0.5, 0.5), (0, 1))
        axis = z_axis(doc, block)
        k = index(doc, "k", 4)
        ring = doc.insert(
            Node.circular_pattern(
                doc, k, block, doc.space(block), (axis, axis, "z"), step=Formula.angle_in(90, deg)
            )
        )
        group = doc.insert(Node.union(ring))
        self.assertEqual(mass_of(evaluate(doc), group).volume, 4.0)

    def test_a_bolt_circle_is_a_full_ring(self):
        doc = Doc()
        block = slab(doc, (2, 3), (-0.5, 0.5), (0, 1))
        axis = z_axis(doc, block)
        k = index(doc, "k", 4)
        ring = doc.insert(Node.bolt_circle(doc, k, 4, block, doc.space(block), (axis, axis, "z")))
        group = doc.insert(Node.union(ring))
        ev = evaluate(doc)
        self.assertEqual(len(ev.value(ring).bodies()), 4)
        self.assertEqual(mass_of(ev, group).volume, 4.0)

    def test_a_ring_takes_exactly_one_spacing(self):
        doc = Doc()
        block = slab(doc, (2, 3), (-0.5, 0.5), (0, 1))
        axis = z_axis(doc, block)
        k = index(doc, "k", 4)
        for spacing in ({}, {"count": 4, "step": Formula.angle_in(90, deg)}):
            with self.subTest(spacing=sorted(spacing)):
                with self.assertRaises(TypeError):
                    Node.circular_pattern(doc, k, block, doc.space(block), (axis, axis, "z"), **spacing)


class TestTheReadSide(unittest.TestCase):
    """What a reader sees: a row is a placement, read back as the
    pattern it is; a read of a member strands typed when the row is
    deleted."""

    def test_a_row_is_a_placement_recognised_as_a_row(self):
        doc = Doc()
        cube = box(doc)
        family = row_of(doc, cube)
        self.assertEqual(doc.node_kind(family), "place")
        seen = doc.recognise(family)
        self.assertEqual(seen.kind, "linear")
        self.assertEqual(doc.var_name(seen.index), VarName("k"))
        self.assertEqual([axis for axis, _ in seen.steps], ["x"])
        self.assertIsNone(seen.step)
        self.assertIsNone(doc.recognise(cube))

    def test_a_ring_is_recognised_as_a_ring(self):
        doc = Doc()
        block = slab(doc, (2, 3), (-0.5, 0.5), (0, 1))
        axis = z_axis(doc, block)
        k = index(doc, "k", 4)
        ring = doc.insert(Node.bolt_circle(doc, k, 4, block, doc.space(block), (axis, axis, "z")))
        seen = doc.recognise(ring)
        self.assertEqual(seen.kind, "circular")
        self.assertEqual(seen.steps, [])
        self.assertEqual(seen.step.dimension, "angle")

    def test_a_grid_is_recognised_as_a_grid(self):
        doc = Doc()
        cube = box(doc)
        i = index(doc, "i", 2)
        j = index(doc, "j", 3, within=i)
        grid = doc.insert(
            Node.grid(
                (i, "x", Formula.length_in(PITCH, m)),
                (j, "y", Formula.length_in(PITCH, m)),
                cube,
                doc.space(cube),
            )
        )
        seen = doc.recognise(grid)
        self.assertEqual(seen.kind, "grid")
        self.assertEqual(doc.var_name(seen.index), VarName("i"))
        self.assertEqual(doc.var_name(seen.inner_index), VarName("j"))
        self.assertEqual([axis for axis, _ in seen.steps], ["x"])
        self.assertEqual([axis for axis, _ in seen.inner_steps], ["y"])
        self.assertIsNone(seen.step)

    def test_the_member_read_is_a_read(self):
        """Deleting the row a member read reads is accepted, says the
        read it strands, and leaves the reader refusing until
        re-pointed."""
        doc = Doc()
        family = row_of(doc, box(doc))
        one = still(doc, family[0])
        doc.apply(DocEdit.delete_node(family))
        stranded = [s for s in doc.last_maintenance if s.variant == "stranded_read"]
        self.assertEqual([s.node for s in stranded], [one])
        self.assertEqual(refusal(self, doc, one), "unresolved_read")


if __name__ == "__main__":
    unittest.main()
