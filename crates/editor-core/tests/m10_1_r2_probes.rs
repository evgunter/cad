//! **M10-1 review probes (R2)** — an INDEPENDENT derivation of what
//! the unit claims, written from `docs/M10-1-SPEC.md` and
//! `docs/ERROR-DESIGN.md` E1/E2 rather than from the implementation's
//! own rows.
//!
//! The rows here exist because the unit's own accounting rows are
//! pinned by CONSTRUCTION: `tail_mass` is *defined* as
//! `1 - box_mass(same interval)`, so every `inside + outside == 1`
//! assertion in `m10_1_analysis.rs` is an identity of `f64` addition
//! and cannot go red for any implementation of the measure. This suite
//! computes the exterior mass a SECOND way — from `erfc` and from
//! abutting partitions — so that "the columns add up" is a claim about
//! the measure rather than about subtraction.
//!
//! Sweep shape (`memories/test-suite-cost.md`): the two randomized
//! rows are COUNTEREXAMPLE SEARCH — varying seed via `test_utils::fuzz`,
//! counts on the shared effort dial, replay string in every assertion
//! message. The rest are witnesses that can be written down, so they
//! are static fixtures and are asserted every run. Rows marked
//! EVIDENCE-ONLY in their doc comment assert only that a documented
//! behaviour is still what it is; they gate nothing new.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/editor-core/src/distribution.rs",
    "crates/editor-core/src/analysis.rs",
    "crates/editor-core/src/measure.rs",
    "crates/geom-core/src/tolerance.rs",
];

use crate::fixture;

use editor_core::{
    AnalysisPolicy, CancelToken, DEFAULT_QUANTILE_MASS, Dimension, Distribution, DocEdit, DocParam,
    DocParamValue, DocumentId, EditError, EvalOptions, MeasureUnavailable, OffsetInterval,
    ParamName, PersistError, ProfileDoc, analyzed_box, apply, box_mass, evaluate, load, save,
    tail_mass,
};
use geom_core::Tol;
use test_utils::fuzz;

fn p(name: &str) -> ParamName {
    ParamName::new(name)
}

fn doc_with(params: &[(&str, DocParam)]) -> ProfileDoc {
    let mut doc = ProfileDoc::empty(DocumentId::derive("m10-1-r2-probes"), Tol::witness());
    for (name, value) in params {
        doc = apply(
            &doc,
            &DocEdit::SetDocParam {
                name: p(name),
                value: value.clone(),
            },
            Tol::witness(),
        )
        .expect("the fixture parameters are valid")
        .doc;
    }
    doc
}

fn annotated(value: f64, distribution: Distribution) -> DocParam {
    DocParam::continuous_with(Dimension::Length, value, distribution)
}

// ---------------------------------------------------------------
// 1. Accounting, derived a SECOND way
// ---------------------------------------------------------------

/// `P(X > x)` for `X ~ N(0, sigma^2)`, via `erfc` — the complementary
/// route, which keeps its precision in the far tail where
/// `1 - erf(...)` cancels.
fn normal_upper_tail(sigma: f64, x: f64) -> f64 {
    0.5 * libm::erfc(x / (sigma * core::f64::consts::SQRT_2))
}

/// **The accounting identity, computed independently.** For a
/// `Normal`, the mass outside `[lo, hi]` is `P(X < lo) + P(X > hi)`,
/// which shares no arithmetic with `1 - box_mass`. If `tail_mass`ever
/// stops meaning "the mass the box left out" — because the box door
/// and the mass door disagree about which interval they are talking
/// about, say — this goes red where the unit's own identity rows
/// cannot.
#[test]
fn the_normal_tail_is_the_exterior_mass_not_merely_one_minus_inside() {
    let mut rng = fuzz::start("m10-1-r2 normal tail");
    let cases = fuzz::scaled(200);
    for case in 0..cases {
        let sigma = 10f64.powf(rng.range(-4.0, 3.0));
        let lo = -sigma * rng.range(0.05, 6.0);
        let hi = sigma * rng.range(0.05, 6.0);
        let dist = Distribution::Normal { sigma };
        let interval = OffsetInterval { lo, hi };
        let reported = tail_mass(&p("n"), &dist, &interval).expect("a normal prices");
        // The second route: the two exterior half-lines, each from
        // `erfc`, never touching `1 - x`.
        let independent = normal_upper_tail(sigma, hi) + normal_upper_tail(sigma, -lo);
        assert!(
            (reported - independent).abs() <= 1e-12 + 8.0 * f64::EPSILON * independent.max(1.0),
            "case {case}: tail_mass says {reported}, the erfc exterior says {independent} \
             (sigma = {sigma}, box = [{lo}, {hi}]); {}",
            fuzz::replay()
        );
    }
}

