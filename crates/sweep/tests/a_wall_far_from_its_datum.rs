//! **A cone wall far from its apex measures its own volume.** A partial
//! revolve of a slightly tapered annulus has two cone walls whose apexes
//! stand `1/τ` and `2/τ` from them. A cone wall's flux is its apex
//! dotted with its vector area, and a closed loop's vector area does not
//! depend on the point it is summed about — but summed about the apex
//! its terms grow with that distance and cancel to the wall's area,
//! carrying their rounding at the distance into a closed form whose pad
//! is zero. A rigid turn moves the rounding off the axis that hid it.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use test_utils::vacuity::Exposure;
use topo::{AtRestBody, Body};

use crate::common::oracles::volume_slack;

/// The revolve's sweep, in radians.
const THETA: f64 = 1.0;

/// `(τ, pivot)`: the annulus `1 ≤ r ≤ 2`, one tall, its top drawn in by
/// `τ`, revolved through `THETA` about the sketch's `y` axis, then
/// turned 1.3 about `(1, 2, 3)` through `pivot`.
const WALLS: [(f64, [f64; 3]); 3] = [
    (1e-5, [0.3, -0.2, 0.5]),
    (1e-3, [0.3, -0.2, 0.5]),
    (1e-1, [300.0, -200.0, 500.0]),
];

#[test]
fn a_turned_cone_wall_far_from_its_apex_measures_its_own_volume() {
    let tol = Tol::witness();
    let mut seen = Exposure::new("turned cone walls");
    for (tau, [x, y, z]) in WALLS {
        let verts = vec![
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(2.0, 0.0), 0.0),
            (Point2::new(2.0 - tau, 1.0), 0.0),
            (Point2::new(1.0 - tau, 1.0), 0.0),
        ];
        let profile = Profile::new(SketchPlane::<f64>::xy(), vec![bulge_loop(verts)])
            .validate(tol)
            .unwrap();
        let axis = sweep::RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        };
        // A revolve this ε cannot certify is a refusal, not a wrong
        // answer; the floor below counts the walls that reach the turn.
        let Ok(built) = sweep::revolve(&profile, axis, sweep::Revolution::Partial(THETA), tol)
        else {
            seen.note("revolve refused");
            continue;
        };
        seen.note("turned");
        let body = AtRestBody::validate(built.body, tol).unwrap();
        let turn = Affine3::rotation_about_axis(
            Point3::new(x, y, z),
            Vec3::new(1.0, 2.0, 3.0) / 14f64.sqrt(),
            1.3,
        );
        let turned: Body<f64> =
            topo::transform_rigid(&body, &geom_core::test_support::rigid(&turn, tol), tol).unwrap();
        // Pappus: the profile's moment about the axis, swept through θ.
        let truth = THETA * (3.0 - tau) / 2.0;
        let after = topo::mass_properties(&turned, tol).unwrap();
        let slack = volume_slack(&after, truth);
        assert!(
            (after.volume - truth).abs() <= slack,
            "τ {tau:e}, turned through {:?}: the volume {:e} misses {truth:e} by {:e}, past {slack:e}",
            [x, y, z],
            after.volume,
            (after.volume - truth).abs()
        );
    }
    seen.report();
    seen.require(
        "turned",
        1,
        "every ε builds a tapered wall to turn: τ = 0.1 is far outside each band",
    );
}
