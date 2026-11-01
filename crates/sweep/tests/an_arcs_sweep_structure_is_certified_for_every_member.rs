//! **An arc's sweep structure is decided once, for every member (C6).**
//! The skin's arc conversion (`sweep::segment_curve`) reads its
//! structure off the sweep and the endpoints: that the sweep is not
//! zero, how many sub-arcs it takes (one reaches `(π/2)(1 + η)`,
//! η = 1/16), whether the arc ends where it starts, and that it is short
//! of a turn (an arc between two points) or one turn (an arc back to its
//! start). At a point scalar they are read off the values; at an
//! enclosure scalar they are certified over their brackets, and a
//! bracket that straddles a breakpoint refuses typed with its margin.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom_core::Bounds;
use geom_core::spline::SplineError;
use geom_core::{Affine3, Arc2, Interval, Point2, Real};
use sweep::{ArcSweepDecision, SketchSegment, SkinError, segment_curve};

/// The arc of radius 1 about the origin from `(1, 0)` sweeping `sweep`,
/// between two points (`to_itself` false: its end is `(−1, 0)`, a
/// point apart from its start, whatever the sweep) or back to its start.
fn arc<T: Real>(sweep: T, to_itself: bool) -> SketchSegment<T> {
    let a = Point2::new(T::one(), T::zero());
    let b = if to_itself {
        a
    } else {
        Point2::new(-T::one(), T::zero())
    };
    SketchSegment::Arc {
        a,
        b,
        arc: Arc2 {
            centre: Point2::new(T::zero(), T::zero()),
            radius: T::one(),
            sweep,
        },
    }
}

fn sub_arcs(sweep: f64, to_itself: bool) -> usize {
    let c = segment_curve(0, arc(sweep, to_itself), Affine3::identity()).expect("the arc converts");
    (c.control().len() - 1) / 2
}

/// **`k·π/2` counts `k`, and η moves only what lies just past it.** At
/// `f64`: a quarter turn is one sub-arc, and so is a sweep 6% past it;
/// 7% past it is two; a half turn two, three quarters three, a full turn
/// four. Red where the ceiling is `π/2` itself (6% past a quarter would
/// count two) or where η is wider than it says.
#[test]
fn a_multiple_of_a_quarter_turn_counts_its_multiple() {
    for (sweep, to_itself, want) in [
        (FRAC_PI_2, false, 1),
        (FRAC_PI_2 * 1.06, false, 1),
        (FRAC_PI_2 * 1.07, false, 2),
        (PI, false, 2),
        (1.5 * PI, false, 3),
        (TAU, true, 4),
    ] {
        assert_eq!(sub_arcs(sweep, to_itself), want, "sweep {sweep}");
    }
}

fn undecided(got: Result<geom::NurbsCurve3<Interval>, SkinError>, which: ArcSweepDecision) {
    match got {
        Err(SkinError::ArcSweepUndecided {
            which: w, margin, ..
        }) => {
            assert_eq!(w, which);
            let geom_core::ErrorTextReading::Enclosure { lo, hi } =
                margin.diagnostic_f64_for_error_text()
            else {
                panic!("an enclosure margin, got {margin:?}");
            };
            assert!(
                lo <= 0.0 && 0.0 <= hi,
                "{which:?}: margin [{lo}, {hi}] straddles"
            );
        }
        other => panic!(
            "{which:?}: expected the undecided refusal, got {:?}",
            other.map(|_| ())
        ),
    }
}

fn convert(lo: f64, hi: f64, to_itself: bool) -> Result<geom::NurbsCurve3<Interval>, SkinError> {
    segment_curve(
        0,
        arc(Interval::from_bounds(lo, hi), to_itself),
        Affine3::identity(),
    )
}

/// **Each sweep decision refuses undecided over a straddling bracket,
/// with its margin.** At `Interval`: a sweep around the sub-arc
/// breakpoint `(π/2)(17/16)`; one from 0; one around a full turn between
/// two points; one around a half turn back to its start. A narrower box
/// clear of the count's breakpoint lifts that refusal, and the arc then
/// stops at the carrier's door, its weight `cos(Δ/2)` no point
/// (`CarrierWeightNotAPoint`, C6's interim until S4/S5). Red where any
/// of the four decisions is read off the value channel alone.
#[test]
fn a_sweep_decision_some_members_break_refuses_undecided_with_its_margin() {
    let q = FRAC_PI_2 * (17.0 / 16.0);
    undecided(
        convert(q * 0.99, q * 1.01, false),
        ArcSweepDecision::SubArcCount,
    );
    undecided(convert(-0.1, 0.2, false), ArcSweepDecision::NonZero);
    undecided(
        convert(TAU - 0.1, TAU + 0.1, false),
        ArcSweepDecision::TurnBetweenPoints,
    );
    undecided(
        convert(PI - 0.1, PI + 0.1, true),
        ArcSweepDecision::TurnToItself,
    );
    match convert(FRAC_PI_2 * 0.99, FRAC_PI_2 * 1.05, false) {
        Err(SkinError::Structure(SplineError::CarrierWeightNotAPoint { index: 1 })) => {}
        other => panic!("a narrower box lifts the count: {:?}", other.map(|_| ())),
    }
}

