//! CERT-7 review lane R2 probes (local only, never pushed).
//!
//! What these rows push on, beyond the shipped suite:
//!
//! - **An e2e rational base nobody shipped**: an ellipse-of-revolution
//!   wall (rational biquadratic, weights carrying `cos` products of
//!   two different sweeps) through the public storage door
//!   `approx_offset_surface`, then re-derived via `recertify_approx`.
//! - **Hostile weight spreads on the fit**: uniform rescaling by
//!   1e3 / 1e-3 / 1e150 / 1e-150 (same surface in R, wildly different
//!   homogeneous magnitudes), and a non-uniform 0.1..10 alternation
//!   (a genuinely different surface). The only admissible outcomes:
//!   a containing certificate or a refusal — never a finite
//!   under-report.
//! - **Recentring pushed farther**: shift 1e8, where the base's own
//!   f64 control points carry ~1.5e-8 of representation granularity
//!   against a micron offset.
//! - **A stall hunt**: fixtures chosen to plateau the bound with
//!   rounds in hand.
//! - **Issue 1321's cap-stop digit**: `budget: 6` reported after the
//!   cap stop.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;
use std::sync::Arc;

use geom::NurbsSurface;
use geom_brep::offset_fit::{
    OffsetFitError, approx_offset_surface, certify_offset, fit_offset, offset_point,
    recertify_approx,
};
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
    NurbsSurface::new(kv2(), kv1(), control, vec![1.0, 1.0, s, s, 1.0, 1.0]).unwrap()
}

/// An ellipse-of-revolution wall: the elliptic meridian arc from
/// angle `t0` to `t1` (circle arc scaled `a` in x, `b` in z), revolved
/// a quarter turn about the z axis. Exact rational biquadratic; the
/// weights are products of two different arc weights, so no shipped
/// fixture shares them. The meridian stays off the axis, so the chart
/// normal is regular.
fn ellipse_wall(a: f64, b: f64, t0: f64, t1: f64) -> NurbsSurface<f64> {
    let wm = ((t1 - t0) * 0.5).cos();
    let tm = 0.5 * (t0 + t1);
    // Circle arc control (unit circle, x-z plane), then scaled.
    let m0 = (t0.cos(), t0.sin());
    let m1 = (tm.cos() / wm, tm.sin() / wm);
    let m2 = (t1.cos(), t1.sin());
    let mer = [(a * m0.0, b * m0.1), (a * m1.0, b * m1.1), (a * m2.0, b * m2.1)];
    let mw = [1.0, wm, 1.0];
    let wl = (FRAC_PI_2 * 0.5).cos();
    let mut control = Vec::with_capacity(9);
    let mut weights = Vec::with_capacity(9);
    for (i, (x, z)) in mer.iter().enumerate() {
        // Longitude ring at radius x: (x,0), (x,x), (0,x), weights 1, wl, 1.
        for (px, py, lw) in [(*x, 0.0, 1.0), (*x, *x, wl), (0.0, *x, 1.0)] {
            control.push(Point3::new(px, py, *z));
            weights.push(mw[i] * lw);
        }
    }
    NurbsSurface::new(kv2(), kv2(), control, weights).unwrap()
}

fn dense(n: usize, m: usize) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for i in 0..=n {
        for j in 0..=m {
            out.push((i as f64 / n as f64, j as f64 / m as f64));
        }
    }
    out
}

fn sampled_residual(base: &NurbsSurface<f64>, fit: &NurbsSurface<f64>, d: f64) -> f64 {
    let mut worst = 0.0f64;
    for (u, v) in dense(40, 40) {
        let target = offset_point(base, d, u, v).unwrap();
        worst = worst.max((fit.eval(u, v) - target).norm());
    }
    worst
}

/// E2E: the ellipse wall through the public storage door, both
/// re-derivation and containment checked from outside.
#[test]
fn r2_ellipse_of_revolution_wall_through_the_storage_door() {
    let wall = ellipse_wall(2.0, 1.0, 0.2, 1.2);
    let d = 0.05;
    let tol = 1e-3;
    let s = approx_offset_surface(Arc::new(wall.clone()), d, tol, band())
        .unwrap_or_else(|e| panic!("the ellipse wall refused through the storage door: {e}"));
    let geom::Surface::Approx(approx) = s else {
        panic!("the storage door returned a non-Approx variant");
    };
    let cert = recertify_approx(&approx, tol, band()).unwrap();
    let worst = sampled_residual(&wall, approx.fit(), d);
    assert!(
        worst <= cert.hull_sup,
        "hull_sup {} UNDER-reports the sampled residual {worst}",
        cert.hull_sup
    );
    eprintln!(
        "R2 ellipse wall: cells={} rounds={} hull_sup={:.3e} sampled={worst:.3e}",
        cert.cells, cert.rounds, cert.hull_sup
    );
}

/// Hostile weight spreads: a rational fit rescaled uniformly is the
/// SAME surface; the certificate must either contain or refuse.
#[test]
fn r2_hostile_uniform_weight_scales_contain_or_refuse() {
    let base = quarter_cylinder(1.0, 1.0);
    let d = 0.2;
    let exact = quarter_cylinder(1.2, 1.0);
    let worst = sampled_residual(&base, &exact, d);
    for k in [1.0, 1.0e3, 1.0e-3, 1.0e150, 1.0e-150] {
        let scaled = NurbsSurface::new(
            exact.knots_u().clone(),
            exact.knots_v().clone(),
            exact.control().to_vec(),
            exact.weights().iter().map(|w| w * k).collect(),
        )
        .unwrap();
        match certify_offset(&base, &scaled, d, 1e-3, band()) {
            Ok(cert) => {
                assert!(
                    cert.hull_sup >= worst,
                    "k={k:e}: hull_sup {} UNDER-reports sampled {worst}",
                    cert.hull_sup
                );
                eprintln!("k={k:e}: certified, hull_sup={:.3e}", cert.hull_sup);
            }
            Err(e) => eprintln!("k={k:e}: refused (honest): {e}"),
        }
    }
}

