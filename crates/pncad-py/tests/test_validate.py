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
is that pair — an instance of it carries the records its census
minted across the document seam, `product` gathers and declares
nothing — and `TestTheRecordsRideWithTheBody` is the row.

THE ORACLES ARE DOORS, NOT RESTATED ARITHMETIC
----------------------------------------------
Nothing below asserts a hand-computed verdict. The fourth rung is
checked against the third (which already shipped), and against the
at-rest check, which reads the same census. The geometry is the tour's own bench,
taken through `test_assembly_eval.opened` rather than rebuilt here, so
this file and the two assembly files agree on the scene by
construction (`bench_scene.py` is the one definition of it).

ONE THING MEASURED AND NOT PINNED
---------------------------------
A glue — two slabs resting face to face, unioned through
`Node.union` — comes out with an EMPTY record set: the union welds
the coincident faces, so no coincidence survives into the
result for a record to back. Its 3′ pass is therefore the empty-record
case, not the certified-seam case, and this file does not pretend
otherwise. The certified-seam case is the assembly's, below. There is
no Python door that reads a body's record count, so the distinction
cannot be asserted from here; it is stated because a reader would
otherwise take `test_a_glued_rest_passes_the_fourth_rung` for
evidence it is not.
"""

import math
import unittest

import pncad
from pncad import (
    Doc,
    Formula,
    Frame,
    Node,
    Open,
    Settle,
    Start,
    StationMove,
    ValidationError,
    DocRef,
    ValidationFinding,
    circle,
    circle_split,
    evaluate,
    m,
    product,
    rad,
    run_checks,
)

from placing import against, placed, assemble_world_copies
from test_assembly_eval import opened
from measuring import PINNED, pinned

RUNGS = ("validate", "validate_closed", "validate_geometric", "validate_pseudomanifold")


def lift(z):
    """The translation `z` up."""
    return Frame.translation((0 * m, 0 * m, z))


def slab(doc, x, y, height):
    """The box [x0,x1] x [y0,y1] x [0,height] in its own coordinates,
    in metres; where it stands is its placement's."""
    profile = doc.insert(
        Node.polygon(
            [
                (Formula.literal(x[0]), Formula.literal(y[0])),
                (Formula.literal(x[1]), Formula.literal(y[0])),
                (Formula.literal(x[1]), Formula.literal(y[1])),
                (Formula.literal(x[0]), Formula.literal(y[1])),
            ],
        )
    )
    return doc.insert(Node.extrude(profile, Formula.literal(height)))


def cylinder(doc, centre, radius, height):
    """A right circular cylinder in its own coordinates — the curved
    carrier the census has an opinion about that a box does not."""
    profile = doc.insert(Node.profile(circle(centre, radius)))
    return doc.insert(Node.extrude(profile, Formula.literal(height)))


# The upper slab of `two_slabs_resting` stands this far up.
RISE = 1 * m


def two_slabs_resting():
    """A unit slab and a smaller one, which rests on its top face once
    it stands `RISE` up.

    The upper slab's four bottom corners land strictly inside the
    lower's top face, so the coincidence is real and its class is a
    REST, which a union glues.
    """
    doc = Doc()
    lower = slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), 1 * m)
    upper = slab(doc, (0.25 * m, 0.75 * m), (0.25 * m, 0.75 * m), 0.5 * m)
    return doc, lower, upper


def place_resting(doc, lower, upper):
    """Both slabs of `two_slabs_resting` placed in the world, the upper
    resting on the lower."""
    doc.place(lower, doc.world)
    placed(doc, upper, lift(RISE))


