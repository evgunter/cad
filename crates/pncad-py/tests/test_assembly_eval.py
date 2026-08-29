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
parts, the flat-pack `layout` (one post instance patterned four ways
plus the shelf) and the mated `stand` (two posts, a shelf, two mates).
A store names its files by identity, so `MANIFEST` carries the label
each identity was derived from.

Regenerate it with the tour's own door:

    cd demos/tour && cargo run -- asm-corpus \\
        ../../crates/pncad-py/tests/corpus/bench

WHAT KEEPS THE CORPUS HONEST, AND WHAT DOES NOT
------------------------------------------------
The volumes asserted here are functions of the scene's dimension
constants, and `test_the_corpus_still_matches_the_scene_it_came_from`
reads those constants out of `assembly.rs` and checks them — so the
corpus cannot silently drift from the tour it claims to be. What that
does NOT catch is a change to the scene's STRUCTURE that leaves the
constants alone (a fifth patterned post, a third mate): the corpus
would stay green while no longer being the tour's. Regeneration is
cheap and the recipe is above.
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
    return [
        body.mass_properties().volume for body in evaluation.value(node).bodies()
    ]


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
        for got, expected in zip(found, want):
            self.assertAlmostEqual(got, expected, delta=self.DELTA)

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
        patterned four ways, plus the shelf."""
        store, docs = opened()
        evaluation = evaluate(docs["layout"], resolver=store)
        self.assertEqual(failures(evaluation), {})
        instance, pattern, shelf = evaluation.order()
        self.assertVolumes(volumes(evaluation, instance), [POST_VOLUME])
        self.assertVolumes(volumes(evaluation, pattern), [POST_VOLUME] * 4)
        self.assertVolumes(volumes(evaluation, shelf), [SHELF_VOLUME])

    def test_the_stand_evaluates_through_the_store(self):
        """The mated bench: two posts and a shelf. The mate nodes carry
        the solve's declarations rather than a body, so they denote no
        volume — which is why the material is three solids, not five."""
        store, docs = opened()
        evaluation = evaluate(docs["stand"], resolver=store)
        self.assertEqual(failures(evaluation), {})
        material = [
            v for node in evaluation.order() for v in volumes(evaluation, node)
        ]
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

    Three of the four are reachable from Python. `part_reference_cycle`
    is not, and cannot be faked: a cycle needs a document whose
    instantiate node points back up its own reference chain, and
    authoring an instantiate node is G18b's half. It is exercised on
    the Rust side (`editor-core`'s seam tests) and stays there.
    """

    def test_a_pin_that_moved_refuses_rather_than_retargeting(self):
        directory = self.scratch()
        store, docs = opened(directory)
        docs["shelf"].apply(
            DocEdit.set_doc_param_value(
                ParamName("thickness"), DocParamValue.length(SHELF_THICKNESS * 1.5 * m)
            )
        )
        store.resave(docs["shelf"])

        refusals = failures(evaluate(docs["layout"], resolver=Workspace(str(directory))))
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
        refusals = failures(evaluate(docs["layout"], resolver=Workspace(str(directory))))
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

    def test_the_two_counters_account_for_every_live_node(self):
        store, docs = opened()
        for label, doc in docs.items():
            with self.subTest(document=label):
                first = evaluate(doc, resolver=store)
                again = evaluate(doc, resolver=store, prior=first)
                for evaluation in (first, again):
                    self.assertEqual(
                        evaluation.reused + evaluation.recomputed,
                        len(evaluation.order()),
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
        """A content key is content, not position — so an unrelated
        prior is well-defined and simply misses. Total, like the rest
        of evaluation: no refusal, no wrong answer."""
        store, docs = opened()
        prior = evaluate(docs["post"])
        stand = evaluate(docs["stand"], resolver=store, prior=prior)
        self.assertEqual(stand.reused, 0)
        self.assertEqual(failures(stand), {})


class TestTheCorpusIsTheToursOwn(unittest.TestCase):
    """The corpus is committed BYTES, and bytes rot. This is the guard
    that says so: the dimensions the assertions above are written
    against are read out of the scene that generated it."""

    def test_the_corpus_still_matches_the_scene_it_came_from(self):
        source = SCENE.read_text(encoding="utf-8")
        for name, value in [
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

    def test_the_store_holds_exactly_the_four_documents_the_manifest_names(self):
        store = Workspace(str(CORPUS))
        self.assertEqual(sorted(store.documents()), sorted(manifest().values()))
        self.assertEqual(sorted(manifest()), ["layout", "post", "shelf", "stand"])


if __name__ == "__main__":
    unittest.main()
