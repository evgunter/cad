"""AUTHORING an assembly from Python: the tour's bench, built from nothing.

`test_assembly_eval.py` is this file's other half. It writes the same
scene into a store and evaluates what it reads back; here nothing goes
near disk that this file did not put there. Two part documents are
authored, written into a workspace, instantiated into two assembly
documents, mated, solved, gathered and gated — the whole of the audit's
row 44 through the public doors.

THE SCENE IS `bench_scene.py`, AND THE ORACLE COMES FROM THE TOUR
-----------------------------------------------------------------
`demos/tour/src/assembly.rs` is the scene these documents ARE, and its
own assertions are the oracle: the layout's three disjoint solids of
`PATTERN_COUNT x post + shelf`, the stand's near post pinned to the
world and each other instance placed by the one mate its bundle holds
and that mate's values, the far post's solved translation, the identity
rotation two face-on-face rests compose to, two minted declarations, and
a gate that certifies. Every number below is that scene's, re-derived from the
same base dimensions rather than copied as a total. The tour's bench
also stands on a turntable and carries a crate, whose declaration is a
third; this scene authors neither (`bench_scene.py`'s header).

The constants, the two part shapes and the two assembly recipes live
in `bench_scene.py`, which both this file and `test_assembly_eval.py`
build from — one definition, so the two cannot disagree about what the
bench is. `test_assembly_eval.TestTheSceneIsTheToursOwn` is what holds
that module to the tour, and its header names what that reaches.

TWO THINGS THIS FILE CANNOT SAY, AND THEY ARE NOT DEFECTS OF IT
---------------------------------------------------------------
1. `shells().count()`. The scene counts solids in the gathered body;
   `Body` answers mass properties, validators and a mesh, and no
   count. What stands in for it here is the VOLUME, which for disjoint
   solids is the sum the shell count is being used to check, plus the
   per-instance name set, which is structural. Binding a solid count
   is a `Body` question and not this unit's.
2. Nothing else. "Instance 1's post cap sits at (x, y, z)" — the
   scene's name-lookup stop — was the second entry here until
   LIB-B-READBACK bound `face_frame`; it is now asserted below,
   against the placement arithmetic rather than against a
   transcribed coordinate.

WHAT THE GATE SAYS OF A MATE READ BELOW A PLACER, AND WHY
---------------------------------------------------------
A mate's pose is read off the body it names, and the gate mints its
declaration on the world copy of that body. A pose may read a
transform that a `placed_union` consumes: no placement reads that
transform, so the mate is no statement about the product, and the seat
it would have declared is a touch between two copies, which the
`unproven-coincidence` check reports against them (asserted below).
`RefusedRef`'s one arm, `ref_ambiguous`, is a side whose body two
placements copy; it is bound and tagged, and the kernel's own row
(`intent_s2_f_mates`) reaches it.

A plane read off a name that is NOT A FACE is built and admitted —
the door is structural — and refused where its placement evaluates
(`mate_side_unevaluated`, asserted below), not at the gate.

THE SPELLING OF THE PLACED FAMILY, MEASURED RATHER THAN CLAIMED
---------------------------------------------------------------
The layout's placed posts are `Node.pattern` — the tour's own node —
and `bench_scene.layout`'s `posts=` says them with `Node.placed_union`
instead. `TestBenchLayout` runs its whole battery against BOTH, which
is what turns "the fused spelling says the same placed family" from a
sentence into a reading: the material, the per-placement cap frames,
the instance-qualified names, their denotations and an A5 gate that
passes outright are the same for a PLURAL `instances` value as for a
single body, and `test_the_two_spellings_differ_in_the_values_
plurality_alone` is what keeps that from being a coincidence of two
identical nodes.

The mate door is measured on the tour's own node: a mate head stands
on a PATTERN's copy, and the solve places the shelf against it.
"""

import math
import tempfile
import unittest
from pathlib import Path

import bench_scene
import pncad
from spoken import tag
from bench_scene import (
    PART_FACE,
    PATTERN_COUNT,
    PATTERN_SPACING,
    POST_HEIGHT,
    POST_SECTION,
    POST_VOLUME,
    SEAT_A,
    SEAT_B,
    SHELF_LENGTH,
    SHELF_ON_POST,
    SHELF_THICKNESS,
    SHELF_VOLUME,
    cap_selector,
    one,
    prism,
    rest,
)
from pncad import (
    CapEnd,
    CheckId,
    ContactClass,
    Coord,
    Doc,
    DocEdit,
    DocRef,
    Formula,
    Frame,
    FreeVar,
    Mate,
    MateValue,
    Node,
    PatternKind,
    PoseDef,
    RoleRead,
    SegTag,
    VarName,
    Workspace,
    WorldPin,
    WorldRead,
    assemble,
    content_pin,
    evaluate,
    m,
    product,
    rad,
    random_document_id,
    run_checks,
    solve_document,
)


class BenchWorkspace(unittest.TestCase):
    """A store holding the two part documents, authored here."""

    def setUp(self):
        self.dir = Path(tempfile.mkdtemp())
        self.addCleanup(lambda: __import__("shutil").rmtree(self.dir, True))
        self.ws = Workspace(str(self.dir))
        (self.post, self.post_ref), (self.shelf, self.shelf_ref) = bench_scene.parts(
            self.ws
        )

    def instance_face(self, doc, node, side):
        """A face of an instance's product, in the ASSEMBLY's names —
        the scene's own door, at this case's store."""
        return bench_scene.instance_face(self.ws, doc, node, side)

    def pinned(self, doc, ref, pose=None):
        """An instance of `ref` pinned in the world at `pose` (the
        identity when `None`) by the "place part" gesture."""
        node = doc.insert(Node.instantiate_part(ref, []))
        doc.pin(node, pose, resolver=self.ws)
        return node


#: The two nodes `bench_scene.layout`'s `posts=` admits, by the name
#: this file reports a failure under. Every expectation below is
#: asserted against BOTH: what the audit row used to carry was a CLAIM
#: that the fused spelling says the same placed family as the tour's
#: plural one, and a claim about two spellings is measured by running
#: both, not by asserting one and writing a sentence about the other.
SPELLINGS = {"pattern": Node.pattern, "placed_union": Node.placed_union}


