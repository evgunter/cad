//! `nurbs_cert`'s three randomized soundness sweeps, in a file of their own.
//!
//! A `test_utils::gated_to!` marker gates a whole file's module, so a fuzz row
//! sharing a file with pinned tests drags them along: gating `nurbs_cert` as a
//! whole would skip three dozen deterministic certificate pins to buy the
//! three rows below. Splitting the file is how that granularity is bought, and
//! the helpers these rows take are `pub(crate)` in `nurbs_cert::tests` rather
//! than copied here.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/mesh/src/nurbs_cert.rs",
    "crates/mesh/src/curved.rs",
    "crates/mesh/src/tessellate.rs",
    "crates/geom-brep/src/patch_bound.rs",
    "crates/geom-core/src/interval.rs",
    "crates/geom-core/src/spline/",
    "crates/geom/src/surfaces/",
    "crates/geom/src/surfaces.rs",
];

use crate::nurbs_cert::tests::{
    Domination, fold_bound, nurbs_face_bound, sample_worst, whole_net_bound,
};
use crate::nurbs_cert::*;
use geom::NurbsSurface;
use geom_core::Point3;
use geom_core::spline::KnotVector;
use test_utils::fuzz;
use topo::FaceKey;

/// **The split selection stays inside the certified ellipse and
/// the aspect window** — the TESS-SPLIT counterpart of the meter's
/// optimizer test, asserted on the ANSWER over random bounds,
/// degenerate corners included (exact-zero directions, degenerate
/// 3-D speeds).
#[test]
fn split_steps_stay_on_the_ellipse_and_inside_the_cap() {
    let mut rng = fuzz::start("nurbs_cert::split_steps_constraints");
    fn mag(r: &mut fuzz::Rng) -> f64 {
        if r.unit() < 0.2 {
            0.0
        } else {
            10.0f64.powf(r.range(-6.0, 4.0))
        }
    }
    for _ in 0..fuzz::scaled(500) {
        let budget = 10.0f64.powf(rng.range(-6.0, -1.0));
        let b = NurbsFaceBound {
            muu: mag(&mut rng),
            muv: mag(&mut rng),
            mvv: mag(&mut rng),
            mu1: mag(&mut rng),
            mv1: mag(&mut rng),
        };
        let s = b.split_steps(budget);
        // Ellipse membership, checked at a finite box (an
        // unconstrained ∞ step realizes as the box extent).
        let ext = 10.0f64.powf(rng.range(-2.0, 2.0));
        let (hu, hv) = (s.hu.min(ext), s.hv.min(ext));
        let q = b.muu * hu.powi(2) + 2.0 * b.muv * hu * hv + b.mvv * hv.powi(2);
        assert!(
            q <= budget * (1.0 + 1e-9),
            "chosen point violates the certificate: q={q:e} > {budget:e} for {b:?} — {}",
            fuzz::replay()
        );
        // Aspect-cap membership whenever the window exists and the
        // chosen steps are finite (the affine ∞ arm has no cell
        // shape to cap).
        if b.mu1 > 0.0 && b.mv1 > 0.0 && s.hu.is_finite() && s.hv.is_finite() {
            let aspect = (s.hu * b.mu1) / (s.hv * b.mv1);
            assert!(
                ((1.0 - 1e-9) / ASPECT_CAP..=ASPECT_CAP * (1.0 + 1e-9)).contains(&aspect),
                "3-D aspect {aspect:e} escapes the cap for {b:?} — {}",
                fuzz::replay()
            );
        }
    }
}

