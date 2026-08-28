//! Adversarial-review probes for M6-2 (PR #176), kept for regression
//! value. Three corruption species against the fitted-cache at-rest
//! pass, each pinning WHY the pass answers what it answers:
//!
//! 1. the PR's own planted corruption (a cache honestly certified for a
//!    DIFFERENT arc of the same locus) is rejected on the **map
//!    residual against the edge's own carrier** — the identity check —
//!    not on a side check like trim containment or the azimuth gate;
//! 2. the `EnvelopeStatement::OnLocusHull` gap, demonstrated concretely:
//!    a fitted image corrupted STRICTLY BETWEEN the nine schedule
//!    samples (exact B-spline basis locality, so every scheduled sample
//!    is bit-identical) **certifies and passes the at-rest pass** — on
//!    an analytic chart the between-samples map residual is certified
//!    at the schedule only, exactly as the variant's rustdoc says. If
//!    this row ever starts FAILING, the gap has been closed and the
//!    docs should flip with it;
//! 3. a cache honestly certified over a SUB-interval of its edge's
//!    span is caught — but by the LOOP-CONTINUITY walk, not by any
//!    param-coverage comparison (`recertify` re-derives over the
//!    cache's own stored params and never consults the edge's). The
//!    net holds today; this row pins which net it is.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use std::sync::Arc;

use geom::{Curve3, NurbsCurve2};
use geom_brep::{Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck};
use geom_core::Tol;
use geom_core::{Band, Point2};
use test_utils::vacuity::stood_down;
use topo::pcurves::{PcurveMintError, validate_pcurves};

/// Species 1: the foreign-arc cache fails the at-rest pass at the
/// schedule's map residual — the re-derivation compares the stored
/// image against the EDGE's carrier (read from the body, per the
/// intensional description), which is the identity check. A rejection
/// on any other check would be coincidental, not the law.
#[test]
fn the_foreign_arc_cache_fails_on_the_map_residual_against_the_edges_carrier() {
    let Some(mut built) = fixture::build::<f64>() else {
        stood_down(
            "M6-2 species 1: foreign-arc cache",
            "the cylinder×sphere fixture stood down on the SSI door's typed \
             FitSampleBudget refusal at this ε, so THIS RUN ASSERTS NOTHING about \
             species 1: not that the foreign-arc cache is rejected at all, and not \
             that the rejection is the map residual against the edge's own carrier \
             rather than a side check",
        );
        return;
    };
    let band = Band::linear(Tol::witness()).unwrap();
    let foreign = fixture::foreign_cache(&built);
    built.body.attach_pcurve(built.he_plus, foreign);
    let findings = validate_pcurves(&built.body, band);
    assert!(
        findings.iter().any(|f| matches!(
            f,
            PcurveMintError::Certify {
                half_edge,
                error: PcurveCertifyError::ResidualExceeded {
                    check: PcurveCheck::MapResidual,
                    ..
                },
            } if *half_edge == built.he_plus
        )),
        "the identity mismatch must surface as the map residual on the corrupted \
         half-edge, not as a side check: {findings:?}"
    );
}

