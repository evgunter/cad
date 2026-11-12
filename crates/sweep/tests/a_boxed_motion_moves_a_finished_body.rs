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

/// **A leaf at the box scalars asks the session box when it moves**
/// (`topo::Replay::moved_by`): every move asks it, whatever it moves, so
/// a replay moved past the box refuses there, typed, and is not handed
/// on for a later taker to find.
#[test]
fn a_box_scalar_leaf_moved_past_the_box_refuses_at_the_box() {
    use topo::LeafBody as _;
    let far = RigidMotion::translation(Vec3::new(iv(2000.0), iv(0.0), iv(0.0))).unwrap();
    let refusal = cylinder_leaf()
        .moved(&far, tol())
        .expect_err("a leaf moved past the session box refuses");
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
/// readings over the box, and the finish refuses (a finish that passed
/// would leave this row nothing to read, so it asserts the refusal).
/// Every finding it makes is an undecided one, none says a datum is
/// outside its convention, none reads as a kernel defect, and each ends
/// as its decision ends an undecided reading past a build (fork 259 item
/// 2): one with no lever of its own (the edge certificate, the pinch
/// corner, the datums) in the box scalars' recourse; a pcurve row's in
/// its check's own lever, which item 2 leaves unchanged (the general
/// lane's "not yet", the incidence lever); and a flat face's in band
/// read, written with no door, in none. The row asserts that every
/// finding is one of these, so a new ending fails here.
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
        let refused = topo::finish(moved.into_body(), tol())
            .map(|_| ())
            .expect_err(&format!("{what}: a later finish over the box refuses"));
        let findings = refused.into_findings();
        assert!(
            !findings.is_empty(),
            "{what}: the refusal names its findings"
        );
        for finding in findings {
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
            let ends_as_its_decision = match finding {
                topo::ValidationError::Pcurve { .. } => {
                    text.ends_with(geom_core::not_yet_ending!())
                        || text.ends_with(
                            "Recourse: move the geometry so the boundary lies clearly on the \
                             face it bounds at this tolerance",
                        )
                }
                topo::ValidationError::PlanarFaceEscalated { .. }
                | topo::ValidationError::PlanarBoundaryEscalated { .. } => {
                    text.ends_with("is undecided")
                }
                _ => text.ends_with(geom_brep::recourse::BOX_SCALAR_RECOURSE),
            };
            assert!(
                ends_as_its_decision,
                "{what}: an undecided reading over the box ends in another recourse: {text}"
            );
        }
    }
}
