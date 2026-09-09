"""Evaluating an assembly from Python: the resolver seam and the memo.

`evaluate(doc, resolver=store)` carries the document seam, so an
assembly document that carries instantiate nodes evaluates, its parts
resolved through a `Workspace` on disk. That is what this file proves,
and it proves it over the tour's own bench scene.

WHERE THE SCENE COMES FROM
--------------------------
`bench_scene.py` is the scene, authored from Python: the `post` and
`shelf` parts, the flat-pack `layout` (one post instance patterned
along +y plus the shelf) and the mated `stand` (two posts, a shelf, two
mates). `opened()` writes all four into a temp `Workspace` and then
RESOLVES each one back out of it, so every document under test below
arrived through the persistence door — the load path is exercised on
every call, over documents nothing had to keep on disk between runs.

`test_assembly_author.py` authors the same scene from the same module.
The two files share one definition of the constants and one definition
of each document, so neither can drift from the other.

WHAT THIS FILE PROVES, AND WHAT IT DOES NOT
-------------------------------------------
It proves the seam and the memo: the refusal family a reference can
raise, the sharing `part_evaluations` counts, the memo's `prior=`
counters and the contract that a memo hit never asks the resolver. It
does NOT prove that the bindings can author an assembly — that is
`test_assembly_author.py`'s subject, and every document here is
authored through the same public doors it uses.

WHAT KEEPS THE SCENE HONEST, AND WHAT DOES NOT
----------------------------------------------
`TestTheSceneIsTheToursOwn` is the guard. The tour's bench
(`demos/tour/src/assembly.rs`) is a detached workspace no test here can
call, so the guard READS ITS SOURCE and compares four things against
`bench_scene`:

* the six base constants, by value;
* the three derived seats, by FORMULA — the expression is parsed out of
  the tour and computed, so a changed formula reds even though the five
  bases it is built from did not move;
* the flat-pack's placement literals — the post's rotation axis, its
  angle, its offset, the pattern's count and spacing, the shelf's
  offset — each by formula;
* the stand's gauge offset and the seats its two mates are authored
  against, in document order.

Four things the guard does NOT see, named rather than summarised:

1. STRUCTURE. It reads named constants and named call sites, not the
   recipe: a third mate, a fourth instance, a different node order in
   either document is invisible to it. `test_the_scene_evaluates_to_
   the_material_the_tour_asserts` and the placement row below pin the
   consequences a body can show; a change that moves neither is not
   caught here.
2. The ONE DELIBERATE difference between this scene and the tour's,
   which is `bench_scene`'s own subject: the tour's prisms are
   parametric where Python's are drawn from literals. The guard would
   red if it compared recipes, and it does not compare recipes. (The
   flat-pack's placed family used to be a second such difference; it
   is `Node.pattern` on both sides now.)
3. Anything in `assembly.rs` outside its constant block, `layout_doc`
   and `stand_doc` — the tour's own assertions above all.
4. A rename or a reformat in the tour, which reds this guard as a false
   alarm rather than as a drift. It reads source text; that is the
   price of reaching into a detached workspace at all.

THE TOLERANCE THE SCENE IS AUTHORED AT
--------------------------------------
The documents are written by this process, so they record the ambient
ε this process runs at and load back at it. One process has one ε, so
there is nothing here to conflict with — which is the difference a
scene authored per run makes over bytes carrying the ε they were
generated under.
"""

import ast
import atexit
import math
import operator
import os
import re
import shutil
import tempfile
import unittest
from pathlib import Path

import bench_scene
import pncad
from bench_scene import (
    FLAT_PACK_GAP,
    FLAT_PACK_SHELF_Y,
    GAUGE_OFFSET_Y,
    PATTERN_COUNT,
    PATTERN_SPACING,
    POST_HEIGHT,
    POST_SECTION,
    POST_SEAT,
    POST_VOLUME,
    SEAT_A,
    SEAT_B,
    SHELF_DEPTH,
    SHELF_LENGTH,
    SHELF_THICKNESS,
    SHELF_VOLUME,
    STAND_SEATS,
)
from pncad import CapEnd, DocEdit, DocRef, Expr, Node, SegTag, Workspace, WrittenLength, evaluate, m, mm

TOUR = Path(__file__).resolve().parents[3] / "demos" / "tour" / "src" / "assembly.rs"

_SCENE = None


