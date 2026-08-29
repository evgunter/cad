"""The advisory-check registry from Python (DISCIPLINES-DESIGN DS6).

Every document here is AUTHORED — profile, extrude, boolean, through
the ordinary doors — and every finding is one the kernel produced from
that geometry. Nothing is stubbed: the disjoint union really has two
components, the overlapping roots really deny the separation
certificate, and the touching union really fails to evaluate.

WHAT THIS FILE IS FOR, ABOVE COVERAGE
-------------------------------------
The posture is the deliverable. `run_checks` REPORTS and gates
nothing; `enforce_checks` is the one refusing path and it refuses only
on what the CALLER set to `Severity.Error`. Both halves are asserted
positively rather than described:

- a document with findings still evaluates, still measures and still
  exports — `test_a_flagged_document_is_a_working_document`;
- the same report under the same findings is `Ok` at `Warn` and raises
  at `Error`, with nothing about the geometry different between the
  two — `test_severity_moves_only_the_gate`;
- the separation resident's refusing position is not merely unused, it
  cannot be SPELLED — `test_the_separation_knob_cannot_express_error`.

The negative half matters as much: `test_a_connected_body_is_clean`
is the document that trips nothing, so a finding here means the check
fired on the geometry and not on every document it is handed.
"""

import unittest

import pncad
from pncad import (
    Advisory,
    BooleanOp,
    CheckId,
    CheckKind,
    CheckRefusal,
    ChecksConfig,
    ChecksError,
    Doc,
    Node,
    Severity,
    enforce_checks,
    evaluate,
    m,
    run_checks,
    subject_body,
)


def slab(doc, x0, x1, y0=0.0, y1=1.0, z0=0.0, z1=1.0):
    """The axis-aligned box [x0,x1] x [y0,y1] x [z0,z1], in metres."""
    profile = doc.insert(
        Node.polygon(
            [
                (x0 * m, y0 * m),
                (x1 * m, y0 * m),
                (x1 * m, y1 * m),
                (x0 * m, y1 * m),
            ],
            elevation=z0 * m,
        )
    )
    return doc.insert(Node.extrude(profile, (z1 - z0) * m))


def disjoint_union():
    """Two boxes three metres apart, deliberately united: ONE root,
    one body, two components."""
    doc = Doc("checks-disjoint")
    a = slab(doc, 0.0, 1.0)
    b = slab(doc, 3.0, 4.0)
    root = doc.insert(Node.boolean(BooleanOp.Union, a, b))
    return doc, root, a


def one_box():
    doc = Doc("checks-connected")
    return doc, slab(doc, 0.0, 1.0)


def two_roots(x0):
    """Two independent roots — nothing consumes either, so the
    document's product gathers both."""
    doc = Doc("checks-two-roots")
    return doc, slab(doc, 0.0, 1.0), slab(doc, x0, x0 + 1.0)


class TestTheConnectednessResident(unittest.TestCase):
    def test_a_disjoint_union_is_one_finding_naming_its_subject(self):
        doc, root, _ = disjoint_union()
        report = run_checks(doc, evaluate(doc))
        self.assertEqual(len(report), 1)
        (finding,) = report.findings
        self.assertEqual(finding.check, CheckId.Connectedness)
        self.assertEqual(finding.root, root)
        self.assertEqual(finding.output_ix, 0)
        self.assertEqual(finding.evidence.variant, "connectedness")
        self.assertEqual(finding.evidence.actual, 2)
        self.assertEqual(finding.evidence.expected, 1)
        # Attributes on EVERY arm: the payload another arm carries
        # reads `None` rather than raising, so handling never has to
        # branch on `variant` before reading.
        self.assertIsNone(finding.evidence.other_root)
        self.assertIsNone(finding.evidence.other_output)
        self.assertIsNone(finding.evidence.reason)
        self.assertIn("component", str(finding))

    def test_a_connected_body_is_clean(self):
        doc, _ = one_box()
        report = run_checks(doc, evaluate(doc))
        self.assertEqual(report.findings, [])
        self.assertEqual(report.skipped, [])
        self.assertEqual(len(report), 0)

    def test_an_interior_void_is_boundary_and_not_a_component(self):
        doc = Doc("checks-voided")
        outer = slab(doc, 0.0, 3.0, 0.0, 3.0, 0.0, 3.0)
        inner = slab(doc, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0)
        doc.insert(Node.boolean(BooleanOp.Subtract, outer, inner))
        self.assertEqual(run_checks(doc, evaluate(doc)).findings, [])

    def test_a_stated_expectation_is_the_acknowledgment(self):
        doc, root, _ = disjoint_union()
        cfg = ChecksConfig(expected_components=[(root, 0, 2)])
        self.assertEqual(run_checks(doc, evaluate(doc), cfg).findings, [])
        self.assertEqual(cfg.expected_components, [(root, 0, 2)])

    def test_an_expectation_nothing_consumed_is_stale_not_ignored(self):
        """Two-directional: the union CONSUMED `a`, so `a` is no
        root and its expectation binds no subject. A stale
        acknowledgment must not read as "checked and fine"."""
        doc, root, consumed = disjoint_union()
        cfg = ChecksConfig(expected_components=[(root, 0, 2), (consumed, 0, 1)])
        report = run_checks(doc, evaluate(doc), cfg)
        (finding,) = report.findings
        self.assertEqual(finding.evidence.variant, "stale_expectation")
        self.assertEqual(finding.root, consumed)
        self.assertEqual(finding.evidence.expected, 1)
        self.assertIsNone(finding.evidence.actual)

    def test_one_subject_cannot_be_expected_twice(self):
        _, root, _ = disjoint_union()
        with self.assertRaises(ValueError) as caught:
            ChecksConfig(expected_components=[(root, 0, 2), (root, 0, 1)])
        self.assertIn("twice", str(caught.exception))