class TestTheFourthRungIsTheStrictest(unittest.TestCase):
    """3′ is tier 3 plus the census actually run. Both halves of that
    sentence are checked: it agrees with tier 3 where tier 3 has an
    opinion, and it refuses where only the census looks."""

    def test_a_plain_body_passes_every_rung(self):
        doc = Doc()
        box = slab(doc, (0 * m, 1 * m), (0 * m, 2 * m), 3 * m)
        body = evaluate(doc).value(box).body()
        for rung in RUNGS:
            with self.subTest(rung=rung):
                getattr(body, rung)()  # raises ValidationError if it fails

    def test_the_census_is_what_the_fourth_rung_adds(self):
        """The separating pin, and the reason the rung is not
        redundant: ONE body, tier 3 clean, tier 3′ refusing.

        Two solids that touch, gathered as the document's product.
        `product` declares nothing — it is the placed copies side by
        side, with no gate — so the seat between them is a coincidence no
        record backs. Tier 3's battery is per-entity and never looks
        across the pair, which is exactly the deferral 3′ closes.
        """
        doc, lower, upper = two_slabs_resting()
        place_resting(doc, lower, upper)
        assemble_world_copies(doc)
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
        a = slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), 1 * m)
        b = slab(doc, (3 * m, 4 * m), (0 * m, 1 * m), 1 * m)
        doc.place(a, doc.world)
        doc.place(b, doc.world)
        assemble_world_copies(doc)
        product(doc, evaluate(doc)).validate_pseudomanifold()

    def test_an_interpenetrating_union_has_no_coincidence_to_record(self):
        """A union whose operands genuinely overlap meets on no
        surface and leaves no seam — so the strictest rung passes
        a boolean result on the ordinary path."""
        doc = Doc()
        base = slab(doc, (0 * m, 3 * m), (0 * m, 2 * m), 1 * m)
        post = slab(doc, (0.5 * m, 1.5 * m), (0.5 * m, 1.5 * m), 1.5 * m)
        post = against(doc, base, post, lift(0.5 * m))
        fused = doc.insert(Node.union([base, post]))
        evaluate(doc).value(fused).body().validate_pseudomanifold()

    def test_a_glued_rest_passes_the_fourth_rung(self):
        """A glued rest, through the fourth rung.

        See the module header: the union WELDS the coincident faces, so
        this is the empty-record case. It is here because the door a
        caller reaches a glued body through must answer, not because
        it demonstrates the capture.
        """
        doc, lower, upper = two_slabs_resting()
        upper = against(doc, lower, upper, lift(RISE))
        glued = doc.insert(Node.union([lower, upper]))
        body = evaluate(doc).value(glued).body()
        body.validate_pseudomanifold()
        # The glue is a glue: the exact dyadic volume of the two parts.
        self.assertEqual(pinned(body).volume, 1.125)


class TestTheRecordsRideWithTheBody(unittest.TestCase):
    """The family's load-bearing claim: the declarations are captured
    with the body, so the verdict turns on which door minted it.

    ONE geometry, two doors. An instance of the document, standing at
    the identity in another one, is a body carrying the records the
    document's census minted for its seats (D-1, across the seam);
    `product` gathers the identical solids and declares nothing. If the
    capture were decorative both would answer alike."""

    @classmethod
    def setUpClass(cls):
        cls.store, cls.docs = opened()

    def evaluated(self, label):
        doc = self.docs[label]
        return doc, evaluate(doc, resolver=self.store)

    def instanced(self, label):
        """The body of an instance of `label`'s document, at the
        identity in a document of its own: the document's product with
        the records its census minted."""
        doc = self.docs[label]
        outer = Doc(f"validate-instance-of-{label}")
        instance = outer.insert(
            Node.instantiate_part(
                DocRef(doc.id, self.store.current_pin(doc.id)), outer.world
            )
        )
        outer.set_product(instance)
        return evaluate(outer, resolver=self.store).value(instance).body()

    def test_the_same_geometry_answers_differently_through_the_two_doors(self):
        doc, ev = self.evaluated("stand")
        gathered = product(doc, ev)
        instanced = self.instanced("stand")

        # It IS the same geometry — the gather and the instance agree on
        # the body, so it is asserted rather than assumed.
        self.assertEqual(
            pinned(gathered).volume,
            pinned(instanced).volume,
        )
        # And on every rung that does not consult declarations.
        for rung in ("validate", "validate_closed", "validate_geometric"):
            with self.subTest(rung=rung):
                getattr(gathered, rung)()
                getattr(instanced, rung)()

        # The fourth rung is where they part. The instance's records
        # back its seats; the gathered body has none, so every seat is
        # an unrecorded contact.
        instanced.validate_pseudomanifold()
        with self.assertRaises(ValidationError) as caught:
            gathered.validate_pseudomanifold()
        self.assertEqual(caught.exception.door, "validate_pseudomanifold")
        self.assertGreater(caught.exception.failure_count, 0)

    def test_the_body_carries_a_record_for_each_seat(self):
        """The stand's two mated seats are two touches between copies,
        and the census records each as one face pair. Without the
        records the gathered body refuses on exactly those two pairs,
        and carrying them the instance's body passes: the body carries a
        record for each seat. (Python sees no record count, so an extra
        record that backs nothing is outside what this row can see.)"""
        doc, ev = self.evaluated("stand")
        with self.assertRaises(ValidationError) as caught:
            product(doc, ev).validate_pseudomanifold()
        self.assertEqual(
            [(f.variant, f.contact_kind) for f in caught.exception.findings],
            [("unrecorded_contact", "conformal_patch")] * 2,
        )
        self.instanced("stand").validate_pseudomanifold()

    def test_a_mate_less_assembly_is_the_control(self):
        """The flat-pack layout: same doors, no mates, no touches —
        and the two bodies agree, because its solids are disjoint.

        Without this row the pair above would only show that an
        instance answers more kindly than `product`. It shows instead
        that they differ exactly when touches exist to differ about,
        and the at-rest check, reading the same census, has nothing to
        report."""
        doc, ev = self.evaluated("layout")
        self.instanced("layout").validate_pseudomanifold()
        product(doc, ev).validate_pseudomanifold()
        self.assertEqual(run_checks(doc, ev).findings, [])