def _authored():
    """The scene's directory and its label -> identity map, authored
    ONCE per process.

    Authoring is not free and nothing below mutates this store: the
    rows that move a document copy it first (`CorpusCase.scratch`), so
    one write serves the file.
    """
    global _SCENE
    if _SCENE is None:
        root = Path(tempfile.mkdtemp(prefix="pncad-bench-"))
        atexit.register(shutil.rmtree, root, True)
        directory = root / "bench"
        directory.mkdir()
        docs = bench_scene.write(Workspace(str(directory)))
        _SCENE = (directory, {label: doc.id for label, doc in docs.items()})
    return _SCENE


def written():
    """The directory the scene was written into."""
    return _authored()[0]


def identities():
    """Label -> document identity, for the four documents the scene
    authors. A store names its files by identity, so this is what a
    consumer needs to ask the scan for one of them by name."""
    return dict(_authored()[1])


def opened(directory=None):
    """The store, and the label -> `Doc` map its CURRENT content gives.

    Resolution goes through a reference built from `current_pin`, which
    is the honest spelling for "whatever version this store holds": the
    assemblies' own references carry the pins they were authored
    against, and those are what the evaluation checks.
    """
    store = Workspace(str(directory or written()))
    docs = {
        label: store.resolve(DocRef(ident, store.current_pin(ident)))
        for label, ident in identities().items()
    }
    return store, docs


def volumes(evaluation, node):
    """Every body volume the node's value denotes, in canonical units."""
    return [body.mass_properties().volume for body in evaluation.value(node).bodies()]


def poisoned(evaluation):
    """How many nodes never ran because an ancestor failed — the
    difference between `reused + recomputed` and the live node count."""
    return sum(1 for r in failures(evaluation).values() if r.reason == "poisoned")


def failures(evaluation):
    """Node → the typed refusal it raises, for every node without a
    value. The refusal is the exception itself: `kind` is the stable
    tag, `reason` says whether the node failed or was poisoned."""
    out = {}
    for node in evaluation.order():
        if not evaluation.succeeded(node):
            try:
                evaluation.value(node)
            except pncad.EvaluationError as refusal:
                out[node] = refusal
    return out


class CorpusCase(unittest.TestCase):
    """The shared spellings: a scratch copy of the store for the tests
    that MOVE it, and the volume comparison the tour itself uses.

    The process's own store is shared evidence; a test that resaved a
    part into it would leave the next test a store whose assemblies pin
    a version nobody wrote.
    """

    #: The tour's own agreement bound on this model (`assembly.rs`
    #: asserts its layout volume to `1e-12`). A body's volume is
    #: summed over its faces, so the last bits of `0.12 * 0.12 * 0.5`
    #: are a function of the summation order and not of the recipe —
    #: bit-exactness is D9's claim about TWO RUNS OF THE SAME
    #: computation, which this is not.
    DELTA = 1e-12

    def assertVolumes(self, found, want):
        """Every volume, in order, to the scene's own agreement bound."""
        self.assertEqual(len(found), len(want), f"{found} vs {want}")
        for index, expected in enumerate(want):
            self.assertAlmostEqual(found[index], expected, delta=self.DELTA)

    def scratch(self):
        directory = Path(tempfile.mkdtemp()) / "bench"
        shutil.copytree(written(), directory)
        self.addCleanup(shutil.rmtree, directory.parent, ignore_errors=True)
        return directory


