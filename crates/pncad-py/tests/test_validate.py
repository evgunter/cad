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
presence is the whole question). The corpus is the tour's own scene,
loaded through `test_assembly_eval.opened` rather than rebuilt here,
so the geometry under test is geometry the Rust side already asserts
about.

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
    assemble,
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

    def test_the_census_findings_still_arrive_as_debug_guts(self):
        """MEASURED, and it is a defect this door was the first to
        reach — pinned so its fix goes red rather than silent.

        Every other typed refusal in this library reads as prose: the
        binding asserts it at its single raise site, and the assertion
        panics rather than shipping a `Debug` dump. Three tier-3′
        arms are worded by the KERNEL out of `Debug`, and only tier 3′
        produces them — so binding the fourth rung is what first made
        the assertion fire, on the ordinary call above.

        Filed as `work/lib/tier-3-prime-findings-render-through-
        debug.md`; not fixed here, because the rendering is the
        kernel's and re-wording it at the boundary would fork a
        diagnosis the kernel already owns. Until it lands, the raise
        takes a narrow single-caller exemption. The Rust half of this
        pin is `src/tests.rs::the_census_findings_are_not_prose_by_
        this_crate_s_own_rule`.
        """
        message = str(self.refusal())
        self.assertIn(" { ", message, "the struct-brace fingerprint")
        # Both halves of the arm render through `Debug`: the census
        # contact itself, and the witness position the kernel builds
        # with `format!(\"{p:?}\")`.
        self.assertIn("VertexOnFace {", message)
        self.assertIn("Point3 {", message)

    def test_no_per_arm_tag_crosses_and_the_census_says_so(self):
        """The census's `CensusContact: INTERIOR` row states that which
        coincidence was found is not something Python can read. That is
        a claim about this exception, so it is checked here: `door` and
        `failure_count` are the whole structured payload."""
        refusal = self.refusal()
        self.assertFalse(hasattr(refusal, "kind"))
        self.assertFalse(hasattr(refusal, "variant"))
        self.assertIsInstance(refusal, pncad.PncadError)


if __name__ == "__main__":
    unittest.main()
