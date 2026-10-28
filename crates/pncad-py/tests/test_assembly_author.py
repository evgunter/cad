"""AUTHORING an assembly from Python: the tour's bench, built from nothing.

`test_assembly_eval.py` is this file's other half. It writes the same
scene into a store and evaluates what it reads back; here nothing goes
near disk that this file did not put there. Two part documents are
authored, written into a workspace, instantiated into two assembly
documents, mated, solved, gathered and checked — the whole of the audit's
row 44 through the public doors.

THE SCENE IS `bench_scene.py`, AND THE ORACLE COMES FROM THE TOUR
-----------------------------------------------------------------
`demos/tour/src/assembly.rs` is the scene these documents ARE, and its
own assertions are the oracle: the layout's three disjoint solids of
`PATTERN_COUNT x post + shelf`, the stand's near post pinned to the
world and each other instance placed by the one mate its bundle holds
and that mate's values, the far post's solved translation, the identity
rotation two face-on-face rests compose to, two recorded seats, and
an at-rest check with nothing to report. Every number below is that scene's, re-derived from the
same base dimensions rather than copied as a total. The tour's bench
also stands on a turntable and carries a crate, whose seat is a
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

WHAT THE CHECKS SAY OF A MATE READ BELOW A PLACER, AND WHY
---------------------------------------------------------
A mate's pose is read off the body it names. A pose may read a node
that a union consumes and no placement reads: the mate states nothing
about the product, and the seat is a touch between
two copies, which the `unproven-coincidence` check reports against
them (the gather's rule, pinned in editor-core).

A plane read off a name that is NOT A FACE is built and admitted —
the door is structural — and refused where its placement evaluates
(`mate_side_unevaluated`, asserted below), not by the checks.

THE SPELLING OF THE PLACED FAMILY, MEASURED RATHER THAN CLAIMED
---------------------------------------------------------------
The layout's placed posts are the members of a row — the tour's own
spelling — and `bench_scene.layout`'s `posts="union"` places the row's
union whole instead. `TestBenchLayout` runs its whole battery against
BOTH, which is what turns "the fused spelling says the same placed
family" from a sentence into a reading: the material, the per-member cap
frames, the member-qualified names, their denotations and an at-rest
check with nothing to report are the same either way, and
`test_the_family_is_plural_and_its_union_one_body` is what keeps that
from being a coincidence of two identical nodes.

The mate door is measured on the tour's own node: a mate head stands
on a row's member, and the solve places the shelf against it.
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
    Placed,
    PoseDef,
    RoleRead,
    SegTag,
    CheckId,
    VarDecl,
    VarName,
    Workspace,
    content_pin,
    evaluate,
    m,
    product,
    rad,
    random_document_id,
    run_checks,
    solve_document,
)

from placing import against, moves, placed, plane_on, assemble_world_copies
from measuring import pinned


def at_rest(doc, evaluation):
    """The at-rest check's findings over `evaluation`, in report order."""
    return [f for f in run_checks(doc, evaluation).findings if f.check == CheckId.AtRest]


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
        identity when `None`, the "place part" gesture), by the values
        that put it there."""
        node = doc.insert(Node.instantiate_part(ref, doc.world))
        if pose is not None:
            doc.set_constraints(node, Placed.unmated(moves(doc, pose)))
        return node


def identity():
    """The identity bundle: no mate, and the constant zero on a slide
    along each of the target space's axes and a spin about each."""
    axes = ("x", "y", "z")
    return [MateValue(Coord.slide(a), 0) for a in axes] + [
        MateValue(Coord.spin(a), 0) for a in axes
    ]


def zeros():
    """The three values a plane mate leaves, the bundle's first entry,
    each the constant zero: its slides along the chart's `x` and `y` and
    its spin about the target's own direction."""
    return [MateValue(coord, 0) for coord in (Coord.slide("x"), Coord.slide("y"), Coord.spin())]


#: The two spellings `bench_scene.layout`'s `posts=` admits, by the
#: name this file reports a failure under. Every expectation below is
#: asserted against BOTH: what the audit row used to carry was a CLAIM
#: that the fused spelling says the same placed family as the tour's
#: plural one, and a claim about two spellings is measured by running
#: both, not by asserting one and writing a sentence about the other.
SPELLINGS = {"members": "members", "union": "union"}