class TestBenchLayout(BenchWorkspace):
    """Row 44's flat-pack: The posts on their side and the shelf
    beside them, nothing touching — A5's disjoint half.

    The layout has no mates, so the bundles, the solve and the minted
    declarations are `TestBenchStand`'s expectations and not this
    document's; what this one is held to is the material, the
    placements, the names and a gate that passes OUTRIGHT with nothing
    minted."""

    def layout(self, posts=Node.pattern):
        doc, post_i, family, shelf_i = bench_scene.layout(
            self.ws, self.post_ref, self.shelf_ref, posts
        )
        return doc, family, post_i, shelf_i

    def test_the_layout_gathers_the_scenes_material_exactly(self):
        for name, posts in SPELLINGS.items():
            with self.subTest(posts=name):
                doc, _, _, _ = self.layout(posts)
                # The placements state the product: the post instance,
                # a loose pattern master and so no part of it; the shelf
                # instance, pinned to the world; then the posts' copies
                # (each pattern instance projected and placed, or the
                # fused family placed whole).
                placed = PATTERN_COUNT + 2 if name == "pattern" else 3
                self.assertEqual(len(doc.placements()), placed)
                ev = evaluate(doc, resolver=self.ws)
                volume = product(doc, ev).mass_properties().volume
                self.assertAlmostEqual(
                    volume, PATTERN_COUNT * POST_VOLUME + SHELF_VOLUME, delta=1e-12
                )

    def test_every_patterned_post_answers_to_an_instance_qualified_name(self):
        for name, posts in SPELLINGS.items():
            with self.subTest(posts=name):
                doc, family, _, _ = self.layout(posts)
                ev = evaluate(doc, resolver=self.ws)
                caps = ev.select(
                    family, cap_selector(CapEnd.End, [SegTag.Instance, *PART_FACE])
                )
                # One per placement, and the name NESTS rather than
                # concatenating: pattern index, then instance, then
                # the part's own cap. Distinct names are the
                # structural claim the volume cannot make.
                self.assertEqual(len(caps), PATTERN_COUNT)
                self.assertEqual(len(set(caps)), PATTERN_COUNT)

    def test_the_disjoint_layout_passes_the_at_rest_gate_outright(self):
        for name, posts in SPELLINGS.items():
            with self.subTest(posts=name):
                doc, _, _, _ = self.layout(posts)
                assembly = assemble(doc, evaluate(doc, resolver=self.ws))
                # No mate declares anything and nothing touches, so the
                # kernel's at-rest door passes with nothing minted.
                # That is exactly what "disjoint assemblies validate
                # today" means — and it is what a PLURAL value reaching
                # the gate had to be measured against rather than
                # assumed: the gate takes the family either way.
                self.assertEqual(assembly.minted, [])
                self.assertGreater(len(assembly.names), 0)
                self.assertAlmostEqual(
                    assembly.body.mass_properties().volume,
                    PATTERN_COUNT * POST_VOLUME + SHELF_VOLUME,
                    delta=1e-12,
                )

    def test_the_two_spellings_differ_in_the_values_plurality_alone(self):
        """What the two nodes actually say, which is the difference the
        rows either side of this one are measuring ACROSS.

        `Node.pattern` answers the family unfused — N bodies, and no
        single-body door at all — where `placed_union` answers one
        body of the same material. Every expectation in this class
        holds for both; this row is why that is a finding and not a
        coincidence, and it is what a reader needs to know the two
        spellings are not the same node under two names.
        """
        volumes = {}
        for name, posts in SPELLINGS.items():
            with self.subTest(posts=name):
                doc, family, _, _ = self.layout(posts)
                value = evaluate(doc, resolver=self.ws).value(family)
                bodies = value.bodies()
                volumes[name] = sum(b.mass_properties().volume for b in bodies)
                if name == "pattern":
                    self.assertEqual(value.kind, "instances")
                    self.assertEqual(len(bodies), PATTERN_COUNT)
                    # The plural value has no single-body door, and
                    # says so typed rather than handing back the first.
                    with self.assertRaises(pncad.EvaluationError):
                        value.body()
                else:
                    self.assertEqual(value.kind, "body")
                    self.assertEqual(len(bodies), 1)
                    value.body()
        self.assertAlmostEqual(
            volumes["pattern"], volumes["placed_union"], delta=1e-12
        )

    def test_a_mate_head_stands_on_a_patterned_copy(self):
        """The mate side of the scene's placed family, measured — NOT
        the scene, which declares nothing.

        A mate's `to` may read a face of a pattern's copy: the copy is
        projected out of the family (`Node.part` — a pose reads one
        body), the plane is read off the face the pattern placed, and
        the solve places the shelf against it. The mate and its values
        are constraints of the shelf's bundle, which was empty, so they
        are what places the shelf.
        """
        doc = Doc("pncad-mate-onto-pattern")
        post_i = self.pinned(doc, self.post_ref)
        family = doc.insert(
            Node.pattern(
                post_i,
                Formula.count(PATTERN_COUNT),
                PatternKind.linear((
                    Formula.literal(0.0),
                    Formula.literal(1.0),
                    Formula.literal(0.0),
                ), Formula.length_in(SHELF_LENGTH, m)),
            )
        )
        first = doc.insert(Node.part(family, pncad.PartSelect.instance(Formula.count(0))))
        # The shelf's bundle is empty: the mate is what places it.
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, []))
        ev = evaluate(doc, resolver=self.ws)
        cap = one(ev.select(first, cap_selector(CapEnd.End, [SegTag.Instance, *PART_FACE])))
        bottom = one(ev.select(shelf_i, cap_selector(CapEnd.Start, PART_FACE)))
        joint, values = rest(
            doc, shelf_i, (shelf_i, bottom), (first, cap), SHELF_ON_POST, resolver=self.ws
        )
        solved = solve_document(doc, resolver=self.ws)
        self.assertIsNone(solved.fault(shelf_i))
        self.assertEqual([ref for ref, _ in doc.bundle(shelf_i)], [joint, *values])
        # Placed on a copy of the post, which is pinned to the world:
        # the shelf is in the world too.
        self.assertEqual(evaluate(doc, resolver=self.ws).loose, [])

    def test_a_patterned_caps_frame_is_where_the_placement_puts_it(self):
        """The scene's name-lookup stop: where does the first
        NON-IDENTITY instance's post cap SIT?

        The oracle is the model asked twice, never a transcribed
        coordinate. The PART document answers where its own cap sits,
        on its own evaluation; the pattern master is a loose copy, so
        its space is its part's own coordinates and that point is where
        the master's cap sits; and the pattern rule steps it along +y
        once per instance. What `face_frame` answers on the LAYOUT's
        family must be that ladder, one rung per instance; and the
        world pin each placed copy holds moves it, bit for bit, to the
        pose the scene lays it at.

        The index is read off the GEOMETRY, not off the name: an
        instance-qualified name is opaque text and the role segment
        the Rust tour filters on (`Instance { i: 2 }`) is deliberately
        unreadable from Python. Which is the honest shape here — the
        question is where a cap sits, and the answer is what
        identifies it.
        """
        # Rung 1: the part's own cap, in the part's own coordinates —
        # on the copy its one placement puts in its world.
        part_ev = evaluate(self.post)
        (placed,) = self.post.placements()
        part_cap = one(part_ev.select(placed, cap_selector(CapEnd.End, [SegTag.Placed])))
        local = part_ev.face_frame(placed, part_cap).origin

        for name, posts in SPELLINGS.items():
            with self.subTest(posts=name):
                doc, family, post_i, _ = self.layout(posts)
                ev = evaluate(doc, resolver=self.ws)

                # Rung 2: the master is loose, so its space is its part's
                # own coordinates: the cap sits where the part says.
                self.assertEqual(ev.space(family), post_i)
                placed = tuple(c.meters for c in local)
                # Each placed copy's world pose is the pose the gesture
                # placed it at: the scene's laid-down pose.
                world = solve_document(doc, resolver=self.ws)
                lying = Frame.rotate_then_translate(
                    (0.0, 1.0, 0.0),
                    -math.pi / 2 * rad,
                    ((bench_scene.FLAT_PACK_GAP + POST_HEIGHT) * m, 0 * m, 0 * m),
                )
                for copy in doc.placements()[2:]:
                    at = world.placement(doc, copy)
                    for got, want in zip(at.origin, lying.origin, strict=True):
                        self.assertAlmostEqual(got.meters, want.meters, delta=1e-12)
                    for got, want in zip(at.columns, lying.columns, strict=True):
                        for g, w in zip(got, want, strict=True):
                            self.assertAlmostEqual(g, w, delta=1e-12)

                # Rung 3: the pattern steps +y once per instance.
                expected = [
                    (placed[0], placed[1] + i * PATTERN_SPACING, placed[2])
                    for i in range(PATTERN_COUNT)
                ]

                caps = ev.select(
                    family, cap_selector(CapEnd.End, [SegTag.Instance, *PART_FACE])
                )
                read = sorted(
                    tuple(c.meters for c in ev.face_frame(family, cap).origin)
                    for cap in caps
                )
                self.assertEqual(len(read), PATTERN_COUNT)
                for got, want in zip(read, sorted(expected), strict=True):
                    for got_axis, want_axis in zip(got, want, strict=True):
                        self.assertAlmostEqual(got_axis, want_axis, delta=1e-12)

                # And the scene's own sentence, which is about instance
                # 1 — the first instance the pattern must actually have
                # moved, because index 0 may be handed back verbatim.
                # One step along +y puts its post between
                # y = PATTERN_SPACING and y = PATTERN_SPACING + section,
                # and one cap frame lies there.
                low = PATTERN_SPACING
                band = [o for o in read if low <= o[1] <= low + POST_SECTION]
                self.assertEqual(len(band), 1, f"instance 1's cap alone: {read}")

    def test_a_cap_name_denotes_one_face_and_says_so_before_it_is_read(self):
        """`denotation` is the door to ask BEFORE a frame: the frame
        doors refuse a tie rather than picking a candidate, and this
        says whether one is coming. The scene's names are all
        unique — which is a fact worth ASSERTING, because it is why
        every `face_frame` above answered at all."""
        for name, posts in SPELLINGS.items():
            with self.subTest(posts=name):
                doc, family, _, _ = self.layout(posts)
                ev = evaluate(doc, resolver=self.ws)
                caps = ev.select(
                    family, cap_selector(CapEnd.End, [SegTag.Instance, *PART_FACE])
                )
                for cap in caps:
                    denotation = ev.denotation(family, cap)
                    self.assertFalse(denotation.tied)
                    self.assertEqual(denotation.candidates, 1)

    def test_an_instance_is_inserted_loose_and_pinned_where_the_gesture_puts_it(self):
        """An inserted instance is a loose copy until something pins
        it; `Doc.pin` places it at the identity, where its part's world
        stands, by a mate on its first planar face and the three values
        that put it there; a pin at a pose moves it; and an instance
        whose constraints are removed is loose again, with no world
        pose."""
        doc = Doc("inserted")
        node = doc.insert(Node.instantiate_part(self.post_ref))
        self.assertEqual(doc.bundle(node), [])
        self.assertEqual(evaluate(doc, resolver=self.ws).loose, [node])
        placed = doc.pin(node, resolver=self.ws)
        self.assertEqual(placed.by, "planar_face")
        for got in solve_document(doc, resolver=self.ws).placement(doc, node).origin:
            self.assertAlmostEqual(got.meters, 0.0, delta=1e-12)

        def unpin(placed):
            for value in placed.values:
                doc.apply(DocEdit.remove_constraint(value))
            doc.apply(DocEdit.remove_constraint(placed.mate))

        unpin(placed)
        moved = doc.pin(node, Frame.translation((1 * m, 0 * m, 0 * m)), resolver=self.ws)
        self.assertEqual(
            [ref for ref, _ in doc.bundle(node)], [moved.mate, *moved.values]
        )
        at = solve_document(doc, resolver=self.ws).placement(doc, node).origin
        for got, want in zip(at, (1.0, 0.0, 0.0), strict=True):
            self.assertAlmostEqual(got.meters, want, delta=1e-12)
        unpin(moved)
        self.assertEqual(doc.bundle(node), [])
        solved = solve_document(doc, resolver=self.ws)
        self.assertIsNone(solved.fault(node), "a loose copy is not a refusal")
        with self.assertRaises(pncad.EvaluationError) as caught:
            solved.placement(doc, node)
        self.assertEqual(caught.exception.kind, "space_mismatch")


