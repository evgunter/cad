"""Name resolution across re-evaluation (LIB-B-RESOLVE).

Every other name door in these bindings answers as of ONE evaluation
and hands back text a caller stores: `select` and the four
materializers MATERIALIZE, `Node.fillet` freezes a name set into the
recipe, and a `PickHit.name` goes into the same slot. This file is the
door-level row for the question that makes storing safe —
**does this stored name still denote, on the next run?**

THE ORACLES ARE THE OTHER DOORS, and here that is unusually literal,
because a verdict is not a measurement and there is no arithmetic to
check it against. So:

* every name a materializer answers with must RESOLVE, and it must
  resolve as the kind of thing the materializer that answered is for;
* a resolved verdict's `(node, body)` must be the pair `NodePick.build`
  takes, and the index built at that address must carry the name — the
  verdict says where the entity lives, and the pick door is what
  independently knows whether that is true;
* `denotation` is the node-scoped sibling, so the two must agree
  wherever they overlap and must DIFFER exactly where the docstrings
  say they do — `resolve` answers for a name whose node is upstream of
  the one asked.

The three states are constructed from documents, not from mocks. A
`failed` verdict is a real deleted node and a real vanished name; an
`indeterminate` one is a real fillet whose radius will not fit,
poisoning a real boolean downstream. The pairs of documents that
produce them are built by ONE builder with one argument changed, and
the tests assert the node ids match across the pair — otherwise the
comparison would be between two unrelated recipes and would prove
nothing.

WHAT IS NOT COVERED, stated rather than implied. The `ambiguous` arm
(an N2 tie) and a NON-EMPTY `offers` list (a merged name for a retired
constituent, a collapsed over-tie group's survivor) are not
constructed here: no door on this Python surface authors a tie row or
a merge, so there is no honest way to reach them from this side. They
would cross as `failed` with different prose, which is the whole of
what `Resolution` distinguishes anyway.

NOTHING HERE READS INSIDE A NAME. Every name is opaque text, compared
with other opaque texts and never parsed.
"""

import unittest

from pncad import (
    BooleanOp,
    Doc,
    DocEdit,
    EntityKind,
    Node,
    NodePick,
    PncadError,
    deg,
    evaluate,
    m,
)

#: The chordal budget the pick indices in this file are built at. The
#: solids are planar-faced, so the tessellation is exact at any budget
#: and nothing below depends on this number.
DELTA = 0.5 * m / 1000.0

#: Every state `Resolution.status` is allowed to take.
STATES = frozenset({"resolved", "failed", "indeterminate"})


def square(doc, side=1.0, at=(0.0, 0.0)):
    """A `side`-metre square on the sketch plane, corner at `at`."""
    x, y = at
    return doc.insert(
        Node.polygon(
            [
                ((x + 0.0) * m, (y + 0.0) * m),
                ((x + side) * m, (y + 0.0) * m),
                ((x + side) * m, (y + side) * m),
                ((x + 0.0) * m, (y + side) * m),
            ],
            plane=doc.sketch_frame(),
        )
    )


def unit_cube(doc, at=(0.0, 0.0)):
    """A 1 m cube on the ground plane — z from 0 to 1."""
    return doc.insert(Node.extrude(square(doc, at=at), 1.0 * m))


