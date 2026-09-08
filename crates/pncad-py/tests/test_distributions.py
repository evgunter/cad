"""Parameter uncertainty and the analysis lane, from Python
(LIB-B-DISTRIBUTIONS, ERROR-DESIGN E1/E2).

The Python mirror of `crates/editor-core/tests/m10_1_analysis.rs` and
of the façade's own end-to-end row
(`crates/pncad/tests/all.rs::distributions_author_save_reload_and_analyze_through_the_facade`).
Every number below is either an ORACLE a reader can check by hand —
±3σ of a stated sigma, the interquartile range of a standard normal,
a symmetric truncation's half — or is read off the door beside the one
under test, never transcribed from the Rust rows.

WHAT THIS FAMILY CLOSES. Before it, Python could author a parameter
and not annotate one: `Distribution` had no spelling, so a document
built here declared nothing about spread, and one read back from a
file carried an annotation no Python caller could see, restate or
price. Four doors close it — the `Distribution` value class, the
`distribution=` argument on the three continuous `DocParam`
constructors, `Doc.doc_param` reading a declaration back, and
`analyzed_box` with the two mass columns on the box it answers.

THE SHARP EDGE, and what "closing" it means. `DocEdit.set_doc_param`
is create-or-REPLACE: a `DocParam` rebuilt from a dimension and a
number replaces the declaration, and any distribution the parameter
carried is deleted with no refusal and no diagnostic.
`TestTheSharpEdge` pins that as it still stands — the deletion is the
kernel's semantics and this unit invents no edit to change it — and
then pins the two ways a Python caller now has out of it, which is
what did not exist before: restate the annotation on the rebuilt
`DocParam`, or move the number through `set_doc_param_value`, which
carries the whole declaration forward. `Doc.doc_param` is what makes
the difference VISIBLE from Python at all.

DELIBERATELY NOT ASSERTED HERE. The kernel's free `tail_mass` /
`box_mass` / `std_deviation` do not cross — only the box-keyed
spellings do, because the free doors take a name, a distribution and
an interval as three loose arguments and a mispairing answers a
plausible number rather than refusing (`AnalyzedBox::axis_tail_mass`'s
own reasoning). The E6 driver, the E4/E5 stackup and the E10 reports
do not cross either: they are behind `#[cfg(feature = "interval")]` at
`crates/pncad/src/analysis.rs:55` and the wheel is built from the
default feature set, so they are absent from the artifact a user
installs. The three doors the E1 charter names are all on the ungated
list at `:49`.
"""

import math
import unittest

import pncad
from pncad import (
    AnalysisPolicy,
    AnalysisPolicyError,
    DEFAULT_QUANTILE_MASS,
    Distribution,
    DimensionError,
    Doc,
    DocEdit,
    DocParam,
    DocParamValue,
    DistributionFault,
    MeasureUnavailable,
    Node,
    ParamName,
    analyzed_box,
    deg,
    load,
    m,
    mm,
    rad,
)


def declared(**params):
    """A document declaring `params`, by name."""
    doc = Doc("distributions")
    for name, value in params.items():
        doc.apply(DocEdit.set_doc_param(ParamName(name), value))
    return doc


def axis(doc, name, policy=None):
    return analyzed_box(doc, policy).get(ParamName(name))


#: One of each form, on one dimension, so the rows below can say "each
#: of the four" without rebuilding them per test.
FORMS = {
    "banded": Distribution.band(-0.1 * mm, 0.2 * mm),
    "spread": Distribution.uniform(-0.1 * mm, 0.2 * mm),
    "measured": Distribution.normal(5 * mm),
    "windowed": Distribution.truncated_normal(5 * mm, -0.1 * mm, 0.2 * mm),
}


