"""The validator ladder from Python, and its fourth rung.

`Body` bound three rungs from the start — `validate` (tiers 1+2),
`validate_closed` (tier 2 alone) and `validate_geometric` (tier 3).
LIB-B-VALIDATE4 binds the fourth, `validate_pseudomanifold`: tier 3's
whole local battery PLUS the global coincidence census tier 3 defers,
certified against the body's own declared contacts.

WHAT MAKES THE FOURTH RUNG DIFFERENT, AND WHY IT NEEDED A DECISION
------------------------------------------------------------------
The Rust door takes TWO arguments — a body and a `ContactRecords` —
and the second one has no Python spelling. It is not a gap: a record
set is minted BY the ops that certify geometry, so there is no
constructor to bind, and a door that accepted one would either be
uncallable or would let a caller hand one body another body's
declarations. That pairing is the exact mistake tier 3′ exists to
refuse ("the validator never blesses discovered contacts").

So the records are CAPTURED: a `Body` carries the declarations its
producer minted for it, and the Python door is a bare method like the
three rungs below it. That claim is only worth what a test can show,
and showing it needs the SAME geometry through two doors that differ
in nothing but whether declarations rode along. The mated bench stand
is that pair — `assemble` gates and mints, `product` gathers and
declares nothing — and `TestTheRecordsRideWithTheBody` is the row.

THE ORACLES ARE DOORS, NOT RESTATED ARITHMETIC
----------------------------------------------
Nothing below asserts a hand-computed verdict. The fourth rung is
checked against the third (which already shipped), against the
`assemble` gate (which takes the same verdict attributed, and whose
passing is the precondition for reaching an `Assembly` at all), and
against `Assembly.minted` (which counts the declarations whose
presence is the whole question). The geometry is the tour's own bench,
taken through `test_assembly_eval.opened` rather than rebuilt here, so
this file and the two assembly files agree on the scene by
construction (`bench_scene.py` is the one definition of it).

ONE THING MEASURED AND NOT PINNED
---------------------------------
A DECLARED glue — two slabs resting face to face, unioned through
`Node.boolean(declare=...)` — comes out with an EMPTY record set: the
union welds the declared faces, so no coincidence survives into the
result for a record to back. Its 3′ pass is therefore the empty-record
case, not the certified-seam case, and this file does not pretend
otherwise. The certified-seam case is the assembly's, below. There is
no Python door that reads a body's record count, so the distinction
cannot be asserted from here; it is stated because a reader would
otherwise take `test_a_declared_glue_passes_the_fourth_rung` for
evidence it is not.
"""

import unittest

import pncad
from pncad import (
    BooleanOp,
    Doc,
    DocEdit,
    Node,
    ValidationError,
    ValidationFinding,
    assemble,
    circle,
    evaluate,
    m,
    product,
)
from test_assembly_eval import opened

RUNGS = ("validate", "validate_closed", "validate_geometric", "validate_pseudomanifold")


def slab(doc, x, y, z):
    """The axis-aligned box [x0,x1] x [y0,y1] x [z0,z1], in metres."""
    profile = doc.insert(
        Node.polygon(
            [(x[0], y[0]), (x[1], y[0]), (x[1], y[1]), (x[0], y[1])],
            plane=doc.sketch_frame(elevation=z[0]),
        )
    )
    return doc.insert(Node.extrude(profile, z[1] - z[0]))


def cylinder(doc, centre, radius, z0, height):
    """A right circular cylinder — the curved carrier the census has an
    opinion about that a box does not."""
    profile = doc.insert(
        Node.profile(circle(centre, radius), doc.sketch_frame(elevation=z0))
    )
    return doc.insert(Node.extrude(profile, height))


def two_slabs_resting():
    """A unit slab with a smaller one resting on its top face.

    The upper slab's four bottom corners land strictly inside the
    lower's top face, so the coincidence is real and its class is a
    REST — the shape `find_flush_candidates` reports and `Node.declare`
    records.
    """
    doc = Doc()
    lower = slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
    upper = slab(doc, (0.25 * m, 0.75 * m), (0.25 * m, 0.75 * m), (1 * m, 1.5 * m))
    return doc, lower, upper


