"""The document surface, and the D9 bit-precision seed.

§L3: Python speaks Doc/DocEdit/evaluate — never an arena key. Every
test here goes through a document; none reaches into the kernel.
"""

import struct
import unittest

import pncad
from pncad import BooleanOp, Doc, DocEdit, EvaluationError, Node, evaluate, m, mm


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

    def test_a_coincident_boolean_is_refused_not_guessed(self):
        # Fail-loud, visible from Python: two boxes sharing the z=0
        # plane are NOT silently fused.
        doc = Doc()
        outer = unit_box(doc, 2 * m, 2 * m, 2 * m)
        inner = unit_box(doc, 1 * m, 1 * m, 1 * m)
        cut = doc.insert(Node.boolean(BooleanOp.Subtract, outer, inner))
        ev = evaluate(doc)
        self.assertFalse(ev.succeeded(cut))
        with self.assertRaises(EvaluationError) as caught:
            ev.value(cut)
        # FINDING (see the PR body): the reason is `no_value`, not the
        # node's actual typed refusal — no curated path leads from an
        # Evaluation to its NodeError yet.
        self.assertEqual(caught.exception.reason, "no_value")


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
            doc = Doc()
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
