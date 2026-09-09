"""The E11.1 advisory Monte-Carlo lane, from Python (LIB-MC).

The lane is UNGATED in the kernel and it is ungated for exactly this
caller: the wheel is built from the default feature set, so the
certified half of the analysis surface is genuinely absent here, and
an advisory estimator that a consumer with no certified scalar could
not reach would be serving nobody. `monte_carlo` is the door,
`sample_offset` the single draw it is built from, and `McRefusal` the
three ways a run has no estimate at all.

EVERY NUMBER IS AN ORACLE THE AUTHORING FIXES, never a constant
transcribed from `crates/editor-core/tests/m10_5_mc.rs`. A slab whose
height is a document parameter declared `1 m ± 1 mm` normal makes the
distance between its caps that same parameter, so the run's mean is
the nominal to within the standard error of a mean of `N` draws
(`3 sigma / sqrt(N)`, computed below from the two dials rather than
written down); a ±3σ analyzed box leaves the normal's own two tails
outside, which is a number a reader can look up; and an assertion
that the height is at least its own nominal is violated by half the
draws because the law is symmetric about it.

WHAT IS ADVISORY IS STRUCTURAL, not a convention this file restates.
No estimate is reachable without the `McReport` that carries the
sample count and the seed, `McReport.render` repeats the label on
every line that carries a number, and nothing here can gate: the lane
answers estimates and the kernel's certified lane is the only gate.
"""

import math
import unittest

import pncad
from pncad import (
    AssertionDir,
    DEFAULT_SAMPLES,
    DEFAULT_SEED,
    Distribution,
    Doc,
    DocEdit,
    DocParam,
    MeasureExpr,
    MeasurePrimitive,
    McConfig,
    Node,
    ParamName,
    analyzed_box,
    deg,
    evaluate,
    m,
    mm,
    monte_carlo,
    sample_offset,
)

SQUARE = [(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)]

#: The declared height of the slab every scene below extrudes, in
#: metres, and the standard deviation of the annotation hung on it.
#: Both are ORACLES: every expectation in this file is derived from
#: these two numbers and the dials on `McConfig`.
NOMINAL = 1.0
SIGMA = 0.001


def face_at_height(ev, node, z):
    """The one face of `node` whose carrier frame sits at height `z`,
    read off the evaluation rather than transcribed as a role name."""
    found = [
        f
        for f in ev.all_faces(node)
        if abs(ev.face_frame(node, f).origin[2].meters - z) < 1e-12
    ]
    assert len(found) == 1, f"one face at z = {z}, found {len(found)}"
    return found[0]


def slab(doc, distribution=None, nominal=NOMINAL):
    """A 1 m x 1 m prism whose HEIGHT is the document parameter `h`,
    annotated with `distribution`.

    The parameter is what the advisory lane varies, and the caps are
    what a measure over the prism reads — so the measured height IS
    the parameter, by construction and not by coincidence.
    """
    doc.apply(
        DocEdit.set_doc_param(
            ParamName("h"), DocParam.length(nominal * m, distribution)
        )
    )
    outline = doc.insert(Node.polygon(SQUARE, plane=doc.sketch_frame()))
    prism = doc.insert(Node.extrude(outline, nominal * m))
    doc.apply(DocEdit.set_param(prism, "distance", doc.parse_expr("h")))
    return prism


def height_measure(doc, prism, nominal=NOMINAL):
    """A measure node reading the distance between the prism's two
    caps — the height, in the document's own vocabulary."""
    ev = evaluate(doc)
    return doc.insert(
        Node.measure(
            MeasureExpr.primitive(MeasurePrimitive.distance(0, 1)),
            [
                (prism, face_at_height(ev, prism, 0.0)),
                (prism, face_at_height(ev, prism, nominal)),
            ],
        )
    )


