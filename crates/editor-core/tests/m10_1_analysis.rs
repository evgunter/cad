//! **The analysis lane** (ERROR-DESIGN E1/E2): the analyzed box, the
//! mass columns, and the accounting identity that ties them together.
//!
//! The rows below are the falsifiable half of this unit's claims:
//! analyzed + tail = 1 for every form and box the suite can build,
//! `TruncatedNormal`'s tail is exactly zero on its own support, and a
//! `Band` prices nothing anywhere a measure is genuinely needed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use editor_core::{
    AnalysisPolicy, AnalysisPolicyError, DEFAULT_QUANTILE_MASS, Dimension, Distribution, DocEdit,
    DocParam, DocumentId, MeasureUnavailable, OffsetInterval, ParamName, ProfileDoc, analyzed_box,
    apply, box_mass, tail_mass,
};
use geom_core::Tol;

fn doc_with(params: &[(&str, DocParam)]) -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-1-analysis"), Tol::witness());
    for (name, value) in params {
        doc = apply(
            &doc,
            &DocEdit::SetDocParam {
                name: ParamName::new(*name),
                value: value.clone(),
            },
            Tol::witness(),
        )
        .expect("a valid parameter sets")
        .doc;
    }
    doc
}

fn annotated(value: f64, distribution: Distribution) -> DocParam {
    DocParam::Continuous {
        dim: Dimension::Length,
        value,
        distribution: Some(distribution),
    }
}

fn p(name: &str) -> ParamName {
    ParamName::new(name)
}

/// The default policy IS the ±3σ convention, and the box it draws for
/// a normal is ±3σ to the precision the convention names.
#[test]
fn the_default_policy_is_the_three_sigma_convention() {
    assert_eq!(
        AnalysisPolicy::default().quantile_mass(),
        DEFAULT_QUANTILE_MASS
    );
    let doc = doc_with(&[("n", annotated(1.0, Distribution::Normal { sigma: 0.01 }))]);
    let b = analyzed_box(&doc, &AnalysisPolicy::default());
    let axis = b.get(&p("n")).expect("the parameter is an axis");
    assert!(
        (axis.offsets.hi - 0.03).abs() < 5e-6,
        "±3σ to the convention's own precision, got {}",
        axis.offsets.hi
    );
    assert_eq!(
        axis.offsets.lo, -axis.offsets.hi,
        "symmetric by construction"
    );
    assert_eq!(axis.nominal, 1.0);
    assert_eq!(axis.absolute().0, 1.0 + axis.offsets.lo);
}

/// The knob is the ANALYSIS's: a wider requested mass widens the box
/// monotonically, and it is checked into `(0, 1)`.
#[test]
fn the_quantile_mass_is_a_checked_request_knob() {
    let doc = doc_with(&[("n", annotated(0.0, Distribution::Normal { sigma: 1.0 }))]);
    let width = |mass: f64| {
        let policy = AnalysisPolicy::new(mass).expect("a mass inside (0, 1)");
        analyzed_box(&doc, &policy)
            .get(&p("n"))
            .expect("axis")
            .offsets
            .width()
    };
    let (narrow, wide) = (width(0.5), width(0.99));
    assert!(narrow < wide, "{narrow} < {wide}");
    // The 50% box of a standard normal is the interquartile range.
    assert!((narrow - 2.0 * 0.674_489_75).abs() < 1e-6, "{narrow}");
    for bad in [0.0, 1.0, -0.5, 2.0, f64::INFINITY] {
        assert_eq!(
            AnalysisPolicy::new(bad),
            Err(AnalysisPolicyError::QuantileMassOutOfRange { mass: bad })
        );
    }
    // NaN refuses too; it just cannot be compared for equality.
    assert!(matches!(
        AnalysisPolicy::new(f64::NAN),
        Err(AnalysisPolicyError::QuantileMassOutOfRange { mass }) if mass.is_nan()
    ));
}

