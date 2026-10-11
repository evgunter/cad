"""An overlap between two copies at rest is a finding, not a refusal
(D10; INTENT stage 5 B).

`assemble` answers an `Assembly` whose `interference` lists every
overlap of two copies' material, one `InterferenceFinding` per
connected overlap, named by the faces of both copies that bound it.
The quieting rule's rows are Rust's
(`crates/editor-core/tests/intent_s5_b_interference.rs`); this file
pins what a Python caller reads.
"""

import unittest

from pncad import Doc, Formula, Node, assemble, evaluate, m


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


if __name__ == "__main__":
    unittest.main()
