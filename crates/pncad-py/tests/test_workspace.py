"""The workspace store, content pins, and cross-document references.

The audit's G15 row said what Python could not do: "it cannot open a
directory of them, resolve a `DocRef`, compute or compare a
`ContentPin`". This file is the positive form of that sentence, and
the oracle it is written against is `demos/tour/src/assembly.rs` —
the tour's own store, walked door for door: two part documents
written side by side, each part's `(id, pin)` reference taken off the
document that was just written, a pin that MOVED refusing rather than
retargeting, and the store's uniqueness invariant refusing two files
that claim one identity.

What is deliberately NOT here, because it is not bound: the assembly
half. No `InstantiatePart`, no mate, no `update_to_store` — a
reference SITE inside a document is what those need, and
`evaluate(doc)` takes no resolver, so a document that references
another has nothing to resolve against from Python. `test_north_star`
pins that absence; this file pins the half that landed.
"""

import hashlib
import os
import shutil
import tempfile
import unittest

import pncad
from pncad import (
    ContentPin,
    DocRef,
    Doc,
    Node,
    PersistError,
    Workspace,
    WorkspaceError,
    canonical_bytes,
    content_pin,
    evaluate,
    header_document_id,
    m,
    random_document_id,
)


def box(doc, width, depth, height):
    """A rectangular prism rooted at the origin, so a document has
    content whose pin can move when the content does."""
    profile = doc.insert(
        Node.polygon(
            [(0 * m, 0 * m), (width, 0 * m), (width, depth), (0 * m, depth)],
            elevation=0 * m,
        )
    )
    return doc.insert(Node.extrude(profile, height))


class StoreCase(unittest.TestCase):
    """A fresh directory per test — a store that accumulated another
    test's documents would resolve a pin nobody wrote."""

    def setUp(self):
        self.dir = tempfile.mkdtemp(prefix="pncad-workspace-")
        self.addCleanup(shutil.rmtree, self.dir, ignore_errors=True)

    def store(self):
        return Workspace(self.dir)


class TestTheStoreHoldsDocumentsSideBySide(StoreCase):
    """G15's own sentence, executed: two documents a workspace will
    accept side by side."""

    def test_two_documents_are_two_parts_and_the_store_holds_both(self):
        ws = self.store()
        self.assertEqual(len(ws), 0)
        post, shelf = Doc(), Doc()
        self.assertNotEqual(post.id, shelf.id)
        box(post, 1 * m, 1 * m, 4 * m)
        box(shelf, 3 * m, 2 * m, 1 * m)
        post_path = ws.create(post)
        shelf_path = ws.create(shelf)
        self.assertEqual(len(ws), 2)

        # The file name is a pure function of the identity, so two
        # runs write the same store.
        self.assertEqual(os.path.basename(post_path), f"{post.id}.pncad")
        self.assertEqual(os.path.basename(shelf_path), f"{shelf.id}.pncad")
        self.assertEqual(
            ws.documents(), {post.id: post_path, shelf.id: shelf_path}
        )
        self.assertEqual(ws.root, self.dir)

        # A SECOND open reads the same store off the same headers —
        # the scan is the store, not a handle held open.
        self.assertEqual(self.store().documents(), ws.documents())

    def test_the_scan_ignores_what_does_not_claim_to_be_a_document(self):
        ws = self.store()
        doc = Doc()
        box(doc, 1 * m, 1 * m, 1 * m)
        ws.create(doc)
        with open(os.path.join(self.dir, "notes.txt"), "w") as f:
            f.write("not a save file")
        os.mkdir(os.path.join(self.dir, "subdir"))
        self.assertEqual(len(self.store()), 1)


