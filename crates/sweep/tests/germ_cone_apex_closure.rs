//! **The cone apex closure**: an apex-closed cone sector of any width
//! answers containment, and its face describes for the section
//! certificate's W2.
//!
//! The apex is not a chart point, so the nearest-branch walk reads an
//! apex-closed sector wider than π as a full period there. The closure
//! sets the jump at a single apex visit so that the lifted loop closes,
//! which is exact: `J = −Σ Δθ` over the other edges. The rows:
//!
//! - `point_in_solid` on partial revolves of the unit cone at widths
//!   π/2, π, 3π/2 and 4.5 rad, and on the full revolve merged to one
//!   cone face, against the closed form on a grid: inside exactly when
//!   `0 < y < 1`, `ρ < 1 − y` (the cone's `|q⊥| ≤ tan α · h`, `α = π/4`)
//!   and the azimuth is in the swept window. Named interior, exterior,
//!   boundary and gap-quadrant points on the 3π/2 cone besides.
//! - `curved_face_containment` on the 3π/2 cone's face: `In` across
//!   the window, `Out` in the gap and on the mirror nappe.
//! - W2's `describes` on the apex-closed faces, the preview cone's
//!   merged face among them.
//!
//! The mutant is the nearest-branch pin restored at the apex: the 3π/2
//! and 4.5 rad rows refuse `PartialConeFace` again, and the merged face
//! stops describing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;

use crate::common::approx::band;
use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom_core::{Point2, Point3, Tol};
use profile::{ProfileLoop, RawLoop};
use revolve_common::*;
use sweep::{Revolution, revolve};
use topo::{Body, FaceContainment, FaceKey, SolidContainment, point_in_solid};

/// The `revolve_cone` triangle: apex `(0, 1, 0)`, base radius `1` at
/// `y = 0`, half-angle `π/4`.
fn triangle() -> ProfileLoop<f64> {
    ProfileLoop::polygon([
        Point2::new(0.0, 0.0),
        Point2::new(1.0, 0.0),
        Point2::new(0.0, 1.0),
    ])
}

/// The triangle revolved through `theta` about `y`; `None` is the full
/// revolve, its coplanar and co-conical faces merged (the preview
/// cone: one cone face whose seam it traverses twice).
fn cone(theta: Option<f64>) -> Body<f64> {
    let revolution = theta.map_or(Revolution::Full, Revolution::Partial);
    let mut body = revolve(
        &validated(vec![triangle()]),
        axis_y(),
        revolution,
        Tol::witness(),
    )
    .unwrap()
    .body;
    if theta.is_none() {
        body.merge_coplanar_faces(Tol::witness()).unwrap();
    }
    assert_all_tiers(&body);
    body
}

fn cone_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cone { .. })
            )
        })
        .map(|(k, _)| k)
        .collect()
}

/// A point at azimuth `phi` about `y` (`x = ρ cos φ`, `z = ρ sin φ`).
/// The sweep through `theta` covers `phi ∈ [−theta, 0]`.
fn at(rho: f64, y: f64, phi: f64) -> Point3<f64> {
    let (s, c) = phi.sin_cos();
    Point3::new(rho * c, y, rho * s)
}

fn pis(body: &Body<f64>, q: Point3<f64>) -> SolidContainment {
    point_in_solid(body, q, band(), Tol::witness())
        .unwrap_or_else(|e| panic!("{q:?} must answer: {e:?}"))
}

/// The closed form: `Some(inside)` for a point at least `margin` from
/// every face of the triangle revolved through `theta` (`None`: full),
/// `None` for a point nearer than that.
fn closed_form(q: Point3<f64>, theta: Option<f64>, margin: f64) -> Option<bool> {
    let rho = q.x.hypot(q.z);
    let y = q.y;
    // Distances to the base plane and the cone carrier, signed inward.
    let to_base = y;
    let to_cone = (1.0 - y - rho) / 2.0_f64.sqrt();
    let in_slab = to_base > 0.0 && to_cone > 0.0;
    let mut near = to_base.abs() < margin || (y < 1.0 + margin && to_cone.abs() < margin);
    let in_sector = match theta {
        None => true,
        Some(theta) => {
            // Azimuth from the φ = 0 wall, measured into the sweep.
            let back = (-q.z.atan2(q.x)).rem_euclid(TAU);
            // Distance to a fan wall's half-plane at angular offset d.
            let wall = |d: f64| {
                let d = d.rem_euclid(TAU);
                let d = d.min(TAU - d);
                if d <= FRAC_PI_2 { rho * d.sin() } else { rho }
            };
            near |= wall(back) < margin || wall(back - theta) < margin;
            back < theta
        }
    };
    near |= rho < margin;
    (!near).then_some(in_slab && in_sector)
}

/// Every definite grid point answers, and answers the closed form.
fn agrees_with_the_closed_form(theta: Option<f64>) {
    let body = cone(theta);
    let axis = |k: u32, n: u32, lo: f64, hi: f64| lo + (hi - lo) * f64::from(k) / f64::from(n);
    let (mut ins, mut outs) = (0, 0);
    for i in 0..=12 {
        for j in 0..=8 {
            for k in 0..=12 {
                let q = Point3::new(
                    axis(i, 12, -1.05, 1.05) + 0.013,
                    axis(j, 8, -0.12, 1.12) + 0.007,
                    axis(k, 12, -1.05, 1.05) - 0.011,
                );
                let Some(inside) = closed_form(q, theta, 0.02) else {
                    continue;
                };
                let want = if inside {
                    ins += 1;
                    SolidContainment::In
                } else {
                    outs += 1;
                    SolidContainment::Out
                };
                assert_eq!(pis(&body, q), want, "theta {theta:?}, {q:?}");
            }
        }
    }
    // Anti-vacuity: the grid reaches both sides of the solid.
    assert!(
        ins >= 20 && outs >= 100,
        "theta {theta:?}: {ins} in, {outs} out"
    );
}

