//! **Where a reader of a joint's chart point reads it**
//! ([`Lift::joint_point`], [`Lift::exit_point`], [`Lift::stored_corners`]):
//! a corner is the meet of two held lines, exact at every scalar; a cut
//! between two images on one line is the entering image's own value
//! there. The loops are stated ([`Lift::stated_for_tests`]), each row a
//! mint's image over a span whose end is a computed length, so at
//! `Interval` every image's own end is an enclosure with width.
#![allow(clippy::unwrap_used, clippy::panic, clippy::float_cmp)]

use super::{ChartPeriods, Lift, LiftedRow};
use crate::joint::{Deck, Winding};
use geom::Surface;
use geom_brep::Pcurve;
use geom_core::spline::KnotVector;
use geom_core::{Bounds, Decide, Interval, Point2, Point3, Real};

/// A span end that is a computed length, `1.2 + 1`.
fn length<T: Real>() -> T {
    T::from_f64(1.2) + T::one()
}

/// `p` as `(u, v)` point brackets, `None` for a coordinate with width.
fn points<T: Bounds>(p: Point2<T>) -> (Option<f64>, Option<f64>) {
    let point = |x: T| (x.lo() == x.hi()).then(|| x.lo());
    (point(p.x), point(p.y))
}

/// `p` as its two brackets.
fn brackets<T: Bounds>(p: Point2<T>) -> [(f64, f64); 2] {
    [(p.x.lo(), p.x.hi()), (p.y.lo(), p.y.hi())]
}

/// The row `v` from `u0` to `u1` over `(t0, t1)`, forward.
fn row<T: Decide>(v: f64, (u0, u1): (f64, f64), span: (T, T)) -> LiftedRow<T> {
    let f = T::from_f64;
    LiftedRow::stated_for_tests(Pcurve::iso_row(f(v), (f(u0), f(u1)), span), span, true)
}

/// The column `u` from `v0` to `v1` over `(t0, t1)`, forward.
fn column<T: Decide>(u: f64, (v0, v1): (f64, f64), span: (T, T)) -> LiftedRow<T> {
    let f = T::from_f64;
    LiftedRow::stated_for_tests(Pcurve::iso_column(f(u), (f(v0), f(v1)), span), span, true)
}

fn closed() -> Option<Winding> {
    Some(Winding {
        deck: Deck::IDENTITY,
    })
}

/// **A corner is the meet, and a cut is the map's value**, at `f64` and
/// at `Interval`. The unit square once round, its bottom side cut at a
/// third: every corner's joint point is the two held values, as points;
/// the row that leaves a corner leaves at the same point the next enters
/// at; the cut is the second piece's own value where it enters, inside
/// the chart, and the first piece leaves at its own value there.
///
/// Red under `Lift::joint_point` (or `exit_point`) reading a corner
/// through the image's map: at `Interval` the corner is then an
/// enclosure, no point.
#[test]
fn a_corner_is_the_meet_and_a_cut_is_the_maps_value() {
    // One body per scalar, by macro: the rows read brackets and decide
    // nothing, and name no compound bound to do it.
    macro_rules! at {
        ($t:ty, $scalar:expr) => {{
            let scalar: &str = $scalar;
            let (zero, len) = (<$t>::zero(), length::<$t>());
            let cut = len / <$t>::from_f64(3.0);
            // The bottom side's two pieces hold one image over their own spans.
            let mut first = row(0.0, (0.0, 1.0), (zero, len));
            let mut second = first.clone();
            first.params = (zero, cut);
            second.params = (cut, len);
            let lift = Lift::stated_for_tests(
                vec![
                    first,
                    second,
                    column(1.0, (0.0, 1.0), (zero, len)),
                    row(1.0, (1.0, 0.0), (zero, len)),
                    column(0.0, (1.0, 0.0), (zero, len)),
                ],
                closed(),
            );
            // Corners: into rows 0, 2, 3, 4.
            for (i, corner) in [
                (0, (0.0, 0.0)),
                (2, (1.0, 0.0)),
                (3, (1.0, 1.0)),
                (4, (0.0, 1.0)),
            ] {
                assert_eq!(
                    points(lift.joint_point(i)),
                    (Some(corner.0), Some(corner.1)),
                    "{scalar}: the joint into row {i} is the meet of its two lines"
                );
                let before = (i + 4) % 5;
                assert_eq!(
                    brackets(lift.exit_point(before)),
                    brackets(lift.joint_point(i)),
                    "{scalar}: row {before} leaves at the corner row {i} enters at"
                );
            }
            // The cut: the entering piece's own value, and the leaving piece's.
            let rows = lift.rows();
            let entering = rows[1].pcurve.eval(cut);
            assert_eq!(
                brackets(lift.joint_point(1)),
                brackets(entering),
                "{scalar}: the cut is the entering image's value there"
            );
            assert_eq!(
                brackets(lift.exit_point(0)),
                brackets(rows[0].pcurve.eval(cut)),
                "{scalar}: the piece before the cut leaves at its own value"
            );
            assert!(
                entering.x.lo() > 0.0 && entering.x.hi() < 1.0,
                "{scalar}: the cut is no chart end"
            );
            // What the meet replaces: the image's own end over a computed span.
            let own_end = rows[2].pcurve.eval(len);
            assert!(
                own_end.y.lo() <= 1.0 && 1.0 <= own_end.y.hi(),
                "{scalar}: the column's own end encloses the corner"
            );
        }};
    }
    at!(f64, "f64");
    at!(Interval, "Interval");
    // At `Interval` an image's own end over a computed span has width:
    // the corner is exact only as a meet.
    let len = length::<Interval>();
    let end = Pcurve::iso_column(
        Interval::one(),
        (Interval::zero(), Interval::one()),
        (Interval::zero(), len),
    )
    .eval(len);
    assert!(end.y.lo() < end.y.hi(), "the map's end is an enclosure");
}