class TestTheFourForms(unittest.TestCase):
    """Each form authored onto a parameter and read back, through both
    read doors: the declaration `Doc.doc_param` answers with, and the
    axis `analyzed_box` derives from it."""

    def test_every_form_authors_and_reads_back_identically(self):
        for name, dist in FORMS.items():
            with self.subTest(form=dist.kind):
                doc = declared(**{name: DocParam.length(4 * mm, dist)})
                back = doc.doc_param(ParamName(name))
                self.assertEqual(back.distribution, dist)
                self.assertEqual(back.distribution.kind, dist.kind)
                self.assertEqual(back.dimension, "length")
                self.assertEqual(back.distribution.dimension, "length")

    def test_the_four_kinds_are_four_words(self):
        self.assertEqual(
            sorted(d.kind for d in FORMS.values()),
            ["band", "normal", "truncated_normal", "uniform"],
        )

    def test_the_offsets_read_back_as_the_quantities_they_were_written_as(self):
        band = FORMS["banded"]
        self.assertEqual(band.lo, -0.1 * mm)
        self.assertEqual(band.hi, 0.2 * mm)
        self.assertIsNone(band.sigma)
        normal = FORMS["measured"]
        self.assertIsNone(normal.lo, "the one unbounded form has no support")
        self.assertIsNone(normal.hi)
        self.assertEqual(normal.sigma, 5 * mm)
        window = FORMS["windowed"]
        self.assertEqual(window.sigma, 5 * mm)
        self.assertEqual(window.lo, -0.1 * mm)
        self.assertEqual(window.hi, 0.2 * mm)

    def test_an_unannotated_parameter_declares_none(self):
        doc = declared(plain=DocParam.length(4 * mm))
        self.assertIsNone(doc.doc_param(ParamName("plain")).distribution)
        self.assertIsNone(doc.doc_param(ParamName("nope")))

    def test_a_count_parameter_carries_no_annotation_and_has_no_door(self):
        """A structural count is fixed under any error analysis, so
        `DocParam.count` takes no distribution and the parameter reads
        back with none."""
        doc = declared(holes=DocParam.count(4))
        self.assertIsNone(doc.doc_param(ParamName("holes")).distribution)
        self.assertEqual(doc.doc_param(ParamName("holes")).dimension, "count")
        with self.assertRaises(TypeError):
            DocParam.count(4, Distribution.normal(1.0))

    def test_the_annotation_is_part_of_the_parameter(self):
        """Equality and hashing see it, so two parameters differing
        only in their annotation are two parameters."""
        plain = DocParam.length(4 * mm)
        annotated = DocParam.length(4 * mm, FORMS["measured"])
        self.assertNotEqual(plain, annotated)
        self.assertEqual(annotated, DocParam.length(4 * mm, FORMS["measured"]))
        self.assertEqual(
            hash(annotated), hash(DocParam.length(4 * mm, FORMS["measured"]))
        )
        self.assertEqual(len({plain, annotated}), 2)

    def test_the_two_spellings_of_zero_are_one_offset(self):
        """`Distribution`'s equality is IEEE on its offsets, so `-0.0`
        and `0.0` are the same offset — and the hash folds through the
        kernel's own `fold_signed_zeros`, so it cannot split what
        equality calls the same."""
        a = Distribution.band(-0.0 * mm, 0.0 * mm)
        b = Distribution.band(0.0 * mm, -0.0 * mm)
        self.assertEqual(a, b)
        self.assertEqual(hash(a), hash(b))
        self.assertEqual(len({a, b}), 1)

    def test_the_dimension_is_part_of_the_value(self):
        """A Length band and a Scalar band of the same numbers are
        different annotations, exactly as a Length 1 and a Scalar 1 are
        different `DocParam`s."""
        self.assertNotEqual(
            Distribution.band(-1 * m, 1 * m), Distribution.band(-1.0, 1.0)
        )

    def test_a_repr_shows_the_form_and_the_dimension(self):
        text = repr(FORMS["measured"])
        self.assertIn("normal", text)
        self.assertIn("length", text)


