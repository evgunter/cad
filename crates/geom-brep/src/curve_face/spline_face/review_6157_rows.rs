//! Review rows for PR 6157's sector ([`super::Pair::sector`]).
//!
//! A sheet `S(u, v) = (u − ½, v + h(u − ½), σ v)` over `[0, 1]²`, its rim
//! `v = 0` the curve `y = h(x)` in the plane `z = 0`, and lines from the
//! rim point `S(½, 0)` (the origin). Every fixture is read in the four
//! chart orientations (each parameter run either way) and from either end
//! of the span.

#![allow(clippy::unwrap_used, clippy::panic)]

use geom::{Curve3, NurbsSurface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use super::{ChartMeeting, SplineFaceRefusal};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The sheet over `u`-columns `cols` of `(x, h)`, knots `ku`, `z = σ v`,
/// with `u` run backwards where `flip_u` and `v` where `flip_v`.
fn sheet(ku: Vec<f64>, cols: &[(f64, f64)], flip_u: bool, flip_v: bool) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(
        if flip_u {
            ku.iter().rev().map(|k| 1.0 - k).collect()
        } else {
            ku
        },
        2,
    )
    .unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut cols = cols.to_vec();
    if flip_u {
        cols.reverse();
    }
    let vs = if flip_v { [1.0, 0.0] } else { [0.0, 1.0] };
    let control: Vec<Point3<f64>> = cols
        .iter()
        .flat_map(|&(x, h)| vs.map(|v| Point3::new(x, v + h, v)))
        .collect();
    let n = control.len();
    NurbsSurface::new(ku, kv, control, vec![1.0; n]).unwrap()
}

/// `h(x) = −κ x²`: one span, the rim curving to `−y` (off the face) for
/// `κ > 0`, so the rim's tangent turns along it (`w·S_u ≠ 0`, the γ term).
fn parabola(kappa: f64, flip_u: bool, flip_v: bool) -> NurbsSurface<f64> {
    sheet(
        vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        &[
            (-0.5, -0.25 * kappa),
            (0.0, 0.25 * kappa),
            (0.5, -0.25 * kappa),
        ],
        flip_u,
        flip_v,
    )
}

/// A crease at `u = ½` (a double knot), right at the end's foot: flat to
/// its left (`h = 0`), and `h(x) = −0.05 x + 0.5 x²` to its right.
fn kinked(flip_u: bool, flip_v: bool) -> NurbsSurface<f64> {
    sheet(
        vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0],
        &[
            (-0.5, 0.0),
            (-0.25, 0.0),
            (0.0, 0.0),
            (0.25, -0.0125),
            (0.5, 0.1),
        ],
        flip_u,
        flip_v,
    )
}

/// The line from the origin along `dir` over `[0, 0.4]`, or the same
/// segment run toward the origin where `toward`.
fn read(
    chart: &NurbsSurface<f64>,
    dir: Vec3<f64>,
    toward: bool,
) -> Result<Vec<ChartMeeting<f64>>, SplineFaceRefusal> {
    let line = if toward {
        Curve3::Line {
            origin: Point3::new(0.0, 0.0, 0.0) + dir * 0.4,
            dir: -dir,
        }
    } else {
        Curve3::Line {
            origin: Point3::new(0.0, 0.0, 0.0),
            dir,
        }
    };
    crate::NurbsLane::<f64>::certified().chart_meetings(&line, (0.0, 0.4), chart, band())
}

const ORIENTATIONS: [(bool, bool, bool); 8] = [
    (false, false, false),
    (true, false, false),
    (false, true, false),
    (true, true, false),
    (false, false, true),
    (true, false, true),
    (false, true, true),
    (true, true, true),
];

/// The one crossing's distance from the origin along the line.
fn one_at(got: &Result<Vec<ChartMeeting<f64>>, SplineFaceRefusal>) -> Option<f64> {
    match got.as_deref() {
        Ok([m]) => Some((m.point - Point3::new(0.0, 0.0, 0.0)).norm()),
        _ => None,
    }
}

