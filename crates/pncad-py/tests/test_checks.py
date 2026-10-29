"""The advisory-check registry from Python (DISCIPLINES-DESIGN DS6).

Every document here is AUTHORED — profile, extrude, boolean, through
the ordinary doors — and every finding is one the kernel produced from
that geometry. Nothing is stubbed: the disjoint union really has two
components, the overlapping placed bodies really overlap at rest, and
the touching union really fails to evaluate.

ONE EXCEPTION, and it is deliberate:
`TestSubjectBodyCarriesItsDeclarations` instantiates the bench
scene's MATED stand, because the claim it pins is about DECLARED
contacts and the authoring vocabulary above declares none. It comes
through `test_assembly_eval.opened` for the same reason `test_validate`
takes it from there — one definition of that scene, authored in
`bench_scene.py` and read back out of a store.

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
- an advisory resident's refusing position is not merely unused, it
  cannot be SPELLED — `test_only_the_at_rest_knob_of_the_two_can_express_error`.

The negative half matters as much: `test_a_connected_body_is_clean`
is the document that trips nothing, so a finding here means the check
fired on the geometry and not on every document it is handed.
"""

import math
import shutil
import tempfile
import unittest
from pathlib import Path

import pncad
from pncad import (
    Advisory,
    ArcSweep,
    Center,
    CheckId,
    CheckKind,
    CheckRefusal,
    ChecksConfig,
    ChecksError,
    Doc,
    DocEdit,
    DocRef,
    Coord,
    Formula,
    FreeValue,
    FreeVar,
    Frame,
    MateValue,
    Node,
    Open,
    Severity,
    Start,
    VarName,
    Workspace,
    enforce_checks,
    evaluate,
    m,
    product,
    run_checks,
    subject_body,
)

from placing import against, assemble_world_copies, placed
from test_assembly_eval import identities, opened
from measuring import pinned


def slab(doc, x0, x1, y0=0.0, y1=1.0, z0=0.0, z1=1.0, on=None):
    """The axis-aligned box [x0,x1] x [y0,y1] x [z0,z1], in metres: in
    its own coordinates `z0` up where it stands alone, or placed
    against `on` (`on`'s own coordinates) where it is combined with
    it."""
    profile = doc.insert(
        Node.polygon(
            [
                (Formula.length_in(x0, m), Formula.length_in(y0, m)),
                (Formula.length_in(x1, m), Formula.length_in(y0, m)),
                (Formula.length_in(x1, m), Formula.length_in(y1, m)),
                (Formula.length_in(x0, m), Formula.length_in(y1, m)),
            ],
        )
    )
    body = doc.insert(Node.extrude(profile, Formula.length_in(z1 - z0, m)))
    if on is None:
        assert z0 == 0.0, "a box standing alone sits on its own xy"
        return body
    return against(doc, on, body, Frame.translation((0 * m, 0 * m, z0 * m)))


def disjoint_union():
    """Two boxes three metres apart, deliberately united and placed:
    ONE copy, one body, two components. Answers the placement — the
    node a finding attributes its subject to — and the first box."""
    doc = Doc("checks-disjoint")
    a = slab(doc, 0.0, 1.0)
    b = slab(doc, 3.0, 4.0, on=a)
    placed = doc.place(doc.insert(Node.union([a, b])), doc.world)
    return doc, placed, a


def one_box():
    """One box, placed; answers its placement."""
    doc = Doc("checks-connected")
    return doc, doc.place(slab(doc, 0.0, 1.0), doc.world)


def two_placed(x0):
    """Two boxes, each placed — the document's product gathers both.
    Answers the two placements."""
    doc = Doc("checks-two-placed")
    first = doc.place(slab(doc, 0.0, 1.0), doc.world)
    return doc, first, doc.place(slab(doc, x0, x0 + 1.0), doc.world)


