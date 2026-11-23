//! **A path sweep's start placement is the plane normal to the path**,
//! and an identity placement is not that plane on a path that departs
//! near world +Z without being on it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::FRAC_PI_2;

use geom::NurbsCurve3;
use geom_core::{Point3, Tol, Vec3};
use profile::SketchPlane;
use sweep::{Lofted, sweep_body};

use crate::common::{normal_start_place, quad};

const S_RADIUS: f64 = 2.0;
const H: f64 = 0.25;

/// The S: two opposed quarter arcs in `x = 0`, 17 exact points at
/// degree 3. `turning_orientation`'s `inflecting_path` and the tour's
/// `s_duct` spine are this curve.
fn inflecting_path() -> NurbsCurve3<f64> {
    let pts: Vec<Point3<f64>> = (0..=8)
        .map(|k| {
            let th = FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(0.0, S_RADIUS * (1.0 - th.cos()), S_RADIUS * th.sin())
        })
        .chain((1..=8).map(|k| {
            let ph = FRAC_PI_2 * f64::from(k) / 8.0;
            Point3::new(
                0.0,
                S_RADIUS + S_RADIUS * ph.sin(),
                2.0 * S_RADIUS - S_RADIUS * ph.cos(),
            )
        }))
        .collect();
    NurbsCurve3::<f64>::interpolate(&pts, 3).unwrap()
}

fn swept_square(path: &NurbsCurve3<f64>, place: SketchPlane<f64>, stations: usize) -> Lofted<f64> {
    let profile = quad([(-H, -H), (H, -H), (H, H), (-H, H)]);
    sweep_body::<f64>(&profile, place, path, stations, 3, Tol::witness()).unwrap()
}

fn volume(b: &Lofted<f64>) -> f64 {
    topo::test_support::pinned_volume(
        "the measured body",
        &topo::test_support::finished(
            "the measured body",
            topo::Body::clone(&b.body),
            Tol::witness(),
        ),
        Tol::witness(),
    )
    .volume
}

/// **An identity placement is not the normal plane on this path**, and
/// a scene that used one was drawing its profile in a tilted plane.
///
/// The S's start tangent is near world +Z and not on it: the
/// interpolant departs at `|n̂.z| = 0.99999911`, so identity's +Z is
/// about `1.3e-3` rad off the tangent. That is small and it is not
/// nothing — the swept body differs.
///
/// ANTI-VACUITY: the tilt is asserted to be in a band, so a fixture
/// that drifted onto the axis (making identity correct by accident)
/// reddens the row instead of silently making it free.
#[test]
fn an_identity_start_placement_is_not_the_normal_plane_of_the_s_path() {
    let path = inflecting_path();
    let (lo, _) = path.domain();
    let tilt = path
        .deriv(lo)
        .normalize()
        .dot(Vec3::unit_z())
        .clamp(-1.0, 1.0)
        .acos();
    println!("identity's +Z is {tilt:.6e} rad off the S path's start tangent");
    assert!(
        (1e-3..2e-3).contains(&tilt),
        "the S path must depart NEAR world +Z and not ON it; measured {tilt} rad"
    );

    let (vi, vd) = (
        volume(&swept_square(&path, SketchPlane::xy(), 13)),
        volume(&swept_square(&path, normal_start_place(&path), 13)),
    );
    println!("s_duct volume: identity {vi:.15}  normal plane {vd:.15}");
    assert!(
        (vd - vi).abs() > 1e-9,
        "the tilted profile plane must build a different body: the two volumes differ \
         by only {:e}",
        (vd - vi).abs()
    );
}