class TestTheConstructorRefuses(unittest.TestCase):
    """`DistributionFault`, reached from Python for every refusing
    input the kernel's `Distribution::check` names.

    The check is the KERNEL's — the same function the edit door and the
    persistence validator run — so what these rows pin is that a Python
    caller reaches every arm of it, and that the payload is projected:
    `variant` plus every field, present on every arm and `None` where
    that arm does not carry one."""

    def assertFault(self, call, variant, **payload):
        with self.assertRaises(DistributionFault) as ctx:
            call()
        err = ctx.exception
        self.assertEqual(err.variant, variant)
        # Every attribute present on every arm: `getattr` never raises,
        # so a caller reads the payload without branching on `variant`.
        for field in ("field", "sigma", "lo", "hi"):
            self.assertTrue(hasattr(err, field), field)
        for field, want in payload.items():
            self.assertEqual(getattr(err, field), want, field)
        for field in {"field", "sigma", "lo", "hi"} - set(payload):
            self.assertIsNone(getattr(err, field), field)
        return err

    def test_a_sigma_that_is_not_positive_refuses(self):
        """A degenerate normal is not a distribution, and a FIXED
        parameter is spelled by having no distribution at all."""
        for sigma in (0.0, -1.0):
            with self.subTest(sigma=sigma):
                self.assertFault(
                    lambda s=sigma: Distribution.normal(s * m),
                    "sigma_not_positive",
                    sigma=sigma,
                )
        self.assertFault(
            lambda: Distribution.truncated_normal(-2.0 * m, -1 * m, 1 * m),
            "sigma_not_positive",
            sigma=-2.0,
        )

    def test_bounds_that_do_not_contain_the_nominal_refuse(self):
        """`lo <= 0 <= hi`. Asymmetric bounds are legal; a nominal
        outside its own support is a document error."""
        for form in (Distribution.band, Distribution.uniform):
            with self.subTest(form=form.__name__):
                self.assertFault(
                    lambda f=form: f(1 * m, 2 * m),
                    "nominal_outside_support",
                    lo=1.0,
                    hi=2.0,
                )
                self.assertFault(
                    lambda f=form: f(-2 * m, -1 * m),
                    "nominal_outside_support",
                    lo=-2.0,
                    hi=-1.0,
                )
        self.assertFault(
            lambda: Distribution.truncated_normal(1 * m, 1 * m, 2 * m),
            "nominal_outside_support",
            lo=1.0,
            hi=2.0,
        )
        # Asymmetric is fine, and so is a degenerate window at zero.
        Distribution.band(-1 * m, 3 * m)
        Distribution.uniform(0 * m, 0 * m)

    def test_a_non_finite_offset_refuses_and_names_the_field(self):
        """The one arm that carries a `field`: which of the three
        offsets was NaN or infinite."""
        self.assertFault(
            lambda: Distribution.normal(float("inf") * m),
            "non_finite",
            field="sigma",
        )
        self.assertFault(
            lambda: Distribution.band(float("-inf") * m, 1 * m), "non_finite", field="lo"
        )
        self.assertFault(
            lambda: Distribution.band(-1 * m, float("nan") * m), "non_finite", field="hi"
        )

    def test_the_refusal_is_a_pncad_error_with_prose(self):
        with self.assertRaises(pncad.PncadError) as ctx:
            Distribution.normal(0 * m)
        self.assertIn("sigma", str(ctx.exception))
        self.assertIn("positive", str(ctx.exception))

    def test_the_offsets_must_share_a_dimension(self):
        """The annotation carries no dimension of its own — it borrows
        the parameter's — so its own offsets have to agree about which
        one they are in."""
        with self.assertRaises(DimensionError) as ctx:
            Distribution.band(-1 * mm, 1 * deg)
        self.assertEqual(ctx.exception.op, "Distribution.band")
        self.assertEqual(ctx.exception.left, "length")
        self.assertEqual(ctx.exception.right, "angle")
        with self.assertRaises(DimensionError):
            Distribution.truncated_normal(1 * mm, -1 * mm, 1.0)

    def test_an_offset_that_is_not_a_quantity_at_all_is_a_type_error(self):
        with self.assertRaises(TypeError):
            Distribution.normal("wide")

    def test_the_declaration_and_the_annotation_must_agree(self):
        """The seam the constructor cannot check and the `DocParam`
        door can: an Angle spread on a Length parameter."""
        with self.assertRaises(DimensionError) as ctx:
            DocParam.length(4 * mm, Distribution.normal(1 * deg))
        self.assertEqual(ctx.exception.op, "DocParam.length")
        self.assertEqual(ctx.exception.left, "length")
        self.assertEqual(ctx.exception.right, "angle")
        with self.assertRaises(DimensionError):
            DocParam.scalar(0.5, Distribution.normal(1 * mm))
        with self.assertRaises(DimensionError):
            DocParam.angle(1 * rad, Distribution.band(-1 * mm, 1 * mm))
        # And the agreeing spellings all pass.
        DocParam.length(4 * mm, Distribution.normal(1 * mm))
        DocParam.angle(1 * rad, Distribution.normal(1 * deg))
        DocParam.scalar(0.5, Distribution.normal(0.01))