class TestTheConnectednessResident(unittest.TestCase):
    def test_a_disjoint_union_is_one_finding_naming_its_subject(self):
        doc, placement, _ = disjoint_union()
        assemble_world_copies(doc)
        report = run_checks(doc, evaluate(doc))
        self.assertEqual(len(report), 1)
        (finding,) = report.findings
        self.assertEqual(finding.check, CheckId.Connectedness)
        self.assertEqual(finding.placement, placement)
        self.assertEqual(finding.output_ix, 0)
        self.assertIsNone(finding.node, "a placement output's finding is about no lone node")
        self.assertEqual(finding.evidence.variant, "connectedness")
        self.assertEqual(finding.evidence.actual, 2)
        self.assertEqual(finding.evidence.expected, 1)
        # Attributes on EVERY arm: the payload another arm carries
        # reads `None` rather than raising, so handling never has to
        # branch on `variant` before reading.
        self.assertIsNone(finding.evidence.reason)
        self.assertIsNone(finding.evidence.inner_variant)
        self.assertIsNone(finding.evidence.at_rest_variant)
        self.assertIsNone(finding.evidence.interference)
        self.assertIsNone(finding.evidence.carried_interference)
        self.assertIsNone(finding.evidence.at_rest)
        self.assertIsNone(finding.census)
        self.assertIsNone(finding.quiet)
        self.assertIn("component", str(finding))

    def test_a_connected_body_is_clean(self):
        doc, _ = one_box()
        assemble_world_copies(doc)
        report = run_checks(doc, evaluate(doc))
        self.assertEqual(report.findings, [])
        # The chart-coherence resident is Off by default and says so:
        # "not checked" is an answer this report carries.
        self.assertEqual(report.skipped, [CheckId.ChartCoherence])
        self.assertEqual(len(report), 0)

    def test_an_interior_void_is_boundary_and_not_a_component(self):
        doc = Doc("checks-voided")
        outer = slab(doc, 0.0, 3.0, 0.0, 3.0, 0.0, 3.0)
        inner = slab(doc, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0, on=outer)
        doc.insert(Node.subtract(outer, inner))
        assemble_world_copies(doc)
        self.assertEqual(run_checks(doc, evaluate(doc)).findings, [])

    def test_a_stated_expectation_is_the_acknowledgment(self):
        doc, placement, _ = disjoint_union()
        cfg = ChecksConfig(expected_components=[(placement, 0, 2)])
        assemble_world_copies(doc)
        self.assertEqual(run_checks(doc, evaluate(doc), cfg).findings, [])
        self.assertEqual(cfg.expected_components, [(placement, 0, 2)])

    def test_an_expectation_nothing_consumed_is_stale_not_ignored(self):
        """Two-directional: the union CONSUMED `a`, so `a` is no
        placement and its expectation binds no subject. A stale
        acknowledgment must not read as "checked and fine"."""
        doc, placement, consumed = disjoint_union()
        cfg = ChecksConfig(expected_components=[(placement, 0, 2), (consumed, 0, 1)])
        assemble_world_copies(doc)
        report = run_checks(doc, evaluate(doc), cfg)
        (finding,) = report.findings
        self.assertEqual(finding.evidence.variant, "stale_expectation")
        self.assertEqual(finding.placement, consumed)
        self.assertEqual(finding.evidence.expected, 1)
        self.assertIsNone(finding.evidence.actual)

    def test_one_subject_cannot_be_expected_twice(self):
        _, placement, _ = disjoint_union()
        with self.assertRaises(ValueError) as caught:
            ChecksConfig(expected_components=[(placement, 0, 2), (placement, 0, 1)])
        self.assertIn("twice", str(caught.exception))


