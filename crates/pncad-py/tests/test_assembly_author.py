"""AUTHORING an assembly from Python: the tour's bench, built from nothing.

`test_assembly_eval.py` is this file's other half. It takes the tour's
own committed corpus through the persistence door and evaluates it,
because at LIB-G18a Python could evaluate an assembly and not write
one. Here nothing arrives from disk that this file did not put there:
two part documents are authored, written into a workspace, instantiated
into two assembly documents, mated, solved, gathered and gated — the
whole of the audit's rows 46 and 47 through the public doors.

THE ORACLE, AND WHERE IT COMES FROM
-----------------------------------
`demos/tour/src/assembly.rs` is the scene these documents ARE, and its
own assertions are the oracle: the layout's five disjoint solids of
`4 x post + shelf`, the stand's cluster of three instances with the
earliest as gauge, the far post's solved translation, the identity
rotation two aligned frame-coincidence mates compose to, two minted
declarations, and a gate that certifies. Every number below is that
scene's, re-derived from the same five base dimensions rather than
copied as a total.

TWO THINGS THIS FILE CANNOT SAY, AND THEY ARE NOT DEFECTS OF IT
---------------------------------------------------------------
1. `shells().count()`. The scene counts solids in the gathered body;
   `Body` answers mass properties, validators and a mesh, and no
   count. What stands in for it here is the VOLUME, which for disjoint
   solids is the sum the shell count is being used to check, plus the
   per-instance name set, which is structural. Binding a solid count
   is a `Body` question and not this unit's.
2. Nothing else. "Instance 2's post cap sits at (x, y, z)" — the
   scene's name-lookup stop — was the second entry here until
   LIB-B-READBACK bound `face_frame`; it is now asserted below,
   against the placement arithmetic rather than against a
   transcribed coordinate.

WHICH `RefusedRef` ARMS THIS FILE REACHES, AND WHY NOT THE OTHERS
----------------------------------------------------------------
`ref_not_a_face` is reached below, by authoring a mate against an
edge. The other three are MEASURED as unreachable from Python
authoring today, which is a finding about the doors and not a gap in
this file:

* `ref_node_gone` — the reference's minting node is not in the
  document. Deleting the instance a mate names does get there in
  principle, but the mate then fails to solve and the GATHER refuses
  first (`root_failed`), so the gate never resolves the reference.
* `ref_vanished` — no product entity answers to the name. Reaching it
  wants the referenced part to change shape under a name the assembly
  still holds, and that is exactly what the pin gate refuses
  (`part_pin_mismatch`) one door earlier.
* `ref_ambiguous` — a tie. Nothing the instantiate seam's naming
  produces is tied, and Python cannot hand-build a name.

Each is bound and tagged; a reach appears when some other door does.

THE ONE SUBSTITUTION, STATED
----------------------------
The layout's four posts are `Node.placed_union`, not `Node.Pattern`.
`Pattern`'s value is a PLURAL payload and stays deliberately unbound
(G8's reason, unchanged); `placed_union` says the same placed family as
ONE node whose value is an ordinary body, and over a disjoint
arrangement it gathers the same material. The volume assertion below is
what holds that claim to the scene's own number rather than to a
sentence about it. It is the same substitution rows 43-45 are `YES*`
on.
"""

import math
import tempfile
import unittest
from pathlib import Path

import pncad
from pncad import (
    Alignment,
    AxisSense,
    CapEnd,
    ContactClass,
    Doc,
    DocEdit,
    DocRef,
    EntityKind,
    Frame,
    MateFrame,
    MatePrimitive,
    MateRole,
    NamePat,
    Node,
    PatternKind,
    SegPat,
    SegTag,
    Selector,
    Workspace,
    assemble,
    content_pin,
    evaluate,
    m,
    product,
    random_document_id,
    solve_document,
)

# The scene's five base dimensions, in metres.
POST_SECTION = 0.12
POST_HEIGHT = 0.5
SHELF_LENGTH = 0.9
SHELF_DEPTH = 0.30
SHELF_THICKNESS = 0.04

# Derived exactly as the scene derives them: where the shelf's
# underside meets each post, in SHELF coordinates, and where a post's
# top meets it in POST coordinates. The posts sit FLUSH with the
# shelf's two ends, which is the obvious way to draw a bench.
SEAT_A = (POST_SECTION / 2.0, SHELF_DEPTH / 2.0, 0.0)
SEAT_B = (SHELF_LENGTH - POST_SECTION / 2.0, SHELF_DEPTH / 2.0, 0.0)
POST_SEAT = (POST_SECTION / 2.0, POST_SECTION / 2.0, POST_HEIGHT)

POST_VOLUME = POST_SECTION * POST_SECTION * POST_HEIGHT
SHELF_VOLUME = SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS

PATTERN_COUNT = 4
PATTERN_STEP = 0.2


def prism(label, width, depth, height):
    """A rectangular prism part document, rooted at the origin."""
    doc = Doc(label)
    profile = doc.insert(
        Node.polygon(
            [
                (0 * m, 0 * m),
                (width * m, 0 * m),
                (width * m, depth * m),
                (0 * m, depth * m),
            ],
            plane=doc.sketch_frame(elevation=0 * m),
        )
    )
    doc.insert(Node.extrude(profile, height * m))
    return doc