class TestBenchLayout(BenchWorkspace):
    """Row 44's flat-pack: The posts on their side and the shelf
    beside them, nothing touching — A5's disjoint half.

    The layout has no mates, so the bundles and the solve are
    `TestBenchStand`'s expectations and not this document's; what this
    one is held to is the material, the placements, the names and an
    at-rest check with nothing to report."""

    def layout(self, posts="members"):
        doc, post_i, family, shelf_i = bench_scene.layout(
            self.ws, self.post_ref, self.shelf_ref, posts
        )
        return doc, family, post_i, shelf_i

    def test_the_layout_gathers_the_scenes_material_exactly(self):
        for name, posts in SPELLINGS.items():
            with self.subTest(posts=name):
                doc, _, _, _ = self.layout(posts)
                # The placements state the product: the post instance,
                # a loose pattern master and so no part of it; the row;
                # the shelf instance, pinned to the world; then the
                # posts' copies (each member placed, or the fused family
                # placed whole).
                placed = PATTERN_COUNT + 3 if name == "members" else 4
                self.assertEqual(len(doc.placements()), placed)
                ev = evaluate(doc, resolver=self.ws)
                volume = pinned(product(doc, ev)).volume
                self.assertAlmostEqual(
                    volume, PATTERN_COUNT * POST_VOLUME + SHELF_VOLUME, delta=1e-12
                )

    def test_every_patterned_post_answers_to_an_instance_qualified_name(self):
        for name, posts in SPELLINGS.items():
            with self.subTest(posts=name):
                doc, family, _, _ = self.layout(posts)
                ev = evaluate(doc, resolver=self.ws)
                caps = ev.select(
                    family, cap_selector(CapEnd.End, [SegTag.Member])
                )
                # One per member, and the name NESTS rather than
                # concatenating: the member's index, then the part's
                # own cap. Distinct names are the
                # structural claim the volume cannot make.
                self.assertEqual(len(caps), PATTERN_COUNT)
                self.assertEqual(len(set(caps)), PATTERN_COUNT)

    def test_the_disjoint_layout_has_nothing_at_rest_to_report(self):
        for name, posts in SPELLINGS.items():
            with self.subTest(posts=name):
                doc, _, _, _ = self.layout(posts)
                assemble_world_copies(doc)
                ev = evaluate(doc, resolver=self.ws)
                # Nothing touches, so the at-rest check reports nothing,
                # and nothing it could not look at. That is exactly what
                # "disjoint assemblies validate today" means — and it is
                # what a PLURAL value reaching the census had to be
                # measured against rather than assumed: the census takes
                # the family either way.
                self.assertEqual(at_rest(doc, ev), [])
                body, names = pncad.product_named(doc, ev)
                self.assertGreater(len(names), 0)
                self.assertAlmostEqual(
                    pinned(body).volume,
                    PATTERN_COUNT * POST_VOLUME + SHELF_VOLUME,
                    delta=1e-12,
                )

    def test_the_family_is_plural_and_its_union_one_body(self):
        """What the two spellings actually place, which is the
        difference the rows either side of this one are measuring
        ACROSS.

        The row answers the family unfused — N bodies, and no
        single-body door at all — where its union answers one body of
        the same material. Every expectation in this class holds for
        both; this row is why that is a finding and not a coincidence.
        """
        doc, family, _, _ = self.layout()
        fused = doc.insert(Node.union(family))
        ev = evaluate(doc, resolver=self.ws)
        value = ev.value(family)
        self.assertEqual(value.kind, "instances")
        bodies = value.bodies()
        self.assertEqual(len(bodies), PATTERN_COUNT)
        # The plural value has no single-body door, and says so typed
        # rather than handing back the first.
        with self.assertRaises(pncad.EvaluationError):
            value.body()
        one_body = ev.value(fused)
        self.assertEqual(one_body.kind, "boolean")
        self.assertAlmostEqual(
            sum(pinned(b).volume for b in bodies),
            pinned(one_body.body()).volume,
            delta=1e-12,
        )

    def test_a_mate_head_stands_on_a_patterned_copy(self):
        """The mate side of the scene's placed family, measured — NOT
        the scene, which declares nothing.

        A mate's `to` may read a face of a row's member: the member is
        read out of the family (`row[0]` — a pose reads one body), the
        plane is read off the face the row placed, and the solve places
        the shelf against it. The mate and its values
        are the shelf's bundle, written whole in place of the identity
        it was inserted with, so they are what places the shelf.
        """
        doc = Doc("pncad-mate-onto-pattern")
        post_i = self.pinned(doc, self.post_ref)
        doc.apply(DocEdit.declare_var(VarName("k"), VarDecl.index(PATTERN_COUNT)))
        family = doc.insert(
            Node.linear_pattern(
                VarName("k"),
                post_i,
                doc.world,
                [("y", Formula.length_in(SHELF_LENGTH, m))],
            )
        )
        first = doc.place(family[0], doc.world, Placed.identity())
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, doc.world))
        ev = evaluate(doc, resolver=self.ws)
        cap = one(ev.select(first, cap_selector(CapEnd.End, [SegTag.Member])))
        bottom = one(ev.select(shelf_i, cap_selector(CapEnd.Start, PART_FACE)))
        joint, values = rest(
            doc, shelf_i, (shelf_i, bottom), (first, cap), SHELF_ON_POST
        )
        solved = solve_document(doc, resolver=self.ws)
        self.assertIsNone(solved.fault(shelf_i))
        self.assertEqual([ref for ref, _ in doc.bundle(shelf_i)], [joint, *values])
        # Placed into the world, on a copy of the post there.
        self.assertEqual(doc.space(shelf_i), doc.space(first))

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
        world mate each placed copy holds moves it, bit for bit, to the
        pose the scene lays it at.

        The index is read off the GEOMETRY, not off the name: a
        member-qualified name is opaque text and the role segment the
        Rust tour filters on (`Member { at, .. }`) is deliberately
        unreadable from Python. Which is the honest shape here — the
        question is where a cap sits, and the answer is what
        identifies it.
        """
        # Rung 1: the part's own cap, in the part's own coordinates —
        # on the copy its one placement puts in its world.
        part_ev = evaluate(self.post)
        (placed,) = self.post.placements()
        part_cap = one(part_ev.select(placed, cap_selector(CapEnd.End)))
        local = part_ev.face_frame(placed, part_cap).origin

        for name, posts in SPELLINGS.items():
            with self.subTest(posts=name):
                doc, family, post_i, _ = self.layout(posts)
                ev = evaluate(doc, resolver=self.ws)

                # Rung 2: the master stands in a space of its own, at its
                # part's own coordinates: the cap sits where the part
                # says.
                self.assertEqual(doc.space(family), doc.space(post_i))
                self.assertNotEqual(doc.space(family), doc.output(doc.world))
                placed = tuple(c.meters for c in local)
                # Each placed copy's world pose is the pose the gesture
                # placed it at: the scene's laid-down pose.
                world = solve_document(doc, resolver=self.ws)
                lying = Frame.rotate_then_translate(
                    (0.0, 1.0, 0.0),
                    -math.pi / 2 * rad,
                    ((bench_scene.FLAT_PACK_GAP + POST_HEIGHT) * m, 0 * m, 0 * m),
                )
                for copy in doc.placements()[3:]:
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
                    family, cap_selector(CapEnd.End, [SegTag.Member])
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
                    family, cap_selector(CapEnd.End, [SegTag.Member])
                )
                for cap in caps:
                    denotation = ev.denotation(family, cap)
                    self.assertFalse(denotation.tied)
                    self.assertEqual(denotation.candidates, 1)

    def test_an_instance_stands_where_its_part_stands_and_is_pinned_where_the_gesture_puts_it(self):
        """An inserted instance with no bundle written stands where its
        part's world stands, by the identity bundle — no mate, the
        constant zero on every freedom, which is the "place part"
        gesture; values written in its place put it at a pose; a value
        removed is written zero again, so the copy stays pinned; and an
        instance placed into a space of its own has no world pose."""
        doc = Doc("inserted")
        node = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
        identity = doc.bundle(node)
        self.assertEqual(len(identity), 6)
        self.assertTrue(all(isinstance(held, MateValue) for _, held in identity))
        for got in solve_document(doc, resolver=self.ws).placement(doc, node).origin:
            self.assertAlmostEqual(got.meters, 0.0, delta=1e-12)
        moved = doc.set_constraints(node, Placed.unmated([MateValue(Coord.slide("x"), 1 * m)]))
        self.assertEqual([ref for ref, _ in doc.bundle(node)], moved)
        at = solve_document(doc, resolver=self.ws).placement(doc, node).origin
        for got, want in zip(at, (1.0, 0.0, 0.0), strict=True):
            self.assertAlmostEqual(got.meters, want, delta=1e-12)
        doc.apply(DocEdit.remove_constraint(moved[0]))
        for got in solve_document(doc, resolver=self.ws).placement(doc, node).origin:
            self.assertAlmostEqual(got.meters, 0.0, delta=1e-12)

        aside = doc.insert(Node.instantiate_part(self.post_ref, doc.new_space()))
        solved = solve_document(doc, resolver=self.ws)
        self.assertIsNone(solved.fault(aside), "a copy in another space is not a refusal")
        with self.assertRaises(pncad.EvaluationError) as caught:
            solved.placement(doc, aside)
        self.assertEqual(caught.exception.kind, "space_mismatch")


class TestBenchStand(BenchWorkspace):
    """Row 44's assembled bench: Two posts and a shelf, the shelf
    SEATED on the posts by mates — only the near post is pinned to the
    world, and the other two poses are solved."""

    def stand(self):
        return bench_scene.stand(self.ws, self.post_ref, self.shelf_ref)

    def assertSeated(self, doc, node, mate):
        """`node`'s bundle is `mate`, which reads another copy, then
        the three values its plane leaves."""
        bundle = doc.bundle(node)
        self.assertEqual(bundle[0][0], mate)
        self.assertIsInstance(bundle[0][1], Mate)
        self.assertEqual(
            [(type(held), held.coord) for _, held in bundle[1:]],
            [(MateValue, Coord.slide("x")), (MateValue, Coord.slide("y")), (MateValue, Coord.spin())],
        )

    def test_each_mate_is_a_constraint_of_the_copy_it_places(self):
        doc, (post_a, shelf_i, post_b), (mate_1, mate_2) = self.stand()
        # The near post is pinned to the world by values alone; each
        # other instance's bundle is the one mate that places it, with
        # its values.
        self.assertTrue(all(isinstance(held, MateValue) for _, held in doc.bundle(post_a)))
        self.assertSeated(doc, shelf_i, mate_1)
        self.assertSeated(doc, post_b, mate_2)
        self.assertEqual((mate_1.placement, mate_2.placement), (shelf_i, post_b))
        # Each placed into the world: every copy is the world's.
        for node in (post_a, shelf_i, post_b):
            self.assertEqual(doc.space(node), doc.space(post_a))
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
        doc, (_post_a, shelf_i, post_b), (_mate_1, _mate_2) = self.stand()
        # `set_constraints`: the stand's second bundle places post_b on
        # the shelf, written over the identity, whose six zeros were
        # anonymous variables and retire — that is what the door just
        # accepted.
        identity_retired = ["anonymous_var_removed"] * 6
        self.assertEqual([m.variant for m in doc.last_maintenance], identity_retired)

        def slab(x, y, z, on=None):
            """A box, inserted through the `insert` door, placed against
            `on` `z0` up where it is combined with it."""
            profile = doc.insert(
                Node.polygon(
                    [
                        (Formula.literal(x[0]), Formula.literal(y[0])),
                        (Formula.literal(x[1]), Formula.literal(y[0])),
                        (Formula.literal(x[1]), Formula.literal(y[1])),
                        (Formula.literal(x[0]), Formula.literal(y[1])),
                    ],
                )
            )
            body = doc.insert(Node.extrude(profile, Formula.literal(z[1] - z[0])))
            if on is None:
                return body
            return against(doc, on, body, Frame.translation((0 * m, 0 * m, z[0])))

        lower = slab((0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        upper = slab((0.25 * m, 0.75 * m), (0.25 * m, 0.75 * m), (1 * m, 1.5 * m), on=lower)
        doc.insert(Node.union([lower, upper]))
        self.assertEqual(doc.last_maintenance, [])
        # `apply`: writing the identity bundle over a mate's records no
        # frame. The mate's poses and selections go with it, each a
        # silent retirement of an anonymous variable.
        doc.apply(DocEdit.set_constraints(post_b, identity()))
        retired = [m.variant for m in doc.last_maintenance]
        self.assertTrue(retired)
        self.assertEqual(set(retired), {"anonymous_var_removed"})
        # A bundle written again: the record is the identity's six zeros
        # retiring, not the mate's retirements still standing from
        # before.
        b_top = self.instance_face(doc, post_b, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        rest(
            doc,
            post_b,
            (post_b, b_top),
            (shelf_i, s_bottom),
            bench_scene.POST_UNDER_SHELF,
        )
        self.assertEqual([m.variant for m in doc.last_maintenance], identity_retired)
        # A REFUSED edit leaves the reading as it was: a mate whose `on`
        # reads another copy pins nothing of this one.
        doc.apply(DocEdit.delete_node(upper))
        stranded = [m.variant for m in doc.last_maintenance]
        elsewhere = Mate(PoseDef.plane(shelf_i, s_bottom), PoseDef.plane(shelf_i, s_bottom))
        with self.assertRaises(pncad.EditError) as caught:
            doc.set_constraints(post_b, [elsewhere, *zeros()])
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

        The cut moves as selected: the near post keeps its world mate in
        the part, and the instance left behind stands where its part's
        world stands. No edit records a frame, so the part reports
        nothing — a variable its own edits mint and retire nets out —
        and the remainder and an inline back report only the anonymous
        variables their edits retire — the cut mates' reads, and the
        part's world reads the inline re-writes at the instance's
        pose."""
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
        # The cut moves as selected: the instance left behind stands
        # where its part's world stands — the identity bundle, six values
        # and no mate — and the near post keeps its pin.
        left_behind = outcome.remainder.bundle(outcome.instance)
        self.assertEqual(len(left_behind), 6)
        for _, value in left_behind:
            self.assertIsInstance(value, MateValue)

        def pinned(part):
            # A placement pinned to its target by values alone.
            return [
                n
                for n in part.placements()
                if all(isinstance(held, MateValue) for _, held in part.bundle(n))
            ]

        self.assertEqual(len(pinned(outcome.part)), 1)
        # The door is PURE, so the input's own reading is untouched —
        # still the record the stand's second bundle left.
        self.assertEqual(
            [m.variant for m in doc.last_maintenance], ["anonymous_var_removed"] * 6
        )

        self.ws.create(outcome.part)
        spliced = pncad.inline(outcome.remainder, outcome.instance, self.ws)
        self.assertEqual(
            {m.variant for m in spliced.doc.last_maintenance} - {"anonymous_var_removed"},
            set(),
            spliced.doc.last_maintenance,
        )
        # Inline copies the part verbatim into a new space and places
        # it where the instance stood, under the instance's identity
        # bundle; flatten folds that placement away, and what is left is
        # the document split was given, up to node ids: three instances
        # again, one pinned where the near post was.
        self.assertEqual(spliced.doc.space(spliced.placement), spliced.doc.space(doc.world))
        self.assertEqual(len(spliced.doc.placements()), 4)
        flat = pncad.flatten(spliced.doc, spliced.placement).doc
        self.assertEqual(len(flat.placements()), 3)
        self.assertEqual(len(pinned(flat)), 1)

    def test_only_the_near_post_is_pinned_to_the_world(self):
        doc, (post_a, shelf_i, post_b), (mate_1, mate_2) = self.stand()
        # The near post is pinned by values on the world's axes, no
        # mate; each other bundle is one face mate on another copy and
        # its values, and their poses are solved.
        pin = doc.bundle(post_a)
        self.assertEqual(len(pin), 6)
        self.assertTrue(all(isinstance(held, MateValue) for _, held in pin))
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
        # The shelf sits on top of the posts, at the world's origin in
        # its plane: flush with the bench's front.
        for got, want in zip(
            solved.placement(doc, shelf_i).origin, (0.0, 0.0, POST_HEIGHT), strict=True
        ):
            self.assertAlmostEqual(got.meters, want, delta=1e-12)

    def test_the_flush_seated_stand_has_nothing_at_rest_to_report(self):
        """The seats are mates, so the census's two touches are
        structural: no `AtRest` finding, and none it could not look at."""
        doc, _, _ = self.stand()
        assemble_world_copies(doc)
        ev = evaluate(doc, resolver=self.ws)
        self.assertEqual(at_rest(doc, ev), [])
        self.assertAlmostEqual(
            pinned(product(doc, ev)).volume,
            2 * POST_VOLUME + SHELF_VOLUME,
            delta=1e-12,
        )