/// A parameter with NO distribution is FIXED, and a `Count` parameter
/// is not an axis at all: the analysis varies exactly what the author
/// declared variable.
#[test]
fn opt_in_means_an_unannotated_param_is_fixed() {
    let doc = doc_with(&[
        ("plain", DocParam::continuous(Dimension::Length, 2.0)),
        ("holes", DocParam::Count { value: 4 }),
        (
            "varies",
            annotated(1.0, Distribution::Band { lo: -0.1, hi: 0.1 }),
        ),
    ]);
    let b = analyzed_box(&doc, &AnalysisPolicy::default());
    assert_eq!(b.params().len(), 2, "Count is not a box axis");
    assert!(b.get(&p("holes")).is_none());
    let fixed = b.get(&p("plain")).expect("continuous params are axes");
    assert_eq!(fixed.offsets, OffsetInterval::FIXED);
    assert!(fixed.offsets.is_fixed());
    assert_eq!(fixed.absolute(), (2.0, 2.0), "width zero AT the nominal");
    assert_eq!(fixed.distribution, None);
    let varying: Vec<&ParamName> = b.varying().map(|(n, _)| n).collect();
    assert_eq!(varying, vec![&p("varies")], "only the declared axis varies");
}

/// The bounded forms ARE their own analyzed box, so nothing escapes:
/// their tail mass is exactly zero, `TruncatedNormal` included.
#[test]
fn the_bounded_forms_have_exactly_zero_tail() {
    for dist in [
        Distribution::Band { lo: -0.1, hi: 0.2 },
        Distribution::Uniform { lo: -0.1, hi: 0.2 },
        Distribution::TruncatedNormal {
            sigma: 0.05,
            lo: -0.1,
            hi: 0.2,
        },
    ] {
        let doc = doc_with(&[("b", annotated(1.0, dist))]);
        let axis = analyzed_box(&doc, &AnalysisPolicy::default())
            .get(&p("b"))
            .copied()
            .expect("axis");
        assert_eq!(axis.offsets, OffsetInterval { lo: -0.1, hi: 0.2 });
        assert_eq!(
            tail_mass(&p("b"), &dist, &axis.offsets),
            Ok(0.0),
            "a bounded form's own support leaves nothing outside: {dist:?}"
        );
    }
}

/// A normal's tail is what the box left out, and the two columns sum
/// to one — the accounting identity, over a spread of policies.
#[test]
fn analyzed_and_tail_mass_sum_to_one_for_a_normal() {
    let dist = Distribution::Normal { sigma: 0.01 };
    let doc = doc_with(&[("n", annotated(1.0, dist))]);
    for mass in [0.5, 0.9, DEFAULT_QUANTILE_MASS, 0.999_999] {
        let policy = AnalysisPolicy::new(mass).expect("valid");
        let axis = analyzed_box(&doc, &policy)
            .get(&p("n"))
            .copied()
            .expect("axis");
        let inside = box_mass(&p("n"), &dist, (axis.offsets.lo, axis.offsets.hi)).expect("priced");
        let tail = tail_mass(&p("n"), &dist, &axis.offsets).expect("priced");
        assert!(
            (inside - mass).abs() < 1e-12,
            "the box holds the mass it was asked for: {inside} vs {mass}"
        );
        assert!(
            (inside + tail - 1.0).abs() < 1e-12,
            "analyzed + tail = 1, got {inside} + {tail}"
        );
        assert!(tail > 0.0, "a normal always leaves something outside");
    }
}

/// Every priceable form × a spread of sub-boxes: mass stays in
/// `[0, 1]`, and inside + outside = 1.
#[test]
fn the_accounting_identity_holds_for_every_priceable_form_and_box() {
    let forms = [
        Distribution::Uniform { lo: -0.2, hi: 0.3 },
        Distribution::Normal { sigma: 0.1 },
        Distribution::TruncatedNormal {
            sigma: 0.1,
            lo: -0.2,
            hi: 0.3,
        },
    ];
    let boxes = [
        OffsetInterval {
            lo: -0.05,
            hi: 0.05,
        },
        OffsetInterval { lo: -0.2, hi: 0.3 },
        OffsetInterval { lo: -1.0, hi: 1.0 },
        OffsetInterval { lo: 0.0, hi: 0.0 },
        OffsetInterval { lo: -0.3, hi: 0.0 },
    ];
    for dist in forms {
        for b in boxes {
            let inside = box_mass(&p("x"), &dist, (b.lo, b.hi)).expect("priceable");
            let outside = tail_mass(&p("x"), &dist, &b).expect("priceable");
            assert!(
                (0.0..=1.0).contains(&inside),
                "{inside} for {dist:?} over {b:?}"
            );
            assert!(
                (inside + outside - 1.0).abs() < 1e-12,
                "{inside} + {outside} for {dist:?} over {b:?}"
            );
        }
    }
    // Sub-boxes partition: two abutting halves sum to the whole.
    let dist = Distribution::Normal { sigma: 0.1 };
    let whole = box_mass(&p("x"), &dist, (-0.4, 0.4)).expect("priced");
    let left = box_mass(&p("x"), &dist, (-0.4, 0.05)).expect("priced");
    let right = box_mass(&p("x"), &dist, (0.05, 0.4)).expect("priced");
    assert!(
        (left + right - whole).abs() < 1e-12,
        "{left} + {right} != {whole}"
    );
}