class TestBenchStand(BenchWorkspace):
    """Row 44's assembled bench: Two posts and a shelf, the shelf
    SEATED on the posts by mates — only the near post is pinned to the
    world, and the other two poses are solved."""

    def stand(self, class_=ContactClass.Rest):
        return bench_scene.stand(self.ws, self.post_ref, self.shelf_ref, class_)

    def assertSeated(self, doc, node, mate):
        """`node`'s bundle is `mate`, which reads another copy rather
        than the world, then the three values that name it."""
        bundle = doc.bundle(node)
        self.assertEqual(bundle[0][0], mate)
        self.assertNotIsInstance(bundle[0][1].to, WorldRead)
        self.assertEqual(
            [(type(held), held.mate) for _, held in bundle[1:]],
            [(MateValue, mate.constraint)] * 3,
        )

    def test_each_mate_is_a_constraint_of_the_copy_it_places(self):
        doc, (post_a, shelf_i, post_b), (mate_1, mate_2) = self.stand()
        # The near post is pinned to the world; each other instance's
        # bundle is the one mate that places it, with its values.
        self.assertIsInstance(doc.bundle(post_a)[0][1].to, WorldRead)
        self.assertSeated(doc, shelf_i, mate_1)
        self.assertSeated(doc, post_b, mate_2)
        self.assertEqual((mate_1.placement, mate_2.placement), (shelf_i, post_b))
        # Pinned, transitively, to the world: no copy is loose.
        ev = evaluate(doc, resolver=self.ws)
        self.assertEqual(ev.loose, [])
        for node in (post_a, shelf_i, post_b):
            self.assertEqual(ev.space(node), "world")
        # A mate's sides are reads, so the mates couple the three
        # instances into one relative-freedom component.
        component = next(
            c for c in pncad.relative_freedom_components(doc) if shelf_i in c
        )
        for node in (post_a, shelf_i, post_b):
            self.assertIn(node, component)

    def test_last_maintenance_describes_the_last_accepted_edit_at_every_door(self):
        """`last_maintenance` says "the LAST accepted edit", and `Doc`
        has two doors that accept one: `apply` and `insert`. A door
        that swaps the document without the
        record leaves the reading describing an EARLIER edit, which is
        worse than no reading — it is a plausible one about the wrong
        subject.

        The recorded refactorings are the third and fourth doors, and
        they answer for a whole edit LIST rather than one edit:
        `test_the_refactoring_doors_hand_back_the_maintenance_their_edits_performed`
        is their half of the same funnel."""
        doc, (_post_a, shelf_i, post_b), (_mate_1, mate_2) = self.stand()
        # `constrain`: the stand's second mate places post_b on the
        # shelf, and clears nothing — that is what the door just
        # accepted.
        self.assertEqual(doc.last_maintenance, [])

        def slab(x, y, z):
            """A box, inserted through the `insert` door."""
            profile = doc.insert(
                Node.polygon(
                    [
                        (Formula.literal(x[0]), Formula.literal(y[0])),
                        (Formula.literal(x[1]), Formula.literal(y[0])),
                        (Formula.literal(x[1]), Formula.literal(y[1])),
                        (Formula.literal(x[0]), Formula.literal(y[1])),
                    ],
                    plane=doc.sketch_frame(elevation=Formula.literal(z[0])),
                )
            )
            return doc.insert(Node.extrude(profile, Formula.literal(z[1] - z[0])))

        lower = slab((0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        upper = slab((0.25 * m, 0.75 * m), (0.25 * m, 0.75 * m), (1 * m, 1.5 * m))
        doc.insert(Node.union([lower, upper]))
        self.assertEqual(doc.last_maintenance, [])
        # `apply`: removing a mate records no frame. Its values go
        # first, and the mate's poses and selections go with it, each a
        # silent retirement of an anonymous variable.
        for ref, _held in doc.bundle(post_b)[1:]:
            doc.apply(DocEdit.remove_constraint(ref), resolver=self.ws)
        doc.apply(DocEdit.remove_constraint(mate_2), resolver=self.ws)
        retired = [m.variant for m in doc.last_maintenance]
        self.assertTrue(retired)
        self.assertEqual(set(retired), {"anonymous_var_removed"})
        # `constrain` again: the record is EMPTY, not the retirements
        # still standing from before.
        b_top = self.instance_face(doc, post_b, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        rest(
            doc,
            post_b,
            (post_b, b_top),
            (shelf_i, s_bottom),
            bench_scene.POST_UNDER_SHELF,
            resolver=self.ws,
        )
        self.assertEqual(doc.last_maintenance, [])
        # A REFUSED edit leaves the reading as it was: a mate whose `on`
        # reads another copy pins nothing of this one.
        doc.apply(DocEdit.delete_node(upper))
        stranded = [m.variant for m in doc.last_maintenance]
        elsewhere = Mate(PoseDef.plane(shelf_i, s_bottom), WorldRead.through("xy"), ContactClass.Rest)
        with self.assertRaises(pncad.EditError) as caught:
            doc.constrain(post_b, elsewhere, resolver=self.ws)
        self.assertEqual(caught.exception.inner_variant, "mate_not_on_the_copy")
        self.assertEqual([m.variant for m in doc.last_maintenance], stranded)

    def test_the_refactoring_doors_hand_back_the_maintenance_their_edits_performed(
        self,
    ):
        """`split` and `inline` mint document VALUES rather than
        swapping one in place, and each value is produced by applying a
        list of edits. The document and what its edits reported cross
        together, so `last_maintenance` on a refactoring's document
        reads that refactoring's own record.

        The cut moves as selected: the near post keeps its world pin in
        the part, and the instance left behind is pinned where it
        stands by the world pin. No edit records a frame, so the part
        reports nothing, and the remainder and an inline back report
        only the anonymous variables their edits retire — the cut
        mates' reads, and the part's world reads the inline re-writes
        at the instance's pose."""
        doc, (post_a, shelf_i, post_b), _ = self.stand()

        outcome = pncad.split(
            doc,
            [post_a, shelf_i, post_b],
            random_document_id(),
            resolver=self.ws,
        )
        self.assertEqual(outcome.part.last_maintenance, [])
        # The cut mates' selections leave the remainder with them.
        self.assertTrue(
            all(
                m.variant == "anonymous_var_removed"
                for m in outcome.remainder.last_maintenance
            ),
            outcome.remainder.last_maintenance,
        )
        # The cut moves as selected: the instance left behind is pinned
        # where it stands by the world pin, which reads nothing, and the
        # near post keeps its pin.
        [(_, left_behind)] = outcome.remainder.bundle(outcome.instance)
        self.assertIsInstance(left_behind, WorldPin)

        def pinned(part):
            return [
                n
                for n in part.placements()
                if isinstance(part.bundle(n)[0][1].to, WorldRead)
            ]

        self.assertEqual(len(pinned(outcome.part)), 1)
        # The door is PURE, so the input's own reading is untouched —
        # still the empty record the stand's second mate left.
        self.assertEqual(doc.last_maintenance, [])

        self.ws.create(outcome.part)
        spliced = pncad.inline(outcome.remainder, outcome.instance, self.ws)
        self.assertEqual(
            {m.variant for m in spliced.doc.last_maintenance} - {"anonymous_var_removed"},
            set(),
            spliced.doc.last_maintenance,
        )
        # Inline of split is the document split was given, up to node
        # ids: three instances again, one pinned where the near post was.
        self.assertEqual(len(spliced.doc.placements()), 3)
        self.assertEqual(len(pinned(spliced.doc)), 1)

    def test_only_the_near_post_is_pinned_to_the_world(self):
        doc, (post_a, shelf_i, post_b), (mate_1, mate_2) = self.stand()
        # The near post's mate reads the world; each other bundle is one
        # face mate on another copy and its values, and their poses are
        # solved.
        pin = doc.bundle(post_a)[0][1]
        self.assertIsInstance(pin.to, WorldRead)
        self.assertEqual(pin.to.kind, "plane")
        self.assertSeated(doc, shelf_i, mate_1)
        self.assertSeated(doc, post_b, mate_2)

    def test_the_solve_places_every_instance_where_the_scene_puts_it(self):
        doc, (post_a, shelf_i, post_b), _ = self.stand()
        solved = solve_document(doc, resolver=self.ws)
        for node in (post_a, shelf_i, post_b):
            self.assertIsNone(solved.fault(node), f"{node} records no fault")
        # The near post is pinned to the world, so its world placement
        # is its pin's pose.
        for got, want in zip(
            solved.placement(doc, post_a).origin,
            (0.0, bench_scene.ROOT_OFFSET_Y, 0.0),
            strict=True,
        ):
            self.assertAlmostEqual(got.meters, want, delta=1e-12)
        # The far post: composed outward from the near post along the
        # mates, never stored. The scene's own expected translation.
        far = solved.placement(doc, post_b)
        want = (
            SEAT_B[0] - SEAT_A[0],
            bench_scene.ROOT_OFFSET_Y,
            0.0,
        )
        for got, expected in zip(far.origin, want, strict=True):
            self.assertAlmostEqual(got.meters, expected, delta=1e-12)
        # And the ROTATION, which a translation check cannot see: both
        # mates rest a cap on a cap at zero spin, so composing out from
        # the near post must leave the post's own axes unturned. A
        # solve that rotated the post and still landed its seating
        # point would pass the check above and put the part in
        # sideways.
        for got, want in zip(
            far.columns, ((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)), strict=True
        ):
            for g, w in zip(got, want, strict=True):
                self.assertAlmostEqual(g, w, delta=1e-12)
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
        """A loose post — its bundle empty, so a mate is what places it
        — and a shelf pinned to the world at the identity."""
        doc = doc or Doc(random_document_id() and "refuse")
        post_i = doc.insert(Node.instantiate_part(self.post_ref, []))
        shelf_i = self.pinned(doc, self.shelf_ref)
        return doc, post_i, shelf_i

    def stand_planar(self):
        """The near post pinned to the world and the shelf resting on
        it by one plane mate and no values: what the stand's first seat
        is before its values set where the shelf rests."""
        doc = Doc("pncad-stand-planar")
        post_a = self.pinned(doc, self.post_ref)
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, []))
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        mate = doc.constrain(
            shelf_i,
            Mate(
                PoseDef.plane(shelf_i, s_bottom),
                PoseDef.flip(PoseDef.plane(post_a, a_top)),
                ContactClass.Rest,
            ),
            resolver=self.ws,
        )
        return doc, (post_a, shelf_i), mate

    def test_a_single_planar_rest_leaves_the_copy_loose(self):
        doc, (post_a, shelf_i), _ = self.stand_planar()
        solved = solve_document(doc, resolver=self.ws)
        # One planar rest fixes the seating plane and nothing else — the
        # shelf may still slide and spin in it, and no value sets where.
        # A bundle that pins less than the copy needs is no refusal: the
        # shelf is a loose copy, in a space of its own, and has no world
        # pose.
        self.assertIsNone(solved.fault(shelf_i))
        ev = evaluate(doc, resolver=self.ws)
        self.assertIn(shelf_i, ev.loose)
        self.assertEqual(ev.space(shelf_i), shelf_i)
        self.assertEqual(ev.space(post_a), "world")
        with self.assertRaises(pncad.EvaluationError) as caught:
            solved.placement(doc, shelf_i)
        self.assertEqual(caught.exception.kind, "space_mismatch")

    def test_two_seats_at_different_heights_on_one_copy_refuse_at_the_door(self):
        """The tour's second refusal, from Python. Two rests in ONE
        bundle fix one normal twice, whatever their heights: a plane
        rest and the same rest on the post's top set back 10 mm along
        its normal (a standoff) are refused where the second is
        authored, naming both, measuring nothing — ending on the
        recourse, which is the repair rather than the diagnosis."""
        doc = Doc("pncad-two-heights")
        post_a = self.pinned(doc, self.post_ref)
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, []))
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        held = doc.constrain(
            shelf_i,
            Mate(
                PoseDef.plane(shelf_i, s_bottom),
                PoseDef.flip(PoseDef.plane(post_a, a_top)),
                ContactClass.Rest,
            ),
            resolver=self.ws,
        )
        seat = PoseDef.flip(PoseDef.plane(post_a, a_top))
        lower = PoseDef.slide(seat, RoleRead(seat, "normal"), 0.01 * m)
        with self.assertRaises(pncad.EditError) as caught:
            doc.constrain(
                shelf_i,
                Mate(PoseDef.plane(shelf_i, s_bottom), lower, ContactClass.Rest),
                resolver=self.ws,
            )
        fault = caught.exception.fault
        self.assertEqual(fault.variant, "mate_overconstrained")
        self.assertIn(held, fault.constraints)
        self.assertEqual(fault.free, 1)
        self.assertIn(pncad.OVERCONSTRAINED_RECOURSE, str(fault))
        # The refused edit leaves the document as it was: the first
        # rest alone, the shelf loose on the post.
        self.assertIsNone(solve_document(doc, resolver=self.ws).fault(shelf_i))

    def test_a_mate_reading_its_own_copy_on_both_sides_refuses(self):
        """A mate equates the copy's pose with a pose of the space it
        joins, so a `to` read off the copy's own output reads the
        placement from its own bundle: a cycle, which the edit door
        refuses where it is authored, and nothing enters the
        document."""
        doc = Doc("self-mate")
        post_i = doc.insert(Node.instantiate_part(self.post_ref, []))
        doc.apply(DocEdit.set_label(post_i, "post"))
        face = self.instance_face(doc, post_i, CapEnd.End)
        before = content_pin(doc)
        with self.assertRaises(pncad.EditError) as caught:
            doc.constrain(
                post_i,
                Mate(
                    PoseDef.plane(post_i, face),
                    PoseDef.flip(PoseDef.plane(post_i, face)),
                    ContactClass.Rest,
                ),
            )
        refusal = caught.exception
        self.assertEqual(refusal.variant, "would_cycle")
        # The refusal speaks the instance as the document holds it.
        self.assertIn(f'InstantiatePart "post" ({tag(post_i)})', str(refusal))
        self.assertEqual(content_pin(doc), before, "nothing entered")

    def test_a_mate_on_a_transformed_instance_is_read_at_the_transform(self):
        """A pose reads a face of a NODE by a NAME. Wrap an instance in
        a transform and the name is unchanged — a transform mints no
        name — so the node is the only thing that says which geometry
        the pose reads. Read at the transform, the plane is the
        transformed body's, and the seat holds where the transformed
        body actually is."""
        doc = Doc("pncad-mate-operand")
        post_a = doc.insert(Node.instantiate_part(self.post_ref, []))
        shelf_i = self.pinned(doc, self.shelf_ref)
        lifted = doc.insert(
            Node.transform(
                shelf_i,
                (
                    Formula.length_in(0, m),
                    Formula.length_in(0, m),
                    Formula.length_in(0.25, m),
                ),
                (Formula.literal(0.0), Formula.literal(0.0), Formula.literal(1.0)),
                Formula.literal(0.0 * pncad.rad),
            )
        )
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        _at_transform, _ = rest(
            doc, post_a, (post_a, a_top), (lifted, s_bottom), (0.0, 0.0), resolver=self.ws
        )
        # The transform is invisible to NAMING (the name resolves
        # through it to the minting instance) and visible to the POSE
        # (read off the transform's own output).
        solved = solve_document(doc, resolver=self.ws)
        self.assertIsNone(solved.fault(post_a))
        self.assertAlmostEqual(
            solved.placement(doc, post_a).origin[2].meters, 0.25 - POST_HEIGHT, delta=1e-12
        )
        # Read at the instance instead and it is a different mate: the
        # node is part of what a pose says.
        [(_, written)] = doc.bundle(post_a)[:1]
        at_mint = Mate(
            PoseDef.plane(post_a, a_top),
            PoseDef.flip(PoseDef.plane(shelf_i, s_bottom)),
            ContactClass.Rest,
        )
        at_lifted = Mate(
            PoseDef.plane(post_a, a_top),
            PoseDef.flip(PoseDef.plane(lifted, s_bottom)),
            ContactClass.Rest,
        )
        self.assertNotEqual(at_mint, at_lifted)
        self.assertEqual(written.class_, ContactClass.Rest)

    def test_a_pose_read_off_a_failed_node_names_its_own_cause(self):
        """The pose's face is read off a transform that cannot place
        what it reads: its rotation axis has no measurable length. The
        door admits the mate, which is structural; where the placement
        evaluates, it is poisoned through the pose it reads, and the
        refusal names the transform that failed and carries its own
        refusal in the transform's own words."""
        doc = Doc("pncad-placer-refused")
        post_a = doc.insert(Node.instantiate_part(self.post_ref, []))
        shelf_i = self.pinned(doc, self.shelf_ref)
        lifted = doc.insert(
            Node.transform(
                shelf_i,
                (
                    Formula.length_in(0, m),
                    Formula.length_in(0, m),
                    Formula.length_in(0.25, m),
                ),
                (Formula.literal(1e200), Formula.literal(0.0), Formula.literal(0.0)),
                Formula.literal(0.5 * pncad.rad),
            )
        )
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        doc.constrain(
            post_a,
            Mate(
                PoseDef.plane(post_a, a_top),
                PoseDef.flip(PoseDef.plane(lifted, s_bottom)),
                ContactClass.Rest,
            ),
        )
        doc.apply(DocEdit.set_label(lifted, "lift"))
        solved = solve_document(doc, resolver=self.ws)
        # Poisoned, not refused: the bundle never folded.
        self.assertIsNone(solved.fault(post_a))
        with self.assertRaises(pncad.EvaluationError) as caught:
            solved.placement(doc, post_a)
        raised = caught.exception
        self.assertEqual(raised.reason, "poisoned")
        self.assertEqual(raised.through, lifted)
        self.assertIn(f'Transform "lift" ({tag(lifted)})', str(raised))
        self.assertIn("transform rotation axis", str(raised))

    def test_a_non_finite_world_move_refuses_where_it_is_written(self):
        """A world read's moves are numbers, and a number that is not
        finite is no literal: the read refuses where it is built, before
        any mate or document holds it."""
        with self.assertRaises(pncad.LiteralError):
            WorldRead.frame().slide(WorldRead.through("x"), float("inf") * m)

    def test_a_mate_whose_operand_is_not_live_refuses_at_the_door(self):
        """A pose is a read of the body its face is selected in, so a
        body that is not a live node at insert is a typo, refused at the
        edit door as the unresolved read it is."""
        doc, post_i, shelf_i = self.two_instances()
        top = self.instance_face(doc, post_i, CapEnd.End)
        bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        dead = doc.insert(Node.instantiate_part(self.post_ref))
        doc.apply(DocEdit.delete_node(dead))
        with self.assertRaises(pncad.EditError) as caught:
            doc.constrain(
                post_i,
                Mate(
                    PoseDef.plane(post_i, top),
                    PoseDef.flip(PoseDef.plane(dead, bottom)),
                    ContactClass.Rest,
                ),
            )
        self.assertEqual(caught.exception.variant, "unresolved_input")

    def test_the_door_refuses_what_a_mates_structure_says(self):
        """The edit door is structural: it refuses a mate whose poses
        are of two kinds, whose world read's moves do not compose, or
        whose `on` reads another copy, and a value on a freedom its
        mate's kind fixes — each carrying the fault whole, its payload
        on the words a caller branches on."""
        doc, post_i, shelf_i = self.two_instances()
        top = self.instance_face(doc, post_i, CapEnd.End)
        bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        edge = sorted(evaluate(doc, resolver=self.ws).all_edges(shelf_i))[0]

        def refused(mate):
            with self.assertRaises(pncad.EditError) as caught:
                doc.constrain(post_i, mate)
            self.assertEqual(caught.exception.variant, "mate_refused")
            return caught.exception.fault

        kinds = refused(
            Mate(PoseDef.plane(post_i, top), PoseDef.axis(shelf_i, edge), ContactClass.Rest)
        )
        self.assertEqual(kinds.variant, "mate_kinds_differ")
        self.assertEqual((kinds.on_kind, kinds.to_kind), ("plane", "axis"))
        self.assertEqual(kinds.mate.placement, post_i)
        slid = WorldRead.through("x").slide(WorldRead.through("x"), 0.1 * m)
        self.assertEqual(slid.kind, "axis")
        bad = slid.role("normal")
        self.assertIsNone(bad.kind)
        world = refused(Mate(PoseDef.plane(post_i, top), bad, ContactClass.Rest))
        self.assertEqual((world.variant, world.inner_variant), ("mate_world_read", "role"))
        elsewhere = refused(
            Mate(PoseDef.plane(shelf_i, bottom), WorldRead.through("xy"), ContactClass.Rest)
        )
        self.assertEqual(elsewhere.variant, "mate_not_on_the_copy")
        floor = doc.constrain(
            post_i, Mate(PoseDef.plane(post_i, top), WorldRead.through("xy"), ContactClass.Rest)
        )
        with self.assertRaises(pncad.EditError) as caught:
            doc.add_value(floor, Coord.slide("z"), 0.1 * m)
        fixed = caught.exception.fault
        self.assertEqual(fixed.variant, "mate_not_a_freedom")
        self.assertEqual(fixed.value_ref.placement, post_i)
        self.assertEqual((fixed.coord, fixed.mate_kind), (Coord.slide("z"), "plane"))
        # A number of the other dimension is the binding's to refuse,
        # before any edit.
        with self.assertRaises(ValueError):
            doc.add_value(floor, Coord.slide("x"), 0.1 * rad)

    def test_a_class_the_gate_cannot_mint_refuses_at_the_gate(self):
        doc, _, (mate_1, mate_2) = TestBenchStand.stand(self, class_=ContactClass.Tangent)
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(doc, evaluate(doc, resolver=self.ws))
        # A Tangent mate SOLVES and mints nothing at rest: the two
        # doors admit different sets, which is the whole reason the
        # admission table is one value both read.
        self.assertEqual(caught.exception.variant, "unminted_mates")
        # EVERY mate that did not mint, in document order. The stand
        # declares TWO Tangent mates, and both are named: an
        # expectation built from the answer's own length could not red
        # on a dropped row, which is the one thing this unit exists to
        # prevent, so both mates and both words are written out.
        rows = caught.exception.refusals
        self.assertEqual(
            [(r.variant, r.mate, r.class_) for r in rows],
            [
                ("no_at_rest_record", mate_1, ContactClass.Tangent),
                ("no_at_rest_record", mate_2, ContactClass.Tangent),
            ],
        )
        # And it says what to do about it: `Rest` is the one class v1
        # carries all the way to the gate.
        self.assertIn(
            pncad.NO_AT_REST_RECORD_RECOURSE, str(caught.exception)
        )

    def test_the_gate_speaks_each_unminted_mate_from_the_document(self):
        # The gate holds the document, so the message and each row's
        # `str` say a mate's placement by its kind, label and tag;
        # `mate` keeps the address.
        doc, (_, shelf_i, post_b), (mate_1, mate_2) = TestBenchStand.stand(
            self, class_=ContactClass.Tangent
        )
        doc.apply(DocEdit.set_label(shelf_i, "left seat"))
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(doc, evaluate(doc, resolver=self.ws))
        rows = caught.exception.refusals
        self.assertEqual([r.mate for r in rows], [mate_1, mate_2])
        labelled = f'InstantiatePart "left seat" ({tag(shelf_i)})'
        self.assertIn(labelled, str(rows[0]))
        self.assertIn(f"InstantiatePart {tag(post_b)}", str(rows[1]))
        self.assertNotIn("left seat", str(rows[1]))
        self.assertIn(labelled, str(caught.exception))

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

    def test_a_mate_read_below_a_placer_declares_nothing(self):
        """The shelf is lifted by a transform and the transform is
        read by a `placed_union`; the mate is read AT the transform.
        The solve places it and the product gathers, but no placement
        reads the transform, so the mate declares nothing: the seat it
        would have declared is a touch between two copies, which the
        gather records and the `unproven-coincidence` check reports
        against those copies, and the gate refuses nothing."""
        doc = Doc("pncad-moved-above")
        # The post is placed by the mate and its values alone; the shelf
        # is pinned to the world, and the transform lifts a construction
        # of it clear of the post's height.
        post_a = doc.insert(Node.instantiate_part(self.post_ref, []))
        shelf_i = self.pinned(doc, self.shelf_ref)
        lifted = doc.insert(
            Node.transform(
                shelf_i,
                (
                    Formula.length_in(0, m),
                    Formula.length_in(0, m),
                    Formula.length_in(1.0, m),
                ),
                (Formula.literal(0.0), Formula.literal(0.0), Formula.literal(1.0)),
                Formula.literal(0.0 * pncad.rad),
            )
        )
        # Two copies, the second clear of the post and of the first:
        # the row is about the copy the mate names.
        family = doc.insert(
            Node.placed_union(
                lifted, Formula.count(2), PatternKind.linear((
                    Formula.literal(1.0),
                    Formula.literal(0.0),
                    Formula.literal(0.0),
                ), Formula.length_in(2.0 * SHELF_LENGTH, m))
            )
        )
        # The world holds the post, the shelf and the union: the
        # transform reaches the product only through it.
        doc.place(family, resolver=self.ws)
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        rest(
            doc,
            post_a,
            (post_a, a_top),
            (lifted, s_bottom),
            bench_scene.POST_UNDER_SHELF,
            resolver=self.ws,
        )
        self.assertIsNone(solve_document(doc, resolver=self.ws).fault(post_a))
        ev = evaluate(doc, resolver=self.ws)
        product(doc, ev)
        assemble(doc, ev)
        seats = [
            f
            for f in run_checks(doc, ev).findings
            if f.check == CheckId.UnprovenCoincidence and f.copies is not None
        ]
        self.assertTrue(seats, "the seat is the two copies' finding")

    def test_a_plane_read_off_an_edge_refuses_where_the_mate_is_added(self):
        """A mate's declaration is a FACE-PAIR contact, and a plane is
        read off a face. Python holds names as opaque text, so a plane
        read off an EDGE's name is built, and the edit door refuses it:
        a selection's kind is the name's own, held where the selection
        is written (`selection_name_kind`), with the side and the
        evaluation's own word in its sentence."""
        doc, post_i, shelf_i = self.two_instances()
        ev = evaluate(doc, resolver=self.ws)
        edge = sorted(ev.all_edges(post_i))[0]
        bottom = one(ev.select(shelf_i, cap_selector(CapEnd.Start, PART_FACE)))
        with self.assertRaises(pncad.EditError) as caught:
            doc.constrain(
                post_i,
                Mate(PoseDef.plane(post_i, edge), PoseDef.flip(PoseDef.plane(shelf_i, bottom)), ContactClass.Rest),
            )
        self.assertEqual(caught.exception.variant, "selection_name_kind")
        self.assertIn("side on", str(caught.exception))
        self.assertIn("edge, not a face", str(caught.exception))
        # The same mate read off the face places the post, so the
        # refusal above is the KIND's and not the fixture's.
        top = one(ev.select(post_i, cap_selector(CapEnd.End, PART_FACE)))
        other, post_j, shelf_j = self.two_instances(Doc("plane-off-a-face"))
        rest(
            other,
            post_j,
            (post_j, top),
            (shelf_j, bottom),
            bench_scene.POST_UNDER_SHELF,
            resolver=self.ws,
        )
        self.assertIsNone(solve_document(other, resolver=self.ws).fault(post_j))

    def test_a_gather_refusal_arrives_under_the_gathers_own_tag(self):
        doc = Doc("no-resolver")
        self.pinned(doc, self.post_ref)
        # Evaluated with no resolver, the instance — a placement — produced
        # no copies, so the GATHER refuses before the gate runs — and it
        # refuses with the gather's own tag, not a wrapper's, because
        # which invariant broke is what a caller branches on.
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(doc, evaluate(doc))
        self.assertEqual(caught.exception.variant, "root_failed")
        self.assertIsNotNone(caught.exception.node)
        self.assertIsNone(caught.exception.refusals)
        with self.assertRaises(pncad.ProductError) as gather:
            product(doc, evaluate(doc))
        self.assertEqual(gather.exception.variant, "root_failed")

    def test_a_gather_refusal_speaks_its_root_from_the_evaluated_document(self):
        doc = Doc("no-resolver-labelled")
        lonely = self.pinned(doc, self.post_ref)
        doc.apply(DocEdit.set_label(lonely, "lonely post"))
        ev = evaluate(doc)
        # The placement failed, and the message says it as the
        # evaluation's document holds it, label and all; said by tag it
        # would read `node <tag>`. The payload keeps the full id.
        for door in (product, pncad.product_named):
            with self.assertRaises(pncad.ProductError) as gather:
                door(doc, ev)
            self.assertEqual(gather.exception.variant, "root_failed")
            self.assertIn('InstantiatePart "lonely post" (', str(gather.exception))
            self.assertEqual(gather.exception.node, lonely)

    def test_a_moved_pin_refuses_and_carries_its_recourse_once_by_either_door(self):
        doc, post_i, _ = self.two_instances()
        # A part legitimately changes on disk. The assembly still pins
        # the old version and is never silently retargeted.
        taller = bench_scene.post(height=POST_HEIGHT * 2)
        self.ws.resave(taller)
        with self.assertRaises(pncad.EvaluationError) as caught:
            evaluate(doc, resolver=self.ws).value(post_i)
        self.assertEqual(caught.exception.kind, "part_pin_mismatch")
        with self.assertRaises(pncad.WorkspaceError) as direct:
            self.ws.resolve(self.post_ref)
        # There are two doors onto this one refusal — the evaluation,
        # which reads the resolver's classified message, and the store,
        # which reads its own `WorkspaceError` — and BOTH have to tell
        # the author what to do about it. Both do, and each says it
        # exactly once: the recourse is written in exactly one place,
        # the store's own message, and the seam carries that message
        # through unaltered rather than re-appending the paragraph
        # (which is what it used to do, #947 — the count through the
        # evaluation was 2 against the store's 1). Counting rather than
        # `assertIn` is the point: `assertIn` passes on one copy and on
        # five, and the failure this pins is a DUPLICATE, not an
        # absence.
        self.assertEqual(str(caught.exception).count(pncad.PIN_MISMATCH_RECOURSE), 1)
        self.assertEqual(str(direct.exception).count(pncad.PIN_MISMATCH_RECOURSE), 1)


