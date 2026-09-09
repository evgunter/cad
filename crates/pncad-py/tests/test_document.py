"""The document surface, and the D9 bit-precision seed.

§L3: Python speaks Doc/DocEdit/evaluate — never an arena key. Every
test here goes through a document; none reaches into the kernel.
"""

import math
import struct
import unittest

import pncad
from pncad import (
    BooleanOp,
    Cmp,
    Distribution,
    Doc,
    DocEdit,
    DocParam,
    EditError,
    EntityKind,
    EvaluationError,
    Frame,
    GeomPred,
    Length,
    MeasureExpr,
    MeasurePrimitive,
    DocParamValue,
    NamePat,
    Node,
    Open,
    ParamName,
    PatternKind,
    Selector,
    SketchPlane,
    Start,
    evaluate,
    import_step,
    load,
    m,
    rad,
)


def unit_box(doc, width, depth, height):
    """Insert a rectangular prism rooted at the origin."""
    return slab(doc, (0 * m, width), (0 * m, depth), (0 * m, height))


def slab(doc, x, y, z):
    """Insert the axis-aligned box [x0,x1] x [y0,y1] x [z0,z1]."""
    x0, x1 = x
    y0, y1 = y
    z0, z1 = z
    profile = doc.insert(
        Node.polygon(
            [(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
            plane=doc.sketch_frame(elevation=z0),
        )
    )
    return doc.insert(Node.extrude(profile, z1 - z0))


class TestDocumentEditing(unittest.TestCase):
    def test_an_empty_document_has_no_nodes(self):
        doc = Doc()
        self.assertEqual(len(doc), 0)
        self.assertEqual(doc.node_count, 0)

    def test_insert_mints_ids_and_grows_the_document(self):
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 1 * m)
        # THREE: the sketch frame, the profile drawn on it, the extrude.
        self.assertEqual(doc.node_count, 3)
        self.assertIn(box, doc.order())

    def test_edits_go_through_the_docedit_vocabulary(self):
        doc = Doc()
        # The frame is its own insert, through the same one door.
        frame = doc.sketch_frame()
        minted = doc.apply(
            DocEdit.insert_node(
                Node.polygon(
                    [(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m)], plane=frame
                )
            )
        )
        self.assertIsNotNone(minted)
        self.assertEqual(doc.node_count, 2)

    def test_a_refused_edit_leaves_the_document_untouched(self):
        doc = Doc()
        box = unit_box(doc, 1 * m, 1 * m, 1 * m)
        before = doc.node_count
        with self.assertRaises(pncad.EditError) as caught:
            # Deleting a node another node depends on must dangle.
            doc.apply(DocEdit.delete_node(doc.order()[0]))
        # The refusal carries a stable tag, not prose (§L4).
        self.assertEqual(caught.exception.variant, "delete_would_dangle")
        self.assertEqual(doc.node_count, before)
        self.assertTrue(evaluate(doc).succeeded(box))

    def test_unknown_node_is_a_typed_evaluation_error(self):
        doc = Doc()
        box = unit_box(doc, 1 * m, 1 * m, 1 * m)
        # A second, LARGER document mints ids the first never used.
        other = Doc()
        unit_box(other, 1 * m, 1 * m, 1 * m)
        unit_box(other, 1 * m, 1 * m, 1 * m)
        stray = [n for n in other.order() if n not in doc.order()]
        self.assertTrue(stray, "the larger document minted unused ids")

        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(box))
        with self.assertRaises(EvaluationError) as caught:
            ev.value(stray[-1])
        self.assertEqual(caught.exception.reason, "unknown_node")
        self.assertEqual(caught.exception.node, stray[-1])


class TestNodeKindReadDoor(unittest.TestCase):
    """`Doc.node_kind` — the read half of the `Node.*` constructors.

    The vocabulary itself is pinned Rust-side
    (`the_node_kind_vocabulary_matches_its_committed_roster`, which
    reads `src/node_kind.rs` and `pncad.pyi`). What is executed here is
    the MAPPING that pin cannot make: which node answers which word,
    driven through real documents.
    """

    def test_each_authored_node_answers_its_own_word(self):
        doc = Doc()
        frame = doc.sketch_frame()
        profile = doc.insert(
            Node.polygon(
                [(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m)], plane=frame
            )
        )
        solid = doc.insert(Node.extrude(profile, 1 * m))
        self.assertEqual(doc.node_kind(frame), "datum")
        self.assertEqual(doc.node_kind(profile), "profile")
        self.assertEqual(doc.node_kind(solid), "extrude")

    def test_a_boolean_answers_a_word_per_operation(self):
        doc = Doc()
        a = unit_box(doc, 2 * m, 2 * m, 2 * m)
        b = slab(doc, (1 * m, 3 * m), (1 * m, 3 * m), (1 * m, 3 * m))
        # One payload shape, three kernel operations, three words —
        # which is what lets a caller say "this recipe spends no
        # subtract" without reading the saved text.
        cut = doc.insert(Node.boolean(BooleanOp.Subtract, a, b))
        fused = doc.insert(Node.boolean(BooleanOp.Union, a, b))
        common = doc.insert(Node.boolean(BooleanOp.Intersect, a, b))
        self.assertEqual(doc.node_kind(cut), "boolean_subtract")
        self.assertEqual(doc.node_kind(fused), "boolean_union")
        self.assertEqual(doc.node_kind(common), "boolean_intersect")

    def test_it_is_the_nodes_kind_and_not_its_values(self):
        doc = Doc()
        solid = unit_box(doc, 2 * m, 2 * m, 2 * m)
        moved = doc.insert(
            Node.transform(solid, (1 * m, 0 * m, 0 * m), (0.0, 0.0, 1.0), 0 * rad)
        )
        ev = evaluate(doc)
        # Two different recipes, one value kind: `Value.kind` is the
        # PAYLOAD's shape and cannot tell an extrude from a rigid
        # placement of it. The node's kind can, and answers with no
        # evaluation in hand at all.
        self.assertEqual(ev.value(solid).kind, "body")
        self.assertEqual(ev.value(moved).kind, "body")
        self.assertEqual(doc.node_kind(solid), "extrude")
        self.assertEqual(doc.node_kind(moved), "transform")

    def test_an_unknown_node_refuses_through_the_edit_vocabulary(self):
        doc = Doc()
        unit_box(doc, 1 * m, 1 * m, 1 * m)
        # A second, LARGER document mints ids the first never used.
        other = Doc()
        unit_box(other, 1 * m, 1 * m, 1 * m)
        unit_box(other, 1 * m, 1 * m, 1 * m)
        stray = [n for n in other.order() if n not in doc.order()]
        self.assertTrue(stray, "the larger document minted unused ids")

        # `unknown_node`, the word the document layer already speaks
        # for this state — a refusal rather than a word or `None`,
        # because "no such node" must not read as a kind.
        with self.assertRaises(EditError) as caught:
            doc.node_kind(stray[-1])
        self.assertEqual(caught.exception.variant, "unknown_node")


