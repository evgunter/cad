//! VERBS-OFF-B r1 reviewer probes — adversarial consumer rows for
//! `geom_brep::offset_fit` and the two meters, independent of the
//! shipped `offset_fit.rs` fixtures.
//!
//! What these rows push on, beyond the shipped suite:
//!
//! - **`certify_offset` on a RATIONAL fit**: the composite's fitted
//!   net is read unweighted (`channel(fit, c, false)`), so a fit with
//!   non-unit weights is polynomialized as the WRONG surface. The row
//!   asserts the sound direction (a returned certificate's `hull_sup`
//!   contains a dense sample's max, or the door refuses) — it goes
//!   red if the hull limb under-reports on an accepted input.
//! - **The sign witness**: a fit built for `−d` certified against
//!   `+d` must refuse, never report a finite wrong bound.
//! - **A tangentially slid fit**: pushes the `τ` limb of the
//!   rationalized bound; containment or refusal, never under-report.
//! - **Extreme `d` near the certified collapse reach** on the exact
//!   rational sphere band, both the inward edge and a large outward
//!   offset — containment against a dense independent sample.
//! - **An extreme-weight rational base** (weights spanning 20×): the
//!   certificate either refuses or contains a dense sample's max.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom::NurbsSurface;
use geom_brep::offset_fit::{OffsetFitError, certify_offset, fit_offset, offset_point};
use geom_brep::offset_meters::{OFFSET_METER_LADDER, patch_collapse};
use geom_brep::patch_bound::patch_cells_refined;
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn kv2() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap()
}

fn kv1() -> KnotVector {
    KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap()
}

/// The shipped suite's exact quarter cylinder (weights `1, √2/2, 1`).
fn quarter_cylinder(r: f64, h: f64) -> NurbsSurface<f64> {
    let s = (FRAC_PI_2 * 0.5).cos();
    let control = vec![
        Point3::new(r, 0.0, 0.0),
        Point3::new(r, 0.0, h),
        Point3::new(r, r, 0.0),
        Point3::new(r, r, h),
        Point3::new(0.0, r, 0.0),
        Point3::new(0.0, r, h),
    ];
    let weights = vec![1.0, 1.0, s, s, 1.0, 1.0];
    NurbsSurface::new(kv2(), kv1(), control, weights).unwrap()
}

/// The shipped suite's exact sphere band (A8.1 weight product).
fn sphere_band(r: f64, lat0: f64, lat1: f64) -> NurbsSurface<f64> {
    let theta = 0.5 * (lat1 - lat0);
    let wm = theta.cos();
    let a = (r * lat0.cos(), r * lat0.sin());
    let b = (r * lat1.cos(), r * lat1.sin());
    let mid = (a.0 + b.0, a.1 + b.1);
    let mlen = (mid.0 * mid.0 + mid.1 * mid.1).sqrt();
    let m = (mid.0 / mlen * r / wm, mid.1 / mlen * r / wm);
    let meridian = [(a.0, a.1, 1.0), (m.0, m.1, wm), (b.0, b.1, 1.0)];
    let wr = (FRAC_PI_2 * 0.5).cos();
    let mut control = Vec::with_capacity(9);
    let mut weights = Vec::with_capacity(9);
    for iu in 0..3 {
        for (x, z, w) in meridian {
            control.push(match iu {
                0 => Point3::new(x, 0.0, z),
                1 => Point3::new(x, x, z),
                _ => Point3::new(0.0, x, z),
            });
            weights.push(if iu == 1 { w * wr } else { w });
        }
    }
    NurbsSurface::new(kv2(), kv2(), control, weights).unwrap()
}

/// A dense deterministic grid, coprime to every schedule in the unit.
fn dense(nu: usize, nv: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::with_capacity(nu * nv);
    for i in 0..nu {
        for j in 0..nv {
            #[allow(clippy::cast_precision_loss)]
            out.push((i as f64 / (nu - 1) as f64, j as f64 / (nv - 1) as f64));
        }
    }
    out
}

/// The true residual of `candidate` against the exact offset locus of
/// `base` at `d`, dense-sampled.
fn sampled_residual(base: &NurbsSurface<f64>, candidate: &NurbsSurface<f64>, d: f64) -> f64 {
    let mut worst = 0.0f64;
    for (u, v) in dense(41, 37) {
        let target = offset_point(base, d, u, v).unwrap();
        worst = worst.max((candidate.eval(u, v) - target).norm());
    }
    worst
}

/// `certify_offset` accepts an externally supplied fit with ANY
/// weights, but the hull limb reads the fitted net unweighted. The
/// sound behaviours are: refuse, or return a certificate whose
/// `hull_sup` contains a dense sample's max. Anything else is a
/// finite wrong bound — the one failure the module promises never to
/// produce.
#[test]
fn r1_certify_offset_on_a_rational_fit_never_under_reports() {
    let base = quarter_cylinder(1.0, 1.0);
    let d = 0.3;
    let (fit, _) = fit_offset(&base, d, 1e-4, band()).unwrap();
    // Perturb one interior weight: the surface moves, the unweighted
    // control net — the only thing the hull limb reads — does not.
    let (cu, cv) = fit.control_counts();
    let mut weights = fit.weights().to_vec();
    weights[(cu / 2) * cv + cv / 2] = 2.0;
    let warped = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        fit.control().to_vec(),
        weights,
    )
    .unwrap();
    let true_residual = sampled_residual(&base, &warped, d);
    // A tolerance the warped surface's on-locus samples clear, so the
    // hull limb is the deciding one.
    let tol = true_residual * 2.0;
    match certify_offset(&base, &warped, d, tol, band()) {
        Err(_) => {} // refusing an out-of-model input is sound
        Ok(cert) => {
            assert!(
                cert.hull_sup >= true_residual,
                "certify_offset accepted a rational fit and its hull limb \
                 UNDER-reports: certified sup {} vs dense sampled residual {} \
                 (the fitted net was read unweighted)",
                cert.hull_sup,
                true_residual
            );
        }
    }
}

