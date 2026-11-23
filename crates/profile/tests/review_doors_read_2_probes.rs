//! Probes on the validated segment's kind holding its directions.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Arc2, Interval, Point2, Sign, Tol, Vec2};
use profile::test_support::bulge_loop;
use profile::{BlendArc, Profile, SegmentKind, SketchPlane, ValidatedSegment};

fn radials<T: geom_core::Real>(kind: SegmentKind<T>) -> [Vec2<T>; 2] {
    match kind {
        SegmentKind::Arc { radials, .. } => radials,
        SegmentKind::Line { .. } => panic!("an arc"),
    }
}

/// **The value channel keeps each arc's radial at the end it was
/// recorded at.** A quarter arc's two radials differ, so a channel that
/// handed the end radial back as the start one reads differently from
/// the `f64` form it was lifted from.
#[test]
fn the_value_channel_keeps_an_arcs_radials_at_their_own_ends() {
    let q = (std::f64::consts::PI / 8.0).tan();
    let vp = Profile::new(
        SketchPlane::<f64>::xy(),
        vec![bulge_loop(vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), q),
            (Point2::new(1.0, 1.0), q),
        ])],
    )
    .validate(Tol::witness())
    .expect("the D validates");
    let channel = vp
        .clone()
        .lift_onto(SketchPlane::<Interval>::xy())
        .expect("the lift")
        .value_channel()
        .expect("a point channel");
    let bits = |v: Vec2<f64>| (v.x.to_bits(), v.y.to_bits());
    let mut arcs = 0;
    for (s, c) in vp.loops()[0]
        .segments()
        .iter()
        .zip(channel.loops()[0].segments())
    {
        if matches!(s.kind, SegmentKind::Line { .. }) {
            continue;
        }
        arcs += 1;
        let ([rs, re], [cs, ce]) = (radials(s.kind), radials(c.kind));
        assert_ne!(bits(rs), bits(re), "a quarter arc's radials differ");
        assert_eq!(bits(cs), bits(rs), "the channel's start radial");
        assert_eq!(bits(ce), bits(re), "the channel's end radial");
    }
    assert_eq!(arcs, 2, "the D has two arcs");
}

/// **What the type still lets another crate write.** Every field of a
/// validated segment and of its kind is public, so each of these
/// compiles outside `profile`: a director that is no unit, an arc whose
/// turn is `Zero` with radials that are no directions at all, and a
/// blend arc that holds a line. None can enter a `ValidatedLoop`, whose
/// fields are private; the unit claim, the turn and "always an arc" are
/// held by what constructs them, not by the type.
#[test]
fn another_crate_can_write_a_free_standing_segment_whose_directions_are_no_units() {
    let o = Point2::new(0.0_f64, 0.0);
    let stretched = ValidatedSegment {
        start: o,
        end: Point2::new(1.0, 0.0),
        kind: SegmentKind::Line {
            director: Vec2::new(5.0, 0.0),
        },
    };
    let flat = SegmentKind::Arc {
        arc: Arc2 {
            centre: o,
            radius: 1.0,
            sweep: 0.0,
        },
        turn: Sign::Zero,
        radials: [Vec2::new(0.0, 0.0); 2],
    };
    let blend = BlendArc {
        segment: 0,
        kind: stretched.kind,
    };
    assert!(matches!(blend.kind, SegmentKind::Line { .. }));
    assert!(matches!(
        flat,
        SegmentKind::Arc {
            turn: Sign::Zero,
            ..
        }
    ));
}