def cap_selector(side, wrapper=None):
    """A cap face, optionally seen through one or two name wrappers.

    The whole point of the wrapper argument: a part's own cap name and
    the same face seen through the instance that placed it are the SAME
    query one nesting deeper. Nothing here reads inside a name.
    """
    pat = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(SegTag.Cap).side(side))
    for tag in reversed(wrapper or []):
        pat = NamePat.of_kind(EntityKind.Face).seg(SegPat.tag(tag).of([pat]))
    return Selector.of(pat)


def one(found):
    assert len(found) == 1, f"expected exactly one name, got {found}"
    return found[0]


def mate_frame(origin):
    """The scene's mate frames: +z axis, +x clocking reference."""
    return MateFrame(
        origin=(origin[0] * m, origin[1] * m, origin[2] * m),
        axis=(0.0, 0.0, 1.0),
        reference=(1.0, 0.0, 0.0),
    )


def seat(a_frame, b_frame, primitive=None):
    """The scene's alignment: two frames meeting, axes aligned, no
    clocking rider."""
    return Alignment(
        mate_frame(a_frame),
        mate_frame(b_frame),
        primitive or MatePrimitive.frame_coincidence(),
        AxisSense.Aligned,
    )


class BenchWorkspace(unittest.TestCase):
    """A store holding the two part documents, authored here."""

    def setUp(self):
        self.dir = Path(tempfile.mkdtemp())
        self.addCleanup(lambda: __import__("shutil").rmtree(self.dir, True))
        self.ws = Workspace(str(self.dir))
        self.post = prism("pncad-demo-post", POST_SECTION, POST_SECTION, POST_HEIGHT)
        self.shelf = prism(
            "pncad-demo-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS
        )
        self.ws.create(self.post)
        self.ws.create(self.shelf)
        self.post_ref = DocRef(self.post.id, content_pin(self.post))
        self.shelf_ref = DocRef(self.shelf.id, content_pin(self.shelf))

    def instance_face(self, doc, node, side):
        """A face of an instance's product, in the ASSEMBLY's names.

        The mate-authoring flow, and the reason no name has to be
        hand-composed: instantiate, evaluate against the store, then
        SELECT on the instantiate node. What comes back is the part's
        own name already wrapped at the instance that placed it, which
        is what a mate reference is.
        """
        found = evaluate(doc, resolver=self.ws).select(
            node, cap_selector(side, [SegTag.InPart])
        )
        return one(found)