class TestTheSeamIsCrossedOrRefused(CorpusCase):
    """`resolver=` is the whole difference between an assembly that
    evaluates and one that refuses — and the refusal was always the
    honest one, never an empty part."""

    def test_without_a_resolver_every_instance_refuses_typed(self):
        _, docs = opened()
        refusals = failures(evaluate(docs["layout"]))
        self.assertEqual(len(refusals), 3, "no node of the layout survives")
        for node, refusal in refusals.items():
            with self.subTest(node=node):
                self.assertEqual(refusal.kind, "part_no_resolver")
                self.assertIn(refusal.reason, ("node_failed", "poisoned"))
                self.assertIn("no part resolver", str(refusal))

    def test_the_layout_evaluates_through_the_store(self):
        """The tour's flat-pack oracle, reproduced: one post instance
        placed PATTERN_COUNT ways, plus the shelf.

        The family is a `Node.pattern` — the tour's own node — so its
        value is PLURAL: one body per placement, each a whole post,
        nothing fused. That is what the count below reads, and it is
        the shape every row further down has to answer over.
        """
        store, docs = opened()
        evaluation = evaluate(docs["layout"], resolver=store)
        self.assertEqual(failures(evaluation), {})
        instance, family, shelf = evaluation.order()
        self.assertVolumes(volumes(evaluation, instance), [POST_VOLUME])
        self.assertVolumes(
            volumes(evaluation, family), [POST_VOLUME] * PATTERN_COUNT
        )
        self.assertVolumes(volumes(evaluation, shelf), [SHELF_VOLUME])

    def test_the_stand_evaluates_through_the_store(self):
        """The mated bench: two posts and a shelf. The mate nodes carry
        the solve's declarations rather than a body, so they denote no
        volume — which is why the material is three solids, not five."""
        store, docs = opened()
        evaluation = evaluate(docs["stand"], resolver=store)
        self.assertEqual(failures(evaluation), {})
        material = [v for node in evaluation.order() for v in volumes(evaluation, node)]
        self.assertVolumes(sorted(material), sorted([POST_VOLUME] * 2 + [SHELF_VOLUME]))

    def test_one_part_document_is_evaluated_once_however_many_instances(self):
        """`part_evaluations` is the seam's sharing evidence: the
        layout instantiates two documents and patterns one of them
        PATTERN_COUNT ways, and crosses the seam exactly twice."""
        store, docs = opened()
        self.assertEqual(evaluate(docs["layout"], resolver=store).part_evaluations, 2)
        self.assertEqual(evaluate(docs["stand"], resolver=store).part_evaluations, 2)
        self.assertEqual(
            evaluate(docs["layout"]).part_evaluations, 0, "nothing crosses"
        )

    def test_a_part_document_alone_needs_no_resolver(self):
        """A resolver is what a REFERENCE needs. A document with none
        evaluates identically with and without one, which is what makes
        `None` the right default rather than a limitation."""
        store, docs = opened()
        with_store = evaluate(docs["post"], resolver=store)
        without = evaluate(docs["post"])
        self.assertEqual(with_store.order(), without.order())
        for node in with_store.order():
            with self.subTest(node=node):
                self.assertEqual(volumes(with_store, node), volumes(without, node))


class TestTheResolutionRefusals(CorpusCase):
    """The seam's refusal family, each reached THROUGH `resolver=`.

    THREE arms are exercised here — `part_no_resolver` (the class
    above), `part_pin_mismatch` and `part_unresolved`. The rest of the
    family is typed and tagged but UNREACHED from Python today, each
    for its own reason, and none of them is singled out:
    `part_epsilon_seam` needs a stored document recording a different
    ε; `part_root_failed` and `part_product` need a part whose own
    product is broken; `part_reference_cycle` needs an instantiate node
    pointing back up its own chain — and an honest store cannot hold
    one at all, since a cycle with valid pins wants a content hash
    containing its own hash, and with invalid pins `part_pin_mismatch`
    fires first, so hand-crafted bytes do not get there either.
    `part_depth_exceeded` is left UNCLAIMED: a hand-crafted acyclic
    chain deep enough might reach it, and this unit did not establish
    whether it does. Authoring any of these documents is G18b's half.
    """

    def test_a_pin_that_moved_refuses_rather_than_retargeting(self):
        """The resave goes through the SAME store the evaluation then
        resolves against — no second `Workspace` is built, because the
        scan is not frozen at construction and a fresh one here would
        teach that it is."""
        directory = self.scratch()
        store, docs = opened(directory)
        # A part legitimately changes on disk: the shelf is re-authored
        # thicker under the same label, so it keeps its identity and
        # moves its pin — which is the version the assemblies hold.
        store.resave(bench_scene.shelf(thickness=SHELF_THICKNESS * 1.5))

        refusals = failures(evaluate(docs["layout"], resolver=store))
        self.assertEqual(
            [r.kind for r in refusals.values()],
            ["part_pin_mismatch"],
            "only the shelf instance refuses; the posts still resolve",
        )
        refusal = next(iter(refusals.values()))
        self.assertIn(pncad.PIN_MISMATCH_RECOURSE, str(refusal))

    def test_a_document_the_store_does_not_hold_refuses_naming_the_reference(self):
        directory = self.scratch()
        os.remove(directory / f"{identities()['post']}.pncad")

        _, docs = opened()
        refusals = failures(
            evaluate(docs["layout"], resolver=Workspace(str(directory)))
        )
        self.assertEqual(
            sorted(r.kind for r in refusals.values()),
            ["part_unresolved", "part_unresolved"],
            "the post instance fails and the pattern over it is poisoned",
        )
        self.assertEqual(
            sorted(r.reason for r in refusals.values()), ["node_failed", "poisoned"]
        )


