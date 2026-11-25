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
use geom_core::{Bounds as _, Decide, Interval, Point3, Real};

/// A span end that is a computed length, `1.2 + 1`.
fn length<T: Real>() -> T {
    T::from_f64(1.2) + T::one()
}

/// `p` as `(u, v)` point brackets, `None` for a coordinate with width.
/// A macro, like `brackets!`: the rows read brackets at two scalars and
/// name no bound to do it.
macro_rules! points {
    ($p:expr) => {{
        let p = $p;
        let point = |lo: f64, hi: f64| (lo == hi).then_some(lo);
        (point(p.x.lo(), p.x.hi()), point(p.y.lo(), p.y.hi()))
    }};
}

/// `p` as its two brackets.
macro_rules! brackets {
    ($p:expr) => {{
        let p = $p;
        [(p.x.lo(), p.x.hi()), (p.y.lo(), p.y.hi())]
    }};
}

/// A span neither of whose ends is zero or a literal: `(0.1 + 0.2,
/// 1.2 + 1)`. A mint's map over it reads each of its own ends through a
/// product and a sum, so the end rounds at `f64` and has width at
/// `Interval`: an image's own entry is then never the corner's bits.
fn span<T: Real>() -> (T, T) {
    (T::from_f64(0.1) + T::from_f64(0.2), length())
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
/// Every row is over [`span`], whose ends are computed, and the top row
/// is traversed backward, so each image's own entry and exit is read
/// through its map's arithmetic: at `Interval` an enclosure with width,
/// which the row asserts of each before it reads the corner.
///
/// Red under `Lift::joint_point` reading the row's own entry where there
/// is a meet ("the joint into row 0 is the meet", at `Interval`), and
/// under `Lift::exit_point` reading its own exit ("row 4 leaves at the
/// corner row 0 enters at"): each planted alone.
#[test]
fn a_corner_is_the_meet_and_a_cut_is_the_maps_value() {
    // One body per scalar, by macro: the rows read brackets and decide
    // nothing, and name no compound bound to do it.
    macro_rules! at {
        ($t:ty, $scalar:expr, $width:expr) => {{
            let scalar: &str = $scalar;
            let (t0, t1) = span::<$t>();
            let cut = t0 + (t1 - t0) / <$t>::from_f64(3.0);
            // The bottom side's two pieces hold one image over their own spans.
            let whole = row(0.0, (0.0, 1.0), (t0, t1));
            let first = whole.clone().piece_for_tests((t0, cut));
            let second = whole.piece_for_tests((cut, t1));
            let lift = Lift::stated_for_tests(
                vec![
                    first,
                    second,
                    column(1.0, (0.0, 1.0), (t0, t1)),
                    // The top, stated left to right and run right to left.
                    row(1.0, (0.0, 1.0), (t0, t1)).reversed_for_tests(),
                    column(0.0, (1.0, 0.0), (t0, t1)),
                ],
                closed(),
            );
            // Each image's own entry and exit, through its map: with
            // width at `Interval` on the moving coordinate, so a reader
            // of the map is told apart from a reader of the meet.
            if $width {
                for (i, entry) in [(0, true), (2, true), (3, true), (4, true), (1, false)] {
                    let (a, b) = lift.rows()[i].entry_exit_points();
                    let p = if entry { a } else { b };
                    assert_eq!(
                        (points!(p).0.is_some(), points!(p).1.is_some()),
                        (i % 2 == 0 && i != 0, i == 0 || i % 2 == 1),
                        "{scalar}: row {i}'s own end has width along its line: {:?}",
                        brackets!(p)
                    );
                }
            }
            // Corners: into rows 0, 2, 3, 4.
            for (i, corner) in [
                (0, (0.0, 0.0)),
                (2, (1.0, 0.0)),
                (3, (1.0, 1.0)),
                (4, (0.0, 1.0)),
            ] {
                assert_eq!(
                    points!(lift.joint_point(i)),
                    (Some(corner.0), Some(corner.1)),
                    "{scalar}: the joint into row {i} is the meet of its two lines"
                );
                let before = (i + 4) % 5;
                assert_eq!(
                    points!(lift.exit_point(before)),
                    (Some(corner.0), Some(corner.1)),
                    "{scalar}: row {before} leaves at the corner row {i} enters at"
                );
            }
            // The cut: the entering piece's own value, and the leaving piece's.
            let rows = lift.rows();
            let entering = rows[1].pcurve.eval(cut);
            assert_eq!(
                brackets!(lift.joint_point(1)),
                brackets!(entering),
                "{scalar}: the cut is the entering image's value there"
            );
            assert_eq!(
                brackets!(lift.exit_point(0)),
                brackets!(rows[0].pcurve.eval(cut)),
                "{scalar}: the piece before the cut leaves at its own value"
            );
            assert_eq!(
                (entering.x.lo() > 0.0, entering.x.hi() < 1.0),
                (true, true),
                "{scalar}: the cut is no chart end: {:?}",
                brackets!(entering)
            );
        }};
    }
    at!(f64, "f64", false);
    at!(Interval, "Interval", true);
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

/// `u` moved `k` periods of the chart's first channel.
fn deck(k: i32) -> Deck {
    Deck {
        u: k,
        v: 0,
        twin: false,
    }
}

/// **A row's stored corners are its domain's ends, at both ends, however
/// far the lift moved it.** On a chart whose `u` domain is `[0.4, 1.7]`,
/// a whole-period bottom row stored over `[0.4, 1.7]` and lifted one
/// period runs, in the lift, between the seam's column one period over
/// and the seam's column two periods over. Its two stored corners are
/// `0.4` and `1.7` themselves, as points, at both scalars: each is the
/// neighbouring column's STORED value moved by the periods between the
/// two rows' decks, and one step off a domain's end is the other end.
///
/// Stated three ways, which a body's seam can be stored in: both seam
/// columns stored at the low end and moved one and two periods; the near
/// one stored at the high end unmoved; the far one stored at the high
/// end and moved one. And the row run backward, whose two corners swap
/// to the edge's own order. A row the lift does not move reads its
/// corners as they are; a row on the twin sheet, or beside one, reads
/// none.
///
/// Red under `Lift::stored_corners` ignoring the periods between the two
/// rows' decks (reading the neighbour's stored value as it is: with both
/// seam columns stored at `lo`, both corners read `0.4`). The reading it
/// replaces, the neighbour's MOVED value un-moved by the row's deck, put
/// the far corner at `hi + P − P` (`1.7000000000000002` at `f64`, a width
/// at `Interval`: the review's probe).
#[test]
fn a_stored_corner_is_the_neighbours_stored_value_moved_between_the_decks() {
    // One body per scalar, by macro: the rows read brackets and decide
    // nothing, and name no compound bound to do it.
    macro_rules! at {
        ($t:ty, $scalar:expr) => {{
            let scalar: &str = $scalar;
            let (lo, hi) = (0.4, 1.7);
            let surface = patch::<$t>(lo, hi);
            let periods = ChartPeriods::stated(&surface);
            let span = span::<$t>();
            // A row as the lift moves it by `k` periods.
            let lifted = |stored: LiftedRow<$t>, k: i32| {
                let image = deck(k).apply(&stored.pcurve, &surface, periods).unwrap();
                let mut moved = stored.moved_for_tests(deck(k));
                moved.pcurve = image;
                moved
            };
            let bracket = |c: Option<$t>| c.map(|c| (c.lo(), c.hi()));
            let ends = [Some((lo, lo)), Some((hi, hi))];
            let bottom = || lifted(row::<$t>(0.0, (lo, hi), span), 1);
            let top = || lifted(row::<$t>(1.0, (hi, lo), span), 1);
            for (name, near, far) in [
                (
                    "both seam columns stored at lo",
                    lifted(column::<$t>(lo, (1.0, 0.0), span), 1),
                    lifted(column::<$t>(lo, (0.0, 1.0), span), 2),
                ),
                (
                    "the near column stored at hi, unmoved",
                    column::<$t>(hi, (1.0, 0.0), span),
                    lifted(column::<$t>(lo, (0.0, 1.0), span), 2),
                ),
                (
                    "the far column stored at hi, moved once",
                    lifted(column::<$t>(lo, (1.0, 0.0), span), 1),
                    lifted(column::<$t>(hi, (0.0, 1.0), span), 1),
                ),
            ] {
                let lift = Lift::stated_for_tests(
                    vec![bottom(), far.clone(), top(), near.clone()],
                    closed(),
                );
                assert_eq!(
                    lift.stored_corners(0, periods).map(bracket),
                    ends,
                    "{scalar}, {name}: the bottom row's stored corners are its domain's ends"
                );
                // The top row runs from the far column back to the near
                // one, and is stated in that order: its start is `hi`.
                assert_eq!(
                    lift.stored_corners(2, periods).map(bracket),
                    [ends[1], ends[0]],
                    "{scalar}, {name}: the top row's, in its edge's order"
                );
                // The same loop run the other way round: each row
                // reversed, the chain's order reversed.
                let back = Lift::stated_for_tests(
                    vec![
                        near.reversed_for_tests(),
                        top().reversed_for_tests(),
                        far.reversed_for_tests(),
                        bottom().reversed_for_tests(),
                    ],
                    closed(),
                );
                assert_eq!(
                    back.stored_corners(3, periods).map(bracket),
                    ends,
                    "{scalar}, {name}: a row run backward reads its corners in its edge's order"
                );
            }
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
            assert_eq!(
                plain.stored_corners(0, periods).map(bracket),
                ends,
                "{scalar}: an unmoved row spans its domain exactly"
            );
            // No corner: a cut (the neighbour on the same line), and the
            // twin sheet on either row.
            let whole = row::<$t>(0.0, (lo, hi), span);
            let mid = span.0 + (span.1 - span.0) / <$t>::from_f64(2.0);
            let cut = Lift::stated_for_tests(
                vec![
                    whole.clone().piece_for_tests((span.0, mid)),
                    whole.piece_for_tests((mid, span.1)),
                    column::<$t>(hi, (0.0, 1.0), span),
                    row::<$t>(1.0, (hi, lo), span),
                    column::<$t>(lo, (1.0, 0.0), span),
                ],
                closed(),
            );
            assert_eq!(
                (
                    cut.stored_corners(0, periods).map(bracket),
                    cut.stored_corners(1, periods).map(bracket)
                ),
                ([ends[0], None], [None, ends[1]]),
                "{scalar}: a cut is no corner of either piece"
            );
            let twin = Deck {
                u: 0,
                v: 0,
                twin: true,
            };
            let on_twin = Lift::stated_for_tests(
                vec![
                    row::<$t>(0.0, (lo, hi), span).moved_for_tests(twin),
                    column::<$t>(hi, (0.0, 1.0), span),
                    row::<$t>(1.0, (hi, lo), span),
                    column::<$t>(lo, (1.0, 0.0), span),
                ],
                closed(),
            );
            assert_eq!(
                (
                    on_twin.stored_corners(0, periods).map(bracket),
                    on_twin.stored_corners(1, periods).map(bracket)
                ),
                ([None, None], [None, Some((1.0, 1.0))]),
                "{scalar}: no stored corner is read across the twin sheet"
            );
        }};
    }
    at!(f64, "f64");
    at!(Interval, "Interval");
}