class TestTheAnalyzedBox(unittest.TestCase):
    """`analyzed_box` and the policy that shapes it."""

    def test_the_default_policy_is_the_three_sigma_convention(self):
        self.assertEqual(AnalysisPolicy().quantile_mass, DEFAULT_QUANTILE_MASS)
        doc = declared(n=DocParam.length(1 * m, Distribution.normal(0.01 * m)))
        n = axis(doc, "n")
        self.assertAlmostEqual(n.offsets[1].meters, 0.03, delta=5e-6)
        self.assertEqual(n.offsets[0], -n.offsets[1], "symmetric by construction")
        self.assertEqual(n.nominal, 1 * m)
        self.assertEqual(n.absolute()[0], n.nominal + n.offsets[0])
        self.assertEqual(n.absolute()[1], n.nominal + n.offsets[1])
        self.assertEqual(n.width.meters, n.offsets[1].meters - n.offsets[0].meters)

    def test_the_quantile_mass_is_a_checked_request_knob(self):
        """The knob is the ANALYSIS's: a wider requested mass widens
        the box monotonically, and it is checked into `(0, 1)`."""
        doc = declared(n=DocParam.length(0 * m, Distribution.normal(1 * m)))

        def width(mass):
            return axis(doc, "n", AnalysisPolicy(mass)).width.meters

        narrow, wide = width(0.5), width(0.99)
        self.assertLess(narrow, wide)
        # The 50% box of a standard normal is the interquartile range.
        self.assertAlmostEqual(narrow, 2.0 * 0.67448975, delta=1e-6)

        for bad in (0.0, 1.0, -0.5, 2.0, float("inf")):
            with self.subTest(mass=bad):
                with self.assertRaises(AnalysisPolicyError) as ctx:
                    AnalysisPolicy(bad)
                self.assertEqual(ctx.exception.variant, "quantile_mass_out_of_range")
                self.assertEqual(ctx.exception.mass, bad)
        with self.assertRaises(AnalysisPolicyError) as ctx:
            AnalysisPolicy(float("nan"))
        self.assertTrue(math.isnan(ctx.exception.mass))

    def test_opt_in_means_an_unannotated_param_is_fixed(self):
        """A parameter with NO distribution is FIXED, and a `Count`
        parameter is not an axis at all: the analysis varies exactly
        what the author declared variable."""
        doc = declared(
            plain=DocParam.length(2 * m),
            holes=DocParam.count(4),
            varies=DocParam.length(1 * m, Distribution.band(-0.1 * m, 0.1 * m)),
        )
        boxed = analyzed_box(doc)
        self.assertEqual(len(boxed), 2, "Count is not a box axis")
        self.assertIsNone(boxed.get(ParamName("holes")))
        self.assertEqual(
            sorted(n.name for n in boxed.names), ["plain", "varies"]
        )
        fixed = boxed.get(ParamName("plain"))
        self.assertTrue(fixed.is_fixed)
        self.assertEqual(fixed.absolute(), (2 * m, 2 * m), "width zero AT the nominal")
        self.assertIsNone(fixed.distribution)
        self.assertEqual(
            [n.name for n in boxed.varying], ["varies"], "only the declared axis varies"
        )

    def test_the_bounded_forms_are_their_own_box(self):
        """The bounded forms ARE their own analyzed interval, so
        nothing escapes: their tail mass is exactly zero,
        `truncated_normal` included."""
        for dist in (
            Distribution.band(-0.1 * m, 0.2 * m),
            Distribution.uniform(-0.1 * m, 0.2 * m),
            Distribution.truncated_normal(0.05 * m, -0.1 * m, 0.2 * m),
        ):
            with self.subTest(form=dist.kind):
                doc = declared(b=DocParam.length(1 * m, dist))
                boxed = analyzed_box(doc)
                self.assertEqual(
                    boxed.get(ParamName("b")).offsets, (-0.1 * m, 0.2 * m)
                )
                self.assertEqual(boxed.tail_mass(ParamName("b")), 0.0)

    def test_a_name_the_document_does_not_declare_is_not_an_axis(self):
        doc = declared(n=DocParam.length(1 * m, Distribution.normal(0.01 * m)))
        boxed = analyzed_box(doc)
        self.assertIsNone(boxed.get(ParamName("nope")))
        self.assertIsNone(boxed.tail_mass(ParamName("nope")))
        self.assertIsNone(boxed.box_mass(ParamName("nope"), -1 * m, 1 * m))

    def test_a_distribution_reaches_no_evaluation(self):
        """A distribution is inert document metadata: the number an
        expression sees is the nominal alone, annotated or not."""
        plain = declared(d=DocParam.length(0.75 * m))
        annotated = declared(
            d=DocParam.length(0.75 * m, Distribution.normal(0.01 * m))
        )
        self.assertEqual(
            plain.eval(plain.parse_expr("d")).meters,
            annotated.eval(annotated.parse_expr("d")).meters,
        )