class TestTheMemoIsObservable(CorpusCase):
    """PYPU's banked finding — "memoized recompute is unobservable from
    Python" — closed at the same signature.

    `reused` and `recomputed` were already bound and could only ever
    read (0, n): with no way to pass a prior, nothing could be reused.
    `prior=` is what makes them a measurement.
    """

    def test_a_prior_reuses_every_node_of_an_unchanged_document(self):
        store, docs = opened()
        first = evaluate(docs["layout"], resolver=store)
        again = evaluate(docs["layout"], resolver=store, prior=first)
        self.assertEqual((first.reused, first.recomputed), (0, 3))
        self.assertEqual((again.reused, again.recomputed), (3, 0))

    def test_the_two_counters_account_for_every_node_that_ran_or_was_reused(self):
        """The invariant is `reused + recomputed == len(order) -
        poisoned`, not `== len(order)`.

        A poisoned node never ran, and the kernel's bookkeeping counts
        it in NEITHER column (a node that ran and FAILED is counted, in
        `recomputed`). The all-success documents below make the two
        forms indistinguishable, which is exactly why the refusal path
        is asserted beside them rather than left to inference — the
        first version of this row tested only the successes and let a
        false docstring through review.
        """
        store, docs = opened()
        for label, doc in docs.items():
            with self.subTest(document=label):
                first = evaluate(doc, resolver=store)
                again = evaluate(doc, resolver=store, prior=first)
                for evaluation in (first, again):
                    self.assertEqual(
                        evaluation.reused + evaluation.recomputed,
                        len(evaluation.order()) - poisoned(evaluation),
                    )
                    self.assertEqual(poisoned(first), 0, "nothing refuses here")

    def test_on_a_refusal_path_the_counters_undershoot_by_the_poisonings(self):
        """The no-resolver layout: two instantiate nodes RUN and fail,
        the pattern over one of them is poisoned and never runs. So the
        sum is 2 against three nodes in `order()` — and the difference
        is exactly the poisoning."""
        _, docs = opened()
        refusing = evaluate(docs["layout"])
        self.assertEqual(poisoned(refusing), 1)
        self.assertEqual(len(refusing.order()), 3)
        self.assertEqual((refusing.reused, refusing.recomputed), (0, 2))
        self.assertEqual(
            refusing.reused + refusing.recomputed,
            len(refusing.order()) - poisoned(refusing),
        )

    def test_a_memo_hit_never_asks_the_seam(self):
        """The counters agree with each other: an instance served from
        the memo does not resolve its reference, so a fully reused run
        crosses the seam zero times — and needs no resolver to do it."""
        store, docs = opened()
        first = evaluate(docs["layout"], resolver=store)
        self.assertEqual(first.part_evaluations, 2)
        again = evaluate(docs["layout"], prior=first)
        self.assertEqual((again.reused, again.recomputed), (3, 0))
        self.assertEqual(again.part_evaluations, 0)

    def test_an_edit_recomputes_only_the_cone_below_it(self):
        """The memo's point, measured on a part document: make the post
        twice as long and the section drawn under it is still reused.

        Putting an expression into an authoring step is a named gap, so
        the length is not a parameter this document holds and the edit
        is the one a Python author actually has — delete the extrude
        and insert a longer one over the same profile. What the memo
        answers to is the same either way: the frame and the profile
        are unchanged, so they are served, and only the node that
        changed runs.
        """
        _, docs = opened()
        post = docs["post"]
        first = evaluate(post)
        frame, profile, extrude = first.order()
        post.apply(DocEdit.delete_node(extrude))
        post.insert(Node.extrude(profile, Expr.written_length(WrittenLength.in_unit(2 * POST_HEIGHT, m))))
        again = evaluate(post, prior=first)
        # TWO reused: the post's sketch frame and the section drawn on
        # it are what the deleted extrude consumed, and neither moved.
        self.assertEqual(again.order()[:2], [frame, profile])
        self.assertEqual((again.reused, again.recomputed), (2, 1))
        self.assertVolumes(volumes(again, again.order()[-1]), [2 * POST_VOLUME])

    def test_a_prior_of_another_document_reuses_nothing_and_is_legal(self):
        """The memo is PER DOCUMENT (DI3): an evaluation carries the
        id of the document it is of, and a prior from elsewhere is
        REFUSED whole — dropped before the first node is looked up,
        never mined for a hit. `evaluate` stays total: the run reuses
        nothing and recomputes everything, with no refusal and no
        wrong answer."""
        store, docs = opened()
        prior = evaluate(docs["post"])
        stand = evaluate(docs["stand"], resolver=store, prior=prior)
        self.assertEqual(stand.reused, 0)
        self.assertEqual(failures(stand), {})

    def test_a_sibling_assembly_over_the_same_parts_still_reuses_nothing(self):
        """The sharp form of the same fact, and the one that shows why
        "a key is content, not position" was the wrong sentence.

        The layout and the stand instantiate the SAME two documents at
        the SAME pins, so their instantiate nodes agree on content, and
        two documents built from one recipe CAN mint the same ids. What
        makes the overlap worth nothing is not luck but DI3's refusal:
        the prior is of another document, so it is dropped whole and
        all five of the stand's nodes recompute.
        """
        store, docs = opened()
        layout = evaluate(docs["layout"], resolver=store)
        stand = evaluate(docs["stand"], resolver=store, prior=layout)
        self.assertEqual(failures(stand), {})
        self.assertEqual(stand.reused, 0, "no cross-document reuse, by construction")
        self.assertEqual(stand.recomputed, len(stand.order()))


