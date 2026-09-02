"""Evaluating an assembly from Python: the resolver seam and the memo.

The audit's G18 row named a STRUCTURAL first door — "`evaluate(doc)`
takes no resolver, so an `InstantiatePart` node cannot evaluate from
Python at all" — and this file is the positive form of that sentence.
`evaluate(doc, resolver=store)` carries the document seam, so an
assembly document that already CARRIES instantiate nodes evaluates,
its parts resolved through a `Workspace` on disk.

What is still not here, and is G18b: Python cannot AUTHOR an
instantiate node, a mate, or a placement. So the documents under test
arrive through the persistence door, exactly as `plate_param` does for
the parametric flagship — which is why the corpus is the tour's own.

THE CORPUS AND ITS PROVENANCE
-----------------------------
`corpus/bench/` is the tour's assembly scene, written by that scene's
own authoring functions (`demos/tour/src/assembly.rs`), so the oracle
below is the oracle the Rust side already asserts on this model and
not a second one invented here. Four documents: the `post` and `shelf`
parts, the flat-pack `layout` (one post instance patterned two ways
plus the shelf) and the mated `stand` (two posts, a shelf, two mates).
A store names its files by identity, so `MANIFEST` carries the label
each identity was derived from.

Regenerate it with the tour's own door:

    cd demos/tour && cargo run -- asm-corpus \\
        ../../crates/pncad-py/tests/corpus/bench

WHAT KEEPS THE CORPUS HONEST, AND WHAT DOES NOT
------------------------------------------------
`test_the_corpus_still_matches_the_scene_it_came_from` reads the five
BASE dimension constants out of `assembly.rs` and checks them, and
`test_the_patterned_posts_sit_where_the_scene_places_them` pins where
the layout actually puts its two posts. Between them a numeric or
placement drift in the tour goes red here. Three things they do NOT
catch, named rather than summarised, because the first draft of this
header disclosed only the first:

1. STRUCTURE with the constants left alone — a fifth patterned post, a
   third mate, a different node order. The corpus would stay green
   while no longer being the tour's scene.
2. DERIVED constants. `SEAT_A`, `SEAT_B` and `POST_SEAT` are computed
   FROM the five (`[POST_SECTION / 2.0, SHELF_DEPTH / 2.0, 0.0]` and
   friends), and the guard reads only the five bases — so a changed
   FORMULA passes it. The placement row above is what would catch the
   consequence for the layout; the stand's two seats have no such row,
   because a mated placement is not readable from Python (no `roots`
   door, `gap: G18 explicit product roots`).
3. That a VOLUME is invariant under placement. That was the whole of
   this file's original oracle set, which is why item 2 went unnoticed
   until review: every committed number could be reproduced by a scene
   that put the parts anywhere at all.

Closing (1) properly needs the tour's authoring functions callable
from a code-tier test, and `demos/tour` is a detached workspace — so
it is scheduled rather than banked: **issue #1186**.

THE TOLERANCE THE CORPUS RECORDS
--------------------------------
All four documents record `epsilon: 1e-9`, the default, because that
is the ambient ε they were generated under; the CI python-suite job
runs at the default ε and sets no `CAD_TOLERANCE_EPS`. One process has
one ε, so loading a document that records a different one refuses
(`PersistError` / `ToleranceConflict` at the door, `part_epsilon_seam`
across the seam). If this file is ever run under a swept ε it will
refuse at `Workspace.resolve`, and that is the reason — not a mystery,
and not a defect in the corpus. Regenerate under the ε you mean to
test at.

"""

import os
import re
import shutil
import tempfile
import unittest
from pathlib import Path

import pncad
from pncad import (
    DocEdit,
    DocParamValue,
    DocRef,
    ParamName,
    Workspace,
    evaluate,
    m,
    mm,
)

CORPUS = Path(__file__).resolve().parent / "corpus" / "bench"
SCENE = Path(__file__).resolve().parents[3] / "demos" / "tour" / "src" / "assembly.rs"

# The scene's dimensions, in metres — the same numbers `assembly.rs`
# declares, restated here and CHECKED against it below.
POST_SECTION = 0.12
POST_HEIGHT = 0.5
SHELF_LENGTH = 0.9
SHELF_DEPTH = 0.30
SHELF_THICKNESS = 0.04

POST_VOLUME = POST_SECTION * POST_SECTION * POST_HEIGHT
SHELF_VOLUME = SHELF_LENGTH * SHELF_DEPTH * SHELF_THICKNESS