class TestAResolvedVerdict(unittest.TestCase):
    """The ordinary run: a name stored off one evaluation, asked about
    the same one."""

    def setUp(self):
        self.doc = Doc()
        self.cube = unit_cube(self.doc)
        self.ev = evaluate(self.doc)

    def test_every_materialized_name_resolves(self):
        """The materializers and the verdict agree, name for name.

        `all_faces` answers with the names the evaluation's tables
        carry, so every one of them must resolve — a materializer that
        handed back a name `resolve` called gone would be handing back
        an address to nowhere. 26 names on a unit cube (6 + 12 + 8),
        every one asked."""
        materialized = (
            self.ev.all_faces(self.cube)
            + self.ev.all_edges(self.cube)
            + self.ev.all_vertices(self.cube)
            + self.ev.all_bodies(self.cube)
        )
        self.assertEqual(len(materialized), 27)
        for name in materialized:
            with self.subTest(name=name):
                self.assertEqual(self.ev.resolve(name).status, "resolved")

    def test_resolved_agrees_with_denotation_where_the_two_overlap(self):
        """`denotation` is the node-scoped sibling: for a name this
        node's table carries, "resolves uniquely" is one fact and both
        doors state it."""
        for name in self.ev.all_faces(self.cube):
            with self.subTest(name=name):
                denotation = self.ev.denotation(self.cube, name)
                self.assertFalse(denotation.tied)
                self.assertEqual(denotation.candidates, 1)
                self.assertEqual(self.ev.resolve(name).status, "resolved")

    def test_the_kind_is_the_materializer_that_answered(self):
        """The verdict says WHAT a stored name denotes, and the door
        that minted the name is the oracle for it.

        This is not a restatement in Python: a name crosses as opaque
        text a caller is told never to parse, and no other door on this
        surface answers "what kind of thing is this name". The verdict
        is where that fact arrives."""
        for door, kind in (
            (self.ev.all_faces, EntityKind.Face),
            (self.ev.all_edges, EntityKind.Edge),
            (self.ev.all_vertices, EntityKind.Vertex),
            (self.ev.all_bodies, EntityKind.Body),
        ):
            for name in door(self.cube):
                with self.subTest(kind=kind, name=name):
                    self.assertEqual(self.ev.resolve(name).kind, kind)

    def test_node_and_body_are_the_address_nodepick_takes(self):
        """**The verdict's location is checkable, and the pick door is
        what checks it.**

        A resolved verdict claims the entity lives at `(node, body)`.
        `NodePick.build` takes exactly that pair, tessellates the body
        it names and indexes it, and `patch_names` reads the names back
        out of the index — independently of anything `resolve` did. So
        feeding one door's answer to the other and finding the name
        again is the claim verified rather than asserted."""
        for name in self.ev.all_faces(self.cube):
            with self.subTest(name=name):
                verdict = self.ev.resolve(name)
                index = NodePick.build(
                    self.ev, verdict.node, verdict.body, DELTA
                )
                self.assertIn(name, index.patch_names(self.ev))

    def test_the_non_resolved_attributes_are_none(self):
        """`detail` and `offers` are the other states' business: a
        resolved name has nothing to explain and nothing to suggest."""
        verdict = self.ev.resolve(self.ev.all_faces(self.cube)[0])
        self.assertIsNone(verdict.detail)
        self.assertIsNone(verdict.offers)


class TestEvaluationWideVersusNodeScoped(unittest.TestCase):
    """`resolve` asks the whole evaluation; `denotation` asks one
    node's table. The difference is not cosmetic and this is where it
    shows."""

    def setUp(self):
        self.doc = Doc()
        self.cube = unit_cube(self.doc)
        self.moved = self.doc.insert(
            Node.transform(self.cube, (3 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0), 0 * deg)
        )
        self.other = unit_cube(self.doc, at=(9.0, 9.0))
        self.ev = evaluate(self.doc)

    def test_resolve_answers_the_first_carrying_node_not_the_one_asked(self):
        """A transform passes its input's names THROUGH, so the same
        name is in two nodes' tables. `all_faces(moved)` hands it back
        under the downstream node; `resolve` answers the upstream one,
        because the first carrier in evaluation order is what it
        reports.

        The evaluation's own `order()` is the oracle for "first"."""
        order = [str(node) for node in self.ev.order()]
        self.assertLess(order.index(str(self.cube)), order.index(str(self.moved)))

        names = self.ev.all_faces(self.moved)
        self.assertEqual(len(names), 6)
        for name in names:
            with self.subTest(name=name):
                # The downstream node's table carries it — `denotation`
                # asked THERE says so.
                self.assertEqual(self.ev.denotation(self.moved, name).candidates, 1)
                # ...and the verdict still names the upstream carrier.
                self.assertEqual(str(self.ev.resolve(name).node), str(self.cube))

    def test_resolve_answers_where_denotation_refuses(self):
        """The sharp form of the same difference: ask `denotation`
        about a node whose table does not carry the name and it refuses
        `no_such_name`, which is correct and node-scoped. `resolve`
        answers, because the name is perfectly good somewhere else in
        this evaluation."""
        name = self.ev.all_faces(self.cube)[0]
        with self.assertRaises(PncadError) as caught:
            self.ev.denotation(self.other, name)
        self.assertEqual(caught.exception.variant, "no_such_name")
        self.assertEqual(self.ev.resolve(name).status, "resolved")


