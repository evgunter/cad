//! **The parameter finding** (M5 PR 6): PR 5's `pcurve` constructors
//! are *locus*-exact but *parameter*-non-affine, so neither can be the
//! stored per-half-edge cache under the spec §2 contract ("the pcurve's
//! parameter IS the edge's carrier parameter").
//!
//! PR 5 built its two constructors to a **locus** standard, and said so
//! (`pcurve.rs` module docs: "a pcurve is a chart-image cache, where the
//! locus is what certification consumes"). PR 6's certified statement is
//! pointwise in the carrier parameter — `|S(P(t)) − C(t)| ≤ ε` at the
//! shared schedule (spec §3) — and those are different requirements.
//! This file MEASURES the gap instead of asserting it, so the deviation
//! it justifies (the stored form is
//! `geom_brep::Pcurve::Harmonic`, not either PR 5 constructor) is a
//! reviewable number rather than a claim:
//!
//! - the rational-quadratic plane chain's parameter is the conic's own
//!   rational parameter, pinned to θ only at 0, ½, 1 of each ≤ 90°
//!   segment (PR 5's own module docs);
//! - the fitted cylinder graph's parameter is the PR 4 loop's
//!   **chord-length** parameter over the sampled data.
//!
//! Both land at millimetre scale on half-metre geometry — 6 to 10 orders
//! above every ε row this kernel's CI runs (1e-6, 1e-9, 1e-12). Nothing
//! about the constructors is wrong; they answer a different question.
//! The fitted variant has since landed — `Pcurve::Fitted` exists and
//! certifies at rest — and this file does NOT establish what parameter
//! the SSI rung's natively-fitted pcurve carries. That is still the
//! open question, stated in the present tense rather than promised of a
//! milestone that has passed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use crate::shared::surf;
use geom::Curve3;
use geom::Surface;
use geom_brep::{ellipse_pcurve_on_cylinder, ellipse_pcurve_on_plane};
use geom_core::Tol;
use geom_core::{Point3, Vec3};

/// The corpus shape (i) configuration: radius 0.5 m disc, tilt 0.3 rad.
const R: f64 = 0.5;
const TILT: f64 = 0.3;
const H: f64 = 0.5;

fn cylinder() -> Surface<f64> {
    surf::cylinder(R)
}

fn section() -> Curve3<f64> {
    Curve3::Ellipse {
        center: Point3::new(0.0, 0.0, H),
        axis: Vec3::new(TILT.sin(), 0.0, TILT.cos()),
        major: R / TILT.cos(),
        minor: R,
        u_ref: Vec3::new(TILT.cos(), 0.0, -TILT.sin()),
    }
}

/// The section plane of that ellipse, as a chart.
fn section_plane() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.0, 0.0, H),
        normal: Vec3::new(TILT.sin(), 0.0, TILT.cos()),
        u_ref: Vec3::new(TILT.cos(), 0.0, -TILT.sin()),
    }
}

/// The largest `|S(P(sᵢ)) − C(tᵢ)|` over the shared 9-sample schedule,
/// with the pcurve's own domain mapped affinely onto `[t0, t1]` — the
/// only correspondence a stored cache could offer without storing a
/// second (peer) reparameterization.
fn schedule_gap(
    curve: &geom::NurbsCurve2<f64>,
    surface: &Surface<f64>,
    carrier: &Curve3<f64>,
    t0: f64,
    t1: f64,
) -> f64 {
    let (d0, d1) = curve.domain();
    let mut worst: f64 = 0.0;
    for i in 0..9 {
        let frac = f64::from(i) / 8.0;
        let t = t0 + (t1 - t0) * frac;
        let s = d0 + (d1 - d0) * frac;
        let q = curve.eval(s);
        worst = worst.max(surface.eval(q.x, q.y).distance(carrier.eval(t)));
    }
    worst
}

/// The exact rational-quadratic plane chain is locus-exact and
/// parameter-non-affine: every schedule sample lies ON the ellipse (to
/// rounding), yet the pointwise-in-`t` displacement is millimetres.
#[test]
fn plane_chain_is_locus_exact_and_parameter_non_affine() {
    let (t0, t1) = (0.0, PI);
    let plane = section_plane();
    let carrier = section();
    let chain = ellipse_pcurve_on_plane(&carrier, &plane, t0, t1).expect("plane chain");

    // Locus limb: every sample of the chain sits on the carrier's locus
    // (its distance to the nearest carrier point is rounding-scale).
    let (d0, d1) = chain.domain();
    for k in 0..=64 {
        let s = d0 + (d1 - d0) * (f64::from(k) / 64.0);
        let q = chain.eval(s);
        let p = plane.eval(q.x, q.y);
        // Nearest point on the ellipse locus, by two-stage dense search
        // — an independent oracle, no inversion machinery involved.
        // Successive bracket refinement: the distance to a curve grows
        // LINEARLY along it, so one dense sweep is resolution-limited
        // (a 1e-6 rad grid still reads ~5e-7 m). Four 128-fold stages
        // take the bracket to ~1e-11 rad.
        let mut lo = t0;
        let mut hi = t1;
        let mut best = f64::INFINITY;
        for _ in 0..5 {
            let mut best_t = lo;
            best = f64::INFINITY;
            for j in 0..=128 {
                let t = lo + (hi - lo) * (f64::from(j) / 128.0);
                let d = p.distance(carrier.eval(t));
                if d < best {
                    best = d;
                    best_t = t;
                }
            }
            let step = (hi - lo) / 128.0;
            lo = best_t - step;
            hi = best_t + step;
        }
        assert!(best < 1e-9, "chain sample {k} is {best:e} off the locus");
    }

    // Parameter limb: the same chain, read at the shared schedule.
    let gap = schedule_gap(&chain, &plane, &carrier, t0, t1);
    assert!(
        gap > 1e-3,
        "expected a millimetre-scale parameter gap, measured {gap:e}"
    );
    assert!(
        gap < 1e-1,
        "sanity: the gap is a parameter defect, not a locus one ({gap:e})"
    );
}

/// The PR 4 fitted cylinder graph is fitted in the **chord-length**
/// parameter of its sampled data, so the same pointwise reading is
/// millimetres out as well.
#[test]
fn fitted_cylinder_graph_is_chord_parameterized() {
    let (t0, t1) = (0.0, PI);
    let cyl = cylinder();
    let carrier = section();
    let fitted = ellipse_pcurve_on_cylinder(&carrier, &cyl, t0, t1, 3, 1e-6).expect("cylinder fit");
    let gap = schedule_gap(&fitted.curve, &cyl, &carrier, t0, t1);
    assert!(
        gap > 1e-4,
        "expected a chord-vs-parameter gap well above every CI ε row, measured {gap:e}"
    );
}

/// The stored form, by contrast, is pointwise exact to rounding at the
/// same schedule — which is the whole reason it is the stored form.
#[test]
fn the_stored_harmonic_form_is_pointwise_exact() {
    let (t0, t1) = (0.0, PI);
    let cyl = cylinder();
    let carrier = section();
    let band = geom_core::Band::linear(Tol::witness()).expect("band");
    let p = geom_brep::chart_pcurve(&carrier, &cyl, band).expect("chart image");
    let mut worst: f64 = 0.0;
    for i in 0..9 {
        let t = t0 + (t1 - t0) * (f64::from(i) / 8.0);
        let q = p.eval(t);
        worst = worst.max(cyl.eval(q.x, q.y).distance(carrier.eval(t)));
    }
    assert!(worst < 1e-14, "stored form pointwise residual {worst:e}");
}