class TestTheAtRestResident(unittest.TestCase):
    def test_interpenetrating_copies_are_one_interference_finding(self):
        doc, first, second = two_placed(0.5)
        assemble_world_copies(doc)
        report = run_checks(doc, evaluate(doc))
        (finding,) = report.findings
        self.assertEqual(finding.check, CheckId.AtRest)
        self.assertEqual(finding.evidence.variant, "interference")
        self.assertEqual(finding.census, doc.product, "the `⊔` whose census decided it")
        self.assertIsNone(finding.placement)
        self.assertIsNone(finding.node)
        self.assertIsNone(finding.quiet)
        interference = finding.evidence.interference
        self.assertEqual(
            (interference.a, interference.b), (doc.output(first), doc.output(second))
        )
        self.assertTrue(interference.loud)
        self.assertIsNone(finding.evidence.at_rest)
        self.assertIsNone(finding.evidence.actual)

    def test_copies_that_are_apart_are_clean(self):
        doc, _, _ = two_placed(5.0)
        assemble_world_copies(doc)
        self.assertEqual(run_checks(doc, evaluate(doc)).findings, [])

    def test_only_the_at_rest_knob_of_the_two_can_express_error(self):
        """DS6's waiver rule as a TYPE, not as a comment asking
        callers not to reach: the at-rest check's acknowledgment is the
        assertion that quiets an overlap, so its knob is a `Severity`;
        the unproven-coincidence lint's is an `Advisory`, whose
        refusing position does not exist."""
        self.assertEqual(ChecksConfig(at_rest=Severity.Error).at_rest, Severity.Error)
        self.assertFalse(hasattr(Advisory, "Error"))
        with self.assertRaises(TypeError):
            ChecksConfig(unproven_coincidence=Severity.Error)


class TestTheReportGateSplit(unittest.TestCase):
    def test_a_flagged_document_is_a_working_document(self):
        """REPORTS, NEVER GATES. The finding says the almost-right
        picture is wrong; it stops nothing."""
        doc, placement, _ = disjoint_union()
        ev = evaluate(doc)
        self.assertEqual(len(run_checks(doc, ev)), 1)
        # Evaluated, measurable, exportable — all of it after the
        # finding, and none of it affected by it.
        body = ev.value(placement).body()
        self.assertAlmostEqual(pinned(body).volume, 2.0, places=9)
        self.assertIn("ISO-10303-21", ev.step_string(placement))

    def test_severity_moves_only_the_gate(self):
        """`Warn` and `Error` produce IDENTICAL findings; the only
        difference is whether `enforce_checks` refuses on them."""
        doc, placement, _ = disjoint_union()
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
        self.assertEqual(refusing.placement, placement)
        self.assertIsInstance(caught.exception, pncad.PncadError)

    def test_the_default_configuration_can_never_refuse(self):
        doc, _, _ = disjoint_union()
        assemble_world_copies(doc)
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
        self.assertEqual(
            report.skipped, [CheckId.Connectedness, CheckId.ChartCoherence]
        )
        both = ChecksConfig(connectedness=Severity.Off, at_rest=Severity.Off)
        self.assertEqual(
            run_checks(doc, ev, both).skipped,
            [CheckId.Connectedness, CheckId.ChartCoherence, CheckId.AtRest],
        )


