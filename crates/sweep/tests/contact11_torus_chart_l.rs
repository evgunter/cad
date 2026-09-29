//! **No boolean mints an L-shaped torus face yet**, and this row says
//! so. A half donut less a bar over one quarter of its upper tube would
//! leave both torus walls L-shaped in the chart (a parallel cut by the
//! bar's floor, a meridian by its side); the pipeline refuses the torus
//! × plane pair before any containment is asked. The chart-box check
//! that refuses such a face at the containment doors is pinned in
//! `topo`'s `an_l_shaped_torus_face_refuses_rather_than_trim_by_its_hull`;
//! when this pair is admitted, this row is where the notch
//! (`point_in_solid` there is `Out`) and the volume (`3π²/8`) are owed.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::revolve_common;
use geom_core::{Point2, Point3, Tol};
use profile::{ProfileLoop, RawLoop};
use revolve_common::{axis_y, validated};
use sweep::{Revolution, revolve};
use topo::{Body, BooleanError};

/// The axis-aligned block `x × y × z`, extruded along `z`.
fn bar(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    use geom_core::{Affine3, Mat3, Vec3};
    let lp = ProfileLoop::polygon([
        Point2::new(x.0, y.0),
        Point2::new(x.1, y.0),
        Point2::new(x.1, y.1),
        Point2::new(x.0, y.1),
    ]);
    let plane = profile::SketchPlane::new(Affine3::from_parts(
        Mat3::from_cols(Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()),
        Point3::new(0.0, 0.0, z.0) - Point3::origin(),
    ));
    let vp = profile::Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    sweep::extrude(&vp, sweep::Extrusion::Distance(z.1 - z.0), Tol::witness())
        .unwrap()
        .body
}

#[test]
fn a_notched_half_donut_refuses_at_the_torus_plane_pair() {
    let vp = validated(vec![revolve_common::donut_profile()]);
    let half = revolve(
        &vp,
        axis_y(),
        Revolution::Partial(core::f64::consts::PI),
        Tol::witness(),
    )
    .unwrap()
    .body;
    let cutter = bar((0.0, 3.0), (0.0, 1.0), (-3.0, 3.0));
    let Err(err) = topo::subtract(&half, &cutter, Tol::witness()) else {
        panic!("the torus × plane pair is refused");
    };
    assert!(
        matches!(err, BooleanError::CurvedPairUnsupported { .. }),
        "refused at the curved pair: {err:?}"
    );
}