/// **An arc at `Interval` stops at the carrier's door, a recorded
/// interim (C6 S3).** A point sweep, a quarter turn lifted, has a
/// certified sweep (one sub-arc), but its weight `cos(π/4)` is an
/// enclosure from rounding alone, so the curve's door refuses it as
/// `CarrierWeightNotAPoint` at weight 1, the sub-arc's middle. It was
/// `SkinError::ArcWeightNotAPoint`; C6's S4/S5 lift it. At `f64` the
/// same arc converts.
#[test]
fn an_arc_at_interval_refuses_at_the_carriers_door_until_s4_s5() {
    match convert(FRAC_PI_2, FRAC_PI_2, false) {
        Err(SkinError::Structure(SplineError::CarrierWeightNotAPoint { index: 1 })) => {}
        other => panic!("a point quarter turn at Interval: {:?}", other.map(|_| ())),
    }
    assert_eq!(sub_arcs(FRAC_PI_2, false), 1);
}

/// **The turn checks read the true 2π's bracket, at both ends of the
/// one turn an arc back to its start may make.** At `Interval`:
/// - `T::tau()` itself between two points refuses undecided: the true
///   2π lies in its bracket, so some member is a full turn. Red where
///   the check reads `f64`'s `TAU` (below the true 2π, so the sweep
///   reads a turn or more, definite) or tests `hi` against the
///   bracket's top.
/// - A sweep around 3π back to its start refuses undecided: red where
///   only the half-turn side is checked.
/// - A sweep from `π`'s bracket's top back to its start refuses
///   undecided: the true π may be that top. Red where the half-turn
///   side compares against the bracket's bottom.
#[test]
fn the_turn_checks_read_the_true_turns_bracket_at_both_ends() {
    undecided(
        segment_curve(0, arc(Interval::tau(), false), Affine3::identity()),
        ArcSweepDecision::TurnBetweenPoints,
    );
    undecided(
        convert(3.0 * PI - 0.1, 3.0 * PI + 0.1, true),
        ArcSweepDecision::TurnToItself,
    );
    let pi_top = Interval::pi().hi();
    undecided(convert(pi_top, 3.3, true), ArcSweepDecision::TurnToItself);
}

/// **Whether an arc ends where it starts is decided for every member.**
/// At `Interval`: an arc from `(1, 0)` to an end whose box holds `(1, 0)`
/// and points apart from it refuses undecided, with the endpoints'
/// distance as its margin, rather than reading as an arc back to its
/// start (which a quarter turn is not, definite). The same end at a
/// point apart converts to the carrier's door. Red where the branch is
/// read off `structure_positive` alone.
#[test]
fn whether_an_arc_ends_where_it_starts_is_decided_for_every_member() {
    let iv = Interval::from_bounds;
    let with_end = |b: Point2<Interval>| SketchSegment::Arc {
        a: Point2::new(Interval::one(), Interval::zero()),
        b,
        arc: Arc2 {
            centre: Point2::new(Interval::zero(), Interval::zero()),
            radius: Interval::one(),
            sweep: Interval::from_f64(FRAC_PI_2),
        },
    };
    match segment_curve(
        0,
        with_end(Point2::new(iv(0.9, 1.1), iv(-0.1, 0.1))),
        Affine3::identity(),
    ) {
        Err(SkinError::ArcSweepUndecided {
            which: ArcSweepDecision::Endpoints,
            margin,
            ..
        }) => {
            let geom_core::ErrorTextReading::Enclosure { lo, hi } =
                margin.diagnostic_f64_for_error_text()
            else {
                panic!("an enclosure margin, got {margin:?}");
            };
            assert!(lo <= 0.0 && 0.0 < hi, "margin [{lo}, {hi}] reaches zero");
        }
        other => panic!("an end some members share: {:?}", other.map(|_| ())),
    }
    match segment_curve(
        0,
        with_end(Point2::new(iv(0.0, 0.0), iv(1.0, 1.0))),
        Affine3::identity(),
    ) {
        Err(SkinError::Structure(SplineError::CarrierWeightNotAPoint { index: 1 })) => {}
        other => panic!("an end apart from the start: {:?}", other.map(|_| ())),
    }
}

/// **A sweep some of whose members are unbounded leaves its count
/// undecided**, not degenerate: `[1, +∞]` has finite members. A sweep
/// with no finite member is degenerate for every one. Red where the
/// whole bracket reads degenerate for its top alone.
#[test]
fn a_sweep_unbounded_above_leaves_its_count_undecided() {
    match convert(1.0, f64::INFINITY, false) {
        Err(SkinError::ArcSweepUndecided {
            which: ArcSweepDecision::SubArcCount,
            ..
        }) => {}
        other => panic!("[1, inf]: {:?}", other.map(|_| ())),
    }
    match convert(f64::INFINITY, f64::INFINITY, false) {
        Err(SkinError::DegenerateSection { .. }) => {}
        other => panic!("[inf, inf]: {:?}", other.map(|_| ())),
    }
}
