//! **The result sort reads roles through the quadrature lane.** An
//! oblique cylinder bored through a block with two cavities leaves one
//! outer shell whose tunnel wall is a cylinder trimmed obliquely by the
//! block's top and bottom — a face whose signed volume the closed forms
//! do not enclose — and the two cavities. The sort
//! (`crates/topo/src/pieces.rs`) reads every shell's role before it
//! files anything, so it reads the tunnel's through check 10's lane:
//! the result is one solid, its two cavities under it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::shell_operands::two_void_box;
use geom_core::{Affine3, Mat3, Point2, Point3, Tol, Vec3};
use sweep::test_support::finished;
use topo::{BooleanResult, ShellRole};

#[test]
fn an_oblique_bore_through_a_two_void_block_stays_one_hollow_solid() {
    let tol = Tol::witness();
    let block = finished("the two-void block", two_void_box().0, tol);
    // A three-arc cylinder of radius 0.3 on (5, 2), z ∈ [-1, 5], tilted
    // 15° about the x axis through its centre: it misses both cavities
    // (x ≤ 3.8) and the side walls, and crosses the top and bottom.
    let upright =
        sweep::test_support::cylinder_of_arcs_at(3, 0.3, Point2::new(5.0, 2.0), -1.0, 6.0, tol);
    let (s, c) = 15f64.to_radians().sin_cos();
    let tilt = Mat3::from_cols(
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, c, s),
        Vec3::new(0.0, -s, c),
    );
    let pivot = Point3::new(5.0, 2.0, 2.0) - Point3::origin();
    let map = Affine3::from_parts(tilt, pivot - tilt * pivot);
    let bore = finished(
        "the tilted cylinder",
        topo::transform_rigid(&upright, &map, tol).expect("the cylinder tilts"),
        tol,
    );

    let started = std::time::Instant::now();
    let r = topo::subtract(&block, &bore, tol).expect("the bore cuts");
    println!(
        "oblique bore through the two-void block: {:?}",
        started.elapsed()
    );
    let BooleanResult::Body(r) = r else {
        panic!("a body")
    };
    let classes = topo::classify_shells(&r.body, tol).expect("every shell classifies");
    let roles: Vec<ShellRole> = classes.iter().map(|c| c.role).collect();
    assert_eq!(r.body.solids().count(), 1, "one piece: {roles:?}");
    assert_eq!(
        (
            roles.iter().filter(|r| **r == ShellRole::Outer).count(),
            roles.iter().filter(|r| **r == ShellRole::Void).count()
        ),
        (1, 2),
        "the bored wall and its two cavities"
    );
    assert_eq!(topo::validate_geometric(&r.body, tol), Ok(()));
}