/// A truncated normal is renormalized, not merely clipped: its own
/// support holds ALL of its mass, and a half of it holds strictly
/// more than the underlying normal would.
#[test]
fn a_truncated_normal_is_renormalized() {
    let sigma = 0.1;
    let dist = Distribution::TruncatedNormal {
        sigma,
        lo: -0.05,
        hi: 0.05,
    };
    let support = OffsetInterval {
        lo: -0.05,
        hi: 0.05,
    };
    assert_eq!(box_mass(&p("t"), &dist, (-0.05, 0.05)), Ok(1.0));
    assert_eq!(tail_mass(&p("t"), &dist, &support), Ok(0.0));
    let half = box_mass(&p("t"), &dist, (0.0, 0.05)).expect("priced");
    assert!(
        (half - 0.5).abs() < 1e-12,
        "symmetric truncation halves: {half}"
    );
    let untruncated =
        box_mass(&p("t"), &Distribution::Normal { sigma }, (0.0, 0.05)).expect("priced");
    assert!(
        half > untruncated,
        "renormalization concentrates mass: {half} vs {untruncated}"
    );
}

/// **A band prices nothing.** It refuses typed, NAMING the parameter,
/// wherever the answer would depend on a shape it does not state — and
/// answers only where every measure on the band agrees.
#[test]
fn a_band_refuses_to_be_priced_and_names_the_parameter() {
    let dist = Distribution::Band { lo: -0.1, hi: 0.1 };
    let refusal = box_mass(&p("bore"), &dist, (-0.05, 0.05));
    assert_eq!(
        refusal,
        Err(MeasureUnavailable::BandHasNoMeasure { param: p("bore") })
    );
    let msg = refusal.unwrap_err().to_string();
    assert!(
        msg.contains("bore"),
        "the refusal names the parameter: {msg}"
    );
    assert!(
        msg.contains("no shape"),
        "and says what it does not know: {msg}"
    );
    // A partial overlap is refused from either side.
    assert!(box_mass(&p("bore"), &dist, (-1.0, 0.05)).is_err());
    assert!(box_mass(&p("bore"), &dist, (-0.05, 1.0)).is_err());
    assert!(tail_mass(&p("bore"), &dist, &OffsetInterval { lo: -0.05, hi: 0.1 }).is_err());
    // The two answers every measure on the band agrees on.
    assert_eq!(box_mass(&p("bore"), &dist, (-0.5, 0.5)), Ok(1.0));
    assert_eq!(box_mass(&p("bore"), &dist, (0.5, 0.6)), Ok(0.0));
    assert_eq!(
        tail_mass(&p("bore"), &dist, &OffsetInterval { lo: -0.2, hi: 0.2 }),
        Ok(0.0),
        "a box containing the whole band leaves nothing outside, whatever the shape"
    );
}

/// A uniform IS priced — the point of keeping the two forms apart. The
/// same limits under `Uniform` answer where `Band` refuses.
#[test]
fn a_uniform_answers_exactly_where_the_band_refuses() {
    let limits = (-0.1, 0.1);
    let uniform = Distribution::Uniform {
        lo: limits.0,
        hi: limits.1,
    };
    let band = Distribution::Band {
        lo: limits.0,
        hi: limits.1,
    };
    let sub = (-0.05, 0.05);
    assert_eq!(box_mass(&p("u"), &uniform, sub), Ok(0.5));
    assert!(box_mass(&p("u"), &band, sub).is_err());
}

/// A distribution moves no evaluation: the parameter environment the
/// evaluator reads is bit-identical with and without one.
#[test]
fn a_distribution_does_not_reach_the_parameter_environment() {
    let plain = doc_with(&[("d", DocParam::continuous(Dimension::Length, 0.75))]);
    let annotated_doc = doc_with(&[("d", annotated(0.75, Distribution::Normal { sigma: 0.01 }))]);
    assert_eq!(
        plain.param_env::<f64>().bindings,
        annotated_doc.param_env::<f64>().bindings,
        "the nominal alone crosses into evaluation"
    );
}