class TestAReferenceNamesAVersion(StoreCase):
    """`DocRef` is `(which part, which version of it)`, and resolving
    one is the store checking the second half."""

    def test_a_reference_resolves_to_the_document_it_pins(self):
        ws = self.store()
        doc = Doc()
        node = box(doc, 2 * m, 3 * m, 1 * m)
        ws.create(doc)
        reference = DocRef(doc.id, content_pin(doc))
        self.assertEqual(reference.id, doc.id)

        resolved = ws.resolve(reference)
        self.assertEqual(resolved.id, doc.id)
        self.assertEqual(resolved.node_count, doc.node_count)
        # The pin holds through the full door sequence a resolve runs
        # — parse, validate, replay, epsilon reconciliation.
        self.assertEqual(content_pin(resolved), reference.pin)
        # And the geometry survives the round trip, which is the claim
        # a pin is standing in for.
        volume = evaluate(resolved).value(node).body().mass_properties().volume
        self.assertEqual(volume, 6.0)

    def test_a_pin_that_moved_refuses_rather_than_retargeting(self):
        """Cargo.lock semantics, the store's whole reason for typing
        this arm: a reference names a VERSION, so a document edited
        under it refuses and hands back both pins."""
        ws = self.store()
        doc = Doc()
        box(doc, 2 * m, 3 * m, 1 * m)
        ws.create(doc)
        pinned = DocRef(doc.id, content_pin(doc))

        box(doc, 1 * m, 1 * m, 1 * m)
        ws.resave(doc)
        moved = content_pin(doc)
        self.assertNotEqual(moved, pinned.pin)

        with self.assertRaises(WorkspaceError) as caught:
            ws.resolve(pinned)
        err = caught.exception
        self.assertEqual(err.variant, "pin_mismatch")
        self.assertEqual(err.id, doc.id)
        self.assertEqual(err.wanted, pinned.pin)
        self.assertEqual(err.found, moved)
        self.assertEqual(err.path, ws.documents()[doc.id])
        # The recourse is prose the library owns, not prose this test
        # restates.
        self.assertIn(pncad.PIN_MISMATCH_RECOURSE, str(err))

        # The identity did not move, so the store still holds the
        # document — and the CURRENT pin resolves.
        self.assertEqual(ws.current_pin(doc.id), moved)
        self.assertEqual(ws.resolve(DocRef(doc.id, moved)).node_count, 4)

    def test_exactly_one_of_two_pins_can_ever_resolve(self):
        """A workspace is one file per document id, so a stale
        reference and a current one cannot both be live at once."""
        ws = self.store()
        doc = Doc()
        box(doc, 1 * m, 1 * m, 1 * m)
        ws.create(doc)
        old = DocRef(doc.id, content_pin(doc))
        box(doc, 2 * m, 2 * m, 2 * m)
        ws.resave(doc)
        new = DocRef(doc.id, content_pin(doc))

        self.assertNotEqual(old, new)
        self.assertEqual(old.id, new.id)
        ws.resolve(new)
        with self.assertRaises(WorkspaceError) as caught:
            ws.resolve(old)
        self.assertEqual(caught.exception.variant, "pin_mismatch")

    def test_one_documents_id_with_anothers_pin_resolves_nowhere(self):
        ws = self.store()
        post, shelf = Doc(), Doc()
        box(post, 1 * m, 1 * m, 4 * m)
        box(shelf, 3 * m, 2 * m, 1 * m)
        ws.create(post)
        ws.create(shelf)
        crossed = DocRef(post.id, content_pin(shelf))
        with self.assertRaises(WorkspaceError) as caught:
            ws.resolve(crossed)
        self.assertEqual(caught.exception.variant, "pin_mismatch")


class TestThePinIsOfContent(StoreCase):
    """Identity answers WHICH PART; the pin answers WHICH VERSION.
    Two questions, two values, and the tests below are the difference
    between them."""

    def test_the_pin_is_the_sha256_of_the_canonical_bytes(self):
        doc = Doc()
        box(doc, 2 * m, 3 * m, 1 * m)
        digest = hashlib.sha256(canonical_bytes(doc)).hexdigest()
        self.assertEqual(content_pin(doc).hex, digest)
        self.assertEqual(len(digest), 64)

    def test_identity_survives_an_edit_and_the_pin_does_not(self):
        doc = Doc()
        box(doc, 1 * m, 1 * m, 1 * m)
        before_id, before_pin = doc.id, content_pin(doc)
        box(doc, 2 * m, 2 * m, 2 * m)
        self.assertEqual(doc.id, before_id)
        self.assertNotEqual(content_pin(doc), before_pin)

    def test_same_part_same_content_is_the_same_pin(self):
        """The derived spelling makes two documents the SAME part
        deliberately; authored alike, they are the same version too —
        canonical means canonical."""
        one, two = Doc("shelf"), Doc("shelf")
        box(one, 3 * m, 2 * m, 1 * m)
        box(two, 3 * m, 2 * m, 1 * m)
        self.assertEqual(one.id, two.id)
        self.assertEqual(content_pin(one), content_pin(two))

    def test_the_same_content_under_two_identities_pins_differently(self):
        """Identity is IN the canonical bytes, so a pin is never a
        content-only fingerprint two parts could share."""
        one, two = Doc("post"), Doc("shelf")
        box(one, 1 * m, 1 * m, 4 * m)
        box(two, 1 * m, 1 * m, 4 * m)
        self.assertNotEqual(content_pin(one), content_pin(two))

    def test_a_pin_is_a_value_that_compares_and_hashes(self):
        doc = Doc()
        box(doc, 1 * m, 1 * m, 1 * m)
        pin = content_pin(doc)
        self.assertEqual(ContentPin(pin.hex), pin)
        self.assertEqual(len({pin, ContentPin(pin.hex)}), 1)
        self.assertIn(pin.hex, repr(pin))
        self.assertEqual(str(pin), pin.hex)

    def test_a_pin_text_that_is_not_canonical_refuses_at_the_boundary(self):
        for text in ("", "abc", "A" * 64, "f" * 63, "g" * 64):
            with self.subTest(text=text), self.assertRaises(ValueError):
                ContentPin(text)


