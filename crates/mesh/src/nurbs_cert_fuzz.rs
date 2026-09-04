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
    "crates/geom-core/src/ring_interval.rs",
    "crates/geom-core/src/spline/",
    "crates/geom/src/surfaces/",
    "crates/geom/src/surfaces.rs",
];

use crate::nurbs_cert::tests::{fold_bound, nurbs_face_bound, sample_worst, whole_net_bound};
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
        let delta_s = 10.0f64.powf(rng.range(-6.0, -1.0));
        let b = NurbsFaceBound {
            muu: mag(&mut rng),
            muv: mag(&mut rng),
            mvv: mag(&mut rng),
            mu1: mag(&mut rng),
            mv1: mag(&mut rng),
        };
        let s = b.split_steps(delta_s);
        // Ellipse membership, checked at a finite box (an
        // unconstrained ∞ step realizes as the box extent).
        let ext = 10.0f64.powf(rng.range(-2.0, 2.0));
        let (hu, hv) = (s.hu.min(ext), s.hv.min(ext));
        let q = b.muu * hu.powi(2) + 2.0 * b.muv * hu * hv + b.mvv * hv.powi(2);
        assert!(
            q <= delta_s * (1.0 + 1e-9),
            "chosen point violates the certificate: q={q:e} > {delta_s:e} for {b:?} — {}",
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
        let spans = 1 + r.below(2);
        let mut k = vec![0.0; p + 1];
        for i in 1..spans {
            #[allow(clippy::cast_precision_loss)]
            k.push(i as f64 / spans as f64);
        }
        k.extend(vec![1.0; p + 1]);
        KnotVector::clamped(k, p).unwrap()
    }
    let mut worst = 0.0f64;
    // TRIALS are breadth; the 61x61 `sample_worst` grid below is the
    // per-trial falsification power and is deliberately NOT reduced —
    // it IS the domination check. With a varying seed, breadth is
    // what successive runs supply for free, so the trial count is the
    // honest lever here and the grid is not.
    for trial in 0..fuzz::scaled(60) {
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
        let (wuu, wuv, wvv) = sample_worst(&s, 60);
        let r = (wuu / b.muu).max(wuv / b.muv).max(wvv / b.mvv);
        worst = worst.max(r);
        assert!(
            wuv <= b.muv && wuu <= b.muu && wvv <= b.mvv,
            "UNSOUND at trial {trial}: ({wuu:.3e},{wuv:.3e},{wvv:.3e}) vs \
             ({:.3e},{:.3e},{:.3e}) — {}",
            b.muu,
            b.muv,
            b.mvv,
            fuzz::replay()
        );
    }
    println!("random sweep: worst truth/bound {worst:.6}");
    // COVERAGE FLOOR: the sweep must keep producing cases where the
    // bound is genuinely tight, otherwise a slack bound would pass by
    // never being challenged. Verified to hold at the shipped trial
    // count; if a run ever trips it, RAISE the count rather than
    // lowering the threshold.
    assert!(
        worst > 0.5,
        "the sweep must stay adversarial (tight cases exist): worst \
         {worst:.6} — {}",
        fuzz::replay()
    );
}

/// **The tighter-or-equal claim, randomized.** The two pinned
/// fixtures show the gap exists and that a smooth net closes it;
/// this row asserts the INEQUALITY over a sweep of random integral
/// nets, which is the form the claim is actually made in. Integral
/// only, because the rational arm has always been a fold and has
/// no whole-net counterpart to be tighter than.
#[test]
fn cert10_the_fold_never_exceeds_the_whole_net_hull() {
    let mut rng = fuzz::start("nurbs_cert::cert10_fold_vs_whole_net");
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
        let spans = 1 + r.below(4);
        let mut k = vec![0.0; p + 1];
        for i in 1..spans {
            #[allow(clippy::cast_precision_loss)]
            let t = i as f64 / spans as f64;
            // 1 ..= p - 1 (a degree-1 direction admits no interior
            // knot at all — `check_direction`'s Degree1Crease).
            let m = if p >= 2 { 1 + r.below(p - 1) } else { 1 };
            for _ in 0..m {
                k.push(t);
            }
        }
        k.extend(vec![1.0; p + 1]);
        KnotVector::clamped(k, p).unwrap()
    }
    let mut strict = 0usize;
    let mut saw_high_mult = false;
    let trials = fuzz::scaled(60);
    for _ in 0..trials {
        let (pu, pv) = (1 + rng.below(3), 1 + rng.below(3));
        let kv_u = mk(&mut rng, pu);
        let kv_v = mk(&mut rng, pv);
        saw_high_mult |= kv_u.interior_knots().any(|(_, m)| m >= 2)
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
        let Ok(s) = NurbsSurface::new(kv_u, kv_v, control, w) else {
            continue;
        };
        let (Some(whole), Some(fold)) = (whole_net_bound(&s), fold_bound(&s)) else {
            continue;
        };
        for (what, f, w) in [
            ("muu", fold.muu, whole.muu),
            ("muv", fold.muv, whole.muv),
            ("mvv", fold.mvv, whole.mvv),
            ("mu1", fold.mu1, whole.mu1),
            ("mv1", fold.mv1, whole.mv1),
        ] {
            assert!(
                f <= w,
                "the fold EXCEEDED the whole-net hull on {what}: {f:.17e} > \
                 {w:.17e} — {}",
                fuzz::replay()
            );
            if f < w {
                strict += 1;
            }
        }
    }
    // COVERAGE FLOOR, both halves. An inequality nothing ever
    // makes strict is a tautology; and a sweep that never leaves
    // multiplicity 1 never challenges the coverage premise the
    // inequality rests on. If a run trips either, RAISE the trial
    // count — never lower the floor.
    assert!(
        saw_high_mult,
        "the sweep never drew an interior multiplicity >= 2, so the fold's window \
         coverage went unchallenged where it is tight — {}",
        fuzz::replay()
    );
    assert!(
        strict > trials,
        "the sweep must keep producing STRICT gaps: {strict} strict of {} \
         comparisons — {}",
        trials * 5,
        fuzz::replay()
    );
    println!(
        "cert10 fold-vs-whole-net: {strict} strict of {} comparisons",
        trials * 5
    );
}
