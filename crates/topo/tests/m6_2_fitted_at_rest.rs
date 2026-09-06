//! **M6-2 acceptance: a fitted rung-3 pcurve cache in a body AT REST**
//! — the row M5 PR 9's spec asked for, the M5 exit walk carried as row
//! 2, and the SSI generic-`T` lift's own acceptance obligation.
//!
//! Until this unit the invariant "every fitted cache at rest carries
//! the full C2 certificate — hull sup-norm plus uniqueness tube" was
//! true only VACUOUSLY: `Pcurve::Fitted` did not exist, because its
//! certificate could not be derived anywhere but `f64` (M5-LOG PR 9c
//! deviation 2). Both halves moved in M6-2, so the row is now a real
//! one, and it is stated at both scalars the lift was for:
//!
//! - **`f64`** (this file's outer rows) — the cylinder×sphere fixture's
//!   small loop of the kernel's own traced-and-fitted branch, restricted
//!   to an edge carrier, certified into a body, and
//!   its cylinder-chart image stored as a `Pcurve::Fitted` cache whose
//!   certificate is RE-DERIVED at rest by the tier-3 pcurve pass;
//! - **`Interval`** (the `certified` module) — the same body at the
//!   interval scalar. This is the non-negotiable half: it is the
//!   evidence that the enclosure/certification stack actually left
//!   `f64`, and it is asserted enclosure-style (bracketing), never by
//!   equality.
//!
//! **ε posture.** No ε literal appears here. Every margin is compared
//! against the run's own resolved band, and the fixture stands down
//! through the SSI door's own typed `FitSampleBudget` refusal when a
//! tight ε demands more march samples than the named budget allows —
//! the `m5_pr7_ssi.rs` discipline, so the 1e-6 / default / 1e-12 rows
//! of the hosted matrix each state something true.
//!
//! **What this row does NOT do**, so nobody reads more into it: it does
//! not wire the cyl×sphere JOIN lane (`run_azimuth_window` has no
//! window analog for a fitted chord — banked past M6). The edge is
//! built through the public certification doors, exactly as
//! `m5_pr7_split_meter.rs`'s rung-3 scaffold is.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use geom_brep::{EnvelopeStatement, Pcurve};
use geom_core::Band;
use geom_core::Tol;
use test_utils::vacuity;

/// The full at-rest run at `f64`: build, validate, and read the
/// certificate the tier-3 pass re-derived.
#[test]
fn a_rung3_edge_at_rest_carries_a_fitted_pcurve_with_the_full_c2_certificate() {
    let Some(built) = fixture::build::<f64>() else {
        vacuity::stood_down(
            &format!(
                "M6-2 rung-3 edge at rest, f64 lane, eps = {:e}",
                Tol::witness().get().eps
            ),
            "the cylinder×sphere fixture stood down on the SSI door's typed \
             FitSampleBudget refusal at this ε — the budget row pins that outcome. \
             THIS RUN ASSERTS NOTHING about the at-rest body: neither that the cache it \
             carries is fitted, nor that the tier-3 pass re-derives the full C2 \
             certificate over it",
        );
        return;
    };
    let band = Band::linear(Tol::witness()).unwrap();

    // 1. The cache at rest IS fitted — the variant reached a body.
    for he in [built.he_plus, built.he_minus] {
        let cache = built
            .body
            .pcurve(he)
            .expect("both half-edges carry a cache");
        assert!(
            matches!(cache.pcurve(), Pcurve::Fitted(_)),
            "the stored chart image is the fitted (rung-3) variant"
        );
        let cert = cache.certificate();
        // 2. The full C2 certificate is present — hull sup-norm AND
        //    uniqueness tube. A schedule-max-only cache is exactly what
        //    this row exists to forbid.
        let ssi = cert
            .ssi
            .expect("a fitted cache carries the SSI certificate");
        assert!(
            ssi.tube_boxes > 0,
            "the uniqueness tube proved one-arc-ness over a real box chain"
        );
        assert!(
            ssi.tube_radius > 0.0 && ssi.tube_transversality > 0.0,
            "the tube has a certified radius and a definitely-positive margin"
        );
        assert_eq!(
            cert.statement,
            EnvelopeStatement::OnLocusHull,
            "an analytic chart's fitted envelope is the on-locus hull bound"
        );
        assert_eq!(
            cert.envelope, ssi.hull_sup,
            "the envelope IS the hull bound"
        );
        // 3. Both statements are inside the run's band, and they are
        //    SEPARATE numbers (a sampled max is not a sup bound).
        assert!(cert.max_residual <= band.zero(), "sampled max within ε");
        assert!(cert.envelope <= band.zero(), "certified sup bound within ε");
    }

    // 4. The tier-3 pcurve pass RE-DERIVES it and finds nothing.
    let findings = topo::pcurves::validate_pcurves(&built.body, band);
    assert!(
        findings.is_empty(),
        "the at-rest pcurve pass re-derives the whole certificate: {findings:?}"
    );
}