def plate(doc, corners):
    """A 0.1 m-thick plate over `corners`, on the sketch plane."""
    profile = doc.insert(
        Node.polygon([(x * m, y * m) for x, y in corners], plane=doc.sketch_frame())
    )
    return profile, doc.insert(Node.extrude(profile, 0.1 * m))


SQUARE = ((0.0, 0.0), (1.0, 0.0), (1.0, 0.5), (0.0, 0.5))
TRIANGLE = ((0.0, 0.0), (1.0, 0.0), (1.0, 0.5))


class TestAFailedVerdict(unittest.TestCase):
    """The name does not denote here and will not come back on its own.
    Two ways to get there, and they are different facts."""

    def test_a_deleted_minting_node_fails(self):
        """Delete the node that minted the name and the name is
        stranded: the recipe no longer contains the feature it
        addressed. Ids are never reused, so this is decided by the
        document rather than guessed."""
        doc = Doc()
        _, extrude = plate(doc, SQUARE)
        stored = evaluate(doc).all_faces(extrude)
        self.assertEqual(len(stored), 6)

        doc.apply(DocEdit.delete_node(extrude))
        after = evaluate(doc)
        for name in stored:
            with self.subTest(name=name):
                verdict = after.resolve(name)
                self.assertEqual(verdict.status, "failed")
                self.assertIn("no longer in the document", verdict.detail)
                # Nothing structural offers itself for a node that is
                # simply gone, and the empty list is the answer — not
                # an absence.
                self.assertEqual(verdict.offers, [])

    def test_a_vanished_name_fails_while_its_node_still_evaluates(self):
        """The other way: the minting node is alive and well and the
        NAME is gone from its table — a side face of a square plate has
        no counterpart on a triangular one.

        The two documents are the same recipe with one argument
        changed, so the node ids match; the test asserts that, because
        a comparison between two unrelated recipes would prove
        nothing. It also asserts the extrude still SUCCEEDS on the
        second, which is what makes this a `failed` and not an
        `indeterminate`."""
        wide = Doc()
        _, wide_extrude = plate(wide, SQUARE)
        narrow = Doc()
        _, narrow_extrude = plate(narrow, TRIANGLE)
        self.assertEqual(str(wide_extrude), str(narrow_extrude))

        before, after = evaluate(wide), evaluate(narrow)
        self.assertTrue(after.succeeded(narrow_extrude))

        was = set(before.all_faces(wide_extrude))
        now = set(after.all_faces(narrow_extrude))
        vanished = sorted(was - now)
        self.assertEqual(len(vanished), 1)

        verdict = after.resolve(vanished[0])
        self.assertEqual(verdict.status, "failed")
        self.assertIn("no longer resolves in this evaluation", verdict.detail)
        self.assertIsInstance(verdict.offers, list)

        # ...and the names that SURVIVED the edit still resolve, so the
        # verdict is discriminating rather than pessimistic.
        for name in sorted(was & now):
            with self.subTest(name=name):
                self.assertEqual(after.resolve(name).status, "resolved")


def blank(radius):
    """A unit cube with all twelve edges blended at `radius`, a peg
    fused onto its side, and the ids of both.

    Called twice with two radii: at 0.12 the blend fits and everything
    downstream evaluates; at 0.6 it cannot fit on a 1 m cube, so the
    fillet node FAILS and the boolean below it is poisoned. The recipe
    is otherwise identical, which is what makes the two documents
    comparable.
    """
    doc = Doc()
    cube = unit_cube(doc)
    edges = evaluate(doc).all_edges(cube)
    assert len(edges) == 12
    blended = doc.insert(Node.fillet(cube, radius * m, edges))
    peg = doc.insert(
        Node.extrude(
            doc.insert(
                Node.polygon(
                    [(0.6 * m, 0.3 * m), (1.4 * m, 0.3 * m),
                     (1.4 * m, 0.7 * m), (0.6 * m, 0.7 * m)],
                    plane=doc.sketch_frame(),
                )
            ),
            0.4 * m,
        )
    )
    lifted = doc.insert(
        Node.transform(peg, (0 * m, 0 * m, 0.3 * m), (0.0, 0.0, 1.0), 0 * deg)
    )
    fused = doc.insert(Node.boolean(BooleanOp.Union, blended, lifted))
    return doc, blended, fused


