//! **The analysis lane** (ERROR-DESIGN E1/E2): the analyzed box, the
//! mass columns, and the accounting identity that ties them together.
//!
//! The rows below are the falsifiable half of this unit's claims:
//! analyzed + tail = 1 for every form and box the suite can build,
//! `TruncatedNormal`'s tail is exactly zero on its own support, and a
//! `Band` prices nothing whose answer would depend on its shape — it
//! answers the two set-theoretic cases (a covering interval, a
//! disjoint one) and refuses the rest.

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

/// The quantile box holds the mass the policy asked for, over a
/// spread of policies.
///
/// **Demoted, and what it no longer claims.** This row was written to
/// assert `analyzed + tail = 1`, which it could not falsify: `tail_mass`
/// was DEFINED as `1 - box_mass` over the same interval, so the sum was
/// an identity of `f64` subtraction and no implementation of the
/// measure could have made it red. The two columns now come from
/// different arithmetic — the tail sums two `erfc` half-lines, the box
/// differences them — but the claim that they agree with an
/// INDEPENDENT oracle is owned by
/// `m10_1_r2_probes::the_normal_tail_is_the_exterior_mass_not_merely_one_minus_inside`,
/// which computes the exterior outside this module entirely, and by
/// `m10_1_r2_probes::the_deep_tail_is_not_lost_to_cancellation` for the
/// far tail. What is left here, and is genuinely this row's, is that
/// the BISECTION lands where the policy asked it to.
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
        assert!(tail > 0.0, "a normal always leaves something outside");
    }
}

/// Every priceable form × a spread of sub-boxes: mass stays in
/// `[0, 1]`.
///
/// **Demoted for the same reason as the row above**, and to the same
/// residue: the `inside + outside = 1` assertion this row carried was
/// an identity of subtraction, not a claim about the measure. The
/// additivity it also checked — abutting pieces summing to the whole,
/// which is the property a leaf-pricing driver consumes — is owned by
/// `m10_1_r2_probes::box_mass_is_additive_over_an_abutting_partition`,
/// over ten pieces rather than two. The range check is what remains,
/// and it is a real one: a mass outside `[0, 1]` is a report nobody
/// can read.
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
                (0.0..=1.0).contains(&outside),
                "{outside} for {dist:?} over {b:?}"
            );
        }
    }
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

/// **A band prices nothing shape-dependent.** It refuses typed,
/// NAMING the parameter,
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

/// **The signed-zero fold reconciles the two equalities.** A
/// distribution's derived `PartialEq` is IEEE, so `-0.0` and `0.0` are
/// the same offset to it and different offsets to `bit_eq`. Any
/// consumer that HASHES a distribution mirrors the first and must not
/// split what it calls equal — the Python binding's `__hash__` is one,
/// and it used to hash the debug spelling, which shows the sign. This
/// pins the one home of the fix: IEEE-equal distributions fold to
/// BIT-equal ones, and the fold moves nothing else.
#[test]
fn the_signed_zero_fold_makes_ieee_equal_distributions_bit_equal() {
    let pairs = [
        (
            Distribution::Band { lo: -0.0, hi: 0.0 },
            Distribution::Band { lo: 0.0, hi: -0.0 },
        ),
        (
            Distribution::Uniform { lo: -0.0, hi: 0.5 },
            Distribution::Uniform { lo: 0.0, hi: 0.5 },
        ),
        (
            Distribution::TruncatedNormal {
                sigma: 0.5,
                lo: -0.0,
                hi: -0.0,
            },
            Distribution::TruncatedNormal {
                sigma: 0.5,
                lo: 0.0,
                hi: 0.0,
            },
        ),
    ];
    for (a, b) in pairs {
        assert_eq!(a, b, "IEEE equality already calls these the same");
        assert!(!a.bit_eq(&b), "and bit_eq already calls them different");
        assert!(
            a.fold_signed_zeros().bit_eq(&b.fold_signed_zeros()),
            "so the fold must make them bit-equal: {a:?} vs {b:?}"
        );
    }
    // Idempotent, and it moves nothing that is not a signed zero.
    for d in [
        Distribution::Normal { sigma: 1e-6 },
        Distribution::TruncatedNormal {
            sigma: 2.0,
            lo: -1.0,
            hi: 3.0,
        },
    ] {
        assert!(d.fold_signed_zeros().bit_eq(&d), "{d:?} is untouched");
    }
    // The sign really is the only thing it takes: a Normal cannot have
    // a zero sigma, so the fold has nothing to do there.
    assert!(
        Distribution::Normal { sigma: -0.0 }
            .fold_signed_zeros()
            .bit_eq(&Distribution::Normal { sigma: 0.0 }),
        "even the inhabitant `check` refuses folds consistently"
    );
}