/// A fit built for `−d`, certified against `+d`: `E·n` carries the
/// wrong sign everywhere, which is exactly what the `D` witness
/// excludes. The door must refuse; a finite certificate here would be
/// a wrong answer.
#[test]
fn r1_a_fit_for_the_wrong_sign_refuses() {
    let base = quarter_cylinder(1.25, 0.75);
    let (fit_neg, _) = fit_offset(&base, -0.3, 1e-3, band()).unwrap();
    match certify_offset(&base, &fit_neg, 0.3, 10.0, band()) {
        Ok(cert) => panic!(
            "a fit of the OPPOSITE offset certified against +d with hull_sup {}",
            cert.hull_sup
        ),
        Err(OffsetFitError::Limb { .. } | OffsetFitError::Meter(_)) => {}
        Err(e) => panic!("refused, but not at a limb or meter: {e}"),
    }
}

/// A tangentially slid fit — the residual is dominated by the `τ`
/// (tangential) limb of the rationalized bound rather than the
/// normal-distance limb. Containment or refusal, never under-report.
#[test]
fn r1_a_slid_fit_s_tau_limb_never_under_reports() {
    let base = quarter_cylinder(1.0, 1.0);
    let d = 0.25;
    let (fit, _) = fit_offset(&base, d, 1e-4, band()).unwrap();
    // Slide the whole net along +z: on a cylinder about z this is
    // purely tangential, so ‖E‖ barely moves while E × m grows.
    let slid_control: Vec<Point3<f64>> = fit
        .control()
        .iter()
        .map(|p| Point3::new(p.x, p.y, p.z + 5e-4))
        .collect();
    let slid = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        slid_control,
        fit.weights().to_vec(),
    )
    .unwrap();
    let true_residual = sampled_residual(&base, &slid, d);
    assert!(
        true_residual >= 4e-4,
        "the slide did not register: {true_residual}"
    );
    match certify_offset(&base, &slid, d, true_residual * 3.0, band()) {
        Err(_) => {}
        Ok(cert) => assert!(
            cert.hull_sup >= true_residual,
            "the slid fit's certified sup {} UNDER-reports the sampled {}",
            cert.hull_sup,
            true_residual
        ),
    }
}

/// Extreme `d`: 90% of the certified inward reach on the exact
/// rational sphere band, and a large outward offset (a sphere never
/// folds outward). Both must certify at a realistic tolerance and
/// contain an independent dense sample of the closed form.
#[test]
fn r1_extreme_d_near_the_collapse_bound_certifies_and_contains() {
    let r = 2.0;
    let base = sphere_band(r, 0.25, 1.25);
    let cells = patch_cells_refined(&base, OFFSET_METER_LADDER[1]).unwrap();
    let reach = patch_collapse(&cells, -1.0).reach;
    assert!(reach > 0.0 && reach.is_finite());
    for d in [-0.9 * reach, 5.0] {
        let tol = 2e-3;
        let (fit, cert) = fit_offset(&base, d, tol, band())
            .unwrap_or_else(|e| panic!("fit_offset refused at d = {d} (reach {reach}): {e}"));
        let mut worst = 0.0f64;
        for (u, v) in dense(41, 37) {
            let p = base.eval(u, v);
            let k = (r + d) / r;
            let want = Point3::new(p.x * k, p.y * k, p.z * k);
            worst = worst.max((fit.eval(u, v) - want).norm());
        }
        assert!(
            worst <= cert.hull_sup,
            "d = {d}: certified sup {} UNDER-reports the closed-form sample {worst}",
            cert.hull_sup
        );
        assert!(worst <= tol, "d = {d}: sampled max {worst} exceeds {tol}");
        eprintln!(
            "extreme d={d:.4} (reach {reach:.4}): cells={} rounds={} hull_sup={:.3e} sampled={worst:.3e}",
            cert.cells, cert.rounds, cert.hull_sup
        );
    }
}

/// An extreme-weight rational base — weights spanning 20× on a
/// bicubic-degree-2 patch. The hull machinery may be arbitrarily
/// conservative here (refusal, or budget exhaustion, is sound); what
/// it may never do is certify a bound below a dense sample's max.
#[test]
fn r1_an_extreme_weight_rational_base_refuses_or_contains() {
    let control: Vec<Point3<f64>> = (0..3)
        .flat_map(|i| {
            (0..3).map(move |j| {
                #[allow(clippy::cast_precision_loss)]
                let (x, y) = (i as f64 * 0.5, j as f64 * 0.5);
                Point3::new(x, y, 0.3 * ((i * 2 + j) as f64).sin())
            })
        })
        .collect();
    let weights = vec![1.0, 0.05, 1.0, 0.4, 1.0, 0.4, 1.0, 0.05, 1.0];
    let base = NurbsSurface::new(kv2(), kv2(), control, weights).unwrap();
    let d = 0.05;
    match fit_offset(&base, d, 5e-4, band()) {
        Err(e) => eprintln!("extreme-weight base refused (sound): {e}"),
        Ok((fit, cert)) => {
            let worst = sampled_residual(&base, &fit, d);
            assert!(
                worst <= cert.hull_sup,
                "extreme-weight base: certified sup {} UNDER-reports sampled {worst}",
                cert.hull_sup
            );
            eprintln!(
                "extreme-weight base: cells={} rounds={} hull_sup={:.3e} sampled={worst:.3e}",
                cert.cells, cert.rounds, cert.hull_sup
            );
        }
    }
}
