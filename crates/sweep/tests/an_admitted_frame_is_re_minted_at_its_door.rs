//! **A frame admitted at the decided door is re-minted, and one off
//! orthogonal beyond the band at the session box refuses there**
//! (`RigidMotion::check_rigid`). A literal frame, a rotation about `z`
//! with its first column stretched off unit, places a cylinder of
//! radius 2.5 m about 990 m from the origin.
//!
//! Inside the band, read at the box's extent, the frame is admitted as
//! the rigid motion nearest it, and the placed cylinder passes tier 3's
//! check 1, which reads its frame's unit-ness at the radius. A stretch
//! of 0.4 of the band unlevered would move a point at the box's edge by
//! hundreds of bands. The door refuses it `NotRigid`, naming the
//! column, before any body is placed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_core::tolerance::SESSION_BOX_EXTENT;
use geom_core::{Affine3, Arc2, Band, Mat3, Point2, RigidMotion, RigidMotionError, Tol, Vec3};
use profile::{Profile, RawLoop, Segment, SketchPlane};
use sweep::{ExtrudeSide, Extrusion, extrude};

const RADIUS: f64 = 2.5;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).unwrap()
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
fn a_frame_inside_the_levered_band_is_re_minted_and_its_cylinder_passes_check_1() {
    let eps = tol().get().eps;
    let stretch = 1e-10_f64.min(0.2 * eps / SESSION_BOX_EXTENT);
    let motion = RigidMotion::check_rigid(&frame(stretch), band())
        .unwrap_or_else(|e| panic!("a column {stretch:e} off unit is admitted: {e}"));
    for (k, axis) in [motion.u(), motion.v(), motion.w()].into_iter().enumerate() {
        let a = axis.get();
        let defect = (a.dot(a) - 1.0).abs();
        assert!(
            defect <= 4.0 * f64::EPSILON,
            "column {k} of the admitted frame is {defect:e} off unit: it was not re-minted"
        );
    }
    let placed = topo::transform_rigid(&cylinder(), &motion, tol()).unwrap();
    if let Err(refusal) = topo::finish(placed, tol()) {
        panic!("the placed cylinder refuses at rest: {refusal}");
    }
}

#[test]
fn a_frame_beyond_the_levered_band_refuses_at_its_door() {
    let stretch = 0.4 * band().zero();
    assert!(
        matches!(
            RigidMotion::check_rigid(&frame(stretch), band()),
            Err(RigidMotionError::NotRigid {
                check: "transform_rigid_col0_unit"
            })
        ),
        "a column {stretch:e} off unit moves a point at the box's edge by {:e} m",
        stretch * SESSION_BOX_EXTENT
    );
}