/// **The deep tail survives, to full relative precision.** Written as
/// the falsifying witness for a defect that is now fixed, and INVERTED
/// at the update site its own text scheduled: `tail_mass` used to
/// compute the exterior as `1 - 0.5*(erf(hi/s) - erf(lo/s))`, whose
/// subtraction cancels — ~2% wrong at ±8σ, and a bit-exact `0.0` from
/// ~±8.5σ outward over a support that is the whole line. It now sums
/// the two `erfc` half-lines instead.
///
/// The assertions below are the same three probes with their senses
/// turned over: the ±3σ default was always accurate and still is; ±8σ
/// must now AGREE with the independent `erfc` oracle to a few ulps
/// instead of disagreeing by more than a part in a thousand; and ±9σ
/// must report the `~2e-19` of mass that is really out there instead
/// of a bit-exact zero. E2 makes the tail an explicit additive term
/// ("reported, never dropped") and E10 builds the unresolved-mass
/// budget on it, which is why the number rather than its order of
/// magnitude is the claim.
///
/// The oracle here is genuinely independent of the fix: it is this
/// file's own `normal_upper_tail`, `0.5 * erfc(x / (sigma * √2))` per
/// half-line, written before the module was changed.
#[test]
fn the_deep_tail_is_not_lost_to_cancellation() {
    let sigma = 1.0;
    let dist = Distribution::Normal { sigma };
    // Where it still holds: the ±3σ default is accurate.
    let near = OffsetInterval { lo: -3.0, hi: 3.0 };
    let near_reported = tail_mass(&p("n"), &dist, &near).expect("priceable");
    let near_exact = normal_upper_tail(sigma, 3.0) * 2.0;
    assert!(
        (near_reported - near_exact).abs() / near_exact < 1e-12,
        "at 3 sigma the tail is accurate: {near_reported} vs {near_exact}"
    );
    // Where it does not: 8 sigma.
    let far = OffsetInterval { lo: -8.0, hi: 8.0 };
    let far_reported = tail_mass(&p("n"), &dist, &far).expect("priceable");
    let far_exact = normal_upper_tail(sigma, 8.0) * 2.0;
    assert!(
        far_exact > 0.0,
        "there really is mass out there: {far_exact}"
    );
    let rel = (far_reported - far_exact).abs() / far_exact;
    assert!(
        rel < 1e-12,
        "at 8 sigma the reported tail {far_reported} must agree with the erfc exterior \
         {far_exact} to full relative precision (saw a relative error of {rel})"
    );
    // And the endpoint the old subtraction rounded away: real mass,
    // reported as the real number.
    let wider = OffsetInterval { lo: -9.0, hi: 9.0 };
    let wider_reported = tail_mass(&p("n"), &dist, &wider).expect("priceable");
    let wider_exact = normal_upper_tail(sigma, 9.0) * 2.0;
    assert!(wider_exact > 0.0, "{wider_exact}");
    assert!(
        wider_reported > 0.0,
        "{wider_exact} of mass is out there and must not be reported as zero"
    );
    assert!(
        (wider_reported - wider_exact).abs() / wider_exact < 1e-12,
        "at 9 sigma: reported {wider_reported}, oracle {wider_exact}"
    );
    // Far enough out that the mass itself underflows, the answer is a
    // true zero rather than a rounded one — and it is still the
    // oracle's answer, not a different door's.
    let vast = OffsetInterval {
        lo: -50.0,
        hi: 50.0,
    };
    assert_eq!(
        tail_mass(&p("n"), &dist, &vast).expect("priceable"),
        normal_upper_tail(sigma, 50.0) * 2.0,
        "where erfc underflows, both routes underflow together"
    );
}