#: The phrase the kernel's prose spells each census contact word with,
#: for the words the refusal below carries; a new word is a KeyError.
CONTACT_PHRASE = {
    "conformal_patch": "two faces lying against each other",
}


class TestTheRefusalsShape(unittest.TestCase):
    """What a caller reads off a tier-3′ refusal, and what they
    cannot."""

    def refusal(self):
        doc, lower, upper = two_slabs_resting()
        place_resting(doc, lower, upper)
        assemble_world_copies(doc)
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
        # Every finding is joined into the message, and every finding
        # states exactly one recourse, so the count and the recourses
        # agree: n findings, n recourses.
        message = str(first)
        self.assertEqual(
            message.count("Recourse:") + message.count("There is no way through"),
            first.failure_count,
        )

    def test_the_message_is_the_kernel_s_own_diagnosis(self):
        message = str(self.refusal())
        self.assertIn("is an unrecorded contact", message)
        self.assertIn(
            "Recourse: move the geometry so the faces clearly touch or clearly part",
            message,
        )

    def test_the_census_findings_arrive_as_prose(self):
        """The tier-3′ arms are the ones that reach a caller through a
        KERNEL rendering rather than a binding one, and they read as
        prose like every other typed refusal in this library.

        They did not always. `UnrecordedContact` rendered its census
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
        # entities coincide, and where. The census reads the slabs'
        # rest as one face pair over their shared region, and asks
        # nothing of the edges and corners bounding it.
        self.assertIn("two faces lying against each other", message)
        self.assertIn("at their shared region", message)

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
            all(f.variant == "unrecorded_contact" for f in refusal.findings)
        )
        self.assertEqual(
            {f.contact_kind for f in refusal.findings},
            {"conformal_patch"},
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
                    self.assertIn(CONTACT_PHRASE[finding.contact_kind], message)

    def test_a_finding_is_a_frozen_value_that_compares_structurally(self):
        """The value shape: two findings that say the same thing ARE
        the same thing, so a caller can put them in a set and ask which
        KINDS of failure a body has without deduplicating by hand."""
        first, second = self.refusal(), self.refusal()
        self.assertEqual(first.findings, second.findings)
        self.assertEqual(first.findings[0], second.findings[0])
        self.assertEqual(
            {(f.variant, f.contact_kind) for f in first.findings},
            {("unrecorded_contact", "conformal_patch")},
        )
        # Frozen: a finding is restated by re-running the validator,
        # never by editing one in place.
        with self.assertRaises(AttributeError):
            first.findings[0].variant = "something_else"

    def test_every_finding_carries_every_attribute(self):
        """No `getattr` trap. Seven attributes on every finding, `None`
        where the arm carries nothing to fill them — so a caller reads
        `subject_kind` without first branching on `variant`."""
        for finding in self.refusal().findings:
            for attribute in (
                "variant",
                "subject_kind",
                "entity_kind",
                "contact_kind",
                "crossing_side",
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
        cylinder is an unrecorded contact, and the curved face within
        reach of the slab's is a candidate the census can neither
        examine nor definitely clear, refused as undecidable rather
        than silently not looked at. Before this the two differed only
        in prose.
        """
        doc = Doc()
        seat = slab(doc, (0 * m, 2 * m), (0 * m, 2 * m), 1 * m)
        post = cylinder(doc, (1 * m, 1 * m), 0.4 * m, 0.5 * m)
        doc.place(seat, doc.world)
        placed(doc, post, lift(1 * m))
        assemble_world_copies(doc)
        gathered = product(doc, evaluate(doc))
        with self.assertRaises(ValidationError) as caught:
            gathered.validate_pseudomanifold()
        refusal = caught.exception
        self.assertEqual(len(refusal.findings), refusal.failure_count)
        self.assertEqual(
            {f.variant for f in refusal.findings},
            {"unrecorded_contact", "census_undecidable"},
        )
        # The payload rides only where the arm carries one.
        for finding in refusal.findings:
            with self.subTest(variant=finding.variant):
                if finding.variant == "unrecorded_contact":
                    self.assertEqual(finding.contact_kind, "vertex_on_face")
                else:
                    self.assertIsNone(finding.contact_kind)

    def test_the_arms_this_suite_cannot_reach_are_named(self):
        """WHAT PYTHON CANNOT PRODUCE, said rather than left implied.

        Python reaches `ValidationError`'s arms
        through five `Body` methods — the four rungs and
        `validate_geometric_measured`, whose gate half is the third
        rung. The structural and geometric arms
        want a corrupt arena or an uncertifiable surface, and the
        public API's every product is tier-1-valid, so no authoring
        script can mint one. `census_unsupported` and
        `census_lane_unsupported` — the two arms that carry the
        `subject_kind` / `entity_kind` half of a finding — want a
        carrier outside the certifiable inventory or a door that holds
        no certified chart-overlap lane (the `_structural` ones, which
        the binding does not expose), and neither is reachable through
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
          witnesses — or carries them from where they were minted: an
          instance's body holds the records its document's census
          left over that document's own geometry. `product` gathers and
          declares nothing, so its bodies are plain. A
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
        seat = slab(doc, (0 * m, 2 * m), (0 * m, 2 * m), 1 * m)
        post = cylinder(doc, (1 * m, 1 * m), 0.4 * m, 0.5 * m)
        doc.place(seat, doc.world)
        placed(doc, post, lift(1 * m))
        with self.assertRaises(ValidationError) as caught:
            assemble_world_copies(doc)
            product(doc, evaluate(doc)).validate_pseudomanifold()
        reached = {f.variant for f in caught.exception.findings}
        self.assertNotIn("census_unsupported", reached)
        self.assertNotIn("census_lane_unsupported", reached)
        self.assertNotIn("stale_contact_declaration", reached)
        self.assertNotIn("ring_meets_outer", reached)


def stacked_loft(doc, sections, length, radius):
    """A loft up its first section's own +z through `sections` closed
    loops, `length` long, the loop at station `t` given by `radius(t)`.

    The one shape in reach of these bindings whose walls are
    QUADRATURE faces rather than closed forms, which is what makes it
    the fixture for a door about what a quadrature costs.
    """
    first = doc.insert(Node.profile(radius(0.0)))
    then = []
    for i in range(1, sections):
        t = i / (sections - 1)
        then.append(
            (
                doc.insert(Node.profile(radius(t))),
                [StationMove.slide("z", Formula.length_in(length * t, m))],
            )
        )
    node = doc.insert(Node.loft(first, then, Formula.count(1)))
    run = evaluate(doc)
    assert run.succeeded(node), "the loft fixture evaluates"
    return run.value(node).body()


def polygonal_loft():
    """A tapered square tube: the sides are STRAIGHT, so the lofted
    walls are polynomial and every refinement round is cheap."""

    def square(t):
        half = (0.024 * (1 - t) + 0.012 * t) * m
        return (
            Open.at((-half, -half))
            .line_to((half, -half))
            .line_to((half, half))
            .line_to((-half, half))
            .line_to(Start)
        )

    return stacked_loft(Doc(), 3, 0.125, square)


def oversized_round_loft():
    """A tapered round tube 8 km long: RATIONAL walls, and a part big
    enough that the certified quadrature cannot reach its target.

    The size is the point rather than an accident. The reporting
    target is `1024 * eps`, a LENGTH that shrinks with the run's
    tolerance, while the width a rational wall reaches by the face
    work wall (`FACE_WORK_WALL`) is a length that grows with the
    PART — so at the suite's own eps the two cross only on a part
    this large. Measured on this tube at the default eps: at 125 m it
    certifies, at 1 km (8x) it certifies, and at 8 km (64x) each wall
    stops at the work wall after 11 rounds, 1.400e-6 m wide against
    the 1.024e-6 m target. The crossing is near 47x, so 64x is the
    first power of two clear of it.
    """
    return stacked_loft(
        Doc(),
        2,
        8000.0,
        lambda t: circle_split(
            (0.0 * m, 0.0 * m), (1497.6 * (1 - t) + 748.8 * t) * m, 2, 0.0 * rad
        ),
    )


class TestGateAndMeasureInOneQuadrature(unittest.TestCase):
    """**`validate_geometric_measured`** — the rung that hands back the
    number its own +V check computed.

    Tier 3's check 7 IS a certified quadrature, so the natural pair,
    `validate_geometric()` then `mass_properties()`, runs one over the
    body and then starts another from round 0. This door continues the
    gate's own certificate to the reporting target instead.

    WHAT IS NOT ASSERTED HERE, AND WHY
    ----------------------------------
    **That it pays once.** The claim is a COUNT — one certified
    quadrature where the pair runs two — and this suite cannot see
    one. The kernel counts piece evaluations through its telemetry
    scalar, which the bindings do not evaluate at (they are `f64`
    alone, `test_measures.py`'s header states the same limit for the
    clearance engine), and a wall-clock stand-in would be a threshold
    on a shared runner's schedule AND would be false on half the
    bodies it could run on: the saving is a property of the body's
    refinement schedule, not of the door. A body whose sign stays
    undecided to the end leaves the continuation nothing to run and
    saves the whole second quadrature; a body whose sign settles early
    leaves rounds open, the lane re-derives each open face's setup,
    and the saving is nil. Both are correct and the kernel's own
    `SignCertificate::measure` docs carry the regime. What this suite can
    show — and does, below — is the OBSERVABLE consequence of one
    quadrature: the door's answer is the reporting walk's own bits.

    **The gate half refusing.** `validate_geometric` refuses on a
    corrupt arena or an uncertifiable surface, and the public API
    mints neither — `TestTheRefusalsShape` names that as this suite's
    standing blind spot. So the door's gate-refusal path is
    unreachable from an authoring script for the same reason the third
    rung's own is; what IS pinned below is which rung the gate is.
    """

    def test_the_answer_is_the_reporting_door_s_own_bits(self):
        """All four fields, `==` and not `assertAlmostEqual`.

        This is the observable form of "one quadrature": the claim is
        that the continuation and the reporting walk are the SAME
        computation over the same rounds in the same arena order, and
        an almost-equality would pass just as well for two different
        quadratures that happened to agree.
        """
        body = polygonal_loft()
        once = pinned(body)
        twice = pinned(body)
        self.assertEqual(once.volume, twice.volume)
        self.assertEqual(once.surface_area, twice.surface_area)
        self.assertEqual(once.volume_pad, twice.volume_pad)
        self.assertEqual(once.area_pad, twice.area_pad)

    def test_the_gate_is_the_third_rung_and_not_the_fourth(self):
        """Which gate this door runs, pinned on the one body where the
        two rungs disagree.

        `two_slabs_resting` gathered by `product` is tier-3 clean and
        tier-3′ refusing — that is what
        `test_the_census_is_what_the_fourth_rung_adds` uses it for. So
        a door wired to the fourth rung reds here and a door wired to
        the third does not, which no assertion about a passing body
        could separate.
        """
        doc, lower, upper = two_slabs_resting()
        place_resting(doc, lower, upper)
        assemble_world_copies(doc)
        gathered = product(doc, evaluate(doc))
        with self.assertRaises(ValidationError):
            gathered.validate_pseudomanifold()
        measured = pinned(gathered)
        self.assertEqual(measured.volume, pinned(gathered).volume)

    def test_a_body_tier_3_admits_at_an_unmet_resolution_answers_its_bracket(self):
        """An unmet resolution is the bracket, not a refusal.

        A body can be tier-3 VALID — its volume's SIGN is definite,
        which is all check 7 reads — and still stop short of a
        resolution at this eps: a rational wall at any reachable size
        stops at the work wall, its removable width still dominant
        there. The door answers the bracket it reached, with the stop
        that says why, and the bracket is this body's.

        TWO certified quadratures are run here, on one body, and that is
        deliberate: this row is the suite's most expensive. The first is
        the resolution's; the second is a deciding predicate run out on
        the same walls, the refusal half of the same door. Measured in
        the dev-profile module this suite's CI job builds: about 47 s
        for the first (the 125 m version of the tube, which meets the
        resolution, takes 17 s).
        """
        body = oversized_round_loft()
        measured = body.mass_properties(PINNED)
        self.assertIn(measured.stop, ("wall", "floor"), measured)
        self.assertIsNone(measured.verdict, "a resolution decides nothing")
        e = measured.enclosure
        lo, hi = e.volume - e.volume_pad, e.volume + e.volume_pad
        self.assertLess(lo, hi)
        self.assertGreater(lo, 0.0, "the bracket is definitely positive")
        self.assertGreater(e.surface_area, 0.0)
        # The bracket is THIS body's and not a placeholder: a tapered
        # tube of radii 1497.6 m and 748.8 m over an 8 km spine
        # encloses the frustum on those radii, and the rational walls
        # the loft fits sit around it.
        frustum = math.pi * 8000.0 * (1497.6**2 + 1497.6 * 748.8 + 748.8**2) / 3.0
        self.assertLess(lo, frustum)
        self.assertGreater(hi, frustum)
        # A deciding predicate the walk runs out on is THE quadrature
        # refusal (`[tolerance.measure]`): asked whether the volume is at
        # most the bracket's own midpoint, every wall runs to its stop
        # with the enclosure still straddling it, so the door raises
        # `undecided` carrying its bracket and its stop, rather than
        # answering a stop with no verdict that `if m.verdict:` would
        # read as a decision.
        with self.assertRaises(ValidationError) as refused:
            body.mass_properties(Settle.at_most(e.volume))
        r = refused.exception
        self.assertEqual(r.measure_refusal, "undecided")
        self.assertIn(r.stop, ("wall", "floor"))
        # The refusal's bracket is the body's, held to the same frustum
        # oracle as the measurement's above.
        rlo, rhi = r.volume
        self.assertLess(rlo, rhi)
        self.assertGreater(rlo, 0.0, "the refusal's bracket is definitely positive")
        self.assertLess(rlo, frustum)
        self.assertGreater(rhi, frustum)


class TestSettle(unittest.TestCase):
    """Each predicate a measurement takes, on the unit cube's closed
    form: volume 1, one value at round 0, so each answers there."""

    def cube(self):
        doc = Doc()
        node = slab(doc, (0 * m, 1 * m), (0 * m, 1 * m), 1 * m)
        return evaluate(doc).value(node).body()

    def test_a_sign_reads_the_cube_outer(self):
        measured = self.cube().mass_properties(Settle.sign())
        self.assertEqual((measured.stop, measured.verdict), ("settled", "outer"))

    def test_a_relation_settles_on_either_side(self):
        cube = self.cube()
        for settle, holds in [
            (Settle.at_most(2.0), True),
            (Settle.at_most(0.5), False),
            (Settle.at_least(0.5), True),
            (Settle.at_least(2.0), False),
        ]:
            with self.subTest(settle=repr(settle)):
                measured = cube.mass_properties(settle)
                self.assertEqual((measured.stop, measured.verdict), ("settled", holds))

    def test_a_containment_holds_where_the_closed_form_lies(self):
        cube = self.cube()
        inside = cube.mass_properties(Settle.contains(0.5, 1.5))
        self.assertEqual((inside.stop, inside.verdict), ("settled", True))
        outside = cube.mass_properties(Settle.contains(2.0, 3.0))
        self.assertEqual((outside.stop, outside.verdict), ("settled", False))

    def test_a_containment_whose_least_is_above_its_greatest_is_refused(self):
        with self.assertRaises(ValidationError) as refused:
            self.cube().mass_properties(Settle.contains(1.5, 0.5))
        self.assertEqual(refused.exception.reason, "mass_properties_failed")
        self.assertEqual(refused.exception.measure_refusal, "empty_contains")

    def test_a_resolution_is_met_and_decides_nothing(self):
        measured = self.cube().mass_properties(PINNED)
        self.assertEqual((measured.stop, measured.verdict), ("met", None))
        self.assertEqual(measured.enclosure.volume, 1.0)

    def test_a_resolution_finer_than_the_tolerance_is_refused(self):
        with self.assertRaises(ValidationError) as refused:
            self.cube().mass_properties(Settle.resolution(1e-10 * m))
        self.assertEqual(refused.exception.reason, "mass_properties_failed")
        self.assertEqual(refused.exception.measure_refusal, "resolution_finer_than_eps")

if __name__ == "__main__":
    unittest.main()
