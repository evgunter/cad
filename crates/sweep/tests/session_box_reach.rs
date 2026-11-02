//! **The session box reads a body's own points, never its carriers'
//! stored terms.** A cut of a large torus inside the box carries spiric
//! edges whose carrier, the whole torus, reaches past it; the box reads
//! each edge over its own span, so the cut is a body.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{brick, finished, revolved_about_y};

/// **A bar cutting a torus of `R = 700`, `r = 50` inside the box builds**
/// (the full review of PR 6380's probe). The torus about `y`, centred
/// at `(0, 300, 0)`, reaches about 828 m; the bar
/// `[100, 400] × [200, 400] × [400, 800]` reaches at most 980 m. Each
/// spiric edge the cut leaves on the torus rides a carrier whose stored
/// terms reach `|c| + R + r ≈ 1050` m, past the box: read so, the cut
/// was refused.
#[test]
fn a_bar_cutting_a_large_torus_inside_the_box_builds() {
    let tol = Tol::witness();
    let (big_r, r) = (700.0, 50.0);
    let torus = revolved_about_y(
        vec![
            (Point2::new(big_r - r, 0.0), 1.0),
            (Point2::new(big_r + r, 0.0), 1.0),
        ],
        Revolution::Full,
        tol,
    );
    let moved = topo::transform_rigid(
        &torus,
        &geom_core::test_support::rigid(&Affine3::translation(Vec3::new(0.0, 300.0, 0.0)), tol),
        tol,
    )
    .unwrap();
    let torus = finished("the torus", moved, tol);
    let bar = finished(
        "the bar",
        brick::<f64>((100.0, 400.0), (200.0, 400.0), (400.0, 800.0), tol),
        tol,
    );
    let cut =
        topo::intersect(&torus, &bar, tol).unwrap_or_else(|e| panic!("the in-box cut builds: {e}"));
    let body = cut.body().expect("the cut is a body");
    assert!(
        topo::validate_geometric(&body.body, tol).is_ok(),
        "the cut passes the finished-body gate"
    );
}