/// **The analyzed box really holds the mass the policy asked for.**
/// The bisection's job is `erf(z/sqrt 2) = mass`; this checks the
/// answer against the measure rather than against the bisection, over
/// random masses, and checks the box is symmetric to the bit.
#[test]
fn the_analyzed_box_holds_the_requested_mass_for_random_policies() {
    let mut rng = fuzz::start("m10-1-r2 quantile box");
    let cases = fuzz::scaled(120);
    for case in 0..cases {
        let mass = rng.range(1e-3, 1.0 - 1e-9);
        let sigma = 10f64.powf(rng.range(-3.0, 2.0));
        let policy = AnalysisPolicy::new(mass).expect("a mass strictly inside (0, 1)");
        let doc = doc_with(&[("n", annotated(1.0, Distribution::Normal { sigma }))]);
        let axis = analyzed_box(&doc, &policy)
            .get(&p("n"))
            .copied()
            .expect("a continuous param is an axis");
        assert_eq!(
            axis.offsets.lo.to_bits(),
            (-axis.offsets.hi).to_bits(),
            "case {case}: the quantile box must be symmetric to the bit; {}",
            fuzz::replay()
        );
        let held = box_mass(
            &p("n"),
            &Distribution::Normal { sigma },
            (axis.offsets.lo, axis.offsets.hi),
        )
        .expect("a normal prices");
        assert!(
            (held - mass).abs() < 1e-12,
            "case {case}: the box asked for {mass} and holds {held} (sigma = {sigma}); {}",
            fuzz::replay()
        );
    }
}

/// **A partition adds to the whole, for every priceable form.** Ten
/// abutting sub-intervals across the analyzed box must sum to the box
/// mass. This is the accounting claim with no subtraction in it at
/// all: it goes red if `box_mass` is not additive over disjoint
/// pieces, which is the property a leaf-pricing driver (E6) actually
/// consumes.
#[test]
fn box_mass_is_additive_over_an_abutting_partition() {
    let forms = [
        Distribution::Uniform { lo: -0.2, hi: 0.3 },
        Distribution::Normal { sigma: 0.1 },
        Distribution::TruncatedNormal {
            sigma: 0.1,
            lo: -0.2,
            hi: 0.3,
        },
    ];
    for dist in forms {
        let (lo, hi) = (-0.4, 0.5);
        let whole = box_mass(&p("x"), &dist, (lo, hi)).expect("priceable");
        let n = 10;
        let step = (hi - lo) / f64::from(n);
        let mut sum = 0.0;
        for i in 0..n {
            let a = lo + step * f64::from(i);
            let b = lo + step * f64::from(i + 1);
            sum += box_mass(&p("x"), &dist, (a, b)).expect("priceable");
        }
        assert!(
            (sum - whole).abs() < 1e-12,
            "{dist:?}: the ten abutting pieces hold {sum}, the whole holds {whole}"
        );
    }
}

/// **A truncated normal's own support holds all of it, and its tail is
/// EXACTLY zero** — not "within an epsilon of zero". Checked against
/// the bit pattern, over an asymmetric window, so a renormalization
/// that merely got close would go red.
#[test]
fn the_truncated_normal_tail_is_bit_exactly_zero_on_its_own_support() {
    for (sigma, lo, hi) in [
        (0.1, -0.05, 0.05),
        (0.01, -0.3, 0.001),
        (1.0, -1e-6, 4.0),
        (1e3, -1.0, 2.0),
    ] {
        let dist = Distribution::TruncatedNormal { sigma, lo, hi };
        let support = OffsetInterval { lo, hi };
        let tail = tail_mass(&p("t"), &dist, &support).expect("priceable");
        assert_eq!(
            tail.to_bits(),
            0.0f64.to_bits(),
            "{dist:?}: tail is {tail}, not a positive zero"
        );
        assert_eq!(box_mass(&p("t"), &dist, (lo, hi)), Ok(1.0), "{dist:?}");
    }
}

// ---------------------------------------------------------------
// 2. Band — what "prices nothing" actually means here
// ---------------------------------------------------------------