class TestTheMemoServesWithoutTheSeamsGates(CorpusCase):
    """**The memo hits before the resolver is consulted.** A reused
    `InstantiatePart` node never asks the store, so the seam's
    AVAILABILITY refusals are raised only for nodes that actually
    re-resolve.

    These rows ASSERT that contract rather than lamenting it, because
    it is now stated at the door (`evaluate`'s `prior=`, in the stub
    and in `py/value.rs`) and the audit page's A4 sentence is qualified
    the same way. What they pin is the shape, so a kernel that ever
    changes it goes red HERE, in a place that names the decision.

    Two framings of the served value, and both are true at once:

    * Against the STORE it is stale — the natural memo workflow (edit
      a part, re-evaluate with the prior) serves the old body.
    * Against the DOCUMENT it is exactly right — the memo serves what
      this document's own `DocRef` pins, certified by content key.
      Nothing is retargeted; what is skipped is the RE-CHECK.

    That is correct BY DESIGN (`docs/DOCM-IDENTITY-DESIGN.md` DI2):
    the memo is a pure function of the document, and for an instantiate
    node the pin IS the content, so the served value is exactly what
    the document pins. Putting store state into memo admission would
    make an evaluation with a prior depend on the filesystem, against
    D9, and cost a seam crossing per reused node. Store freshness is
    the mounting SESSION's, not the memo's — which is why these rows
    pin what they pin, and why weakening them would be a change of
    design and not a change of test. Adopted from the reviewer probe
    branches `lib/g18a-r1b-probes` (two rows, red as written against
    the unstated contract) and `lib/g18a-r2-probes` (`R2P1`).
    """

    def test_a_prior_serves_a_moved_pin_without_refusing(self):
        directory = self.scratch()
        store, docs = opened(directory)
        before = evaluate(docs["layout"], resolver=store)
        self.assertEqual(failures(before), {})
        shelf_node = before.order()[2]
        pinned_body = volumes(before, shelf_node)

        store.resave(bench_scene.shelf(thickness=SHELF_THICKNESS * 1.5))

        # The same call, the same store, differing only in the prior.
        fresh = evaluate(docs["layout"], resolver=store)
        self.assertEqual(
            [r.kind for r in failures(fresh).values()],
            ["part_pin_mismatch"],
            "an evaluation that ASKS still refuses the moved pin",
        )

        memoized = evaluate(docs["layout"], resolver=store, prior=before)
        self.assertEqual(
            failures(memoized), {}, "the memo never asks, so nothing refuses"
        )
        self.assertEqual((memoized.reused, memoized.recomputed), (3, 0))
        self.assertEqual(memoized.part_evaluations, 0, "the seam is not crossed")
        self.assertVolumes(
            volumes(memoized, shelf_node),
            pinned_body,
        )

    def test_a_prior_serves_a_missing_document_without_refusing(self):
        """The same contract on the other availability arm — the class
        is `part_pin_mismatch` AND `part_unresolved`, so fixing or
        pinning only one would be a half-answer."""
        directory = self.scratch()
        store, docs = opened(directory)
        before = evaluate(docs["layout"], resolver=store)

        os.remove(directory / f"{identities()['post']}.pncad")
        gone = Workspace(str(directory))

        self.assertEqual(
            sorted(
                r.kind
                for r in failures(evaluate(docs["layout"], resolver=gone)).values()
            ),
            ["part_unresolved", "part_unresolved"],
            "an evaluation that ASKS refuses the document that is not there",
        )
        memoized = evaluate(docs["layout"], resolver=gone, prior=before)
        self.assertEqual(failures(memoized), {})
        self.assertEqual((memoized.reused, memoized.recomputed), (3, 0))
        self.assertEqual(memoized.part_evaluations, 0)

    def test_a_prior_evaluates_an_assembly_with_no_resolver_at_all(self):
        """The limit case of the same rule, and the one the unit always
        asserted: with every node a memo hit, the seam is not needed."""
        store, docs = opened()
        before = evaluate(docs["layout"], resolver=store)
        after = evaluate(docs["layout"], prior=before)
        self.assertEqual(failures(after), {}, "no part_no_resolver either")
        self.assertEqual((after.reused, after.recomputed), (3, 0))
        self.assertEqual(after.part_evaluations, 0)