class TestEvaluation(unittest.TestCase):
    def test_evaluate_returns_typed_per_node_values(self):
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 1 * m)
        ev = evaluate(doc)
        # THREE nodes: the sketch frame the profile is drawn on comes
        # first, because a profile names one.
        frame_node, profile_node, extrude_node = doc.order()
        self.assertEqual(ev.value(frame_node).kind, "datum")
        self.assertEqual(ev.value(profile_node).kind, "profile")
        value = ev.value(extrude_node)
        self.assertEqual(value.kind, "body")
        self.assertEqual(value.body().mass_properties().volume, 6.0)
        self.assertEqual(box, extrude_node)

    def test_a_body_validates(self):
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 1 * m)
        body = evaluate(doc).value(box).body()
        body.validate()  # raises ValidationError if it fails
        body.validate_closed()

    def test_wrong_kind_is_a_typed_refusal(self):
        doc = Doc()
        unit_box(doc, 1 * m, 1 * m, 1 * m)
        profile_node = doc.order()[0]
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(profile_node).body()
        self.assertEqual(caught.exception.reason, "wrong_kind")

    def test_boolean_union_through_the_document(self):
        # The post is strictly interior in x and y and pokes out of the
        # base's top, so the solids genuinely INTERPENETRATE and no two
        # faces are coincident. That matters: the kernel never infers
        # coincidence from values, so boxes merely touching on a shared
        # plane are refused until the author declares the contact.
        doc = Doc()
        base = slab(doc, (0 * m, 3 * m), (0 * m, 2 * m), (0 * m, 1 * m))  # 6.0
        post = slab(doc, (0.5 * m, 1.5 * m), (0.5 * m, 1.5 * m), (0.5 * m, 2 * m))
        fused = doc.insert(Node.boolean(BooleanOp.Union, base, post))

        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(fused), "the union evaluated")
        self.assertEqual(ev.value(fused).kind, "boolean")
        body = ev.value(fused).body()
        body.validate_closed()
        # 6.0 base + 1.5 post - 0.5 shared = 7.0
        self.assertEqual(body.mass_properties().volume, 7.0)

    def test_a_coincident_boolean_carries_its_typed_refusal(self):
        # Fail-loud, visible from Python: two boxes sharing the z=0
        # plane are NOT silently fused — and the refusal now arrives
        # WITH its typed cause (LIB-DOORS F3; U9S's `no_value`
        # placeholder is gone).
        doc = Doc()
        outer = unit_box(doc, 2 * m, 2 * m, 2 * m)
        inner = unit_box(doc, 1 * m, 1 * m, 1 * m)
        cut = doc.insert(Node.boolean(BooleanOp.Subtract, outer, inner))
        ev = evaluate(doc)
        self.assertFalse(ev.succeeded(cut))
        with self.assertRaises(EvaluationError) as caught:
            ev.value(cut)
        self.assertEqual(caught.exception.reason, "node_failed")
        self.assertEqual(caught.exception.node, cut)
        # Since register R3 (LIB-PYG5) the undeclared-contact refusal
        # is the typed MENU: its own stable tag, and the candidate
        # declaration attached as a `FlushFinding` value.
        self.assertEqual(caught.exception.kind, "undeclared_contact")
        self.assertIsNone(caught.exception.through)
        finding = caught.exception.finding
        self.assertIsInstance(finding, pncad.FlushFinding)
        # Both boxes rise from z=0: the shared bottom planes face the
        # same way — the flush-wall (merge-stage) flavor.
        self.assertEqual(finding.relation, pncad.PlaneRelation.SameOriented)
        self.assertEqual(finding.class_, pncad.ContactClass.Rest)
        self.assertEqual(finding.rung, pncad.FlushRung.DecidedCoincident)
        # The pair's names speak the one opaque alphabet: each side is
        # a FACE name of its own operand's evaluation.
        self.assertIn(finding.a, ev.all_faces(outer))
        self.assertIn(finding.b, ev.all_faces(inner))
        # F6 (reopened on review): the MESSAGE is prose stating the
        # problem and the two-armed recourse, not Debug guts.
        message = str(caught.exception)
        self.assertIn("Boolean refused an undeclared contact", message)
        self.assertIn("declare that finding", message)
        for guts in ("UndeclaredCoincidence", "UndeclaredContact", "{", "NodeError"):
            self.assertNotIn(guts, message)

    def test_a_poisoned_node_names_its_failed_ancestor(self):
        doc = Doc()
        outer = unit_box(doc, 2 * m, 2 * m, 2 * m)
        inner = unit_box(doc, 1 * m, 1 * m, 1 * m)
        cut = doc.insert(Node.boolean(BooleanOp.Subtract, outer, inner))
        downstream = doc.insert(Node.boolean(BooleanOp.Union, cut, outer))
        ev = evaluate(doc)
        with self.assertRaises(EvaluationError) as caught:
            ev.value(downstream)
        self.assertEqual(caught.exception.reason, "poisoned")
        self.assertEqual(caught.exception.node, downstream)
        self.assertEqual(caught.exception.through, cut)
        # The root cause's tag rides along: the ancestor's refusal.
        self.assertEqual(caught.exception.kind, "undeclared_contact")
        # The menu payload does NOT ride a poisoning — the recourse
        # belongs to the node that refused; here it is None (attributes
        # never go missing, LIB-DOORS F3).
        self.assertIsNone(caught.exception.finding)
        self.assertIn("poisoned by failed ancestor", str(caught.exception))


class TestDetectDeclareDoors(unittest.TestCase):
    """LIB-PYG5 (G5): the detect/declare doors' own contracts —
    positive paths through every spelling, adversarial args refused
    typed. The scene-level flips live in `test_north_star.py`
    (`TestTable`, `TestCrosslapGlued`); the guide's executed block is
    the end-to-end menu recourse."""

    def stacked(self):
        doc = Doc()
        lower = slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        upper = slab(
            doc, (0.25 * m, 0.75 * m), (0.25 * m, 0.75 * m), (1 * m, 1.5 * m)
        )
        return doc, lower, upper

    def test_every_declare_spelling_feeds_the_boolean(self):
        # One resting contact; three spellings of the declare arm,
        # each wired into the SAME union, each at the exact volume
        # 1 + 0.5^2 * 0.5 = 1.125 (dyadic).
        for spelling in ("doc_declare", "doc_declare_all", "node_declare"):
            with self.subTest(spelling=spelling):
                doc, lower, upper = self.stacked()
                ev = evaluate(doc)
                findings = ev.find_flush_candidates(lower, upper)
                self.assertEqual(len(findings), 1)
                self.assertEqual(
                    findings[0].relation, pncad.PlaneRelation.SameOpposite
                )
                if spelling == "doc_declare":
                    decl = doc.declare(findings[0])
                elif spelling == "doc_declare_all":
                    decl = doc.declare_all(findings)
                else:
                    decl = doc.insert(Node.declare(findings))
                glued = doc.insert(
                    Node.boolean(BooleanOp.Union, lower, upper, declare=decl)
                )
                ev = evaluate(doc)
                body = ev.value(glued).body()
                body.validate()
                self.assertEqual(body.mass_properties().volume, 1.125)

    def test_declaring_nothing_refuses_typed_at_every_door(self):
        # An empty Declare records no intent — refused, never inserted
        # (`no_findings`), at the sugar AND at the node constructor.
        doc = Doc()
        with self.assertRaises(EditError) as caught:
            doc.declare_all([])
        self.assertEqual(caught.exception.variant, "no_findings")
        # The human message is the declare door's own prose, not a
        # mangled literal (review MINOR-1: a doubled-space run shipped
        # once because nothing pinned the text) and not a struct dump.
        message = str(caught.exception)
        self.assertIn("declare", message)
        self.assertIn("records no intent", message)
        self.assertIn("pass the findings", message)
        self.assertNotIn("  ", message)
        self.assertNotIn("{", message)
        self.assertNotIn("NoFindings", message)
        self.assertEqual(len(doc), 0, "a refused declare inserts nothing")
        with self.assertRaises(EditError) as caught:
            Node.declare([])
        self.assertEqual(caught.exception.variant, "no_findings")
        self.assertNotIn("  ", str(caught.exception))

    def test_detection_answers_empty_for_separated_and_unevaluated(self):
        # Separated in EVERY plane family: a pair sharing any plane —
        # even with disjoint faces (two boxes side by side on one
        # floor) — is honestly a finding, so "no findings" needs no
        # shared carrier at all.
        doc = Doc()
        a = slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        b = slab(doc, (3 * m, 4 * m), (5 * m, 6 * m), (2 * m, 3 * m))
        ev = evaluate(doc)
        self.assertEqual(ev.find_flush_candidates(a, b), [])
        # A node the evaluation does not know: empty, like `select`.
        c = slab(doc, (6 * m, 7 * m), (8 * m, 9 * m), (4 * m, 5 * m))
        self.assertEqual(ev.find_flush_candidates(a, c), [])

    def test_findings_are_values_with_opaque_names(self):
        doc, lower, upper = self.stacked()
        ev = evaluate(doc)
        finding = ev.find_flush_candidates(lower, upper)[0]
        # The names are the same alphabet the materializers speak.
        self.assertIn(finding.a, ev.all_faces(lower))
        self.assertIn(finding.b, ev.all_faces(upper))
        self.assertEqual(finding.class_, pncad.ContactClass.Rest)
        self.assertEqual(finding.rung, pncad.FlushRung.DecidedCoincident)
        # Value semantics: re-detection answers an equal value.
        self.assertEqual(finding, ev.find_flush_candidates(lower, upper)[0])


