//! **A section ellipse nearly along a cylinder's axis builds at every ε
//! row**, once each chord seams it at the principal vertex nearest its
//! arc. A plane `θ` off the axis cuts an ellipse of semi-axis about
//! `r/θ` along it, and the face's part of it sits at its minor vertices:
//! seamed at a major vertex, the parameter there resolves only to
//! `ulp(π/2)·r/θ`, 7.3e-12 at `θ = 3e-5`, and the chord's ends missed
//! their vertices in band at ε = 1e-12 whatever the vertex's offset.
//!
//! - **Planes through a seam vertex**, `(1, 0, 0)` or `(−1, 0, 1)`, with
//!   normal `(0, 1, θ)`, the vertex `c·ε` off along the normal: the
//!   dual review of PR 4810's coin-flip family, 66 poses.
//! - **Planes through the axis**, normal `(cos φ, sin φ, θ)`,
//!   `θ = 1e-6 … 3e-4`: 96 poses.
//!
//! At ε = 1e-6 the smallest tilts read in band of along the axis, and at
//! ε = 1e-9 the seam-vertex planes at `θ = 1e-4` read a sector at the
//! vertex in band: those refuse upstream as the input's readings, as they
//! do on main. No pose at any row refuses at a chord's attachment.
//!
//! Every pose was measured at its row.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Point3, Tol, Vec3};
use topo::splitting::{SplitError, SplitReduceError, split};
use topo::{AtRestBody, SplitJoinError};

/// The unit cylinder about `z`, `0 ≤ z ≤ 1`, its seam vertices at
/// `(±1, 0, 0)` and `(±1, 0, 1)`.
fn cylinder(tol: Tol) -> AtRestBody<f64> {
    AtRestBody::validate(
        sweep::test_support::prism(
            vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)],
            1.0,
            tol,
        ),
        tol,
    )
    .expect("a valid operand")
}

fn cut(
    body: &AtRestBody<f64>,
    origin: Point3<f64>,
    normal: Vec3<f64>,
    tol: Tol,
) -> Result<topo::SplitResult<f64>, SplitError> {
    let plane = topo::test_support::split_plane(origin, normal / normal.norm(), tol);
    split(body, &plane, tol)
}

/// A refusal at a chord's attachment: the join's Euler operation, which
/// carries the section curve's own miss.
fn at_a_chord(e: &SplitError) -> bool {
    matches!(e, SplitError::Join(SplitJoinError::Euler(_)))
}

fn is_eps(eps: f64, row: f64) -> bool {
    (eps - row).abs() <= 1e-3 * row
}

/// The coin-flip family builds at ε = 1e-12 and 1e-6, and at 1e-9 all
/// but the `θ = 1e-4` planes, which read the sector at their vertex in
/// band (`SliverSector`).
#[test]
fn planes_nearly_along_the_axis_through_a_seam_vertex_build() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let cyl = cylinder(tol);
    let mut wrong = Vec::new();
    let mut built = 0;
    for th in [3e-5, 1e-4, 3e-4] {
        for (vx, vz) in [(1.0, 0.0), (-1.0, 1.0)] {
            for c in [
                0.0, 0.5, -0.5, 0.8, -0.8, 0.9, -0.9, 0.99, -0.99, 50.0, -50.0,
            ] {
                let n = Vec3::new(0.0, 1.0, th);
                let o = Point3::new(vx, 0.0, vz) + n / n.norm() * (c * eps);
                let tag = format!("θ {th:e} vertex ({vx}, 0, {vz}) c {c}");
                match cut(&cyl, o, n, tol) {
                    Ok(_) => built += 1,
                    Err(SplitError::Reduce(SplitReduceError::SliverSector { .. }))
                        if is_eps(eps, 1e-9) && th == 1e-4 => {}
                    Err(e) => wrong.push(format!("{tag}: {e:?}")),
                }
            }
        }
    }
    let want = if is_eps(eps, 1e-9) { 54 } else { 66 };
    assert!(
        wrong.is_empty() && built == want,
        "{built} of {want} built:\n{}",
        wrong.join("\n")
    );
}

/// The through-axis planes build at ε = 1e-12 and 1e-9; at 1e-6 the
/// `θ = 3e-4` ones do. No pose at any row refuses at a chord.
#[test]
fn planes_through_the_axis_nearly_along_it_build() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let cyl = cylinder(tol);
    let mut wrong = Vec::new();
    for th in [1e-6, 3e-6, 1e-5, 3e-5, 1e-4, 3e-4] {
        for k in 0..8 {
            let phi = 0.4 + f64::from(k) * 0.7;
            for z in [0.5, 0.3] {
                let n = Vec3::new(phi.cos(), phi.sin(), th);
                let tag = format!("θ {th:e} φ {phi:.1} z {z}");
                let builds = !is_eps(eps, 1e-6) || th == 3e-4;
                match cut(&cyl, Point3::new(0.0, 0.0, z), n, tol) {
                    Ok(_) => {}
                    Err(e) if at_a_chord(&e) || builds => {
                        wrong.push(format!("{tag}: {e:?}"));
                    }
                    Err(_) => {}
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
