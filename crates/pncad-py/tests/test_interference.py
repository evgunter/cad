"""An overlap between two copies at rest is a finding, not a refusal
(D10; INTENT stage 5 B).

`assemble` answers an `Assembly` whose `interference` lists every
overlap of two copies' material, one `InterferenceFinding` per
connected overlap, named by the faces of both copies that bound it.
The quieting rule's rows are Rust's
(`crates/editor-core/tests/intent_s5_b_interference.rs`); this file
pins what a Python caller reads.
"""

import shutil
import tempfile
import unittest
from pathlib import Path

import pncad
from pncad import Doc, DocRef, Formula, Node, Workspace, assemble, evaluate, m


def block(doc, corner, side, elevation, height):
    """A square prism of `side` with its low corner at `corner`, its
    base at `elevation`."""
    x, y = corner
    plane = doc.sketch_frame(elevation=Formula.length_in(elevation, m))
    outline = doc.insert(
        Node.polygon(
            [
                (Formula.length_in(x, m), Formula.length_in(y, m)),
                (Formula.length_in(x + side, m), Formula.length_in(y, m)),
                (Formula.length_in(x + side, m), Formula.length_in(y + side, m)),
                (Formula.length_in(x, m), Formula.length_in(y + side, m)),
            ],
            plane=plane,
        )
    )
    return doc.insert(Node.extrude(outline, Formula.length_in(height, m)))


class TestInterferenceIsAFinding(unittest.TestCase):
    def test_two_overlapping_copies_assemble_with_one_loud_finding(self):
        """A unit block and a smaller one sunk 0.05 into its +x side,
        strictly inside it across y and z: the gate answers, with one
        loud finding naming both copies and faces of each."""
        doc = Doc()
        a = block(doc, (0.0, 0.0), 1.0, 0.0, 1.0)
        b = block(doc, (0.95, 0.2), 0.6, 0.25, 0.5)
        doc.place(a)
        doc.place(b)
        assembly = assemble(doc, evaluate(doc))
        self.assertEqual(len(assembly.interference), 1)
        (finding,) = assembly.interference
        self.assertTrue(finding.loud)
        self.assertIsNone(finding.quiet_by)
        self.assertIsNone(finding.unlocalized)
        self.assertNotEqual(finding.a, finding.b)
        self.assertEqual(finding.a.kind, "body")
        faces = finding.faces
        self.assertIsNotNone(faces)
        copies = {site.copy for site in faces}
        self.assertEqual(copies, {finding.a, finding.b}, "both copies bound it")
        self.assertTrue(all(site.face for site in faces))

    def test_disjoint_copies_report_no_interference(self):
        doc = Doc()
        doc.place(block(doc, (0.0, 0.0), 1.0, 0.0, 1.0))
        doc.place(block(doc, (2.0, 0.0), 1.0, 0.0, 1.0))
        self.assertEqual(assemble(doc, evaluate(doc)).interference, [])


class TestAPartsOverlapIsThePartsFinding(unittest.TestCase):
    """A part crosses into the document instantiating it as one copy, so
    an overlap between the part's own copies lies inside one copy there:
    it is the part's finding, reported once on
    `Assembly.carried_interference` with the route it came by, and never
    raised (`work/intent/a-part-drops-its-census-findings-at-the-seam.md`)."""

    def test_a_parts_overlap_is_reported_as_carried(self):
        directory = Path(tempfile.mkdtemp())
        self.addCleanup(shutil.rmtree, directory, ignore_errors=True)
        store = Workspace(str(directory))
        part = Doc("pncad-carried-overlap-part")
        part.place(block(part, (0.0, 0.0), 1.0, 0.0, 1.0))
        part.place(block(part, (0.95, 0.2), 0.6, 0.25, 0.5))
        (own,) = assemble(part, evaluate(part)).interference
        store.create(part)
        outer = Doc("pncad-carried-overlap")
        instance = outer.insert(
            Node.instantiate_part(DocRef(part.id, pncad.content_pin(part)), [])
        )
        outer.pin(instance, resolver=store)
        assembly = assemble(outer, evaluate(outer, resolver=store))
        self.assertEqual(assembly.interference, [], "not this document's overlap")
        (carried,) = assembly.carried_interference
        self.assertEqual(carried.of, part.id)
        self.assertEqual(carried.through, instance)
        self.assertEqual(carried.via, [instance])
        finding = carried.finding
        self.assertTrue(finding.loud)
        self.assertEqual(
            (finding.a.hex, finding.b.hex), (own.a.hex, own.b.hex),
            "the part's own copies",
        )
        self.assertIsNone(finding.a.kind, "a variable of the part, not of this document")


if __name__ == "__main__":
    unittest.main()
