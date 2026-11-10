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

/// A touching pair: its name, the two carriers and spans, and where it
/// touches.
type Pair = (
    &'static str,
    Curve3<f64>,
    (f64, f64),
    Curve3<f64>,
    (f64, f64),
    Point3<f64>,
);

fn band(eps: f64) -> Band {
    Band::new(eps, 10.0 * eps).unwrap()
}

/// The three coplanar touches, each scaled by `at`: an ellipse `(2, 1)`
/// about `(at, at)` and one `(1, 0.5)` inside it, touching at
/// `(at, at + 1)` (the subdivision); a circle of radius `at` about the
/// origin and one of a quarter that inside it, touching at `(at, 0)`;
/// and that circle beside the line `x = at`, touching there (the closed
/// forms). Each pair's spans, and where it touches.
fn touches(at: f64) -> [Pair; 3] {
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

/// **The subdivision's touch whose spread exceeds the band escalates,
/// and one past it refuses on its meter.** The ellipses scaled out at
/// ε = 1e-12: at `1e4` twice the spread, `F′`'s rounding over the least
/// bend at the carrier's top speed, reads in the band's gap and the
/// touch escalates; at `1e5` it reads past the band and refuses on the
/// spread's meter. Red where the spread is not read: each then reads one
/// meeting.
#[test]
fn the_subdivisions_touch_whose_spread_exceeds_the_band_escalates() {
    let b = band(1e-12);
    for (scale, past) in [(1e4, false), (1e5, true)] {
        let (what, x, tx, y, ty, _) = &touches(scale)[0];
        let got = read(x, *tx, y, *ty, b);
        let held = match &got {
            Err(MeetFault::Escalated(gap)) => !past && gap.predicate == Some(SPREAD),
            Err(MeetFault::Unsettled(Unsettled::Meter { row })) => past && *row == SPREAD,
            _ => false,
        };
        assert!(held, "scale {scale:e}, {what}: {got:?}");
    }
}

/// **A closed form's touch is exact at a point scalar, and its angle's
/// enclosure is its spread at `Interval`** (`[simple.conditioned]`).
/// The closed forms' pairs at unit scale, at `f64`, are scaled out to
/// `1e5` and still read one meeting: the angle is a well-conditioned
/// reading of the harmonic's coefficients and is not charged. At
/// `Interval`, every input widened by `w`:
/// - two circles of radii `1` and `1 − d` touching inside at `(1, 0)`,
///   their centres `d = 10⁻³` apart, so the angle's enclosure is about
///   `w/d` wide, at `w = 10⁻¹²` and ε = 1e-9;
/// - the unit circle beside the line `x = 1`, at `w = 10⁻¹³` and
///   ε = 1e-12.
///
/// The circles escalate on the spread. The line's pose has no
/// cancellation to widen its angle past its extreme, so it escalates on
/// the extreme first (`curve_pair_pose_extreme`), and is pinned as
/// escalating on either. At `w = 10⁻¹⁴` and ε = 1e-9 each reads one
/// meeting. Red where the spread is not read: the widened circles then
/// pass the pose and escalate later, on the meeting's own reading of
/// both spans, not on the spread.
#[test]
fn a_closed_forms_touch_is_held_to_its_angles_enclosure() {
    use geom_core::{Interval, Real};
    for (what, x, tx, y, ty, at) in touches(1e5).into_iter().skip(1) {
        let m = read(&x, tx, &y, ty, band(1e-12))
            .unwrap_or_else(|e| panic!("{what} at f64, far out: {e:?}"));
        assert!(
            m.points.len() == 1 && (m.points[0].point - at).norm() < 1e-9,
            "{what} at f64, far out: one meeting at the touch: {m:?}"
        );
    }
    let iv = |a: f64, b: f64| (Interval::from_f64(a), Interval::from_f64(b));
    let d = 1e-3;
    for (w, eps, escalates) in [
        (1e-12, 1e-9, Some("circles")),
        (1e-13, 1e-12, Some("line")),
        (1e-14, 1e-9, None),
    ] {
        let widen = |x: f64| Interval::from_bounds(x - w, x + w);
        let big = circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0).map_scalar(widen);
        let small = circle([d, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0 - d).map_scalar(widen);
        let beside = line([1.0, -1.0, 0.0], [0.0, 1.0, 0.0]).map_scalar(widen);
        for (what, other, span) in [
            ("circles", &small, iv(-1.0, 1.0)),
            ("line", &beside, iv(0.0, 2.0)),
        ] {
            let got = meet(
                &Span::new(&big, iv(-1.0, 1.0)),
                &Span::new(other, span),
                band(eps),
            )
            .map_err(|e| e.fault);
            let held = match (&got, escalates) {
                (Err(MeetFault::Escalated(gap)), Some(which)) if which == what => {
                    gap.predicate == Some(SPREAD)
                        || (what == "line" && gap.predicate == Some("curve_pair_pose_extreme"))
                }
                (Ok(m), Some(which)) if which != what => m.points.len() == 1,
                (Ok(m), None) => m.points.len() == 1,
                (Err(_), Some(which)) => which != what,
                _ => false,
            };
            assert!(held, "w {w:e}, ε {eps:e}, {what}: {got:?}");
        }
    }
}
