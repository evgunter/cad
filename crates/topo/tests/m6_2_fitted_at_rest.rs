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
        let tube_positive = match ssi.tube {
            geom_brep::SsiTube::Spatial { radius } => radius > 0.0,
            geom_brep::SsiTube::Chart { rung, pad_u, pad_v } => {
                rung > 0.0 && pad_u > 0.0 && pad_v > 0.0
            }
        };
        assert!(
            tube_positive && ssi.tube_transversality > 0.0,
            "the tube has a certified region and a definitely-positive margin: {:?}",
            ssi.tube
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

/// **The `Dual` lane's refusing side, executed at the dual.** The
/// fixture's own image, carrier and operand pair, lifted exactly to
/// `Dual64`, offered to the fitted-grade door
/// (`PcurveCache::certify_general`) with what the dual's policy
/// answers — no fitted door. A dual may not certify (D1,
/// 2026-08-19 — it carries a bracket and still may not reach
/// certification arithmetic, C9), so no cache comes back.
///
/// **The refusal sits at check 4, and the row pins where.** Checks 1–3
/// read no door, so at the dual they run for real — the carrier class,
/// the metered interval, the schedule of map residuals, all evaluated
/// in dual arithmetic — and the absent door is asked for only where the
/// C2 certificate would be derived. Three offers say so:
///
/// - the fixture's inputs pass checks 1–3 and refuse
///   `FittedLaneUnsupported`, naming the dual;
/// - the same inputs over a reversed interval refuse at check 2
///   (`IntervalNotForward`), as they do at `f64`;
/// - the image of a DIFFERENT arc of the same locus (`foreign_cache`'s)
///   refuses at check 3 on the map residual, as it does at `f64`.
///
/// A door refused before check 1 would answer all three with the first
/// one's refusal.
///
/// **Then the refusal's text.** The dual's check-4 refusal must name
/// the dual and say the door is held only by scalars with
/// certification rights; it must not say the scalar carries no
/// bracket, which D1 made false; and its replay list names every
/// scalar whose fitted door answers `Some` (the telemetry probe's arm
/// is `probe`-gated, so its name is not read here).
///
/// **And the other reading of `None`, through `PcurveCache::recertify`**
/// — the one door a fitted cache meets an absent door through: the
/// fixture's own `f64` cache handed no door, as a caller at a
/// certifying scalar may do. It passes checks 1–3, refuses at check 4
/// naming `f64`, and its text must not claim `f64` may not certify.
#[test]
fn the_dual_refuses_at_check_four_and_says_so() {
    use geom_brep::{ChartWindow, PcurveCache, PcurveCertifyError, PcurveCheck};
    use geom_core::{Dual64, Real};
    use topo::AtRestPolicy;
    let Some(built) = fixture::build::<f64>() else {
        vacuity::stood_down(
            &format!(
                "M6-2 dual-lane refusal at check 4, eps = {:e}",
                Tol::witness().get().eps
            ),
            "the cylinder×sphere fixture stood down on the SSI door's typed \
             FitSampleBudget refusal at this ε, so THIS RUN ASSERTS NOTHING about the \
             dual lane: neither that it runs checks 1–3 in dual arithmetic, nor that it \
             refuses at check 4, nor what the refusal's text says",
        );
        return;
    };
    let band = Band::linear(Tol::witness()).unwrap();
    let lift = Dual64::from_f64;
    let carrier = geom::Curve3::Nurbs(std::sync::Arc::new(built.carrier.map_scalar(lift)));
    let (cylinder, sphere) = (
        built.cylinder.map_scalar(lift),
        built.sphere.map_scalar(lift),
    );
    let w = built.window;
    let window = ChartWindow {
        u_min: lift(w.u_min),
        u_max: lift(w.u_max),
        v_min: lift(w.v_min),
        v_max: lift(w.v_max),
    };
    let (f0, f1) = built.carrier.domain();
    let offer = |image: &geom::NurbsCurve2<f64>, t0: f64, t1: f64| {
        PcurveCache::<Dual64>::certify_general(
            std::sync::Arc::new(image.map_scalar(lift)),
            lift(t0),
            lift(t1),
            &carrier,
            &cylinder,
            Some(&sphere),
            window,
            band,
            <Dual64 as AtRestPolicy>::fitted_lane(),
        )
        .map(|_| ())
    };

    let at_four = offer(&built.image, f0, f1);
    let Err(refused @ PcurveCertifyError::FittedLaneUnsupported { scalar: "dual" }) = at_four
    else {
        panic!(
            "CHECK 4: the fixture passes checks 1–3 at the dual and refuses where the door is \
             asked for: {at_four:?}"
        );
    };
    let at_two = offer(&built.image, f1, f0);
    assert!(
        matches!(at_two, Err(PcurveCertifyError::IntervalNotForward)),
        "CHECK 2: a reversed interval refuses at check 2 at the dual, before any door is \
         asked for: {at_two:?}"
    );
    let foreign = fixture::foreign_cache(&built);
    let geom_brep::Pcurve::Fitted(foreign_image) = foreign.pcurve() else {
        panic!("the foreign cache is fitted");
    };
    let at_three = offer(foreign_image, f0, f1);
    assert!(
        matches!(
            at_three,
            Err(PcurveCertifyError::ResidualExceeded {
                check: PcurveCheck::MapResidual,
                ..
            })
        ),
        "CHECK 3: another arc's image refuses at check 3 at the dual, before any door is \
         asked for: {at_three:?}"
    );

    // The text, through `recertify` over the fixture's own cache.
    let cache = built
        .body
        .pcurve(built.he_plus)
        .expect("the fixture's half-edge carries the fitted cache");
    let err = cache
        .recertify(
            &geom::Curve3::Nurbs(std::sync::Arc::clone(&built.carrier)),
            &built.cylinder,
            Some(&built.sphere),
            built.window,
            Band::linear(Tol::witness()).unwrap(),
            None,
        )
        .expect_err("RECERTIFY: a fitted cache with no door refuses");
    assert!(
        matches!(
            err,
            PcurveCertifyError::FittedLaneUnsupported { scalar: "f64" }
        ),
        "RECERTIFY: a withheld door's refusal is the fitted-lane one, naming the scalar the \
         check ran at: {err:?}"
    );
    let withheld = format!("{err}");
    assert!(
        withheld.contains("f64 scalar") && !withheld.contains("may not certify"),
        "TEXT: the refusal names f64 and does not deny it the right it has: {withheld}"
    );

    let msg = format!("{refused}");
    assert!(
        msg.contains("dual scalar") && msg.contains("certification rights"),
        "TEXT: the refusal names the scalar and who holds the door: {msg}"
    );
    assert!(
        !msg.contains("no bracket") && !msg.contains("carries no bracket"),
        "TEXT: the refusal must not re-assert the premise D1 invalidated: {msg}"
    );
    fn replay_list_names<T: AtRestPolicy>(msg: &str) {
        let name = T::NAME;
        assert!(
            T::fitted_lane().is_some(),
            "TEXT: {name} is checked against the replay list, so it must hold the door"
        );
        assert!(
            msg.contains(name),
            "TEXT: the refusal's replay list omits {name}, whose fitted door answers Some: {msg}"
        );
    }
    replay_list_names::<f64>(&msg);
    replay_list_names::<geom_core::interval::Interval>(&msg);
    replay_list_names::<geom_core::Sym<f64>>(&msg);
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
        // C9 certification hull bound, an `f64`, lifted through `from_f64`, so it
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
        // It is `T::from_f64` of a C9 certification bound, so it is thin at both
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