// **RETIRED (2026-08-13 test-time audit):
// `a_corrupted_fitted_cache_fails_the_at_rest_pass`.** It built this
// file's cyl×sphere fixture at `f64`, attached `fixture::foreign_cache`
// to `he_plus`, ran `validate_pcurves`, and asserted
// `!findings.is_empty()` — the re-derivation is not a formality.
//
// The gate that owns that claim now is
// `review_m6_2_probes::the_foreign_arc_cache_fails_on_the_map_residual\
// _against_the_edges_carrier`: SAME fixture (`fixture::build::<f64>()`),
// SAME corruption (`fixture::foreign_cache`), SAME half-edge, SAME
// `validate_pcurves` call — and instead of "some finding", it requires
// the finding to be `PcurveMintError::Certify` on `he_plus` carrying
// `PcurveCertifyError::ResidualExceeded { check: PcurveCheck::\
// MapResidual }`. A non-empty findings list is implied by that match
// existing, so the retired row's assertion is a strict weakening of the
// successor's. Nothing is lost.

/// The `Dual` lane's refusing side, executed rather than assumed: a
/// fitted cache cannot be certified by a scalar that may not certify
/// (D1, 2026-08-19 — a dual now carries a bracket and still may not
/// reach the C9 ring), and it says so.
///
/// **The asserted substring changed with D1, and it had to.** This row
/// used to require the message to contain `"bracket"`, which was the
/// reason the refusal gave: *"this scalar carries no bracket to reach the
/// ring with"*. That sentence is now false — a dual carries the value
/// channel's bracket — so the message says the true reason instead, and
/// this row asserts the true reason. It is the assertion that keeps the
/// user-facing string honest, so it is the one place that must move when
/// the string does: `msg.contains("bracket")` passing again would mean
/// someone reintroduced the stale claim.
#[test]
fn the_dual_lane_refuses_a_fitted_cache_typed() {
    let Some(built) = fixture::build::<f64>() else {
        vacuity::stood_down(
            &format!(
                "M6-2 dual-lane refusal, eps = {:e}",
                Tol::witness().get().eps
            ),
            "the cylinder×sphere fixture stood down on the SSI door's typed \
             FitSampleBudget refusal at this ε, so THIS RUN ASSERTS NOTHING about the \
             dual lane: neither that it refuses a fitted cache typed, nor that the \
             refusal names the lane and its post-D1 reason instead of re-asserting the \
             premise D1 invalidated",
        );
        return;
    };
    let err = fixture::certify_at_dual(&built);
    let msg = format!("{err}");
    assert!(
        msg.contains("dual") && msg.contains("may not certify"),
        "the refusal names the lane and the true reason it has none: {msg}"
    );
    assert!(
        !msg.contains("no bracket") && !msg.contains("carries no bracket"),
        "the refusal must not re-assert the premise D1 invalidated: {msg}"
    );
}

