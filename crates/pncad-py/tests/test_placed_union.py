"""The group boolean and its placement vocabulary, from Python.

The Python mirror of `crates/editor-core/tests/lib_placedunion.rs`:
ONE prototype, a placement rule, ONE BODY OUT. The load-bearing rows
are the same ones — the group equals the transform-union chain it
replaces, the rule's refusals are typed, and per-instance names are
one segment deep — asserted through the bound doors rather than
restated as prose.
"""

import unittest

from pncad import (
    BooleanOp,
    Doc,
    DocEdit,
    DocParam,
    EditError,
    EntityKind,
    EvaluationError,
    Frame,
    FrameError,
    NamePat,
    Node,
    ParamName,
    PatternKind,
    SegPat,
    SegTag,
    Selector,
    deg,
    evaluate,
    m,
    rad,
)


def slab(doc, x, y, z):
    """The axis-aligned box [x0,x1] x [y0,y1] x [z0,z1], in metres."""
    profile = doc.insert(
        Node.polygon(
            [
                (x[0] * m, y[0] * m),
                (x[1] * m, y[0] * m),
                (x[1] * m, y[1] * m),
                (x[0] * m, y[1] * m),
            ],
            elevation=z[0] * m,
        )
    )
    return doc.insert(Node.extrude(profile, (z[1] - z[0]) * m))


def mass_of(doc, node):
    ev = evaluate(doc)
    assert ev.succeeded(node), "the group evaluated"
    body = ev.value(node).body()
    body.validate()
    return body.mass_properties()


# The heat-sink fin, constant for constant with the corpus document
# `heat_sink_fins`: footprint 0.1875 x 0.75 at z = 0.1875, extruded
# 0.8125, stepped 0.3125 apart — which leaves 0.125 of clear air
# between neighbours, the clearance the disjointness certificate needs.
PITCH = 0.3125
FIN_VOLUME = 0.1875 * 0.75 * 0.8125
FIN_AREA = 2 * 0.140625 + 2 * (0.1875 + 0.75) * 0.8125


def fin_only(doc):
    """The fin prototype alone — no base, no group."""
    return slab(doc, (0.25, 0.4375), (0.125, 0.875), (0.1875, 1.0))