/// The bilinear patch over `[lo, hi] × [0, 1]`.
fn patch<T: Decide>(lo: f64, hi: f64) -> Surface<T> {
    let ku = KnotVector::clamped(vec![lo, lo, hi, hi], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control = [(0.0, 0.0), (0.0, 1.0), (1.0, 0.0), (1.0, 1.0)]
        .map(|(x, y)| Point3::new(x, y, 0.0))
        .to_vec();
    let net = geom::NurbsSurface::new(ku, kv, control, vec![1.0; 4]).unwrap();
    Surface::Nurbs(std::sync::Arc::new(net.map_scalar(T::from_f64)))
}

/// **A corner read on the image's own branch undoes the row's deck.** On
/// a chart whose `u` domain is `[0.4, 1.7]`, a bottom row stored from
/// `u = 0.4` and lifted one period starts, in the lift, at the seam's
/// column `u = 1.7`. Its stored corner there is the domain's low end
/// `0.4` itself, at both scalars: the meet `1.7` moved back one period
/// of a period stated by its ends. A row the lift does not move reads
/// its meets as they are.
///
/// Red under `Lift::stored_corners` leaving the meet un-moved (the
/// corner reads `1.7`), and under `Period::moved` subtracting the
/// period's length (`1.7 − 1.3` is no point at `Interval`).
#[test]
fn a_stored_corner_undoes_the_rows_own_deck() {
    // One body per scalar, by macro: the rows read brackets and decide
    // nothing, and name no compound bound to do it.
    macro_rules! at {
        ($t:ty, $scalar:expr) => {{
            let scalar: &str = $scalar;
            let (lo, hi) = (0.4, 1.7);
            let surface = patch::<$t>(lo, hi);
            let periods = ChartPeriods::stated(&surface);
            let span = (<$t>::zero(), length::<$t>());
            let once = Deck {
                u: 1,
                v: 0,
                twin: false,
            };
            // The stored bottom row, and the same row as the lift moves it.
            let stored = row::<$t>(0.0, (lo, hi), span);
            let mut lifted = stored.clone().moved_for_tests(once);
            lifted.pcurve = once.apply(&stored.pcurve, &surface, periods).unwrap();
            let seam = column::<$t>(hi, (1.0, 0.0), span);
            let far = column::<$t>(3.0, (0.0, 1.0), span);
            let top = row::<$t>(1.0, (3.0, hi), span);
            let lift = Lift::stated_for_tests(vec![lifted, far, top, seam], closed());
            let [start, _] = lift.stored_corners(0, periods);
            let start = start.expect("the row meets the seam's column at its start");
            assert_eq!(
                (start.lo(), start.hi()),
                (lo, lo),
                "{scalar}: the corner at the lifted seam is the stored branch's low end"
            );
            // An unmoved row's corners are its meets.
            let plain = Lift::stated_for_tests(
                vec![
                    row::<$t>(0.0, (lo, hi), span),
                    column::<$t>(hi, (0.0, 1.0), span),
                    row::<$t>(1.0, (hi, lo), span),
                    column::<$t>(lo, (1.0, 0.0), span),
                ],
                closed(),
            );
            let [a, b] = plain.stored_corners(0, periods).map(|c| c.unwrap());
            assert_eq!(
                ((a.lo(), a.hi()), (b.lo(), b.hi())),
                ((lo, lo), (hi, hi)),
                "{scalar}: an unmoved row spans its domain exactly"
            );
        }};
    }
    at!(f64, "f64");
    at!(Interval, "Interval");
}
