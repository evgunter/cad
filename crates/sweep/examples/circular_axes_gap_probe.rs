//! **The near-square tilts a plane × cylinder or plane × cone section
//! refuses**: a measurement probe for
//! `circular-axes-refuses-between-the-rim-alignment-band-and-the-axes-band`.
//!
//! A plane tilted `θ` off square to the axis, swept log-spaced across
//! `sin θ ∈ [ε/(4r), 4·√(2Kε/r)]`, at the run's ε, over three scales.
//! Each pose prints one `POSE` line: the section table's outcome, a
//! split of the operand body by the plane, and how far each circle
//! that could stand in for the section misses the true section (the
//! largest distance from a section point to the circle, sampled):
//!
//! - `rim`: the cylinder's rim circle through the axis' pierce point
//!   (axis the cylinder's, radius `r`); its miss is `r·tan θ`.
//! - `flat`: the circle in the plane about the pierce point, radius
//!   `r`; its miss is `a − b = r·(1/cos θ − 1)`.
//!
//! `CAD_TOLERANCE_EPS=1e-9 cargo run -p sweep --release --example circular_axes_gap_probe`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{EllipseInvalid, Surface};
use geom_brep::Reach;
use geom_brep::intersect::{
    PlaneConeSection, SectionError, plane_cone_section, plane_cylinder_section,
};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use topo::splitting::split;
use topo::{AtRestBody, Body};

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
}

fn finished(body: Body<f64>) -> AtRestBody<f64> {
    AtRestBody::validate(body, tol()).expect("a valid operand")
}

fn split_outcome(body: &AtRestBody<f64>, origin: Point3<f64>, n: Vec3<f64>) -> String {
    let plane = topo::test_support::split_plane(origin, n, tol());
    match split(body, &plane, tol()) {
        Ok(_) => "SPLITS".to_owned(),
        Err(e) => {
            let s = e.to_string();
            let tag = if s.contains("semi-axes coincide") {
                "circular-axes"
            } else if s.contains("circle or an ellipse") {
                "axes-undecided"
            } else {
                "other"
            };
            format!(
                "REFUSES({tag}): {}",
                s.chars().take(220).collect::<String>()
            )
        }
    }
}

fn section_word<S: core::fmt::Debug>(got: &Result<S, SectionError>) -> String {
    match got {
        Ok(s) => format!("{s:?}").chars().take_while(|c| *c != '(').collect(),
        Err(SectionError::Escalated(d)) => format!("Escalated[{}]", d.predicate.unwrap_or("?")),
        Err(SectionError::Carrier(EllipseInvalid::CircularAxes(_))) => "CircularAxes".to_owned(),
        Err(SectionError::Carrier(EllipseInvalid::Escalated(d))) => {
            format!("Carrier::Escalated[{}]", d.predicate.unwrap_or("?"))
        }
        Err(e) => format!("{e:?}"),
    }
}

/// The largest distance from the section of the cylinder about `z`
/// (radius `r`) by the plane through the origin with normal `n` to the
/// circle about `z` through the origin square to `axis`, radius `r`.
fn circle_miss(r: f64, n: Vec3<f64>, axis: Vec3<f64>) -> f64 {
    let mut worst = 0.0_f64;
    for i in 0..720 {
        let phi = f64::from(i) * core::f64::consts::PI / 360.0;
        let (x, y) = (r * phi.cos(), r * phi.sin());
        let z = -(n.x * x + n.y * y) / n.z;
        let p = Vec3::new(x, y, z);
        let h = p.dot(axis);
        let rho = (p - axis * h).norm();
        worst = worst.max(h.hypot(rho - r));
    }
    worst
}

fn sines(r: f64) -> Vec<f64> {
    let eps = tol().eps();
    let k = band().escalate() / eps;
    let (lo, hi) = (
        (eps / r / 4.0).ln(),
        (4.0 * (2.0 * k * eps / r).sqrt()).ln(),
    );
    (0..=24)
        .map(|i| (lo + (hi - lo) * f64::from(i) / 24.0).exp())
        .collect()
}

fn cyl_family() {
    let eps = tol().eps();
    for r in [1e-2, 1.0, 1e2] {
        let body = finished(sweep::test_support::prism(
            vec![(Point2::new(r, 0.0), 1.0), (Point2::new(-r, 0.0), 1.0)],
            r,
            tol(),
        ));
        let cyl = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: r,
            u_ref: Vec3::unit_x(),
        };
        let at = Point3::new(0.0, 0.0, 0.5 * r);
        let reach = Reach::Measured {
            at,
            lever: (5.0_f64).sqrt() * r,
        };
        for s in sines(r) {
            let n = Vec3::new(s, 0.0, (1.0 - s * s).sqrt());
            let plane = Surface::Plane {
                origin: at,
                normal: n,
                u_ref: Vec3::unit_y(),
            };
            let got = plane_cylinder_section(&plane, &cyl, &reach, band());
            let rim = circle_miss(r, n, Vec3::unit_z());
            let flat = circle_miss(r, n, n);
            let cut = split_outcome(&body, at, n);
            println!(
                "POSE cyl eps{eps:e} r{r:e} sin{s:.3e} | r·sin/ε {:9.3e} | rim miss/ε {:9.3e} | flat miss/ε {:9.3e} | {} | {cut}",
                r * s / eps,
                rim / eps,
                flat / eps,
                section_word(&got),
            );
        }
    }
}

fn cone_family() {
    let eps = tol().eps();
    let alpha = 0.5_f64.atan();
    for scale in [1e-2, 1.0, 1e2] {
        let (h, r1) = (scale, 0.5 * scale);
        let r0 = r1 + h * alpha.tan();
        let body = finished(sweep::test_support::revolved_about_y(
            vec![
                (Point2::new(0.0, 0.0), 0.0),
                (Point2::new(r0, 0.0), 0.0),
                (Point2::new(r1, h), 0.0),
                (Point2::new(0.0, h), 0.0),
            ],
            sweep::Revolution::Full,
            tol(),
        ));
        let apex = Point3::new(0.0, h + r1 / alpha.tan(), 0.0);
        let cone = Surface::Cone {
            apex,
            axis: Vec3::new(0.0, -1.0, 0.0),
            half_angle: alpha,
            u_ref: Vec3::unit_x(),
        };
        let reach = Reach::Measured {
            at: apex,
            lever: (2.0 * r0).max((h * h + (r0 + r1).powi(2)).sqrt()),
        };
        let q = Point3::new(0.0, 0.5 * h, 0.0);
        let rim_r = 0.5 * (r0 + r1);
        for s in sines(rim_r) {
            let n = Vec3::new(s, (1.0 - s * s).sqrt(), 0.0);
            let plane = Surface::Plane {
                origin: q,
                normal: n,
                u_ref: Vec3::unit_z(),
            };
            let got = plane_cone_section(&plane, &cone, &reach, band());
            let axes = match &got {
                Ok(PlaneConeSection::TiltedEllipse(geom::Curve3::Ellipse {
                    semi_u,
                    semi_v,
                    ..
                })) => format!("a−b/ε {:9.3e}", (semi_u - semi_v).abs() / eps),
                _ => "a−b/ε -".to_owned(),
            };
            let cut = split_outcome(&body, q, n);
            println!(
                "POSE cone eps{eps:e} S{scale:e} sin{s:.3e} | rim_r·sin/ε {:9.3e} | {axes} | {} | {cut}",
                rim_r * s / eps,
                section_word(&got),
            );
        }
    }
}

fn main() {
    cyl_family();
    cone_family();
}