/// **EVIDENCE-ONLY (a reported deviation, pinned).** The spec's §4
/// says `box_mass` on a `Band` "refuses typed, naming the parameter",
/// and review claim 4 says "Band prices nothing anywhere". Neither is
/// literally what the code does: a `Band` ANSWERS `Ok(1.0)` for a box
/// covering its support and `Ok(0.0)` for a disjoint one. The PR
/// discloses and argues this (they are the values every measure
/// supported on the band agrees on), so the row pins the behaviour
/// rather than asserting it is wrong — but it pins it, so a later
/// reading of "prices nothing" as "always refuses" has to change a
/// test rather than discover the disagreement in a driver.
#[test]
fn a_band_does_price_the_two_set_theoretic_cases() {
    let dist = Distribution::Band { lo: -0.1, hi: 0.1 };
    assert_eq!(
        box_mass(&p("bore"), &dist, (-0.5, 0.5)),
        Ok(1.0),
        "a covering box is priced at 1, not refused"
    );
    assert_eq!(box_mass(&p("bore"), &dist, (0.5, 0.6)), Ok(0.0));
    assert_eq!(box_mass(&p("bore"), &dist, (-0.1, 0.1)), Ok(1.0));
    // And the consequence a driver sees: a document whose only
    // parameter is a Band reports a COMPLETE analysis — zero tail —
    // over a distribution that states no shape at all.
    let doc = doc_with(&[("bore", annotated(1.0, dist))]);
    let axis = analyzed_box(&doc, &AnalysisPolicy::default())
        .get(&p("bore"))
        .copied()
        .expect("axis");
    assert_eq!(tail_mass(&p("bore"), &dist, &axis.offsets), Ok(0.0));
    // Anything finer refuses, which is the honest half.
    assert!(matches!(
        box_mass(&p("bore"), &dist, (-0.05, 0.05)),
        Err(MeasureUnavailable::BandHasNoMeasure { .. })
    ));
}

// ---------------------------------------------------------------
// 3. Degenerate but legal inhabitants
// ---------------------------------------------------------------

/// **A zero-width bounded form is legal and is a point mass.**
/// `check` requires `sigma > 0` on the normal forms with the reason
/// "a fixed parameter is spelled by having NO distribution", yet
/// `Band { 0, 0 }`, `Uniform { 0, 0 }` and `TruncatedNormal { .., 0,
/// 0 }` are all accepted and all analyze to the FIXED axis — a second
/// spelling of the fixed parameter, reachable through every door.
/// Pinned because it is the corner where the mass doors divide by a
/// zero width.
#[test]
fn zero_width_bounded_forms_are_accepted_and_analyze_as_fixed() {
    for dist in [
        Distribution::Band { lo: 0.0, hi: 0.0 },
        Distribution::Uniform { lo: 0.0, hi: 0.0 },
        Distribution::TruncatedNormal {
            sigma: 0.1,
            lo: 0.0,
            hi: 0.0,
        },
    ] {
        assert!(dist.check().is_ok(), "{dist:?} is a legal inhabitant");
        let doc = doc_with(&[("z", annotated(2.0, dist))]);
        let axis = analyzed_box(&doc, &AnalysisPolicy::default())
            .get(&p("z"))
            .copied()
            .expect("axis");
        assert_eq!(axis.offsets, OffsetInterval::FIXED, "{dist:?}");
        assert!(
            axis.offsets.is_fixed(),
            "{dist:?} is indistinguishable from an UNANNOTATED param on the box"
        );
        // ... and yet it carries a distribution, unlike the plain one.
        assert!(axis.distribution.is_some());
        // The point mass is held whole by any window containing it,
        // and nothing divides by zero.
        let m = box_mass(&p("z"), &dist, (-1.0, 1.0));
        assert!(matches!(m, Ok(x) if x == 1.0), "{dist:?}: {m:?}");
    }
}