class TestTheSeparationResident(unittest.TestCase):
    def test_interpenetrating_roots_deny_the_certificate(self):
        doc, first, second = two_roots(0.5)
        report = run_checks(doc, evaluate(doc))
        (finding,) = report.findings
        self.assertEqual(finding.check, CheckId.Separation)
        self.assertEqual(finding.evidence.variant, "not_separated")
        self.assertEqual(finding.root, first)
        self.assertEqual(finding.evidence.other_root, second)
        self.assertEqual(finding.evidence.other_output, 0)
        self.assertIsNone(finding.evidence.actual)

    def test_roots_that_are_apart_are_clean(self):
        doc, _, _ = two_roots(5.0)
        self.assertEqual(run_checks(doc, evaluate(doc)).findings, [])

    def test_the_separation_knob_cannot_express_error(self):
        """DS6's waiver rule as a TYPE, not as a comment asking
        callers not to reach: this resident ships no acknowledgment
        record, so its refusing position does not exist."""
        self.assertFalse(hasattr(Advisory, "Error"))
        with self.assertRaises(TypeError):
            ChecksConfig(separation=Severity.Error)


class TestTheReportGateSplit(unittest.TestCase):
    def test_a_flagged_document_is_a_working_document(self):
        """REPORTS, NEVER GATES. The finding says the almost-right
        picture is wrong; it stops nothing."""
        doc, root, _ = disjoint_union()
        ev = evaluate(doc)
        self.assertEqual(len(run_checks(doc, ev)), 1)
        # Evaluated, measurable, exportable — all of it after the
        # finding, and none of it affected by it.
        body = ev.value(root).body()
        self.assertAlmostEqual(body.mass_properties().volume, 2.0, places=9)
        self.assertIn("ISO-10303-21", ev.step_string(root))

    def test_severity_moves_only_the_gate(self):
        """`Warn` and `Error` produce IDENTICAL findings; the only
        difference is whether `enforce_checks` refuses on them."""
        doc, root, _ = disjoint_union()
        ev = evaluate(doc)
        warn = ChecksConfig(connectedness=Severity.Warn)
        error = ChecksConfig(connectedness=Severity.Error)
        warned = run_checks(doc, ev, warn)
        errored = run_checks(doc, ev, error)
        self.assertEqual(warned.findings, errored.findings)
        # Running the checks at Error refuses NOTHING on its own.
        self.assertIsNone(enforce_checks(warned, warn))
        with self.assertRaises(CheckRefusal) as caught:
            enforce_checks(errored, error)
        (refusing,) = caught.exception.findings
        self.assertEqual(refusing, errored.findings[0])
        self.assertEqual(refusing.root, root)
        self.assertIsInstance(caught.exception, pncad.PncadError)

    def test_the_default_configuration_can_never_refuse(self):
        doc, _, _ = disjoint_union()
        report = run_checks(doc, evaluate(doc))
        self.assertEqual(len(report), 1)
        self.assertIsNone(enforce_checks(report))

    def test_off_is_visibly_skipped_and_per_resident(self):
        """"Checked and fine" and "not checked" are different
        answers, and `skipped` is the difference."""
        doc, _, _ = disjoint_union()
        ev = evaluate(doc)
        off = ChecksConfig(connectedness=Severity.Off)
        report = run_checks(doc, ev, off)
        self.assertEqual(report.findings, [])
        self.assertEqual(report.skipped, [CheckId.Connectedness])
        both = ChecksConfig(connectedness=Severity.Off, separation=Advisory.Off)
        self.assertEqual(
            run_checks(doc, ev, both).skipped,
            [CheckId.Connectedness, CheckId.Separation],
        )