/// Species 2: the OnLocusHull gap, concretely. The image is knot-refined
/// (exact in ℝ) and ONE control point whose basis support lies strictly
/// inside an inter-sample gap is displaced by 1e-3 m in the height
/// channel. B-spline locality makes every scheduled sample EXACTLY
/// untouched; between samples the chart image is off the carrier by
/// ~1e-3 m (six orders above the default ε). The full C2 certificate
/// (on-locus hull + uniqueness tube, both statements about the CARRIER)
/// admits it, and the at-rest pass finds nothing — the documented
/// honesty of `EnvelopeStatement::OnLocusHull`, pinned so it cannot
/// silently be mistaken for a map-residual sup bound later.
#[test]
fn a_between_samples_image_corruption_survives_the_full_c2_certificate() {
    let Some(mut built) = fixture::build::<f64>() else {
        stood_down(
            "M6-2 species 2: between-samples corruption",
            "the cylinder×sphere fixture stood down on the SSI door's typed \
             FitSampleBudget refusal at this ε, so THIS RUN ASSERTS NOTHING about \
             species 2: the OnLocusHull gap is unexercised — neither that a \
             between-samples image corruption still certifies, nor that the at-rest \
             pass finds nothing against it",
        );
        return;
    };
    let band = Band::linear(Tol::witness()).unwrap();
    let (t0, t1) = built.image.domain();

    // 64 uniform spans: schedule samples (i/8 of the span) land ON
    // inserted knots, so a control point supported strictly inside
    // (4/8, 5/8) of the span cannot touch any of them.
    let add: Vec<f64> = (1..64)
        .map(|i| t0 + (t1 - t0) * (f64::from(i) / 64.0))
        .collect();
    let refined = built.image.refine_knots(&add).expect("exact refinement");
    let knots = refined.knots().knots().to_vec();
    let p = refined.knots().degree();
    let mut ctl: Vec<Point2<f64>> = refined.control().to_vec();
    let (lo, hi) = (t0 + (t1 - t0) * 0.5, t0 + (t1 - t0) * 0.625);
    let idx = (0..ctl.len())
        .find(|&i| knots[i] > lo && knots[i + p + 1] < hi)
        .expect("a control point supported strictly between two schedule samples");
    let support_mid = 0.5 * (knots[idx] + knots[idx + p + 1]);
    ctl[idx].y += 1e-3; // height channel only: azimuth untouched
    let corrupted =
        NurbsCurve2::new(refined.knots().clone(), ctl, refined.weights().to_vec()).unwrap();

    // (a) every scheduled sample is EXACTLY untouched (basis locality);
    for i in 0..9u32 {
        let t = t0 + (t1 - t0) * (f64::from(i) / 8.0);
        let d = corrupted.eval(t) - refined.eval(t);
        assert!(
            d.norm() == 0.0,
            "sample {i} moved by {} — the corruption leaked onto the schedule",
            d.norm()
        );
    }
    // (b) and the corruption is real between them.
    let drift = (corrupted.eval(support_mid) - refined.eval(support_mid)).norm();
    assert!(
        drift > 1e-4,
        "the between-samples displacement is substantial, got {drift}"
    );

    // (c) the full C2 certificate admits it...
    let img = Arc::new(corrupted);
    let window = Pcurve::Fitted(Arc::clone(&img)).chart_box(t0, t1);
    let cache = PcurveCache::certify_fitted(
        Arc::clone(&img),
        t0,
        t1,
        &Curve3::Nurbs(Arc::clone(&built.carrier)),
        &built.cylinder,
        Some(&built.sphere),
        window,
        band,
    )
    .expect(
        "OnLocusHull certifies the CARRIER's incidence, not the image between \
         samples — the documented gap; if this starts refusing, the gap closed",
    );

    // (d) ...and so does the at-rest pass.
    built.body.attach_pcurve(built.he_plus, cache);
    let findings = validate_pcurves(&built.body, band);
    assert!(
        findings.is_empty(),
        "the at-rest pass re-derives the same certificate and admits the same \
         gap: {findings:?}"
    );
}

/// Species 3: a sub-interval cache. `recertify` re-derives over the
/// cache's own stored `(t0, t1)` and no check compares them to the
/// edge's span — but the at-rest pass still REJECTS the cache, through
/// the loop-continuity walk: a half-edge whose image stops at the
/// half-way point cannot meet its neighbour in the chart. The net that
/// catches this species is loop continuity, not param coverage; pinned
/// so that if the walk ever loosens, the species resurfaces here.
#[test]
fn a_sub_interval_cache_is_caught_by_loop_continuity() {
    let Some(mut built) = fixture::build::<f64>() else {
        stood_down(
            "M6-2 species 3: sub-interval cache",
            "the cylinder×sphere fixture stood down on the SSI door's typed \
             FitSampleBudget refusal at this ε, so THIS RUN ASSERTS NOTHING about \
             species 3: not that a sub-interval cache is caught, and not that loop \
             continuity rather than a param-coverage comparison is the net that \
             catches it",
        );
        return;
    };
    let band = Band::linear(Tol::witness()).unwrap();
    let (t0, t1) = built.image.domain();
    let tm = t0 + (t1 - t0) * 0.5;
    let window = Pcurve::Fitted(Arc::clone(&built.image)).chart_box(t0, t1);
    let cache = PcurveCache::certify_fitted(
        Arc::clone(&built.image),
        t0,
        tm,
        &Curve3::Nurbs(Arc::clone(&built.carrier)),
        &built.cylinder,
        Some(&built.sphere),
        window,
        band,
    )
    .expect("the half-interval certifies honestly");
    built.body.attach_pcurve(built.he_plus, cache);
    let findings = validate_pcurves(&built.body, band);
    assert!(
        findings
            .iter()
            .any(|f| matches!(f, PcurveMintError::LoopDiscontinuity { .. })),
        "the loop-continuity walk is what catches a sub-interval cache: {findings:?}"
    );
}
