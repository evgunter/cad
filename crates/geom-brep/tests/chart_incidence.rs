//! **The class gates of the sphere and cone charts against their
//! incidence tests.** `chart_pcurve`'s sphere arm reads two class gates
//! — polar (the carrier plane ⊥ the axis, centred on it) and meridian —
//! and the cone's reads one (a rim: ⊥ the axis, centred on it). Every
//! carrier failing them is decided by an incidence test
//! (`pcurve_sphere_chart_incident`, `pcurve_cone_chart_incident`): on
//! the chart, an uncovered class; off it, a carrier off the chart. A
//! gate that reads a SECOND, amplified quantity after an in-band first
//! one — or a first-order quantity for a second-order departure — must
//! not decide off-chart on its own; these rows build on-chart carriers
//! at exactly that boundary.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::{band, eps};
use geom::{Curve3, Surface};
use geom_brep::{PcurveCertifyError, UncoveredClass, chart_pcurve};
use geom_core::{Point3, Vec3};

/// The right circular cone of half-angle `alpha` about `+z` with its
/// apex at the origin.
fn cone(alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    }
}

/// The worst distance of a carrier's points from that cone's upper
/// nappe: `|p_r|·cos α − p_z·sin α`.
fn off_cone(carrier: &Curve3<f64>, alpha: f64) -> f64 {
    (0..256)
        .map(|i| {
            let p = carrier.eval(f64::from(i) * core::f64::consts::TAU / 256.0);
            (p.x.hypot(p.y) * alpha.cos() - p.z * alpha.sin()).abs()
        })
        .fold(0.0_f64, f64::max)
}

/// The cone's section by the plane through `(0, 0, h)` whose normal is
/// tilted `theta` off the axis toward `+x`: an ellipse, major along the
/// plane's `x` direction. With `T = tan²α` and `A = cos²θ − T·sin²θ`,
/// the centre sits `−T·h·sin θ / A` along that direction and the
/// semi-axes are `h·tan α·cos θ / A` and `h·tan α·cos θ / √A`.
fn section(alpha: f64, h: f64, theta: f64) -> (Point3<f64>, Vec3<f64>, Vec3<f64>, f64, f64) {
    let t = alpha.tan().powi(2);
    let (s, c) = theta.sin_cos();
    let a = c * c - t * s * s;
    let u = Vec3::new(c, 0.0, -s);
    let centre = Point3::new(0.0, 0.0, h) + u * (-t * h * s / a);
    let k = h * alpha.tan() * c;
    (centre, Vec3::new(s, 0.0, c), u, k / a, k / a.sqrt())
}

/// A tilted plane section of a cone lies on it, and is uncovered; the
/// same ellipse moved off it is a carrier off the chart.
#[test]
fn a_cone_section_is_uncovered_and_a_moved_one_is_off_the_chart() {
    let alpha = 0.5;
    let (centre, axis, u_ref, major, minor) = section(alpha, 2.0, 0.3);
    let on = Curve3::Ellipse {
        center: centre,
        axis,
        major,
        minor,
        u_ref,
    };
    let dist = off_cone(&on, alpha);
    assert!(dist < 1e-14, "on the cone: {dist:e}");
    let got = chart_pcurve(&on, &cone(alpha), band());
    assert!(
        matches!(
            got,
            Err(PcurveCertifyError::UnsupportedCarrier {
                class: UncoveredClass::ConeSection,
                ..
            })
        ),
        "a cone section is uncovered: {got:?}"
    );
    let moved = Curve3::Ellipse {
        center: centre + Vec3::new(0.0, 1e-3, 0.0),
        axis,
        major,
        minor,
        u_ref,
    };
    let got = chart_pcurve(&moved, &cone(alpha), band());
    assert!(
        matches!(got, Err(PcurveCertifyError::CarrierOffChart { .. })),
        "a moved section is off the cone: {got:?}"
    );
}