class TestAnIndeterminateVerdict(unittest.TestCase):
    """The NAME is fine and the RUN is not — the state whose whole
    reason for existing is that it must not be mistaken for `failed`.

    Rebinding on an `indeterminate` repairs the wrong end of the
    document: the stored name never broke, the node that mints it did,
    and the reference answers again the moment that node does.
    """

    def setUp(self):
        self.good_doc, self.good_blend, self.good_fuse = blank(0.12)
        self.broken_doc, self.broken_blend, self.broken_fuse = blank(0.6)
        # Same recipe shape, so a name minted on one is addressed at
        # the same node on the other. Without this the comparison is
        # between two unrelated documents.
        self.assertEqual(str(self.good_blend), str(self.broken_blend))
        self.assertEqual(str(self.good_fuse), str(self.broken_fuse))
        self.good = evaluate(self.good_doc)
        self.broken = evaluate(self.broken_doc)

    def test_the_pair_is_what_it_claims_to_be(self):
        """The fixture asserted, not assumed: the blend and the fuse
        both evaluate on one document and neither does on the other —
        and on the broken one they are the ONLY two nodes that did not,
        with the fuse poisoned specifically THROUGH the blend.

        That last is what lets the poisoning test below assert prose
        without reading a node number out of it: the evaluation's own
        refusal names the ancestor, as a `NodeId` that compares."""
        self.assertTrue(self.good.succeeded(self.good_blend))
        self.assertTrue(self.good.succeeded(self.good_fuse))

        stalled = [n for n in self.broken.order() if not self.broken.succeeded(n)]
        self.assertEqual(
            sorted(str(n) for n in stalled),
            sorted(str(n) for n in (self.broken_blend, self.broken_fuse)),
        )
        with self.assertRaises(PncadError) as blend_refusal:
            self.broken.value(self.broken_blend)
        self.assertEqual(blend_refusal.exception.reason, "node_failed")
        with self.assertRaises(PncadError) as fuse_refusal:
            self.broken.value(self.broken_fuse)
        self.assertEqual(fuse_refusal.exception.reason, "poisoned")
        self.assertEqual(fuse_refusal.exception.through, self.broken_blend)

    def test_a_failed_minting_node_is_indeterminate_and_resolves_again(self):
        """A name minted by the blend itself. On the broken run its
        minting node failed, so the reference is unanswerable — and on
        the good run the SAME name resolves, which is the "it resolves
        again when the node does" promise checked rather than
        restated."""
        names = self.good.all_faces(self.good_blend)
        self.assertGreater(len(names), 6)
        for name in names:
            with self.subTest(name=name):
                verdict = self.broken.resolve(name)
                self.assertEqual(verdict.status, "indeterminate")
                self.assertIn("failed this evaluation", verdict.detail)
                # Not a rebind candidate: there is nothing to rebind to
                # and nothing to suggest.
                self.assertIsNone(verdict.offers)
                self.assertEqual(self.good.resolve(name).status, "resolved")

    def test_a_poisoned_minting_node_is_indeterminate_too(self):
        """A name minted by the BOOLEAN, one node further down. Its own
        node did not fail — it was never reached, because the blend it
        consumes failed. The verdict is the same state with the repair
        pointed one node upstream, and it is emphatically not
        `failed`."""
        names = self.good.all_faces(self.good_fuse)
        self.assertGreater(len(names), 6)
        for name in names:
            with self.subTest(name=name):
                verdict = self.broken.resolve(name)
                self.assertEqual(verdict.status, "indeterminate")
                self.assertIn("poisoned by the failure at node", verdict.detail)
                self.assertIn("the repair is upstream", verdict.detail)

    def test_indeterminate_carries_no_location(self):
        """There is no location to carry: the run produced no value for
        the minting node, so `node`, `body` and `kind` would all be
        claims about an evaluation that did not happen."""
        verdict = self.broken.resolve(self.good.all_faces(self.good_blend)[0])
        self.assertIsNone(verdict.node)
        self.assertIsNone(verdict.body)
        self.assertIsNone(verdict.kind)