class TestTheRegistryVocabulary(unittest.TestCase):
    def test_every_resident_carries_its_honesty_label(self):
        self.assertEqual(CheckId.Connectedness.kind, CheckKind.Certified)
        self.assertEqual(CheckId.AtRest.kind, CheckKind.Certified)
        self.assertEqual(CheckId.ChartCoherence.kind, CheckKind.Certified)
        self.assertEqual(str(CheckId.Connectedness), "connectedness")
        self.assertEqual(str(CheckId.AtRest), "at-rest")

    def test_the_configuration_reads_one_severity_vocabulary(self):
        cfg = ChecksConfig(
            connectedness=Severity.Error,
            at_rest=Severity.Off,
            unproven_coincidence=Advisory.Off,
        )
        self.assertEqual(cfg.connectedness, Severity.Error)
        self.assertEqual(cfg.at_rest, Severity.Off)
        self.assertEqual(cfg.unproven_coincidence, Advisory.Off)
        # The Advisory widening: one vocabulary at the read door.
        self.assertEqual(cfg.severity(CheckId.Connectedness), Severity.Error)
        self.assertEqual(cfg.severity(CheckId.AtRest), Severity.Off)
        self.assertEqual(cfg.severity(CheckId.UnprovenCoincidence), Severity.Off)
        self.assertEqual(ChecksConfig().at_rest, Severity.Warn, "the default")
        self.assertEqual(
            cfg,
            ChecksConfig(
                connectedness=Severity.Error,
                at_rest=Severity.Off,
                unproven_coincidence=Advisory.Off,
            ),
        )

    def test_a_report_speaks_its_placements_from_the_evaluated_document(self):
        """`str()` of the report, of a finding and of the refusal says
        each placement as the evaluated document holds it — kind, label and
        tag — and `repr()` keeps the full id. A label set after
        `evaluate` is not the evaluated document's, so it is not said."""
        doc, placement, _ = disjoint_union()
        doc.apply(DocEdit.set_label(placement, "joined"))
        evaluation = evaluate(doc)
        doc.apply(DocEdit.set_label(placement, "renamed"))
        strict = ChecksConfig(connectedness=Severity.Error)
        report = run_checks(doc, evaluation, strict)
        (finding,) = report.findings
        spoken = 'check connectedness: Place "joined" ('
        self.assertIn(spoken, str(report))
        self.assertIn(spoken, str(finding))
        self.assertNotIn("renamed", str(report))
        full = repr(placement).removeprefix("NodeId(").removesuffix(")")
        self.assertNotIn(f"placement {full[:12]}", str(finding))
        self.assertIn(f"placement {full},", repr(finding))
        with self.assertRaises(CheckRefusal) as caught:
            enforce_checks(report, strict)
        self.assertIn(spoken, str(caught.exception))
        self.assertIn(spoken, str(caught.exception.findings[0]))

    def test_equal_means_the_same_findings_over_the_same_document(self):
        """Another document holding the same ids as other nodes reports
        findings that read alike and are not equal; a label is not
        compared."""

        def checked(seed, text):
            doc = Doc(seed)
            first = doc.place(slab(doc, 0.0, 1.0), doc.world)
            doc.place(slab(doc, 0.5, 1.5), doc.world)
            doc.apply(DocEdit.set_label(first, text))
            assemble_world_copies(doc)
            report = run_checks(doc, evaluate(doc))
            return first, report

        first, report = checked("checks-equal-a", "one")
        again_first, again = checked("checks-equal-a", "two")
        other_first, other = checked("checks-equal-b", "one")
        self.assertEqual((first, other_first), (again_first, first))
        self.assertEqual(report, again)
        self.assertEqual(report.findings, again.findings)
        self.assertNotEqual(report, other)
        self.assertNotEqual(report.findings[0], other.findings[0])

    def test_a_report_renders_itself(self):
        doc, _, _ = disjoint_union()
        assemble_world_copies(doc)
        rendered = str(run_checks(doc, evaluate(doc)))
        self.assertIn("1 finding", rendered)
        clean, _ = one_box()
        assemble_world_copies(clean)
        self.assertIn("no findings", str(run_checks(clean, evaluate(clean))))


class TestSubjectBody(unittest.TestCase):
    def test_a_findings_attribution_resolves_to_the_flagged_body(self):
        doc, _, _ = disjoint_union()
        ev = evaluate(doc)
        (finding,) = run_checks(doc, ev).findings
        body = subject_body(ev, finding.placement, finding.output_ix)
        self.assertIsNotNone(body)
        # The two unit cubes the count found, as one body.
        self.assertAlmostEqual(pinned(body).volume, 2.0, places=9)

    def test_an_attribution_with_no_subject_answers_none(self):
        """The `None` a `stale_expectation` finding names — an answer,
        not a failure.

        Measured rather than assumed, and it is the narrower of the
        two readings: the `None` is about SUBJECTHOOD only where the
        value is missing. An expectation keyed at a node the union
        CONSUMED is stale — nothing places that node, so no subject
        consumed the entry — and yet its body still resolves here,
        because the node evaluated. Staleness is a fact about the
        placements; this door is a fact about the evaluation."""
        doc, placement, consumed = disjoint_union()
        ev = evaluate(doc)
        self.assertIsNone(subject_body(ev, placement, 7))
        self.assertIsNotNone(subject_body(ev, consumed, 0))

        sliver = Doc("checks-sliver-subject")
        a = slab(sliver, 0.0, 1.0)
        # 2 nm apart: in band, so the union refuses and has no value.
        b = slab(sliver, 1.0 + 2e-9, 2.0, on=a)
        failed = sliver.insert(Node.union([a, b]))
        self.assertIsNone(subject_body(evaluate(sliver), failed, 0))


