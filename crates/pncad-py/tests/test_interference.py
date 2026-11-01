"""An overlap between two copies at rest is a finding, not a refusal
(D10; INTENT stage 5 B and C).

`run_checks` reports, under the `AtRest` check, every overlap of two
copies' material, one finding per connected overlap, its evidence an
`InterferenceFinding` named by the faces of both copies that bound it.
Nothing at rest refuses: a pair the census could not look at is a
finding too, and `enforce_checks` at `Error` refuses a loud overlap and
nothing else. The quieting rule's rows are Rust's
(`crates/editor-core/tests/intent_s5_b_interference.rs`,
`intent_s5_c_at_rest_check.rs`); this file pins what a Python caller
reads.
"""

import shutil
import tempfile
import unittest
from pathlib import Path

import pncad
from pncad import (
    CheckId,
    CheckRefusal,
    ChecksConfig,
    Doc,
    DocRef,
    Formula,
    Frame,
    Node,
    Severity,
    Workspace,
    enforce_checks,
    evaluate,
    m,
    run_checks,
)

from placing import placed, assemble_world_copies


def block(doc, corner, side, elevation, height):
    """A square prism of `side` with its low corner at `corner`, its
    copy standing in the world with its base at `elevation`. Answers
    the placement."""
    x, y = corner
    outline = doc.insert(
        Node.polygon(
            [
                (Formula.length_in(x, m), Formula.length_in(y, m)),
                (Formula.length_in(x + side, m), Formula.length_in(y, m)),
                (Formula.length_in(x + side, m), Formula.length_in(y + side, m)),
                (Formula.length_in(x, m), Formula.length_in(y + side, m)),
            ],
        )
    )
    body = doc.insert(Node.extrude(outline, Formula.length_in(height, m)))
    if elevation == 0.0:
        return doc.place(body, doc.world)
    return placed(doc, body, Frame.translation((0 * m, 0 * m, elevation * m)))


def sunk(doc):
    """A unit block and a smaller one sunk 0.05 into its +x side,
    strictly inside it across y and z, as the operands of the product's
    `⊔`. Answers the two placements."""
    big = block(doc, (0.0, 0.0), 1.0, 0.0, 1.0)
    small = block(doc, (0.95, 0.2), 0.6, 0.25, 0.5)
    assemble_world_copies(doc)
    return big, small


def at_rest(doc, evaluation):
    """The `AtRest` check's findings over `evaluation`, in report order."""
    return [f for f in run_checks(doc, evaluation).findings if f.check == CheckId.AtRest]


class TestInterferenceIsAFinding(unittest.TestCase):
    def test_two_overlapping_copies_report_one_loud_finding(self):
        """The overlap is one loud `AtRest` finding under the product's
        `⊔`, naming both copies and faces of each."""
        doc = Doc()
        sunk(doc)
        (finding,) = at_rest(doc, evaluate(doc))
        self.assertEqual(finding.evidence.variant, "interference")
        self.assertEqual(finding.census, doc.product, "under the `⊔` that decided it")
        self.assertIsNone(finding.quiet)
        interference = finding.evidence.interference
        self.assertTrue(interference.loud)
        self.assertIsNone(interference.quiet_by)
        self.assertTrue(interference.quietable, "a gap between two parallel faces could quiet it")
        self.assertIsNone(interference.unlocalized)
        self.assertNotEqual(interference.a, interference.b)
        self.assertEqual(interference.a.kind, "body")
        faces = interference.faces
        self.assertIsNotNone(faces)
        copies = {site.copy for site in faces}
        self.assertEqual(copies, {interference.a, interference.b}, "both copies bound it")
        self.assertTrue(all(site.face for site in faces))

    def test_the_finding_moved_off_the_gate_with_its_faces(self):
        """Spec §6 row 19: `pncad.assemble` is gone, `Doc.assemble` still
        authors the `⊔`, and the overlap row 5's document reported is
        the report's `AtRest` finding with the same sites — the big
        block's +x face, and the five faces of the small block that lie
        inside the big one, each named once."""
        self.assertFalse(hasattr(pncad, "assemble"), "the gate is gone")
        self.assertFalse(hasattr(pncad, "Assembly"))
        self.assertFalse(hasattr(pncad, "AssemblyError"))
        doc = Doc()
        big, small = sunk(doc)
        (join,) = [n for n in doc.order() if doc.node_kind(n) == "assemble"]
        self.assertEqual(doc.product, doc.output(join), "the product reads the `⊔`")
        (finding,) = at_rest(doc, evaluate(doc))
        interference = finding.evidence.interference
        self.assertEqual(
            (interference.a, interference.b),
            (doc.output(big), doc.output(small)),
            "the copies, earlier in gather order first",
        )
        per_copy = {}
        for site in interference.faces:
            per_copy.setdefault(site.copy, []).append(site.face)
        self.assertEqual(
            {copy: len(faces) for copy, faces in per_copy.items()},
            {doc.output(big): 1, doc.output(small): 5},
        )
        for faces in per_copy.values():
            self.assertEqual(len(set(faces)), len(faces), "each face once")

    def test_disjoint_copies_report_no_at_rest_finding(self):
        doc = Doc()
        block(doc, (0.0, 0.0), 1.0, 0.0, 1.0)
        block(doc, (2.0, 0.0), 1.0, 0.0, 1.0)
        assemble_world_copies(doc)
        self.assertEqual(at_rest(doc, evaluate(doc)), [])

    def test_a_loud_overlap_refuses_at_error_only(self):
        doc = Doc()
        sunk(doc)
        report = run_checks(doc, evaluate(doc))
        enforce_checks(report)
        enforce_checks(report, ChecksConfig(at_rest=Severity.Warn))
        with self.assertRaises(CheckRefusal) as caught:
            enforce_checks(report, ChecksConfig(at_rest=Severity.Error))
        (refused,) = caught.exception.findings
        self.assertEqual(refused.evidence.variant, "interference")