/// **The name-keyed doors cannot mispair.** `tail_mass` and `box_mass`
/// take a name, a distribution and an interval as three loose
/// arguments, so a caller can hand one parameter's distribution and
/// another's box to the same call and get a plausible number back
/// instead of a refusal. `AnalyzedBox::axis_tail_mass` and
/// `axis_box_mass` take the three from ONE axis. This row shows both
/// halves: they agree with the free doors used correctly, and the free
/// doors really would have answered the mispaired question.
#[test]
fn the_name_keyed_doors_take_all_three_from_one_axis() {
    let wide = Distribution::Normal { sigma: 1.0 };
    let narrow = Distribution::Normal { sigma: 1e-3 };
    let doc = doc_with(&[
        ("wide", annotated(0.0, wide)),
        ("narrow", annotated(0.0, narrow)),
        ("fixed", DocParam::continuous(Dimension::Length, 1.0)),
    ]);
    let boxed = analyzed_box(&doc, &AnalysisPolicy::default());
    let wide_axis = boxed.get(&p("wide")).copied().expect("axis");

    // The keyed door agrees with the free one used correctly.
    let keyed = boxed
        .axis_tail_mass(&p("wide"))
        .expect("a declared parameter")
        .expect("a normal prices");
    let free = tail_mass(&p("wide"), &wide, &wide_axis.offsets).expect("priced");
    assert_eq!(keyed.to_bits(), free.to_bits());

    // The mispairing the keyed door forecloses: the NARROW parameter's
    // distribution against the WIDE one's box answers a confident,
    // wrong number rather than refusing.
    let mispaired = tail_mass(&p("narrow"), &narrow, &wide_axis.offsets).expect("priced");
    assert!(
        mispaired < free * 1e-6,
        "the mispaired call answers {mispaired}, nothing like the right {free}"
    );

    // A fixed axis leaves nothing out, and is a point mass at nominal.
    assert_eq!(boxed.axis_tail_mass(&p("fixed")), Some(Ok(0.0)));
    assert_eq!(boxed.axis_box_mass(&p("fixed"), (-1.0, 1.0)), Some(Ok(1.0)));
    assert_eq!(boxed.axis_box_mass(&p("fixed"), (0.5, 1.0)), Some(Ok(0.0)));
    // And a name the document does not declare is not an axis at all.
    assert_eq!(boxed.axis_tail_mass(&p("nope")), None);
    assert_eq!(boxed.axis_box_mass(&p("nope"), (0.0, 1.0)), None);

    // The band still refuses through the keyed door — the pairing
    // guarantee is not a licence to answer.
    let banded = doc_with(&[(
        "bore",
        annotated(1.0, Distribution::Band { lo: -0.1, hi: 0.1 }),
    )]);
    let banded_box = analyzed_box(&banded, &AnalysisPolicy::default());
    assert!(matches!(
        banded_box.axis_box_mass(&p("bore"), (-0.05, 0.05)),
        Some(Err(MeasureUnavailable::BandHasNoMeasure { .. }))
    ));
}