class TestTheFourthRungIsTheStrictest(unittest.TestCase):
    """3′ is tier 3 plus the census actually run. Both halves of that
    sentence are checked: it agrees with tier 3 where tier 3 has an
    opinion, and it refuses where only the census looks."""

    def test_a_plain_body_passes_every_rung(self):
        doc = Doc()
        box = slab(doc, (0 * m, 1 * m), (0 * m, 2 * m), (0 * m, 3 * m))
        body = evaluate(doc).value(box).body()
        for rung in RUNGS:
            with self.subTest(rung=rung):
                getattr(body, rung)()  # raises ValidationError if it fails

    def test_the_census_is_what_the_fourth_rung_adds(self):
        """The separating pin, and the reason the rung is not
        redundant: ONE body, tier 3 clean, tier 3′ refusing.

        Two solids that touch, gathered as the document's product.
        `product` declares nothing — it is the root list side by side,
        with no gate — so the seat between them is a coincidence no
        record backs. Tier 3's battery is per-entity and never looks
        across the pair, which is exactly the deferral 3′ closes.
        """
        doc, lower, upper = two_slabs_resting()
        doc.apply(DocEdit.set_roots([lower, upper]))
        gathered = product(doc, evaluate(doc))

        gathered.validate()
        gathered.validate_closed()
        gathered.validate_geometric()
        with self.assertRaises(ValidationError) as caught:
            gathered.validate_pseudomanifold()
        self.assertEqual(caught.exception.door, "validate_pseudomanifold")
        self.assertGreater(caught.exception.failure_count, 0)

    def test_separated_solids_pass_the_census_they_run(self):
        """The control for the row above: the census RUNS here too and
        finds nothing, so the refusal is about the touching and not
        about `product` bodies as such."""
        doc = Doc()
        a = slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        b = slab(doc, (3 * m, 4 * m), (0 * m, 1 * m), (0 * m, 1 * m))
        doc.apply(DocEdit.set_roots([a, b]))
        product(doc, evaluate(doc)).validate_pseudomanifold()

    def test_an_interpenetrating_union_has_no_coincidence_to_declare(self):
        """A union whose operands genuinely overlap needs no
        declaration and leaves no seam — so the strictest rung passes
        a boolean result on the ordinary path."""
        doc = Doc()
        base = slab(doc, (0 * m, 3 * m), (0 * m, 2 * m), (0 * m, 1 * m))
        post = slab(doc, (0.5 * m, 1.5 * m), (0.5 * m, 1.5 * m), (0.5 * m, 2 * m))
        fused = doc.insert(Node.boolean(BooleanOp.Union, base, post))
        evaluate(doc).value(fused).body().validate_pseudomanifold()

    def test_a_declared_glue_passes_the_fourth_rung(self):
        """The declare protocol's result, through the fourth rung.

        See the module header: the union WELDS the declared faces, so
        this is the empty-record case. It is here because the door a
        caller reaches a glued body through must answer, not because
        it demonstrates the capture.
        """
        doc, lower, upper = two_slabs_resting()
        findings = evaluate(doc).find_flush_candidates(lower, upper)
        self.assertEqual(len(findings), 1)
        decl = doc.declare_all(findings)
        glued = doc.insert(Node.boolean(BooleanOp.Union, lower, upper, declare=decl))
        body = evaluate(doc).value(glued).body()
        body.validate_pseudomanifold()
        # The glue is a glue: the exact dyadic volume of the two parts.
        self.assertEqual(body.mass_properties().volume, 1.125)