class TestTheResolverSnapshot(CorpusCase):
    """WHEN the store is read: at the `evaluate` call, not at
    `Workspace(...)` and not at some earlier freeze.

    `Workspace.resolver()` copies the id -> path scan per call, which
    makes the resolver a snapshot AS OF THE CALL — so a write through
    the same Python object is visible to the next `evaluate`, and no
    caller has to rebuild a store to be seen. Adopted from
    `lib/g18a-r1b-probes`, which asked whether the unit's own
    pin-mismatch row needed the fresh `Workspace` it was building. It
    did not, and it no longer builds one.
    """

    def test_a_resave_through_the_same_object_is_seen_by_a_later_evaluate(self):
        directory = self.scratch()
        store, docs = opened(directory)
        store.resave(bench_scene.shelf(thickness=SHELF_THICKNESS * 1.5))
        self.assertEqual(
            [
                r.kind
                for r in failures(evaluate(docs["layout"], resolver=store)).values()
            ],
            ["part_pin_mismatch"],
            "the store a resave went through sees its own write",
        )

    def test_a_create_before_the_call_is_inside_the_snapshot(self):
        directory = self.scratch()
        os.remove(directory / f"{identities()['post']}.pncad")
        gone = Workspace(str(directory))
        _, whole = opened()

        self.assertEqual(
            sorted(
                r.kind
                for r in failures(evaluate(whole["layout"], resolver=gone)).values()
            ),
            ["part_unresolved", "part_unresolved"],
        )
        gone.create(whole["post"])
        self.assertEqual(
            failures(evaluate(whole["layout"], resolver=gone)),
            {},
            "a create before the call is inside the snapshot",
        )


class TestTheSceneEvaluatesToWhatTheTourAsserts(CorpusCase):
    """The scene's material and its placements, read off the store.

    A VOLUME is invariant under placement, so the rows above could all
    be met by a scene that put the parts anywhere at all. This is the
    cheapest oracle that reads POSITION, through the tessellator the
    binding already exposes.
    """

    def test_the_placed_posts_lie_where_the_scene_places_them(self):
        """It pins three things a volume cannot: the placement's
        ROTATION — the post is on its side, so its long axis is x and
        its square section is y-z — the FLAT_PACK_GAP that stands it
        clear of the bench, and the pattern's extent along +y.

        The whole family's box, through the tessellator the binding
        already exposes. A pattern's value is PLURAL, so the outline is
        taken over every body it denotes rather than over one fused
        one — which is the same box, and the row below is what
        separates the posts inside it.
        """
        store, docs = opened()
        evaluation = evaluate(docs["layout"], resolver=store)
        family = evaluation.order()[1]
        positions = [
            p
            for body in evaluation.value(family).bodies()
            for p in body.tessellate(5 * mm).positions
        ]
        axes = [[p[i].meters for p in positions] for i in range(3)]
        self.assertEqual(
            tuple((round(min(a), 9), round(max(a), 9)) for a in axes),
            (
                (round(FLAT_PACK_GAP, 9), round(FLAT_PACK_GAP + POST_HEIGHT, 9)),
                (0.0, round((PATTERN_COUNT - 1) * PATTERN_SPACING + POST_SECTION, 9)),
                (0.0, round(POST_SECTION, 9)),
            ),
            "the posts lie on their side beside the bench, stepped along +y",
        )

    def test_each_placed_post_answers_at_its_own_step(self):
        """Where each post in the family actually IS, one cap frame per
        placement — the COUNT and the SPACING, which an outline over
        the whole family cannot separate.

        The cap is the post's top in its own coordinates; lying down
        about +y puts it at the -x end, which is why every one of them
        answers at exactly FLAT_PACK_GAP.
        """
        store, docs = opened()
        evaluation = evaluate(docs["layout"], resolver=store)
        family = evaluation.order()[1]
        caps = evaluation.select(
            family,
            bench_scene.cap_selector(CapEnd.End, [SegTag.Instance, SegTag.InPart]),
        )
        read = sorted(
            tuple(round(c.meters, 9) for c in evaluation.face_frame(family, cap).origin)
            for cap in caps
        )
        self.assertEqual(
            read,
            [
                (
                    round(FLAT_PACK_GAP, 9),
                    round(POST_SECTION / 2.0 + index * PATTERN_SPACING, 9),
                    round(POST_SECTION / 2.0, 9),
                )
                for index in range(PATTERN_COUNT)
            ],
        )

    def test_the_store_holds_exactly_the_four_documents_the_scene_authors(self):
        store, _ = opened()
        self.assertEqual(sorted(store.documents()), sorted(identities().values()))
        self.assertEqual(sorted(identities()), ["layout", "post", "shelf", "stand"])


