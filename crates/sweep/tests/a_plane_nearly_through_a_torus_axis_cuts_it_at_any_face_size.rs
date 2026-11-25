//! **A plane nearly through a torus's axis cuts it the same way whatever
//! the size of the face that carries it**: the torus `R = 2`, `r = 1`
//! about `y`, turned about its axis so no seam lies near the cut, met by
//! a brick whose bottom face holds the axis, turned off it about `x` by
//! a tilt straddling the band. Whether the pose is the closed form's
//! (the two meridian circles) is the section value's question, asked
//! over the torus's ball, and every chord of the join reads that
//! answer. Asked at each chord's own face reach, a brick a few times the
//! torus's size read `1.5e-10` closed to the value and tilted to the
//! chord on the plane face, and a brick fifteen times its size read
//! `3e-11` the same: the Boolean refused `RoutesToGeneralRung`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{brick, finished};
use sweep::{Revolution, revolve};

use crate::revolve_common::axis_y;

/// `|torus ∩ brick|` for the brick of half-width `half` above the plane
/// `z = 0`, turned about `x` by `tilt`: half the torus, to the band.
#[test]
fn a_plane_nearly_through_a_torus_axis_cuts_it_at_any_face_size() {
    let tol = Tol::witness();
    let lp = bulge_loop(vec![
        (Point2::new(2.0, -1.0), 1.0),
        (Point2::new(2.0, 1.0), 1.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(tol)
        .unwrap();
    let torus = revolve(&vp, axis_y(), Revolution::Full, tol)
        .expect("the profile revolves")
        .body;
    let spin = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), 0.7);
    let torus = finished(
        "the torus",
        topo::transform_rigid(&torus, &geom_core::test_support::rigid(&spin, tol)).unwrap(),
        tol,
    );
    let half_torus = PI * PI * 2.0;
    for (half, tilt) in [(5.0, 1.5e-10), (30.0, 3e-11), (30.0, 2e-10)] {
        let slab = brick::<f64>((-half, half), (-half, half), (0.0, half), tol);
        let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_x(), tilt);
        let slab = finished(
            "the slab",
            topo::transform_rigid(&slab, &geom_core::test_support::rigid(&turn, tol)).unwrap(),
            tol,
        );
        let got = topo::intersect(&torus, &slab, tol)
            .unwrap_or_else(|e| panic!("half {half}, tilt {tilt:e}: refused: {e:?}"));
        let body = got.body().expect("the cut keeps half the torus");
        let m = topo::mass_properties(&body.body, tol, topo::test_support::pinned_resolution(tol))
            .unwrap_or_else(|e| panic!("half {half}, tilt {tilt:e}: does not measure: {e:?}"))
            .enclosure;
        assert!(
            (m.volume - half_torus).abs() <= m.volume_pad + 1e-9,
            "half {half}, tilt {tilt:e}: |torus ∩ slab| = {} ± {}, half the torus {half_torus}",
            m.volume,
            m.volume_pad
        );
    }
}
