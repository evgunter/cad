//! Offset-certificate probes driven through the public doors only.
//!
//! **Where the independence is, and where it is not.** The e2e row's
//! base is a rational the shipped suite uses nowhere — that one is
//! built here and stays here. The two hostile-weight rows are not
//! about the carrier at all: what they perturb is the FIT's weight
//! net, and the carrier under it was character-for-character
//! `offset_fit.rs`'s quarter cylinder, so it is now the one
//! `crate::shared::fixture::quarter_cylinder`.
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

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;
use std::sync::Arc;

use geom::NurbsSurface;
use geom_brep::offset_fit::{approx_offset_surface, certify_offset, fit_offset, recertify_approx};
use geom_core::Point3;

use crate::shared::fixture::{arc_weight, kv2, quarter_cylinder};
use crate::shared::sample::{grid, worst_offset_residual};
use crate::shared::tol::band;

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
    let mer = [
        (a * m0.0, b * m0.1),
        (a * m1.0, b * m1.1),
        (a * m2.0, b * m2.1),
    ];
    let mw = [1.0, wm, 1.0];
    let wl = arc_weight(FRAC_PI_2);
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

/// The residual against the exact offset locus, on 41 x 41 stations —
/// a schedule that lands ON a 40-cell fit grid rather than off it, so
/// it is a lower bound on the sup and nothing more. Every row that
/// uses it asserts containment (`hull_sup` at or above this), which is
/// the direction that survives that.
fn sampled_residual(base: &NurbsSurface<f64>, fit: &NurbsSurface<f64>, d: f64) -> f64 {
    worst_offset_residual(base, fit, d, &grid(41, 41)).unwrap()
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
    let weights: Vec<f64> = (0..n)
        .map(|i| if i % 2 == 0 { 0.1 } else { 10.0 })
        .collect();
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