/// R1 randomized soundness sweep: random rational patches (degrees
/// 1-3, 1-3 spans, log-uniform weights 1e-2..1e2), dense-sampled true
/// second partials vs the certified sups. This sweep is the row that
/// KILLS the one mutation the rest of the suite missed (dropping the
/// `v0*w11` term from `suv` — the recentred-value x
/// mixed-weight-derivative cross term).
///
/// The trial count rides `CAD_FUZZ_EFFORT` and the seed varies per
/// run. The 61x61 `sample_worst` grid does NOT: it is the domination
/// check itself, not a sweep dimension.
#[test]
fn r1_random_rational_soundness_sweep() {
    let mut rng = fuzz::start("nurbs_cert::r1_random_rational_soundness");
    fn mk(r: &mut fuzz::Rng, p: usize) -> KnotVector {
        // Single-span at degree 1, for the reason `cert10`'s generator
        // states: a degree-1 direction admits no interior knot, so a
        // knotted one is refused by `nurbs_face_bound` and the trial is
        // thrown away. Measured 2026-09-11 before this line: 32% of
        // this sweep's trials, every one of them a `pu == 1` or
        // `pv == 1` draw.
        let spans = if p >= 2 { 1 + r.below(2) } else { 1 };
        let mut k = vec![0.0; p + 1];
        for i in 1..spans {
            #[allow(clippy::cast_precision_loss)]
            k.push(i as f64 / spans as f64);
        }
        k.extend(vec![1.0; p + 1]);
        KnotVector::clamped(k, p).unwrap()
    }
    let mut worst = 0.0f64;
    let mut compared = 0usize;
    // TRIALS are breadth; the 61x61 `sample_worst` grid below is the
    // per-trial falsification power and is deliberately NOT reduced —
    // it IS the domination check. With a varying seed, breadth is
    // what successive runs supply for free, so the trial count is the
    // honest lever here and the grid is not.
    let trials = fuzz::scaled(60);
    for trial in 0..trials {
        let pu = 1 + rng.below(3);
        let pv = 1 + rng.below(3);
        let kv_u = mk(&mut rng, pu);
        let kv_v = mk(&mut rng, pv);
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let mut control = Vec::new();
        let mut weights = Vec::new();
        for _ in 0..nu * nv {
            control.push(Point3::new(
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
                rng.range(-2.0, 2.0),
            ));
            weights.push(10f64.powf(rng.range(-2.0, 2.0)));
        }
        let s = NurbsSurface::new(kv_u, kv_v, control, weights).unwrap();
        let Ok(b) = nurbs_face_bound(&s, FaceKey::default()) else {
            continue;
        };
        compared += 1;
        let (wuu, wuv, wvv) = sample_worst(&s, 60);
        let r = (wuu / b.muu).max(wuv / b.muv).max(wvv / b.mvv);
        worst = worst.max(r);
        let d = Domination::second_partials((wuu, wuv, wvv), &b);
        assert!(
            d.holds(),
            "UNSOUND at trial {trial} (degree {pu}x{pv}): {d} — {}",
            fuzz::replay()
        );
    }
    println!("random sweep: worst truth/bound {worst:.6} over {compared} trials");
    // The breadth this row's floor is written against is the breadth it
    // ACTUALLY ran. A discarded trial lowers `worst` silently, so the
    // adversarial floor below would be satisfied by a smaller sweep than
    // the one the trial count promises — the same silence that let
    // `cert10`'s floor drift, in the shape a max takes rather than a
    // count.
    assert_eq!(
        compared,
        trials,
        "{} of {trials} trials produced no bound, so this sweep is narrower than \
         its floor assumes — {}",
        trials - compared,
        fuzz::replay()
    );
    // COVERAGE FLOOR: the sweep must keep producing cases where the
    // bound is genuinely tight, otherwise a slack bound would pass by
    // never being challenged. Verified to hold at the shipped trial
    // count; if a run ever trips it, RAISE the count rather than
    // lowering the threshold.
    assert!(
        worst > 0.5,
        "the sweep must stay adversarial (tight cases exist): worst sampled/certified \
         {worst:.17e} is not above the floor 0.5 — {}",
        fuzz::replay()
    );
}

/// **The tighter-or-equal claim, randomized.** The two pinned
/// fixtures show the gap exists and that a smooth net closes it;
/// this row asserts the INEQUALITY over a sweep of random integral
/// nets, which is the form the claim is actually made in. Integral
/// only, because the rational arm has always been a fold and has
/// no whole-net counterpart to be tighter than.
///
/// A counterexample search: the seed varies and the count rides the
/// effort dial. Its coverage floors live in
/// [`cert10_a_pinned_draw_makes_the_gap_strict_at_high_multiplicity`].
#[test]
fn cert10_the_fold_never_exceeds_the_whole_net_hull() {
    let mut rng = fuzz::start("nurbs_cert::cert10_fold_vs_whole_net");
    let tally = cert10_sweep(&mut rng, fuzz::scaled(60));
    println!(
        "cert10 fold-vs-whole-net: {} strict of {} comparisons",
        tally.strict, tally.compared
    );
}

