//! **A one-segment closed loop** (D1's profile format: a full turn is
//! ONE segment at ONE vertex, |Δθ| = 2π, so a closed carrier is one
//! edge), written through the fixture door and decided at validate.
//!
//! Rows: the loop validates at `f64` and at `Interval`, outer and hole,
//! with its carrier carried verbatim; the scalar crossings and the
//! reversal copy that carrier (no re-lowering from a zero chord); and a
//! one-vertex table that is not a full turn is refused typed, by the
//! check that reads what is wrong with it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom_core::{Arc2, Bounds, Interval, Point2, Real, Sign, Tol};
use profile::{
    ArcCheck, LoopRole, Profile, ProfileError, ProfileLoop, RawLoop, Segment, SegmentKind,
    SegmentRef, SketchPlane,
};

/// A one-segment circle about `(cx, cy)` of radius `r`, its vertex at
/// carrier angle 0, turning `sweep`.
fn circle(cx: f64, cy: f64, r: f64, sweep: f64) -> ProfileLoop<f64> {
    RawLoop::new([(
        Point2::new(cx + r, cy),
        Segment::Arc(Arc2 {
            centre: Point2::new(cx, cy),
            radius: r,
            sweep,
        }),
    )])
}

fn square(h: f64) -> ProfileLoop<f64> {
    RawLoop::polygon([
        Point2::new(-h, -h),
        Point2::new(h, -h),
        Point2::new(h, h),
        Point2::new(-h, h),
    ])
}

fn validate<T: geom_core::Decide>(
    loops: Vec<ProfileLoop<T>>,
) -> Result<profile::ValidatedProfile<T>, ProfileError> {
    Profile::new(SketchPlane::<T>::xy(), loops).validate(Tol::witness())
}

fn bits(x: &dyn core::fmt::Debug) -> String {
    format!("{x:?}")
}

/// A counterclockwise one-segment circle validates as the outer loop,
/// one vertex and one arc whose carrier is the stored one bit for bit;
/// a clockwise one is reversed into it, its carrier still copied and
/// its sweep negated.
#[test]
fn a_one_segment_circle_validates_with_its_carrier_verbatim() {
    for (sweep, turn) in [(TAU, Sign::Positive), (-TAU, Sign::Negative)] {
        let lp = circle(0.25, -0.5, 1.5, sweep);
        let vp = validate(vec![lp]).unwrap_or_else(|e| panic!("sweep {sweep}: {e}"));
        let out = &vp.loops()[0];
        assert_eq!(out.role(), LoopRole::Outer);
        assert_eq!(out.vertices().len(), 1, "one vertex");
        assert_eq!(out.segments().len(), 1, "one segment");
        let seg = out.segments()[0];
        assert_eq!(
            bits(&seg.start),
            bits(&seg.end),
            "the segment closes on its vertex"
        );
        let SegmentKind::Arc { arc, turn: got } = seg.kind else {
            panic!("a full turn is an arc, got {:?}", seg.kind);
        };
        // Canonical outers run counterclockwise: the clockwise input is
        // reversed, which negates the sweep and keeps the carrier.
        assert_eq!(got, Sign::Positive, "the outer runs counterclockwise");
        assert_eq!(bits(&arc.centre), bits(&Point2::new(0.25, -0.5)));
        assert_eq!(arc.radius.to_bits(), 1.5f64.to_bits());
        assert_eq!(arc.sweep.to_bits(), TAU.to_bits(), "input turn {turn:?}");
    }
}

/// The same loop at `Interval`: the lifted table validates, and the
/// validated `f64` profile lifted onto `Interval` carries each stored
/// field as its degenerate enclosure.
#[test]
fn a_one_segment_circle_validates_at_interval_and_lifts_verbatim() {
    let lp = circle(0.25, -0.5, 1.5, TAU);
    let at_i = lp.map_scalar(Interval::from_f64);
    let vi = validate(vec![at_i]).expect("the lifted circle validates at Interval");
    assert_eq!(vi.loops()[0].segments().len(), 1);
    let vf = validate(vec![lp]).expect("the circle validates at f64");
    let lifted = vf.lift_onto(SketchPlane::<Interval>::xy());
    for vp in [&vi, &lifted] {
        let SegmentKind::Arc { arc, turn } = vp.loops()[0].segments()[0].kind else {
            panic!("an arc");
        };
        let lohi = |x: Interval| (x.lo(), x.hi());
        assert_eq!(turn, Sign::Positive);
        assert_eq!(lohi(arc.centre.x), (0.25, 0.25));
        assert_eq!(lohi(arc.centre.y), (-0.5, -0.5));
        assert_eq!(lohi(arc.radius), (1.5, 1.5));
        assert_eq!(lohi(arc.sweep), (TAU, TAU));
    }
}

/// **The prerequisite**: every crossing that used to re-lower a segment
/// from its chord and kept bulge (`map_scalar`, `reversed`) now copies
/// the stored carrier, so a one-segment circle — chord zero — comes
/// back with its own centre and radius, never a NaN one.
#[test]
fn crossings_copy_a_one_segment_circles_carrier() {
    let lp = circle(3.0, 1.0, 0.5, TAU);
    let carrier = |l: &ProfileLoop<f64>| match l.segments()[0] {
        Segment::Arc(arc) => arc,
        Segment::Line => panic!("an arc"),
    };
    let back = lp.reversed();
    let (f, r) = (carrier(&lp), carrier(&back));
    assert_eq!(
        bits(&r.centre),
        bits(&f.centre),
        "reversal keeps the centre"
    );
    assert_eq!(r.radius.to_bits(), f.radius.to_bits(), "and the radius");
    assert_eq!(r.sweep.to_bits(), (-TAU).to_bits(), "and negates the sweep");
    let at_i = lp.map_scalar(Interval::from_f64);
    let Segment::Arc(i) = at_i.segments()[0] else {
        panic!("an arc");
    };
    assert!(!i.centre.x.is_poison() && !i.radius.is_poison(), "{i:?}");
    assert_eq!((i.centre.x.lo(), i.radius.lo()), (3.0, 0.5));
}