def scene(distribution=None, bound=None):
    """A document, its analyzed box, and the measure node — plus an
    assertion over the measure when `bound` is given."""
    doc = Doc()
    prism = slab(doc, distribution)
    measure = height_measure(doc, prism)
    assertion = None
    if bound is not None:
        assertion = doc.insert(
            Node.assertion(measure, AssertionDir.AtLeast, doc.parse_expr(bound))
        )
    return doc, analyzed_box(doc), measure, assertion


class TestTheDialsAreRecorded(unittest.TestCase):
    """E11.1 requires the sample count and the seed on every result,
    and here that is structural rather than a convention: there is no
    door that hands out a mean."""

    def test_the_shipped_config_is_the_kernels(self):
        default = McConfig()
        self.assertEqual(default.samples, DEFAULT_SAMPLES)
        self.assertEqual(default.seed, DEFAULT_SEED)
        self.assertTrue(default.parallel)
        self.assertEqual(default, McConfig(DEFAULT_SAMPLES, DEFAULT_SEED, True))
        # A frozen value: comparable, hashable, and restated by
        # building a new one rather than by editing this one.
        self.assertEqual(len({McConfig(), McConfig(samples=8)}), 2)
        self.assertEqual(hash(McConfig()), hash(McConfig()))

    def test_the_report_carries_the_dials_it_ran_at(self):
        doc, box, _measure, _a = scene(Distribution.normal(SIGMA * m))
        config = McConfig(samples=64, seed=0x1234, parallel=False)
        report = monte_carlo(doc, box, config)
        self.assertEqual(report.samples, 64)
        self.assertEqual(report.seed, 0x1234)
        # And the default config is the shipped one, not a second set
        # of dials this binding invented.
        self.assertEqual(monte_carlo(doc, box).samples, DEFAULT_SAMPLES)
        self.assertEqual(monte_carlo(doc, box).seed, DEFAULT_SEED)

    def test_the_rendering_labels_every_line_it_puts_a_number_on(self):
        doc, box, _measure, _a = scene(Distribution.normal(SIGMA * m))
        config = McConfig(samples=32)
        text = monte_carlo(doc, box, config).render()
        numbered = [line for line in text.splitlines() if "mean" in line]
        self.assertTrue(numbered, "the report rendered no estimate at all")
        for line in numbered:
            self.assertIn("ADVISORY", line)
            self.assertIn("32 samples", line)
        self.assertIn("ADVISORY", repr(monte_carlo(doc, box, config)))


class TestTheEstimateIsTheAuthoring(unittest.TestCase):
    """The mean of a measure over a normally distributed parameter is
    that parameter's nominal, to within the standard error of a mean
    of `N` draws."""

    def test_the_mean_is_the_nominal_within_three_standard_errors(self):
        doc, box, measure, _a = scene(Distribution.normal(SIGMA * m))
        config = McConfig(samples=512)
        report = monte_carlo(doc, box, config)
        self.assertEqual(len(report.measures), 1)
        row = report.measures[0]
        self.assertEqual(row.node, measure)
        # Every draw of a normal has a value, so nothing is unmeasured
        # and the count is the whole run.
        self.assertEqual(row.measured, config.samples)
        self.assertEqual(row.unmeasured, 0)
        tolerance = 3 * SIGMA / math.sqrt(config.samples)
        self.assertLess(abs(row.mean - NOMINAL), tolerance)
        # The spread is the annotation's, to a tenth of itself — a
        # sample deviation over 512 draws is not the law's sigma and
        # this does not pretend otherwise.
        self.assertLess(abs(row.sigma - SIGMA), 0.1 * SIGMA)
        self.assertLessEqual(row.min, row.mean)
        self.assertLessEqual(row.mean, row.max)

    def test_a_fixed_document_has_no_spread_at_all(self):
        """An unannotated parameter is FIXED, so every draw is the
        nominal and the estimator says so exactly."""
        doc, box, _measure, _a = scene()
        report = monte_carlo(doc, box, McConfig(samples=16))
        row = report.measures[0]
        self.assertEqual(row.mean, NOMINAL)
        self.assertEqual(row.sigma, 0.0)
        self.assertEqual(row.min, row.max)
        # A fixed axis is not varied at all, so no draw can be outside
        # a box it was never moved within.
        self.assertEqual(report.outside_box, 0.0)

    def test_the_tail_outside_the_box_is_the_normals_own(self):
        """The default policy asks for ±3σ, so a normal leaves its two
        3σ tails outside — about 0.27% of the mass, which is the
        empirical twin of what `tail_mass` computes exactly."""
        doc, box, _measure, _a = scene(Distribution.normal(SIGMA * m))
        report = monte_carlo(doc, box, McConfig(samples=512))
        exact = box.tail_mass(ParamName("h"))
        self.assertLess(abs(report.outside_box - exact), 0.02)
        self.assertGreater(report.outside_box, 0.0)

    def test_a_truncated_law_puts_nothing_outside_its_own_box(self):
        """A truncated normal's support IS the analyzed box, so no
        draw can land outside it — an exact zero, not an estimate that
        happens to be small."""
        doc, box, _measure, _a = scene(
            Distribution.truncated_normal(SIGMA * m, -2 * SIGMA * m, 2 * SIGMA * m)
        )
        report = monte_carlo(doc, box, McConfig(samples=128))
        self.assertEqual(report.outside_box, 0.0)