class TestMateFaultPayload(BenchWorkspace):
    """**Every payload attribute a mate refusal carries, read off an
    authored mistake.**

    The rule the crate states is that every arm's payload is an
    attribute, present on every arm and `None` where the arm carries
    none — so each row below reads the attributes its arm CARRIES and
    then reads attributes of other arms off the same value, which must
    be `None` rather than raise.

    Two attributes have no row that carries them here and that is a
    fact about the DOORS, not a gap: `margin_low`/`margin_high` are the
    interval scalar's enclosure, which no f64 solve produces.
    `crates/pncad-py/src/tests.rs`'s arm table builds them directly;
    the rows here own the other half, that they are present and `None`
    on the arms an author can reach."""

    def two_instances(self):
        """A loose post, which a mate places, and a shelf pinned to the
        world."""
        doc = Doc(random_document_id() and "payload")
        post_i = doc.insert(Node.instantiate_part(self.post_ref, []))
        shelf_i = self.pinned(doc, self.shelf_ref)
        return doc, post_i, shelf_i

    def test_an_overconstrained_bundle_names_its_placement_and_what_it_leaves(self):
        """A mate the rest of its bundle already fixes is refused where
        it is authored: two plane mates fix a normal twice. The fault
        names the placement and its constraints and how many freedoms
        the bundle would leave, and no single mate."""
        doc = Doc("payload-over")
        post = doc.insert(Node.instantiate_part(self.post_ref, []))

        def facing(normal):
            return PoseDef.plane(post, bench_scene.face_facing(self.ws, doc, post, normal))

        doc.constrain(
            post, Mate(facing((0.0, 0.0, -1.0)), WorldRead.through("xy").flip(), ContactClass.Rest)
        )
        raised = WorldRead.through("xy").slide(WorldRead.through("z"), POST_HEIGHT * m)
        with self.assertRaises(pncad.EditError) as caught:
            doc.constrain(post, Mate(facing((0.0, 0.0, 1.0)), raised, ContactClass.Rest))
        fault = caught.exception.fault
        self.assertEqual(fault.variant, "mate_overconstrained")
        self.assertEqual(fault.placement, post)
        self.assertEqual(fault.free, 1)
        self.assertIsNone(fault.mate)
        self.assertIn(pncad.OVERCONSTRAINED_RECOURSE, str(fault))

    def test_a_solve_read_for_another_document_names_both(self):
        """`SolvedPoses.placement` must answer with a frame or not at
        all, so a solve of ANOTHER document refuses before any frame
        is read — and names both ids as `Doc.id` spells them, so a
        caller compares them directly."""
        doc, _, _ = self.two_instances()
        other = Doc("elsewhere")
        lone = self.pinned(other, self.post_ref)
        with self.assertRaises(pncad.MateError) as caught:
            solve_document(other, resolver=self.ws).placement(doc, lone)
        fault = caught.exception.fault
        self.assertEqual(fault.variant, "mate_poses_of_another_document")
        self.assertEqual(fault.expected_document, doc.id)
        self.assertEqual(fault.found_document, other.id)
        # The subject is two documents, so no mate is named at all.
        self.assertIsNone(fault.mate)
        self.assertIsNone(fault.side)

    def test_a_value_on_a_point_mates_rotation_has_no_chart(self):
        """A lone point mate leaves three rotations with no direction to
        measure one about, so a spin value on it refuses at the door,
        naming the value and why; its other attributes are `None`."""
        doc, post_i, _ = self.two_instances()
        corner = sorted(evaluate(doc, resolver=self.ws).all_vertices(post_i))[0]
        ball = doc.constrain(
            post_i,
            Mate(PoseDef.point(post_i, corner), WorldRead.through("origin"), ContactClass.Rest),
        )
        with self.assertRaises(pncad.EditError) as caught:
            doc.add_value(ball, Coord.spin(), 0)
        fault = caught.exception.fault
        self.assertEqual(fault.variant, "mate_no_chart")
        self.assertEqual(fault.why, "point_rotation")
        self.assertEqual(fault.value_ref.placement, post_i)
        for absent in ("mate", "side", "coord", "named", "free"):
            self.assertIsNone(getattr(fault, absent), absent)


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
        thicker = bench_scene.shelf(thickness=SHELF_THICKNESS * 2)
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
        second = bench_scene.shelf(thickness=SHELF_THICKNESS * 3)
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
        post_i = self.pinned(doc, self.post_ref)
        shelf_i = self.pinned(doc, self.shelf_ref, Frame.translation((0 * m, 0.9 * m, 0 * m)))
        return doc, post_i, shelf_i

    def volume(self, doc):
        return product(doc, evaluate(doc, resolver=self.ws)).mass_properties().volume

    def test_split_and_inline_list_their_node_maps_in_document_order(self):
        # A NodeId has no order a caller can read, so the list's own
        # order is the only one it gets: the document's. Six instances
        # make an id order that happens to match it a 1-in-720 accident.
        doc = Doc("refactor-order")
        cut = []
        for i in range(6):
            cut.append(self.pinned(doc, self.post_ref, Frame.translation((i * m, 0 * m, 0 * m))))
        outcome = pncad.split(doc, cut, random_document_id(), resolver=self.ws)
        self.assertEqual([a for a, _ in outcome.node_map], cut)
        self.assertEqual([b for _, b in outcome.node_map], outcome.part.order())

        self.ws.create(outcome.part)
        spliced = pncad.inline(outcome.remainder, outcome.instance, self.ws)
        self.assertEqual([a for a, _ in spliced.node_map], outcome.part.order())
        landed = {b for _, b in spliced.node_map}
        self.assertEqual(
            [b for _, b in spliced.node_map],
            [n for n in spliced.doc.order() if n in landed],
        )

    def test_split_then_inline_preserves_the_products_material_exactly(self):
        doc, _, shelf_i = self.layout()
        before = self.volume(doc)
        self.assertEqual(doc.placements()[1], shelf_i)

        # The cut takes the shelf, a placement pinned to the world: a
        # part delivers only its world.
        outcome = pncad.split(doc, [shelf_i], random_document_id(), resolver=self.ws)
        # PURE: the two documents come back as VALUES with the edits
        # that produce them, and the original is untouched.
        self.assertEqual(self.volume(doc), before)
        self.assertGreater(len(outcome.part_edits), 0)
        self.assertGreater(len(outcome.remainder_edits), 0)
        self.assertEqual(len(outcome.node_map), 1)
        # An instance of the new part is what replaced the cut, and it
        # carries the part's reference; the remainder holds it beside
        # the post.
        self.assertEqual(len(outcome.remainder.placements()), 2)
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
        thicker = bench_scene.shelf(thickness=SHELF_THICKNESS * 2)
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
        profile = doc.insert(Node.polygon([
            (Formula.length_in(0, m), Formula.length_in(0, m)),
            (Formula.length_in(1, m), Formula.length_in(0, m)),
            (Formula.length_in(1, m), Formula.length_in(1, m)),
        ], plane=doc.sketch_frame()))
        body = doc.insert(Node.extrude(profile, Formula.length_in(1, m)))
        with self.assertRaises(pncad.InlineError) as caught:
            pncad.inline(doc, body, self.ws)
        self.assertEqual(caught.exception.variant, "not_an_instance")
        self.assertEqual(caught.exception.node, body)

    def test_split_refuses_a_definition_tied_to_both_sides_naming_each(self):
        # `k = w + j` is read by nothing; `w` only by the cut block and
        # `j` by the kept one, so `k` could go with neither document.
        doc = Doc("straddle")
        for name, value in (("w", 1.5), ("j", 0.5)):
            doc.apply(DocEdit.declare_var(VarName(name), FreeVar.length(value * m)))
        doc.apply(DocEdit.declare_var(VarName("k"), doc.parse_formula("w + j")))

        def block(x0, depth):
            frame = doc.sketch_frame()
            profile = doc.insert(Node.polygon([
                (Formula.length_in(x0, m), Formula.length_in(0, m)),
                (Formula.length_in(x0 + 1, m), Formula.length_in(0, m)),
                (Formula.length_in(x0 + 1, m), Formula.length_in(1, m)),
            ], plane=frame))
            body = doc.insert(Node.extrude(profile, Formula.length_in(1, m)))
            doc.apply(DocEdit.set_param(body, "distance", doc.parse_formula(depth)))
            return [frame, profile, body]

        cut = block(0.0, "w")
        block(10.0, "j")
        with self.assertRaises(pncad.SplitError) as caught:
            pncad.split(doc, cut, random_document_id())
        refusal = caught.exception
        self.assertEqual(refusal.variant, "definition_straddles_cut")
        self.assertEqual(
            (refusal.param, refusal.moving, refusal.staying), ("k", "w", "j")
        )