class TestTheRegistryVocabulary(unittest.TestCase):
    def test_every_resident_carries_its_honesty_label(self):
        self.assertEqual(CheckId.Connectedness.kind, CheckKind.Certified)
        self.assertEqual(CheckId.Separation.kind, CheckKind.Certified)
        self.assertEqual(str(CheckId.Connectedness), "connectedness")

    def test_the_configuration_reads_one_severity_vocabulary(self):
        cfg = ChecksConfig(connectedness=Severity.Error, separation=Advisory.Off)
        self.assertEqual(cfg.connectedness, Severity.Error)
        self.assertEqual(cfg.separation, Advisory.Off)
        # The Advisory widening: one vocabulary at the read door.
        self.assertEqual(cfg.severity(CheckId.Connectedness), Severity.Error)
        self.assertEqual(cfg.severity(CheckId.Separation), Severity.Off)
        self.assertEqual(cfg, ChecksConfig(connectedness=Severity.Error,
                                           separation=Advisory.Off))

    def test_a_report_renders_itself(self):
        doc, _, _ = disjoint_union()
        rendered = str(run_checks(doc, evaluate(doc)))
        self.assertIn("1 finding", rendered)
        clean, _ = one_box()
        self.assertIn("no findings", str(run_checks(clean, evaluate(clean))))


class TestSubjectBody(unittest.TestCase):
    def test_a_findings_attribution_resolves_to_the_flagged_body(self):
        doc, _, _ = disjoint_union()
        ev = evaluate(doc)
        (finding,) = run_checks(doc, ev).findings
        body = subject_body(ev, finding.root, finding.output_ix)
        self.assertIsNotNone(body)
        # The two unit cubes the count found, as one body.
        self.assertAlmostEqual(body.mass_properties().volume, 2.0, places=9)

    def test_an_attribution_with_no_subject_answers_none(self):
        """The `None` a `stale_expectation` finding names — an answer,
        not a failure.

        Measured rather than assumed, and it is the narrower of the
        two readings: the `None` is about SUBJECTHOOD only where the
        value is missing. An expectation keyed at a node the union
        CONSUMED is stale — that node is no root, so no subject
        consumed the entry — and yet its body still resolves here,
        because the node evaluated. Staleness is a fact about the
        root list; this door is a fact about the evaluation."""
        doc, root, consumed = disjoint_union()
        ev = evaluate(doc)
        self.assertIsNone(subject_body(ev, root, 7))
        self.assertIsNotNone(subject_body(ev, consumed, 0))

        touching = Doc("checks-touching-subject")
        a = slab(touching, 0.0, 1.0)
        b = slab(touching, 1.0, 2.0)
        failed = touching.insert(Node.boolean(BooleanOp.Union, a, b))
        self.assertIsNone(subject_body(evaluate(touching), failed, 0))


class TestTheChecksCouldNotRun(unittest.TestCase):
    def test_a_root_without_a_value_refuses_typed(self):
        """A report over a partial evaluation would claim more than
        was checked. `evaluate` is total, so the failure arrives as a
        valueless root and the registry refuses on it rather than
        reporting over what did evaluate."""
        doc = Doc("checks-touching")
        a = slab(doc, 0.0, 1.0)
        b = slab(doc, 1.0, 2.0)
        root = doc.insert(Node.boolean(BooleanOp.Union, a, b))
        ev = evaluate(doc)
        with self.assertRaises(ChecksError) as caught:
            run_checks(doc, ev)
        self.assertEqual(caught.exception.variant, "root_without_value")
        self.assertEqual(caught.exception.node, root)
        self.assertIsInstance(caught.exception, pncad.PncadError)


if __name__ == "__main__":
    unittest.main()