class TestSubjectBodyCarriesItsDeclarations(unittest.TestCase):
    """One body, one tier-3′ verdict, however it is reached.

    `subject_body` used to hand back a PLAIN body — the editor-core
    door dropped the `ContactRecords` `product::sources_of` had built
    beside it one line earlier — so a subject whose declarations were
    the whole reason its seam is legal reported that seam as an
    unrecorded contact, while the identical body read off its own
    `Value` passed. The narrowing failed loud rather than silently (an
    absent record set can only make the gate REFUSE), which is what
    made it a diagnosis a caller could not act on rather than a wrong
    blessing.

    The subject here is an INSTANCE of the mated stand, whose records
    are the D-1 declarations a referenced document's product minted,
    carried in its value's carry. A boolean's own records ride the same
    carry, but they cannot carry this row: a declared glue WELDS the
    faces it declares, so its record set comes out empty and its 3′
    pass is vacuous (`test_validate`'s module header states the same
    measurement). So this row is the one record source that can be
    observed from Python at all, and it is named as such rather than
    presented as the whole claim.
    """

    def instance_of_the_stand(self):
        """A document placing one instance of the bench scene's mated stand.

        The stand's product carries the records its two mates minted,
        and those cross the document seam with the instance — so this
        node's value is a body whose seats are declared.
        """
        store, _ = opened()
        ident = identities()["stand"]
        doc = Doc("checks-instantiated-assembly")
        instance = doc.insert(
            Node.instantiate_part(DocRef(ident, store.current_pin(ident)), doc.world)
        )
        doc.set_product(instance)
        return doc, instance, evaluate(doc, resolver=store)

    def test_the_subject_is_the_body_its_producer_minted(self):
        _doc, instance, ev = self.instance_of_the_stand()
        through_the_value = ev.value(instance).body()
        through_the_attribution = subject_body(ev, instance, 0)
        # It is ONE body: same geometry by every door that cannot see
        # declarations.
        self.assertEqual(
            pinned(through_the_value).volume,
            pinned(through_the_attribution).volume,
        )
        for rung in ("validate", "validate_closed", "validate_geometric"):
            with self.subTest(rung=rung):
                getattr(through_the_attribution, rung)()
        # And it is ONE verdict on the rung that CAN see them. This is
        # the regression: the second call refused before the records
        # travelled with the body.
        through_the_value.validate_pseudomanifold()
        through_the_attribution.validate_pseudomanifold()

    def test_the_census_really_looks_at_this_geometry(self):
        """The control, without which the row above proves nothing.

        A pass on the fourth rung means either "declared" or "no
        coincidence found", and only the first is interesting. So the
        SAME geometry is taken through `product`, which gathers the
        solids and declares nothing by contract — and it refuses, with
        findings. The seats are therefore real, and the two passes
        above are the records doing work rather than an empty census.
        """
        doc, _, ev = self.instance_of_the_stand()
        with self.assertRaises(pncad.ValidationError) as caught:
            product(doc, ev).validate_pseudomanifold()
        self.assertEqual(caught.exception.door, "validate_pseudomanifold")
        self.assertGreater(caught.exception.failure_count, 0)