/// **EVIDENCE-ONLY.** `tail_mass` returning exactly `0.0` does NOT
/// mean the support is bounded: a wide enough box makes the mass
/// outside it UNDERFLOW, and an unbounded normal then reports a
/// bit-exact zero tail. The type carries no distinction between
/// "provably no mass outside" and "the mass outside underflowed", and
/// E2's honesty gate (the unresolved-mass budget) is downstream of
/// exactly this number.
///
/// What the `erfc` rewrite moved is WHERE this starts, not whether it
/// happens: the boundary was `erf` saturating at ~±8.5σ and is now the
/// smallest subnormal at ~±38σ, so the box below sits at ±100σ, far
/// outside anything a policy in `(0, 1)` can draw. The distinction the
/// type does not carry is unchanged, which is why this row stays.
#[test]
fn an_unbounded_normal_can_report_a_bit_exactly_zero_tail() {
    let dist = Distribution::Normal { sigma: 1.0 };
    let wide = OffsetInterval {
        lo: -100.0,
        hi: 100.0,
    };
    let tail = tail_mass(&p("n"), &dist, &wide).expect("priceable");
    assert_eq!(
        tail.to_bits(),
        0.0f64.to_bits(),
        "an unbounded support reports the same zero a truncated one does, got {tail}"
    );
    // The default policy does NOT hit this, which is why it is a
    // latent shape rather than a live defect.
    let doc = doc_with(&[("n", annotated(1.0, dist))]);
    let axis = analyzed_box(&doc, &AnalysisPolicy::default())
        .get(&p("n"))
        .copied()
        .expect("axis");
    let default_tail = tail_mass(&p("n"), &dist, &axis.offsets).expect("priceable");
    assert!(
        default_tail > 0.0,
        "at the ±3σ default the tail is real: {default_tail}"
    );
    assert!(
        (default_tail - (1.0 - DEFAULT_QUANTILE_MASS)).abs() < 1e-12,
        "and it is the mass the policy declined to analyze: {default_tail}"
    );
}

/// **The bisection is deterministic and monotone.** Repeated calls
/// agree bit for bit (it consults nothing but `mass`), and a wider
/// requested mass never draws a narrower box — the property that makes
/// "the error only moves mass between columns" meaningful.
#[test]
fn the_quantile_bisection_is_deterministic_and_monotone() {
    let width = |mass: f64| {
        let doc = doc_with(&[("n", annotated(0.0, Distribution::Normal { sigma: 1.0 }))]);
        analyzed_box(&doc, &AnalysisPolicy::new(mass).expect("valid"))
            .get(&p("n"))
            .copied()
            .expect("axis")
            .offsets
    };
    let a = width(DEFAULT_QUANTILE_MASS);
    for _ in 0..8 {
        let b = width(DEFAULT_QUANTILE_MASS);
        assert_eq!(a.lo.to_bits(), b.lo.to_bits(), "bit-identical on replay");
        assert_eq!(a.hi.to_bits(), b.hi.to_bits(), "bit-identical on replay");
    }
    let mut prev = 0.0;
    for mass in [
        1e-6,
        0.01,
        0.25,
        0.5,
        0.9,
        0.99,
        DEFAULT_QUANTILE_MASS,
        0.999_999_9,
    ] {
        let w = width(mass).width();
        assert!(
            w >= prev,
            "monotone in the requested mass: {mass} drew {w} after {prev}"
        );
        assert!(w.is_finite() && w > 0.0, "{mass} drew a real box: {w}");
        prev = w;
    }
}

// ---------------------------------------------------------------
// 4. Planted corruptions the unit's own suite does not plant
// ---------------------------------------------------------------

/// A saved v15 document with one `Normal` parameter, and the text of
/// its body, for corruption.
fn saved_normal() -> String {
    let doc = doc_with(&[("s", annotated(1.0, Distribution::Normal { sigma: 0.01 }))]);
    save(&doc, &[], Tol::witness()).expect("saves")
}

/// **The OTHER shape fault, planted at LOAD.** The unit pins a
/// hand-written `sigma: -1`; the bounds fault (`lo <= 0 <= hi`) has no
/// load-door row. It refuses correctly — this row is the proof, not a
/// re-reading of the validator.
#[test]
fn a_hand_written_nominal_outside_support_refuses_at_load() {
    let doc = doc_with(&[(
        "b",
        annotated(1.0, Distribution::Uniform { lo: -0.2, hi: 0.3 }),
    )]);
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let corrupt = text.replace("\"lo\": -0.2", "\"lo\": 0.2");
    assert_ne!(corrupt, text, "the corruption must land");
    match load(&corrupt, Tol::witness()) {
        Err(PersistError::Distribution { name, fault }) => {
            assert_eq!(name.0, "b");
            assert_eq!(
                fault,
                editor_core::DistributionFault::NominalOutsideSupport { lo: 0.2, hi: 0.3 }
            );
        }
        other => panic!("a nominal outside its own support must refuse at LOAD, got {other:?}"),
    }
}

