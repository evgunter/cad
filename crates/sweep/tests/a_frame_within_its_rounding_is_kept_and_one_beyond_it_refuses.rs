//! **A frame within its rounding is kept by the `f64` door, and one off
//! unit beyond it refuses** (`RigidMotion::check_rigid`). A literal
//! frame, a rotation about `z` with its first column stretched off unit,
//! places a cylinder of radius 2.5 m about 990 m from the origin.
//!
//! Within the rounding the frame's own arithmetic carries, the door
//! admits it with its bits kept, and the cylinder it places passes tier
//! 3's check 1, which reads its frame's unit-ness at the radius. Beyond
//! it, however small the defect against any length, the door refuses
//! `NotRigid`, naming the column, before any body is placed: no band
//! enters, and no frame is re-minted on the document's behalf
//! (`work/onehome/a-rigid-map-admitted-on-the-metre-band-mints-frames-check-1-refuses.md`).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_core::{Affine3, Arc2, Mat3, Point2, RigidMotion, RigidMotionError, Tol, Vec3};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};

const RADIUS: f64 = 2.5;

fn tol() -> Tol {
    Tol::witness()
}

/// A cylinder of radius [`RADIUS`] about the `z` axis, one unit tall.
fn cylinder() -> topo::Body<f64> {
    let rim = RawLoop::new([(
        Point2::new(RADIUS, 0.0),
        Segment::Arc(Arc2 {
            centre: Point2::new(0.0, 0.0),
            radius: RADIUS,
            sweep: TAU,
        }),
    )]);
    let profile = Profile::new(SketchPlane::<f64>::xy(), vec![rim])
        .validate(tol())
        .unwrap();
    let depth = Extrusion::Distance {
        depth: 1.0,
        side: ExtrudeSide::Along,
    };
    extrude(&profile, depth, tol()).unwrap().body.into_body()
}

/// A turn about `z` by 0.3 rad with its first column `1 + stretch`
/// long, standing at `(700, 700, 0)`: the cylinder's reach is under
/// 993 m, inside the box.
fn frame(stretch: f64) -> Affine3<f64> {
    let (s, c) = 0.3_f64.sin_cos();
    Affine3::from_parts(
        Mat3::from_cols(
            Vec3::new(c, s, 0.0) * (1.0 + stretch),
            Vec3::new(-s, c, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ),
        Vec3::new(700.0, 700.0, 0.0),
    )
}

#[test]
fn a_frame_within_its_rounding_is_kept_and_places_its_cylinder() {
    let stretch = f64::EPSILON;
    let kept = RigidMotion::check_rigid(&frame(stretch))
        .unwrap_or_else(|e| panic!("a column {stretch:e} off unit is admitted: {e}"));
    assert_eq!(
        kept.u().get().x.to_bits(),
        frame(stretch).linear.c0.x.to_bits(),
        "the door admits the frame as given"
    );
    let placed = topo::transform_rigid(&cylinder(), &kept, tol()).unwrap();
    if let Err(refusal) = topo::finish(placed, tol()) {
        panic!("the placed cylinder refuses at rest: {refusal}");
    }
}

#[test]
fn a_frame_beyond_its_rounding_refuses_at_its_door() {
    for stretch in [1e-12, 1e-10] {
        assert!(
            matches!(
                RigidMotion::check_rigid(&frame(stretch)),
                Err(RigidMotionError::NotRigid {
                    check: "transform_rigid_col0_unit"
                })
            ),
            "a column {stretch:e} off unit is beyond its rounding"
        );
    }
}