class TestTheAssertionRowsAreEmpirical(unittest.TestCase):
    def test_a_symmetric_law_violates_its_own_nominal_half_the_time(self):
        doc, box, _measure, assertion = scene(
            Distribution.normal(SIGMA * m), bound="1 m"
        )
        report = monte_carlo(doc, box, McConfig(samples=512))
        self.assertEqual(len(report.assertions), 1)
        row = report.assertions[0]
        self.assertEqual(row.node, assertion)
        # Every sample decided, so the three counts partition the run.
        self.assertEqual(row.holds + row.violated + row.unevaluated, 512)
        self.assertEqual(row.unevaluated, 0)
        # A normal is symmetric about its nominal, so "at least the
        # nominal" fails on half the draws. The window is three
        # standard errors of a fair-coin proportion over 512 trials.
        window = 3 * math.sqrt(0.25 / 512)
        self.assertLess(abs(row.violation_fraction - 0.5), window)

    def test_an_assertion_nothing_decided_has_no_fraction(self):
        """The denominator is the DECIDED samples, so a run that
        decided none answers `None` rather than dividing by zero or
        calling the undecided samples passes."""
        doc = Doc()
        prism = slab(doc, Distribution.normal(SIGMA * m))
        ev = evaluate(doc)
        clearance = doc.insert(
            Node.measure(
                MeasureExpr.primitive(MeasurePrimitive.min_clearance(0, 1)),
                [
                    (prism, face_at_height(ev, prism, 0.0)),
                    (prism, face_at_height(ev, prism, NOMINAL)),
                ],
            )
        )
        assertion = doc.insert(
            Node.assertion(clearance, AssertionDir.AtLeast, doc.parse_expr("1 mm"))
        )
        report = monte_carlo(doc, analyzed_box(doc), McConfig(samples=8))
        row = next(r for r in report.assertions if r.node == assertion)
        self.assertIsNone(row.violation_fraction)
        self.assertEqual(row.unevaluated, 8)
        # And its measure is unmeasured at every draw: a
        # `min_clearance`'s value is an enclosure and this lane
        # evaluates at `f64`, so there is nothing for it to estimate.
        measured = next(r for r in report.measures if r.node == clearance)
        self.assertEqual(measured.unmeasured, 8)
        self.assertEqual(measured.measured, 0)