class TestTheWorld(BenchWorkspace):
    """`place` and `product`: a document's product is its world, every
    copy a placement defines, and nothing places but `place`."""

    def volume(self, doc):
        return product(doc, evaluate(doc, resolver=self.ws)).mass_properties().volume

    def block(self, doc, x0):
        profile = doc.insert(
            Node.polygon(
                [
                    (Formula.length_in(x0, m), Formula.length_in(0, m)),
                    (Formula.length_in(x0 + 1, m), Formula.length_in(0, m)),
                    (Formula.length_in(x0 + 1, m), Formula.length_in(1, m)),
                    (Formula.length_in(x0, m), Formula.length_in(1, m)),
                ],
                plane=doc.sketch_frame(),
            )
        )
        return doc.insert(Node.extrude(profile, Formula.length_in(1, m)))

    def test_the_placements_say_what_the_product_gathers_and_in_what_order(self):
        doc = Doc("world")
        a = self.block(doc, 0.0)
        b = self.block(doc, 3.0)
        # Nothing is placed by an insert, so the world is empty and the
        # refusal names every unplaced body, in document order.
        self.assertEqual(doc.placements(), [])
        with self.assertRaises(pncad.ProductError) as empty:
            product(doc, evaluate(doc))
        self.assertEqual(empty.exception.variant, "empty_product")
        self.assertEqual(empty.exception.unplaced_bodies, [doc.output(a), doc.output(b)])
        self.assertIsNone(empty.exception.node)
        # The world is the placements in document order, whatever the
        # order of the bodies they read.
        on_b = doc.place(b)
        on_a = doc.place(a)
        self.assertEqual(doc.placements(), [on_b, on_a])
        self.assertEqual(doc.node_kind(on_b), "place")
        self.assertEqual(self.volume(doc), 2.0)

    def test_an_instance_is_a_placement(self):
        """An instance IS a placement: its part's world, moved by its
        bundle — pinned to the world, it is in the product with no
        `place` of its own; loose, it is not."""
        doc = Doc("world-instances")
        post_i = self.pinned(doc, self.post_ref)
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, []))
        self.assertEqual(doc.placements(), [post_i, shelf_i])
        self.assertAlmostEqual(self.volume(doc), POST_VOLUME, delta=1e-12)
        self.assertEqual(evaluate(doc, resolver=self.ws).loose, [shelf_i])

    def test_a_placement_whose_body_is_deleted_strands_and_refuses_naming_it(self):
        doc = Doc("world-stranded")
        body = self.block(doc, 0.0)
        placed = doc.place(body)
        # The delete is accepted: the placement is a reader left
        # reading nothing, never silently re-pointed or dropped.
        doc.apply(DocEdit.delete_node(body))
        self.assertEqual(doc.placements(), [placed])
        with self.assertRaises(pncad.ProductError) as caught:
            product(doc, evaluate(doc))
        self.assertEqual(caught.exception.variant, "stranded_placement")
        self.assertEqual(caught.exception.node, placed)
        self.assertIsNone(caught.exception.unplaced_bodies)

    def test_a_boolean_inserted_without_place_leaves_the_world_as_it_was(self):
        """Nothing places as a side effect: a cut of two placed bodies
        is a new body nobody placed, so the product is still the two
        copies — the same door `DocEdit::place` is for the Rust
        façade, with the same answer."""
        doc = Doc("world-no-side-effect")

        def block(x0):
            profile = doc.insert(
                Node.polygon(
                    [
                        (Formula.length_in(x0, m), Formula.length_in(0, m)),
                        (Formula.length_in(x0 + 1, m), Formula.length_in(0, m)),
                        (Formula.length_in(x0 + 1, m), Formula.length_in(1, m)),
                        (Formula.length_in(x0, m), Formula.length_in(1, m)),
                    ],
                    plane=doc.sketch_frame(),
                )
            )
            return doc.insert(Node.extrude(profile, Formula.length_in(1, m)))

        a, b = block(0.0), block(3.0)
        placed = [doc.place(a), doc.place(b)]
        cut = doc.insert(Node.subtract(a, b))
        self.assertEqual(doc.placements(), placed, "the boolean is not placed")
        self.assertEqual(self.volume(doc), 2.0, "the world is still [a', b']")
        # Placing it is what puts it in the world, beside the two.
        doc.place(cut)
        self.assertEqual(self.volume(doc), 3.0)

    def test_two_placements_of_one_body_are_two_copies(self):
        doc = Doc("world-two-copies")
        body = self.block(doc, 0.0)
        doc.place(body)
        doc.place(body, pose=Frame.translation((2 * m, 0 * m, 0 * m)))
        self.assertEqual(len(doc.placements()), 2)
        self.assertEqual(self.volume(doc), 2.0)

    def test_a_document_with_no_body_has_an_empty_world(self):
        doc = Doc("datum-only")
        doc.insert(Node.datum_plane((
            Formula.length_in(0, m),
            Formula.length_in(0, m),
            Formula.length_in(0, m),
        ), (
            Formula.literal(0.0),
            Formula.literal(0.0),
            Formula.literal(1.0),
        )))
        with self.assertRaises(pncad.ProductError) as caught:
            product(doc, evaluate(doc))
        self.assertEqual(caught.exception.variant, "empty_product")
        self.assertEqual(caught.exception.unplaced_bodies, [], "a datum is no body to place")