def sliver(doc):
    """Two copies 3 nm apart, in the sliver band the census cannot
    decide, as the operands of the product's `⊔`."""
    block(doc, (0.0, 0.0), 1.0, 0.0, 1.0)
    block(doc, (1.0 + 3e-9, 0.2), 0.6, 0.25, 0.5)
    assemble_world_copies(doc)


class TestWhatTheCensusCouldNotLookAt(unittest.TestCase):
    def test_an_undecided_pair_is_a_finding_that_never_refuses(self):
        doc = Doc()
        sliver(doc)
        findings = at_rest(doc, evaluate(doc))
        self.assertTrue(findings, "an undecided pair is not a clean report")
        for finding in findings:
            with self.subTest(finding=str(finding)):
                self.assertEqual(finding.evidence.variant, "at_rest")
                self.assertEqual(finding.census, doc.product)
                self.assertIsNone(finding.quiet)
                evidence = finding.evidence.at_rest
                self.assertEqual(evidence.variant, finding.evidence.at_rest_variant)
                self.assertIn(evidence.variant, {"no_lane", "malformed", "inside_arrival"})
                self.assertIsNotNone(evidence.inner_variant)
                self.assertIsNone(evidence.carried)
                self.assertEqual(str(evidence), finding.evidence.reason)
        report = run_checks(doc, evaluate(doc))
        enforce_checks(report, ChecksConfig(at_rest=Severity.Error))


class TestAPartsFindingsAreThePart(unittest.TestCase):
    """A part crosses into the document instantiating it as one copy, so
    an overlap between the part's own copies lies inside one copy there:
    it is the part's finding, reported once with the route it came by,
    and never raised (`work/intent/a-part-drops-its-census-findings-at-the-seam.md`)."""

    def instanced(self, label, build):
        directory = Path(tempfile.mkdtemp())
        self.addCleanup(shutil.rmtree, directory, ignore_errors=True)
        store = Workspace(str(directory))
        part = Doc(f"{label}-part")
        build(part)
        store.create(part)
        outer = Doc(label)
        instance = outer.insert(
            Node.instantiate_part(DocRef(part.id, pncad.content_pin(part)), outer.world)
        )
        outer.set_product(instance)
        return part, outer, instance, evaluate(outer, resolver=store)

    def test_a_parts_overlap_is_reported_as_carried(self):
        part, outer, instance, ev = self.instanced("pncad-carried-overlap", sunk)
        (own,) = at_rest(part, evaluate(part))
        own = own.evidence.interference
        (finding,) = at_rest(outer, ev)
        self.assertEqual(finding.evidence.variant, "carried_interference")
        self.assertIsNone(finding.evidence.interference, "not this document's overlap")
        carried = finding.evidence.carried_interference
        self.assertEqual(carried.of, part.id)
        self.assertEqual(carried.through, instance)
        self.assertEqual(carried.via, [instance])
        self.assertEqual(carried.pin, pncad.content_pin(part))
        self.assertTrue(carried.finding.loud)
        self.assertIsNone(finding.quiet)
        self.assertEqual(
            (carried.finding.a.hex, carried.finding.b.hex),
            (own.a.hex, own.b.hex),
            "the part's own copies",
        )
        self.assertIsNone(carried.finding.a.kind, "a variable of the part, not of this document")

    def test_what_a_part_could_not_look_at_is_reported_as_carried(self):
        """The sliver part's own could-not-look findings cross the seam,
        one carried finding per row, each the part's evidence with its
        route; none refuses, at any severity."""
        part, outer, instance, ev = self.instanced("pncad-carried-at-rest", sliver)
        own = [f.evidence.at_rest for f in at_rest(part, evaluate(part))]
        self.assertTrue(own)
        carried = [f for f in at_rest(outer, ev) if f.evidence.at_rest_variant == "carried"]
        self.assertEqual(len(carried), len(own), "one carried finding per part row")
        rows = [f.evidence.at_rest.carried for f in carried]
        self.assertEqual([row.finding for row in rows], own, "the part's own evidence")
        for row in rows:
            self.assertEqual(row.of, part.id)
            self.assertEqual(row.through, instance)
            self.assertEqual(row.via, [instance])
            self.assertEqual(row.pin, pncad.content_pin(part))
            self.assertIn("could not look at", str(row))
        enforce_checks(run_checks(outer, ev), ChecksConfig(at_rest=Severity.Error))

    def test_what_a_part_could_not_look_at_is_reported_once(self):
        """The part decided its own pairs, so the instantiating
        document's census does not report them again as its own: every
        at-rest finding there is the part's, carried."""
        _part, outer, _instance, ev = self.instanced("pncad-carried-once", sliver)
        self.assertEqual(
            {f.evidence.at_rest_variant for f in at_rest(outer, ev)}, {"carried"}
        )


if __name__ == "__main__":
    unittest.main()
