//! **A circle grazing a torus is answered nowhere within the zero band,
//! and truly in its gap.** Circles grazing a torus, at any tilt, judged
//! against the exact oracle
//! ([`crate::curve_face::conic_oracle::exact`]) at
//! `ε = 1e-6, 1e-9, 1e-12` and the zero band rather than `Kε`: none
//! within it is answered, and every answer in the gap is true.

test_utils::gated_to![
    "crates/geom-brep/src/curve_face/circle_torus.rs",
    "crates/geom-brep/src/curve_face/circle_roots.rs",
    "crates/geom-brep/src/half_angle.rs",
    "crates/geom-brep/src/quartic.rs",
    "crates/geom-core/src/bisect.rs",
    "crates/geom-brep/src/curve_face/conic_oracle.rs",
    "crates/geom-brep/src/curve_face/conic_oracle/",
    "crates/geom-brep/src/implicit.rs",
    "crates/geom-core/src/predicate.rs",
    "crates/geom-core/src/running.rs",
    "crates/geom-core/src/real.rs",
];

use core::f64::consts::{PI, TAU};

use super::*;
use crate::curve_face::conic_oracle::{exact, graze};
use geom_core::{Point3, Vec3};
use test_utils::fuzz;

const EPS_ROWS: [f64; 3] = [1e-6, 1e-9, 1e-12];

/// **A circle grazing a torus is answered nowhere within the zero band,
/// and truly in its gap** Each pose crosses, grazes or
/// clears a torus `R = 1` of tube `0.05 ≤ r < 1` at a point anywhere on
/// the tube by a depth drawn inside the zero band, inside its gap, or
/// zero, at any tilt. The torus door reads its clear margin through the
/// torus's ceiling on `|F|/|res|`, so no piece reads clear short of the
/// band; an answer it gives must be true, and none within `ε` may be
/// given.
#[test]
fn a_circle_grazing_a_torus_is_answered_truly_or_not_at_all() {
    let mut rng = fuzz::start("conic_quadric_in_band_torus_sweep");
    let n = fuzz::scaled(100);
    let mut wrong = Vec::new();
    for eps in EPS_ROWS {
        let band = Band::new(eps, 10.0 * eps).unwrap();
        for i in 0..3 * n {
            let small = rng.range(0.05, 1.0);
            let torus = geom::Surface::Torus {
                center: Point3::origin(),
                axis: Vec3::new(0.0, 0.0, 1.0),
                major_radius: 1.0,
                minor_radius: small,
                u_ref: Vec3::new(1.0, 0.0, 0.0),
            };
            let (phi, psi) = (rng.range(0.0, TAU), rng.range(0.0, TAU));
            let radial = Vec3::new(phi.cos(), phi.sin(), 0.0);
            let z = Vec3::new(0.0, 0.0, 1.0);
            let on =
                Point3::origin() + radial * (1.0 + small * psi.cos()) + z * (small * psi.sin());
            let normal = radial * psi.cos() + z * psi.sin();
            let (along_phi, along_psi) = (
                Vec3::new(-phi.sin(), phi.cos(), 0.0),
                z * psi.cos() - radial * psi.sin(),
            );
            let g = rng.range(0.0, TAU);
            let tangent = (along_phi * g.cos() + along_psi * g.sin()).normalize();
            let rho = rng.log_uniform(0.01, 100.0);
            let side = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
            let depth = match i % 3 {
                0 => side * rng.range(0.0, eps),
                1 => side * rng.range(eps, 10.0 * eps),
                _ => 0.0,
            };
            let c = graze(on, normal, tangent, (rho, depth, rng.range(0.0, TAU)));
            let (m, h) = (rng.range(-PI, PI), rng.range(0.01, PI));
            let truth = exact::truth(&c, &torus);
            if truth.residual_depth >= 10.0 * eps {
                continue;
            }
            let got = circle_torus_roots(&c, m - h, m + h, &torus, band);
            let true_answer = match &got {
                Ok(Err(_)) | Err(_) => continue,
                Ok(Ok(CircleRoots::Miss)) => truth.roots.is_empty(),
                Ok(Ok(CircleRoots::Certified(found))) => found.count() == truth.roots.len(),
                Ok(Ok(CircleRoots::On(_))) => false,
            };
            if truth.residual_depth < eps || !true_answer {
                wrong.push(format!(
                    "ε {eps:e}, depth {depth:e} (true {:.3e}, {} true roots): {got:?}\n  \
                     {c:?} torus R 1 r {small}",
                    truth.depth,
                    truth.roots.len()
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "{} grazes answered wrongly or within the zero band ({}):\n{}",
        wrong.len(),
        fuzz::replay(),
        wrong.join("\n")
    );
}