/// A one-segment circle as a hole in a square, and a square inside a
/// one-segment circle: each validates, with the roles the containment
/// forest gives, and the hole is wound clockwise.
#[test]
fn a_one_segment_circle_is_a_hole_and_holds_one() {
    let vp = validate(vec![square(2.0), circle(0.0, 0.0, 1.0, TAU)])
        .expect("a circular hole in a square validates");
    assert_eq!(vp.loops()[1].role(), LoopRole::Hole);
    let SegmentKind::Arc { turn, .. } = vp.loops()[1].segments()[0].kind else {
        panic!("an arc");
    };
    assert_eq!(turn, Sign::Negative, "a hole runs clockwise");

    let vp = validate(vec![square(0.5), circle(0.0, 0.0, 2.0, -TAU)])
        .expect("a square hole in a one-segment circle validates");
    assert_eq!(vp.loops()[0].segments().len(), 1, "the circle is the outer");
    assert_eq!(vp.loops()[1].segments().len(), 4);

    let at_i: Vec<ProfileLoop<Interval>> = [square(2.0), circle(0.0, 0.0, 1.0, TAU)]
        .iter()
        .map(|l| l.map_scalar(Interval::from_f64))
        .collect();
    validate(at_i).expect("the holed square validates at Interval");
}

/// Two one-segment circles that cross, touch or coincide are refused by
/// the simplicity pass, as any two loops' segments are.
#[test]
fn two_one_segment_circles_meet_as_any_two_loops_do() {
    let crossing = validate(vec![circle(0.0, 0.0, 1.0, TAU), circle(1.0, 0.0, 1.0, TAU)]);
    assert!(
        matches!(crossing, Err(ProfileError::NonSimple { .. })),
        "{crossing:?}"
    );
    let same = validate(vec![
        circle(0.0, 0.0, 1.0, TAU),
        circle(0.0, 0.0, 1.0, -TAU),
    ]);
    assert!(
        matches!(same, Err(ProfileError::NonSimple { .. })),
        "{same:?}"
    );
    let nested = validate(vec![circle(0.0, 0.0, 2.0, TAU), circle(0.5, 0.0, 1.0, TAU)]);
    assert!(nested.is_ok(), "{nested:?}");
}

/// A one-vertex table that is not a full turn is refused, typed: a
/// line from the vertex to itself has no length, an arc turning by
/// nothing has none either, a half turn does not land on its own start,
/// a start off the carrier is off it, and two full turns land but are
/// past one.
#[test]
fn a_one_vertex_table_that_is_not_a_full_turn_is_refused() {
    let at = SegmentRef {
        loop_index: 0,
        segment_index: 0,
    };
    let line: ProfileLoop<f64> = RawLoop::new([(Point2::new(1.0, 0.0), Segment::Line)]);
    assert_eq!(
        validate(vec![line]).err(),
        Some(ProfileError::DegenerateSegment(at))
    );
    assert_eq!(
        validate(vec![circle(0.0, 0.0, 1.0, 1e-15)]).err(),
        Some(ProfileError::DegenerateSegment(at)),
    );
    assert_eq!(
        validate(vec![circle(0.0, 0.0, 0.1 * Tol::witness().eps(), TAU)]).err(),
        Some(ProfileError::DegenerateSegment(at)),
    );
    assert_eq!(
        validate(vec![circle(0.0, 0.0, 1.0, PI)]).err(),
        Some(ProfileError::InconsistentArc {
            at,
            check: ArcCheck::Landing
        }),
    );
    assert_eq!(
        validate(vec![circle(0.0, 0.0, 1.0, 2.0 * TAU)]).err(),
        Some(ProfileError::InconsistentArc {
            at,
            check: ArcCheck::SweepRange
        }),
    );
    let off: ProfileLoop<f64> = RawLoop::new([(
        Point2::new(1.5, 0.0),
        Segment::Arc(Arc2 {
            centre: Point2::new(0.0, 0.0),
            radius: 1.0,
            sweep: TAU,
        }),
    )]);
    assert_eq!(
        validate(vec![off]).err(),
        Some(ProfileError::InconsistentArc {
            at,
            check: ArcCheck::OnCarrier
        }),
    );
}

/// **A chained leg's full turn stays refused.** In a loop of two or
/// more vertices a segment leaves its vertex for another, so a stored
/// full turn there cannot land on its far end: the table is refused at
/// the landing check, and the two-vertex loop whose two vertices
/// coincide is refused as a zero chord.
#[test]
fn a_full_turn_between_two_vertices_is_refused() {
    let at = SegmentRef {
        loop_index: 0,
        segment_index: 0,
    };
    let full = Segment::Arc(Arc2 {
        centre: Point2::new(0.0, 0.0),
        radius: 1.0,
        sweep: TAU,
    });
    let two: ProfileLoop<f64> = RawLoop::new([
        (Point2::new(1.0, 0.0), full),
        (Point2::new(-1.0, 0.0), Segment::Line),
    ]);
    assert_eq!(
        validate(vec![two]).err(),
        Some(ProfileError::InconsistentArc {
            at,
            check: ArcCheck::Landing
        }),
    );
    let doubled: ProfileLoop<f64> =
        RawLoop::new([(Point2::new(1.0, 0.0), full), (Point2::new(1.0, 0.0), full)]);
    assert_eq!(
        validate(vec![doubled]).err(),
        Some(ProfileError::DegenerateSegment(at)),
    );
}
