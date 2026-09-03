"""The group boolean and its placement vocabulary, from Python.

The Python mirror of `crates/editor-core/tests/lib_placedunion.rs`:
ONE prototype, a placement rule, ONE BODY OUT. The load-bearing rows
are the same ones — the group equals the transform-union chain it
replaces, the rule's refusals are typed, and per-instance names are
one segment deep — asserted through the bound doors rather than
restated as prose.

Both corpus twins are here: the LINEAR rule's `heat_sink_fins`
(extrude-only, bound at LIB-PYPU) and the EXPLICIT rule's `die_tool`,
whose prototype is a REVOLVE about a `Datum::Axis` and which was
banked behind that half until LIB-DIETOOL measured it cleared.
"""

import math
import unittest
from pathlib import Path

from pncad import (
    BooleanOp,
    Bulge,
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
    Open,
    ParamName,
    PatternKind,
    SegPat,
    SegTag,
    Selector,
    SketchPlane,
    Start,
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
            plane=doc.sketch_frame(elevation=z[0] * m),
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


# The die's cutting tool, constant for constant with the corpus
# document `die_tool` (`crates/editor-core/tests/corpus/die_tool.rs`):
# a unit cube, a radius-0.09 ball whose centre stands R - H outside
# each face plane so the cavity is a cap of height exactly H, one ball
# per face.
DIE_L = 1.0
PIP_R = 0.09
PIP_H = 0.05
PIP_C = DIE_L + (PIP_R - PIP_H)


def pip_placements():
    """The six face-centre frames, in the corpus document's order and
    with its rotations: each carries the master ball's +Z pole onto the
    face normal it cuts, so every chart stays polar to the plane that
    cuts it. Every angle is 0, +-pi/2 or pi about a coordinate axis —
    the placement is DATA, which is what an explicit rule is for."""
    h = DIE_L / 2.0
    lo = DIE_L - PIP_C  # the -normal faces' centre coordinate
    x = (1.0, 0.0, 0.0)
    y = (0.0, 1.0, 0.0)
    z = (0.0, 0.0, 1.0)
    rows = [
        (z, 0.0, (h, h, PIP_C)),
        (x, math.pi, (h, h, lo)),
        (y, math.pi / 2.0, (PIP_C, h, h)),
        (y, -math.pi / 2.0, (lo, h, h)),
        (x, -math.pi / 2.0, (h, PIP_C, h)),
        (x, math.pi / 2.0, (h, lo, h)),
    ]
    return [
        Frame.rotate_then_translate(
            axis, angle * rad, (t[0] * m, t[1] * m, t[2] * m)
        )
        for axis, angle, t in rows
    ]


def die_tool_document():
    """`die_tool` re-authored through the bound Python doors, node for
    node in the corpus document's insert order.

    The label is the one `tests/fixture/mod.rs::Recorder` derives from,
    so the document's IDENTITY matches too and the saved-text pin below
    compares whole lines rather than a redacted subset.
    """
    doc = Doc("mod")

    # ---- the sharp cube, [0, L]^3 ----
    square = doc.insert(
        Node.polygon(
            [
                (0 * m, 0 * m),
                (DIE_L * m, 0 * m),
                (DIE_L * m, DIE_L * m),
                (0 * m, DIE_L * m),
            ],
            plane=doc.sketch_frame(elevation=0 * m),
        )
    )
    cube = doc.insert(Node.extrude(square, DIE_L * m))

    # ---- the master ball, poled along +Z ----
    # `die_pips::half_disc_program` verbatim: ONE bulge-1 semicircle
    # pole to pole, closed by its on-axis diameter. BOTH vertices are
    # on the revolve axis — the chart the retired equator workaround
    # existed to dodge, and the whole Revolve/datum half of this
    # document.
    plane = doc.sketch_frame(
        plane=SketchPlane.from_frame(
            (0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0), (0.0, 0.0, 1.0)
        )
    )
    # The revolve axis is written IN that frame: world +Z is the
    # frame's own v direction, so the meridian's pole-to-pole line is
    # (0, 1) through the origin. Being in the plane is no longer a
    # tolerance question — it is what the four numbers mean.
    axis = doc.insert(Node.datum_axis_in_plane(plane, (0 * m, 0 * m), (0.0, 1.0)))
    half_disc = (
        Open.at((0 * m, -PIP_R * m))
        .arc_to(Bulge((0 * m, PIP_R * m), 1.0))
        .line_to(Start)
    )
    ball_p = doc.insert(Node.profile(half_disc, plane=plane))
    ball = doc.insert(Node.revolve(ball_p, axis, (2.0 * math.pi) * rad))

    # ---- the whole cutting tool, in ONE node ----
    tool = doc.insert(Node.placed_union_at(ball, pip_placements()))
    pipped = doc.insert(
        Node.boolean(BooleanOp.Subtract, cube, tool)
    )
    return doc, ball, tool, pipped


class TestTheDieTool(unittest.TestCase):
    """The EXPLICIT rule's corpus twin, `die_tool`, authored from
    Python — one prototype ball, six listed frames, one body out, fed
    straight into a Subtract.

    This is the row `work/lib/log.md` carried as "die_tool's Python
    re-authoring (banked behind its Revolve/datum half)". The bank was
    the ball: `heat_sink_fins` is extrude-only, while this prototype is
    a `Node.revolve` about a `Node.datum_axis` whose meridian runs pole
    to pole. LIB-DIETOOL measured the half CLEARED — the natural
    meridian is what the document below authors, and it is not a
    re-chart of anything.
    """

    # The corpus document's own saved bytes, pinned by
    # `crates/editor-core/tests/lib_dietool_crossing.rs`. It cannot
    # rot: that test re-authors the registered document and writes this
    # file, so a recipe change on either side is a red run here.
    FIXTURE = (
        Path(__file__).resolve().parents[3]
        / "crates" / "editor-core" / "tests" / "corpus" / "die_tool.pncad"
    )

    def test_the_tool_is_one_node_and_still_cuts(self):
        """The tour's recipe spends N transforms and N-1 unions on this
        shape; the group spends ONE node — and the collapsed tool is
        still a legal boolean operand."""
        doc, _ball, tool, pipped = die_tool_document()
        ev = evaluate(doc)

        # NINE nodes: frame, profile, extrude, frame, profile, datum,
        # revolve, group, subtract. Two of the nine are the sketch
        # frames the cube and the meridian are drawn on — the cube's
        # is the xy plane, the meridian's is the xz plane, and they
        # are different planes, so they are different nodes. The
        # pairwise tool this replaces spends the same seven upstream
        # and then six transforms, five unions and the subtract, so
        # the group's saving is the eleven it collapses into one.
        # Which of the nine is the group is not asserted by counting
        # kinds here (the document layer exposes no node-kind read
        # door); it is settled outright by the byte pin below, whose
        # text names every node's kind.
        self.assertEqual(len(doc), 9)

        # An ordinary BODY out of the group — the property that lets a
        # boolean consume it at all.
        self.assertEqual(ev.value(tool).kind, "body")

        body = ev.value(pipped).body()
        body.validate()
        # Six cavities, each contributing its faces to the ONE solid:
        # 6 box faces + 6 x 2 cap half-bands.
        self.assertEqual(len(ev.all_faces(pipped)), 18)

        # The oracle is `die_pips`': L^3 - 6 * cap(R, H), pi-valued and
        # so not dyadic — asserted at rounding scale, which is why the
        # corpus document carries no mass pin either.
        cap = math.pi * PIP_H ** 2 * (3.0 * PIP_R - PIP_H) / 3.0
        want = DIE_L ** 3 - 6.0 * cap
        self.assertAlmostEqual(
            body.mass_properties().volume, want, delta=1e-12 * want
        )

    def test_every_cavity_face_is_one_instance_segment_deep(self):
        """The pairwise tool buries the FIRST ball's cavity faces under
        one qualifier per union; the group gives every instance the
        same ONE-segment qualifier, whatever the pip count."""
        doc, _ball, tool, _pipped = die_tool_document()
        ev = evaluate(doc)

        names = ev.all_faces(tool)
        # Two band half-faces per ball, six balls, all distinct.
        self.assertEqual(len(names), 12)
        self.assertEqual(len(set(names)), 12)

        one_deep = Selector.of(
            NamePat.of_kind(EntityKind.Face).path([SegPat.tag(SegTag.Instance)])
        )
        self.assertEqual(sorted(ev.select(tool, one_deep)), sorted(names))
        two_deep = Selector.of(
            NamePat.of_kind(EntityKind.Face).path(
                [SegPat.tag(SegTag.Instance), SegPat.any()]
            )
        )
        self.assertEqual(ev.select(tool, two_deep), [])

    def test_the_re_authoring_is_the_corpus_document_byte_for_byte(self):
        """The claim this unit exists to make: what Python authors is
        not a lookalike of `die_tool`, it IS `die_tool`.

        Every line of the saved text matches the registered document's
        — identity, node bodies, programs, expressions — except the
        snapshot's ONE `epsilon` line, which CI's tolerance rows sweep
        by design (`crates/pncad/tests/all.rs`'s plate_param pin states
        that disposition; this row inherits it)."""
        doc, _ball, _tool, _pipped = die_tool_document()

        def sans_epsilon(text):
            kept, excluded = [], []
            for line in text.splitlines():
                (excluded if line.lstrip().startswith('"epsilon":') else kept).append(line)
            # Exactly one such line per side: a missing or duplicated
            # epsilon is fixture damage, not sweep variance.
            self.assertEqual(len(excluded), 1, excluded)
            return "\n".join(kept)

        committed = self.FIXTURE.read_text(encoding="utf-8")
        self.assertEqual(
            sans_epsilon(doc.save()),
            sans_epsilon(committed),
            "the Python re-authoring and the corpus document have diverged — "
            "regenerate the fixture with `PNCAD_BLESS=1 cargo test -p editor-core "
            "--test all lib_dietool_crossing` and read the diff",
        )


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