/// **Strict serde reaches INSIDE the new type.** A hand-written
/// distribution carrying a field its form has no name for — a `sigma`
/// on a `Band`, a stray key on a `Normal` — is unreadable by this
/// build rather than being dropped on the floor.
#[test]
fn an_unknown_key_inside_a_distribution_refuses_at_load() {
    let text = saved_normal();
    for corrupt in [
        text.replace("\"sigma\": 0.01", "\"sigma\": 0.01, \"lo\": -1.0"),
        text.replace("\"Normal\"", "\"Band\""),
    ] {
        assert_ne!(corrupt, text, "the corruption must land");
        let got = load(&corrupt, Tol::witness());
        assert!(
            matches!(got, Err(PersistError::Unreadable { .. })),
            "a mis-shaped distribution must refuse as unreadable, got {got:?}"
        );
    }
}

/// **A `Count` parameter cannot be handed a distribution even by
/// hand.** E11.3's "no distributions on structural parameters" is
/// claimed to come out UNREPRESENTABLE; the load door is where a
/// hand-written file would test that claim, and `deny_unknown_fields`
/// on `DocParam` is what has to enforce it.
#[test]
fn a_distribution_on_a_count_param_refuses_at_load() {
    let doc = doc_with(&[("n", DocParam::Count { value: 4 })]);
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let corrupt = text.replace(
        "\"value\": 4",
        "\"value\": 4, \"distribution\": {\"Normal\": {\"sigma\": 1.0}}",
    );
    assert_ne!(corrupt, text, "the corruption must land");
    let got = load(&corrupt, Tol::witness());
    assert!(
        matches!(got, Err(PersistError::Unreadable { .. })),
        "a Count carrying a distribution has no spelling, got {got:?}"
    );
}

/// **EVIDENCE-ONLY: an explicit `null` is accepted and normalized
/// away.** `#[serde(default)]` on an `Option` means a hand-written
/// `"distribution": null` loads as `None` and then re-saves WITHOUT
/// the key — so the byte-for-byte round trip the golden rows pin holds
/// only for files this writer produced. Recorded because "v15 goldens
/// round-trip bit-exact" is a claim about the writer's own output, not
/// about every legal v15 file.
#[test]
fn an_explicit_null_distribution_loads_and_is_normalized_out() {
    let doc = doc_with(&[("plain", DocParam::continuous(Dimension::Length, 1.0))]);
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    let corrupt = text.replace("\"value\": 1.0", "\"value\": 1.0, \"distribution\": null");
    assert_ne!(corrupt, text, "the corruption must land");
    let back = load(&corrupt, Tol::witness()).expect("an explicit null is accepted");
    assert_eq!(back.doc.params()[&p("plain")].distribution(), None);
    let again = save(&back.doc, &[], Tol::witness()).expect("saves");
    assert_ne!(
        again, corrupt,
        "the key the file spelled is gone from the rewrite"
    );
    assert_eq!(again, text, "and the rewrite is the canonical form");
}

// ---------------------------------------------------------------
// 5. Zero evaluation impact — the whole evaluation, not the env
// ---------------------------------------------------------------

/// **A distribution moves NO evaluation, checked at the memo
/// currency.** The unit's own row compares the parameter environment;
/// this one evaluates a real multi-node document (the die fixture,
/// whose `pip_depth` is a mid-DAG continuous parameter) with and
/// without a `Normal` on that parameter and compares every node's
/// CONTENT KEY, NAMING KEY and verdict log. Those are the three things
/// a memo and a diff are keyed on, so if a distribution ever leaked
/// below E1's boundary this is where it would show.
#[test]
fn a_distribution_changes_no_content_key_naming_key_or_verdict() {
    let plain = fixture::die().doc;
    let annotated_doc = apply(
        &plain,
        &DocEdit::SetDocParam {
            name: p("pip_depth"),
            value: annotated(fixture::DEPTH, Distribution::Normal { sigma: 0.001 }),
        },
        Tol::witness(),
    )
    .expect("annotating an existing parameter is a legal edit")
    .doc;
    assert!(
        !plain.bit_eq(&annotated_doc),
        "the two documents really do differ (or this row proves nothing)"
    );
    let run = |d: &ProfileDoc| {
        evaluate::<f64>(
            d,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        )
    };
    let (a, b) = (run(&plain), run(&annotated_doc));
    let mut compared = 0usize;
    for &id in plain.order() {
        let (va, vb) = (a.value(id), b.value(id));
        match (va, vb) {
            (Some(va), Some(vb)) => {
                assert_eq!(va.content_key, vb.content_key, "node {}", id.0);
                assert_eq!(va.naming_key, vb.naming_key, "node {}", id.0);
                assert_eq!(va.verdicts, vb.verdicts, "node {}", id.0);
                compared += 1;
            }
            (None, None) => {}
            _ => panic!("node {} evaluated on one side only", id.0),
        }
    }
    assert!(
        compared >= 8,
        "the fixture must actually evaluate for this to mean anything, compared {compared}"
    );
}