class TestTheScheduleChangesNothing(unittest.TestCase):
    """D9 idiom 1: each sample is seeded from its own index, so the
    sequential and the parallel schedules produce the same bits."""

    def test_the_two_schedules_agree_bit_for_bit(self):
        doc, box, _measure, _a = scene(Distribution.normal(SIGMA * m), bound="1 m")
        parallel = monte_carlo(doc, box, McConfig(samples=256, parallel=True))
        serial = monte_carlo(doc, box, McConfig(samples=256, parallel=False))
        self.assertEqual(parallel.measures, serial.measures)
        self.assertEqual(parallel.assertions, serial.assertions)
        self.assertEqual(
            parallel.outside_box.hex(), serial.outside_box.hex(), "not the same bits"
        )
        for left, right in zip(parallel.measures, serial.measures, strict=True):
            for field in ("mean", "sigma", "min", "max"):
                self.assertEqual(
                    getattr(left, field).hex(),
                    getattr(right, field).hex(),
                    f"{field} differs between the two schedules",
                )

    def test_a_different_seed_is_a_different_run(self):
        doc, box, _measure, _a = scene(Distribution.normal(SIGMA * m))
        one = monte_carlo(doc, box, McConfig(samples=64, seed=1))
        two = monte_carlo(doc, box, McConfig(samples=64, seed=2))
        self.assertNotEqual(one.measures, two.measures)
        # Same seed, same bits, whatever else moved.
        again = monte_carlo(doc, box, McConfig(samples=64, seed=1))
        self.assertEqual(one.measures, again.measures)


class TestTheThreeRefusals(unittest.TestCase):
    """Every arm of `McRefusal`, reached from Python with its payload,
    and every attribute present on every arm."""

    def assert_shape(self, refusal):
        self.assertIsInstance(refusal, pncad.PncadError)
        for attr in ("variant", "param", "node", "cause"):
            getattr(refusal, attr)

    def test_a_band_refuses_the_whole_run_naming_the_parameter(self):
        """Limits without a shape cannot be drawn from, and the lane
        refuses the WHOLE run: a mean over the parameters it CAN
        sample is an estimate of a different document."""
        doc, box, _measure, _a = scene(Distribution.band(-1 * mm, 1 * mm))
        with self.assertRaises(pncad.McRefusal) as caught:
            monte_carlo(doc, box)
        refusal = caught.exception
        self.assert_shape(refusal)
        self.assertEqual(refusal.variant, "band_has_no_measure")
        self.assertEqual(refusal.param, "h")
        self.assertIsNone(refusal.node)
        self.assertIsNone(refusal.cause)
        # The word is the mass doors' own, because the fault is: the
        # same band refuses the same way one rung up, at any question
        # whose answer would depend on the shape it withholds.
        with self.assertRaises(pncad.MeasureUnavailable) as priced:
            box.box_mass(ParamName("h"), 0 * mm, 1 * mm)
        self.assertEqual(priced.exception.variant, refusal.variant)
        self.assertEqual(priced.exception.param, refusal.param)

    def test_zero_samples_have_no_estimate(self):
        doc, box, _measure, _a = scene(Distribution.normal(SIGMA * m))
        with self.assertRaises(pncad.McRefusal) as caught:
            monte_carlo(doc, box, McConfig(samples=0))
        refusal = caught.exception
        self.assert_shape(refusal)
        self.assertEqual(refusal.variant, "no_samples")
        self.assertIsNone(refusal.param)
        self.assertIsNone(refusal.node)
        self.assertIsNone(refusal.cause)

    def test_a_document_that_does_not_build_has_nothing_to_replay(self):
        """Found ONCE, at the nominal, rather than `samples` times —
        and the refusal names the node and carries its rendered
        error."""
        doc = Doc()
        prism = slab(doc, Distribution.normal(SIGMA * m), nominal=0.0)
        with self.assertRaises(pncad.McRefusal) as caught:
            monte_carlo(doc, analyzed_box(doc), McConfig(samples=4))
        refusal = caught.exception
        self.assert_shape(refusal)
        self.assertEqual(refusal.variant, "nominal_does_not_build")
        self.assertEqual(refusal.node, prism)
        self.assertIsNone(refusal.param)
        self.assertTrue(refusal.cause)
        # The same refusal an ordinary evaluation of that document
        # reports, rendered: the lane invents no diagnosis of its own.
        with self.assertRaises(pncad.EvaluationError) as built:
            evaluate(doc).value(prism)
        self.assertIn(built.exception.kind.split("_")[0], refusal.cause.lower() + "_")