class TestTheMassColumns(unittest.TestCase):
    """The tail column and the leaf column, and the band that prices
    neither."""

    def test_the_box_holds_the_mass_the_policy_asked_for(self):
        doc = declared(n=DocParam.length(1 * m, Distribution.normal(0.01 * m)))
        for mass in (0.5, 0.9, DEFAULT_QUANTILE_MASS, 0.999999):
            with self.subTest(mass=mass):
                boxed = analyzed_box(doc, AnalysisPolicy(mass))
                lo, hi = boxed.get(ParamName("n")).offsets
                inside = boxed.box_mass(ParamName("n"), lo, hi)
                self.assertAlmostEqual(inside, mass, delta=1e-12)
                self.assertGreater(
                    boxed.tail_mass(ParamName("n")),
                    0.0,
                    "a normal always leaves something outside",
                )

    def test_every_priceable_form_and_box_stays_in_zero_to_one(self):
        forms = (
            Distribution.uniform(-0.2 * m, 0.3 * m),
            Distribution.normal(0.1 * m),
            Distribution.truncated_normal(0.1 * m, -0.2 * m, 0.3 * m),
        )
        boxes = (
            (-0.05 * m, 0.05 * m),
            (-0.2 * m, 0.3 * m),
            (-1.0 * m, 1.0 * m),
            (0.0 * m, 0.0 * m),
            (-0.3 * m, 0.0 * m),
        )
        for dist in forms:
            doc = declared(x=DocParam.length(1 * m, dist))
            boxed = analyzed_box(doc)
            for lo, hi in boxes:
                with self.subTest(form=dist.kind, box=(lo.meters, hi.meters)):
                    inside = boxed.box_mass(ParamName("x"), lo, hi)
                    self.assertGreaterEqual(inside, 0.0)
                    self.assertLessEqual(inside, 1.0)
            self.assertGreaterEqual(boxed.tail_mass(ParamName("x")), 0.0)
            self.assertLessEqual(boxed.tail_mass(ParamName("x")), 1.0)

    def test_a_truncated_normal_is_renormalized(self):
        """Renormalized, not merely clipped: its own support holds ALL
        of its mass, and a half of it holds strictly more than the
        underlying normal would."""
        sigma = 0.1 * m
        window = (-0.05 * m, 0.05 * m)
        doc = declared(
            t=DocParam.length(1 * m, Distribution.truncated_normal(sigma, *window)),
            n=DocParam.length(1 * m, Distribution.normal(sigma)),
        )
        boxed = analyzed_box(doc)
        self.assertEqual(boxed.box_mass(ParamName("t"), *window), 1.0)
        self.assertEqual(boxed.tail_mass(ParamName("t")), 0.0)
        half = boxed.box_mass(ParamName("t"), 0.0 * m, 0.05 * m)
        self.assertAlmostEqual(half, 0.5, delta=1e-12, msg="symmetric truncation halves")
        untruncated = boxed.box_mass(ParamName("n"), 0.0 * m, 0.05 * m)
        self.assertGreater(half, untruncated, "renormalization concentrates mass")

    def test_a_band_refuses_to_be_priced_and_names_the_parameter(self):
        """A band prices nothing shape-dependent. It refuses typed,
        NAMING the parameter, wherever the answer would depend on a
        shape it does not state — and answers only where every measure
        on the band agrees."""
        doc = declared(bore=DocParam.length(1 * m, Distribution.band(-0.1 * m, 0.1 * m)))
        boxed = analyzed_box(doc)
        with self.assertRaises(MeasureUnavailable) as ctx:
            boxed.box_mass(ParamName("bore"), -0.05 * m, 0.05 * m)
        self.assertEqual(ctx.exception.variant, "band_has_no_measure")
        self.assertEqual(ctx.exception.param, "bore")
        self.assertIn("bore", str(ctx.exception))
        self.assertIn("no shape", str(ctx.exception))
        # A partial overlap is refused from either side.
        for lo, hi in ((-1.0 * m, 0.05 * m), (-0.05 * m, 1.0 * m)):
            with self.assertRaises(MeasureUnavailable):
                boxed.box_mass(ParamName("bore"), lo, hi)
        # The two answers every measure on the band agrees on.
        self.assertEqual(boxed.box_mass(ParamName("bore"), -0.5 * m, 0.5 * m), 1.0)
        self.assertEqual(boxed.box_mass(ParamName("bore"), 0.5 * m, 0.6 * m), 0.0)
        # And a box containing the whole band leaves nothing outside,
        # whatever the shape.
        self.assertEqual(boxed.tail_mass(ParamName("bore")), 0.0)

    def test_a_uniform_answers_exactly_where_the_band_refuses(self):
        """The point of keeping the two forms apart: the same limits
        under `uniform` answer where `band` refuses."""
        limits = (-0.1 * m, 0.1 * m)
        doc = declared(
            u=DocParam.length(1 * m, Distribution.uniform(*limits)),
            b=DocParam.length(1 * m, Distribution.band(*limits)),
        )
        boxed = analyzed_box(doc)
        sub = (-0.05 * m, 0.05 * m)
        self.assertAlmostEqual(boxed.box_mass(ParamName("u"), *sub), 0.5, delta=1e-12)
        with self.assertRaises(MeasureUnavailable):
            boxed.box_mass(ParamName("b"), *sub)

    def test_a_fixed_axis_is_a_point_mass_at_its_nominal(self):
        doc = declared(fixed=DocParam.length(1 * m))
        boxed = analyzed_box(doc)
        self.assertEqual(boxed.tail_mass(ParamName("fixed")), 0.0)
        self.assertEqual(boxed.box_mass(ParamName("fixed"), -1 * m, 1 * m), 1.0)
        self.assertEqual(boxed.box_mass(ParamName("fixed"), 0.5 * m, 1 * m), 0.0)

    def test_the_leaf_interval_is_in_the_axis_own_dimension(self):
        """The mispairing the box-keyed door forecloses one rung out
        from the kernel's: an interval in another dimension is a
        refusal, not a plausible number."""
        doc = declared(n=DocParam.length(1 * m, Distribution.normal(0.01 * m)))
        boxed = analyzed_box(doc)
        with self.assertRaises(DimensionError) as ctx:
            boxed.box_mass(ParamName("n"), -1 * deg, 1 * deg)
        self.assertEqual(ctx.exception.op, "AnalyzedBox.box_mass")
        self.assertEqual(ctx.exception.left, "length")
        self.assertEqual(ctx.exception.right, "angle")
        with self.assertRaises(DimensionError):
            boxed.box_mass(ParamName("n"), -1 * m, 1 * deg)


