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

/// The admission rule holds `(ε, K = 10)` ([`geom_core::tolerance::admissible`]).
fn admitted(eps: f64) -> Band {
    assert!(
        geom_core::tolerance::admissible(eps, 10.0),
        "ε {eps:e} is admitted"
    );
    band(eps)
}

/// What a pin expects a touch to read.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Expect {
    /// One meeting.
    One,
    /// An escalation on the spread.
    Spread,
    /// A refusal on the spread's meter, past the band.
    Past,
    /// An escalation on the extreme, before the spread is read.
    Extreme,
}

fn held<T: geom_core::Real>(got: &Result<Meetings<T>, MeetFault>, expect: Expect) -> bool {
    match (got, expect) {
        (Ok(m), Expect::One) => m.points.len() == 1,
        (Err(MeetFault::Escalated(gap)), Expect::Spread) => gap.predicate == Some(SPREAD),
        (Err(MeetFault::Unsettled(Unsettled::Meter { row })), Expect::Past) => *row == SPREAD,
        (Err(MeetFault::Escalated(gap)), Expect::Extreme) => {
            gap.predicate == Some("curve_pair_pose_extreme")
        }
        _ => false,
    }
}

/// **The subdivision's touch whose spread exceeds the band escalates, in
/// an admitted run.** An ellipse `(2, 1)` and a nearly equal one inside
/// it touching at its top vertex, the relative bend low, so the least
/// `G″` on the piece, `c`, is small and the spread `(|G′(θ*)| + e₁)/c`
/// at the top speed is large:
/// - at unit scale (about `(1, 1)`), `(1.98, 0.99)` inside, at
///   ε = 2·10⁻¹³ (just above ε_min at K = 10), escalates on the spread;
/// - `100` m out, `(1.9, 0.95)` inside, at ε = 1e-12, escalates on the
///   spread, and `(1.98, 0.99)` there refuses on its meter, past the
///   band;
/// - `(1, 0.5)` inside, a higher relative bend, reads one meeting at
///   both.
///
/// Every scale is in the session box and every ε admitted. Red where the
/// spread is not read: each then reads one meeting.
#[test]
fn the_subdivisions_touch_whose_spread_exceeds_the_band_escalates() {
    for (far, (a, b), eps, expect) in [
        (1.0, (1.98, 0.99), 2e-13, Expect::Spread),
        (1.0, (1.0, 0.5), 2e-13, Expect::One),
        (1e2, (1.9, 0.95), 1e-12, Expect::Spread),
        (1e2, (1.98, 0.99), 1e-12, Expect::Past),
        (1e2, (1.0, 0.5), 1e-12, Expect::One),
    ] {
        let outer = flat_ellipse([far, far], [1.0, 0.0], 2.0, 1.0);
        let inner = flat_ellipse([far, far + 1.0 - b], [1.0, 0.0], a, b);
        let got = read(&outer, (0.0, TAU), &inner, (0.0, TAU), admitted(eps));
        assert!(
            held(&got, expect),
            "{far:e} out, ({a}, {b}) inside, ε {eps:e}: {expect:?}, read {got:?}"
        );
    }
}

/// **A closed form's touch is exact at a point scalar, and its angle's
/// enclosure is its spread at `Interval`** (`[simple.conditioned]`).
/// At `f64`, the closed forms' pairs `1e3` out (the session box's edge)
/// read one meeting at ε = 1e-12: the angle is a well-conditioned reading
/// of the harmonic's coefficients and is not charged. At `Interval`,
/// every input widened by `w`, at admitted ε:
/// - two circles of radii `1` and `1 − d` touching inside at `(1, 0)`,
///   their centres `d = 10⁻³` apart, so the angle's enclosure is about
///   `w/d` wide: at `w = 10⁻¹²` and ε = 1e-9 the touch escalates on the
///   spread;
/// - an ellipse `(10, 1)` beside the line along its major axis at its
///   minor vertex, whose angle's enclosure is about `a/b` of `w` wide
///   (the swing is the minor semi-axis): at `w = 10⁻¹⁴` and ε = 1e-12
///   the line conic's arm escalates on the spread, before its extreme;
/// - the unit circle beside a line, with no such leverage: at
///   `w = 10⁻¹³` and ε = 1e-12 its extreme escalates first;
/// - each at `w = 10⁻¹⁵` and ε = 1e-9 reads one meeting.
///
/// Red where the spread is not read: the circles and the ellipse then
/// pass the pose.
#[test]
fn a_closed_forms_touch_is_held_to_its_angles_enclosure() {
    use geom_core::{Interval, Real};
    for (what, x, tx, y, ty, at) in touches(1e3).into_iter().skip(1) {
        let m = read(&x, tx, &y, ty, admitted(1e-12))
            .unwrap_or_else(|e| panic!("{what} at f64, at the box's edge: {e:?}"));
        assert!(
            m.points.len() == 1 && (m.points[0].point - at).norm() < 1e-9,
            "{what} at f64, at the box's edge: one meeting at the touch: {m:?}"
        );
    }
    let iv = |a: f64, b: f64| (Interval::from_f64(a), Interval::from_f64(b));
    let d = 1e-3;
    for (what, w, eps, expect) in [
        ("circles", 1e-12, 1e-9, Expect::Spread),
        ("circles", 1e-15, 1e-9, Expect::One),
        ("ellipse and line", 1e-14, 1e-12, Expect::Spread),
        ("ellipse and line", 1e-15, 1e-9, Expect::One),
        ("circle and line", 1e-13, 1e-12, Expect::Extreme),
        ("circle and line", 1e-15, 1e-9, Expect::One),
    ] {
        let widen = |x: f64| Interval::from_bounds(x - w, x + w);
        let (x, tx, y, ty) = match what {
            "circles" => (
                circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0),
                iv(-1.0, 1.0),
                circle([d, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0 - d),
                iv(-1.0, 1.0),
            ),
            "ellipse and line" => (
                flat_ellipse([0.0, 0.0], [1.0, 0.0], 10.0, 1.0),
                iv(0.5, 2.5),
                line([-1.0, 1.0, 0.0], [1.0, 0.0, 0.0]),
                iv(0.0, 2.0),
            ),
            _ => (
                circle([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], 1.0),
                iv(-1.0, 1.0),
                line([1.0, -1.0, 0.0], [0.0, 1.0, 0.0]),
                iv(0.0, 2.0),
            ),
        };
        let (x, y) = (x.map_scalar(widen), y.map_scalar(widen));
        let got = meet(&Span::new(&x, tx), &Span::new(&y, ty), admitted(eps)).map_err(|e| e.fault);
        assert!(
            held(&got, expect),
            "{what}, w {w:e}, ε {eps:e}: {expect:?}, read {got:?}"
        );
    }
}
