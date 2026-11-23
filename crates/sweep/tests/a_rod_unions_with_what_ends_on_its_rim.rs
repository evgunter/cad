//! **A rod unions with a body built on its end cap**, two bodies whose
//! walls both end on the cap's rim: a half ball, whose sphere leaves the
//! rim G1 off the rod's cylinder (a π junction whose relative curvature
//! is definite all round), and a frustum, whose cone leaves it at a
//! corner (no junction: the crossing lanes meet it as any corner). The
//! glue door routes each rim before it reads anything of the pair, and
//! each union is the two volumes' sum, sound at tier 3 and 3′, in both
//! member orders.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Point2, Tol};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{AtRestBody, BooleanResult};

/// The rod: radius 1, `y ∈ [−2, 0]`, about the sketch `y` axis.
fn rod() -> AtRestBody<f64> {
    let body = revolved_about_y(
        vec![
            (Point2::new(0.0, -2.0), 0.0),
            (Point2::new(1.0, -2.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 0.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    finished("the rod", body, Tol::witness())
}

/// The unit ball's half above `y = 0`, on the rod's cap.
fn half_ball() -> AtRestBody<f64> {
    let body = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), (PI / 8.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    finished("the half ball", body, Tol::witness())
}

/// A frustum on the rod's cap: radius 1 at `y = 0` to 0.8 at `y = 1`.
fn frustum() -> AtRestBody<f64> {
    let body = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.8, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    );
    finished("the frustum", body, Tol::witness())
}

#[track_caller]
fn unions_at(what: &str, a: &AtRestBody<f64>, b: &AtRestBody<f64>, want: f64) {
    for (order, x, y) in [("rod first", a, b), ("rod second", b, a)] {
        let what = format!("{what}, {order}");
        let Ok(BooleanResult::Body(bb)) = topo::union(x, y, Tol::witness()) else {
            panic!("{what}: the union builds one body");
        };
        assert_eq!(bb.body.solids().count(), 1, "{what}: one solid");
        assert_eq!(
            topo::validate_geometric(&bb.body, Tol::witness()),
            Ok(()),
            "{what}: tier 3"
        );
        assert_eq!(
            topo::validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
            Ok(()),
            "{what}: tier 3′"
        );
        let v = topo::test_support::pinned_volume("the union", &bb.body, Tol::witness()).volume;
        assert!(
            (v - want).abs() <= 1e-12 * want,
            "{what}: volume {v} against the closed form {want}"
        );
    }
}

#[test]
fn a_rod_unions_with_a_half_ball_on_its_cap() {
    unions_at("half ball", &rod(), &half_ball(), 2.0 * PI + 2.0 / 3.0 * PI);
}

#[test]
fn a_rod_unions_with_a_frustum_on_its_cap() {
    let frustum_volume = PI / 3.0 * (1.0 + 0.8 + 0.64);
    unions_at("frustum", &rod(), &frustum(), 2.0 * PI + frustum_volume);
}
