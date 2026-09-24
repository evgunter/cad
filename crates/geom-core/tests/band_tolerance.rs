//! The tolerance-coupled band constructors ([`Band::linear`],
//! [`Band::linear_at`] and [`Band::angular_at`]) against the run's
//! global [`Tolerance`], and both reachable arms of their `# Errors`.
//!
//! The funnel-test discipline (see `src/tolerance.rs`'s test module):
//! the global tolerance commits exactly once per process, the lib test
//! binary's single global-touching test owns that binary's commitment,
//! and `tests/tolerance_init.rs` owns the explicit-`init` path. Exactly
//! ONE test here reads the committed global in-process
//! — `bands_track_the_global_tolerance` — because a second would race
//! it for first touch.
//!
//! **The arm rows commit a PATHOLOGICAL tolerance, so each takes its own
//! process.** They are `#[ignore]`d probes re-exec'd by
//! `both_band_error_arms_are_reachable`, the `ambiguity_k_env.rs`
//! pattern: the spawner touches no global itself, and a probe is inert
//! in an ordinary run. Each probe commits its pair through the REAL
//! [`Tolerance::init`] — which runs the same private `validate` the env
//! path runs — so the premise *"the run's own validator admits this"*
//! is the door's answer and not a restatement of its conditions.
//!
//! Deliberately NO explicit `init` here: the global self-initializes from
//! the environment on the first `Band::linear(Tol::witness())` call, so the multi-ε CI
//! matrix (`CAD_TOLERANCE_EPS`) genuinely exercises *different bands*
//! through this test — every assertion is written relative to the run's
//! ε **and its K**, never to a fixed value of either. Both halves of the
//! band are configurable (`CAD_AMBIGUITY_K` admits any K > 1), so a
//! multiplier written as a literal would pin this file to the default
//! K = 10 while still reading as ε-relative.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Tol;
use geom_core::tolerance::{DEFAULT_K, Tolerance};
use geom_core::{Band, BandError, BandField, Decide, MarginDiag, Sign};

#[test]
fn bands_track_the_global_tolerance() {
    let band =
        Band::linear(Tol::witness()).expect("the run's eps is sane, so K*eps cannot overflow");
    let tolerance = Tol::witness().get();

    // linear() is exactly (eps, K*eps) of the committed tolerance...
    assert_eq!(band.zero(), tolerance.eps);
    assert_eq!(band.escalate(), tolerance.k * tolerance.eps);

    // ...and angular_at derives its threshold per lever arm as eps/r —
    // there is no global angular tolerance (D4 ¶1, revised 2026-07-16).
    // At the unit lever arm (r = 1) the derived angle is exactly eps, so
    // the angular band coincides with the linear one.
    let unit = Band::angular_at(Tol::witness(), 1.0)
        .expect("the run's eps is sane, so eps/1 forms a band");
    assert_eq!(unit.zero(), tolerance.eps);
    assert_eq!(unit.escalate(), tolerance.k * tolerance.eps);
    assert_eq!(unit, band);

    // A curvature-style lever arm r = 1/kappa_rel scales the threshold:
    // zero = eps/r, escalate = K*(eps/r).
    let kappa_rel = 4.0;
    let arm = 1.0 / kappa_rel;
    let curved = Band::angular_at(Tol::witness(), arm).expect("eps/arm is a sane finite threshold");
    assert_eq!(curved.zero(), tolerance.eps / arm);
    assert_eq!(curved.escalate(), tolerance.k * (tolerance.eps / arm));

    // Overflow residue: a lever arm tiny enough that eps/arm is finite but
    // K*(eps/arm) overflows surfaces as the existing InvalidValue-on-
    // escalate error (routed through Band::new), not a silently bad band.
    // Reaching it needs a zero threshold in (MAX/K, MAX], which depends on
    // the run's K: the target below is MAX*(1+K)/(2K), a fraction in
    // (1/2, 1] that is under MAX for every K > 1 and over MAX/K for every
    // K > 1, so the residue stays reachable at whatever K the session
    // committed instead of only at a large one. (The factor is formed
    // before scaling MAX: MAX*(1+K) would itself overflow.) The arm is
    // necessarily subnormal — eps/arm has to land near MAX — so at the
    // tightest matrix eps the construction runs out of significand for a
    // K within ~1e-4 of 1; a band that narrow is degenerate anyway.
    let overflow_zero = f64::MAX * ((1.0 + tolerance.k) / (2.0 * tolerance.k));
    let tiny_arm = tolerance.eps / overflow_zero;
    assert_eq!(
        Band::angular_at(Tol::witness(), tiny_arm),
        Err(BandError::InvalidValue {
            field: BandField::Escalate,
            value: f64::INFINITY,
        })
    );

    // Classification tracks the run's band: margins placed relative to
    // eps AND K land in the same region at every matrix value. Every
    // multiplier is derived from the run's own K rather than assuming
    // one, so this row holds at each K a session can commit (K > 1) and
    // not only at the default 10; both margins are safely interior to
    // their regions under fp rounding.
    let eps = tolerance.eps;
    assert_eq!((0.5 * eps).sign_within(band), Ok(Sign::Zero));
    assert_eq!((-0.5 * eps).sign_within(band), Ok(Sign::Zero));
    let definite = 2.0 * tolerance.k * eps;
    assert_eq!(definite.sign_within(band), Ok(Sign::Positive));
    assert_eq!((-definite).sign_within(band), Ok(Sign::Negative));
    let mid = (1.0 + tolerance.k) / 2.0 * eps; // strictly inside (eps, K*eps)
    let sliver = mid
        .sign_within(band)
        .expect_err("the band midpoint lies inside the ambiguity band");
    assert_eq!(sliver.margin, MarginDiag::Value(mid));
    assert_eq!(sliver.band, band);
}

