//! **A point past every face of a spline-walled solid reads its side
//! at infinity**; any other point keeps the probe's refusal. The ray
//! test has no arm for a NURBS face, so `point_in_solid` decides a point
//! of such a selection only where it stands definitely outside the hull
//! of every face's certified reach (a NURBS face's is its control net's
//! hull) along a direction that turns with the solid — a cap's normal, or
//! the axis from the faces' anchor to the point — whence a straight path
//! to infinity meets no face.
//!
//! The loft prism (`sweep::test_support::loft_prism`): four NURBS walls
//! between the planar caps `z = 0` and `z = 2`, anchored at `(0, 0, 1)`.
//! Its `+x` wall flares to `x = 1.375` at `y = −1`, `z = 1`, and its net
//! reaches `x = 1.75` there.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol};
use sweep::test_support::{finished, loft_prism};
use topo::{PointInSolidError, SolidContainment};

#[test]
fn a_point_past_the_prisms_reach_is_outside_and_none_within_it_is_read() {
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
    // The centre stands outside every face's own reach (each wall's box
    // lies at |x| ≥ 1 or |y| = 1, each cap's at z = 0 or 2) and inside the
    // solid; the point beside the flare stands outside the solid (the
    // wall is at x ≈ 1.356 there) and inside the wall's net along every
    // candidate (the net's support along the anchor axis is ≈ 2.01, the
    // point's ≈ 1.75). Neither is past the whole hull, so neither is read.
    for (what, q) in [
        ("the centre", Point3::new(0.0, 0.0, 1.0)),
        ("beside the flare", Point3::new(1.5, -0.9, 1.0)),
    ] {
        let got = topo::point_in_solid(&prism, q, band, tol);
        assert!(
            matches!(
                got,
                Err(PointInSolidError::KindUnsupported {
                    kind: geom::SurfaceKind::Nurbs,
                    ..
                })
            ),
            "{what}: the probe reads no NURBS face within the hull, got {got:?}"
        );
    }
}
