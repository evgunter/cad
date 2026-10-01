//! **`cone_conic_incidence` against adversarial near-sections.** A
//! counterexample search: random small perturbations of a cone
//! section's eight parameters, at sections from mildly to extremely
//! elongated (`z_centre/z_min` up to ~400). Each draw is AIMED where a
//! reading metered at the conic's CENTRE is blind: scaled so that
//! reading — the kernel's folded residual amplitude, over
//! `2·sin α·|z_centre|` instead of `2·sin α·z_min` — is `0.9·ε`, which
//! near an elongated section's low vertex puts the ellipse far past the
//! sliver. Every draw the kernel calls uncovered must lie within the
//! sliver (`K·ε`) of the cone.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/pcurve_cache.rs",
    "crates/geom-brep/tests/shared/"
];

use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::{PcurveCertifyError, chart_pcurve};
use geom_core::{Point3, Tol, Vec3};
use test_utils::fuzz;

/// The cone of half-angle `alpha` about `+z`, apex at the origin.
fn cone(alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    }
}

/// The worst distance of the carrier from the double cone.
fn off_cone(c: &Curve3<f64>, alpha: f64) -> f64 {
    (0..4096)
        .map(|i| {
            let p = c.eval(f64::from(i) * core::f64::consts::TAU / 4096.0);
            (p.x.hypot(p.y) * alpha.cos() - p.z.abs() * alpha.sin()).abs()
        })
        .fold(0.0_f64, f64::max)
}

/// The section by the plane through `(0, 0, h)` tilted `theta` toward
/// `+x` (`chart_incidence::section`'s construction).
fn section(alpha: f64, h: f64, theta: f64) -> (Point3<f64>, Vec3<f64>, Vec3<f64>, f64, f64) {
    let t = alpha.tan().powi(2);
    let (s, c) = theta.sin_cos();
    let a = c * c - t * s * s;
    let u = Vec3::new(c, 0.0, -s);
    let centre = Point3::new(0.0, 0.0, h) + u * (-t * h * s / a);
    let k = h * alpha.tan() * c;
    (centre, Vec3::new(s, 0.0, c), u, k / a, k / a.sqrt())
}

/// The centre-height reading the draws are aimed by:
/// `cone_conic_incidence`'s folded residual amplitude
/// `|k₀| + |(k₁, k₂)| + |(k₃, k₄)|` over `2·sin α·|z_centre|`.
fn centre_reading(c: (Point3<f64>, Vec3<f64>, Vec3<f64>, f64, f64), alpha: f64) -> f64 {
    let (center, axis, u_ref, major, minor) = c;
    let w = center - Point3::origin();
    let a = u_ref * major;
    let b = axis.cross(u_ref) * minor;
    let (wz, az, bz) = (w.z, a.z, b.z);
    let c2 = alpha.cos().powi(2);
    let k0 = c2 * (w.dot(w) + (a.dot(a) + b.dot(b)) * 0.5) - wz * wz - (az * az + bz * bz) * 0.5;
    let (k1, k2) = (
        (c2 * w.dot(a) - wz * az) * 2.0,
        (c2 * w.dot(b) - wz * bz) * 2.0,
    );
    let (k3, k4) = (
        (c2 * (a.dot(a) - b.dot(b)) - (az * az - bz * bz)) * 0.5,
        c2 * a.dot(b) - az * bz,
    );
    (k0.abs() + k1.hypot(k2) + k3.hypot(k4)) / (2.0 * alpha.sin() * wz.abs())
}

#[test]
fn an_uncovered_perturbed_section_lies_within_the_sliver() {
    let (e, k) = (eps(), Tol::witness().k());
    let alpha = 0.5_f64;
    let crit = (1.0 / alpha.tan()).atan();
    let mut g = fuzz::start("cone_incidence_fuzz::elongated");
    for frac in [0.3_f64, 0.9, 0.99, 0.999] {
        let (c0, n0, u0, ma0, mi0) = section(alpha, 1.0, frac * crit);
        for _ in 0..fuzz::scaled(300) {
            let dir: [f64; 8] = core::array::from_fn(|_| g.range(-1.0, 1.0));
            let make = |s: f64| {
                let om = Vec3::new(dir[3], dir[4], dir[5]) * s;
                let n = (n0 + om.cross(n0)).normalize();
                let u = u0 + om.cross(u0);
                let u = (u - n * u.dot(n)).normalize();
                (
                    c0 + Vec3::new(dir[0], dir[1], dir[2]) * s,
                    n,
                    u,
                    ma0 + dir[6] * s,
                    mi0 + dir[7] * s,
                )
            };
            let ellipse = |(center, axis, u_ref, major, minor)| Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            };
            // The reading is linear in a small scale, so one probe
            // places the draw at its target.
            let per_unit = centre_reading(make(1e-7), alpha) / 1e-7;
            let c = ellipse(make(0.9 * e / per_unit));
            if let Err(PcurveCertifyError::UnsupportedCarrier { .. }) =
                chart_pcurve(&c, &cone(alpha), band())
            {
                let d = off_cone(&c, alpha);
                assert!(
                    d < k * e,
                    "frac {frac}: an ellipse {:.2}·ε off the cone was called uncovered ({})",
                    d / e,
                    fuzz::replay()
                );
            }
        }
    }
}