#: The tour's arithmetic vocabulary: the names its constants and its
#: placement literals are built from, with the values `bench_scene`
#: declares. `PI` is `std::f64::consts::PI`, which the flat-pack's
#: rotation is a quarter of.
TOUR_NAMES = {
    "POST_SECTION": POST_SECTION,
    "POST_HEIGHT": POST_HEIGHT,
    "SHELF_LENGTH": SHELF_LENGTH,
    "SHELF_DEPTH": SHELF_DEPTH,
    "SHELF_THICKNESS": SHELF_THICKNESS,
    "FLAT_PACK_GAP": FLAT_PACK_GAP,
    "PI": math.pi,
}

_ARITHMETIC = {
    ast.Add: operator.add,
    ast.Sub: operator.sub,
    ast.Mult: operator.mul,
    ast.Div: operator.truediv,
}


def tour_value(source):
    """What a Rust arithmetic expression over `TOUR_NAMES` is worth.

    The tour spells its derived seats and its placement offsets as
    FORMULAS over the base dimensions, and the formula is what changes
    when a seat moves — so the guard reads the expression and computes
    it rather than comparing a number that would move on both sides at
    once. Rust's arithmetic on `f64` literals and its array literals
    are also Python's, so the expression is parsed rather than
    translated; anything richer than `+ - * /`, a name, a number, a
    parenthesis and a bracketed list refuses instead of guessing.
    """

    def walk(node):
        if isinstance(node, ast.Constant) and isinstance(node.value, (int, float)):
            return float(node.value)
        if isinstance(node, ast.Name):
            return TOUR_NAMES[node.id]
        if isinstance(node, ast.UnaryOp) and isinstance(node.op, ast.USub):
            return -walk(node.operand)
        if isinstance(node, ast.BinOp) and type(node.op) in _ARITHMETIC:
            return _ARITHMETIC[type(node.op)](walk(node.left), walk(node.right))
        if isinstance(node, (ast.List, ast.Tuple)):
            return tuple(walk(element) for element in node.elts)
        raise AssertionError(f"not the tour's arithmetic: {source!r}")

    return walk(ast.parse(source.strip(), mode="eval").body)