/// A non-uniform hostile spread (0.1 / 10 alternation): a genuinely
/// different surface, and a weight ratio far beyond the shipped rows'
/// 1.6x / 2.0x perturbations.
#[test]
fn r2_hostile_alternating_weights_contain_or_refuse() {
    let base = quarter_cylinder(1.0, 1.0);
    let d = 0.25;
    let (fit, _) = fit_offset(&base, d, 1e-3, band()).unwrap();
    let n = fit.weights().len();
    let weights: Vec<f64> = (0..n).map(|i| if i % 2 == 0 { 0.1 } else { 10.0 }).collect();
    let hostile = NurbsSurface::new(
        fit.knots_u().clone(),
        fit.knots_v().clone(),
        fit.control().to_vec(),
        weights,
    )
    .unwrap();
    let worst = sampled_residual(&base, &hostile, d);
    match certify_offset(&base, &hostile, d, worst * 4.0, band()) {
        Ok(cert) => {
            assert!(
                cert.hull_sup >= worst,
                "hostile spread: hull_sup {} UNDER-reports sampled {worst}",
                cert.hull_sup
            );
            eprintln!(
                "hostile 0.1/10: certified, hull_sup={:.3e} sampled={worst:.3e} ratio {:.2}x",
                cert.hull_sup,
                cert.hull_sup / worst
            );
        }
        Err(e) => eprintln!("hostile 0.1/10: refused (honest, sampled {worst:.3e}): {e}"),
    }
}

/// Recentring pushed to 1e8: containment must hold whatever happens
/// to the bound's size; the degradation, if any, is reported.
#[test]
fn r2_recentring_at_shift_1e8() {
    let d = 1e-6;
    for shift in [0.0_f64, 1.0e3, 1.0e8] {
        let c = quarter_cylinder(1.0, 1.0);
        let control: Vec<Point3<f64>> = c
            .control()
            .iter()
            .map(|p| Point3::new(p.x + shift, p.y + shift, p.z))
            .collect();
        let base = NurbsSurface::new(
            c.knots_u().clone(),
            c.knots_v().clone(),
            control,
            c.weights().to_vec(),
        )
        .unwrap();
        match fit_offset(&base, d, 1e-2, band()) {
            Ok((fit, cert)) => {
                let worst = sampled_residual(&base, &fit, d);
                assert!(
                    worst <= cert.hull_sup,
                    "shift {shift:e}: hull_sup {} UNDER-reports sampled {worst}",
                    cert.hull_sup
                );
                eprintln!(
                    "shift {shift:e}: cells={} hull_sup={:.4e} sampled={worst:.3e}",
                    cert.cells, cert.hull_sup
                );
            }
            Err(e) => eprintln!("shift {shift:e}: refused (honest?): {e}"),
        }
    }
}

/// The stall hunt: fixtures chosen to plateau the bound. A
/// RefinementStalled here is a finding; a BudgetExhausted (or a
/// certificate) corroborates deviation 7.
#[test]
fn r2_stall_hunt() {
    let qc = quarter_cylinder(1.0, 1.0);
    let wall = ellipse_wall(2.0, 1.0, 0.2, 1.2);
    let cases: Vec<(&str, &NurbsSurface<f64>, f64, f64)> = vec![
        ("qc d=1e-6 tol=1e-12", &qc, 1e-6, 1e-12),
        ("qc d=1e-8 tol=1e-12", &qc, 1e-8, 1e-12),
        ("wall d=1e-6 tol=1e-12", &wall, 1e-6, 1e-12),
        ("wall d=0.3 tol=1e-14", &wall, 0.3, 1e-14),
    ];
    for (name, base, d, tol) in cases {
        match fit_offset(base, d, tol, band()) {
            Ok((_, cert)) => eprintln!("{name}: certified hull_sup={:.3e}", cert.hull_sup),
            Err(OffsetFitError::RefinementStalled {
                rounds,
                grid,
                achieved,
                ..
            }) => {
                eprintln!(
                    "{name}: STALLED after {rounds} rounds on {}x{} achieved={achieved:.3e}",
                    grid.0, grid.1
                );
            }
            Err(OffsetFitError::BudgetExhausted {
                budget,
                grid,
                achieved,
                ..
            }) => {
                eprintln!(
                    "{name}: budget-exhausted (budget {budget}) on {}x{} achieved={achieved:.3e}",
                    grid.0, grid.1
                );
            }
            Err(e) => eprintln!("{name}: other refusal: {e}"),
        }
    }
}

/// Issue 1321's measured instance: at d = 1e-7 the loop stops on the
/// sample cap and reports `budget: 6`.
#[test]
fn r2_issue_1321_cap_stop_reports_budget_six() {
    let base = quarter_cylinder(1.0, 1.0);
    match fit_offset(&base, 1e-7, 1e-9, band()) {
        Err(OffsetFitError::BudgetExhausted { budget, grid, .. }) => {
            eprintln!("d=1e-7 tol=1e-9: BudgetExhausted budget={budget} grid={grid:?}");
        }
        other => eprintln!("d=1e-7 tol=1e-9: {other:?}"),
    }
}