class TestTheSharpEdge(unittest.TestCase):
    """`set_doc_param` is create-or-replace, and what that costs.

    The deletion is the kernel's semantics and this unit invents no
    edit to change it. What changed is that Python can now SEE the
    declaration, and can restate it."""

    SPREAD = Distribution.normal(5e-6 * m)

    def annotated(self):
        return declared(bore_r=DocParam.length(4 * mm, self.SPREAD))

    def test_before_a_rebuilt_docparam_deletes_the_annotation(self):
        """The edge, as it stands: the natural spelling of a value
        change — rebuild the parameter from a dimension and a number —
        applies cleanly and silently drops the spread."""
        doc = self.annotated()
        self.assertEqual(doc.doc_param(ParamName("bore_r")).distribution, self.SPREAD)
        doc.apply(DocEdit.set_doc_param(ParamName("bore_r"), DocParam.length(4.5 * mm)))
        self.assertIsNone(
            doc.doc_param(ParamName("bore_r")).distribution,
            "create-or-replace replaced the whole declaration",
        )
        self.assertTrue(
            analyzed_box(doc).get(ParamName("bore_r")).is_fixed,
            "and the analysis now varies nothing",
        )

    def test_after_the_value_door_carries_the_declaration_forward(self):
        """The door a caller moving a number should use, and the one
        that already existed: it never names a declaration, so it
        cannot replace one."""
        doc = self.annotated()
        doc.apply(
            DocEdit.set_doc_param_value(ParamName("bore_r"), DocParamValue.length(4.5 * mm))
        )
        back = doc.doc_param(ParamName("bore_r"))
        self.assertEqual(back.distribution, self.SPREAD)
        self.assertEqual(analyzed_box(doc).get(ParamName("bore_r")).nominal, 4.5 * mm)

    def test_after_a_redeclaration_can_restate_the_annotation(self):
        """The half this family adds: a `DocParam` that CARRIES the
        distribution, so a caller who really is redeclaring — a new
        dimension, a new spread — can say the whole thing."""
        doc = self.annotated()
        carried = doc.doc_param(ParamName("bore_r")).distribution
        doc.apply(
            DocEdit.set_doc_param(
                ParamName("bore_r"), DocParam.length(4.5 * mm, carried)
            )
        )
        self.assertEqual(doc.doc_param(ParamName("bore_r")).distribution, self.SPREAD)

    def test_the_edit_door_refuses_a_broken_annotation_typed(self):
        """`Distribution`'s constructors run the kernel's check, so the
        only way to a broken annotation is a file — and the edit door
        refuses one under its own tag either way."""
        doc = self.annotated()
        with self.assertRaises(DistributionFault):
            DocParam.length(4 * mm, Distribution.normal(0 * m))
        self.assertEqual(doc.doc_param(ParamName("bore_r")).distribution, self.SPREAD)