class TestBenchLayout(BenchWorkspace):
    """Row 47: the flat-pack. Four posts on their side and the shelf
    beside them, nothing touching — A5's disjoint half."""

    def layout(self):
        doc = Doc("pncad-demo-layout")
        post_i = doc.insert(Node.instantiate_part(self.post_ref))
        # The post is laid on its SIDE: a rotation, which is why the
        # frame stores a general linear part and not a translation.
        doc.apply(
            DocEdit.set_placement(
                post_i,
                Frame.rotate_then_translate(
                    (0.0, 1.0, 0.0),
                    -math.pi / 2 * pncad.rad,
                    (POST_HEIGHT * m, 0 * m, 0 * m),
                ),
            )
        )
        family = doc.insert(
            Node.placed_union(
                post_i,
                PATTERN_COUNT,
                PatternKind.linear((0.0, 1.0, 0.0), PATTERN_STEP * m),
            )
        )
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref))
        doc.apply(
            DocEdit.set_placement(shelf_i, Frame.translation((0 * m, 0.9 * m, 0 * m)))
        )
        return doc, family, post_i, shelf_i

    def test_the_layout_gathers_the_scenes_material_exactly(self):
        doc, family, _, shelf_i = self.layout()
        # The roots state the product: the family (which consumed the
        # instance) and the shelf instance, in that order.
        self.assertEqual(doc.roots, [family, shelf_i])
        ev = evaluate(doc, resolver=self.ws)
        volume = product(doc, ev).mass_properties().volume
        self.assertAlmostEqual(
            volume, PATTERN_COUNT * POST_VOLUME + SHELF_VOLUME, delta=1e-12
        )

    def test_every_patterned_post_answers_to_an_instance_qualified_name(self):
        doc, family, _, _ = self.layout()
        ev = evaluate(doc, resolver=self.ws)
        caps = ev.select(
            family, cap_selector(CapEnd.Top, [SegTag.Instance, SegTag.InPart])
        )
        # One per placement, and the name NESTS rather than
        # concatenating: pattern index, then instance, then the part's
        # own cap. Four distinct names is the structural claim the
        # volume cannot make.
        self.assertEqual(len(caps), PATTERN_COUNT)
        self.assertEqual(len(set(caps)), PATTERN_COUNT)

    def test_the_disjoint_layout_passes_the_at_rest_gate_outright(self):
        doc, _, _, _ = self.layout()
        assembly = assemble(doc, evaluate(doc, resolver=self.ws))
        # No mate declares anything and nothing touches, so the
        # kernel's at-rest door passes with nothing minted. That is
        # exactly what "disjoint assemblies validate today" means.
        self.assertEqual(assembly.minted, [])
        self.assertGreater(len(assembly.names), 0)
        self.assertAlmostEqual(
            assembly.body.mass_properties().volume,
            PATTERN_COUNT * POST_VOLUME + SHELF_VOLUME,
            delta=1e-12,
        )

    def test_a_patterned_caps_frame_is_where_the_placement_puts_it(self):
        """The scene's name-lookup stop: where does instance 2's post
        cap SIT?

        The oracle is the model asked twice, never a transcribed
        coordinate. The PART document answers where its own cap sits,
        on its own evaluation; the placement `set_placement` was given
        maps that point into the assembly; and the pattern rule steps
        it along +y once per instance. What `face_frame` answers on
        the LAYOUT must be that ladder, one rung per instance.

        The index is read off the GEOMETRY, not off the name: an
        instance-qualified name is opaque text and the role segment
        the Rust tour filters on (`Instance { i: 2 }`) is deliberately
        unreadable from Python. Which is the honest shape here — the
        question is where a cap sits, and the answer is what
        identifies it.
        """
        doc, family, post_i, _ = self.layout()
        ev = evaluate(doc, resolver=self.ws)

        # Rung 1: the part's own cap, in the part's own coordinates.
        part_ev = evaluate(self.post)
        part_root = self.post.roots[0]
        part_cap = one(part_ev.select(part_root, cap_selector(CapEnd.Top)))
        local = part_ev.face_frame(part_root, part_cap).origin

        # Rung 2: the placement the layout gave that instance, applied
        # as the affine map it is — the frame's own columns and
        # origin, not a hand-written matrix.
        frame = doc.placement(post_i)
        cols, shift = frame.columns, frame.origin
        placed = tuple(
            sum(cols[j][axis] * local[j].meters for j in range(3)) + shift[axis].meters
            for axis in range(3)
        )

        # Rung 3: the pattern steps +y once per instance.
        expected = [
            (placed[0], placed[1] + i * PATTERN_STEP, placed[2])
            for i in range(PATTERN_COUNT)
        ]

        caps = ev.select(
            family, cap_selector(CapEnd.Top, [SegTag.Instance, SegTag.InPart])
        )
        read = sorted(
            tuple(c.meters for c in ev.face_frame(family, cap).origin) for cap in caps
        )
        self.assertEqual(len(read), PATTERN_COUNT)
        for got, want in zip(read, sorted(expected), strict=True):
            for got_axis, want_axis in zip(got, want, strict=True):
                self.assertAlmostEqual(got_axis, want_axis, delta=1e-12)

        # And the scene's own sentence, which is about instance 2:
        # two 200 mm steps along +y put its post between y = 0.4 and
        # y = 0.4 + section, and exactly one cap frame lies there.
        band = [o for o in read if 0.4 <= o[1] <= 0.4 + POST_SECTION]
        self.assertEqual(len(band), 1, f"instance 2's cap alone: {read}")

    def test_a_cap_name_denotes_one_face_and_says_so_before_it_is_read(self):
        """`denotation` is the door to ask BEFORE a frame: the frame
        doors refuse a tie rather than picking a candidate, and this
        says whether one is coming. The scene's names are all
        unique — which is a fact worth ASSERTING, because it is why
        every `face_frame` above answered at all."""
        doc, family, _, _ = self.layout()
        ev = evaluate(doc, resolver=self.ws)
        caps = ev.select(
            family, cap_selector(CapEnd.Top, [SegTag.Instance, SegTag.InPart])
        )
        for cap in caps:
            denotation = ev.denotation(family, cap)
            self.assertFalse(denotation.tied)
            self.assertEqual(denotation.candidates, 1)

    def test_an_instance_carries_no_frame_until_one_is_set(self):
        doc = Doc("unplaced")
        node = doc.insert(Node.instantiate_part(self.post_ref))
        # `placement` is TOTAL and answers the identity; `placements`
        # is what distinguishes "at the identity" from "no row".
        self.assertNotIn(node, doc.placements())
        self.assertEqual(doc.placement(node).origin, (0 * m, 0 * m, 0 * m))
        doc.apply(DocEdit.set_placement(node, Frame.translation((1 * m, 0 * m, 0 * m))))
        self.assertIn(node, doc.placements())


