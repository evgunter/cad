//! The snowman: two balls of revolution on distinct centres, meeting in
//! a real circle, under every boolean.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::Body;

/// A ball of radius `r` centred on the y axis at height `y`.
fn ball(r: f64, y: f64) -> Body<f64> {
    revolved_about_y(
        vec![(Point2::new(0.0, y - r), 1.0), (Point2::new(0.0, y + r), 0.0)],
        Revolution::Full,
        Tol::witness(),
    )
}

#[test]
fn snowman_probe() {
    let a = ball(1.0, 0.0);
    let b = ball(0.8, 1.4);
    for (name, r) in [
        ("A u B", topo::boolean::union(&a, &b, Tol::witness()).map(|_| ())),
        ("A - B", topo::boolean::subtract(&a, &b, Tol::witness()).map(|_| ())),
        ("B - A", topo::boolean::subtract(&b, &a, Tol::witness()).map(|_| ())),
        ("A n B", topo::boolean::intersect(&a, &b, Tol::witness()).map(|_| ())),
    ] {
        println!("{name}: {r:?}");
    }
}