class TestLiteralRefusals(unittest.TestCase):
    """LIB-DOORS F5 + fix pass: the kernel's own refusal, with the
    offending value restored to the exception payload."""

    def test_a_non_finite_literal_carries_kind_value_and_prose(self):
        doc = Doc()
        box = unit_box(doc, 1 * m, 1 * m, 1 * m)
        profile_node = doc.order()[0]
        with self.assertRaises(pncad.LiteralError) as caught:
            doc.insert(Node.extrude(profile_node, float("nan") * m))
        self.assertEqual(caught.exception.kind, "non_finite")
        self.assertNotEqual(
            caught.exception.value, caught.exception.value
        )  # NaN != NaN: the offending value itself rides the exception
        message = str(caught.exception)
        self.assertIn("finite", message)
        self.assertNotIn("NonFiniteLiteral", message)  # prose, not variant name
        self.assertTrue(evaluate(doc).succeeded(box), "the document is untouched")


class TestD9BitReplaySeed(unittest.TestCase):
    """The first form of the cross-platform bit-replay pin (D9).

    D9's pure-libm determinism means a wheel replays BIT-IDENTICALLY
    across platforms. This test pins ONE volume at full f64 precision.

    Scope, stated honestly: this run is SINGLE-PLATFORM. It proves the
    value is reproducible here and gives the future cross-platform
    matrix an exact number to compare against; it does not by itself
    demonstrate cross-platform identity.
    """

    # 2 m x 3 m x 0.5 m. Pinned by its exact IEEE-754 bits, not by a
    # tolerance — the whole point of the D9 claim.
    EXPECTED_VOLUME_HEX = "0x1.8000000000000p+1"  # exactly 3.0

    def test_volume_is_bit_exact(self):
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 0.5 * m)
        volume = evaluate(doc).value(box).body().mass_properties().volume
        self.assertEqual(
            volume.hex(),
            self.EXPECTED_VOLUME_HEX,
            f"volume drifted: {volume.hex()} (bits "
            f"{struct.pack('>d', volume).hex()})",
        )

    def test_replay_of_the_same_recipe_is_bit_identical(self):
        def build():
            # `bit_eq` compares identity too, so the two builds are
            # the SAME part on purpose — the labelled constructor is
            # what says that. `Doc()` mints a fresh id and would (and
            # should) compare unequal.
            doc = Doc(label="replay-of-the-same-recipe")
            box = unit_box(doc, 2 * m, 3 * m, 0.5 * m)
            return doc, evaluate(doc).value(box).body().mass_properties()

        first_doc, first = build()
        second_doc, second = build()
        self.assertTrue(first_doc.bit_eq(second_doc), "documents are bit-equal")
        self.assertEqual(
            struct.pack(">d", first.volume),
            struct.pack(">d", second.volume),
        )
        self.assertEqual(
            struct.pack(">d", first.surface_area),
            struct.pack(">d", second.surface_area),
        )


class TestDocumentIdentity(unittest.TestCase):
    """A document id answers WHICH PART, and the workspace store's
    uniqueness invariant is keyed on it — so an id that is the same
    for every Python-authored document makes them all one part, and
    makes two of them unstorable side by side."""

    def test_two_authored_documents_are_two_parts(self):
        first, second = Doc(), Doc()
        self.assertNotEqual(
            first.id, second.id, "two documents authored here are one part"
        )

    def test_an_id_is_the_canonical_thirty_two_hex_digits(self):
        doc_id = Doc().id
        self.assertEqual(len(doc_id), 32)
        self.assertEqual(doc_id, doc_id.lower())
        int(doc_id, 16)  # parses, or this raises

    def test_the_id_is_what_the_save_header_carries(self):
        # The workspace scan reads exactly this line to build its
        # id -> path map, so two saved documents landing in one
        # directory are distinguishable by the store.
        first, second = Doc(), Doc()
        headers = [
            next(
                line
                for line in doc.save().splitlines()
                if line.startswith("id: ")
            )
            for doc in (first, second)
        ]
        self.assertEqual(headers[0], f"id: {first.id}")
        self.assertEqual(headers[1], f"id: {second.id}")
        self.assertNotEqual(headers[0], headers[1])

    def test_identity_survives_every_edit(self):
        doc = Doc()
        before = doc.id
        unit_box(doc, 1 * m, 1 * m, 1 * m)
        self.assertEqual(doc.id, before, "an edit does not change which part")

    def test_a_labelled_document_is_the_same_part_every_time(self):
        self.assertEqual(Doc(label="plate-param").id, Doc(label="plate-param").id)
        self.assertEqual(Doc("plate-param").id, Doc(label="plate-param").id)
        self.assertNotEqual(Doc(label="plate-param").id, Doc(label="bracket").id)

    def test_a_loaded_document_keeps_the_id_it_was_saved_under(self):
        doc = Doc()
        unit_box(doc, 1 * m, 1 * m, 1 * m)
        self.assertEqual(load(doc.save()).doc.id, doc.id)


class TestPersistence(unittest.TestCase):
    """LIB-DOORS F1: the persistence doors, through the curated facade."""

    def test_save_load_evaluate_round_trip_is_bit_exact(self):
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 0.5 * m)
        before = evaluate(doc).value(box).body().mass_properties().volume

        text = doc.save()
        self.assertTrue(
            text.startswith(f"id: {doc.id}\n"),
            "the file's header names the document",
        )

        loaded = load(text)
        self.assertEqual(loaded.edit_count, 0)  # snapshot-only file
        self.assertTrue(loaded.doc.bit_eq(doc), "load replays to the SAME document")
        after = evaluate(loaded.doc).value(box).body().mass_properties().volume
        # D9: the same recipe replays to the same bits.
        self.assertEqual(struct.pack(">d", before), struct.pack(">d", after))

    def test_a_garbage_file_is_a_typed_refusal(self):
        with self.assertRaises(pncad.PersistError) as caught:
            load("not a document")
        refusal = caught.exception
        self.assertEqual(refusal.variant, "header_id")
        # The arm's payload: what the id line looked like. Every other
        # attribute is present and `None`, so a caller reads the
        # payload without first branching on `variant`.
        self.assertEqual(refusal.found, "not a document")
        for absent in (
            "inner_variant", "site", "node", "name", "unit", "declared",
            "detail", "header", "snapshot", "line", "column", "index",
            "process", "document",
        ):
            self.assertIsNone(getattr(refusal, absent), absent)

    def test_a_body_this_build_cannot_read_is_a_typed_refusal(self):
        """The format carries no schema version: a document a build
        cannot read refuses on the deserializer's own rejection, naming
        the vocabulary it could not place, with the regenerate recourse."""
        body = '{"snapshot": {"no_such_field": 1}, "edits": []}'
        with self.assertRaises(pncad.PersistError) as caught:
            load(f"id: {'0' * 32}\n{body}")
        refusal = caught.exception
        self.assertEqual(refusal.variant, "unreadable")
        self.assertIn("no_such_field", str(refusal))
        self.assertIn("regenerate", str(refusal))
        # The deserializer's position and its own words, as payload
        # rather than as prose to parse: `detail` is the reporter's
        # words on this arm, on `parse` and on `serialize` alike.
        self.assertEqual(refusal.line, 1)
        self.assertGreater(refusal.column, 0)
        self.assertIn("no_such_field", refusal.detail)
        self.assertIsNone(refusal.found)

    def test_a_header_that_disagrees_with_the_snapshot_names_both_ids(self):
        """A tampered or hand-assembled file: the save door writes the
        snapshot's id, so the two agree by construction."""
        doc = Doc()
        unit_box(doc, 1 * m, 1 * m, 1 * m)
        _, body = doc.save().split("\n", 1)
        other = "0" * 32
        with self.assertRaises(pncad.PersistError) as caught:
            load(f"id: {other}\n{body}")
        refusal = caught.exception
        self.assertEqual(refusal.variant, "id_mismatch")
        self.assertEqual(refusal.header, other)
        self.assertEqual(refusal.snapshot, doc.id)
        self.assertIsNone(refusal.inner_variant)

    def test_a_snapshot_invariant_names_the_invariant_it_broke(self):
        """The wrapped refusal's own word rides beside the stage's:
        `variant` says the snapshot failed, `inner_variant` says which
        invariant."""
        doc = Doc()
        unit_box(doc, 1 * m, 1 * m, 1 * m)
        text = doc.save()
        # Wind the mint counter back behind ids the document holds.
        tampered = text.replace('"next_id": 3', '"next_id": 1')
        self.assertNotEqual(tampered, text, "the tamper found its slot")
        with self.assertRaises(pncad.PersistError) as caught:
            load(tampered)
        refusal = caught.exception
        self.assertEqual(refusal.variant, "snapshot")
        self.assertEqual(refusal.inner_variant, "id_beyond_counter")
        # The snapshot refusal's own node ids are the snapshot door's
        # surface, not this one's: the word crosses, the payload does
        # not.
        self.assertIsNone(refusal.node)