/// The same claim at the INTERVAL scalar — the lane the spec names
/// alongside f64.
#[cfg(feature = "interval")]
#[test]
fn a_distribution_changes_no_content_key_at_interval() {
    let plain = fixture::die().doc;
    let annotated_doc = apply(
        &plain,
        &DocEdit::SetDocParam {
            name: p("pip_depth"),
            value: annotated(fixture::DEPTH, Distribution::Normal { sigma: 0.001 }),
        },
        Tol::witness(),
    )
    .expect("legal edit")
    .doc;
    let run = |d: &ProfileDoc| {
        evaluate::<geom_core::Interval>(
            d,
            None,
            &CancelToken::new(),
            &EvalOptions::default(),
            Tol::witness(),
        )
    };
    let (a, b) = (run(&plain), run(&annotated_doc));
    let mut compared = 0usize;
    for &id in plain.order() {
        if let (Some(va), Some(vb)) = (a.value(id), b.value(id)) {
            assert_eq!(va.content_key, vb.content_key, "node {}", id.0);
            assert_eq!(va.naming_key, vb.naming_key, "node {}", id.0);
            assert_eq!(va.verdicts, vb.verdicts, "node {}", id.0);
            compared += 1;
        }
    }
    assert!(compared >= 8, "compared {compared}");
}

// ---------------------------------------------------------------
// 6. The carry-forward CLASS, at the API level
// ---------------------------------------------------------------

/// **The rebuild-from-parts hazard, stated as a test.** `SetDocParam`
/// is create-or-replace, so ANY caller that reconstructs a parameter
/// from `(dim, value)` silently deletes the annotation, with no
/// refusal and no diagnostic. The hazard was never the GUI's: it
/// belongs to the door.
///
/// The door is still create-or-replace and this row still pins its
/// sharp edge, because that is what create-or-replace MEANS and a
/// redeclaration really does replace. What closed the class is that
/// rebuilding is no longer the only spelling of "move the value":
/// `SetDocParamValue` carries the declaration forward, and the
/// companion row below pins that it does. The pair is the point —
/// this one shows the edge, that one shows the door with no edge.
#[test]
fn rebuilding_a_param_from_dim_and_value_silently_drops_the_distribution() {
    let before = doc_with(&[(
        "hole_r",
        annotated(0.003, Distribution::Normal { sigma: 1e-5 }),
    )]);
    let existing = before.params()[&p("hole_r")].clone();
    assert!(existing.distribution().is_some());
    // The natural "just change the value" edit, spelled the only way
    // the constructors offer.
    let after = apply(
        &before,
        &DocEdit::SetDocParam {
            name: p("hole_r"),
            value: DocParam::continuous(existing.dim(), 0.004),
        },
        Tol::witness(),
    )
    .expect("the edit door accepts it — there is nothing to refuse")
    .doc;
    assert_eq!(
        after.params()[&p("hole_r")].distribution(),
        None,
        "the annotation is gone, silently"
    );
    // And the analysis agrees the parameter is now FIXED — the
    // modelling statement changed without anyone saying so.
    let axis = analyzed_box(&after, &AnalysisPolicy::default())
        .get(&p("hole_r"))
        .copied()
        .expect("axis");
    assert!(
        axis.offsets.is_fixed(),
        "a value edit turned a varying parameter into a fixed one"
    );
}