# The layout's declared PLACEMENTS, which no volume can see: the post
# is laid on its side (rotated -pi/2 about +y, so POST_HEIGHT runs
# along x and POST_SECTION along y and z), set FLAT_PACK_GAP along +x
# so the flat-pack sits beside the assembled bench, and patterned two
# times along +y at this spacing.
FLAT_PACK_GAP = 1.4
PATTERN_SPACING = 0.2
PATTERN_COUNT = 2


def manifest():
    """Label → document identity, as the corpus generator wrote it."""
    text = (CORPUS / "MANIFEST").read_text(encoding="utf-8")
    return dict(line.split() for line in text.splitlines() if line.strip())


def opened(directory=CORPUS):
    """The store, and the label → `Doc` map its CURRENT content gives.

    Resolution goes through a reference built from `current_pin`, which
    is the honest spelling for "whatever version this store holds": the
    assemblies' own references carry the pins they were authored
    against, and those are what the evaluation checks.
    """
    store = Workspace(str(directory))
    names = manifest()
    docs = {
        label: store.resolve(DocRef(ident, store.current_pin(ident)))
        for label, ident in names.items()
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

    The committed corpus is read-only evidence; a test that resaved a
    part into it would leave the next test a store whose assemblies
    pin a version nobody wrote.
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
        shutil.copytree(CORPUS, directory)
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
        patterned PATTERN_COUNT ways, plus the shelf."""
        store, docs = opened()
        evaluation = evaluate(docs["layout"], resolver=store)
        self.assertEqual(failures(evaluation), {})
        instance, pattern, shelf = evaluation.order()
        self.assertVolumes(volumes(evaluation, instance), [POST_VOLUME])
        self.assertVolumes(volumes(evaluation, pattern), [POST_VOLUME] * PATTERN_COUNT)
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
        layout instantiates two documents and patterns one of them four
        ways, and crosses the seam exactly twice."""
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
        docs["shelf"].apply(
            DocEdit.set_doc_param_value(
                ParamName("thickness"), DocParamValue.length(SHELF_THICKNESS * 1.5 * m)
            )
        )
        store.resave(docs["shelf"])

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
        names = manifest()
        os.remove(directory / f"{names['post']}.pncad")

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
        """The memo's point, measured on a part document: change the
        one parameter the extrude consumes and the profile above it is
        still reused."""
        _, docs = opened()
        post = docs["post"]
        first = evaluate(post)
        post.apply(
            DocEdit.set_doc_param_value(
                ParamName("height"), DocParamValue.length(2 * POST_HEIGHT * m)
            )
        )
        again = evaluate(post, prior=first)
        self.assertEqual((again.reused, again.recomputed), (1, 1))
        self.assertVolumes(volumes(again, again.order()[-1]), [2 * POST_VOLUME])

    def test_a_prior_of_another_document_reuses_nothing_and_is_legal(self):
        """The memo is PER DOCUMENT: the lookup is by node id first and
        content second, and ids are minted per document, so a prior
        from elsewhere is legal and simply misses. Total, like the rest
        of evaluation: no refusal, no wrong answer."""
        store, docs = opened()
        prior = evaluate(docs["post"])
        stand = evaluate(docs["stand"], resolver=store, prior=prior)
        self.assertEqual(stand.reused, 0)
        self.assertEqual(failures(stand), {})

    def test_a_sibling_assembly_over_the_same_parts_still_reuses_nothing(self):
        """The sharp form of the same fact, and the one that shows why
        "a key is content, not position" was the wrong sentence.

        The layout and the stand instantiate the SAME two documents at
        the SAME pins, so their instantiate nodes agree on content. The
        lookup is `prior[node_id]` first, and the two documents mint
        their ids independently — so the overlap is worth nothing and
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

    Whether that is correct by design, or whether memo admission should
    know about resolver state, is **issue #1185** — kernel-side, and
    deliberately not decided here. Adopted from the reviewer probe
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

        docs["shelf"].apply(
            DocEdit.set_doc_param_value(
                ParamName("thickness"), DocParamValue.length(SHELF_THICKNESS * 1.5 * m)
            )
        )
        store.resave(docs["shelf"])

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

        os.remove(directory / f"{manifest()['post']}.pncad")
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
        docs["shelf"].apply(
            DocEdit.set_doc_param_value(
                ParamName("thickness"), DocParamValue.length(SHELF_THICKNESS * 1.5 * m)
            )
        )
        store.resave(docs["shelf"])
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
        os.remove(directory / f"{manifest()['post']}.pncad")
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


class TestTheCorpusIsTheToursOwn(unittest.TestCase):
    """The corpus is committed BYTES, and bytes rot. This is the guard
    that says so: the dimensions the assertions above are written
    against are read out of the scene that generated it."""

    def test_the_corpus_still_matches_the_scene_it_came_from(self):
        source = SCENE.read_text(encoding="utf-8")
        for name, value in [
            ("FLAT_PACK_GAP", FLAT_PACK_GAP),
            ("POST_SECTION", POST_SECTION),
            ("POST_HEIGHT", POST_HEIGHT),
            ("SHELF_LENGTH", SHELF_LENGTH),
            ("SHELF_DEPTH", SHELF_DEPTH),
            ("SHELF_THICKNESS", SHELF_THICKNESS),
        ]:
            with self.subTest(constant=name):
                found = re.search(rf"^const {name}: f64 = ([0-9.]+);$", source, re.M)
                self.assertIsNotNone(found, f"{name} is no longer declared there")
                self.assertEqual(
                    float(found.group(1)),
                    value,
                    f"{name} moved in the tour — regenerate the corpus "
                    f"(cd demos/tour && cargo run -- asm-corpus "
                    f"../../crates/pncad-py/tests/corpus/bench)",
                )
        # The pattern's count and spacing are authored INLINE in the
        # layout scene (`count: pe("2", ...)`, `spacing: pe("200 mm",
        # ...)`), not as `const`s, so they are read out of `layout_doc`'s
        # own body — the two remaining quantities the corpus pins, and
        # the two that drifted unseen once (PR 1506 took the flat-pack
        # from four posts to two and the corpus was not regenerated).
        start = source.index("fn layout_doc(")
        layout = source[start : source.index("\n}\n", start)]
        count = re.search(r'count: pe\("(\d+)", &scope\)', layout)
        self.assertIsNotNone(count, "layout_doc no longer authors its pattern count inline")
        self.assertEqual(int(count.group(1)), PATTERN_COUNT, "PATTERN_COUNT moved in the tour")
        spacing = re.search(r'spacing: pe\("(\d+) mm", &scope\)', layout)
        self.assertIsNotNone(spacing, "layout_doc no longer authors its spacing inline, in mm")
        self.assertEqual(
            int(spacing.group(1)) / 1000.0,
            PATTERN_SPACING,
            "PATTERN_SPACING moved in the tour",
        )

    def test_the_patterned_posts_sit_where_the_scene_places_them(self):
        """A VOLUME is invariant under placement, and until this row
        every oracle in this file was a volume — so the whole committed
        set could have been reproduced by a scene that put the parts
        anywhere at all. This is the cheapest oracle that reads
        position, through the tessellator the binding already exposes.

        It pins three things a volume cannot: the pattern's SPACING,
        its COUNT, and the instance's ROTATION — the post is on its
        side, so its long axis is x and its square section is y-z. Any
        of the three drifting in `assembly.rs` reds here after a
        regeneration. Adopted from `lib/g18a-r1b-probes`, widened from
        "four distinct origins" to the boxes themselves.
        """
        store = Workspace(str(CORPUS))
        names = manifest()
        layout = store.resolve(
            DocRef(names["layout"], store.current_pin(names["layout"]))
        )
        evaluation = evaluate(layout, resolver=store)
        pattern = evaluation.order()[1]

        def box(body):
            mesh = body.tessellate(5 * mm)
            axes = [[p[i].meters for p in mesh.positions] for i in range(3)]
            return tuple((round(min(a), 9), round(max(a), 9)) for a in axes)

        boxes = sorted(box(b) for b in evaluation.value(pattern).bodies())
        self.assertEqual(len(boxes), PATTERN_COUNT)
        for index, found in enumerate(boxes):
            with self.subTest(instance=index):
                y0 = index * PATTERN_SPACING
                self.assertEqual(
                    found,
                    (
                        (round(FLAT_PACK_GAP, 9), round(FLAT_PACK_GAP + POST_HEIGHT, 9)),
                        (round(y0, 9), round(y0 + POST_SECTION, 9)),
                        (0.0, round(POST_SECTION, 9)),
                    ),
                    "the post lies on its side beside the bench, stepped along +y",
                )

    def test_the_store_holds_exactly_the_four_documents_the_manifest_names(self):
        store = Workspace(str(CORPUS))
        self.assertEqual(sorted(store.documents()), sorted(manifest().values()))
        self.assertEqual(sorted(manifest()), ["layout", "post", "shelf", "stand"])


if __name__ == "__main__":
    unittest.main()