class TestStepExport(unittest.TestCase):
    """LIB-DOORS F2: the document-layer export door and its oracle."""

    def test_export_reimports_with_the_same_volume(self):
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 0.5 * m)
        ev = evaluate(doc)
        step = ev.step_string(box, product_name="doors-box")
        self.assertIn("ISO-10303-21", step)
        # The oracle is the kernel's own importer: the text PARSES and
        # adopts as a first-class solid whose volume agrees. The
        # enclosure is the import gate's OWN measurement, so reading it
        # is the whole journey — nothing here measures twice.
        report = import_step(step)
        self.assertAlmostEqual(report.enclosure.volume, 3.0, places=9)

    def test_the_reports_enclosure_is_not_a_second_computation(self):
        """The gate already ran the certified quadrature to decide this
        body's orientation invariant, and `enclosure` is that result
        handed back rather than dropped.

        BIT equality, not `assertAlmostEqual`: the claim is that the
        two are the same computation over the same body at the same
        band, and a tolerance here would pass just as happily if they
        were two different ones that happened to agree.
        """
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 0.5 * m)
        report = import_step(evaluate(doc).step_string(box))
        again = report.body.mass_properties()
        for field in ("volume", "surface_area", "volume_pad", "area_pad"):
            self.assertEqual(
                getattr(report.enclosure, field).hex(),
                getattr(again, field).hex(),
                f"{field} differs between the gate's enclosure and a re-measure",
            )

    def test_the_report_carries_every_field_of_the_import(self):
        """Each field of the importer's success value is an attribute,
        and each record row spells out its own payload.

        The exported box states no assembly and needs no re-minting, so
        two of the three lists are empty and the third is not: the
        assembly record is kept whether or not the file states one,
        which is what makes `instances` an answer rather than a
        leftover.
        """
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 0.5 * m)
        report = import_step(evaluate(doc).step_string(box))
        self.assertIsInstance(report.body, pncad.Body)
        self.assertIsInstance(report.enclosure, pncad.MassProperties)
        self.assertGreater(report.eps_in, 0.0)
        self.assertEqual(report.normalizations, [])
        self.assertEqual(report.promotions, [])
        self.assertEqual(len(report.instances), 1, "one row per solid")

        instance = report.instances[0]
        self.assertEqual(instance.index, 0)
        self.assertGreater(instance.solid, 0)
        self.assertGreater(instance.component, 0)
        # The file places nothing, so the occurrence half of the record
        # is `None` rather than absent — a read never raises.
        for absent in ("occurrence", "relationship", "transform", "placement"):
            self.assertIsNone(getattr(instance, absent))

    def test_the_report_is_frozen(self):
        """A report is a VALUE: it is restated by importing again,
        never by editing one in place."""
        doc = Doc()
        box = unit_box(doc, 1 * m, 1 * m, 1 * m)
        report = import_step(evaluate(doc).step_string(box))
        with self.assertRaises(AttributeError):
            report.eps_in = 1.0
        with self.assertRaises(AttributeError):
            report.instances[0].index = 7

    def test_export_of_a_profile_is_a_typed_refusal(self):
        doc = Doc()
        unit_box(doc, 1 * m, 1 * m, 1 * m)
        # Index 1: the frame the profile is drawn on comes first.
        profile_node = doc.order()[1]
        ev = evaluate(doc)
        with self.assertRaises(pncad.ExportError) as caught:
            ev.step_string(profile_node)
        self.assertEqual(caught.exception.variant, "not_a_body")
        self.assertEqual(caught.exception.kind, "profile")

    def test_the_export_refusal_names_the_node_as_a_bare_id(self):
        """A node reaches prose as its number, not as a Rust wrapper.

        The one part of an export refusal a caller can act on is which
        node it is about. `RecipeNodeId`'s `Debug` spelling puts a Rust
        type name in front of that number — a token with no meaning on
        this side of the boundary.
        """
        doc = Doc()
        unit_box(doc, 1 * m, 1 * m, 1 * m)
        profile_node = doc.order()[0]
        ev = evaluate(doc)
        with self.assertRaises(pncad.ExportError) as caught:
            ev.step_string(profile_node)
        message = str(caught.exception)
        self.assertNotIn("RecipeNodeId", message)
        self.assertRegex(message, r"node \d+ ")

    def test_every_step_option_reaches_the_written_file(self):
        """The whole `StepOptions` record is the door's keywords.

        Each keyword is read back out of the Part 21 text it writes,
        so this says the argument REACHED the writer — the census in
        `pncad-py`'s Rust tests says the keyword exists, which is a
        weaker claim and the one that stops the surface going silent
        again.
        """
        doc = Doc()
        box = unit_box(doc, 1 * m, 1 * m, 1 * m)
        step = evaluate(doc).step_string(
            box,
            product_name="options-box",
            timestamp="2026-09-01T12:00:00",
            author="Ada",
            organization="Analytical Engines",
            originating_system="pncad-test",
            uncertainty=1e-9 * m,
        )
        for written in (
            "'options-box.step'",
            "'2026-09-01T12:00:00'",
            "'Ada'",
            "'Analytical Engines'",
            "'pncad-test'",
        ):
            self.assertIn(written, step)
        # The uncertainty is the schema's own length measure, so it
        # lands in the geometric context rather than the header.
        self.assertIn("UNCERTAINTY_MEASURE_WITH_UNIT(LENGTH_MEASURE(1.0E-9)", step)

    def test_the_defaults_are_the_rust_defaults(self):
        doc = Doc()
        box = unit_box(doc, 1 * m, 1 * m, 1 * m)
        ev = evaluate(doc)
        self.assertEqual(ev.step_string(box), ev.step_string(box, product_name="part"))

    def test_a_non_positive_uncertainty_is_the_writers_refusal(self):
        """Python pre-checks nothing: the rule is the writer's, and
        its refusal arrives as the export door's typed error."""
        doc = Doc()
        box = unit_box(doc, 1 * m, 1 * m, 1 * m)
        ev = evaluate(doc)
        with self.assertRaises(pncad.ExportError) as caught:
            ev.step_string(box, uncertainty=0 * m)
        self.assertEqual(caught.exception.variant, "step_refused")

    def test_export_of_a_failed_node_is_a_typed_refusal(self):
        doc = Doc()
        outer = unit_box(doc, 2 * m, 2 * m, 2 * m)
        inner = unit_box(doc, 1 * m, 1 * m, 1 * m)
        cut = doc.insert(Node.boolean(BooleanOp.Subtract, outer, inner))
        ev = evaluate(doc)
        with self.assertRaises(pncad.ExportError) as caught:
            ev.step_string(cut)
        self.assertEqual(caught.exception.variant, "node_failed")

    def test_import_of_garbage_is_a_typed_refusal(self):
        """The tag names WHICH refusal, not that there was one.

        It used to be the literal `refused` for all twenty-one arms of
        the importer's error, so this row could not tell a malformed
        file from an unsupported entity from a tier refusal — and the
        id and line that would separate them live in the message prose.
        """
        with self.assertRaises(pncad.StepImportError) as caught:
            import_step("not a step file")
        self.assertEqual(caught.exception.variant, "syntax")

    def test_a_parsed_file_with_no_body_refuses_under_its_own_tag(self):
        """The second tag, so the row above is pinning a MAP and not a
        constant: two different refusals of the same door must not
        arrive under one name."""
        header = (
            "ISO-10303-21;\n"
            "HEADER;\n"
            "FILE_DESCRIPTION((''),'2;1');\n"
            "FILE_NAME('','',(''),(''),'','','');\n"
            "FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\n"
            "ENDSEC;\n"
            "DATA;\n"
            "ENDSEC;\n"
            "END-ISO-10303-21;\n"
        )
        with self.assertRaises(pncad.StepImportError) as caught:
            import_step(header)
        self.assertNotEqual(caught.exception.variant, "syntax")