class TestTheFinGroup(unittest.TestCase):
    """ONE node, ONE body: the group says with a single `PlacedUnion`
    what the tour's heat sink says with a pattern plus five transforms
    and five booleans."""

    def test_the_group_is_one_node_and_one_body(self):
        doc = Doc()
        fin = fin_only(doc)
        before = len(doc)
        group = doc.insert(
            Node.placed_union(fin, 5, PatternKind.linear((1.0, 0.0, 0.0), PITCH * m))
        )
        self.assertEqual(len(doc) - before, 1)

        ev = evaluate(doc)
        # An ordinary BODY — which is precisely what a pattern's plural
        # `instances` payload is not, and why a group can feed a
        # boolean where a pattern cannot.
        self.assertEqual(ev.value(group).kind, "body")
        mass = mass_of(doc, group)
        self.assertEqual(mass.volume, 5 * FIN_VOLUME)
        self.assertEqual(mass.surface_area, 5 * FIN_AREA)

    def test_the_group_equals_the_transform_union_chain(self):
        """The design's byte-level promise: the group and the chain it
        replaces are the same solid, bit for bit on both oracles."""
        chain_doc = Doc()
        fin = fin_only(chain_doc)
        acc = None
        for i in range(5):
            placed = chain_doc.insert(
                Node.transform(fin, (i * PITCH * m, 0 * m, 0 * m), (0.0, 0.0, 1.0), 0 * rad)
            )
            acc = (
                placed
                if acc is None
                else chain_doc.insert(Node.boolean(BooleanOp.Union, acc, placed))
            )

        group_doc = Doc()
        group = group_doc.insert(
            Node.placed_union(
                fin_only(group_doc), 5, PatternKind.linear((1.0, 0.0, 0.0), PITCH * m)
            )
        )

        chain, grouped = mass_of(chain_doc, acc), mass_of(group_doc, group)
        self.assertEqual(chain.volume, grouped.volume)
        self.assertEqual(chain.surface_area, grouped.surface_area)

        chain_ev, group_ev = evaluate(chain_doc), evaluate(group_doc)

        def census(ev, node):
            return (
                len(ev.all_faces(node)),
                len(ev.all_edges(node)),
                len(ev.all_vertices(node)),
            )

        self.assertEqual(census(chain_ev, acc), census(group_ev, group))

    def test_every_instance_is_one_instance_segment_deep(self):
        """The pairwise chain buries the first copy's faces under one
        qualifier per union; the group gives every instance the SAME
        one-segment instance qualifier, whatever the count."""
        doc = Doc()
        group = doc.insert(
            Node.placed_union(
                fin_only(doc), 5, PatternKind.linear((1.0, 0.0, 0.0), PITCH * m)
            )
        )
        ev = evaluate(doc)
        names = ev.all_faces(group)
        # Six faces per fin, five fins, all distinct.
        self.assertEqual(len(names), 30)
        self.assertEqual(len(set(names)), 30)

        # DEPTH, asserted through the selector's role-path shape (which
        # is an EXACT length, segment for segment) rather than by
        # reading inside the opaque name text: every one of those faces
        # sits under exactly ONE instance segment...
        one_deep = Selector.of(
            NamePat.of_kind(EntityKind.Face).path([SegPat.tag(SegTag.Instance)])
        )
        self.assertEqual(sorted(ev.select(group, one_deep)), sorted(names))
        # ...and none sits under two, which is the shape a pairwise
        # union chain would have grown one segment at a time.
        two_deep = Selector.of(
            NamePat.of_kind(EntityKind.Face).path(
                [SegPat.tag(SegTag.Instance), SegPat.any()]
            )
        )
        self.assertEqual(ev.select(group, two_deep), [])


class TestThePlacementRuleRefuses(unittest.TestCase):
    """Every fault the shared placement-rule door names, executed from
    Python. One vocabulary for the edit door, the persist re-check and
    the evaluation backstop — so these tags are the Rust ones."""

    def test_an_explicit_rule_has_no_count_slot(self):
        doc = Doc()
        fin = fin_only(doc)
        rule = PatternKind.explicit([Frame.translation((0 * m, 0 * m, 0 * m))])
        with self.assertRaises(EditError) as caught:
            Node.placed_union(fin, 1, rule)
        self.assertEqual(caught.exception.variant, "placement_rule_mismatch")

    def test_an_empty_placement_list_refuses(self):
        doc = Doc()
        box = slab(doc, (0, 1), (0, 1), (0, 1))
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.placed_union_at(box, []))
        self.assertEqual(caught.exception.variant, "empty_placement_list")

    def test_a_non_finite_frame_refuses(self):
        """A zero rotation axis normalizes to NaN, so the frame is
        non-finite — refused, never read as "no rotation"."""
        doc = Doc()
        box = slab(doc, (0, 1), (0, 1), (0, 1))
        poisoned = Frame.rotate_then_translate(
            (0.0, 0.0, 0.0), 90 * deg, (0 * m, 0 * m, 0 * m)
        )
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.placed_union_at(box, [poisoned]))
        self.assertEqual(caught.exception.variant, "non_finite_placement")

    def test_an_improper_frame_refuses(self):
        """A mirror is REPRESENTABLE so that it can be refused (A6,
        pending the equivariance audit) — the reason a frame stores a
        general linear part rather than an axis-angle triple."""
        doc = Doc()
        box = slab(doc, (0, 1), (0, 1), (0, 1))
        mirror = Frame.mirror_across_plane((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0))
        self.assertLess(mirror.determinant, 0.0)
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.placed_union_at(box, [mirror]))
        self.assertEqual(caught.exception.variant, "improper_placement")

    def test_overlapping_placements_refuse_at_evaluate(self):
        """Disjointness is CERTIFIED, never declared. Two placements of
        one unit box a quarter apart interpenetrate; nothing is built,
        and the refusal is typed."""
        doc = Doc()
        box = slab(doc, (0, 1), (0, 1), (0, 1))
        group = doc.insert(
            Node.placed_union_at(
                box,
                [
                    Frame.translation((0 * m, 0 * m, 0 * m)),
                    Frame.translation((0.25 * m, 0 * m, 0 * m)),
                ],
            )
        )
        ev = evaluate(doc)
        self.assertFalse(ev.succeeded(group))
        with self.assertRaises(EvaluationError) as caught:
            ev.value(group)
        self.assertEqual(caught.exception.kind, "placements_uncertified")

    def test_touching_boxes_over_disjoint_solids_refuse_the_same_way(self):
        """The certificate is sufficient-not-necessary, so a
        box-touching but genuinely disjoint arrangement refuses too —
        the honest answer, refinable later, never a silent maybe."""
        doc = Doc()
        box = slab(doc, (0, 1), (0, 1), (0, 1))
        rotated = Frame.rotate_then_translate(
            (0.0, 0.0, 1.0), 45 * deg, (1.4 * m, 0 * m, 0 * m)
        )
        group = doc.insert(
            Node.placed_union_at(
                box, [Frame.translation((0 * m, 0 * m, 0 * m)), rotated]
            )
        )
        ev = evaluate(doc)
        with self.assertRaises(EvaluationError) as caught:
            ev.value(group)
        self.assertEqual(caught.exception.kind, "placements_uncertified")