class TestTheChecksCouldNotRun(unittest.TestCase):
    def test_a_placement_without_a_value_refuses_typed(self):
        """A report over a partial evaluation would claim more than
        was checked. `evaluate` is total, so the failure arrives as a
        valueless placement and the registry refuses on it rather than
        reporting over what did evaluate."""
        doc = Doc("checks-sliver")
        a = slab(doc, 0.0, 1.0)
        # Placed 0.5 m beside the first along x, by a value reading
        # `apart`; then `apart` closes the gap to 2 nm: in band, so the
        # union refuses.
        doc.apply(DocEdit.declare_var(VarName("apart"), FreeVar.length(1.5 * m)))
        b = against(doc, a, slab(doc, 0.0, 1.0), Frame.translation((1.5 * m, 0 * m, 0 * m)))
        # The bundle written again whole, its slide along `x` reading
        # `apart`.
        values = [c for _, c in doc.bundle(b)]
        doc.set_constraints(
            b,
            [
                MateValue(
                    c.coord,
                    doc.var(VarName("apart")) if c.coord == Coord.slide("x") else c.by,
                )
                for c in values
            ],
        )
        union = doc.insert(Node.union([a, b]))
        placement = doc.place(union, doc.world)
        self.assertAlmostEqual(
            pinned(evaluate(doc).value(union).body()).volume, 2.0, places=9
        )
        doc.apply(DocEdit.set_var_value(VarName("apart"), FreeValue.length((1.0 + 2e-9) * m)))
        ev = evaluate(doc)
        with self.assertRaises(ChecksError) as caught:
            run_checks(doc, ev)
        self.assertEqual(caught.exception.variant, "placement_without_value")
        self.assertEqual(caught.exception.node, placement)
        self.assertIsInstance(caught.exception, pncad.PncadError)


class TestTheUnprovenCoincidenceResident(unittest.TestCase):
    """The `unproven-coincidence` lint (D10): a plate resting flush on
    a block, unioned. The margin decides the rest from values, so the
    union records it, and the door does not prove it: the two caps are
    two constructions, and the block's top and the plate's floor are two
    variables typed to one value. The plate's four corners, each decided
    on the block's top, are recorded and reported alike. The report's
    rows are the evaluation's unproven rows, word for word."""

    def test_a_glued_rest_and_its_corners_are_unproven_coincidences(self):
        doc = Doc()
        lower = slab(doc, 0.0, 1.0)
        upper = slab(doc, 0.25, 0.75, 0.25, 0.75, 1.0, 1.5, on=lower)
        glued = doc.insert(Node.union([lower, upper]))
        ev = evaluate(doc)
        rows = ev.coincidences(glued)
        self.assertEqual(len(rows), 5)
        row = rows[0]
        self.assertEqual(
            [(r.relation, r.site) for r in rows[1:]],
            [("on_carrier", "vertex_fusion")] * 4,
        )
        self.assertEqual((row.relation, row.site), ("same_opposite", "plane_ladder"))
        # The door replays the document with every variable a symbol,
        # and says so of a row it does not prove.
        self.assertIsNone(row.rung)
        self.assertEqual(
            row.residual,
            "the two cells are two constructions, placed or reflected apart; with every variable a symbol, "
            "neither the carriers' verdict nor the deciding margin is shown "
            "identically zero within the door's budget",
        )
        # Each cell is named in the table of the read it came in
        # through: the two members' outputs.
        self.assertEqual(
            [read for read, _ in row.cells], [doc.output(lower, 0), doc.output(upper, 0)]
        )
        self.assertTrue(all(name is not None for _, name in row.cells))

        cfg = ChecksConfig(
            connectedness=Severity.Off,
            at_rest=Severity.Off,
            unproven_coincidence=Advisory.Warn,
        )
        report = run_checks(doc, ev, cfg)
        self.assertEqual(len(report.findings), 5)
        (finding,) = [
            f for f in report.findings if f.evidence.site == "plane_ladder"
        ]
        self.assertEqual(finding.check, CheckId.UnprovenCoincidence)
        self.assertEqual(finding.check.kind, CheckKind.Certified)
        self.assertEqual(finding.node, glued)
        self.assertIsNone(finding.placement)
        self.assertIsNone(finding.output_ix)
        evidence = finding.evidence
        self.assertEqual(evidence.variant, "unproven_coincidence")
        self.assertEqual(
            (evidence.relation, evidence.site, evidence.reason),
            (row.relation, row.site, row.residual),
        )
        self.assertEqual(evidence.coincidence.cells, row.cells)

        off = run_checks(doc, ev, ChecksConfig(unproven_coincidence=Advisory.Off))
        self.assertIn(CheckId.UnprovenCoincidence, off.skipped)
        self.assertEqual(
            [f for f in off.findings if f.check == CheckId.UnprovenCoincidence], []
        )

    def test_a_parts_unproven_rest_is_reported_through_its_instance(self):
        """The resting part, instanced: the instantiating document
        reports each of the part's five unproven rows once, the part's
        own finding over the part's document, by the route through the
        instance and shown there."""
        part = Doc("pncad-carried-coincidence-part")
        lower = slab(part, 0.0, 1.0)
        upper = slab(part, 0.25, 0.75, 0.25, 0.75, 1.0, 1.5, on=lower)
        glued = part.insert(Node.union([lower, upper]))
        placed(part, glued, Frame.translation((0 * m, 0 * m, 0 * m)))
        assemble_world_copies(part)
        cfg = ChecksConfig(
            connectedness=Severity.Off,
            at_rest=Severity.Off,
            unproven_coincidence=Advisory.Warn,
        )
        own = run_checks(part, evaluate(part), cfg).findings
        self.assertEqual(len(own), 5)
        directory = Path(tempfile.mkdtemp())
        self.addCleanup(shutil.rmtree, directory, ignore_errors=True)
        store = Workspace(str(directory))
        store.create(part)
        outer = Doc("pncad-carried-coincidence")
        instance = outer.insert(
            Node.instantiate_part(DocRef(part.id, pncad.content_pin(part)), outer.world)
        )
        outer.set_product(instance)
        found = run_checks(outer, evaluate(outer, resolver=store), cfg).findings
        self.assertEqual(len(found), len(own), "each of the part's rows, once")
        for finding, mine in zip(found, own, strict=True):
            self.assertEqual(finding.check, CheckId.UnprovenCoincidence)
            self.assertEqual(finding.node, instance, "shown at the instance")
            self.assertIsNone(finding.quiet)
            self.assertEqual(finding.evidence.variant, "carried_coincidence")
            self.assertIsNone(finding.evidence.coincidence, "not this document's row")
            carried = finding.evidence.carried_coincidence
            self.assertEqual(carried.finding, mine, "the part's own finding")
            self.assertEqual(carried.finding.node, glued, "in the part's ids")
            self.assertEqual(
                carried.finding.evidence.coincidence.cells, mine.evidence.coincidence.cells
            )
            self.assertEqual(carried.of, part.id)
            self.assertEqual(carried.through, instance)
            self.assertEqual(carried.via, [instance])
            self.assertEqual(carried.pin, pncad.content_pin(part))
            self.assertIn("leaves a coincidence of its own unproven", str(finding))


