//! **A coplanar touch is held to its spread**
//! (`work/onecopy/touch-point-spread.md`; `pose`'s module docs, "A
//! touch's spread"): the subdivision's touch and the two closed forms'
//! read as before where they are well conditioned, and escalate or
//! refuse on `curve_pair_pose_touch_spread` where the point they report
//! could lie further than the band from another reading of it.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::TAU;

use geom::Curve3;
use geom_core::{Band, Point3};

use super::meet_rows::{circle, flat_ellipse, line};
use super::{MeetFault, Meetings, Span, meet};
use crate::roots::Unsettled;

const SPREAD: &str = "curve_pair_pose_touch_spread";

fn band(eps: f64) -> Band {
    Band::new(eps, 10.0 * eps).unwrap()
}

/// The three coplanar touches, each scaled by `at`: an ellipse `(2, 1)`
/// about `(at, at)` and one `(1, 0.5)` inside it, touching at
/// `(at, at + 1)` (the subdivision); a circle of radius `at` about the
/// origin and one of a quarter that inside it, touching at `(at, 0)`;
/// and that circle beside the line `x = at`, touching there (the closed
/// forms). Each pair's spans, and where it touches.
fn touches(
    at: f64,
) -> [(
    &'static str,
    Curve3<f64>,
    (f64, f64),
    Curve3<f64>,
    (f64, f64),
    Point3<f64>,
); 3] {
    [
        (
            "the subdivision's ellipses",
            flat_ellipse([at, at], [1.0, 0.0], 2.0, 1.0),
            (0.0, TAU),
            flat_ellipse([at, at + 0.5], [1.0, 0.0], 1.0, 0.5),
            (0.0, TAU),
            Point3::new(at, at + 1.0, 0.0),
        ),
        (
            "the closed form's circles",
            circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], at),
            (-1.0, 1.0),
            circle([0.75 * at, 0.0, 0.0], [0.0, 0.0, 1.0], 0.25 * at),
            (-1.0, 1.0),
            Point3::new(at, 0.0, 0.0),
        ),
        (
            "the closed form's circle and line",
            circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], at),
            (-1.0, 1.0),
            line([at, -1.0, 0.0], [0.0, 1.0, 0.0]),
            (0.0, 2.0),
            Point3::new(at, 0.0, 0.0),
        ),
    ]
}

fn read(
    x: &Curve3<f64>,
    tx: (f64, f64),
    y: &Curve3<f64>,
    ty: (f64, f64),
    b: Band,
) -> Result<Meetings<f64>, MeetFault> {
    meet(&Span::new(x, tx), &Span::new(y, ty), b).map_err(|e| e.fault)
}

/// **A well-conditioned touch reads as before**: each of the three at
/// unit scale, at ε = 1e-6, 1e-9 and 1e-12, is one meeting at the touch.
#[test]
fn a_well_conditioned_touch_reads_one_meeting_at_the_touch() {
    for eps in [1e-6, 1e-9, 1e-12] {
        for (what, x, tx, y, ty, at) in touches(1.0) {
            let m = read(&x, tx, &y, ty, band(eps))
                .unwrap_or_else(|e| panic!("ε {eps:e}, {what}: {e:?}"));
            assert!(
                m.points.len() == 1 && (m.points[0].point - at).norm() < eps,
                "ε {eps:e}, {what}: one meeting at the touch: {m:?}"
            );
        }
    }
}

/// **A touch whose spread exceeds the band escalates, and one past it
/// refuses on its meter.** Each of the three scaled out at ε = 1e-12:
/// where twice the spread reads in the band's gap the touch escalates,
/// and where it reads past it the touch refuses on the spread's meter.
/// Red where the spread is not read: each then reads one meeting. The
/// scales are each path's own: the closed forms charge the angle's
/// evaluation at the radius, the subdivision `F′`'s rounding over its
/// least bend.
#[test]
fn a_touch_whose_spread_exceeds_the_band_escalates() {
    let b = band(1e-12);
    // Per path, a scale whose spread reads in the gap and one past it.
    let scales = [(1e4, 1e5), (1e3, 1e4), (5e2, 5e3)];
    for (k, &(gap, beyond)) in scales.iter().enumerate() {
        for (scale, past) in [(gap, false), (beyond, true)] {
            let (what, x, tx, y, ty, _) = &touches(scale)[k];
            let got = read(x, *tx, y, *ty, b);
            let held = match &got {
                Err(MeetFault::Escalated(gap)) => !past && gap.predicate == Some(SPREAD),
                Err(MeetFault::Unsettled(Unsettled::Meter { row })) => past && *row == SPREAD,
                _ => false,
            };
            assert!(held, "scale {scale:e}, {what}: {got:?}");
        }
    }
}