class TestTheSingleDraw(unittest.TestCase):
    """`sample_offset` is the lane's only way to draw a parameter
    value, and it answers an OFFSET in the parameter's own dimension."""

    def test_the_offset_carries_the_distributions_dimension(self):
        name = ParamName("h")
        length = sample_offset(name, Distribution.normal(SIGMA * m), 0.5)
        self.assertIsInstance(length, pncad.Length)
        self.assertAlmostEqual(length.meters, 0.0, places=12)
        angle = sample_offset(name, Distribution.normal(1 * deg), 0.5)
        self.assertIsInstance(angle, pncad.Angle)
        self.assertAlmostEqual(angle.radians, 0.0, places=12)
        scalar = sample_offset(name, Distribution.normal(1.0), 0.5)
        self.assertIsInstance(scalar, float)
        self.assertAlmostEqual(scalar, 0.0, places=12)

    def test_a_uniform_law_is_its_own_linear_interpolation(self):
        """The oracle is the definition: the quantile of a uniform on
        `[lo, hi]` is `lo + (hi - lo) * u`."""
        name = ParamName("h")
        law = Distribution.uniform(-2 * mm, 6 * mm)
        for u, expected_mm in ((0.0, -2.0), (0.25, 0.0), (0.5, 2.0), (0.75, 4.0)):
            with self.subTest(u=u):
                drawn = sample_offset(name, law, u)
                self.assertAlmostEqual(drawn.in_unit(mm), expected_mm, places=9)

    def test_it_draws_from_the_whole_law_and_not_from_the_box(self):
        """The tail the analyzed box excludes is exactly the region
        the certified answer does not cover, so a draw past ±3σ is the
        point of this lane rather than an escape from it."""
        far = sample_offset(ParamName("h"), Distribution.normal(SIGMA * m), 0.9999)
        self.assertGreater(far.meters, 3 * SIGMA)

    def test_a_band_refuses_typed_and_names_the_parameter(self):
        with self.assertRaises(pncad.MeasureUnavailable) as caught:
            sample_offset(ParamName("bore_r"), Distribution.band(-1 * mm, 1 * mm), 0.5)
        self.assertEqual(caught.exception.variant, "band_has_no_measure")
        self.assertEqual(caught.exception.param, "bore_r")


class TestTheReportIsAValue(unittest.TestCase):
    def test_a_document_with_no_sinks_reports_no_rows(self):
        """A run over a document that measures nothing is not a
        refusal: the tail column is still an answer about the
        document, and the estimator has nothing else to say."""
        doc = Doc()
        slab(doc, Distribution.normal(SIGMA * m))
        report = monte_carlo(doc, analyzed_box(doc), McConfig(samples=8))
        self.assertEqual(report.measures, [])
        self.assertEqual(report.assertions, [])
        self.assertEqual(report.samples, 8)

    def test_the_rows_are_frozen_comparable_values(self):
        doc, box, _measure, _a = scene(Distribution.normal(SIGMA * m), bound="1 m")
        config = McConfig(samples=32)
        one = monte_carlo(doc, box, config)
        two = monte_carlo(doc, box, config)
        self.assertEqual(one.measures[0], two.measures[0])
        self.assertEqual(hash(one.measures[0]), hash(two.measures[0]))
        self.assertEqual(len({one.assertions[0], two.assertions[0]}), 1)
        with self.assertRaises(AttributeError):
            one.measures[0].mean = 0.0


if __name__ == "__main__":
    unittest.main()