/// **The class, closed: the value door carries the declaration.** The
/// same edit the row above spells destructively, spelled through
/// `SetDocParamValue`: the number moves, the dimension stays, the
/// distribution survives BIT for bit, and the analysis still sees a
/// varying axis. Its two refusals are here too, because a door whose
/// safety depends on the caller checking first is not safe.
#[test]
fn the_value_door_carries_the_declaration_forward() {
    let dist = Distribution::Normal { sigma: 1e-5 };
    let before = doc_with(&[
        ("hole_r", annotated(0.003, dist)),
        ("ribs", DocParam::Count { value: 4 }),
    ]);
    let after = apply(
        &before,
        &DocEdit::SetDocParamValue {
            name: p("hole_r"),
            value: DocParamValue::Continuous(0.004),
        },
        Tol::witness(),
    )
    .expect("a value edit on a declared parameter applies")
    .doc;
    match after.params()[&p("hole_r")] {
        DocParam::Continuous {
            dim,
            value,
            display_unit: _,
            distribution,
        } => {
            assert_eq!(value, 0.004, "the number moved");
            assert_eq!(dim, Dimension::Length, "the dimension is the declaration's");
            let got = distribution.expect("the annotation SURVIVED");
            assert!(got.bit_eq(&dist), "and survived bit for bit");
        }
        DocParam::Count { .. } => panic!("still continuous"),
    }
    let axis = analyzed_box(&after, &AnalysisPolicy::default())
        .get(&p("hole_r"))
        .copied()
        .expect("axis");
    assert!(
        !axis.offsets.is_fixed(),
        "the parameter is still varying: {axis:?}"
    );
    // A count moves through the same door.
    let counted = apply(
        &before,
        &DocEdit::SetDocParamValue {
            name: p("ribs"),
            value: DocParamValue::Count(7),
        },
        Tol::witness(),
    )
    .expect("a count value edit applies")
    .doc;
    assert_eq!(counted.params()[&p("ribs")], DocParam::Count { value: 7 });
    // Refusal 1: nothing to carry forward.
    assert_eq!(
        apply(
            &before,
            &DocEdit::SetDocParamValue {
                name: p("never_declared"),
                value: DocParamValue::Continuous(1.0),
            },
            Tol::witness(),
        ),
        Err(EditError::DocParamNotDeclared {
            name: p("never_declared")
        })
    );
    // Refusal 2: a kind change is a redeclaration, not a value edit.
    assert_eq!(
        apply(
            &before,
            &DocEdit::SetDocParamValue {
                name: p("hole_r"),
                value: DocParamValue::Count(2),
            },
            Tol::witness(),
        ),
        Err(EditError::DocParamValueKindMismatch {
            name: p("hole_r"),
            declared: Dimension::Length,
            offered: DocParamValue::Count(2),
        })
    );
    assert!(matches!(
        apply(
            &before,
            &DocEdit::SetDocParamValue {
                name: p("ribs"),
                value: DocParamValue::Continuous(2.0),
            },
            Tol::witness(),
        ),
        Err(EditError::DocParamValueKindMismatch { .. })
    ));
}

/// **The value door survives a round trip.** The new edit is file
/// data: a document whose log carries one saves, loads and replays to
/// the same parameter — annotation included — so the carry-forward is
/// a property of the FORMAT and not only of this process's `apply`.
#[test]
fn a_value_edit_round_trips_through_the_file() {
    let dist = Distribution::TruncatedNormal {
        sigma: 2e-5,
        lo: -4e-5,
        hi: 3e-5,
    };
    let doc = doc_with(&[("bore", annotated(0.01, dist))]);
    let edits = [DocEdit::SetDocParamValue {
        name: p("bore"),
        value: DocParamValue::Continuous(0.011),
    }];
    let text = save(&doc, &edits, Tol::witness()).expect("saves");
    let back = load(&text, Tol::witness()).expect("loads");
    match back.doc.params()[&p("bore")] {
        DocParam::Continuous {
            value,
            distribution,
            ..
        } => {
            assert_eq!(value, 0.011, "the replayed value");
            assert!(
                distribution.expect("annotation replayed").bit_eq(&dist),
                "the annotation crossed the file and the replay"
            );
        }
        DocParam::Count { .. } => panic!("still continuous"),
    }
}
