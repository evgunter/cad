//! **A point past every face of a spline-walled solid reads its side
//! at infinity**, and one within the faces' reach is read by its rays,
//! which cross the NURBS walls (`geom_brep::curve_face::line_patch`).
//!
//! The loft prism (`sweep::test_support::loft_prism`): four NURBS walls
//! between the planar caps `z = 0` and `z = 2`, anchored at `(0, 0, 1)`.
//! Its `+x` wall flares to `x = 1.375` at `y = −1`, `z = 1`, and its net
//! reaches `x = 1.75` there.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol};
use sweep::test_support::{finished, loft_prism};
use topo::SolidContainment;

#[test]
fn a_point_past_the_prisms_reach_is_outside_and_one_within_it_is_read_by_its_rays() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    let prism = finished("the loft prism", loft_prism(tol), tol);
    for q in [
        Point3::new(5.0, 0.0, 1.0),
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(0.0, -3.0, 3.0),
        // Inside the world hull, past the net along the anchor axis by
        // about 0.47 m.
        Point3::new(1.6, 0.9, 1.0),
    ] {
        assert_eq!(
            topo::point_in_solid(&prism, q, band, tol).unwrap_or_else(|e| panic!("{q:?}: {e:?}")),
            SolidContainment::Out,
            "{q:?} stands past every face"
        );
    }
    // Inside the hull of the faces' reaches the rays read the walls: the
    // centre is inside, the point beside the flare (the wall is at
    // x ≈ 1.356 there) outside, and one just short of the flare inside.
    for (what, q, want) in [
        (
            "the centre",
            Point3::new(0.0, 0.0, 1.0),
            SolidContainment::In,
        ),
        (
            "beside the flare",
            Point3::new(1.5, -0.9, 1.0),
            SolidContainment::Out,
        ),
        (
            "short of the flare",
            Point3::new(1.3, -0.9, 1.0),
            SolidContainment::In,
        ),
        (
            "under the top cap",
            Point3::new(0.4, 0.3, 1.9),
            SolidContainment::In,
        ),
    ] {
        let got = topo::point_in_solid(&prism, q, band, tol);
        assert_eq!(
            got.as_ref().ok(),
            Some(&want),
            "{what}: the rays cross the NURBS walls, got {got:?}"
        );
    }
}