class TestTheReferenceIsAValue(StoreCase):
    def test_a_docref_compares_hashes_and_reads_back(self):
        doc = Doc()
        box(doc, 1 * m, 1 * m, 1 * m)
        pin = content_pin(doc)
        one, two = DocRef(doc.id, pin), DocRef(doc.id, pin)
        self.assertEqual(one, two)
        self.assertEqual(len({one, two}), 1)
        self.assertEqual(one.id, doc.id)
        self.assertEqual(one.pin, pin)

    def test_a_reference_can_name_a_version_no_store_holds(self):
        """Constructing one consults nothing — which is exactly what
        a stale reference IS."""
        doc = Doc()
        box(doc, 1 * m, 1 * m, 1 * m)
        DocRef(random_document_id(), content_pin(doc))

    def test_an_id_that_is_not_canonical_refuses_at_the_boundary(self):
        doc = Doc()
        pin = content_pin(doc)
        for text in ("", "abc", "F" * 32, "0" * 31, "z" * 32):
            with self.subTest(text=text), self.assertRaises(ValueError):
                DocRef(text, pin)


class TestIdentityMinting(StoreCase):
    def test_random_ids_are_canonical_and_distinct(self):
        first, second = random_document_id(), random_document_id()
        self.assertNotEqual(first, second)
        for got in (first, second):
            self.assertEqual(len(got), 32)
            self.assertEqual(got, got.lower())
            int(got, 16)

    def test_the_header_answers_a_saved_documents_id(self):
        """The store's scan door: the id without the body."""
        doc = Doc()
        box(doc, 1 * m, 1 * m, 1 * m)
        self.assertEqual(header_document_id(doc.save()), doc.id)

    def test_a_text_that_is_not_a_save_file_refuses_typed(self):
        with self.assertRaises(PersistError) as caught:
            header_document_id("not a save file")
        self.assertIsInstance(caught.exception.variant, str)


class TestTheStoreRefusesLoudly(StoreCase):
    """Every arm the bound doors can reach, and each carries its own
    payload rather than prose to be parsed."""

    def test_two_files_cannot_claim_one_identity(self):
        ws = self.store()
        one, two = Doc("shelf"), Doc("shelf")
        box(one, 3 * m, 2 * m, 1 * m)
        box(two, 9 * m, 9 * m, 9 * m)
        first = ws.create(one)
        with self.assertRaises(WorkspaceError) as caught:
            ws.create(two)
        err = caught.exception
        self.assertEqual(err.variant, "duplicate_id")
        self.assertEqual(err.id, one.id)
        # Both paths are `{id}.pncad`, because the name IS a function
        # of the identity: `first` is the file that already claims the
        # id and `second` the file this create would have written, and
        # for this door they are necessarily the same path.
        self.assertEqual(err.first, first)
        self.assertEqual(err.second, first)
        # Nothing was written, so the store still holds the FIRST
        # document's content.
        self.assertEqual(len(self.store()), 1)
        self.assertEqual(ws.current_pin(one.id), content_pin(one))

    def test_the_store_refuses_an_id_it_does_not_hold(self):
        ws = self.store()
        held = Doc()
        box(held, 1 * m, 1 * m, 1 * m)
        ws.create(held)
        absent = random_document_id()
        stale = DocRef(absent, content_pin(held))
        for name, door in (
            ("current_pin", lambda: ws.current_pin(absent)),
            ("resolve", lambda: ws.resolve(stale)),
        ):
            with self.subTest(door=name), self.assertRaises(WorkspaceError) as caught:
                door()
            self.assertEqual(caught.exception.variant, "unknown_id")
            self.assertEqual(caught.exception.id, absent)

    def test_resave_never_creates(self):
        ws = self.store()
        doc = Doc()
        box(doc, 1 * m, 1 * m, 1 * m)
        with self.assertRaises(WorkspaceError) as caught:
            ws.resave(doc)
        self.assertEqual(caught.exception.variant, "unknown_id")
        self.assertEqual(caught.exception.id, doc.id)
        self.assertEqual(len(self.store()), 0)

    def test_a_file_claiming_to_be_a_document_must_scan_clean(self):
        path = os.path.join(self.dir, "broken.pncad")
        with open(path, "w") as f:
            f.write("this has no header at all\n")
        with self.assertRaises(WorkspaceError) as caught:
            self.store()
        err = caught.exception
        self.assertEqual(err.variant, "header")
        self.assertEqual(err.path, path)

    def test_a_directory_that_is_not_there_refuses_naming_it(self):
        missing = os.path.join(self.dir, "no-such-store")
        with self.assertRaises(WorkspaceError) as caught:
            Workspace(missing)
        err = caught.exception
        self.assertEqual(err.variant, "io")
        self.assertEqual(err.path, missing)

    def test_every_arm_carries_every_attribute(self):
        """`None` where an arm does not apply, never absent: error
        handling reads `err.wanted` without branching on `variant`
        first."""
        ws = self.store()
        doc = Doc()
        box(doc, 1 * m, 1 * m, 1 * m)
        ws.create(doc)
        with self.assertRaises(WorkspaceError) as caught:
            ws.current_pin(random_document_id())
        err = caught.exception
        for field in ("variant", "path", "id", "first", "second", "wanted", "found"):
            self.assertTrue(hasattr(err, field), f"{field} is absent")
        self.assertIsNone(err.path)
        self.assertIsNone(err.wanted)
        self.assertIsNone(err.found)
        self.assertIsInstance(err, pncad.PncadError)


if __name__ == "__main__":
    unittest.main()