class TestNoArenaKeysCross(unittest.TestCase):
    """§L3's boundary rule, asserted rather than assumed."""

    def test_the_module_exposes_no_key_types(self):
        exposed = set(dir(pncad))
        for forbidden in ("EntityRef", "EntityKey", "Entry", "FaceKey", "EdgeKey", "VertexKey"):
            self.assertNotIn(forbidden, exposed)

    def test_node_ids_are_the_only_identifier(self):
        doc = Doc()
        box = unit_box(doc, 1 * m, 1 * m, 1 * m)
        self.assertIsInstance(box, pncad.NodeId)


if __name__ == "__main__":
    unittest.main()


class TestDocParamEquality(unittest.TestCase):
    """LIB-PYBUNDLE rider (a): `DocParam` mirrors Rust's `PartialEq`.

    Which is IEEE comparison of the stored value, NOT the bit
    comparison `DocParam::bit_eq` makes — so the two spellings of zero
    are the SAME parameter here and different ones to `bit_eq`,
    exactly as in Rust. The hash follows the equality it mirrors."""

    def test_equality_is_value_and_dimension(self):
        self.assertEqual(DocParam.length(2 * m), DocParam.length(2 * m))
        self.assertNotEqual(DocParam.length(2 * m), DocParam.length(3 * m))
        self.assertNotEqual(DocParam.length(1 * m), DocParam.scalar(1.0))
        self.assertNotEqual(DocParam.count(1), DocParam.scalar(1.0))
        self.assertEqual(DocParam.count(4), DocParam.count(4))

    def test_the_two_zeros_are_one_parameter_and_hash_alike(self):
        plus, minus = DocParam.length(0.0 * m), DocParam.length(-0.0 * m)
        self.assertEqual(plus, minus)
        self.assertEqual(hash(plus), hash(minus))

    def test_equal_parameters_are_interchangeable_dict_keys(self):
        table = {DocParam.length(2 * m): "thickness", DocParam.count(3): "ribs"}
        self.assertEqual(table[DocParam.length(2 * m)], "thickness")
        self.assertEqual(table[DocParam.count(3)], "ribs")


class TestSketchPlaneFrame(unittest.TestCase):
    """LIB-PYBUNDLE rider (b): the plane's frame reads back, and the
    equality that read-back supports is BIT-exact — Rust's
    `SketchPlane::bit_eq`, crossing unchanged."""

    def test_the_named_frames_read_back_as_the_cyclic_convention(self):
        self.assertEqual(SketchPlane.xy().normal, (0.0, 0.0, 1.0))
        self.assertEqual(SketchPlane.yz().u, (0.0, 1.0, 0.0))
        self.assertEqual(SketchPlane.yz().normal, (1.0, 0.0, 0.0))
        self.assertEqual(SketchPlane.zx().normal, (0.0, 1.0, 0.0))

    def test_a_frame_round_trips_through_its_accessors(self):
        frame = SketchPlane.from_frame(
            (1 * m, 2 * m, 3 * m), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)
        )
        self.assertEqual([c.meters for c in frame.origin], [1.0, 2.0, 3.0])
        rebuilt = SketchPlane.from_frame(frame.origin, frame.u, frame.v)
        self.assertEqual(frame, rebuilt)
        self.assertEqual(hash(frame), hash(rebuilt))

    def test_equality_is_bit_exact_not_tolerant(self):
        """The `Doc.bit_eq` precedent: a sketch plane carries no
        epsilon, so `-0.0` keeps its own identity rather than being
        quietly folded into `0.0`."""
        plus = SketchPlane.from_frame(
            (0.0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)
        )
        minus = SketchPlane.from_frame(
            (-0.0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0), (0.0, 1.0, 0.0)
        )
        self.assertNotEqual(plus, minus)
        self.assertEqual(plus, SketchPlane.xy())


class TestDatumReadback(unittest.TestCase):
    """What a datum's numbers cross AS. A position carries `Length`
    whichever frame it is written in; a direction is dimensionless and
    crosses bare. `Datum.in_plane` is the field that holds both, so it
    is where the rule is visible in one value."""

    def axis(self, x, y):
        doc = Doc()
        frame = doc.sketch_frame()
        node = doc.insert(Node.datum_axis_in_plane(frame, (x, y), (0.0, 1.0)))
        return evaluate(doc).value(node).datum()

    def test_the_in_plane_origin_reads_back_as_the_length_pair_it_was_written_as(self):
        """The write door takes `tuple[Length, Length]`; the read door
        answers the same. A frame-local coordinate changes the datum a
        position is measured from, not its dimension — so what goes in
        comes back out, equal and still typed."""
        datum = self.axis(0.25 * m, 0.5 * m)
        origin, direction = datum.in_plane
        self.assertIsInstance(origin[0], Length)
        self.assertIsInstance(origin[1], Length)
        self.assertEqual(origin, (0.25 * m, 0.5 * m))
        # The second pair is a DIRECTION: dimensionless, and bare.
        self.assertEqual(direction, (0.0, 1.0))
        self.assertNotIsInstance(direction[0], Length)

    def test_the_world_origin_of_the_same_axis_is_dimensioned_too(self):
        """The sibling field, so the class is read as one convention
        rather than two: both origins are positions and both carry
        `Length`."""
        datum = self.axis(0.25 * m, 0.5 * m)
        self.assertEqual(datum.kind, "axis_in_plane")
        for coordinate in datum.origin:
            self.assertIsInstance(coordinate, Length)
        # The sketch frame is the world xy plane, so the two spellings
        # of the same point agree coordinate for coordinate.
        self.assertEqual(datum.origin, (0.25 * m, 0.5 * m, 0 * m))


