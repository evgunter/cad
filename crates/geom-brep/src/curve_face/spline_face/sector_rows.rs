//! The sector at an end ([`super::Pair::sector`]): a line ending on the
//! rim of the full rational cylinder `x² + y² = 1`, `z ∈ [0, 1]`, at its
//! rim `z = 0`. Leaving the rim straight down it is tangent to the wall
//! there and runs off its side, and is cleared, on a crease of the chart
//! (the joint of two arcs) and off one. Leaving up along the wall, or up
//! into the cylinder, it departs to the face's side, and is never
//! cleared.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::FRAC_1_SQRT_2;

use geom::{Curve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use super::{ChartMeeting, SplineFaceRefusal};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The full rational cylinder, `u` around it with creases at its arcs'
/// joints `u = ¼, ½, ¾` and its seam at `(1, 0, z)`, `v` up it.
fn cylinder() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(
        vec![
            0.0, 0.0, 0.0, 0.25, 0.25, 0.5, 0.5, 0.75, 0.75, 1.0, 1.0, 1.0,
        ],
        2,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let ring = [
        (1.0, 0.0),
        (1.0, 1.0),
        (0.0, 1.0),
        (-1.0, 1.0),
        (-1.0, 0.0),
        (-1.0, -1.0),
        (0.0, -1.0),
        (1.0, -1.0),
        (1.0, 0.0),
    ];
    let control = ring
        .into_iter()
        .flat_map(|(x, y)| [Point3::new(x, y, 0.0), Point3::new(x, y, 1.0)])
        .collect();
    let weights = (0..9)
        .flat_map(|i| {
            let w = if i % 2 == 0 { 1.0 } else { FRAC_1_SQRT_2 };
            [w, w]
        })
        .collect();
    NurbsSurface::new(ku, kv, control, weights).unwrap()
}

fn read(
    carrier: &Curve3<f64>,
    params: (f64, f64),
) -> Result<Vec<ChartMeeting<f64>>, SplineFaceRefusal> {
    crate::NurbsLane::<f64>::certified().chart_meetings(carrier, params, &cylinder(), band())
}

/// The line from the rim point at chart `u` (`v = 0`) along `dir`, read
/// from the rim, or toward it where `toward`.
fn from_rim(
    u: f64,
    dir: Vec3<f64>,
    toward: bool,
) -> Result<Vec<ChartMeeting<f64>>, SplineFaceRefusal> {
    let at = cylinder().eval(u, 0.0);
    if toward {
        read(
            &Curve3::Line {
                origin: at - dir * 0.5,
                dir,
            },
            (0.0, 0.5),
        )
    } else {
        read(&Curve3::Line { origin: at, dir }, (0.0, 0.5))
    }
}

/// **A span leaving the rim tangent to the wall, off the face's side, is
/// cleared**, at either end of the span, on the crease at `u = ¼` (the
/// klein loop's arc joint) and between creases. Without the sector the
/// walk reads past the end along the wall and refuses.
#[test]
fn a_line_leaving_a_rim_tangent_and_away_from_the_face_is_cleared() {
    for u in [0.25, 0.125] {
        for toward in [false, true] {
            let dir = if toward {
                Vec3::unit_z()
            } else {
                -Vec3::unit_z()
            };
            let got = from_rim(u, dir, toward);
            assert!(
                matches!(got.as_deref(), Ok([])),
                "u = {u}, the rim the span's {}: cleared, got {got:?}",
                if toward { "end" } else { "start" }
            );
        }
    }
}

/// **A span leaving the rim to the face's side is never cleared**. Up
/// along the wall it lies on the face, and refuses. Up and into the
/// cylinder it leaves the wall at once, but on the face's side of the
/// rim, so the sector certifies nothing; on the crease no end's box can
/// read the end either, so it refuses, where a sector read the wrong way
/// round would clear it.
#[test]
fn a_line_leaving_a_rim_into_the_faces_side_is_not_cleared() {
    for u in [0.25, 0.125] {
        let got = from_rim(u, Vec3::unit_z(), false);
        assert!(
            matches!(got, Err(SplineFaceRefusal::Unsettled(_))),
            "u = {u}, up along the wall: a span lying on the face refuses, got {got:?}"
        );
    }
    let at = cylinder().eval(0.25, 0.0);
    let inward = Vec3::new(-at.x, -at.y, 1.0).normalize();
    let got = from_rim(0.25, inward, false);
    assert!(
        matches!(got, Err(SplineFaceRefusal::Unsettled(_))),
        "into the cylinder from the crease: refused, never cleared, got {got:?}"
    );
}

/// A hook extruded along `y ∈ [0, 10]` (`u`): in `xz` a cubic (`v`) that
/// leaves its rim `x = z = 0` straight up, curls over and comes down
/// past the rim's height within a few centimetres.
fn hook() -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let curl = [(0.0, 0.0), (0.0, 0.02), (0.04, 0.02), (0.04, -0.04)];
    let control = [0.0, 10.0]
        .into_iter()
        .flat_map(|y| curl.map(|(x, z)| Point3::new(x, y, z)))
        .collect();
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
}

/// **A crossing within the sector's reach is the walk's, not the end's.**
/// The line leaves the hook's rim off its side, down and out, and the
/// hook comes down across it a few centimetres on. Read over the window,
/// the face's departure enclosure holds the hook's descent, so no sector
/// reaching the crossing certifies; read at the rim alone it would, and
/// the crossing would be dropped.
#[test]
fn a_crossing_within_the_sectors_reach_is_found() {
    let chart = hook();
    let out = Curve3::Line {
        origin: Point3::new(0.0, 5.0, 0.0),
        dir: Vec3::new(1.0, 0.0, -0.5).normalize(),
    };
    let got = crate::NurbsLane::<f64>::certified().chart_meetings(&out, (0.0, 2.0), &chart, band());
    let found = got.unwrap_or_else(|e| panic!("the crossing reads: {e:?}"));
    assert_eq!(found.len(), 1, "one crossing past the end: {found:?}");
    let m = found[0];
    assert!(
        (m.point - chart.eval(m.uv.0, m.uv.1)).norm() <= band().zero() && m.t > 0.03 && m.t < 0.06,
        "on the hook's descent, a few centimetres on: {m:?}"
    );
}
