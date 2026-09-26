//! **The cylinder wall arm's per-hit read, one row per bounding side and
//! per junction case.** [`point_on_wall_in_face`] reads a wall face
//! through its resolved [`WallOutline`]; these rows hand it outlines
//! directly, so each side of each piece and each way a hit can meet a
//! junction is asked on its own, including in the band where the arm
//! must graze or escalate rather than pick a side. The outlines that
//! real bodies resolve to are the pinned suite's
//! (`crates/sweep/tests/pis_arc_capped_poses.rs`); the rows at the end
//! resolve one here from a real wall.
//!
//! The wall is the unit cylinder about `z`.

#![allow(clippy::unwrap_used, clippy::panic)]

use super::*;
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A perpendicular offset strictly inside the band's gap: past the
/// coincidence threshold, short of the escalation one.
fn in_band() -> f64 {
    let tol = Tol::witness();
    tol.eps() * (1.0 + tol.k()) / 2.0
}

fn z() -> Vec3<f64> {
    Vec3::new(0.0, 0.0, 1.0)
}

const PI: f64 = core::f64::consts::PI;
const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;

fn rim(height: f64) -> WallPlane<f64> {
    WallPlane {
        point: Point3::new(0.0, 0.0, height),
        normal: z(),
    }
}

/// The plane through `(0, 0, 1.25)` tilted by `t` about `y`, its normal
/// along `+z`.
fn section(t: f64) -> WallPlane<f64> {
    WallPlane {
        point: Point3::new(0.0, 0.0, 1.25),
        normal: Vec3::new(t.sin(), 0.0, t.cos()),
    }
}

/// The height of [`section`]`(t)` on the ruling at `theta`.
fn section_height(t: f64, theta: f64) -> f64 {
    1.25 - t.tan() * theta.cos()
}

/// The height of a plane on the ruling at `theta`.
fn plane_height(plane: &WallPlane<f64>, theta: f64) -> f64 {
    let foot = Point3::new(theta.cos(), theta.sin(), 0.0);
    plane.normal.dot(plane.point - foot) / plane.normal.dot(z())
}

/// A wall between a lower piece and an upper piece over one window,
/// joined by a meridian at each end: the cut cylinder's upper wall when
/// the lower plane is [`section`]`(0.3)` and the upper is [`rim`]`(2.5)`.
///
/// The loop runs the lower piece from `lo` to `hi`, a meridian up, the
/// upper piece from `hi` back to `lo`, a meridian down. Junction 0 is at
/// `hi`, junction 1 at `lo`.
fn between(lower: WallPlane<f64>, upper: WallPlane<f64>, (lo, hi): (f64, f64)) -> WallOutline<f64> {
    let span = |a: f64| {
        let (x, y) = (plane_height(&lower, a), plane_height(&upper, a));
        Some((x.min(y), x.max(y)))
    };
    WallOutline::Chart {
        pieces: vec![
            WallPiece {
                plane: lower,
                az: (lo, hi),
                ends: (1, 0),
            },
            WallPiece {
                plane: upper,
                az: (lo, hi),
                ends: (1, 0),
            },
        ],
        junctions: vec![
            WallJunction {
                az: hi,
                pieces: (0, 1),
                span: span(hi),
            },
            WallJunction {
                az: lo,
                pieces: (1, 0),
                span: span(lo),
            },
        ],
    }
}

