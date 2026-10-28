"""The placement value and the die's cutting tool, from Python.

`Frame` is what a placement's pose reads back as: constructed, read
back, compared, and refused where it is not a rigid motion.

The die tool is corpus document `die_tool`
(`crates/editor-core/tests/corpus/die_tool.rs`) re-authored through the
bound doors: an irregular family is its members written out, each
saying where it is (D10 `[repetition]`) — one placement of the master
ball per pip and one union of the six, fed straight into a subtract.
"""

import math
import unittest
from pathlib import Path

from pncad import (
    Bulge,
    Coord,
    Doc,
    Formula,
    Frame,
    FrameError,
    Mate,
    MateValue,
    Node,
    Open,
    Orientation,
    Placed,
    PoseDef,
    RoleRead,
    Start,
    evaluate,
    m,
    rad,
)

from measuring import pinned


# The die's cutting tool, constant for constant with the corpus
# document `die_tool`: a unit cube, a radius-0.09 ball whose centre
# stands R - H outside each face plane so the cavity is a cap of height
# exactly H, one ball per face.
DIE_L = 1.0
PIP_R = 0.09
PIP_H = 0.05
PIP_C = DIE_L + (PIP_R - PIP_H)


def pip_values(doc):
    """The six face-centre placements' values, in the corpus document's
    order and with its rotations: the slides to the pip's centre on the
    cube's space's axes, then one turn about a coordinate axis written
    as the fraction of a turn it is, carrying the master ball's +Z pole
    onto the face normal it cuts."""
    h = DIE_L / 2.0
    lo = DIE_L - PIP_C  # the -normal faces' centre coordinate
    rows = [
        ("z", (0, 1), (h, h, PIP_C)),
        ("x", (1, 2), (h, h, lo)),
        ("y", (1, 4), (PIP_C, h, h)),
        ("y", (-1, 4), (lo, h, h)),
        ("x", (-1, 4), (h, PIP_C, h)),
        ("x", (1, 4), (h, lo, h)),
    ]
    out = []
    for axis, (num, den), t in rows:
        values = [MateValue(Coord.slide(a), Formula.literal(v * m)) for a, v in zip("xyz", t, strict=True)]
        if num != 0:
            values.append(MateValue(Coord.spin(axis), doc.parse_formula(f"{num}/{den} * turn")))
        out.append(values)
    return out


def die_tool_document():
    """`die_tool` re-authored through the bound Python doors, node for
    node in the corpus document's insert order.

    The label is the one `tests/fixture/mod.rs::Recorder` derives from,
    so the document's IDENTITY matches too and the saved-text pin below
    compares whole lines rather than a redacted subset.
    """
    doc = Doc("mod")

    # ---- the sharp cube, [0, L]^3 ----
    square = doc.insert(
        Node.polygon(
            [
                (Formula.length_in(0, m), Formula.length_in(0, m)),
                (Formula.length_in(DIE_L, m), Formula.length_in(0, m)),
                (Formula.length_in(DIE_L, m), Formula.length_in(DIE_L, m)),
                (Formula.length_in(0, m), Formula.length_in(DIE_L, m)),
            ],
        )
    )
    cube = doc.insert(Node.extrude(square, Formula.length_in(DIE_L, m)))

    # ---- the master ball, poled along +Z ----
    # ONE bulge-1 semicircle pole to pole, closed by its on-axis
    # diameter, revolved about the meridian's own +y through its origin
    # (the constant zero, not a written 0 m).
    axis = (
        (Formula.zero("length"), Formula.zero("length")),
        Formula.turn_over(4),
    )
    half_disc = (
        Open.at((0 * m, -PIP_R * m))
        .arc_to(Bulge((0 * m, PIP_R * m), 1.0))
        .line_to(Start)
    )
    ball_p = doc.insert(Node.profile(half_disc))
    ball = doc.insert(Node.revolve(ball_p, *axis, Formula.angle_in(2.0 * math.pi, rad)))
    # The master ball is placed into the cube's space, its pole on the
    # cube's own frame at its origin corner: an axis mate from the
    # revolve's own axis, under the orientation that puts the
    # meridian's +y on the frame's z, slid zero along it and spun zero
    # about it — the corpus document's `Recorder::on_axis`.
    zero = (0 * m, 0 * m, 0 * m)
    own = doc.corner_frame(cube, zero, (0 * m, 0 * m, DIE_L * m), (DIE_L * m, 0 * m, 0 * m))
    master = doc.insert(
        Node.place(
            doc.output(ball, 0),
            doc.space(cube),
            [
                Mate(
                    doc.output(ball, 1),
                    PoseDef.slide(RoleRead(own, "z"), RoleRead(own, "z"), Formula.zero("length")),
                    orient=Orientation("+x", "-y"),
                ),
                MateValue(Coord.slide("z"), Formula.zero("length")),
                MateValue(Coord.spin(), Formula.zero("angle")),
            ],
        )
    )

    # ---- the cutting tool: one placement of the master per pip ----
    pips = [
        doc.insert(Node.place(master, doc.space(cube), Placed.of(doc, [], values)))
        for values in pip_values(doc)
    ]
    tool = doc.insert(Node.union(pips))
    pipped = doc.insert(Node.subtract(cube, tool))
    # The pipped die is the product: placed once, last, as the corpus
    # document places it, and its copy the slot reads.
    doc.set_product(doc.place(pipped, doc.world))
    return doc, master, tool, pipped