class TestTheRoundTrip(unittest.TestCase):
    """A first-time user's whole loop, through the bindings alone:
    declare, save, load, analyze."""

    def test_the_annotation_survives_save_and_load_bit_for_bit(self):
        doc = declared(
            bore_r=DocParam.length(4 * mm, Distribution.normal(5e-6 * m)),
            plate_t=DocParam.length(12 * mm, Distribution.band(-2e-4 * m, 2e-4 * m)),
        )
        back = load(doc.save()).doc
        self.assertTrue(back.bit_eq(doc), "the annotation round-trips bit for bit")
        self.assertEqual(
            back.doc_param(ParamName("bore_r")).distribution,
            doc.doc_param(ParamName("bore_r")).distribution,
        )

        boxed = analyzed_box(back)
        bore = boxed.get(ParamName("bore_r"))
        plate = boxed.get(ParamName("plate_t"))
        # The normal's box is the ±3σ quantile box; the band's IS its
        # support.
        self.assertAlmostEqual(bore.offsets[1].meters, 15e-6, delta=1e-8)
        self.assertEqual(plate.offsets, (-2e-4 * m, 2e-4 * m))
        self.assertEqual(plate.absolute(), (12 * mm - 2e-4 * m, 12 * mm + 2e-4 * m))

        # The tail column: the normal leaves a little outside its box,
        # the band leaves nothing outside its own support.
        tail = boxed.tail_mass(ParamName("bore_r"))
        self.assertAlmostEqual(tail, 1.0 - DEFAULT_QUANTILE_MASS, delta=1e-12)
        self.assertEqual(boxed.tail_mass(ParamName("plate_t")), 0.0)

        # Pricing a sub-box: the normal answers, the band refuses BY
        # NAME.
        half = boxed.box_mass(ParamName("bore_r"), 0 * m, bore.offsets[1])
        self.assertAlmostEqual(half, 0.5 * (1.0 - tail), delta=1e-9)
        with self.assertRaises(MeasureUnavailable) as ctx:
            boxed.box_mass(ParamName("plate_t"), 0 * m, 1e-4 * m)
        self.assertEqual(ctx.exception.param, "plate_t")

    def test_an_angle_parameter_carries_an_angle_spread(self):
        """The annotation is in the PARAMETER's dimension, whichever
        that is — the offsets read back as the quantities they were
        written as, through a save and a load."""
        doc = declared(draft=DocParam.angle(2 * deg, Distribution.normal(0.1 * deg)))
        back = load(doc.save()).doc
        spread = back.doc_param(ParamName("draft")).distribution
        self.assertEqual(spread.dimension, "angle")
        self.assertAlmostEqual(spread.sigma.in_unit(deg), 0.1, delta=1e-12)
        drafted = analyzed_box(back).get(ParamName("draft"))
        self.assertEqual(drafted.dimension, "angle")
        self.assertAlmostEqual(drafted.offsets[1].in_unit(deg), 0.3, delta=5e-5)


class TestTheAnnotationDoesNotMoveGeometry(unittest.TestCase):
    """A distribution feeds no evaluation, no content key and no
    predicate: an annotated document builds the same solid."""

    def build(self, param):
        doc = Doc("annotated-solid")
        doc.apply(DocEdit.set_doc_param(ParamName("h"), param))
        profile = doc.insert(
            Node.polygon(
                [(0 * m, 0 * m), (1 * m, 0 * m), (1 * m, 1 * m), (0 * m, 1 * m)],
                plane=doc.sketch_frame(),
            )
        )
        return doc, doc.insert(Node.extrude(profile, 2 * m))

    def test_the_solid_is_the_same_annotated_or_not(self):
        plain_doc, plain_solid = self.build(DocParam.length(2 * m))
        marked_doc, marked_solid = self.build(
            DocParam.length(2 * m, Distribution.normal(0.01 * m))
        )
        plain = pncad.evaluate(plain_doc).value(plain_solid).body()
        marked = pncad.evaluate(marked_doc).value(marked_solid).body()
        self.assertAlmostEqual(
            plain.mass_properties().volume, marked.mass_properties().volume, delta=1e-12
        )


if __name__ == "__main__":
    unittest.main()