class TestTheRecordsRideWithTheBody(unittest.TestCase):
    """The family's load-bearing claim: the declarations are captured
    with the body, so the verdict turns on which door minted it.

    ONE geometry, two doors. `assemble` gates the product against its
    mates' minted records and hands back a body carrying them;
    `product` gathers the identical solids and declares nothing. If the
    capture were decorative both would answer alike."""

    @classmethod
    def setUpClass(cls):
        cls.store, cls.docs = opened()

    def evaluated(self, label):
        doc = self.docs[label]
        return doc, evaluate(doc, resolver=self.store)

    def test_the_same_geometry_answers_differently_through_the_two_doors(self):
        doc, ev = self.evaluated("stand")
        gathered = product(doc, ev)
        assembly = assemble(doc, ev)

        # It IS the same geometry — the gather and the gate agree on
        # the body, which `test_assembly_author` already pins and this
        # row depends on, so it is re-asserted rather than assumed.
        self.assertEqual(
            gathered.mass_properties().volume,
            assembly.body.mass_properties().volume,
        )
        # And on every rung that does not consult declarations.
        for rung in ("validate", "validate_closed", "validate_geometric"):
            with self.subTest(rung=rung):
                getattr(gathered, rung)()
                getattr(assembly.body, rung)()

        # The fourth rung is where they part. The at-rest body's
        # declarations back its seats; the gathered body has none, so
        # every seat is an undeclared contact.
        assembly.body.validate_pseudomanifold()
        with self.assertRaises(ValidationError) as caught:
            gathered.validate_pseudomanifold()
        self.assertEqual(caught.exception.door, "validate_pseudomanifold")
        self.assertGreater(caught.exception.failure_count, 0)

    def test_the_declarations_the_gate_minted_are_what_it_carries(self):
        """The stand's mates are the source of those records, and
        `minted` is the Python-visible count of them: two solved mates,
        two declarations, and reaching an `Assembly` at all means the
        kernel's at-rest gate already passed over this exact pair."""
        doc, ev = self.evaluated("stand")
        assembly = assemble(doc, ev)
        self.assertEqual(len(assembly.minted), 2)
        # The same verdict re-taken, un-attributed.
        assembly.body.validate_pseudomanifold()

    def test_a_mate_less_assembly_is_the_control(self):
        """The flat-pack layout: same doors, no mates, no declarations
        — and the two bodies agree, because its solids are disjoint.

        Without this row the pair above would only show that
        `assemble` answers more kindly than `product`. It shows
        instead that they differ exactly when declarations exist to
        differ about."""
        doc, ev = self.evaluated("layout")
        assembly = assemble(doc, ev)
        self.assertEqual(assembly.minted, [])
        assembly.body.validate_pseudomanifold()
        product(doc, ev).validate_pseudomanifold()