class TestTheDieTool(unittest.TestCase):
    """`die_tool` authored from Python — one master ball, six placements
    of it, one union, fed straight into a subtract."""

    # The corpus document's own saved bytes, pinned by
    # `crates/editor-core/tests/lib_dietool_crossing.rs`. It cannot
    # rot: that test re-authors the registered document and writes this
    # file, so a recipe change on either side is a red run here.
    FIXTURE = (
        Path(__file__).resolve().parents[3]
        / "crates" / "editor-core" / "tests" / "corpus" / "die_tool.pncad"
    )

    def test_the_tool_cuts_six_cavities(self):
        doc, _ball, tool, pipped = die_tool_document()
        ev = evaluate(doc)

        # Fourteen nodes: profile, extrude, profile, revolve, the
        # master ball's placement, six pip placements, the union, the
        # subtract, and the subtract's world placement.
        kinds = [doc.node_kind(n) for n in doc.order()]
        self.assertEqual(
            (kinds.count("union"), kinds.count("place")),
            (1, 8),
        )
        self.assertEqual(doc.node_kind(tool), "union")
        self.assertEqual(doc.node_kind(pipped), "subtract")

        body = ev.value(pipped).body()
        body.validate()
        # Six cavities, each contributing its faces to the ONE solid:
        # 6 box faces + 6 x 2 cap half-bands.
        self.assertEqual(len(ev.all_faces(pipped)), 18)

        # The oracle is `die_pips`': L^3 - 6 * cap(R, H), pi-valued and
        # so not dyadic — asserted at rounding scale, which is why the
        # corpus document carries no mass pin either.
        cap = math.pi * PIP_H ** 2 * (3.0 * PIP_R - PIP_H) / 3.0
        want = DIE_L ** 3 - 6.0 * cap
        self.assertAlmostEqual(
            pinned(body).volume, want, delta=1e-12 * want
        )

    def test_the_re_authoring_is_the_corpus_document_byte_for_byte(self):
        """What Python authors is not a lookalike of `die_tool`, it IS
        `die_tool`.

        Every line of the saved text matches the registered document's
        — identity, node bodies, programs, expressions — except the
        snapshot's ONE `epsilon` line, which CI's tolerance rows sweep
        by design (`crates/pncad/tests/all.rs`'s plate_param pin states
        that disposition; this row inherits it)."""
        doc, _ball, _tool, _pipped = die_tool_document()

        def sans_epsilon(text):
            kept, excluded = [], []
            for line in text.splitlines():
                (excluded if line.lstrip().startswith('"epsilon":') else kept).append(line)
            # Exactly one such line per side: a missing or duplicated
            # epsilon is fixture damage, not sweep variance.
            self.assertEqual(len(excluded), 1, excluded)
            return "\n".join(kept)

        committed = self.FIXTURE.read_text(encoding="utf-8")
        self.assertEqual(
            sans_epsilon(doc.save()),
            sans_epsilon(committed),
            "the Python re-authoring and the corpus document have diverged — "
            "regenerate the fixture with `PNCAD_BLESS=1 cargo test -p editor-core "
            "--test all lib_dietool_crossing` and read the diff",
        )


