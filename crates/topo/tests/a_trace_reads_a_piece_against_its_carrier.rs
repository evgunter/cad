//! **The trace door** (`topo::boolean::trace`), read on carriers whose
//! answers are known in closed form: where a line or a circle crosses a
//! carrier, in which sense, the face's outward gradient there, and
//! whether a plane misses a carrier at all. A piece on the carrier and
//! a piece tangent to it refuse.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::boolean::trace::{
    Passage, TraceFault, carrier_extent, circle_crossings, plane_misses, ray_crossings,
};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, 1.0),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn floor() -> Surface<f64> {
    Surface::Plane {
        origin: Point3::origin(),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

fn near(a: f64, b: f64, what: &str) {
    assert!((a - b).abs() <= 1e-12, "{what}: {a} against {b}");
}

/// A line through a unit ball enters at one and leaves at three, nearest
/// first, the outward gradient pointing out of the ball at each; the
/// face's sense reversed, the ball is the hole, and the passages swap.
#[test]
fn a_line_enters_and_leaves_a_ball_in_the_faces_sense() {
    let ray = |sense| {
        ray_crossings(
            &sphere(),
            sense,
            Point3::new(-2.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            4.0,
            band(),
        )
        .expect("two crossings")
    };
    let out = ray(true);
    assert_eq!(
        out.iter().map(|c| c.passage).collect::<Vec<_>>(),
        [Passage::Enter, Passage::Leave]
    );
    near(out[0].along, 1.0, "the entry");
    near(out[1].along, 3.0, "the exit");
    near(out[0].point.x, -1.0, "the entry's point");
    assert!(
        out[0].outward.x < 0.0 && out[1].outward.x > 0.0,
        "out of the ball"
    );
    let hole = ray(false);
    assert_eq!(
        hole.iter().map(|c| c.passage).collect::<Vec<_>>(),
        [Passage::Leave, Passage::Enter]
    );
    assert!(
        hole[0].outward.x > 0.0,
        "into the ball, out of the hole's material"
    );
}

/// A circle about the floor's `x` axis, from `(0, 1, 0)`, turns up and
/// over and crosses the floor once, half a turn on, entering the floor's
/// material below it; the whole turn back to the pen is not a crossing.
#[test]
fn a_circle_crosses_a_plane_half_a_turn_on() {
    let out = circle_crossings(
        &floor(),
        true,
        (
            Point3::origin(),
            Vec3::new(1.0, 0.0, 0.0),
            1.0,
            Vec3::new(0.0, 1.0, 0.0),
        ),
        band(),
    )
    .expect("one crossing");
    assert_eq!(out.len(), 1, "{out:?}");
    near(out[0].along, std::f64::consts::PI, "half a turn");
    near(out[0].point.y, -1.0, "at (0, -1, 0)");
    assert_eq!(out[0].passage, Passage::Enter);
}

/// A line in the floor lies on it, and a line grazing the ball touches
/// it once: both refuse.
#[test]
fn a_piece_on_or_tangent_to_a_carrier_refuses() {
    let on = ray_crossings(
        &floor(),
        true,
        Point3::origin(),
        Vec3::new(1.0, 0.0, 0.0),
        1.0,
        band(),
    );
    assert_eq!(on.map(|c| c.len()), Err(TraceFault::OnCarrier));
    let grazing = ray_crossings(
        &sphere(),
        true,
        Point3::new(-2.0, 1.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        4.0,
        band(),
    );
    assert_eq!(grazing.map(|c| c.len()), Err(TraceFault::Grazing));
}

/// A plane misses a ball beyond its radius and a torus beyond its tube
/// (along the axis) or its spine's shadow and tube (across it); one
/// carrier-extent table answers both, and the largest reach of a torus
/// in any direction is its spine and tube together.
#[test]
fn a_plane_misses_a_carrier_beyond_its_extent() {
    let misses = |surface: &Surface<f64>, at: [f64; 3], normal: [f64; 3]| {
        plane_misses(
            surface,
            Point3::new(at[0], at[1], at[2]),
            Vec3::new(normal[0], normal[1], normal[2]),
            band(),
        )
        .expect("decided")
    };
    let up = [0.0, 0.0, 1.0];
    let across = [1.0, 0.0, 0.0];
    assert!(misses(&sphere(), [0.0, 0.0, 1.5], up));
    assert!(!misses(&sphere(), [0.0, 0.0, 0.5], up));
    assert!(misses(&torus(), [0.0, 0.0, 0.75], up));
    assert!(!misses(&torus(), [0.0, 0.0, 0.25], up));
    assert!(misses(&torus(), [2.75, 0.0, 0.0], across));
    assert!(!misses(&torus(), [2.25, 0.0, 0.0], across));
    assert!(misses(&floor(), [0.0, 0.0, 1.0], up));
    assert!(!misses(&floor(), [0.0, 0.0, 0.0], up));
    assert!(!misses(&floor(), [0.0, 0.0, 1.0], across));

    let (_, everywhere) = carrier_extent(&torus(), None).expect("closed form");
    near(everywhere, 2.5, "spine and tube");
    let (_, along_axis) =
        carrier_extent(&torus(), Some(Vec3::new(0.0, 0.0, 1.0))).expect("closed form");
    near(along_axis, 0.5, "the tube alone along the axis");
}