/// **The sweep's generator reaches what the inequality rests on** —
/// strict gaps, and interior multiplicity >= 2, where the fold's window
/// coverage is tight. An inequality nothing makes strict is a tautology,
/// and a sweep that never leaves multiplicity 1 never challenges the
/// coverage premise.
///
/// A coverage witness too long to write out, so the seed is pinned: it
/// is a fixture identifier, and the trial count is fixed rather than on
/// the effort dial so the fixture is the same at every effort. Over 1000
/// varying seeds at 60 trials the strict count ran 112..=197 (mean 153,
/// sd 14) against the floor of 60; this seed draws 173.
#[test]
fn cert10_a_pinned_draw_makes_the_gap_strict_at_high_multiplicity() {
    let mut rng = fuzz::pinned("nurbs_cert::cert10_coverage", 0x6365_7274_3130);
    let trials = 60;
    let tally = cert10_sweep(&mut rng, trials);
    println!(
        "cert10 coverage: {} strict of {} comparisons, high multiplicity drawn: {}",
        tally.strict, tally.compared, tally.saw_high_mult
    );
    assert!(
        tally.saw_high_mult,
        "the draw never reached an interior multiplicity >= 2, so the fold's window \
         coverage went unchallenged where it is tight — {}",
        fuzz::replay()
    );
    assert!(
        tally.strict > trials,
        "the draw must produce STRICT gaps: {} strict of {} comparisons — {}",
        tally.strict,
        tally.compared,
        fuzz::replay()
    );
}

struct Cert10Tally {
    strict: usize,
    compared: usize,
    saw_high_mult: bool,
}

/// `trials` random integral nets, each asserting fold <= whole-net hull
/// per channel. Every drawn net must be bounded by both readings: a
/// refused draw would shrink the sample a floor is written against, so
/// it is a failure here rather than a skip.
fn cert10_sweep(rng: &mut fuzz::Rng, trials: usize) -> Cert10Tally {
    /// Interior multiplicities are drawn up to `p - 1`, the C¹
    /// gate's ceiling — NOT left at 1. That is where the fold's
    /// coverage premise is TIGHT: a cell window is
    /// `Span::window()` / `first_derived_window()` /
    /// `derived_window(2)`, and the claim that those windows COVER
    /// the derivative nets is what makes `max over cells == whole
    /// net` per channel. Raising a multiplicity shortens the
    /// derivative net and shifts every window; at `p - 1` the
    /// second-derivative net is at its shortest that
    /// `check_direction` still admits, which is exactly the case a
    /// multiplicity-1 generator never reaches.
    fn mk(r: &mut fuzz::Rng, p: usize) -> KnotVector {
        // A degree-1 direction is drawn single-span: it admits no
        // interior knot at all (`check_direction`'s Degree1Crease).
        let spans = if p >= 2 { 1 + r.below(4) } else { 1 };
        let mut k = vec![0.0; p + 1];
        for i in 1..spans {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f64 / spans as f64;
            let m = 1 + r.below(p - 1); // 1 ..= p - 1; p >= 2 here
            for _ in 0..m {
                k.push(t);
            }
        }
        k.extend(vec![1.0; p + 1]);
        KnotVector::clamped(k, p).unwrap()
    }
    let mut tally = Cert10Tally {
        strict: 0,
        compared: 0,
        saw_high_mult: false,
    };
    for trial in 0..trials {
        let (pu, pv) = (1 + rng.below(3), 1 + rng.below(3));
        let kv_u = mk(rng, pu);
        let kv_v = mk(rng, pv);
        tally.saw_high_mult |= kv_u.interior_knots().any(|(_, m)| m >= 2)
            || kv_v.interior_knots().any(|(_, m)| m >= 2);
        let (nu, nv) = (kv_u.control_count(), kv_v.control_count());
        let control: Vec<Point3<f64>> = (0..nu * nv)
            .map(|_| {
                Point3::new(
                    rng.range(-4.0, 4.0),
                    rng.range(-4.0, 4.0),
                    rng.range(-4.0, 4.0),
                )
            })
            .collect();
        let w = vec![1.0; control.len()];
        let s = NurbsSurface::new(kv_u, kv_v, control, w).unwrap_or_else(|e| {
            panic!(
                "trial {trial} (degree {pu}x{pv}): the generator drew a net \
                 NurbsSurface::new refuses: {e:?} — {}",
                fuzz::replay()
            )
        });
        let (Some(whole), Some(fold)) = (whole_net_bound(&s), fold_bound(&s)) else {
            panic!(
                "trial {trial} (degree {pu}x{pv}): `whole_net_bound` or `fold_bound` \
                 refused a drawn net — {}",
                fuzz::replay()
            );
        };
        for (what, f, w) in [
            ("muu", fold.muu, whole.muu),
            ("muv", fold.muv, whole.muv),
            ("mvv", fold.mvv, whole.mvv),
            ("mu1", fold.mu1, whole.mu1),
            ("mv1", fold.mv1, whole.mv1),
        ] {
            tally.compared += 1;
            assert!(
                f <= w,
                "trial {trial}: the fold EXCEEDED the whole-net hull on {what}: \
                 {f:.17e} > {w:.17e} — {}",
                fuzz::replay()
            );
            if f < w {
                tally.strict += 1;
            }
        }
    }
    tally
}