class TestTheFrameValue(unittest.TestCase):
    """The placement value itself: constructed, read back, compared."""

    def test_the_accessors_read_the_frame_back(self):
        frame = Frame.translation((1 * m, 2 * m, 3 * m))
        self.assertEqual(frame.columns, ((1.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)))
        self.assertEqual(frame.origin, (1 * m, 2 * m, 3 * m))
        self.assertEqual(frame.determinant, 1.0)

    def test_equality_is_bit_exact(self):
        """A frame carries no epsilon, so `-0.0` keeps its own
        identity."""
        zero = Frame.translation((0 * m, 0 * m, 0 * m))
        minus = Frame.translation((-0.0 * m, 0 * m, 0 * m))
        self.assertEqual(zero, Frame.translation((0 * m, 0 * m, 0 * m)))
        self.assertNotEqual(zero, minus)

    def test_a_frame_compares_and_does_not_hash(self):
        """`editor_core::Frame` derives `PartialEq` and no `Hash`, and
        this class mirrors its derives: a pose is a value the solver
        consumes, not a key, so `__hash__` is `None`."""
        self.assertIsNone(Frame.__hash__)
        with self.assertRaises(TypeError):
            {Frame.translation((0 * m, 0 * m, 0 * m))}

    def test_the_frame_trio_builds_and_refuses(self):
        """`point_at` and `path_start_frame` — the frame constructors,
        with their typed refusals."""
        aimed = Frame.point_at(
            (0 * m, 0 * m, 0 * m), (0 * m, 0 * m, 1 * m), (0.0, 1.0, 0.0)
        )
        self.assertAlmostEqual(aimed.determinant, 1.0, places=12)
        self.assertEqual(aimed.origin, (0 * m, 0 * m, 0 * m))

        start = Frame.path_start_frame((0 * m, 0 * m, 0 * m), (1.0, 0.0, 0.0))
        self.assertAlmostEqual(start.determinant, 1.0, places=12)

        # A roll reference ALONG the aim fixes no roll, and refuses
        # rather than picking a fallback.
        with self.assertRaises(FrameError) as caught:
            Frame.point_at(
                (0 * m, 0 * m, 0 * m), (0 * m, 0 * m, 1 * m), (0.0, 0.0, 1.0)
            )
        self.assertEqual(caught.exception.variant, "degenerate_roll_reference")

        with self.assertRaises(FrameError) as caught:
            Frame.point_at(
                (0 * m, 0 * m, 0 * m), (0 * m, 0 * m, 0 * m), (0.0, 1.0, 0.0)
            )
        self.assertEqual(caught.exception.variant, "degenerate_aim")

        with self.assertRaises(FrameError) as caught:
            Frame.path_start_frame((0 * m, 0 * m, 0 * m), (0.0, 0.0, 0.0))
        self.assertEqual(caught.exception.variant, "degenerate_tangent")

    def test_a_direction_with_no_finite_length_is_refused_at_this_door(self):
        """The public door with no pre-validation in front of it.

        A normal past the ~1e154 overflow band makes the norm infinite,
        which is maximally DEFINITE to the sign gate, and the division
        that follows would collapse the direction to zero. It is not a
        `degenerate_*` refusal, because the direction is not zero and
        no tolerance lever reaches it."""
        with self.assertRaises(FrameError) as caught:
            Frame.path_start_frame((0 * m, 0 * m, 0 * m), (1e200, 0.0, 0.0))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "non_finite_tangent")
        self.assertIn("no finite length", str(refusal))
        self.assertIn("scale the geometry into the session's range", str(refusal))
        # Nothing was classified, so there is no classifier payload.
        for absent in (
            "inner_variant", "margin", "margin_low", "margin_high",
            "zero", "escalate", "predicate", "field", "value",
        ):
            self.assertIsNone(getattr(refusal, absent), absent)

    def test_an_exactly_zero_direction_carries_no_classifier_payload(self):
        """A definite zero is not an escalation: the margin never
        landed in the band, so there is nothing for the classifier to
        report. Every attribute is present and `None`."""
        with self.assertRaises(FrameError) as caught:
            Frame.path_start_frame((0 * m, 0 * m, 0 * m), (0.0, 0.0, 0.0))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "degenerate_tangent")
        for absent in (
            "inner_variant", "margin", "margin_low", "margin_high",
            "zero", "escalate", "predicate", "field", "value",
        ):
            self.assertIsNone(getattr(refusal, absent), absent)

    def test_an_in_band_direction_carries_the_margin_and_the_band(self):
        """A direction SHORT of the coincidence threshold but not
        exactly zero lands in the ambiguity band, and the refusal
        reports what the classifier saw rather than making a caller
        parse it out of the message.

        The recourse is the message's own three levers — the payload
        is diagnostic, and the escalation contract is that no sound
        branch exists here."""
        # A length inside the ambiguity band (eps, K*eps), derived
        # from the process epsilon rather than hard-coded: the band's
        # lower edge moves with the run's tolerance.
        in_band = 1.5 * Doc().epsilon
        with self.assertRaises(FrameError) as caught:
            Frame.path_start_frame((0 * m, 0 * m, 0 * m), (in_band, 0.0, 0.0))
        refusal = caught.exception
        self.assertEqual(refusal.variant, "degenerate_tangent")
        self.assertIsNotNone(refusal.margin)
        self.assertIsNotNone(refusal.zero)
        self.assertIsNotNone(refusal.escalate)
        self.assertLess(refusal.zero, refusal.escalate)
        self.assertLess(refusal.margin, refusal.escalate)
        # An f64 classification saw a VALUE, not an enclosure.
        self.assertIsNone(refusal.margin_low)
        self.assertIsNone(refusal.margin_high)
        # The band arm's own payload is absent on a degenerate arm.
        self.assertIsNone(refusal.inner_variant)
        self.assertIsNone(refusal.field)
        self.assertIsNone(refusal.value)

    def test_a_short_aim_reports_the_same_pair(self):
        """The third arm through a different door: `point_at`'s aim,
        with the same attribute set on the same class."""
        in_band = 1.5 * Doc().epsilon
        with self.assertRaises(FrameError) as caught:
            Frame.point_at(
                (0 * m, 0 * m, 0 * m), (in_band * m, 0 * m, 0 * m), (0.0, 1.0, 0.0)
            )
        refusal = caught.exception
        self.assertIn(
            refusal.variant, ("degenerate_aim", "degenerate_roll_reference")
        )
        self.assertIsNotNone(refusal.zero)
        self.assertIsNotNone(refusal.escalate)


if __name__ == "__main__":
    unittest.main()