class TestBenchStand(BenchWorkspace):
    """Row 46: the assembled bench. Two posts and a shelf, the shelf
    SEATED on the posts by mates — only the gauge post carries an
    authored frame, and the other two poses are solved."""

    def stand(self, primitive=None, class_=ContactClass.Rest):
        doc = Doc("pncad-demo-stand")
        post_a = doc.insert(Node.instantiate_part(self.post_ref))
        doc.apply(
            DocEdit.set_placement(
                post_a,
                Frame.translation(
                    (0 * m, (SHELF_DEPTH - POST_SECTION) / 2 * m, 0 * m)
                ),
            )
        )
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref))
        post_b = doc.insert(Node.instantiate_part(self.post_ref))
        a_top = self.instance_face(doc, post_a, CapEnd.Top)
        b_top = self.instance_face(doc, post_b, CapEnd.Top)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Bottom)
        mate_1 = doc.insert(
            Node.mate(a_top, s_bottom, class_, seat(POST_SEAT, SEAT_A, primitive))
        )
        mate_2 = doc.insert(
            Node.mate(s_bottom, b_top, class_, seat(SEAT_B, POST_SEAT, primitive))
        )
        return doc, (post_a, shelf_i, post_b), (mate_1, mate_2)

    def test_the_mates_couple_the_three_instances_into_one_cluster(self):
        doc, (post_a, shelf_i, post_b), (mate_1, mate_2) = self.stand()
        self.assertEqual(pncad.clusters(doc), [[post_a, shelf_i, post_b]])
        # The gauge is the cluster's earliest instance in document
        # order, and every member agrees on it.
        for node in (post_a, shelf_i, post_b):
            self.assertEqual(pncad.gauge_of(doc, node), post_a)
        # A mate's references are not recipe edges; the READING edges
        # are what couples the graph, recomputed from the name heads.
        self.assertEqual(
            set(pncad.reading_edges(doc)),
            {
                (mate_1, post_a),
                (mate_1, shelf_i),
                (mate_2, shelf_i),
                (mate_2, post_b),
            },
        )

    def test_last_maintenance_describes_the_last_accepted_edit_at_every_door(self):
        """`last_maintenance` says "the LAST accepted edit", and `Doc`
        has four doors that accept one: `apply`, `insert`, `declare`
        and `declare_all`. A door that swaps the document without the
        record leaves the reading describing an EARLIER edit, which is
        worse than no reading — it is a plausible one about the wrong
        subject."""
        doc, (post_a, _shelf_i, post_b), (mate_1, mate_2) = self.stand()
        # `insert`: the stand's second mate joins post_b's cluster into
        # post_a's, and that join is what the door just accepted.
        joins = doc.last_maintenance
        self.assertEqual([r.variant for r in joins], ["join"])
        self.assertEqual((joins[0].survived, joins[0].absorbed), (post_a, post_b))

        def slab(x, y, z):
            """A box, inserted through the same `insert` door."""
            profile = doc.insert(
                Node.polygon(
                    [(x[0], y[0]), (x[1], y[0]), (x[1], y[1]), (x[0], y[1])],
                    plane=doc.sketch_frame(elevation=z[0]),
                )
            )
            return doc.insert(Node.extrude(profile, z[1] - z[0]))

        # `insert` again, on an edit that moves no mate graph: the
        # record is now EMPTY, not the join still standing from before.
        lower = slab((0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        upper = slab((0.25 * m, 0.75 * m), (0.25 * m, 0.75 * m), (1 * m, 1.5 * m))
        self.assertEqual(doc.last_maintenance, [])
        # `apply`: deleting a mate splits the cluster it coupled.
        doc.apply(DocEdit.delete_node(mate_2))
        self.assertEqual([r.variant for r in doc.last_maintenance], ["split"])
        # `declare` and `declare_all`, each from that split: a declared
        # flush contact moves no mate graph, so each door's own reading
        # is empty — the split belonged to the edit before it.
        findings = evaluate(doc).find_flush_candidates(lower, upper)
        self.assertEqual(len(findings), 1)
        doc.declare(findings[0])
        self.assertEqual(doc.last_maintenance, [])
        doc.apply(DocEdit.delete_node(mate_1))
        self.assertEqual([r.variant for r in doc.last_maintenance], ["split"])
        doc.declare_all(findings)
        self.assertEqual(doc.last_maintenance, [])

    def test_only_the_gauge_carries_an_authored_frame(self):
        doc, (post_a, shelf_i, post_b), _ = self.stand()
        # Placement lives on the CLUSTER. Two of the three instances
        # were never placed and never will be: their poses are solved.
        self.assertEqual(list(doc.placements()), [post_a])
        self.assertNotIn(post_b, doc.placements())
        self.assertNotIn(shelf_i, doc.placements())

    def test_the_solve_places_every_instance_where_the_scene_puts_it(self):
        doc, (post_a, shelf_i, post_b), (mate_1, mate_2) = self.stand()
        solved = solve_document(doc)
        for node in (post_a, shelf_i, post_b, mate_1, mate_2):
            self.assertIsNone(solved.fault(node), f"{node} records no fault")
        for mate in (mate_1, mate_2):
            self.assertEqual(solved.role(mate), MateRole.Determining)
        # The gauge's relative pose is the identity, bit-exactly, so
        # its world placement is its recorded frame verbatim.
        self.assertEqual(
            solved.placement(doc, post_a).origin,
            doc.placement(post_a).origin,
        )
        # The far post: composed outward from the gauge along the mate
        # tree, never stored. The scene's own expected translation.
        far = solved.placement(doc, post_b)
        want = (
            SEAT_B[0] - SEAT_A[0],
            (SHELF_DEPTH - POST_SECTION) / 2.0,
            0.0,
        )
        for got, expected in zip(far.origin, want, strict=True):
            self.assertAlmostEqual(got.meters, expected, delta=1e-12)
        # And the ROTATION, which a translation check cannot see: both
        # mates align +z with +z at zero clocking, so composing out
        # from the gauge must leave the post's own axes unturned. A
        # solve that rotated the post and still landed its seating
        # point would pass the check above and put the part in
        # sideways.
        self.assertEqual(
            far.columns, ((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0))
        )
        # The shelf sits on top of the posts.
        self.assertAlmostEqual(
            solved.placement(doc, shelf_i).origin[2].meters, POST_HEIGHT, delta=1e-12
        )

    def test_the_flush_seated_stand_certifies_at_the_at_rest_gate(self):
        doc, _, (mate_1, mate_2) = self.stand()
        assembly = assemble(doc, evaluate(doc, resolver=self.ws))
        # One record per solved mate, at face granularity, in the
        # alphabet the mates were authored in.
        self.assertEqual([d.mate for d in assembly.minted], [mate_1, mate_2])
        self.assertEqual(
            [d.class_ for d in assembly.minted],
            [ContactClass.Rest, ContactClass.Rest],
        )
        self.assertAlmostEqual(
            assembly.body.mass_properties().volume,
            2 * POST_VOLUME + SHELF_VOLUME,
            delta=1e-12,
        )

    def test_the_gather_and_the_gate_agree_on_the_body(self):
        doc, _, _ = self.stand()
        ev = evaluate(doc, resolver=self.ws)
        gathered = product(doc, ev)
        gated = assemble(doc, ev).body
        # `assemble` IS the gather plus the check, so the body is the
        # same body. Volume rather than identity: the two are separate
        # values, and equality of bodies is not a door Python has.
        self.assertEqual(
            gathered.mass_properties().volume, gated.mass_properties().volume
        )
        _, names = pncad.product_named(doc, ev)
        self.assertEqual(sorted(names), sorted(assemble(doc, ev).names))


class TestAssemblyRefusals(BenchWorkspace):
    """The refusal families an assembly author actually hits. Each is
    reached by authoring the mistake, never by hand-building a
    refusal."""

    def two_instances(self, doc=None):
        doc = doc or Doc(random_document_id() and "refuse")
        post_i = doc.insert(Node.instantiate_part(self.post_ref))
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref))
        return doc, post_i, shelf_i

    def test_a_single_planar_rest_leaves_the_pair_free_and_refuses(self):
        doc, (post_a, shelf_i, _), (mate_1, _) = self.stand_planar()
        fault = solve_document(doc).fault(mate_1)
        # One planar rest between two parts fixes the seating plane and
        # nothing else — the pair may still slide and spin in it. The
        # solve refuses and names the RESIDUAL in class vocabulary
        # rather than picking a pose.
        self.assertEqual(fault.variant, "mate_under")
        self.assertEqual(fault.residual.variant, "planar")
        self.assertEqual(fault.residual.normal, (0.0, 0.0, 1.0))
        self.assertEqual(fault.parent, post_a)
        self.assertEqual(fault.child, shelf_i)
        # The residual is point-FREE: no base point distinguishes one
        # plane from a parallel one.
        self.assertIsNone(fault.residual.point)
        self.assertIn(pncad.UNDER_RECOURSE, str(fault))

    def stand_planar(self):
        return TestBenchStand.stand(self, MatePrimitive.planar_rest(0 * m))

    def test_a_refused_solve_poisons_its_cluster_and_no_more(self):
        self.stand_planar()
        other = Doc("elsewhere")
        lone = other.insert(Node.instantiate_part(self.post_ref))
        # The refusal reaches the refusing mate and every instance in
        # its cluster that consequently has no pose. A second document
        # is untouched, which is the whole reason the solve is total.
        self.assertIsNone(solve_document(other).fault(lone))

    def test_a_mate_naming_one_instance_twice_refuses(self):
        doc = Doc("self-mate")
        post_i = doc.insert(Node.instantiate_part(self.post_ref))
        face = self.instance_face(doc, post_i, CapEnd.Top)
        mate = doc.insert(Node.mate(face, face, ContactClass.Rest, seat(POST_SEAT, POST_SEAT)))
        fault = solve_document(doc).fault(mate)
        # A pair is two instances; a self-mate constrains nothing and
        # is a recipe mistake, refused rather than folded into a
        # tautology.
        self.assertEqual(fault.variant, "mate_self")
        self.assertEqual(fault.instance, post_i)

    def test_a_class_the_gate_cannot_mint_refuses_at_the_gate(self):
        doc, _, (mate_1, _) = TestBenchStand.stand(self, class_=ContactClass.Tangent)
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(doc, evaluate(doc, resolver=self.ws))
        # A Tangent mate SOLVES and mints nothing at rest: the two
        # doors admit different sets, which is the whole reason the
        # admission table is one value both read.
        self.assertEqual(caught.exception.variant, "no_at_rest_record")
        self.assertEqual(caught.exception.mate, mate_1)
        self.assertEqual(caught.exception.class_, ContactClass.Tangent)

    def test_the_admission_table_says_so_before_the_edit_lands(self):
        rest = pncad.class_admission(ContactClass.Rest)
        tangent = pncad.class_admission(ContactClass.Tangent)
        self.assertEqual(rest.variant, "mints")
        self.assertTrue(rest.mints and rest.solves)
        self.assertIsNone(rest.why)
        # The gap the table exists to state: solved by one door,
        # mintable by neither.
        self.assertEqual(tangent.variant, "no_at_rest_record")
        self.assertTrue(tangent.solves)
        self.assertFalse(tangent.mints)
        self.assertIn("at rest", tangent.why)

    def test_a_mate_reference_that_is_not_a_face_refuses_at_the_gate(self):
        doc, post_i, shelf_i = self.two_instances()
        ev = evaluate(doc, resolver=self.ws)
        edge = sorted(ev.all_edges(post_i))[0]
        bottom = one(ev.select(shelf_i, cap_selector(CapEnd.Bottom, [SegTag.InPart])))
        mate = doc.insert(
            Node.mate(edge, bottom, ContactClass.Rest, seat(POST_SEAT, SEAT_A))
        )
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(doc, evaluate(doc, resolver=self.ws))
        err = caught.exception
        # A mate's declaration is a FACE-PAIR contact. An edge
        # reference is a different statement, refused rather than
        # widened — and the refusal says which side and which way.
        self.assertEqual(err.variant, "mate_reference_refused")
        self.assertEqual(err.mate, mate)
        self.assertEqual(err.side, pncad.MateSide.A)
        self.assertEqual(err.why.variant, "ref_not_a_face")
        self.assertEqual(err.why.kind, "edge")
        self.assertIsNone(err.why.width)

    def test_a_gather_refusal_arrives_under_the_gathers_own_tag(self):
        doc = Doc("no-resolver")
        doc.insert(Node.instantiate_part(self.post_ref))
        # Evaluated with no resolver, the instance produced no body, so
        # the GATHER refuses before the gate runs — and it refuses with
        # the gather's own tag, not a wrapper's, because which
        # invariant broke is what a caller branches on.
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(doc, evaluate(doc))
        self.assertEqual(caught.exception.variant, "root_failed")
        self.assertIsNotNone(caught.exception.node)
        self.assertIsNone(caught.exception.mate)
        with self.assertRaises(pncad.ProductError) as gather:
            product(doc, evaluate(doc))
        self.assertEqual(gather.exception.variant, "root_failed")

    def test_a_moved_pin_refuses_and_carries_its_recourse_twice(self):
        doc, post_i, _ = self.two_instances()
        # A part legitimately changes on disk. The assembly still pins
        # the old version and is never silently retargeted.
        taller = prism("pncad-demo-post", POST_SECTION, POST_SECTION, POST_HEIGHT * 2)
        self.ws.resave(taller)
        with self.assertRaises(pncad.EvaluationError) as caught:
            evaluate(doc, resolver=self.ws).value(post_i)
        self.assertEqual(caught.exception.kind, "part_pin_mismatch")
        # GAP (#947): the recourse paragraph arrives TWICE across the
        # seam — the store's own message ends on it and the resolver
        # appends it again when it classifies the failure. ASSERTED so
        # it goes red when fixed; ONE copy means it was, and this count
        # must be flipped in that same change. The store's own door
        # emits it once, which is the contrast that says where the
        # second copy comes from.
        self.assertEqual(
            str(caught.exception).count(pncad.PIN_MISMATCH_RECOURSE), 2
        )
        with self.assertRaises(pncad.WorkspaceError) as direct:
            self.ws.resolve(self.post_ref)
        self.assertEqual(str(direct.exception).count(pncad.PIN_MISMATCH_RECOURSE), 1)