fn cut_upper(window: (f64, f64)) -> WallOutline<f64> {
    between(section(0.3), rim(2.5), window)
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

const UPPER_HALF: (f64, f64) = (0.0, PI);

#[test]
fn between_the_two_planes_is_on_the_face() {
    let got = ask(&cut_upper(UPPER_HALF), UPPER_HALF, on_wall(HALF_PI, 2.0));
    assert_eq!(got.unwrap(), Some(true));
}

#[test]
fn past_the_rim_plane_is_off_the_face() {
    let got = ask(&cut_upper(UPPER_HALF), UPPER_HALF, on_wall(HALF_PI, 2.6));
    assert_eq!(got.unwrap(), Some(false));
}

/// The over-cover the vertex rectangle made: its height range reaches
/// down to the section's lowest vertex, `1.25 − tan 0.3 ≈ 0.941`, on
/// every ruling, but at `θ = 2.8` the section stands at ≈ 1.541, so a
/// hit at 1.2 there is below it and off the face.
#[test]
fn below_the_section_where_it_stands_high_is_off_the_face() {
    assert!(section_height(0.3, 2.8) > 1.5);
    let got = ask(&cut_upper(UPPER_HALF), UPPER_HALF, on_wall(2.8, 1.2));
    assert_eq!(got.unwrap(), Some(false));
}

/// The under-cover: on the window `(−π/2, π/2)` both vertices on the
/// section sit at 1.25, but the section dips to ≈ 0.941 at `θ = 0`, so
/// a hit at 1.1 near there is on the face though below every vertex.
#[test]
fn above_the_section_below_every_vertex_is_on_the_face() {
    let window = (-HALF_PI, HALF_PI);
    assert!(section_height(0.3, 0.1) < 1.0);
    let got = ask(&cut_upper(window), window, on_wall(0.1, 1.1));
    assert_eq!(got.unwrap(), Some(true));
}

#[test]
fn outside_the_azimuth_window_is_off_the_face() {
    let got = ask(&cut_upper(UPPER_HALF), UPPER_HALF, on_wall(-1.0, 2.0));
    assert_eq!(got.unwrap(), Some(false));
}

/// ON the section, to the bits the arithmetic allows: a trim-boundary
/// graze, which the ray schedule retries and the pre-pass reads as
/// on-boundary.
#[test]
fn on_the_section_is_a_graze() {
    let p = on_wall(1.0, section_height(0.3, 1.0));
    assert_eq!(ask(&cut_upper(UPPER_HALF), UPPER_HALF, p).unwrap(), None);
}

/// In the band's gap on either side of the section, and under the rim
/// plane — perpendicular distances, which is what the band meters: the
/// arm escalates, naming its own trim row, rather than deciding which
/// side the hit is on.
#[test]
fn in_band_of_a_bounding_plane_escalates() {
    let theta = 1.0;
    let along = in_band() / 0.3f64.cos();
    for p in [
        on_wall(theta, section_height(0.3, theta) + along),
        on_wall(theta, section_height(0.3, theta) - along),
        on_wall(theta, 2.5 - in_band()),
    ] {
        match ask(&cut_upper(UPPER_HALF), UPPER_HALF, p) {
            Err(PointInSolidError::Escalated { diag, .. }) => {
                assert_eq!(diag.predicate, Some("bool_wall_trim"), "{p:?}");
            }
            other => panic!("{p:?}: {other:?}"),
        }
    }
}

/// **A steep section is judged by distance, not height.** At
/// `n̂·â = 0.05` a hit half the coincidence threshold off the plane sits
/// ten thresholds away along its ruling. It is ON the section to this
/// band, so it grazes or escalates; it is never put on a side.
#[test]
fn half_a_threshold_off_a_steep_section_never_answers() {
    let t = 0.05f64.acos();
    let window = (HALF_PI - 0.05, HALF_PI + 0.05);
    let outline = between(section(t), rim(40.0), window);
    let eps = Tol::witness().eps();
    for off in [0.5 * eps, -0.5 * eps] {
        let p = on_wall(HALF_PI, section_height(t, HALF_PI) + off / 0.05);
        match ask(&outline, window, p) {
            Ok(None) | Err(PointInSolidError::Escalated { .. }) => {}
            other => panic!("{off}: {other:?}"),
        }
    }
}

/// A stepped (L-shaped) iso outline over `(0, π)`: the floor at 0 up
/// to `π/2`, a meridian step up to 1, the floor at 1 on to `π`, the
/// roof at 2.5 all the way back.
fn stepped() -> WallOutline<f64> {
    WallOutline::Chart {
        pieces: vec![
            WallPiece {
                plane: rim(0.0),
                az: (0.0, HALF_PI),
                ends: (2, 0),
            },
            WallPiece {
                plane: rim(1.0),
                az: (HALF_PI, PI),
                ends: (0, 1),
            },
            WallPiece {
                plane: rim(2.5),
                az: (0.0, PI),
                ends: (2, 1),
            },
        ],
        junctions: vec![
            WallJunction {
                az: HALF_PI,
                pieces: (0, 1),
                span: Some((0.0, 1.0)),
            },
            WallJunction {
                az: PI,
                pieces: (1, 2),
                span: Some((1.0, 2.5)),
            },
            WallJunction {
                az: 0.0,
                pieces: (2, 0),
                span: Some((0.0, 2.5)),
            },
        ],
    }
}

/// The step's notch is off the face, the rest on it: the rectangle its
/// vertices span would have held the notch.
#[test]
fn a_stepped_outline_reads_its_notch() {
    let got = |theta, h| ask(&stepped(), UPPER_HALF, on_wall(theta, h)).unwrap();
    assert_eq!(got(PI / 4.0, 0.5), Some(true));
    assert_eq!(got(3.0 * PI / 4.0, 0.5), Some(false));
    assert_eq!(got(3.0 * PI / 4.0, 2.0), Some(true));
}

/// **At a junction the hit is decided there, never by picking a
/// piece.** On the step's azimuth, or in band of it on either side:
/// inside the meridian's span the hit is on the meridian (a graze);
/// above both floors it is decided as just past the junction, which is
/// on the face; above the roof it is off it.
#[test]
fn a_hit_on_a_junction_azimuth_is_decided_at_the_junction() {
    let nudge = Tol::witness().eps() / 2.0;
    for theta in [HALF_PI, HALF_PI + nudge, HALF_PI - nudge] {
        let got = |h| ask(&stepped(), UPPER_HALF, on_wall(theta, h)).unwrap();
        assert_eq!(got(0.5), None, "{theta}: on the step");
        assert_eq!(got(2.0), Some(true), "{theta}: above both floors");
        assert_eq!(got(2.6), Some(false), "{theta}: above the roof");
    }
}

/// A wall outside the class refuses only a hit the face could hold: one
/// definitely outside the ball holding its loop, or outside its azimuth
/// window, is still a miss.
#[test]
fn an_unreadable_outline_refuses_only_within_its_reach() {
    let outline = WallOutline::Unsupported {
        reach: Some((Point3::new(1.0, 0.0, 1.0), 1.6)),
    };
    assert!(matches!(
        ask(&outline, UPPER_HALF, on_wall(HALF_PI, 1.5)),
        Err(PointInSolidError::WallOutlineUnsupported { .. })
    ));
    // Past the ball, inside the window.
    assert_eq!(
        ask(&outline, UPPER_HALF, on_wall(HALF_PI, 2.5)).unwrap(),
        Some(false)
    );
    // Inside the ball, outside the window.
    assert_eq!(
        ask(&outline, UPPER_HALF, on_wall(-0.3, 1.0)).unwrap(),
        Some(false)
    );
}

/// A ringed wall carries no ball, since its outer loop need not hold
/// it: a hit far from every boundary vertex but inside the window
/// refuses, and only the window answers a miss.
#[test]
fn a_ringed_outline_trusts_only_its_window() {
    let outline = WallOutline::Unsupported { reach: None };
    assert!(matches!(
        ask(&outline, UPPER_HALF, on_wall(HALF_PI, 100.0)),
        Err(PointInSolidError::WallOutlineUnsupported { .. })
    ));
    assert_eq!(
        ask(&outline, UPPER_HALF, on_wall(-0.3, 1.0)).unwrap(),
        Some(false)
    );
}

/// **A wall no ray reaches never has its class asked.** A real wall
/// (a [`cyl_wall_sheet`](crate::test_support_fixtures::cyl_wall_sheet)
/// grown beside a brick) resolves to the rectangle class. Put its
/// cylinder's radius in band of its rims' and the class escalates when
/// it is asked. A query inside the brick, whose rays never meet that
/// wall's window, still answers: the class is resolved per hit, not per
/// body.
#[test]
fn a_wall_no_ray_reaches_never_escalates_the_query() {
    use crate::test_support_fixtures::{CylFrame, brick, cyl_wall_sheet};
    let tol = Tol::witness();
    let mut body: Body<f64> = brick((10.0, 11.0), (10.0, 11.0), (0.0, 1.0), tol);
    let wall = cyl_wall_sheet(
        &mut body,
        CylFrame::canonical(1.0),
        None,
        (0.5, 2.0),
        (0.0, 1.0),
        tol,
    );
    let outline = |body: &Body<f64>| {
        let surface = body.get_face(wall).unwrap().surface;
        let Some(&Surface::Cylinder {
            origin,
            axis,
            radius,
            ..
        }) = body.get_surface(surface)
        else {
            panic!("the sheet is a cylinder wall");
        };
        let (az, h) = cylinder_chart_trim(body, wall, origin, axis, band()).unwrap();
        wall_outline(body, wall, origin, axis, radius, az, h, band())
    };
    assert!(matches!(outline(&body), Ok(WallOutline::Rectangle { .. })));
    let surface = body.get_face(wall).unwrap().surface;
    if let Some(Surface::Cylinder { radius, .. }) = body.surfaces.get_mut(surface) {
        *radius += in_band();
    }
    assert!(matches!(
        outline(&body),
        Err(PointInSolidError::Escalated { .. })
    ));
    let q = Point3::new(10.5, 10.5, 0.5);
    assert_eq!(
        point_in_solid(&body, q, band(), tol).unwrap(),
        SolidContainment::In
    );
}

/// **A junction the hit cannot be put on one side of escalates.** From
/// a resolved outline this cannot happen: near a vertex without a
/// meridian both incident planes pass within the band of the hit, and a
/// meridian run's span catches a hit between its ends. So the branch is
/// a guard, and this row holds it: the step with its meridian span
/// dropped, asked between the two floors on the step's azimuth.
#[test]
fn a_hit_between_a_junctions_pieces_escalates() {
    let WallOutline::Chart {
        pieces,
        mut junctions,
    } = stepped()
    else {
        unreachable!("the stepped outline is a chart");
    };
    junctions[0].span = None;
    let outline = WallOutline::Chart { pieces, junctions };
    match ask(&outline, UPPER_HALF, on_wall(HALF_PI, 0.5)) {
        Err(PointInSolidError::Escalated { diag, .. }) => {
            assert_eq!(diag.predicate, Some("bool_wall_junction"));
        }
        other => panic!("{other:?}"),
    }
}