// ---------------------------------------------------------------------
// Both reachable arms of the constructors' `# Errors`, through the real
// doors, at tolerances the real validator admits.
// ---------------------------------------------------------------------

/// The least K [`Tolerance`] admits: the first double above 1.
fn least_k() -> f64 {
    1.0f64.next_up()
}

/// ε = 2⁻¹⁰²³ = 2⁵¹·2⁻¹⁰⁷⁴ — the LARGEST ε that still collapses, and it
/// collapses only on the tie: the exact product is n + ½ with n even, so
/// round-half-to-even lands back on n. One ulp above this ε nothing
/// collapses at any admitted K.
fn collapse_boundary_eps() -> f64 {
    f64::from_bits(1u64 << 51)
}

/// Commits `tolerance` through the door that validates it, and hands
/// back the witness. Panics if the validator refuses, which is the
/// premise these probes rest on.
fn commit(eps: f64, k: f64) -> Tol {
    Tolerance::init(Tolerance { eps, k })
        .expect("the run's own validator admits this pair — that is the premise");
    let tol = Tol::witness();
    assert_eq!((tol.eps(), tol.k()), (eps, k), "the committed pair is ours");
    tol
}

/// PROBE (own process). The COLLAPSE arm on the door the sentence is
/// on: a committed ε of 2⁻¹⁰²³ with the least admitted K, and
/// `Band::linear` itself returns `Empty`.
#[test]
#[ignore]
fn probe_collapse_arm_on_linear() {
    let eps = collapse_boundary_eps();
    let tol = commit(eps, least_k());
    assert_eq!(
        Band::linear(tol),
        Err(BandError::Empty {
            zero: eps,
            escalate: eps,
        }),
        "K·ε rounds back onto ε at the boundary ε, so linear has no band"
    );
    println!("PROBE collapse-on-linear OK at eps={eps:e}");
}

/// PROBE (own process). The OVERFLOW arm on the same door, at the
/// largest ε the validator admits and the ratified default K.
#[test]
#[ignore]
fn probe_overflow_arm_on_linear() {
    let tol = commit(f64::MAX, DEFAULT_K);
    assert_eq!(
        Band::linear(tol),
        Err(BandError::InvalidValue {
            field: BandField::Escalate,
            value: f64::INFINITY,
        }),
        "K·ε overflows at ε = f64::MAX, so escalate is not a threshold"
    );
    println!("PROBE overflow-on-linear OK");
}

/// PROBE (own process). The boundary and the region, read through
/// `Band::linear_at` at an ORDINARY committed ε with the least admitted
/// K — so the run itself is sane and only the named scale is not.
#[test]
#[ignore]
fn probe_collapse_region_through_linear_at() {
    let tol = commit(1e-9, least_k());
    let boundary = collapse_boundary_eps();
    let above = f64::from_bits((1u64 << 51) + 1);
    let min_subnormal = f64::from_bits(1);

    assert!(Band::linear(tol).is_ok(), "the RUN's own ε forms a band");
    for (eps, why) in [
        (min_subnormal, "the smallest ε there is"),
        (boundary, "the largest ε that collapses, on the tie"),
    ] {
        assert_eq!(
            Band::linear_at(tol, eps),
            Err(BandError::Empty {
                zero: eps,
                escalate: eps,
            }),
            "{why}: {eps:e}"
        );
    }
    assert!(
        Band::linear_at(tol, above).is_ok(),
        "one ulp above the boundary nothing collapses: {above:e}"
    );

    // NO NORMAL ε COLLAPSES — every binade, at both ends of its
    // significand. This is the half of the claim that says the hazard
    // needs a subnormal ε, and it is checked rather than asserted.
    for e in -1022..=1023 {
        let bottom = 2.0f64.powi(e);
        let top = bottom * 1.999_999_999_999_999_8;
        for eps in [bottom, top] {
            assert!(
                !matches!(Band::linear_at(tol, eps), Err(BandError::Empty { .. })),
                "a NORMAL ε collapsed at the least admitted K: 2^{e} → {eps:e}"
            );
        }
    }

    // The overflow arm through the same door, and the region's other
    // knob: at the ratified default K nothing collapses at all.
    assert_eq!(
        Band::linear_at(tol, f64::MAX),
        Err(BandError::InvalidValue {
            field: BandField::Escalate,
            value: f64::INFINITY,
        })
    );
    println!("PROBE linear_at-region OK: boundary 2^-1023, no normal ε collapses");
}