class TestPinUpdateDoor(BenchWorkspace):
    """Moving a pin at its sites — and what each door reads, and
    when."""

    def layout(self, shelf_ref=None):
        doc = Doc("update-layout")
        post_i = doc.insert(Node.instantiate_part(self.post_ref))
        shelf_i = doc.insert(Node.instantiate_part(shelf_ref or self.shelf_ref))
        return doc, post_i, shelf_i

    def thicker_shelf(self):
        """The shelf, changed on disk. Same id, new content, new pin."""
        thicker = prism(
            "pncad-demo-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS * 2
        )
        self.ws.resave(thicker)
        return thicker

    def test_update_references_reads_no_store_and_takes_the_pin_as_given(self):
        doc, _, shelf_i = self.layout()
        thicker = self.thicker_shelf()
        edits = pncad.update_references(doc, self.shelf.id, content_pin(thicker))
        # Pure: the elaboration answered without touching the document.
        self.assertEqual(len(edits), 1)
        self.assertEqual(doc.reference(shelf_i), self.shelf_ref)
        for edit in edits:
            doc.apply(edit)
        self.assertEqual(doc.reference(shelf_i).pin, content_pin(thicker))

    def test_a_pin_nothing_holds_is_accepted_here_and_refused_at_evaluation(self):
        doc, _, shelf_i = self.layout()
        # The new pin is RECIPE DATA, not a resolution: this layer has
        # no store, so a pin naming content nothing holds is a legal
        # edit and refuses at the seam instead. Checking here would
        # make the edit's meaning depend on which store was mounted
        # when it was recorded.
        invented = content_pin(prism("elsewhere", 1.0, 1.0, 1.0))
        for edit in pncad.update_references(doc, self.shelf.id, invented):
            doc.apply(edit)
        self.assertEqual(doc.reference(shelf_i).pin, invented)
        with self.assertRaises(pncad.EvaluationError) as caught:
            evaluate(doc, resolver=self.ws).value(shelf_i)
        self.assertEqual(caught.exception.kind, "part_pin_mismatch")

    def test_the_two_empty_update_arms_are_separate_because_recourses_differ(self):
        doc, _, _ = self.layout()
        with self.assertRaises(pncad.UpdateError) as pinned:
            pncad.update_references(doc, self.shelf.id, content_pin(self.shelf))
        self.assertEqual(pinned.exception.variant, "already_pinned")
        self.assertEqual(pinned.exception.id, self.shelf.id)
        self.assertEqual(pinned.exception.pin, content_pin(self.shelf))
        with self.assertRaises(pncad.UpdateError) as absent:
            pncad.update_references(
                doc, random_document_id(), content_pin(self.shelf)
            )
        self.assertEqual(absent.exception.variant, "no_such_reference")
        self.assertIsNone(absent.exception.pin)

    def test_update_to_store_snapshots_the_store_at_the_call(self):
        doc, _, shelf_i = self.layout()
        first = self.thicker_shelf()
        edits = self.ws.update_to_store(doc, self.shelf.id)
        # The store moves again BEFORE the caller applies. The edits
        # carry the pin as a literal and nothing re-reads, so what
        # lands is the version the store held at the call — a
        # snapshot, not a subscription. This is the contract stated at
        # the door, executed.
        second = prism(
            "pncad-demo-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS * 3
        )
        self.ws.resave(second)
        for edit in edits:
            doc.apply(edit)
        self.assertEqual(doc.reference(shelf_i).pin, content_pin(first))
        self.assertNotEqual(doc.reference(shelf_i).pin, content_pin(second))

    def test_update_to_store_forwards_the_elaborations_own_refusal(self):
        doc, _, _ = self.layout()
        with self.assertRaises(pncad.WorkspaceError) as caught:
            self.ws.update_to_store(doc, self.shelf.id)
        # The store did its part — it found the current pin. The
        # refusal is about the ASSEMBLY, and it says so.
        self.assertEqual(caught.exception.variant, "update")

    def test_mixed_pins_reports_a_staged_migration_and_never_gates(self):
        doc, _, first_site = self.layout()
        thicker = self.thicker_shelf()
        second_site = doc.insert(
            Node.instantiate_part(DocRef(self.shelf.id, content_pin(thicker)))
        )
        report = pncad.mixed_pins(doc)
        self.assertEqual(len(report), 1)
        self.assertEqual(report[0].id, self.shelf.id)
        # Both pins listed, each with the sites holding it. Two pins of
        # one id is legal, sometimes-INTENDED state, so this reports
        # and nothing refuses it: the document saves and evaluates.
        self.assertEqual(len(report[0].pins), 2)
        self.assertEqual(
            {n for p in report[0].pins for n in p.nodes},
            {first_site, second_site},
        )
        pncad.load(doc.save())
        # And "update everywhere" stays usable FROM the staged state:
        # the site that already moved contributes no edit.
        edits = pncad.update_references(doc, self.shelf.id, content_pin(thicker))
        self.assertEqual(len(edits), 1)
        for edit in edits:
            doc.apply(edit)
        self.assertEqual(pncad.mixed_pins(doc), [])

    def test_a_clean_document_reports_an_empty_lint(self):
        doc, _, _ = self.layout()
        # The difference between "checked and fine" and "not checked".
        self.assertEqual(pncad.mixed_pins(doc), [])