class TestTheRefusalsShape(unittest.TestCase):
    """What a caller reads off a tier-3′ refusal, and what they
    cannot."""

    def refusal(self):
        doc, lower, upper = two_slabs_resting()
        doc.apply(DocEdit.set_roots([lower, upper]))
        gathered = product(doc, evaluate(doc))
        with self.assertRaises(ValidationError) as caught:
            gathered.validate_pseudomanifold()
        return caught.exception

    def test_every_rung_names_itself_on_the_refusal(self):
        """`door` is the branchable half, and the four rungs share one
        exception class — so the tag is how a caller tells which gate
        spoke. The fourth is pinned against a real refusal; the other
        three are pinned as the strings they already are."""
        self.assertEqual(self.refusal().door, "validate_pseudomanifold")
        self.assertIn("validate_pseudomanifold reported", str(self.refusal()))

    def test_the_count_is_the_findings_and_it_is_deterministic(self):
        first, second = self.refusal(), self.refusal()
        self.assertEqual(first.failure_count, second.failure_count)
        self.assertEqual(str(first), str(second))
        # Every finding is joined into the message, so the count and
        # the separators agree: n findings, n-1 joins.
        self.assertEqual(
            str(first).count("tier-3′ census:"), first.failure_count
        )

    def test_the_message_is_the_kernel_s_own_diagnosis(self):
        message = str(self.refusal())
        self.assertIn("undeclared contact", message)
        self.assertIn("never blessed from discovery", message)

    def test_the_census_findings_arrive_as_prose(self):
        """The tier-3′ arms are the ones that reach a caller through a
        KERNEL rendering rather than a binding one, and they read as
        prose like every other typed refusal in this library.

        They did not always. `UndeclaredContact` rendered its census
        contact through `Debug`, `StaleContactDeclaration` its stale
        declaration, and the witness position was a `Point3`'s derived
        rendering — so the first honest call of this door tripped the
        binding's own prose assertion. Each renders through `Display`
        now, the raise takes no exemption, and this row is the pin that
        the guts do not come back. The Rust half is
        `src/tests.rs::the_census_findings_read_as_prose_by_this_crate_s_
        own_rule`.
        """
        message = str(self.refusal())
        self.assertNotIn(" { ", message, "the struct-brace fingerprint")
        self.assertNotIn("Point3 {", message)
        # The prose still carries what the Debug guts carried: which
        # entities coincide, and where. A finding that lost its
        # witness would read as prose and say nothing actionable.
        self.assertIn("vertex", message)
        self.assertRegex(message, r"at \(-?\d")

    def test_the_per_arm_words_cross_and_the_census_says_so(self):
        """Which coincidence the census found IS something Python can
        read, and this is the row that says so.

        It used to say the opposite. `CensusContact` and
        `CensusSubject` were `INTERIOR` in the binding census and this
        test asserted the absence — `kind` and `variant` were checked
        NOT to exist, because `door` and `failure_count` were the whole
        structured payload and which arm refused was prose. Turning
        that around is what closing the question cost, so the pin is
        rewritten rather than deleted: the scalar words a caller might
        reach for are still absent (one raise carries N findings, so
        neither could name one of them), and the sequence is where the
        arms live.
        """
        refusal = self.refusal()
        self.assertFalse(hasattr(refusal, "kind"))
        self.assertFalse(hasattr(refusal, "variant"))
        self.assertIsInstance(refusal, pncad.PncadError)
        self.assertTrue(
            all(f.variant == "undeclared_contact" for f in refusal.findings)
        )
        self.assertEqual(
            {f.contact_kind for f in refusal.findings},
            {"vertex_on_face", "edge_face_overlap"},
        )

    def test_the_findings_are_the_count(self):
        """`failure_count` is the sequence's length, at every rung that
        can refuse — the invariant that makes the count readable as an
        index bound rather than as a second, independent number."""
        refusal = self.refusal()
        self.assertEqual(len(refusal.findings), refusal.failure_count)
        self.assertTrue(
            all(isinstance(f, ValidationFinding) for f in refusal.findings)
        )

    def test_each_finding_s_word_is_the_message_s_own(self):
        """The words and the prose are one diagnosis, not two. Every
        variant a finding names is a phrase the joined message spells
        for itself — so a caller that branches on the word and a reader
        who reads the sentence are told the same thing."""
        refusal = self.refusal()
        message = str(refusal)
        for finding in refusal.findings:
            with self.subTest(variant=finding.variant):
                self.assertIn(finding.variant.replace("_", " "), message)
            if finding.contact_kind is not None:
                with self.subTest(contact=finding.contact_kind):
                    self.assertIn(finding.contact_kind.split("_")[0], message)

    def test_a_finding_is_a_frozen_value_that_compares_structurally(self):
        """The value shape: two findings that say the same thing ARE
        the same thing, so a caller can put them in a set and ask which
        KINDS of failure a body has without deduplicating by hand."""
        first, second = self.refusal(), self.refusal()
        self.assertEqual(first.findings, second.findings)
        self.assertEqual(first.findings[0], second.findings[0])
        self.assertEqual(
            len({(f.variant, f.contact_kind) for f in first.findings}), 2
        )
        # Frozen: a finding is restated by re-running the validator,
        # never by editing one in place.
        with self.assertRaises(AttributeError):
            first.findings[0].variant = "something_else"

    def test_every_finding_carries_every_attribute(self):
        """No `getattr` trap. Six attributes on every finding, `None`
        where the arm carries nothing to fill them — so a caller reads
        `subject_kind` without first branching on `variant`."""
        for finding in self.refusal().findings:
            for attribute in (
                "variant",
                "subject_kind",
                "entity_kind",
                "contact_kind",
                "stale_kind",
                "ring_contact_kind",
            ):
                with self.subTest(attribute=attribute):
                    self.assertTrue(hasattr(finding, attribute))
            self.assertIsInstance(finding.variant, str)
            # This scene's arms carry a contact and no subject; the
            # `entity`/`face_pair` half is pinned in Rust, below.
            self.assertIsNone(finding.subject_kind)
            self.assertIsNone(finding.entity_kind)
            # An undeclared coincidence is neither an unconfirmed
            # declaration nor a ring standing on its outer loop, and
            # `None` is what says so on the arm that carries neither.
            self.assertIsNone(finding.stale_kind)
            self.assertIsNone(finding.ring_contact_kind)

    def test_two_distinct_arms_arrive_off_one_raise(self):
        """The claim the sequence exists for: ONE raise, several arms,
        each named.

        A cylinder resting on a slab, gathered by `product`, is the
        smallest scene that reaches two: the flat seat under the
        cylinder is an undeclared contact, and the curved face within
        reach of the slab's is a candidate the census can neither
        examine nor definitely clear, refused as undecidable rather
        than silently not looked at. Before this the two differed only
        in prose.
        """
        doc = Doc()
        seat = slab(doc, (0 * m, 2 * m), (0 * m, 2 * m), (0 * m, 1 * m))
        post = cylinder(doc, (1 * m, 1 * m), 0.4 * m, 1 * m, 0.5 * m)
        doc.apply(DocEdit.set_roots([seat, post]))
        gathered = product(doc, evaluate(doc))
        with self.assertRaises(ValidationError) as caught:
            gathered.validate_pseudomanifold()
        refusal = caught.exception
        self.assertEqual(len(refusal.findings), refusal.failure_count)
        self.assertEqual(
            {f.variant for f in refusal.findings},
            {"undeclared_contact", "census_undecidable"},
        )
        # The payload rides only where the arm carries one.
        for finding in refusal.findings:
            with self.subTest(variant=finding.variant):
                if finding.variant == "undeclared_contact":
                    self.assertEqual(finding.contact_kind, "vertex_on_face")
                else:
                    self.assertIsNone(finding.contact_kind)

    def test_the_arms_this_suite_cannot_reach_are_named(self):
        """WHAT PYTHON CANNOT PRODUCE, said rather than left implied.

        `ValidationError` has seventy-one arms and Python reaches them
        through four `Body` methods. The structural and geometric arms
        want a corrupt arena or an uncertifiable surface, and the
        public API's every product is tier-1-valid, so no authoring
        script can mint one. `census_unsupported` and
        `census_lane_unsupported` — the two arms that carry the
        `subject_kind` / `entity_kind` half of a finding — want a
        carrier outside the certifiable inventory or a scalar with no
        certified chart-overlap lane, and neither is reachable through
        the doors this suite has: extruded boxes, cylinders and lofts
        all certify.

        So those two are pinned in Rust, where the refusal constructs
        (`src/tests.rs::every_validation_finding_carries_every_word_
        its_arm_has`), and this row is the statement that the gap is
        the DOORS' and not the projection's. What Python reaches is
        the census pair above.

        The two payload arms below are the same statement about the
        same doors, and each has its own reason:

        - `stale_contact_declaration` (`stale_kind`) wants a declared
          record the geometry stopped backing. Every door that hands
          Python a body WITH declarations either mints them from the
          geometry it is looking at — `Value.body` off a boolean,
          whose surviving records are the ones the result still
          witnesses — or gates them first: `assemble` answers an
          `Assembly` only after tier 3′ passed over exactly that pair,
          and refuses with `AssemblyError` when it does not. `product`
          gathers and declares nothing, so its bodies are plain. A
          `ContactRecords` has no Python spelling, so nothing here can
          part a record from its witness; the kernel's own suites do
          it by tampering with the record set directly.
        - `ring_meets_outer` (`ring_contact_kind`) wants a face whose
          ring stands on its own outer loop. That is built by raw
          Euler surgery — the shell verb's suites glue a lifted
          counterpart chart on with `kfmrh` to make one — and this
          surface exposes no Euler operator; every body Python holds
          came out of a verb that validated it.

        Both are pinned in Rust by construction, one row per arm
        (`src/tests.rs::every_stale_declaration_arm_projects_the_
        payload_it_carries`, and its ring counterpart).
        """
        doc = Doc()
        seat = slab(doc, (0 * m, 2 * m), (0 * m, 2 * m), (0 * m, 1 * m))
        post = cylinder(doc, (1 * m, 1 * m), 0.4 * m, 1 * m, 0.5 * m)
        doc.apply(DocEdit.set_roots([seat, post]))
        with self.assertRaises(ValidationError) as caught:
            product(doc, evaluate(doc)).validate_pseudomanifold()
        reached = {f.variant for f in caught.exception.findings}
        self.assertNotIn("census_unsupported", reached)
        self.assertNotIn("census_lane_unsupported", reached)
        self.assertNotIn("stale_contact_declaration", reached)
        self.assertNotIn("ring_meets_outer", reached)

    def test_a_declared_glue_leaves_no_record_for_the_census_to_miss(self):
        """The nearest a Python scene gets to a stale declaration, and
        why it is not one.

        A declared rest between two slabs, wired into the union that
        welds it: the seam the declaration names is consumed by the
        boolean, so the result carries no record that could lose its
        witness, and the fourth rung passes. This is the row behind
        the reason `stale_kind` is pinned in Rust rather than driven
        from here — the scene reaches the declare/boolean pair, which
        is the only door that hands Python a body carrying records it
        did not gate, and it still cannot mint an unwitnessed one.
        """
        doc, lower, upper = two_slabs_resting()
        findings = evaluate(doc).find_flush_candidates(lower, upper)
        self.assertEqual(len(findings), 1)
        declaration = doc.declare(findings[0])
        glued = doc.insert(
            Node.boolean(BooleanOp.Union, lower, upper, declare=declaration)
        )
        body = evaluate(doc).value(glued).body()
        body.validate_pseudomanifold()  # raises if a record went stale


if __name__ == "__main__":
    unittest.main()