/// ε is never a literal here; this row states what the file relies on.
#[test]
fn the_band_is_the_runs_own() {
    let band = Band::linear(Tol::witness()).unwrap();
    assert_eq!(band.zero(), Tol::witness().get().eps);
}

// ==================================================================
// The interval lane — the evidence that the lift happened
// ==================================================================

/// **Loud skip.** Without `--features interval` this file contributes
/// no certified coverage, and a lane that silently lost its interval
/// rows must stay visible in the battery log.
#[cfg(not(feature = "interval"))]
#[test]
fn interval_lane_skipped_no_certified_coverage_here() {
    println!(
        "SKIPPED (no --features interval): m6_2_fitted_at_rest.rs contributes NO \
         certified coverage in this run — the fitted-cache-at-rest certificate \
         derived AT THE INTERVAL SCALAR is the row that shows the SSI lift happened."
    );
}

#[cfg(feature = "interval")]
mod certified {
    use super::fixture;
    use geom_brep::{EnvelopeStatement, Pcurve};
    use geom_core::Tol;
    use geom_core::{Band, Bounds, Interval};
    use test_utils::vacuity;

    /// The same body, at the interval scalar: the C2 certificate is
    /// DERIVED there, every claim is a bracketing claim — **and it
    /// DOMINATES the `f64` lane's.**
    ///
    /// # One interval build, one f64 build, every interval-lane claim
    ///
    /// The dominance claim was its own row (`the_interval_bounds_\
    /// dominate_the_f64_ones`) until the test-cost audit. It built the
    /// SAME two fixtures this row builds — `fixture::build::<Interval>()`
    /// and `fixture::build::<f64>()`, both restricting the first quarter
    /// of the one traced cylinder×sphere locus — and read the SAME
    /// certificate off `he_plus`. Under nextest's process-per-test
    /// isolation the `OnceLock` in `fixture/mod.rs` shares nothing
    /// between test processes, so the split paid the trace twice over
    /// and the `f64` assembly twice; the `f64` build folded in here is
    /// the one the `OnceLock` was written for.
    ///
    /// What the split bought and a merged row cannot is failure
    /// ISOLATION: a broken interval derivation and a broken cross-scalar
    /// dominance now surface under one test id. So every assertion below
    /// NAMES its property — `INTERVAL`, `ENCLOSURE`, `TUBE`, `AT-REST`,
    /// `DOMINANCE`, `THIN` — and the message alone says which one broke.
    /// Keep that discipline when adding assertions here.
    #[test]
    fn the_fitted_certificate_is_derived_at_the_interval_scalar_and_dominates_f64() {
        let Some(built) = fixture::build::<Interval>() else {
            vacuity::stood_down(
                &format!(
                    "M6-2 fitted certificate at the interval scalar, eps = {:e}",
                    Tol::witness().get().eps
                ),
                "the cylinder×sphere fixture stood down on the SSI door's typed \
                 FitSampleBudget refusal at this ε — THIS RUN CONTRIBUTES NO INTERVAL-LANE \
                 COVERAGE: neither the derived-at-Interval certificate nor its dominance \
                 over the f64 lane was asserted",
            );
            return;
        };
        let band = Band::linear(Tol::witness()).unwrap();
        let cache = built
            .body
            .pcurve(built.he_plus)
            .expect("the interval body carries the cache");
        assert!(
            matches!(cache.pcurve(), Pcurve::Fitted(_)),
            "INTERVAL: the stored chart image is the fitted (rung-3) variant"
        );
        let cert = cache.certificate();
        let ssi = cert.ssi.expect(
            "INTERVAL: the interval lane derives the SSI certificate — it is not an f64 shadow",
        );
        assert_eq!(
            cert.statement,
            EnvelopeStatement::OnLocusHull,
            "INTERVAL: an analytic chart's fitted envelope is the on-locus hull bound"
        );

        // Enclosure-style, never equality: every certified quantity is
        // an enclosure whose UPPER end is what the band admitted, and
        // whose bracket contains a non-negative residual.
        for (what, v) in [
            ("sampled max", cert.max_residual),
            ("envelope", cert.envelope),
            ("on-locus max", ssi.on_locus_max),
            ("hull sup", ssi.hull_sup),
        ] {
            assert!(
                v.lo() <= v.hi(),
                "ENCLOSURE {what}: a well-formed enclosure"
            );
            assert!(
                v.lo() >= 0.0,
                "ENCLOSURE {what}: a magnitude encloses no negatives"
            );
            assert!(
                v.hi() <= band.zero(),
                "ENCLOSURE {what}: the whole enclosure is within ε"
            );
        }
        // The tube's margin is definitely positive at the interval
        // scalar — its LOWER end clears zero, which is the one-arc
        // proof surviving the widening.
        assert!(
            ssi.tube_transversality.lo() > 0.0,
            "TUBE: the transversality margin's enclosure excludes zero"
        );
        assert!(ssi.tube_boxes > 0, "TUBE: a real box chain");

        // And the at-rest pass re-derives all of it at Interval.
        let findings = topo::pcurves::validate_pcurves(&built.body, band);
        assert!(findings.is_empty(), "AT-REST: {findings:?}");

        // ---- The cross-scalar half -----------------------------------
        //
        // The interval certificate is not merely present but HONEST: its
        // bounds dominate the `f64` lane's, because the same computation
        // at the interval scalar can only widen.
        //
        // The quantity compared is deliberately `on_locus_max`, limb 1's
        // **evaluated** residual, and not `envelope`: the envelope is the
        // C9 ring's own `f64` hull bound lifted through `from_f64`, so it
        // is THIN at both scalars and a comparison of it would pass by
        // exact equality — a row with no teeth. `on_locus_max` is computed
        // by evaluating `implicit_residual` at the scalar, so the interval
        // lane genuinely widens it, and dominance there is a real claim
        // about the lift.
        //
        // INVARIANT: the `f64` build here shares the memoized trace with
        // the interval one above, which is what makes this a claim about
        // the LIFT and not about two independent traces agreeing — see
        // `fixture/mod.rs`'s `branch_or_budget`. A row that ever wants
        // two independent traces must call `trace_branch` and say why.
        let fl = fixture::build::<f64>()
            .expect("DOMINANCE: the f64 lane shares the memoized trace the interval lane used");
        let fc = fl.body.pcurve(fl.he_plus).unwrap().certificate();
        let f_ssi = fc.ssi.expect("DOMINANCE: f64 certificate");
        assert!(
            ssi.on_locus_max.hi() >= f_ssi.on_locus_max,
            "DOMINANCE: the interval on-locus residual's upper end dominates the f64 one \
             ({} vs {})",
            ssi.on_locus_max.hi(),
            f_ssi.on_locus_max
        );
        // The envelope is deliberately NOT compared across scalars.
        // It is `T::from_f64` of a C9-ring bound, so it is thin at both
        // — but "thin" is not "the same number": the tube ladder's
        // extent and lever arm are evaluated at `T`
        // (`carrier_diameter`), so the interval lane can select a
        // different rung and land on a different certificate
        // STRUCTURE, and neither direction of a cross-scalar
        // comparison of the resulting bound is guaranteed. The hosted
        // ε = 1e-6 row executed exactly that: the two lanes' envelopes
        // are both thin and not equal. What IS sound is the dominance
        // asserted above, on the quantity that is genuinely evaluated
        // at the scalar.
        assert_eq!(
            cert.envelope.lo(),
            cert.envelope.hi(),
            "THIN: the ring-derived bound is thin at the interval scalar"
        );
    }
}