class TestRefactorings(BenchWorkspace):
    """`split` and `inline`: the recorded refactorings, and the
    acceptance property each step preserves."""

    def layout(self):
        doc = Doc("refactor-me")
        post_i = doc.insert(Node.instantiate_part(self.post_ref))
        doc.apply(
            DocEdit.set_placement(post_i, Frame.translation((0 * m, 0 * m, 0 * m)))
        )
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref))
        doc.apply(
            DocEdit.set_placement(shelf_i, Frame.translation((0 * m, 0.9 * m, 0 * m)))
        )
        return doc, post_i, shelf_i

    def volume(self, doc):
        return product(doc, evaluate(doc, resolver=self.ws)).mass_properties().volume

    def test_split_then_inline_preserves_the_products_material_exactly(self):
        doc, _, shelf_i = self.layout()
        before = self.volume(doc)

        outcome = pncad.split(doc, [shelf_i], random_document_id())
        # PURE: the two documents come back as VALUES with the edits
        # that produce them, and the original is untouched.
        self.assertEqual(self.volume(doc), before)
        self.assertGreater(len(outcome.part_edits), 0)
        self.assertGreater(len(outcome.remainder_edits), 0)
        self.assertEqual(len(outcome.node_map), 1)
        # An instance of the new part is what replaced the cut, and it
        # carries the part's reference.
        self.assertIn(outcome.instance, outcome.remainder.roots)
        self.assertIsNotNone(outcome.remainder.reference(outcome.instance))
        # An authored instance crosses nothing; this one crossed
        # nothing either, because no mate spanned the cut.
        self.assertEqual(len(outcome.remainder.interface(outcome.instance)), 0)
        self.assertEqual(outcome.remainder.interface(outcome.instance).crossings, [])

        # Persisting is the STORE's write side, not the refactoring's.
        self.ws.create(outcome.part)
        self.assertEqual(self.volume(outcome.remainder), before)

        spliced = pncad.inline(outcome.remainder, outcome.instance, self.ws)
        self.assertEqual(self.volume(spliced.doc), before)
        self.assertGreater(len(spliced.edits), 0)

    def test_split_refuses_typed_and_names_what_it_refused(self):
        doc, post_i, _ = self.layout()
        with self.assertRaises(pncad.SplitError) as empty:
            pncad.split(doc, [], random_document_id())
        self.assertEqual(empty.exception.variant, "empty_cut")
        with self.assertRaises(pncad.SplitError) as collides:
            pncad.split(doc, [post_i], doc.id)
        # A fresh identity is what lets both documents live in one
        # store, so a colliding one refuses NAMING the collision.
        self.assertEqual(collides.exception.variant, "part_id_collides")
        self.assertEqual(collides.exception.id, doc.id)

    def test_inline_crosses_the_seam_at_the_call_and_refuses_a_stale_pin(self):
        doc, _, shelf_i = self.layout()
        thicker = prism(
            "pncad-demo-shelf", SHELF_LENGTH, SHELF_DEPTH, SHELF_THICKNESS * 2
        )
        self.ws.resave(thicker)
        with self.assertRaises(pncad.InlineError) as caught:
            pncad.inline(doc, shelf_i, self.ws)
        # Inline resolves the referenced document AT THIS CALL, under
        # the full pin gate — so a stale pin refuses under the SEAM's
        # own tag, the one `EvaluationError.kind` already speaks,
        # rather than splicing the version on disk.
        self.assertEqual(caught.exception.variant, "part_pin_mismatch")

    def test_inline_of_a_node_that_is_not_an_instance_refuses(self):
        doc = Doc("plain")
        profile = doc.insert(Node.polygon([(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m)], plane=doc.sketch_frame()))
        body = doc.insert(Node.extrude(profile, 1 * m))
        with self.assertRaises(pncad.InlineError) as caught:
            pncad.inline(doc, body, self.ws)
        self.assertEqual(caught.exception.variant, "not_an_instance")
        self.assertEqual(caught.exception.node, body)