class TestTheSceneIsTheToursOwn(unittest.TestCase):
    """`bench_scene` claims to be the tour's bench. This is the guard
    that holds it to that.

    `demos/tour` is a detached workspace — its own `[workspace]` table,
    excluded from the root manifest — so no test here can call its
    authoring functions and compare documents. What it can do is READ
    THE SOURCE, and every number the scene is built from is declared
    there as a constant or written at a named call site. The file
    header says what that reaches and what it does not.
    """

    @classmethod
    def setUpClass(cls):
        cls.source = TOUR.read_text(encoding="utf-8")

    def body_of(self, function):
        """The text of one of the tour's authoring functions."""
        start = self.source.index(f"fn {function}(")
        return self.source[start : self.source.index("\n}\n", start)]

    def declared(self, name, kind):
        """The right-hand side of a `const` the tour declares."""
        found = re.search(rf"^const {name}: {kind} = (.+);$", self.source, re.M)
        self.assertIsNotNone(found, f"{name} is no longer declared in the tour")
        return found.group(1)

    def assertScene(self, found, want, what):
        """A tour reading against the Python scene's own value."""
        self.assertEqual(found, want, f"{what} moved in the tour")

    def test_the_base_dimensions_still_match_the_tour(self):
        for name, value in TOUR_NAMES.items():
            if name == "PI":
                continue
            with self.subTest(constant=name):
                self.assertScene(tour_value(self.declared(name, "f64")), value, name)

    def test_the_derived_seats_still_use_the_tours_formulas(self):
        """The hole a value comparison leaves: `SEAT_A`, `SEAT_B` and
        `POST_SEAT` are COMPUTED from the bases, so a guard that read
        only the bases passed a changed formula. The formula is read
        and evaluated here, against the values `bench_scene` derives
        the same way."""
        for name, value in (
            ("SEAT_A", SEAT_A),
            ("SEAT_B", SEAT_B),
            ("POST_SEAT", POST_SEAT),
        ):
            with self.subTest(constant=name):
                self.assertScene(
                    tour_value(self.declared(name, r"\[f64; 3\]")), value, name
                )

    def test_the_flat_packs_placement_literals_still_match_the_tour(self):
        """The layout's placements are literals at their call sites,
        read by nothing until this row: the post's rotation and offset,
        the pattern's count and spacing, the shelf's offset."""
        layout = self.body_of("layout_doc")
        placed = re.search(
            r"Frame::rotate_then_translate\(\s*(\[[^\]]*\]),\s*([^,]+),\s*(\[[^\]]*\]),",
            layout,
        )
        self.assertIsNotNone(placed, "the post is no longer placed by a rotated frame")
        self.assertScene(tour_value(placed.group(1)), (0.0, 1.0, 0.0), "the post's axis")
        self.assertScene(
            tour_value(placed.group(2)), -math.pi / 2, "the post's rotation"
        )
        self.assertScene(
            tour_value(placed.group(3)),
            (FLAT_PACK_GAP + POST_HEIGHT, 0.0, 0.0),
            "the post's flat-pack offset",
        )

        count = re.search(r'count: pe\("(\d+)", &scope\)', layout)
        self.assertIsNotNone(count, "layout_doc no longer authors its count inline")
        self.assertScene(int(count.group(1)), PATTERN_COUNT, "PATTERN_COUNT")
        spacing = re.search(r'spacing: pe\("(\d+) mm", &scope\)', layout)
        self.assertIsNotNone(spacing, "layout_doc no longer authors its spacing in mm")
        self.assertScene(
            int(spacing.group(1)) / 1000.0, PATTERN_SPACING, "PATTERN_SPACING"
        )

        shelf = re.search(r"Frame::translation\((\[[^\]]*\])\)", layout)
        self.assertIsNotNone(shelf, "the shelf is no longer placed by a translation")
        self.assertScene(
            tour_value(shelf.group(1)),
            (FLAT_PACK_GAP, FLAT_PACK_SHELF_Y, 0.0),
            "the shelf's flat-pack offset",
        )

    def test_the_stands_placement_and_seats_still_match_the_tour(self):
        """The gauge post's inset, and the seat each of the two mates
        is authored against IN DOCUMENT ORDER — a swapped pair of seats
        is a different bench that every volume in this file would
        still accept."""
        stand = self.body_of("stand_doc")
        gauge = re.search(r"Frame::translation\((\[[^\]]*\])\)", stand)
        self.assertIsNotNone(gauge, "the gauge post no longer carries a frame")
        self.assertScene(
            tour_value(gauge.group(1)),
            (0.0, GAUGE_OFFSET_Y, 0.0),
            "the gauge post's offset",
        )
        seats = re.findall(r"^\s+[ab]: mate_frame\((\w+)\),$", stand, re.M)
        self.assertEqual(len(seats), 4, "the stand no longer authors exactly two mates")
        named = {"SEAT_A": SEAT_A, "SEAT_B": SEAT_B, "POST_SEAT": POST_SEAT}
        self.assertScene(
            tuple(named[name] for name in seats),
            tuple(seat for mate in STAND_SEATS for seat in mate),
            "the stand's mate seats",
        )


if __name__ == "__main__":
    unittest.main()