#: A seat a metre past the shelf's near end: resting there, the shelf
#: lies in the post's top plane clear of the post.
OFF_THE_POST = (-1.0, SEAT_A[1], 0.0)


class TestCarriedAcrossTheSeam(BenchWorkspace):
    """What an inner document's mates say, at the OUTER document's
    gate.

    A part's declared contacts cross the instantiation seam with its
    geometry, and so does the bookkeeping that names them. Three things
    a Python author sees because of that: a refuted carried declaration
    names the mate that authored it, a declined one reaches the
    frontier arm under the same name, and an inner part whose own mate
    could not be minted refuses the OUTER gate rather than passing
    silently.

    ONE RULE at every door here: where a value carries a FOREIGN mate,
    it carries `of` (the document that mate is in) and `via` (the
    instances this document reached it through), and its `mate` is an
    address in `of`'s space. A bare address with no document is not
    something a caller can look up."""

    def stand_doc(self, seed, class_=ContactClass.Rest, b_seat=None):
        """The bench stand as its OWN document, so it can be
        instantiated.

        The shelf rests on the post's top where `b_seat` — a point of
        its underside, `SEAT_A` unless given — meets the top's centre.
        A seat a metre beyond the shelf's end puts the shelf in the
        post's top plane clear of the post: the shelf seats where the
        mate says and the declared rest is definite counter-evidence
        (`OFF_THE_POST`). A seat moved out to the shelf's edge is the
        GRAZING case: the post's top square lies outside the shelf's
        footprint and shares one edge with it, so the census can decide
        the pair in neither direction."""
        doc = Doc(seed)
        post_a = self.pinned(
            doc, self.post_ref, Frame.translation((0 * m, bench_scene.ROOT_OFFSET_Y * m, 0 * m))
        )
        # The shelf sits where its mate puts it: its bundle is the mate
        # and its values.
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, []))
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        mate = doc.constrain(
            shelf_i,
            Mate(
                PoseDef.plane(shelf_i, s_bottom),
                PoseDef.flip(PoseDef.plane(post_a, a_top)),
                class_,
            ),
            resolver=self.ws,
        )
        slide = bench_scene.shelf_on_post(b_seat or SEAT_A)
        for coord, by in (
            (Coord.slide("x"), slide[0] * m),
            (Coord.slide("y"), slide[1] * m),
            (Coord.spin(), 0),
        ):
            doc.add_value(mate, coord, by)
        self.ws.create(doc)
        return doc, mate, DocRef(doc.id, content_pin(doc))

    def instantiated_pinned(self, seed, ref):
        doc = Doc(seed)
        instance = self.pinned(doc, ref)
        return doc, instance

    def instantiated(self, seed, ref):
        return self.instantiated_pinned(seed, ref)

    def carried(self, findings, relation):
        return [f for f in findings if f.attribution and f.attribution.relation == relation]

    def test_a_refuted_carried_declaration_names_its_mate_and_route(self):
        # The shelf's seat is a metre past its end, so it rests in the
        # post's top plane clear of the post and the declared rest is
        # definite counter-evidence. A mate holds its two planes
        # together, so an angle can no longer refute a rest: the offset
        # is what is left to.
        inner, inner_mate, ref = self.stand_doc("carried-refuted", b_seat=OFF_THE_POST)
        outer, instance = self.instantiated("carried-refuted-outer", ref)
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(outer, evaluate(outer, resolver=self.ws))
        err = caught.exception
        # A refuted declaration is a verdict against the document,
        # whichever document authored it.
        self.assertEqual(err.variant, "at_rest")
        refuted = self.carried(err.findings, "carried_refuted")
        self.assertTrue(refuted, [str(f) for f in err.findings])
        for finding in refuted:
            a = finding.attribution
            # The mate is a constraint of the INNER document, and `of`
            # and `via` are what make that address usable.
            self.assertEqual(a.declaration.mate, inner_mate)
            self.assertEqual(a.of, str(inner.id))
            self.assertEqual(a.via, [instance])

    def test_a_carried_decline_reaches_the_frontier_arm(self):
        # The post moved out to the shelf's end: its top square lies
        # beside the shelf's footprint and shares one edge with it, so
        # the declared rest is decidable in neither direction —
        # nothing refuted, nothing undeclared, nothing decided.
        inner, inner_mate, ref = self.stand_doc(
            "carried-declined", b_seat=(-POST_SECTION / 2, SEAT_A[1], 0.0)
        )
        outer, instance = self.instantiated("carried-declined-outer", ref)
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(outer, evaluate(outer, resolver=self.ws))
        err = caught.exception
        self.assertEqual(err.variant, "uncertified")
        declined = self.carried(err.findings, "carried_declined")
        self.assertTrue(declined, [str(f) for f in err.findings])
        for finding in declined:
            a = finding.attribution
            self.assertEqual(a.declaration.mate, inner_mate)
            self.assertEqual(a.of, str(inner.id))
            self.assertEqual(a.via, [instance])

    def test_an_inner_mate_that_cannot_be_minted_refuses_the_outer_gate(self):
        # A `Tangent` mate solves and mints no record at rest, so the
        # stand refuses its own gate — and an outer document is not at
        # rest over a part whose contact nothing verified.
        inner, inner_mate, ref = self.stand_doc(
            "carried-unmintable", class_=ContactClass.Tangent
        )
        outer, instance = self.instantiated("carried-unmintable-outer", ref)
        outer.apply(DocEdit.set_label(instance, "left bracket"))
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(outer, evaluate(outer, resolver=self.ws))
        err = caught.exception
        self.assertEqual(err.variant, "carried_mint_refusal")
        # Every carried row travels, each with its own route:
        # `through` is the instantiating node of the document that was
        # asked about; the row's `refusal.mate` is the inner
        # document's, and `of` and `via` are what make it a mate a
        # caller can go and find.
        (row,) = err.refusals
        self.assertEqual(row.through, instance)
        self.assertEqual(row.refusal.mate, inner_mate)
        self.assertEqual(row.refusal.variant, "no_at_rest_record")
        self.assertEqual(row.of, str(inner.id))
        self.assertEqual(row.via, [instance])
        self.assertIn("at rest", str(err))
        # The route's first instance is the outer document's, so the
        # row says it as that document holds it; the inner mate is
        # said as the inner document holds it.
        through = f'through InstantiatePart "left bracket" ({tag(instance)})'
        self.assertIn(through, str(row))
        self.assertIn(f"InstantiatePart {tag(inner_mate.placement)}", str(row))
        self.assertIn(through, str(err))

    def test_a_carried_row_says_the_parts_label_where_the_outer_holds_its_id(self):
        # The outer document is the same stand, so it mints the same
        # ids: it holds the inner mate's placement's id as its own,
        # labelled apart. Every spelling of the carried row says the
        # inner document's label, and the outer's never.
        inner, inner_mate, _ = self.stand_doc(
            "carried-twin", class_=ContactClass.Tangent
        )
        inner.apply(DocEdit.set_label(inner_mate.placement, "inner seat"))
        self.ws.resave(inner)
        ref = DocRef(inner.id, content_pin(inner))
        outer, outer_mate, _ = self.stand_doc(
            "carried-twin-outer", class_=ContactClass.Tangent
        )
        self.assertEqual(outer_mate, inner_mate, "both stands mint from the zero chain")
        outer.apply(DocEdit.set_label(outer_mate.placement, "outer seat"))
        instance = self.pinned(outer, ref)
        outer.apply(DocEdit.set_label(instance, "left bracket"))
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(outer, evaluate(outer, resolver=self.ws))
        err = caught.exception
        self.assertEqual(err.variant, "carried_mint_refusal")
        (row,) = err.refusals
        self.assertEqual(row.refusal.mate, inner_mate)
        labelled = f'InstantiatePart "inner seat" ({tag(inner_mate.placement)})'
        for said in (str(err), str(row), str(row.refusal)):
            self.assertIn(labelled, said)
            self.assertNotIn("outer seat", said)

    def test_a_carried_finding_says_the_parts_label_where_the_outer_holds_its_id(self):
        # Both stands seat the shelf off the post, so both declared
        # rests are refuted; the outer document holds the inner mate's
        # placement's id as its own.
        inner, inner_mate, _ = self.stand_doc("carried-twin-refuted", b_seat=OFF_THE_POST)
        inner.apply(DocEdit.set_label(inner_mate.placement, "inner seat"))
        self.ws.resave(inner)
        ref = DocRef(inner.id, content_pin(inner))
        outer, outer_mate, _ = self.stand_doc("carried-twin-refuted-outer", b_seat=OFF_THE_POST)
        self.assertEqual(outer_mate, inner_mate, "both stands mint from the zero chain")
        outer.apply(DocEdit.set_label(outer_mate.placement, "outer seat"))
        self.pinned(outer, ref, Frame.translation((5 * m, 0 * m, 0 * m)))
        with self.assertRaises(pncad.AssemblyError) as caught:
            assemble(outer, evaluate(outer, resolver=self.ws))
        refuted = self.carried(caught.exception.findings, "carried_refuted")
        self.assertTrue(refuted, [str(f) for f in caught.exception.findings])
        for finding in refuted:
            self.assertEqual(finding.attribution.declaration.mate, inner_mate)
            for said in (str(finding), str(finding.attribution)):
                self.assertIn(f'InstantiatePart "inner seat" ({tag(inner_mate.placement)})', said)
                self.assertNotIn("outer seat", said)

    def test_a_certified_assembly_names_the_carried_mates_it_certified_over(self):
        # Two stands side by side, each certifying: the assembly keeps
        # the rows, so it can say which inner mates its verdict
        # answered for.
        inner, inner_mate, ref = self.stand_doc("carried-certified")
        outer = Doc("carried-certified-outer")
        first = self.pinned(outer, ref)
        second = self.pinned(outer, ref, Frame.translation((5 * m, 0 * m, 0 * m)))
        assembly = assemble(outer, evaluate(outer, resolver=self.ws))
        self.assertEqual(assembly.minted, [])
        self.assertEqual(
            [(c.declaration.mate, c.of, c.via) for c in assembly.carried],
            [
                (inner_mate, str(inner.id), [first]),
                (inner_mate, str(inner.id), [second]),
            ],
        )