/// PROBE (own process). `Band::angular_at` reaches the collapse arm at
/// an ORDINARY ε, through the LEVER ARM — a caller argument, not a run
/// setting, and the session-box extent the method recommends is the
/// documented road to a large one.
#[test]
#[ignore]
fn probe_angular_collapse_through_the_lever_arm() {
    let tol = commit(1e-9, least_k());
    let arm = 1e300;
    let theta = tol.eps() / arm;
    assert!(
        theta > 0.0 && theta < f64::MIN_POSITIVE,
        "the derived angle is a nonzero subnormal: {theta:e}"
    );
    assert_eq!(
        Band::angular_at(tol, arm),
        Err(BandError::Empty {
            zero: theta,
            escalate: theta,
        }),
        "an ordinary ε and a large arm reach Empty through angular_at"
    );
    println!("PROBE angular-collapse OK: theta={theta:e}");
}

/// PROBE (own process). `angular_at`'s THIRD residue: the derived angle
/// underflows to zero, refused on `zero` rather than on `escalate`. The
/// rule is about the pair — θ ties to 0 at `lever_arm` ≥ ε·2¹⁰⁷⁵ — and
/// the boundary is SHARP: one ulp under that arm the angle is a nonzero
/// subnormal and the band forms.
#[test]
#[ignore]
fn probe_angular_underflow_third_residue() {
    let tol = commit(1e-16, DEFAULT_K);
    let boundary_arm = tol.eps() / f64::from_bits(1) * 2.0; // ε·2¹⁰⁷⁵
    assert_eq!(
        boundary_arm, 4.048_045_066_146_212_3e307,
        "the boundary arm at ε = 1e-16, as the `# Errors` text states it"
    );
    assert_eq!(tol.eps() / boundary_arm, 0.0, "θ ties to even = 0 there");
    assert_eq!(
        Band::angular_at(tol, boundary_arm),
        Err(BandError::InvalidValue {
            field: BandField::Zero,
            value: 0.0,
        })
    );
    assert_eq!(
        Band::angular_at(tol, f64::MAX),
        Err(BandError::InvalidValue {
            field: BandField::Zero,
            value: 0.0,
        }),
        "and every larger arm with it"
    );
    let inside = f64::from_bits(boundary_arm.to_bits() - 1);
    assert!(
        tol.eps() / inside > 0.0 && Band::angular_at(tol, inside).is_ok(),
        "one ulp under the boundary arm the angle survives and the band forms"
    );
    println!("PROBE angular-underflow OK: boundary arm {boundary_arm:e}");
}

/// **Both arms of the constructors' `# Errors` are reachable from a
/// tolerance the run's own validator admits** — the claim
/// `Band::linear`'s `# Errors` makes, and the reason a refusal carries
/// the `BandError` instead of discarding it: the two ends want opposite
/// repairs (lower ε; raise ε or K).
///
/// Each row is a re-exec'd child because each commits a different
/// global. `editor-core`'s `band_refusals_name_which_band_failure_they_caught`
/// (`tests/wire_band_cause.rs`) carries the other half — that the
/// refusals which forward a `BandError` render the cause verbatim.
#[test]
fn both_band_error_arms_are_reachable() {
    for probe in [
        "probe_collapse_arm_on_linear",
        "probe_overflow_arm_on_linear",
        "probe_collapse_region_through_linear_at",
        "probe_angular_collapse_through_the_lever_arm",
        "probe_angular_underflow_third_residue",
    ] {
        spawn_probe(probe);
    }
}

/// Re-execs this binary at one `#[ignore]`d probe, in its own process so
/// the probe can commit its own global. Names the probe by MODULE PATH:
/// `tests/all.rs` aggregates every suite into one binary, so libtest
/// sees it as `<this_module>::<probe>`.
fn spawn_probe(probe: &str) {
    let exe = std::env::current_exe().expect("test exe path");
    let filter = match module_path!().split_once("::") {
        Some((_, m)) => format!("{m}::{probe}"),
        None => probe.to_string(),
    };
    let out = std::process::Command::new(&exe)
        .args([filter.as_str(), "--ignored", "--exact", "--nocapture"])
        .env_remove("CAD_TOLERANCE_EPS")
        .env_remove("CAD_AMBIGUITY_K")
        .output()
        .expect("the probe process spawns");
    assert!(
        out.status.success(),
        "{probe} failed:\n{}\n{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(
        String::from_utf8_lossy(&out.stdout).contains("1 passed"),
        "{probe} did not RUN (a filter that matches nothing also exits 0):\n{}",
        String::from_utf8_lossy(&out.stdout)
    );
}