class TestDatumPointAndFrame(unittest.TestCase):
    """The two arms a Python author could not write. Six of the six
    datum kinds now have a `Node.datum_*` constructor; each of these
    two reads back through `Value.datum()`, and each is here because a
    downstream door consumes it — a selection measures its distance to
    a point, a profile is drawn on a frame."""

    def test_a_point_reads_back_as_its_position_and_faces_no_way(self):
        doc = Doc()
        point = doc.insert(Node.datum_point((1 * m, 2 * m, 3 * m)))
        datum = evaluate(doc).value(point).datum()
        self.assertEqual(datum.kind, "point")
        for coordinate in datum.origin:
            self.assertIsInstance(coordinate, Length)
        self.assertEqual(datum.origin, (1 * m, 2 * m, 3 * m))
        # The fields the other kinds fill are `None`, not a zero triple
        # standing in for a direction a point does not have.
        self.assertIsNone(datum.direction)
        self.assertIsNone(datum.axes)
        self.assertIsNone(datum.in_plane)

    def test_a_selection_measures_its_distance_to_a_point(self):
        """The downstream door: `GeomPred.datum_distance` is UNSIGNED
        to a point, so a point is how a rule says "near here" without
        naming a face."""
        doc = Doc()
        cube = unit_box(doc, 1 * m, 1 * m, 1 * m)
        # On the bottom cap's centroid, a metre under the top cap's.
        here = doc.insert(Node.datum_point((0.5 * m, 0.5 * m, 0 * m)))
        ev = evaluate(doc)
        faces = Selector.of(NamePat.of_kind(EntityKind.Face))
        on_it = ev.select_where(cube, faces, [GeomPred.datum_distance(here, Cmp.Approx, 0 * m)])
        far = ev.select_where(cube, faces, [GeomPred.datum_distance(here, Cmp.Greater, 0.9 * m)])
        self.assertEqual(len(on_it), 1)
        self.assertEqual(len(far), 1)
        self.assertNotEqual(on_it, far)

    def test_a_frame_reads_back_as_an_orthonormalized_pair(self):
        """`u` is KEPT as written and `v` yields its component along it,
        so a pair that is merely not perpendicular is legal and the
        axes come back unit."""
        doc = Doc()
        frame = doc.insert(
            Node.datum_frame((0 * m, 0 * m, 1 * m), (1.0, 0.0, 0.0), (1.0, 2.0, 0.0))
        )
        datum = evaluate(doc).value(frame).datum()
        self.assertEqual(datum.kind, "frame")
        self.assertEqual(datum.origin, (0 * m, 0 * m, 1 * m))
        self.assertIsNone(datum.in_plane)
        u, v = datum.axes
        # An axis is a DIRECTION: dimensionless, and bare.
        self.assertNotIsInstance(u[0], Length)
        self.assertEqual(u, (1.0, 0.0, 0.0))
        for got, want in zip(v, (0.0, 1.0, 0.0), strict=True):
            self.assertAlmostEqual(got, want)
        # The normal rides ALONGSIDE the axes: u x v, so a reader
        # asking which way the frame faces gets a plane's answer.
        for got, want in zip(datum.direction, (0.0, 0.0, 1.0), strict=True):
            self.assertAlmostEqual(got, want)

    def test_a_profile_is_drawn_on_an_authored_frame(self):
        """The downstream door, and the frame's tilt showing in the
        body: the same square extruded on a frame leaning 45 degrees
        puts material above the metre the world-xy version tops out
        at."""
        ground = (0 * m, 0 * m, 0 * m)
        corners = [(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)]

        def prism(plane_node, doc):
            square = doc.insert(Node.polygon(corners, plane=plane_node))
            return doc.insert(Node.extrude(square, 1 * m))

        doc = Doc()
        tilted = doc.insert(Node.datum_frame(ground, (1.0, 0.0, 0.0), (0.0, 1.0, 1.0)))
        leaning = prism(tilted, doc)
        upright = prism(doc.sketch_frame(), doc)
        floor = doc.insert(Node.datum_plane(ground, (0.0, 0.0, 1.0)))
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(leaning))
        # A rigid tilt is volume-preserving; where the material SITS is
        # what moved.
        self.assertAlmostEqual(
            ev.value(leaning).body().mass_properties().volume, 1.0, delta=1e-9
        )
        faces = Selector.of(NamePat.of_kind(EntityKind.Face))
        above = [GeomPred.datum_distance(floor, Cmp.Greater, 1 * m)]
        self.assertEqual(ev.select_where(upright, faces, above), [])
        self.assertNotEqual(ev.select_where(leaning, faces, above), [])

    def test_a_frame_whose_axes_span_no_plane_refuses_at_evaluate(self):
        """Gram-Schmidt states "these two span no plane" as a LENGTH,
        so the refusal is the direction one every datum raises, naming
        which axis went."""
        for u, v, axis in (
            ((0.0, 0.0, 0.0), (0.0, 1.0, 0.0), "x"),
            ((1.0, 0.0, 0.0), (2.0, 0.0, 0.0), "y"),
        ):
            with self.subTest(axis=axis):
                doc = Doc()
                bad = doc.insert(Node.datum_frame((0 * m, 0 * m, 0 * m), u, v))
                with self.assertRaises(EvaluationError) as caught:
                    evaluate(doc).value(bad)
                self.assertEqual(caught.exception.kind, "degenerate_direction")
                self.assertIn(f"datum frame {axis} axis", str(caught.exception))

    def test_a_non_finite_coordinate_refuses_at_the_door(self):
        """Both doors build literals, so the kernel's own literal
        refusal arrives where the call was written, carrying the
        offending number."""
        with self.assertRaises(pncad.LiteralError) as caught:
            Node.datum_point((float("nan") * m, 0 * m, 0 * m))
        self.assertEqual(caught.exception.kind, "non_finite")
        with self.assertRaises(pncad.LiteralError) as caught:
            Node.datum_frame(
                (0 * m, 0 * m, 0 * m), (float("inf"), 0.0, 0.0), (0.0, 1.0, 0.0)
            )
        self.assertEqual(caught.exception.value, float("inf"))


class TestBooleanDeclareArgument(unittest.TestCase):
    """LIB-PYBUNDLE rider (c): `Node.boolean` grew `declare=`, the
    DATA door for a declared contact. The protocol that BUILDS a
    declaration is still unbound, so the only thing the argument can
    be handed today is another node — and the EDIT door refuses one
    that is not a `Declare`, typed, rather than ignoring it or letting
    a document carry the mis-wire to its evaluation."""

    def test_the_default_is_the_undeclared_lane(self):
        doc = Doc()
        a = unit_box(doc, 1 * m, 1 * m, 1 * m)
        b = slab(doc, (0.5 * m, 1.5 * m), (0.5 * m, 1.5 * m), (0.5 * m, 1.5 * m))
        fused = doc.insert(Node.boolean(BooleanOp.Union, a, b))
        self.assertTrue(evaluate(doc).succeeded(fused))

    def test_a_non_declaration_input_is_refused_not_ignored(self):
        doc = Doc()
        a = unit_box(doc, 1 * m, 1 * m, 1 * m)
        b = slab(doc, (0.5 * m, 1.5 * m), (0.5 * m, 1.5 * m), (0.5 * m, 1.5 * m))
        # The declare edge names a THIRD node, not one of the operands:
        # a node's inputs are pairwise distinct (DM5), so pointing it at
        # `a` is refused at the edit door and never reaches the
        # evaluation this row is about. Any live non-`Declare` node
        # makes the same point.
        c = slab(doc, (5 * m, 6 * m), (5 * m, 6 * m), (5 * m, 6 * m))
        with self.assertRaises(EditError) as caught:
            doc.insert(Node.boolean(BooleanOp.Union, a, b, declare=c))
        self.assertEqual(caught.exception.variant, "declare_input_not_declare")
        self.assertIn("is not a declaration", str(caught.exception))