/// **γ is load-bearing.** The line `(1, −0.02, 0.01)` leaves the rim off
/// the face (`w·c < 0`), but the rim (`κ > 0`) bends round under it and
/// the sheet rises to meet it at `s* = 0.03 / κ` from the end, inside the
/// face (`v* = 0.01 s*`). Every such crossing is found, swept across the
/// sector's reaches (so a crossing just inside and just past each).
#[test]
fn review_6157_a_rim_bending_under_a_departing_line_keeps_its_crossing() {
    let dir = Vec3::new(1.0, -0.02, 0.01);
    for s in [0.01, 0.03, 0.05, 0.1, 0.2, 0.3] {
        let kappa = 0.03 / s;
        for (fu, fv, toward) in ORIENTATIONS {
            let got = read(&parabola(kappa, fu, fv), dir, toward);
            assert!(
                one_at(&got).is_some_and(|d| (d - s * dir.norm()).abs() < 1e-6),
                "s* = {s}, flips ({fu}, {fv}), toward {toward}: found at s*, got {got:?}"
            );
        }
    }
}

/// The same line where the rim bends away (`κ < 0`) meets nothing.
#[test]
fn review_6157_a_rim_bending_away_is_cleared() {
    let dir = Vec3::new(1.0, -0.02, 0.01);
    for (fu, fv, toward) in ORIENTATIONS {
        let got = read(&parabola(-1.0, fu, fv), dir, toward);
        assert!(
            matches!(got.as_deref(), Ok([])),
            "flips ({fu}, {fv}), toward {toward}: cleared, got {got:?}"
        );
    }
}

/// **Into the face's side, then a crossing**: `(1, 0.02, 0.01)` leaves
/// into the face (`w·c > 0`) and the sheet (`κ = −0.5`) meets it at
/// `s* = 0.02`. A sector reading `w` the wrong way round would drop it.
#[test]
fn review_6157_a_line_leaving_into_the_face_keeps_its_crossing() {
    let dir = Vec3::new(1.0, 0.02, 0.01);
    for (fu, fv, toward) in ORIENTATIONS {
        let got = read(&parabola(-0.5, fu, fv), dir, toward);
        assert!(
            one_at(&got).is_some() || matches!(got, Err(SplineFaceRefusal::Unsettled(_))),
            "flips ({fu}, {fv}), toward {toward}: found or refused, never cleared, got {got:?}"
        );
    }
}

/// **Claim 1: across a crease the window must read both sides.** The
/// end's foot is on the crease. Read left of it (flat), the line
/// `(1, −0.02, 0.01)` leaves off the face; right of it the rim drops at
/// slope 0.05 under the line and the sheet meets it at `s* = 0.04`. The
/// crossing is found, or the end refuses; it is never cleared.
#[test]
fn review_6157_a_crease_at_the_foot_reads_both_sides() {
    let dir = Vec3::new(1.0, -0.02, 0.01);
    for (fu, fv, toward) in ORIENTATIONS {
        let got = read(&kinked(fu, fv), dir, toward);
        assert!(
            one_at(&got).is_some() || matches!(got, Err(SplineFaceRefusal::Unsettled(_))),
            "flips ({fu}, {fv}), toward {toward}: found or refused, never cleared, got {got:?}"
        );
    }
}

/// The same, sharper: `(1, 0.04, 0.035)` leaves into the face and the
/// sheet (`κ = −0.2`) meets it at `s* = 0.025`, inside the largest box the
/// sector tries and with the rim's lean small enough there that a sector
/// reading the departure the wrong way round would certify that box.
#[test]
fn review_6157_a_line_leaving_into_the_face_keeps_a_close_crossing() {
    let dir = Vec3::new(1.0, 0.04, 0.035);
    for (fu, fv, toward) in ORIENTATIONS {
        let got = read(&parabola(-0.2, fu, fv), dir, toward);
        assert!(
            one_at(&got).is_some() || matches!(got, Err(SplineFaceRefusal::Unsettled(_))),
            "flips ({fu}, {fv}), toward {toward}: found or refused, never cleared, got {got:?}"
        );
    }
}
