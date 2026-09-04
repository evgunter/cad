//! **#974's residual one-sign story, measured.**
//!
//! `boolean::rest::tangent_locus`'s CONTRACT is the separation
//! invariant: every configuration the lane mints a locus for has each
//! carrier wholly in ONE closed residual half-space of the other. #974
//! recorded the coaxial cylinder×sphere circle arm as blocked on that
//! story, on the grounds that its residuals are "one-signed in OPPOSITE
//! orientations per direction".
//!
//! Both halves of that are measured here, and the measurement changes
//! what the sentence means:
//!
//! 1. The coaxial tangency (`R = r`, sphere centre on the axis) IS
//!    one-signed in each direction — the sphere never leaves the
//!    cylinder's closed inside, the cylinder never enters the sphere's
//!    closed inside. The invariant HOLDS.
//! 2. The orientations are indeed opposite. But the pair the lane
//!    ALREADY admits under its internal `|r1 − r2|` fallback — two
//!    internally tangent parallel cylinders — has exactly the same
//!    opposite-orientation structure, so opposite orientations cannot
//!    be what disqualifies a configuration from this contract.
//!
//! What still blocks the arm is downstream and structural, and is
//! stated at `tangent_locus` itself: `TangentLocus` carries a LINE
//! only, its consumers read a locus DIRECTION, and none has a circle
//! story. This suite deliberately makes no claim about that half — it
//! measures the residual story and nothing else.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::implicit_residual;
use geom_core::{Band, Point3, Tol, Vec3};

/// A z-axis cylinder of radius `r` through `origin`.
fn cyl(origin: Point3<f64>, r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin,
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    }
}

fn sph(center: Point3<f64>, r: f64) -> Surface<f64> {
    Surface::Sphere {
        center,
        radius: r,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// Sampled points of a sphere's surface.
fn sphere_points(center: Point3<f64>, r: f64) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for i in 0..=24 {
        let lat = -core::f64::consts::FRAC_PI_2 + f64::from(i) / 24.0 * core::f64::consts::PI;
        for j in 0..24 {
            let az = f64::from(j) / 24.0 * core::f64::consts::TAU;
            out.push(
                center
                    + Vec3::new(
                        r * lat.cos() * az.cos(),
                        r * lat.cos() * az.sin(),
                        r * lat.sin(),
                    ),
            );
        }
    }
    out
}

/// Sampled points of a z-axis cylinder's wall, over an axial window.
fn cylinder_points(origin: Point3<f64>, r: f64, half_height: f64) -> Vec<Point3<f64>> {
    let mut out = Vec::new();
    for i in 0..=24 {
        let z = -half_height + f64::from(i) / 24.0 * 2.0 * half_height;
        for j in 0..24 {
            let az = f64::from(j) / 24.0 * core::f64::consts::TAU;
            out.push(origin + Vec3::new(r * az.cos(), r * az.sin(), z));
        }
    }
    out
}

/// The strongest one-sign claim the samples support: every residual is
/// `<= tol` (or every one is `>= -tol`), and at least one is definitely
/// away from zero so the row is not vacuous.
fn one_signed(name: &str, residuals: &[f64]) -> i32 {
    let tol = 1e-12;
    let all_non_positive = residuals.iter().all(|&x| x <= tol);
    let all_non_negative = residuals.iter().all(|&x| x >= -tol);
    let spread = residuals.iter().fold(0.0_f64, |a, &x| a.max(x.abs()));
    assert!(
        spread > 1e-3,
        "{name}: every residual is ~0, so the row proves nothing"
    );
    assert!(
        all_non_positive || all_non_negative,
        "{name}: residuals straddle zero (min {}, max {})",
        residuals.iter().cloned().fold(f64::INFINITY, f64::min),
        residuals.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
    );
    if all_non_positive { -1 } else { 1 }
}

/// **The coaxial cylinder×sphere tangency satisfies the separation
/// invariant.** Sphere wholly inside the cylinder, cylinder wholly
/// outside the sphere — one-signed in BOTH directions.
#[test]
fn the_coaxial_tangency_is_one_signed_in_both_directions() {
    let c = cyl(Point3::origin(), 1.0);
    let s = sph(Point3::origin(), 1.0);
    let sphere_vs_cylinder: Vec<f64> = sphere_points(Point3::origin(), 1.0)
        .into_iter()
        .map(|p| implicit_residual(&c, p))
        .collect();
    let cylinder_vs_sphere: Vec<f64> = cylinder_points(Point3::origin(), 1.0, 2.0)
        .into_iter()
        .map(|p| implicit_residual(&s, p))
        .collect();
    let a = one_signed("sphere against the cylinder", &sphere_vs_cylinder);
    let b = one_signed("cylinder against the sphere", &cylinder_vs_sphere);
    assert_eq!(a, -1, "the sphere is inside the cylinder");
    assert_eq!(b, 1, "the cylinder is outside the sphere");
    // And the orientations are OPPOSITE, exactly as #974 recorded.
    assert_ne!(a, b);
}

/// **The lane ALREADY admits the same opposite-orientation shape.** Two
/// internally tangent parallel cylinders — the `|r1 − r2|` fallback's
/// own configuration — are one-signed in opposite orientations per
/// direction too. So "opposite orientations" is not a disqualifier
/// under this contract; it is a property the admitted set already has.
#[test]
fn the_admitted_internal_cylinder_tangency_has_the_same_orientation_shape() {
    let big = cyl(Point3::origin(), 1.0);
    let small = cyl(Point3::new(0.6, 0.0, 0.0), 0.4);
    let small_vs_big: Vec<f64> = cylinder_points(Point3::new(0.6, 0.0, 0.0), 0.4, 2.0)
        .into_iter()
        .map(|p| implicit_residual(&big, p))
        .collect();
    let big_vs_small: Vec<f64> = cylinder_points(Point3::origin(), 1.0, 2.0)
        .into_iter()
        .map(|p| implicit_residual(&small, p))
        .collect();
    let a = one_signed("small against big", &small_vs_big);
    let b = one_signed("big against small", &big_vs_small);
    assert_eq!(a, -1);
    assert_eq!(b, 1);
    assert_ne!(a, b, "the admitted pair has opposite orientations too");
    // The lane really does admit it: the internal fallback mints.
    let got = topo::tangent_locus(&big, &small, Band::linear(Tol::witness()).unwrap());
    assert!(
        got.is_ok(),
        "the internally tangent parallel pair is admitted: {got:?}"
    );
}