#[test]
fn a_quarter_cone_agrees_with_the_closed_form() {
    agrees_with_the_closed_form(Some(FRAC_PI_2));
}

/// Width π: the nearest-branch pin's knife edge, which the closure
/// takes without a tie.
#[test]
fn a_half_cone_agrees_with_the_closed_form() {
    agrees_with_the_closed_form(Some(PI));
}

#[test]
fn a_three_quarter_cone_agrees_with_the_closed_form() {
    agrees_with_the_closed_form(Some(1.5 * PI));
}

#[test]
fn a_4_5_rad_cone_agrees_with_the_closed_form() {
    agrees_with_the_closed_form(Some(4.5));
}

/// The full revolve merged to one cone face: the face alone covers the
/// turn, and the grid holds.
#[test]
fn the_merged_full_cone_agrees_with_the_closed_form() {
    agrees_with_the_closed_form(None);
}

/// **The 3π/2 cone, point by point.** Interior points at three azimuths
/// across the sweep, exterior points past the wall and above the apex,
/// boundary points on the cone face, the base and a fan wall, and the
/// gap quadrant `x > 0, z > 0` at the same radius and height as an
/// interior point.
#[test]
fn the_three_quarter_cone_classifies_every_region() {
    let body = cone(Some(1.5 * PI));
    for phi in [-0.25 * PI, -0.75 * PI, -1.25 * PI] {
        assert_eq!(pis(&body, at(0.3, 0.3, phi)), SolidContainment::In, "{phi}");
        assert_eq!(
            pis(&body, at(0.9, 0.5, phi)),
            SolidContainment::Out,
            "{phi}"
        );
        assert_eq!(
            pis(&body, at(0.5, 0.5, phi)),
            SolidContainment::OnBoundary,
            "on the cone face at {phi}"
        );
        assert_eq!(
            pis(&body, at(0.4, 0.0, phi)),
            SolidContainment::OnBoundary,
            "on the base at {phi}"
        );
    }
    assert_eq!(pis(&body, at(0.3, 0.3, 0.25 * PI)), SolidContainment::Out);
    assert_eq!(pis(&body, at(0.1, 0.8, 0.25 * PI)), SolidContainment::Out);
    assert_eq!(pis(&body, at(0.3, 0.3, 0.0)), SolidContainment::OnBoundary);
    assert_eq!(
        pis(&body, at(0.3, 0.3, -1.5 * PI)),
        SolidContainment::OnBoundary
    );
    assert_eq!(
        pis(&body, Point3::new(0.0, 1.2, 0.0)),
        SolidContainment::Out
    );
    assert_eq!(
        pis(&body, Point3::new(0.0, 1.0, 0.0)),
        SolidContainment::OnBoundary
    );
}

/// **Face containment on the 3π/2 cone's face.** Carrier points across
/// the window are `In`; the gap quadrant, a hair past either wall, and
/// the mirror nappe are `Out`.
#[test]
fn the_three_quarter_cone_face_holds_its_window() {
    let body = cone(Some(1.5 * PI));
    let [f] = cone_faces(&body)[..] else {
        panic!("one cone face");
    };
    let contain = |q| topo::curved_face_containment(&body, f, q, band()).unwrap();
    let off = 1e-5_f64.max(1e3 * eps());
    for y in [0.2, 0.5, 0.8] {
        let rho = 1.0 - y;
        for phi in [-off, -0.25 * PI, -0.75 * PI, -1.25 * PI, -1.5 * PI + off] {
            assert_eq!(
                contain(at(rho, y, phi)),
                Some(FaceContainment::In),
                "y {y}, phi {phi}"
            );
        }
        for phi in [off, 0.25 * PI, 0.5 * PI - off] {
            assert_eq!(
                contain(at(rho, y, phi)),
                Some(FaceContainment::Out),
                "the gap: y {y}, phi {phi}"
            );
        }
        assert_eq!(
            contain(at(rho, 2.0 - y, -0.75 * PI)),
            Some(FaceContainment::Out),
            "the mirror nappe"
        );
    }
}

/// **W2 on apex-closed faces.** The preview cone's merged face, each
/// half-band of the unmerged full cone, and the partial sectors at
/// every width describe: the apex closure closes on each.
#[test]
fn apex_closed_faces_describe() {
    let mut bodies = vec![("merged full cone", cone(None))];
    for theta in [FRAC_PI_2, PI, 1.5 * PI, 4.5] {
        bodies.push(("partial cone", cone(Some(theta))));
    }
    bodies.push((
        "full cone, two half-bands",
        revolve(
            &validated(vec![triangle()]),
            axis_y(),
            Revolution::Full,
            Tol::witness(),
        )
        .unwrap()
        .body,
    ));
    for (name, body) in &bodies {
        let faces = cone_faces(body);
        assert!(!faces.is_empty(), "{name}");
        for f in faces {
            assert!(
                topo::test_support::face_describes(body, f, band()),
                "{name}: face {f:?} must describe"
            );
        }
    }
}