class TestTheInnerArmBesideTheOpWord(unittest.TestCase):
    """The two words a refusal carries, from real documents.

    `kind` is the CARRIER's discriminant — which door refused — and
    `inner_kind` is the arm of the kernel refusal that door holds.
    They are two enums, not one division stored twice, and the rows
    below are what makes that observable: one op, two refusals, the
    same `kind` and different `inner_kind`s. A caller branching on the
    op ladder is untouched by the second word's arrival; a caller that
    needs to tell a bad angle from a bad axis no longer has to read
    prose.
    """

    @staticmethod
    def square(doc, frame, x0=0.0, side=1.0):
        chain = (
            Open.at((x0 * m, 0 * m))
            .line_to(((x0 + side) * m, 0 * m))
            .line_to(((x0 + side) * m, side * m))
            .line_to((x0 * m, side * m))
            .line_to(Start)
        )
        return doc.insert(Node.profile(chain, plane=frame))

    def revolved(self, x0, angle):
        """A square revolved about the sketch's own +y through 0."""
        doc = Doc()
        frame = doc.sketch_frame()
        profile = self.square(doc, frame, x0)
        axis = doc.insert(
            Node.datum_axis_in_plane(frame, (0 * m, 0 * m), (0.0, 1.0))
        )
        node = doc.insert(Node.revolve(profile, axis, angle))
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(node)
        return caught.exception

    def test_one_op_two_refusals_one_word_apart(self):
        # A zero sweep and a profile straddling the axis are different
        # faults with different repairs. Before the inner word they
        # were both `revolve` and differed only in prose.
        angle = self.revolved(1.0, 0 * rad)
        self.assertEqual((angle.kind, angle.inner_kind), ("revolve", "degenerate_angle"))
        across = self.revolved(-0.5, 2 * math.pi * rad)
        self.assertEqual(
            (across.kind, across.inner_kind), ("revolve", "vertex_crosses_axis")
        )
        # The op word did not move, which is the half of the ruling
        # that says why this is a second attribute and not a longer
        # first one.
        self.assertEqual(angle.kind, across.kind)
        self.assertNotEqual(angle.inner_kind, across.inner_kind)

    def test_a_second_op_speaks_its_own_arms(self):
        doc = Doc()
        frame = doc.sketch_frame()
        flat = doc.insert(Node.extrude(self.square(doc, frame), 0 * m))
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(flat)
        self.assertEqual(
            (caught.exception.kind, caught.exception.inner_kind),
            ("extrude", "degenerate_extrusion"),
        )

    def test_an_arm_with_no_inner_refusal_says_none(self):
        # The undeclared-contact refusal is the document layer's own
        # arm: its payload is the candidate declaration, which crosses
        # whole as `finding`, and there is no inner enum to name.
        doc = Doc()
        outer = unit_box(doc, 2 * m, 2 * m, 2 * m)
        inner = unit_box(doc, 1 * m, 1 * m, 1 * m)
        cut = doc.insert(Node.boolean(BooleanOp.Subtract, outer, inner))
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(cut)
        self.assertEqual(caught.exception.kind, "undeclared_contact")
        self.assertIsNone(caught.exception.inner_kind)
        self.assertIsNotNone(caught.exception.finding)

    def test_a_poisoned_node_carries_both_of_its_ancestors_words(self):
        # The poisoning path reports the ROOT cause, so it reports both
        # of the root cause's words or neither.
        doc = Doc()
        frame = doc.sketch_frame()
        flat = doc.insert(Node.extrude(self.square(doc, frame), 0 * m))
        moved = doc.insert(
            Node.transform(flat, (0 * m, 0 * m, 1 * m), (0.0, 0.0, 1.0), 0 * rad)
        )
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(moved)
        self.assertEqual(caught.exception.reason, "poisoned")
        self.assertEqual(caught.exception.through, flat)
        self.assertEqual(
            (caught.exception.kind, caught.exception.inner_kind),
            ("extrude", "degenerate_extrusion"),
        )

    def test_the_edit_door_carries_the_same_pair(self):
        """`EditError` is the second carrier, and reads the same way.

        The placement frame's axis door is the reachable one: its
        refusal IS an evaluation refusal, so `inner_variant` speaks
        the vocabulary `EvaluationError.kind` speaks — a zero axis and
        a poisoned one are different repairs, scale versus direction.
        """
        with self.assertRaises(EditError) as zero:
            Frame.rotate_then_translate((0.0, 0.0, 0.0), 1 * rad, (0 * m, 0 * m, 0 * m))
        self.assertEqual(
            (zero.exception.variant, zero.exception.inner_variant),
            ("placement_axis", "degenerate_direction"),
        )
        with self.assertRaises(EditError) as poisoned:
            Frame.rotate_then_translate(
                (float("inf"), 0.0, 0.0), 1 * rad, (0 * m, 0 * m, 0 * m)
            )
        self.assertEqual(
            (poisoned.exception.variant, poisoned.exception.inner_variant),
            ("placement_axis", "non_finite_direction"),
        )

    def test_an_edit_arm_with_no_inner_refusal_says_none(self):
        doc = Doc()
        frame = doc.sketch_frame()
        profile = self.square(doc, frame)
        doc.insert(Node.extrude(profile, 1 * m))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.delete_node(profile))
        self.assertEqual(caught.exception.variant, "delete_would_dangle")
        self.assertIsNone(caught.exception.inner_variant)

    def test_the_edit_arms_that_hold_a_refusal_are_pre_checked_elsewhere(self):
        """Why the rows above reach ONE inner word and not four.

        `EditError` has five arms carrying an inner refusal. The
        placement axis is the only one an authoring caller can reach:
        each of the others is refused at a NARROWER door first, which
        is the fail-loud shape working — the refusal a caller gets
        names the thing they typed. Pinned here so that a door
        widening later shows up as this test failing rather than as a
        silent hole in the map.
        """
        doc = Doc()
        # A distribution's invariants are checked by its own
        # constructor, so `invalid_distribution` cannot be inserted.
        with self.assertRaises(pncad.DistributionFault):
            Distribution.normal(0 * m)
        # A measured expression's reference indices are checked by
        # `Node.measure`, so `measure_malformed` cannot be inserted.
        with self.assertRaises(pncad.MeasureNodeFault):
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.distance(0, 5)), []
            )
        # A profile program's geometry is checked by the PATHS chain,
        # so `profile_program_refused` cannot be inserted.
        with self.assertRaises(pncad.PathError):
            (
                Open.at((0 * m, 0 * m))
                .line_to((1 * m, 0 * m))
                .toward(0.0, 1.0)
                .fillet(50 * m)
                .toward(-1.0, 0.0)
                .to((0 * m, 1 * m))
                .line_to(Start)
            )
        # `dimension` rides the expression-path edit and `roots` reads
        # its word off the fault already — neither has a Python door
        # that could carry a second one.
        self.assertFalse(hasattr(DocEdit, "set_expr_at"))
        self.assertEqual(len(doc.order()), 0)


#: Every attribute an `EditError` carries, in publication order — the
#: whole of the class's shape, and what "present on every arm" is a
#: claim about.
EDIT_ATTRS = (
    "variant",
    "inner_variant",
    "node",
    "input",
    "referenced_by",
    "slot",
    "param",
    "name",
    "key",
    "expected",
    "found",
    "kind",
    "from_kind",
    "to_kind",
    "count",
    "first",
    "again",
    "value",
    "offered",
    "determinant",
    "path",
    "value_path",
    "pin",
)


