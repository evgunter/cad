"""The document surface, and the D9 bit-precision seed.

§L3: Python speaks Doc/DocEdit/evaluate — never an arena key. Every
test here goes through a document; none reaches into the kernel.
"""

import struct
import unittest

import pncad
from pncad import (
    BooleanOp,
    Doc,
    DocEdit,
    DocParam,
    EditError,
    EvaluationError,
    Node,
    SketchPlane,
    evaluate,
    import_step,
    load,
    m,
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
            elevation=z0,
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
        self.assertEqual(doc.node_count, 2)
        self.assertIn(box, doc.order())

    def test_edits_go_through_the_docedit_vocabulary(self):
        doc = Doc()
        minted = doc.apply(DocEdit.insert_node(Node.polygon([(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m)])))
        self.assertIsNotNone(minted)
        self.assertEqual(doc.node_count, 1)

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


class TestEvaluation(unittest.TestCase):
    def test_evaluate_returns_typed_per_node_values(self):
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 1 * m)
        ev = evaluate(doc)
        profile_node, extrude_node = doc.order()
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
    """LIB-DOORS F1: the schema-v4 doors, through the curated facade."""

    def test_save_load_evaluate_round_trip_is_bit_exact(self):
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 0.5 * m)
        before = evaluate(doc).value(box).body().mass_properties().volume

        text = doc.save()
        schema = pncad.__build_info__["schema_version"]
        self.assertTrue(
            text.startswith(f"schema: {schema}\n"),
            "the file speaks the build's own schema version",
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
        self.assertEqual(caught.exception.variant, "header")

    def test_an_unknown_schema_is_a_typed_refusal(self):
        with self.assertRaises(pncad.PersistError) as caught:
            load("schema: 9999\n{}")
        self.assertEqual(caught.exception.variant, "unknown_schema")


class TestStepExport(unittest.TestCase):
    """LIB-DOORS F2: the document-layer export door and its oracle."""

    def test_export_reimports_with_the_same_volume(self):
        doc = Doc()
        box = unit_box(doc, 2 * m, 3 * m, 0.5 * m)
        ev = evaluate(doc)
        step = ev.step_string(box, product_name="doors-box")
        self.assertIn("ISO-10303-21", step)
        # The oracle is the kernel's own importer: the text PARSES and
        # adopts as a first-class solid whose volume agrees.
        body = import_step(step)
        volume = body.mass_properties().volume
        self.assertAlmostEqual(volume, 3.0, places=9)

    def test_export_of_a_profile_is_a_typed_refusal(self):
        doc = Doc()
        unit_box(doc, 1 * m, 1 * m, 1 * m)
        profile_node = doc.order()[0]
        ev = evaluate(doc)
        with self.assertRaises(pncad.ExportError) as caught:
            ev.step_string(profile_node)
        self.assertEqual(caught.exception.variant, "not_a_body")
        self.assertEqual(caught.exception.kind, "profile")

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


class TestBooleanDeclareArgument(unittest.TestCase):
    """LIB-PYBUNDLE rider (c): `Node.boolean` grew `declare=`, the
    DATA door for a declared contact. The protocol that BUILDS a
    declaration is still unbound, so the only thing the argument can
    be handed today is another node — and the kernel refuses one that
    is not a `Declare`, typed, rather than ignoring it."""

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
        fused = doc.insert(Node.boolean(BooleanOp.Union, a, b, declare=a))
        with self.assertRaises(EvaluationError) as caught:
            evaluate(doc).value(fused)
        self.assertEqual(caught.exception.kind, "wrong_operand")