class TestProductRoots(BenchWorkspace):
    """`set_roots` and `product`: a document states what it IS."""

    def test_the_roots_say_what_the_product_gathers_and_in_what_order(self):
        doc, post_i, shelf_i = Doc("roots"), None, None
        post_i = doc.insert(Node.instantiate_part(self.post_ref))
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref))
        self.assertEqual(doc.roots, [post_i, shelf_i])
        # The designate door is TOTAL: one edit states the whole list,
        # so the product's solid order is never inferred from an edit
        # sequence.
        doc.apply(DocEdit.set_roots([shelf_i, post_i]))
        self.assertEqual(doc.roots, [shelf_i, post_i])
        volume = self.volume(doc)
        self.assertAlmostEqual(volume, POST_VOLUME + SHELF_VOLUME, delta=1e-12)

    def volume(self, doc):
        return product(doc, evaluate(doc, resolver=self.ws)).mass_properties().volume

    def test_the_root_invariants_refuse_under_their_own_tags(self):
        doc = Doc("roots-refuse")
        post_i = doc.insert(Node.instantiate_part(self.post_ref))
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref))
        with self.assertRaises(pncad.EditError) as dup:
            doc.apply(DocEdit.set_roots([post_i, post_i]))
        self.assertEqual(dup.exception.variant, "root_duplicate")
        with self.assertRaises(pncad.EditError) as uncovered:
            doc.apply(DocEdit.set_roots([post_i]))
        # A live node reaching no root is a silently dead subgraph.
        self.assertEqual(uncovered.exception.variant, "root_uncovered")
        # A refused edit leaves the document untouched.
        self.assertEqual(doc.roots, [post_i, shelf_i])

    def test_a_document_with_no_body_root_has_no_product(self):
        doc = Doc("datum-only")
        doc.insert(Node.datum_plane((0 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0)))
        with self.assertRaises(pncad.ProductError) as caught:
            product(doc, evaluate(doc))
        self.assertEqual(caught.exception.variant, "no_body_roots")


if __name__ == "__main__":
    unittest.main()