class TestAPoseFollowsItsFace(BenchWorkspace):
    """A mate's pose is read off a face at every evaluation, through the
    part's own evaluation: it follows the face through an edit of the
    part, its values slide the seat along the face and a parameter can
    drive them, and a face that is gone is the placement's own refusal
    where it evaluates."""

    def seated(self, seed, at=(0.0, 0.0), a_top=None, prelude=None, post=None):
        """The shelf resting on a post pinned in the world (by `post`,
        the "place part" gesture's pin unless given), its values `at` —
        each a length in metres or a formula — and a zero spin."""
        doc = Doc(seed)
        if prelude is not None:
            prelude(doc)
        post_i = post(doc) if post is not None else self.pinned(doc, self.post_ref)
        # The shelf's bundle is empty: the mate is what places it, on
        # the post pinned to the world.
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, []))
        if a_top is None:
            a_top = self.instance_face(doc, post_i, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        mate = doc.constrain(
            shelf_i,
            Mate(
                PoseDef.plane(shelf_i, s_bottom),
                PoseDef.flip(PoseDef.plane(post_i, a_top)),
                ContactClass.Rest,
            ),
            resolver=self.ws,
        )
        slides = [
            doc.add_value(mate, Coord.slide(axis), by if isinstance(by, Formula) else by * m)
            for axis, by in zip(("x", "y"), at, strict=True)
        ]
        doc.add_value(mate, Coord.spin(), 0)
        return doc, post_i, shelf_i, mate, slides

    def test_the_mate_follows_the_edited_face(self):
        """The post stands on the floor by its base cap, so growing the
        part grows it up; the shelf rests on its top cap, read at every
        evaluation."""

        def base_pinned(doc):
            post_i = doc.insert(Node.instantiate_part(self.post_ref, []))
            bench_scene.on_floor(
                doc, post_i, post_i, self.instance_face(doc, post_i, CapEnd.Start), self.ws
            )
            return post_i

        doc, _post_i, shelf_i, mate, _ = self.seated("from-face-follows", post=base_pinned)
        evaluate(doc, resolver=self.ws).value(mate.placement)
        before = solve_document(doc, resolver=self.ws).placement(doc, shelf_i)
        self.assertAlmostEqual(before.origin[2].meters, POST_HEIGHT, places=12)
        # The post grows on disk, edited in place so its cap keeps the
        # name the pose reads; the reference moves; the shelf comes up
        # with the cap, by exactly the height change.
        (extrude,) = [n for n in self.post.order() if self.post.node_kind(n) == "extrude"]
        self.post.apply(
            DocEdit.set_param(extrude, "distance", Formula.length_in(POST_HEIGHT + 0.1, m))
        )
        self.ws.resave(self.post)
        for edit in pncad.update_references(doc, self.post.id, content_pin(self.post)):
            doc.apply(edit, resolver=self.ws)
        evaluate(doc, resolver=self.ws).value(mate.placement)
        after = solve_document(doc, resolver=self.ws).placement(doc, shelf_i)
        self.assertAlmostEqual(
            after.origin[2].meters - before.origin[2].meters, 0.1, places=12
        )

    def test_a_value_slides_the_seat_and_a_parameter_drives_it(self):
        """A slide value moves the copy along the face it rests on: 0.1
        along the chart's x moves the shelf 0.1 along x and leaves its
        height. A value reading a document parameter moves it with one
        value edit."""
        doc, _, shelf_i, _, _ = self.seated("face-offset-plain")
        plain = solve_document(doc, resolver=self.ws).placement(doc, shelf_i)
        doc, _, shelf_i, _, _ = self.seated("face-offset-slid", at=(0.1, 0.0))
        moved = solve_document(doc, resolver=self.ws).placement(doc, shelf_i)
        self.assertAlmostEqual(
            moved.origin[0].meters - plain.origin[0].meters, 0.1, places=12
        )
        self.assertAlmostEqual(moved.origin[2].meters, plain.origin[2].meters, places=12)

        def declare(d):
            d.apply(DocEdit.declare_var(pncad.VarName("slide"), pncad.FreeVar.length(0.1 * m)))

        probe = Doc("face-offset-probe")
        declare(probe)
        doc, _, shelf_i, _, _ = self.seated(
            "face-offset-param", at=(probe.parse_formula("slide"), 0.0), prelude=declare
        )
        at = solve_document(doc, resolver=self.ws).placement(doc, shelf_i)
        self.assertAlmostEqual(at.origin[0].meters, moved.origin[0].meters, places=12)
        doc.apply(
            DocEdit.set_var_value(pncad.VarName("slide"), pncad.FreeValue.length(0.3 * m)),
            resolver=self.ws,
        )
        later = solve_document(doc, resolver=self.ws).placement(doc, shelf_i)
        self.assertAlmostEqual(later.origin[0].meters - at.origin[0].meters, 0.2, places=12)

    def test_a_value_reads_back_as_written_and_compares_bit_exact(self):
        doc, _, shelf_i, mate, (x, _) = self.seated("face-offset-read", at=(0.1, 0.0))
        held = dict(doc.bundle(shelf_i))[x]
        self.assertIsInstance(held, MateValue)
        self.assertEqual((held.mate, held.coord), (mate.constraint, Coord.slide("x")))
        plus = MateValue(mate, Coord.slide("x"), 0.1 * m)
        self.assertEqual(plus, MateValue(mate.constraint, Coord.slide("x"), 0.1 * m))
        self.assertNotEqual(plus, MateValue(mate, Coord.slide("x"), 0.2 * m))
        self.assertNotEqual(plus, MateValue(mate, Coord.slide("y"), 0.1 * m))

    def test_a_vanished_face_refuses_where_the_placement_evaluates(self):
        # The post's own cap, wrapped at the instance as a selection is,
        # with the part-local name innermost — inside the copy the
        # post's placement makes of it — re-headed at a node the post
        # does not have: the shape of a name whose face an edit removed.
        import json

        probe = Doc("from-face-vanished-probe")
        probe_i = self.pinned(probe, self.post_ref)
        spelled = json.loads(self.instance_face(probe, probe_i, CapEnd.End))

        def inner_names(value, depth=0):
            if isinstance(value, dict):
                if depth > 0 and "node" in value:
                    yield value
                for child in value.values():
                    yield from inner_names(child, depth + 1)
            elif isinstance(value, list):
                for child in value:
                    yield from inner_names(child, depth + 1)

        (_copy, local) = list(inner_names(spelled))
        local["node"] = "0:0000000000000063"
        # The door is structural, so the mate is admitted; the pose it
        # reads does not evaluate, which is the placement's refusal.
        doc, _, shelf_i, mate, _ = self.seated(
            "from-face-vanished", a_top=json.dumps(spelled)
        )
        fault = solve_document(doc, resolver=self.ws).fault(shelf_i)
        self.assertEqual(fault.variant, "mate_side_unevaluated")
        self.assertEqual((fault.mate, fault.side), (mate, pncad.MateSide.To))
        self.assertIsNotNone(fault.error)
        self.assertIsInstance(fault.cause, pncad.EvaluationError)
        self.assertIn("did not evaluate", str(fault))


if __name__ == "__main__":
    unittest.main()