class TestAProfileJunctionTheDoorDecides(unittest.TestCase):
    """D1's profile tangency: a junction's tangency is its turn levered
    by the shorter of its two legs' arms, one reading the path door and
    validation both take. So no lattice program reaches a junction that
    validation decides tangent or one carrier: the door has built it as
    a corner, or refused it at the verb."""

    def test_a_line_into_an_arc_turning_sqrt_eps_is_a_corner_and_no_row(self):
        # Turned by sqrt(eps) at the unit arm of both legs: a definite
        # corner, so the profile carries no `profile_junction` row.
        phi = math.sqrt(1e-9)
        cx, cy = 1.0 - math.sin(phi), math.cos(phi)
        loop = (
            Open.at((0 * m, 0 * m))
            .line_to((1 * m, 0 * m))
            .arc_to(Center((cx * m, cy * m), ArcSweep.Ccw, (cx * m, (cy + 1.0) * m)))
            .line_to(Start)
        )
        doc = Doc()
        node = doc.insert(Node.profile(loop))
        ev = evaluate(doc)
        self.assertTrue(ev.succeeded(node))
        self.assertEqual(ev.coincidences(node), [])

    def test_a_leg_on_one_carrier_refuses_at_the_verb(self):
        # A 1 mm leg rising 1e-10 m off a 10 m line: its turn levered by
        # the 1 mm arm is 0.1 eps, Zero, so the undeclared tangent
        # junction refuses at the verb that authored it.
        tip = Open.at((0 * m, 0 * m)).line_to((10 * m, 0 * m))
        with self.assertRaises(pncad.PathError) as caught:
            tip.line_to((10.001 * m, 1e-10 * m))
        self.assertEqual(caught.exception.variant, "junction_tangent")


if __name__ == "__main__":
    unittest.main()