/// **The cone's rim gates on a needle and on a wide cone.** A circle
/// approximating the section by a plane tilted inside the axial band
/// (`ρ·sin θ = ε/2`) lies on the cone to rounding. Its centre is
/// `≈ tan α · ε/2` off the axis: inside the centring band on a needle
/// cone, where it is a rim and images in closed form, and past it on a
/// wide one, where the centring gate fails. There the incidence test,
/// not the gate, decides — and the circle is on the cone.
#[test]
fn a_near_rim_circle_tilted_inside_the_axial_band_is_on_the_cone() {
    let e = eps();
    for (alpha, rim) in [(0.05_f64, true), (1.55, false)] {
        let h = 1.0;
        let theta = 0.5 * e / (h * alpha.tan());
        let (centre, axis, u_ref, major, minor) = section(alpha, h, theta);
        let circle = Curve3::Circle {
            center: centre,
            axis,
            radius: (major * minor).sqrt(),
            u_ref,
        };
        let dist = off_cone(&circle, alpha);
        assert!(dist < 0.01 * e, "alpha = {alpha}: on the cone: {dist:e}");
        let got = chart_pcurve(&circle, &cone(alpha), band());
        if rim {
            assert!(
                got.is_ok(),
                "alpha = {alpha}: a needle cone's near-rim is a rim: {got:?}"
            );
        } else {
            assert!(
                centre.x.abs() > 10.0 * e,
                "alpha = {alpha}: the centre offset is past the band: {:e}",
                centre.x
            );
            assert!(
                matches!(
                    got,
                    Err(PcurveCertifyError::UnsupportedCarrier {
                        class: UncoveredClass::ConeSection,
                        ..
                    })
                ),
                "alpha = {alpha}: an on-cone circle is never off the chart: {got:?}"
            );
        }
    }
}

/// A small circle exactly on the unit sphere near its pole, whose plane
/// is tilted off ⊥-the-axis by an angle `θ` small enough that the
/// polar class's axial gate (`|a·z| + |b·z| = ρ·sin θ`) reads ZERO, but
/// whose centre's radial offset (`h·sin θ`, `h ≈ R`) is `h/ρ` times
/// larger and reads nonzero. The circle lies on the sphere (its centre
/// is the foot of the sphere's centre on its plane, `ρ² + h² = R²`), so
/// the pair is an uncovered general circle, never off the chart.
#[test]
fn a_near_pole_small_circle_tilted_inside_the_axial_band_is_on_the_sphere() {
    let e = eps();
    let sphere = Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    for rho in [1e-2_f64, 1e-3, 1e-4] {
        let h = (1.0 - rho * rho).sqrt();
        // ρ·sin θ = ε/2: inside the axial gate's zero half.
        let s = 0.5 * e / rho;
        let c = (1.0 - s * s).sqrt();
        let n = Vec3::new(s, 0.0, c);
        let carrier = Curve3::Circle {
            center: Point3::new(h * s, 0.0, h * c),
            axis: n,
            radius: rho,
            u_ref: Vec3::new(c, 0.0, -s),
        };
        let worst = (0..64)
            .map(|i| {
                let p = carrier.eval(f64::from(i) * core::f64::consts::TAU / 64.0);
                ((p.x * p.x + p.y * p.y + p.z * p.z).sqrt() - 1.0).abs()
            })
            .fold(0.0_f64, f64::max);
        assert!(worst < 1e-14, "rho = {rho:e}: on the sphere: {worst:e}");
        // The amplified quantity really is past the band, so the row
        // exercises the second gate rather than passing the first two.
        assert!(h * s > 10.0 * e, "rho = {rho:e}: centre offset {:e}", h * s);
        let got = chart_pcurve(&carrier, &sphere, band());
        assert!(
            matches!(
                got,
                Err(PcurveCertifyError::UnsupportedCarrier {
                    class: UncoveredClass::SphereGeneralCircle,
                    ..
                })
            ),
            "rho = {rho:e}: an on-sphere circle is an uncovered general circle: {got:?}"
        );
    }
}