class TestTheVerdictSurface(unittest.TestCase):
    """Shape properties that hold across every state."""

    def setUp(self):
        doc = Doc()
        cube = unit_cube(doc)
        self.ev = evaluate(doc)
        self.name = self.ev.all_faces(cube)[0]

        stranded = Doc()
        _, extrude = plate(stranded, SQUARE)
        gone_names = evaluate(stranded).all_faces(extrude)
        stranded.apply(DocEdit.delete_node(extrude))
        gone_ev = evaluate(stranded)

        broken_doc, _, _ = blank(0.6)
        good_doc, good_blend, _ = blank(0.12)
        good_ev = evaluate(good_doc)
        broken_ev = evaluate(broken_doc)

        self.verdicts = {
            "resolved": self.ev.resolve(self.name),
            "failed": gone_ev.resolve(gone_names[0]),
            "indeterminate": broken_ev.resolve(
                good_ev.all_faces(good_blend)[0]
            ),
        }

    def test_every_state_is_reachable_and_spelled_as_promised(self):
        """All three, constructed from documents — so the tag set the
        stub and `tags.rs` promise is the tag set that ships."""
        for want, verdict in self.verdicts.items():
            with self.subTest(state=want):
                self.assertEqual(verdict.status, want)
                self.assertIn(verdict.status, STATES)

    def test_no_attribute_ever_goes_missing(self):
        """The convention every projected door here follows: every
        attribute present on every state, `None` where the state does
        not carry it, so `getattr` never raises and a caller never has
        to branch on `status` before reading."""
        for state, verdict in self.verdicts.items():
            for attribute in ("status", "node", "body", "kind", "detail", "offers"):
                with self.subTest(state=state, attribute=attribute):
                    getattr(verdict, attribute)

    def test_offers_distinguishes_empty_from_inapplicable(self):
        """A list on `failed` — empty when nothing structural offers
        itself — and `None` on the other two. "No suggestions" and
        "suggestions do not apply" are different facts."""
        self.assertIsInstance(self.verdicts["failed"].offers, list)
        self.assertIsNone(self.verdicts["resolved"].offers)
        self.assertIsNone(self.verdicts["indeterminate"].offers)

    def test_detail_is_prose_exactly_where_there_is_something_to_explain(self):
        self.assertIsNone(self.verdicts["resolved"].detail)
        for state in ("failed", "indeterminate"):
            with self.subTest(state=state):
                self.assertTrue(self.verdicts[state].detail)

    def test_the_repr_states_the_state(self):
        self.assertIn("resolved", repr(self.verdicts["resolved"]))
        self.assertIn("failed", repr(self.verdicts["failed"]))
        self.assertIn("indeterminate", repr(self.verdicts["indeterminate"]))

    def test_text_that_is_not_a_name_is_a_boundary_refusal(self):
        """The one raise this door has, and it is the boundary's, not
        the kernel's: a string that is not a name at all never reaches
        resolution. A well-formed name that denotes nothing is a
        verdict, which is the whole point of the door — so this must
        not be a `PncadError`."""
        for text in ("the top face", "", "{", "[1, 2, 3]"):
            with self.subTest(text=text):
                with self.assertRaises(ValueError) as caught:
                    self.ev.resolve(text)
                self.assertNotIsInstance(caught.exception, PncadError)

    def test_a_name_from_another_document_is_a_verdict_not_a_raise(self):
        """The case the door exists for, in its rawest form: a name
        from somewhere else entirely — a file, another session — asked
        of this evaluation. It answers; it does not raise."""
        elsewhere = Doc()
        _, extrude = plate(elsewhere, TRIANGLE)
        foreign = evaluate(elsewhere).all_faces(extrude)
        for name in foreign:
            with self.subTest(name=name):
                self.assertIn(self.ev.resolve(name).status, STATES)


if __name__ == "__main__":
    unittest.main()