class TestTheFrameValue(unittest.TestCase):
    """The placement value itself: constructed, read back, compared."""

    def test_the_accessors_read_the_frame_back(self):
        frame = Frame.translation((1 * m, 2 * m, 3 * m))
        self.assertEqual(frame.columns, ((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)))
        self.assertEqual(frame.origin, (1 * m, 2 * m, 3 * m))
        self.assertEqual(frame.determinant, 1.0)

    def test_equality_is_bit_exact(self):
        """A frame carries no epsilon, so `-0.0` keeps its own
        identity — the sketch-plane rule, unchanged."""
        zero = Frame.translation((0 * m, 0 * m, 0 * m))
        minus = Frame.translation((-0.0 * m, 0 * m, 0 * m))
        self.assertEqual(zero, Frame.translation((0 * m, 0 * m, 0 * m)))
        self.assertNotEqual(zero, minus)
        self.assertEqual(hash(zero), hash(Frame.translation((0 * m, 0 * m, 0 * m))))
        self.assertEqual(len({zero, minus}), 2)

    def test_a_placement_and_a_transform_agree_bit_for_bit(self):
        """The D9 promise the group-versus-chain equality rests on:
        `rotate_then_translate` is the transform node's own
        composition order, normalized at the same step."""
        doc = Doc()
        box = slab(doc, (0, 1), (0, 1), (0, 1))
        placed = doc.insert(
            Node.placed_union_at(
                box,
                [
                    Frame.rotate_then_translate(
                        (0.0, 0.0, 2.0), 30 * deg, (5 * m, 0 * m, 0 * m)
                    )
                ],
            )
        )
        moved = doc.insert(
            Node.transform(box, (5 * m, 0 * m, 0 * m), (0.0, 0.0, 2.0), 30 * deg)
        )
        self.assertEqual(mass_of(doc, placed).volume, mass_of(doc, moved).volume)

    def test_the_frame_trio_builds_and_refuses(self):
        """`point_at`, `path_start_frame`, `mirror_across_plane` — the
        frame constructors, with their typed refusals."""
        aimed = Frame.point_at(
            (0 * m, 0 * m, 0 * m), (0 * m, 0 * m, 1 * m), (0.0, 1.0, 0.0)
        )
        self.assertAlmostEqual(aimed.determinant, 1.0, places=12)
        self.assertEqual(aimed.origin, (0 * m, 0 * m, 0 * m))

        start = Frame.path_start_frame((0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0))
        self.assertAlmostEqual(start.determinant, 1.0, places=12)

        mirror = Frame.mirror_across_plane((0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0))
        self.assertAlmostEqual(mirror.determinant, -1.0, places=12)

        # A roll reference ALONG the aim fixes no roll, and refuses
        # rather than picking a fallback.
        with self.assertRaises(FrameError) as caught:
            Frame.point_at(
                (0 * m, 0 * m, 0 * m), (0 * m, 0 * m, 1 * m), (0.0, 0.0, 1.0)
            )
        self.assertEqual(caught.exception.variant, "degenerate_roll_reference")

        with self.assertRaises(FrameError) as caught:
            Frame.point_at(
                (0 * m, 0 * m, 0 * m), (0 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0)
            )
        self.assertEqual(caught.exception.variant, "degenerate_aim")

        with self.assertRaises(FrameError) as caught:
            Frame.path_start_frame((0 * m, 0 * m, 0 * m), (0.0, 0.0, 0.0))
        self.assertEqual(caught.exception.variant, "degenerate_tangent")

        with self.assertRaises(FrameError) as caught:
            Frame.mirror_across_plane((0 * m, 0 * m, 0 * m), (0.0, 0.0, 0.0))
        self.assertEqual(caught.exception.variant, "degenerate_mirror_normal")