class TestAssemblyRefusals(BenchWorkspace):
    """The refusal families an assembly author actually hits. Each is
    reached by authoring the mistake, never by hand-building a
    refusal."""

    def two_instances(self, doc=None):
        """A post standing in the world by the identity bundle — a
        bundle written whole is what places it — and a shelf pinned to
        the world at the identity."""
        doc = doc or Doc(random_document_id() and "refuse")
        post_i = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
        shelf_i = self.pinned(doc, self.shelf_ref)
        return doc, post_i, shelf_i

    def test_a_single_planar_rest_refuses_unpinned(self):
        """One planar rest fixes the seating plane and nothing else —
        the shelf may still slide and spin in it — so a bundle of that
        mate and no values leaves three freedoms unwritten, and every
        bundle pins its copy: the door refuses it, naming the placement,
        and the shelf keeps the bundle it had."""
        doc = Doc("pncad-stand-planar")
        post_a = self.pinned(doc, self.post_ref)
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, doc.world))
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        before = doc.bundle(shelf_i)
        with self.assertRaises(pncad.EditError) as caught:
            doc.set_constraints(
                shelf_i,
                [
                    Mate(
                        PoseDef.plane(shelf_i, s_bottom),
                        PoseDef.flip(PoseDef.plane(post_a, a_top)),
                    )
                ],
            )
        fault = caught.exception.fault
        self.assertEqual(fault.variant, "mate_unpinned")
        self.assertEqual(fault.placement, shelf_i)
        self.assertEqual(fault.free, 3)
        self.assertIn(pncad.UNPINNED_RECOURSE, str(fault))
        self.assertEqual(doc.bundle(shelf_i), before)

    def test_two_seats_at_different_heights_on_one_copy_refuse_at_the_door(self):
        """The tour's second refusal, from Python. Two rests in ONE
        bundle fix one normal twice, whatever their heights: a plane
        rest and the same rest on the post's top set back 10 mm along
        its normal (a standoff) are refused where the second is
        authored, naming both, measuring nothing — ending on the
        recourse, which is the repair rather than the diagnosis."""
        doc = Doc("pncad-two-heights")
        post_a = self.pinned(doc, self.post_ref)
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, doc.world))
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        seat = PoseDef.flip(PoseDef.plane(post_a, a_top))
        lower = PoseDef.slide(seat, RoleRead(seat, "normal"), 0.01 * m)
        before = doc.bundle(shelf_i)
        with self.assertRaises(pncad.EditError) as caught:
            doc.set_constraints(
                shelf_i,
                [
                    Mate(PoseDef.plane(shelf_i, s_bottom), seat),
                    Mate(PoseDef.plane(shelf_i, s_bottom), lower),
                ],
            )
        fault = caught.exception.fault
        self.assertEqual(fault.variant, "mate_overconstrained")
        self.assertEqual(len(fault.constraints), 2)
        self.assertEqual(fault.free, 1)
        self.assertIn(pncad.OVERCONSTRAINED_RECOURSE, str(fault))
        # The refused edit leaves the document as it was: the shelf
        # standing by the bundle it held.
        self.assertEqual(doc.bundle(shelf_i), before)
        self.assertIsNone(solve_document(doc, resolver=self.ws).fault(shelf_i))

    def test_a_mate_reading_its_own_copy_on_both_sides_refuses(self):
        """A mate equates the copy's pose with a pose of the space it
        joins, so a `to` read off the copy's own output reads the
        placement from its own bundle: a cycle, which the edit door
        refuses where it is authored, and nothing enters the
        document."""
        doc = Doc("self-mate")
        post_i = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
        doc.apply(DocEdit.set_label(post_i, "post"))
        face = self.instance_face(doc, post_i, CapEnd.End)
        before = content_pin(doc)
        with self.assertRaises(pncad.EditError) as caught:
            doc.set_constraints(
                post_i,
                [
                    Mate(
                        PoseDef.plane(post_i, face),
                        PoseDef.flip(PoseDef.plane(post_i, face)),
                    ),
                    *zeros(),
                ],
            )
        refusal = caught.exception
        self.assertEqual(refusal.variant, "would_cycle")
        # The refusal speaks the instance as the document holds it.
        self.assertIn(f'InstantiatePart "post" ({tag(post_i)})', str(refusal))
        self.assertEqual(content_pin(doc), before, "nothing entered")

    def test_a_pose_read_off_a_failed_node_names_its_own_cause(self):
        """The pose's face is read off a copy whose placement cannot be
        solved: its one mate rests the shelf on the floor slid up by
        `1 m / gap`, and `gap` is 0, so the world read it mates to does
        not evaluate. The door admits the mate, which is structural;
        where the post's placement evaluates, it is poisoned through the
        pose it reads, and the refusal names the placement that failed
        and carries its own refusal in its own words."""
        doc = Doc("pncad-placer-refused")
        post_a = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
        shelf_i = self.pinned(doc, self.shelf_ref)
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        doc.apply(DocEdit.declare_var(VarName("gap"), FreeVar.scalar(0.0)))
        lifted = doc.place(
            shelf_i,
            doc.world,
            Placed.unmated([MateValue(Coord.slide("z"), doc.parse_formula("1 m / gap"))]),
        )
        doc.set_constraints(
            post_a,
            [
                Mate(
                    PoseDef.plane(post_a, a_top),
                    PoseDef.flip(PoseDef.plane(lifted, s_bottom)),
                ),
                *zeros(),
            ],
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
        self.assertIn(f'Place "lift" ({tag(lifted)})', str(raised))
        self.assertEqual(raised.kind, "expr")

    def test_a_non_finite_value_refuses_where_it_is_written(self):
        """A value is a number, and a number that is not finite is no
        literal: the value refuses where it is built, before any bundle
        or document holds it."""
        with self.assertRaises(pncad.LiteralError):
            MateValue(Coord.slide("x"), float("inf") * m)

    def test_a_mate_whose_operand_is_not_live_refuses_at_the_door(self):
        """A pose is a read of the body its face is selected in, so a
        body that is not a live node at insert is a typo, refused at the
        edit door as the unresolved read it is."""
        doc, post_i, shelf_i = self.two_instances()
        top = self.instance_face(doc, post_i, CapEnd.End)
        bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        dead = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
        doc.apply(DocEdit.delete_node(dead))
        with self.assertRaises(pncad.EditError) as caught:
            doc.set_constraints(
                post_i,
                [
                    Mate(
                        PoseDef.plane(post_i, top),
                        PoseDef.flip(PoseDef.plane(dead, bottom)),
                    ),
                    *zeros(),
                ],
            )
        self.assertEqual(caught.exception.variant, "unresolved_input")

    def test_the_door_refuses_what_a_mates_structure_says(self):
        """The edit door is structural: it refuses a mate whose poses
        are of two kinds or whose `on` reads another copy, and a value
        on a freedom its mate fixes — each carrying the fault whole, its
        payload on the words a caller branches on."""
        doc, post_i, shelf_i = self.two_instances()
        top = self.instance_face(doc, post_i, CapEnd.End)
        bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        edge = sorted(evaluate(doc, resolver=self.ws).all_edges(shelf_i))[0]

        def refused(mate, *values):
            with self.assertRaises(pncad.EditError) as caught:
                doc.set_constraints(post_i, [mate, *values])
            self.assertEqual(caught.exception.variant, "mate_refused")
            return caught.exception.fault

        kinds = refused(
            Mate(PoseDef.plane(post_i, top), PoseDef.axis(shelf_i, edge))
        )
        self.assertEqual(kinds.variant, "mate_kinds_differ")
        self.assertEqual((kinds.on_kind, kinds.to_kind), ("plane", "axis"))
        self.assertEqual(kinds.mate.placement, post_i)
        elsewhere = refused(
            Mate(PoseDef.plane(shelf_i, bottom), PoseDef.plane(shelf_i, bottom))
        )
        self.assertEqual(elsewhere.variant, "mate_not_on_the_copy")
        seat = Mate(PoseDef.plane(post_i, top), PoseDef.flip(PoseDef.plane(shelf_i, bottom)))
        fixed = refused(seat, *zeros(), MateValue(Coord.slide("z"), 0.1 * m))
        self.assertEqual(fixed.variant, "mate_overconstrained")
        self.assertEqual(fixed.placement, post_i)
        # A number of the other dimension is the binding's to refuse,
        # before any edit.
        with self.assertRaises(ValueError):
            MateValue(Coord.slide("x"), 0.1 * rad)

    def test_a_plane_read_off_an_edge_refuses_where_the_mate_is_added(self):
        """A mate rests two FACES, and a plane is read off a face. Python holds names as opaque text, so a plane
        read off an EDGE's name is built, and the edit door refuses it:
        a selection's kind is the name's own, held where the selection
        is written (`selection_name_kind`), with the side and the
        evaluation's own word in its sentence."""
        doc, post_i, shelf_i = self.two_instances()
        ev = evaluate(doc, resolver=self.ws)
        edge = sorted(ev.all_edges(post_i))[0]
        bottom = one(ev.select(shelf_i, cap_selector(CapEnd.Start, PART_FACE)))
        with self.assertRaises(pncad.EditError) as caught:
            doc.set_constraints(
                post_i,
                [
                    Mate(PoseDef.plane(post_i, edge), PoseDef.flip(PoseDef.plane(shelf_i, bottom))),
                    *zeros(),
                ],
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
        )
        self.assertIsNone(solve_document(other, resolver=self.ws).fault(post_j))

    def test_a_gather_refusal_arrives_under_the_gathers_own_tag(self):
        doc = Doc("no-resolver")
        self.pinned(doc, self.post_ref)
        # Evaluated with no resolver, the instance — a placement — produced
        # no copies, so the GATHER refuses with its own tag, because which
        # invariant broke is what a caller branches on. The checks cannot
        # run over a placement with no value either, and say which.
        assemble_world_copies(doc)
        ev = evaluate(doc)
        with self.assertRaises(pncad.ProductError) as gather:
            product(doc, ev)
        self.assertEqual(gather.exception.variant, "read_failed")
        self.assertIsNotNone(gather.exception.node)
        with self.assertRaises(pncad.ChecksError) as checks:
            run_checks(doc, ev)
        self.assertEqual(checks.exception.variant, "placement_without_value")
        self.assertEqual(checks.exception.node, gather.exception.node)

    def test_a_gather_refusal_speaks_its_piece_from_the_evaluated_document(self):
        doc = Doc("no-resolver-labelled")
        lonely = self.pinned(doc, self.post_ref)
        doc.apply(DocEdit.set_label(lonely, "lonely post"))
        doc.set_product(lonely)
        ev = evaluate(doc)
        # The placement failed, and the message says it as the
        # evaluation's document holds it, label and all; said by tag it
        # would read `node <tag>`. The payload keeps the full id.
        for door in (product, pncad.product_named):
            with self.assertRaises(pncad.ProductError) as gather:
                door(doc, ev)
            self.assertEqual(gather.exception.variant, "read_failed")
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
    be `None` rather than raise."""

    def two_instances(self):
        """A post standing in the world by the identity bundle, which a
        bundle written whole places, and a shelf pinned to the world."""
        doc = Doc(random_document_id() and "payload")
        post_i = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
        shelf_i = self.pinned(doc, self.shelf_ref)
        return doc, post_i, shelf_i

    def test_an_overconstrained_bundle_names_its_placement_and_what_it_leaves(self):
        """A mate the rest of its bundle already fixes is refused where
        it is authored: two plane mates fix a normal twice. The fault
        names the placement and its constraints and how many freedoms
        the bundle would leave, and no single mate."""
        doc = Doc("payload-over")
        post = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
        floor = self.pinned(doc, self.post_ref)

        def facing(node, normal):
            return PoseDef.plane(node, bench_scene.face_facing(self.ws, doc, node, normal))

        with self.assertRaises(pncad.EditError) as caught:
            doc.set_constraints(
                post,
                [
                    Mate(facing(post, (0.0, 0.0, -1.0)), PoseDef.flip(facing(floor, (0.0, 0.0, 1.0)))),
                    Mate(facing(post, (0.0, 0.0, 1.0)), facing(floor, (0.0, 0.0, 1.0))),
                ],
            )
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


class TestPinUpdateDoor(BenchWorkspace):
    """Moving a pin at its sites — and what each door reads, and
    when."""

    def layout(self, shelf_ref=None):
        doc = Doc("update-layout")
        post_i = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
        shelf_i = doc.insert(Node.instantiate_part(shelf_ref or self.shelf_ref, doc.world))
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
            Node.instantiate_part(DocRef(self.shelf.id, content_pin(thicker)), doc.world)
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
        assemble_world_copies(doc)
        return doc, post_i, shelf_i

    def volume(self, doc):
        return pinned(product(doc, evaluate(doc, resolver=self.ws))).volume

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
        # The part's product is the `⊔` of its six copies, a node of the
        # part's own that no cut node maps to, last.
        (*carried, assembled) = outcome.part.order()
        self.assertEqual(outcome.part.node_kind(assembled), "assemble")
        self.assertEqual([b for _, b in outcome.node_map], carried)

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
        ]))
        body = doc.insert(Node.extrude(profile, Formula.length_in(1, m)))
        with self.assertRaises(pncad.InlineError) as caught:
            pncad.inline(doc, body, self.ws)
        self.assertEqual(caught.exception.variant, "not_an_instance")
        self.assertEqual(caught.exception.node, body)

    def test_inline_refuses_a_part_holding_a_named_pose_nothing_reads(self):
        # A named pose outlives its reader, and a pose crosses only with
        # one, so inline refuses rather than drop it.
        part = Doc("unread-pose-part")
        profile = part.insert(Node.polygon([
            (Formula.length_in(0, m), Formula.length_in(0, m)),
            (Formula.length_in(1, m), Formula.length_in(0, m)),
            (Formula.length_in(1, m), Formula.length_in(1, m)),
        ]))
        block = part.insert(Node.extrude(profile, Formula.length_in(1, m)))
        placed(part, block, Frame.translation((0 * m, 0 * m, 0 * m)))
        assemble_world_copies(part)
        cut = part.insert(Node.split(block, plane_on(part, block, (0.0, 0.0, 0.5), (0.0, 0.0, 1.0))))
        pose = part.slot(cut, "tool")
        part.apply(DocEdit.rename_var(pose, VarName("cut_plane")))
        part.apply(DocEdit.delete_node(cut))
        self.ws.create(part)
        host = Doc("unread-pose-host")
        instance = host.insert(Node.instantiate_part(DocRef(part.id, content_pin(part)), host.world))
        with self.assertRaises(pncad.InlineError) as caught:
            pncad.inline(host, instance, self.ws)
        self.assertEqual(caught.exception.variant, "unread_pose")
        self.assertIn("cut_plane", str(caught.exception))

    def test_split_refuses_a_definition_tied_to_both_sides_naming_each(self):
        # `k = w + j` is read by nothing; `w` only by the cut block and
        # `j` by the kept one, so `k` could go with neither document.
        doc = Doc("straddle")
        for name, value in (("w", 1.5), ("j", 0.5)):
            doc.apply(DocEdit.declare_var(VarName(name), FreeVar.length(value * m)))
        doc.apply(DocEdit.declare_var(VarName("k"), doc.parse_formula("w + j")))

        def block(x0, depth):
            profile = doc.insert(Node.polygon([
                (Formula.length_in(x0, m), Formula.length_in(0, m)),
                (Formula.length_in(x0 + 1, m), Formula.length_in(0, m)),
                (Formula.length_in(x0 + 1, m), Formula.length_in(1, m)),
            ]))
            body = doc.insert(Node.extrude(profile, Formula.length_in(1, m)))
            doc.apply(DocEdit.set_param(body, "distance", doc.parse_formula(depth)))
            return [profile, body]

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
    """`place`, `set_product` and `product`: a document's product is the
    body its product slot reads, and nothing sets it but `set_product`
    — no placement joins it by being placed."""

    def volume(self, doc):
        return pinned(product(doc, evaluate(doc, resolver=self.ws))).volume

    def block(self, doc, x0):
        profile = doc.insert(
            Node.polygon(
                [
                    (Formula.length_in(x0, m), Formula.length_in(0, m)),
                    (Formula.length_in(x0 + 1, m), Formula.length_in(0, m)),
                    (Formula.length_in(x0 + 1, m), Formula.length_in(1, m)),
                    (Formula.length_in(x0, m), Formula.length_in(1, m)),
                ],
            )
        )
        return doc.insert(Node.extrude(profile, Formula.length_in(1, m)))

    def test_the_slot_says_what_the_product_is_and_in_what_order(self):
        doc = Doc("world")
        a = self.block(doc, 0.0)
        b = self.block(doc, 3.0)
        # Nothing is placed by an insert, and nothing is the product
        # until the slot reads it.
        self.assertEqual(doc.placements(), [])
        self.assertIsNone(doc.product)
        with self.assertRaises(pncad.ProductError) as empty:
            product(doc, evaluate(doc))
        self.assertEqual(empty.exception.variant, "empty_product")
        self.assertIsNone(empty.exception.node)
        # Placed, the copies are still no product: the slot reads the
        # `⊔` of them, in its members' order.
        on_b = doc.place(b, doc.world)
        on_a = doc.place(a, doc.world)
        self.assertEqual(doc.placements(), [on_b, on_a])
        self.assertEqual(doc.node_kind(on_b), "place")
        with self.assertRaises(pncad.ProductError):
            product(doc, evaluate(doc))
        both = doc.assemble([on_b, on_a])
        self.assertEqual(doc.node_kind(both), "assemble")
        doc.set_product(both)
        self.assertEqual(doc.product, doc.output(both))
        self.assertEqual(self.volume(doc), 2.0)

    def test_an_instance_is_a_placement(self):
        """An instance IS a placement: its part's product, moved by its
        bundle. Placed into the world, the slot can read it; placed
        into a space of its own, it cannot."""
        doc = Doc("world-instances")
        post_i = self.pinned(doc, self.post_ref)
        aside = doc.new_space()
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, aside))
        self.assertEqual(doc.placements(), [post_i, shelf_i])
        doc.set_product(post_i)
        self.assertAlmostEqual(self.volume(doc), POST_VOLUME, delta=1e-12)
        self.assertEqual(doc.space(shelf_i), doc.output(aside))
        self.assertNotEqual(doc.space(post_i), doc.space(shelf_i))
        with self.assertRaises(pncad.EditError) as caught:
            doc.set_product(shelf_i)
        self.assertEqual(caught.exception.variant, "product_read")
        self.assertEqual(doc.product, doc.output(post_i), "the refused edit changed nothing")

    def test_a_product_whose_body_is_deleted_strands_and_refuses_naming_it(self):
        doc = Doc("world-stranded")
        body = self.block(doc, 0.0)
        placed = doc.place(body, doc.world)
        doc.set_product(placed)
        # The delete is accepted: the placement and the slot are
        # readers left reading nothing, never silently re-pointed or
        # dropped.
        doc.apply(DocEdit.delete_node(placed))
        self.assertEqual(doc.placements(), [])
        with self.assertRaises(pncad.ProductError) as caught:
            product(doc, evaluate(doc))
        self.assertEqual(caught.exception.variant, "stranded_product")
        self.assertEqual(caught.exception.read, doc.product)
        self.assertIsNone(caught.exception.node)

    def test_a_boolean_inserted_leaves_the_product_as_it_was(self):
        """Nothing sets the product as a side effect: a cut of two
        placed bodies is a new body the slot does not read, so the
        product is still the two copies until the slot is set again."""
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
                )
            )
            return doc.insert(Node.extrude(profile, Formula.length_in(1, m)))

        a, b = block(0.0), block(3.0)
        copies = [doc.place(a, doc.world), doc.place(b, doc.world)]
        doc.set_product(doc.assemble(copies))
        # The cut reads `b` in `a`'s coordinates, through a copy placed
        # against `a` where it was drawn.
        tool = against(doc, a, b, Frame.translation((0 * m, 0 * m, 0 * m)))
        cut = doc.insert(Node.subtract(a, tool))
        self.assertEqual(self.volume(doc), 2.0, "the product is still a' ⊔ b'")
        # The slot reading a `⊔` with the cut's copy beside the two is
        # what puts it in the product.
        doc.set_product(doc.assemble([*copies, doc.place(cut, doc.world)]))
        self.assertEqual(self.volume(doc), 3.0)

    def test_two_placements_of_one_body_are_two_copies(self):
        doc = Doc("world-two-copies")
        body = self.block(doc, 0.0)
        first = doc.place(body, doc.world)
        second = placed(doc, body, Frame.translation((2 * m, 0 * m, 0 * m)))
        self.assertEqual(len(doc.placements()), 2)
        doc.set_product(doc.assemble([first, second]))
        self.assertEqual(self.volume(doc), 2.0)

    def test_a_document_with_no_product_refuses_the_gather(self):
        doc = Doc("sketch-only")
        doc.insert(Node.polygon([
            (Formula.length_in(0, m), Formula.length_in(0, m)),
            (Formula.length_in(1, m), Formula.length_in(0, m)),
            (Formula.length_in(1, m), Formula.length_in(1, m)),
        ]))
        with self.assertRaises(pncad.ProductError) as caught:
            product(doc, evaluate(doc))
        self.assertEqual(caught.exception.variant, "empty_product")


class TestNestedAcrossTheSeam(BenchWorkspace):
    """An inner document's seats, at the OUTER document's census.

    The census over the inner stand records its seat as a row and a
    record, and the record crosses the instantiation seam with the
    stand's geometry, so the outer census confirms the seat inside each
    copy rather than reading it as a touch no record backs."""

    def stand_doc(self, seed):
        """The bench stand as its OWN document, so it can be
        instantiated: the shelf resting on the post's top where `SEAT_A`
        meets the top's centre."""
        doc = Doc(seed)
        post_a = self.pinned(
            doc, self.post_ref, Frame.translation((0 * m, bench_scene.ROOT_OFFSET_Y * m, 0 * m))
        )
        # The shelf sits where its mate puts it: its bundle is the mate
        # and its values, written whole.
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, doc.world))
        a_top = self.instance_face(doc, post_a, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        slide = bench_scene.shelf_on_post(SEAT_A)
        doc.set_constraints(
            shelf_i,
            [
                Mate(
                    PoseDef.plane(shelf_i, s_bottom),
                    PoseDef.flip(PoseDef.plane(post_a, a_top)),
                ),
                MateValue(Coord.slide("x"), slide[0] * m),
                MateValue(Coord.slide("y"), slide[1] * m),
                MateValue(Coord.spin(), 0),
            ],
        )
        assemble_world_copies(doc)
        self.ws.create(doc)
        return DocRef(doc.id, content_pin(doc))

    def test_two_nested_stands_assemble_in_the_outer_document(self):
        ref = self.stand_doc("nested-stands")
        outer = Doc("nested-stands-outer")
        self.pinned(outer, ref)
        self.pinned(outer, ref, Frame.translation((5 * m, 0 * m, 0 * m)))
        assemble_world_copies(outer)
        ev = evaluate(outer, resolver=self.ws)
        self.assertEqual(at_rest(outer, ev), [])
        _, names = pncad.product_named(outer, ev)
        self.assertGreater(len(names), 0)


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
        # The shelf's bundle, written whole, is the mate that places it
        # on the post pinned to the world, and its values.
        shelf_i = doc.insert(Node.instantiate_part(self.shelf_ref, doc.world))
        if a_top is None:
            a_top = self.instance_face(doc, post_i, CapEnd.End)
        s_bottom = self.instance_face(doc, shelf_i, CapEnd.Start)
        mate, *slides, _spin = doc.set_constraints(
            shelf_i,
            [
                Mate(
                    PoseDef.plane(shelf_i, s_bottom),
                    PoseDef.flip(PoseDef.plane(post_i, a_top)),
                ),
                *(
                    MateValue(Coord.slide(axis), by if isinstance(by, Formula) else by * m)
                    for axis, by in zip(("x", "y"), at, strict=True)
                ),
                MateValue(Coord.spin(), 0),
            ],
        )
        return doc, post_i, shelf_i, mate, slides

    def test_the_mate_follows_the_edited_face(self):
        """The post stands on the floor by its base cap, so growing the
        part grows it up; the shelf rests on its top cap, read at every
        evaluation."""

        def base_pinned(doc):
            post_i = doc.insert(Node.instantiate_part(self.post_ref, doc.world))
            bench_scene.on_floor(doc, post_i)
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
            doc.apply(edit)
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
        )
        later = solve_document(doc, resolver=self.ws).placement(doc, shelf_i)
        self.assertAlmostEqual(later.origin[0].meters - at.origin[0].meters, 0.2, places=12)

    def test_a_value_reads_back_as_written_and_compares_bit_exact(self):
        doc, _, shelf_i, _, (x, _) = self.seated("face-offset-read", at=(0.1, 0.0))
        held = dict(doc.bundle(shelf_i))[x]
        self.assertIsInstance(held, MateValue)
        self.assertEqual(held.coord, Coord.slide("x"))
        plus = MateValue(Coord.slide("x"), 0.1 * m)
        self.assertEqual(plus, MateValue(Coord.slide("x"), 0.1 * m))
        self.assertNotEqual(plus, MateValue(Coord.slide("x"), 0.2 * m))
        self.assertNotEqual(plus, MateValue(Coord.slide("y"), 0.1 * m))

    def test_a_vanished_face_refuses_where_the_placement_evaluates(self):
        # The post's own cap, as the instance's copy carries it verbatim,
        # re-headed at a node the post does not have: the shape of a
        # name whose face an edit removed.
        import json

        probe = Doc("from-face-vanished-probe")
        probe_i = self.pinned(probe, self.post_ref)
        spelled = json.loads(self.instance_face(probe, probe_i, CapEnd.End))
        spelled["node"] = "0:0000000000000063"
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
