//! **The cylinder wall arm's per-hit membership, one row per bounding
//! side.** [`point_on_wall_in_face`] reads a wall face through its
//! resolved [`WallOutline`]; these rows hand it outlines directly, so each
//! side of each bounding plane is asked on its own, including in the band
//! where the arm must escalate rather than pick a side.
//!
//! The wall is the unit cylinder about `z`; the two planes are the cut
//! cylinder's: the rim plane `z = 2.5` (the face below it) and the section
//! plane through `(0, 0, 1.25)` tilted 0.3 rad about `y` (the face above
//! it), whose height on the ruling at azimuth `θ` is
//! `1.25 − tan 0.3 · cos θ`.

#![allow(clippy::unwrap_used, clippy::panic)]

use super::*;
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A height offset strictly inside the band's gap: past the coincidence
/// threshold, short of the escalation one.
fn in_band() -> f64 {
    let tol = Tol::witness();
    tol.eps() * (1.0 + tol.k()) / 2.0
}

fn z() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}

fn tilt() -> Vec3<f64> {
    Vec3::new(0.3f64.sin(), 0.0, 0.3f64.cos())
}

/// The section plane's height on the ruling at `theta`.
fn section_height(theta: f64) -> f64 {
    1.25 - 0.3f64.tan() * theta.cos()
}

fn sections() -> WallOutline<f64> {
    let plane = |point: Point3<f64>, normal: Vec3<f64>, side: f64| WallSection {
        point,
        normal,
        normal_dot_axis: normal.dot(z()),
        side,
    };
    WallOutline::Sections([
        plane(Point3::new(0.0, 0.0, 2.5), z(), -1.0),
        plane(Point3::new(0.0, 0.0, 1.25), tilt(), 1.0),
    ])
}

fn on_wall(theta: f64, height: f64) -> Point3<f64> {
    Point3::new(theta.cos(), theta.sin(), height)
}

fn ask(
    outline: &WallOutline<f64>,
    az: (f64, f64),
    p: Point3<f64>,
) -> Result<Option<bool>, PointInSolidError> {
    point_on_wall_in_face(
        FaceKey::default(),
        Point3::new(0.0, 0.0, 0.0),
        z(),
        1.0,
        Vec3::new(1.0, 0.0, 0.0),
        az,
        outline,
        p,
        band(),
    )
}

const UPPER_HALF: (f64, f64) = (0.0, core::f64::consts::PI);

#[test]
fn between_the_two_planes_is_on_the_face() {
    let theta = core::f64::consts::FRAC_PI_2;
    assert_eq!(
        ask(&sections(), UPPER_HALF, on_wall(theta, 2.0)).unwrap(),
        Some(true)
    );
}

#[test]
fn past_the_rim_plane_is_off_the_face() {
    let theta = core::f64::consts::FRAC_PI_2;
    assert_eq!(
        ask(&sections(), UPPER_HALF, on_wall(theta, 2.6)).unwrap(),
        Some(false)
    );
}

/// The over-cover the vertex rectangle made: its height range reaches
/// down to the section's lowest vertex, `1.25 − tan 0.3 ≈ 0.941`, on
/// every ruling, but at `θ = 2.8` the section stands at ≈ 1.541, so a
/// hit at 1.2 there is below it and off the face.
#[test]
fn below_the_section_where_it_stands_high_is_off_the_face() {
    let theta = 2.8;
    assert!(section_height(theta) > 1.5);
    assert_eq!(
        ask(&sections(), UPPER_HALF, on_wall(theta, 1.2)).unwrap(),
        Some(false)
    );
}

/// The under-cover: on the window `(−π/2, π/2)` both vertices on the
/// section sit at 1.25, but the section dips to ≈ 0.941 at `θ = 0`, so
/// a hit at 1.1 near there is on the face though below every vertex.
#[test]
fn above_the_section_below_every_vertex_is_on_the_face() {
    let half = core::f64::consts::FRAC_PI_2;
    let theta = 0.1;
    assert!(section_height(theta) < 1.0);
    assert_eq!(
        ask(&sections(), (-half, half), on_wall(theta, 1.1)).unwrap(),
        Some(true)
    );
}

#[test]
fn outside_the_azimuth_window_is_off_the_face() {
    assert_eq!(
        ask(&sections(), UPPER_HALF, on_wall(-1.0, 2.0)).unwrap(),
        Some(false)
    );
}

/// ON the section, to the bits the arithmetic allows: a trim-boundary
/// graze, which the ray schedule retries and the pre-pass reads as
/// on-boundary.
#[test]
fn on_the_section_is_a_graze() {
    let theta = 1.0;
    assert_eq!(
        ask(
            &sections(),
            UPPER_HALF,
            on_wall(theta, section_height(theta))
        )
        .unwrap(),
        None
    );
}

/// In the band's gap on either side of the section, and under the rim
/// plane: the arm escalates, naming its own trim row, rather than
/// deciding which side the hit is on.
#[test]
fn in_band_of_a_bounding_plane_escalates() {
    let theta = 1.0;
    for p in [
        on_wall(theta, section_height(theta) + in_band()),
        on_wall(theta, section_height(theta) - in_band()),
        on_wall(theta, 2.5 - in_band()),
    ] {
        match ask(&sections(), UPPER_HALF, p) {
            Err(PointInSolidError::Escalated { diag, .. }) => {
                assert_eq!(diag.predicate, Some("bool_wall_trim"), "{p:?}");
            }
            other => panic!("{p:?}: {other:?}"),
        }
    }
}

/// A wall outside the class refuses only a hit the face could hold: one
/// definitely outside the ball holding its loop, or outside its azimuth
/// window, is still a miss.
#[test]
fn an_unreadable_outline_refuses_only_within_its_reach() {
    let outline = WallOutline::Unsupported {
        anchor: Point3::new(1.0, 0.0, 1.0),
        reach: 1.6,
    };
    let theta = core::f64::consts::FRAC_PI_2;
    assert!(matches!(
        ask(&outline, UPPER_HALF, on_wall(theta, 1.5)),
        Err(PointInSolidError::WallOutlineUnsupported { .. })
    ));
    // Past the ball, inside the window.
    assert_eq!(
        ask(&outline, UPPER_HALF, on_wall(theta, 2.5)).unwrap(),
        Some(false)
    );
    // Inside the ball, outside the window.
    assert_eq!(
        ask(&outline, UPPER_HALF, on_wall(-0.3, 1.0)).unwrap(),
        Some(false)
    );
}

/// The shared cosine-window sites [`point_on_wall_in_face`]'s inventory
/// lists — the count a reader checks that list against, held here so
/// a new caller cannot land without it.
#[test]
fn the_shared_window_sites_are_the_five_listed() {
    let calls = include_str!("solid_contain.rs")
        .lines()
        .filter(|l| {
            let l = l.trim_start();
            !l.starts_with("//") && l.contains("chart_azimuth_margin(") && !l.contains("fn ")
        })
        .count();
    assert_eq!(calls, 5);
}