class TestTheEditDoorsPayload(unittest.TestCase):
    """The refusing arm's payload, off real edits.

    The document layer's `EditError` has 58 arms and most have no
    Python door — a rebind, a witness, an appearance write and an
    expression-path edit are not among the ten `DocEdit` verbs. What
    the rows below pin is the half a Python caller can provoke: the
    payload arrives as attributes, the ids are the ids that were used,
    and the words are stable words rather than prose sliced out of the
    message. The arms with no door are pinned by construction in
    `src/tests.rs`, where they can be built.
    """

    @staticmethod
    def slab(doc, x, y, z):
        x0, x1 = x
        y0, y1 = y
        z0, z1 = z
        profile = doc.insert(
            Node.polygon(
                [(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
                plane=doc.sketch_frame(elevation=z0),
            )
        )
        return doc.insert(Node.extrude(profile, z1 - z0))

    def set_of(self, refusal):
        """The attributes this refusal CARRIES, with the rest asserted
        present and `None` — the two halves of the rule in one read."""
        for attr in EDIT_ATTRS:
            self.assertTrue(
                hasattr(refusal, attr),
                f"`{attr}` is missing from a {refusal.variant} refusal",
            )
        return {a for a in EDIT_ATTRS if getattr(refusal, a) is not None}

    def test_the_two_node_roles_answer_with_the_ids_that_were_used(self):
        # A delete that would dangle names BOTH ends: the node asked
        # for, and the live node still reading it. They are different
        # roles, so folding them into one attribute would lose which
        # is which — and the pair is what a caller needs to build the
        # cascade.
        doc = Doc()
        box = self.slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        profile = doc.order()[1]
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.delete_node(profile))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "delete_would_dangle")
        self.assertEqual(refusal.node, profile)
        self.assertEqual(refusal.referenced_by, box)
        self.assertEqual(self.set_of(refusal), {"variant", "node", "referenced_by"})

    def test_a_foreign_id_arrives_under_the_role_the_door_read_it_in(self):
        # The SAME id, refused at two doors, lands under two different
        # attributes: as the target of a read (`node`) and as the
        # unresolvable operand of an insert (`input`). The role is the
        # answer, not the id.
        doc = Doc()
        self.slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        other = Doc()
        for _ in range(4):
            self.slab(other, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        stray = other.order()[-1]

        with self.assertRaises(EditError) as read:
            doc.node_kind(stray)
        self.assertEqual(read.exception.variant, "unknown_node")
        self.assertEqual(read.exception.node, stray)
        self.assertIsNone(read.exception.input)
        self.assertEqual(self.set_of(read.exception), {"variant", "node"})

        with self.assertRaises(EditError) as written:
            doc.insert(Node.extrude(stray, 1 * m))
        self.assertEqual(written.exception.variant, "unresolved_input")
        self.assertEqual(written.exception.input, stray)
        self.assertIsNone(written.exception.node)
        self.assertEqual(self.set_of(written.exception), {"variant", "input"})

    def test_a_parameter_binding_names_the_node_the_slot_and_the_param(self):
        # A slot is a NAME, never an index: `count` is the word, and it
        # is the same word whichever door refused at it.
        doc = Doc()
        box = self.slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        pattern = doc.insert(
            Node.pattern(box, 3, PatternKind.linear((1.0, 0.0, 0.0), 2 * m))
        )
        with self.assertRaises(EditError) as unknown:
            doc.apply(DocEdit.bind_count_param(pattern, ParamName("n")))
        self.assertEqual(unknown.exception.variant, "unknown_doc_param")
        self.assertEqual(unknown.exception.node, pattern)
        self.assertEqual(unknown.exception.slot, "count")
        self.assertEqual(unknown.exception.param, "n")
        self.assertEqual(
            self.set_of(unknown.exception), {"variant", "node", "slot", "param"}
        )

        # A node with no count slot at all refuses at the same door and
        # names the slot it lacks.
        with self.assertRaises(EditError) as absent:
            doc.apply(DocEdit.bind_count_param(box, ParamName("n")))
        self.assertEqual(absent.exception.variant, "unknown_slot")
        self.assertEqual(absent.exception.slot, "count")
        self.assertEqual(self.set_of(absent.exception), {"variant", "node", "slot"})

    def test_the_dimension_pair_crosses_as_words_not_prose(self):
        # `expected` and `found` are the dimension the door required
        # and the one it was offered, in the alphabet `Expr.dimension`
        # answers in — one pair however the kernel's arm spells it.
        doc = Doc()
        box = self.slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        pattern = doc.insert(
            Node.pattern(box, 3, PatternKind.linear((1.0, 0.0, 0.0), 2 * m))
        )
        doc.apply(DocEdit.set_doc_param(ParamName("len"), DocParam.length(1 * m)))
        with self.assertRaises(EditError) as caught:
            doc.apply(DocEdit.bind_count_param(pattern, ParamName("len")))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "doc_param_dimension_mismatch")
        self.assertEqual(refusal.expected, "length")
        self.assertEqual(refusal.found, "count")
        self.assertEqual(
            self.set_of(refusal),
            {"variant", "node", "slot", "param", "expected", "found"},
        )

        # The value door's own mismatch spells the offered VALUE
        # beside the declared dimension — an exact `int`, because a
        # count is exact and rounding it into a float would be the
        # fabrication this surface refuses elsewhere.
        with self.assertRaises(EditError) as offered:
            doc.apply(
                DocEdit.set_doc_param_value(ParamName("len"), DocParamValue.count(3))
            )
        self.assertEqual(offered.exception.variant, "doc_param_value_kind_mismatch")
        self.assertEqual(offered.exception.expected, "length")
        self.assertEqual(offered.exception.offered, 3)
        self.assertIsInstance(offered.exception.offered, int)
        self.assertEqual(
            self.set_of(offered.exception), {"variant", "param", "expected", "offered"}
        )

    def test_a_refused_scalar_and_a_root_pair_cross_as_themselves(self):
        doc = Doc()
        box = self.slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        with self.assertRaises(EditError) as eps:
            doc.apply(DocEdit.set_tolerance(-1.0))
        self.assertEqual(eps.exception.variant, "invalid_tolerance")
        self.assertEqual(eps.exception.value, -1.0)
        self.assertEqual(self.set_of(eps.exception), {"variant", "value"})

        # A root that is an ancestor of another root reads the same
        # two node roles a dangling delete does: the offender, and the
        # node downstream that references it. `variant` is the FAULT's
        # word already, so `inner_variant` stays `None`.
        pattern = doc.insert(
            Node.pattern(box, 3, PatternKind.linear((1.0, 0.0, 0.0), 2 * m))
        )
        with self.assertRaises(EditError) as roots:
            doc.apply(DocEdit.set_roots([box, pattern]))
        self.assertEqual(roots.exception.variant, "root_ancestor")
        self.assertEqual(roots.exception.node, box)
        self.assertEqual(roots.exception.referenced_by, pattern)
        self.assertIsNone(roots.exception.inner_variant)
        self.assertEqual(
            self.set_of(roots.exception), {"variant", "node", "referenced_by"}
        )

    def test_an_arm_with_no_payload_answers_none_all_the_way_down(self):
        # The placement axis refuses with an inner word and NOTHING
        # else: the axis it was given is not a document node, a slot
        # or a name, so every payload attribute is present and `None`.
        # `getattr` still answers, which is the whole point.
        with self.assertRaises(EditError) as caught:
            Frame.rotate_then_translate(
                (0.0, 0.0, 0.0), 1 * rad, (0 * m, 0 * m, 0 * m)
            )
        refusal = caught.exception
        self.assertEqual(refusal.variant, "placement_axis")
        self.assertEqual(refusal.inner_variant, "degenerate_direction")
        self.assertEqual(self.set_of(refusal), {"variant", "inner_variant"})

    def test_the_declare_sugars_own_arms_carry_the_shape_and_no_payload(self):
        # `DeclareError` is a second raise site of this class, and its
        # own two arms hold no document-layer payload — so they answer
        # `None` for all of it rather than dropping the attributes a
        # caller reads without branching.
        doc = Doc()
        with self.assertRaises(EditError) as caught:
            doc.declare_all([])
        self.assertEqual(caught.exception.variant, "no_findings")
        self.assertEqual(self.set_of(caught.exception), {"variant"})

    def test_a_refusal_the_boundary_builds_has_the_same_shape(self):
        # `Node.placed_union` refuses BEFORE the document layer sees
        # the edit — the boundary decides it — and the exception is
        # still one shape: every attribute present, `None` where this
        # refusal carries nothing.
        doc = Doc()
        box = self.slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        with self.assertRaises(EditError) as caught:
            Node.placed_union(box, 3, PatternKind.explicit([]))
        self.assertEqual(caught.exception.variant, "placement_rule_mismatch")
        self.assertEqual(self.set_of(caught.exception), {"variant"})
