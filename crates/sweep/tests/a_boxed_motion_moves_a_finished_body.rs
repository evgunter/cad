//! **A rigid motion over a box moves a finished body at `Interval`**
//! (`topo::FinishedBody::moved`;
//! `work/shell/transform-rigid-recertifies-images-enclosed-apart.md`).
//! Each image of one widened map is enclosed on its own, so
//! re-certifying them against each other reads a correlation the
//! enclosures no longer carry: a cylinder's radial vector, a carrier
//! point minus its axis's anchor, widens by twice the box. The move
//! re-certifies nothing. It carries the verdict, which holds of every
//! rigid image the box encloses, and asks only the session box.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_core::{Arc2, Bounds, Interval, Point2, Real, RigidMotion, Tol, UnitVec3, Vec3};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};
use topo::FinishedBody as _;

fn tol() -> Tol {
    Tol::witness()
}

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

/// A cylinder of radius 1, one unit tall, at `Interval`: the sweep
/// door's leaf, a replay.
fn cylinder_leaf() -> topo::Replay<Interval> {
    let rim = RawLoop::new([(
        Point2::new(iv(1.0), iv(0.0)),
        Segment::Arc(Arc2 {
            centre: Point2::new(iv(0.0), iv(0.0)),
            radius: iv(1.0),
            sweep: iv(TAU),
        }),
    )]);
    let profile = Profile::new(SketchPlane::<Interval>::xy(), vec![rim])
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: iv(1.0),
        side: ExtrudeSide::Along,
    };
    extrude(&profile, depth, tol()).unwrap().body
}

/// The same cylinder, finished.
fn cylinder() -> topo::AtRestBody<Interval> {
    let body = cylinder_leaf().into_body();
    topo::test_support::finished("the Interval cylinder", body, tol())
}

/// **A leaf at the box scalars is finished in its own frame, then moved**:
/// the taker's finish (`topo::take`) runs on the body the sweep door
/// built, and the finished body's move asks the session box of its image,
/// so a copy moved past the box refuses there, typed.
#[test]
fn a_box_scalar_leaf_is_taken_then_moved_and_refuses_past_the_box() {
    let leaf = std::sync::Arc::new(cylinder_leaf());
    let finished = topo::take::<Interval>(&leaf, tol()).expect("the leaf finishes where it stands");
    let far = RigidMotion::translation(Vec3::new(iv(2000.0), iv(0.0), iv(0.0))).unwrap();
    let refusal = finished
        .moved(&far, tol())
        .expect_err("a body moved past the session box refuses");
    let shown = refusal.to_string();
    assert!(
        matches!(
            &refusal.into_findings()[..],
            [topo::ValidationError::OutsideSessionBox { .. }]
        ),
        "the session box is the move's one refusal: {shown}"
    );
}

#[test]
fn a_boxed_translation_and_a_boxed_turn_move_a_finished_cylinder() {
    let source = cylinder();
    let half = 0.25;
    let shift = RigidMotion::translation(Vec3::new(
        Interval::from_bounds(2.0 - half, 2.0 + half),
        iv(0.0),
        iv(0.0),
    ))
    .unwrap();
    let band = geom_core::Band::linear(tol()).unwrap();
    let axis = UnitVec3::new(Vec3::new(iv(1.0), iv(0.0), iv(0.0)), "a boxed turn", band).unwrap();
    let turn = RigidMotion::rotation(axis, Interval::from_bounds(0.3, 0.31)).unwrap();
    for (what, motion) in [("the boxed translation", shift), ("the boxed turn", turn)] {
        let moved = source
            .moved(&motion, tol())
            .unwrap_or_else(|e| panic!("{what}: the finished cylinder moves: {e}"));
        for ((k, p), (k_moved, q)) in source.vertex_points().zip(moved.vertex_points()) {
            assert_eq!(k, k_moved, "{what}: the move keeps every vertex key");
            let want = motion.transform_point(p);
            let contains = |outer: Interval, inner: Interval| {
                outer.lo() <= inner.lo() && inner.hi() <= outer.hi()
            };
            assert!(
                contains(q.x, want.x) && contains(q.y, want.y) && contains(q.z, want.z),
                "{what}: vertex {k:?} moved to {q:?}, not the motion's image {want:?}"
            );
        }
    }
}

/// **A later finish of a boxed move reports undecided as undecided**
/// (ONEHOME fork 259, Ev's fork-144 sentence): the moved cylinder's
/// construction state, finished afresh over the boxed motion, reads its
/// readings over the box. Each finding it makes is an undecided one,
/// none says a datum is outside its
/// convention, and none reads as a kernel defect. A certificate's
/// undecided reading ends in the box scalars' recourse; a decision with
/// its own lever (the dihedral's) ends in that.
#[test]
fn a_later_finish_of_a_boxed_move_reports_only_undecided_findings() {
    let source = cylinder();
    let band = geom_core::Band::linear(tol()).unwrap();
    let axis = UnitVec3::new(Vec3::new(iv(1.0), iv(0.0), iv(0.0)), "a boxed turn", band).unwrap();
    let turn = RigidMotion::rotation(axis, Interval::from_bounds(0.3, 0.31)).unwrap();
    let half = 0.25;
    let shift = RigidMotion::translation(Vec3::new(
        Interval::from_bounds(2.0 - half, 2.0 + half),
        iv(0.0),
        iv(0.0),
    ))
    .unwrap();
    for (what, motion) in [("the boxed translation", shift), ("the boxed turn", turn)] {
        let moved = source.moved(&motion, tol()).unwrap();
        let Err(refused) = topo::finish(moved.into_body(), tol()) else {
            continue;
        };
        for finding in refused.into_findings() {
            let text = finding.to_string();
            assert!(
                !text.contains("report it"),
                "{what}: an undecided reading reads as a kernel defect: {text}"
            );
            assert!(
                !matches!(
                    finding,
                    topo::ValidationError::UnrepresentableSurfaceDatum { .. }
                        | topo::ValidationError::UnrepresentableCurveDatum { .. }
                ),
                "{what}: an undecided convention reading reads as definite: {text}"
            );
            assert!(
                text.contains("undecided"),
                "{what}: a later finish of a boxed move makes a definite finding: {text}"
            );
        }
    }
}