class TestTheCircularRule(unittest.TestCase):
    def test_a_circular_group_places_around_a_datum_axis(self):
        """The rule vocabulary is the pattern node's, reused whole:
        the stepped map is literally the same one."""
        doc = Doc()
        box = slab(doc, (2, 3), (-0.5, 0.5), (0, 1))
        axis = doc.insert(Node.datum_axis((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0)))
        group = doc.insert(
            Node.placed_union(box, 4, PatternKind.circular(axis, 90 * deg))
        )
        self.assertEqual(mass_of(doc, group).volume, 4.0)


class TestTheCountParamBinding(unittest.TestCase):
    """`bind_count_param` — the narrowed structural-slot edit. The
    count stops being a literal and becomes a named number one
    `set_doc_param` away from any other value."""

    def build(self):
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("fins"), DocParam.count(2)))
        group = doc.insert(
            Node.placed_union(
                fin_only(doc), 2, PatternKind.linear((1.0, 0.0, 0.0), PITCH * m)
            )
        )
        return doc, group

    def test_the_bound_count_follows_the_document_parameter(self):
        doc, group = self.build()
        doc.apply(DocEdit.bind_count_param(group, ParamName("fins")))
        self.assertEqual(mass_of(doc, group).volume, 2 * FIN_VOLUME)
        doc.apply(DocEdit.set_doc_param(ParamName("fins"), DocParam.count(4)))
        self.assertEqual(mass_of(doc, group).volume, 4 * FIN_VOLUME)

    def test_binding_an_unknown_parameter_refuses(self):
        doc, group = self.build()
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.bind_count_param(group, ParamName("nope")))
        self.assertEqual(caught.exception.variant, "unknown_doc_param")

    def test_binding_a_parameter_of_the_wrong_dimension_refuses(self):
        """The slot is a Count, and a Length parameter is not one —
        the edit's own dimension check, arriving unchanged."""
        doc, group = self.build()
        doc.apply(DocEdit.set_doc_param(ParamName("width"), DocParam.length(1 * m)))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.bind_count_param(group, ParamName("width")))
        self.assertEqual(caught.exception.variant, "doc_param_dimension_mismatch")

    def test_an_explicit_group_has_no_count_slot_to_bind(self):
        """The list IS the count, so there is nothing for a parameter
        to drive — the two-sources-of-truth state, refused."""
        doc = Doc()
        doc.apply(DocEdit.set_doc_param(ParamName("fins"), DocParam.count(2)))
        group = doc.insert(
            Node.placed_union_at(
                fin_only(doc), [Frame.translation((0 * m, 0 * m, 0 * m))]
            )
        )
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.bind_count_param(group, ParamName("fins")))
        self.assertEqual(caught.exception.variant, "unknown_slot")


if __name__ == "__main__":
    unittest.main()
