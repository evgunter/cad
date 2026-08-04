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
//!   small loop, refit into an edge carrier, certified into a body, and
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

mod fixture;

use geom_brep::{EnvelopeStatement, Pcurve};
use geom_core::{Band, Tolerance};

/// The full at-rest run at `f64`: build, validate, and read the
/// certificate the tier-3 pass re-derived.
#[test]
fn a_rung3_edge_at_rest_carries_a_fitted_pcurve_with_the_full_c2_certificate() {
    let Some(built) = fixture::build::<f64>() else {
        println!(
            "SKIPPED: the cylinder×sphere fixture stood down on the SSI door's typed \
             FitSampleBudget refusal at this ε — the budget row pins that outcome"
        );
        return;
    };
    let band = Band::linear().unwrap();

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

/// The re-derivation is not a formality: corrupt the stored cache's
/// chart image and the at-rest pass refuses, typed.
#[test]
fn a_corrupted_fitted_cache_fails_the_at_rest_pass() {
    let Some(mut built) = fixture::build::<f64>() else {
        println!("SKIPPED: FitSampleBudget stand-down at this ε");
        return;
    };
    let band = Band::linear().unwrap();
    let foreign = fixture::foreign_cache(&built);
    built.body.attach_pcurve(built.he_plus, foreign);
    let findings = topo::pcurves::validate_pcurves(&built.body, band);
    assert!(
        !findings.is_empty(),
        "a foreign arc's chart image must fail the re-derivation, not ride its own \
         (perfectly true) stored bound"
    );
}

/// The `Dual` lane's refusing side, executed rather than assumed: a
/// fitted cache cannot be certified where there is no bracket to reach
/// the C9 ring with, and it says so.
#[test]
fn the_dual_lane_refuses_a_fitted_cache_typed() {
    let Some(built) = fixture::build::<f64>() else {
        println!("SKIPPED: FitSampleBudget stand-down at this ε");
        return;
    };
    let err = fixture::certify_at_dual(&built);
    let msg = format!("{err}");
    assert!(
        msg.contains("dual") && msg.contains("bracket"),
        "the refusal names the lane and why it has none: {msg}"
    );
}

/// ε is never a literal here; this row states what the file relies on.
#[test]
fn the_band_is_the_runs_own() {
    let band = Band::linear().unwrap();
    assert_eq!(band.zero(), Tolerance::get().eps);
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
    use geom_core::{Band, Bounds, Interval};

    /// The same body, at the interval scalar: the C2 certificate is
    /// DERIVED there, and every claim is a bracketing claim.
    #[test]
    fn the_fitted_certificate_is_derived_at_the_interval_scalar() {
        let Some(built) = fixture::build::<Interval>() else {
            println!("SKIPPED: FitSampleBudget stand-down at this ε");
            return;
        };
        let band = Band::linear().unwrap();
        let cache = built
            .body
            .pcurve(built.he_plus)
            .expect("the interval body carries the cache");
        assert!(matches!(cache.pcurve(), Pcurve::Fitted(_)));
        let cert = cache.certificate();
        let ssi = cert
            .ssi
            .expect("the interval lane derives the SSI certificate — it is not an f64 shadow");
        assert_eq!(cert.statement, EnvelopeStatement::OnLocusHull);

        // Enclosure-style, never equality: every certified quantity is
        // an enclosure whose UPPER end is what the band admitted, and
        // whose bracket contains a non-negative residual.
        for (what, v) in [
            ("sampled max", cert.max_residual),
            ("envelope", cert.envelope),
            ("on-locus max", ssi.on_locus_max),
            ("hull sup", ssi.hull_sup),
        ] {
            assert!(v.lo() <= v.hi(), "{what}: a well-formed enclosure");
            assert!(v.lo() >= 0.0, "{what}: a magnitude encloses no negatives");
            assert!(
                v.hi() <= band.zero(),
                "{what}: the whole enclosure is within ε"
            );
        }
        // The tube's margin is definitely positive at the interval
        // scalar — its LOWER end clears zero, which is the one-arc
        // proof surviving the widening.
        assert!(
            ssi.tube_transversality.lo() > 0.0,
            "the transversality margin's enclosure excludes zero"
        );
        assert!(ssi.tube_boxes > 0);

        // And the at-rest pass re-derives all of it at Interval.
        let findings = topo::pcurves::validate_pcurves(&built.body, band);
        assert!(findings.is_empty(), "{findings:?}");
    }

    /// The interval certificate is not merely present but HONEST: its
    /// bounds dominate the `f64` lane's, because the same computation
    /// at the interval scalar can only widen.
    ///
    /// The quantity compared is deliberately `on_locus_max`, limb 1's
    /// **evaluated** residual, and not `envelope`: the envelope is the
    /// C9 ring's own `f64` hull bound lifted through `from_f64`, so it
    /// is THIN at both scalars and a comparison of it would pass by
    /// exact equality — a row with no teeth. `on_locus_max` is computed
    /// by evaluating `implicit_residual` at the scalar, so the interval
    /// lane genuinely widens it, and dominance there is a real claim
    /// about the lift.
    #[test]
    fn the_interval_bounds_dominate_the_f64_ones() {
        let (Some(iv), Some(fl)) = (fixture::build::<Interval>(), fixture::build::<f64>()) else {
            println!("SKIPPED: FitSampleBudget stand-down at this ε");
            return;
        };
        let ic = iv.body.pcurve(iv.he_plus).unwrap().certificate();
        let fc = fl.body.pcurve(fl.he_plus).unwrap().certificate();
        let (i_ssi, f_ssi) = (
            ic.ssi.expect("interval certificate"),
            fc.ssi.expect("f64 certificate"),
        );
        assert!(
            i_ssi.on_locus_max.hi() >= f_ssi.on_locus_max,
            "the interval on-locus residual's upper end dominates the f64 one \
             ({} vs {})",
            i_ssi.on_locus_max.hi(),
            f_ssi.on_locus_max
        );
        // The ring-derived bound is thin at both scalars — stated, so
        // the equality below is read as the C6/C9 boundary it is and
        // not as a suspiciously exact numeric coincidence.
        assert_eq!(ic.envelope.lo(), ic.envelope.hi());
        assert_eq!(ic.envelope.hi(), fc.envelope);
    }
}
