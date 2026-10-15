//! **Two halves of one ball, the upper turned a little about a diameter
//! of their shared equator.**
//!
//! A ball of radius [`R`] about [`C`] cut by the plane `z = C.z` gives
//! its two hemispheres; the upper one, turned `θ` about the `y`
//! diameter, still lies on the same sphere, and its rim (a great circle
//! tilted `θ`) crosses the lower one's at the diameter's ends. Over
//! `x > C.x` the upper half dips into the lower one by a spherical wedge
//! of angle `θ`, of volume `⅔·R³·θ`.
//!
//! Whether the one rim crosses the other half's boundary is
//! `carrier_cross`'s circle-against-circle question for two planes
//! definitely transverse at a tilt of `10³·Kε`. Away from the world
//! origin their common line, formed from the planes' offsets from it by
//! a difference over `sin²θ`, sat up to `u·|C|/θ²` off both rims, and
//! the rims read clear of each other while crossing.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;

use geom_core::{Affine3, Point3, Tol, Vec3};
use sweep::test_support::{ball_poled_z, brick};
use topo::test_support::finished;
use topo::{AtRestBody, BooleanResult};

/// The ball's radius.
const R: f64 = 1.0;
/// The ball's centre, away from the world origin.
const C: [f64; 3] = [3.7, -2.1, 0.4];

/// The lower half, and the upper half turned `theta` about the `y`
/// diameter.
fn halves(theta: f64, tol: Tol) -> (AtRestBody<f64>, AtRestBody<f64>) {
    let [x, y, z] = C;
    let half = |what: &str, zs: (f64, f64)| {
        let ball = finished("the ball", ball_poled_z(R, Vec3::new(x, y, z), tol), tol);
        let slab = finished(
            what,
            brick((x - 3.0, x + 3.0), (y - 3.0, y + 3.0), zs, tol),
            tol,
        );
        match topo::intersect(&ball, &slab, tol) {
            Ok(BooleanResult::Body(b)) => b.body,
            other => panic!("the ball ∩ {what}: {other:?}"),
        }
    };
    let lower = half("the lower slab", (z - 3.0, z));
    let upper = half("the upper slab", (z, z + 3.0));
    let turn = Affine3::rotation_about_axis(Point3::new(x, y, z), Vec3::unit_y(), theta);
    let upper =
        topo::transform_rigid(&upper, &geom_core::test_support::rigid(&turn, tol), tol).unwrap();
    (lower, finished("the turned upper half", upper, tol))
}

/// **Every op builds at its closed form** at a tilt of `10³·Kε`. At
/// ε = 1e-9 and 1e-12 the common line put the rims' meetings `1e-6` and
/// metres off both rims; at ε = 1e-6 the tilt is `1e-2`, the line held,
/// and the row stands as the same check.
#[test]
fn halves_of_a_ball_turned_a_little_apart_build_every_op_at_their_closed_forms() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let theta = 1e3 * tol.k() * eps;
    let (a, b) = halves(theta, tol);
    let (half, wedge) = (2.0 / 3.0 * PI * R.powi(3), 2.0 / 3.0 * R.powi(3) * theta);
    for (op, out, truth) in [
        ("a ∪ b", topo::union(&a, &b, tol), half + half - wedge),
        ("a ∩ b", topo::intersect(&a, &b, tol), wedge),
        ("a ∖ b", topo::subtract(&a, &b, tol), half - wedge),
        ("b ∖ a", topo::subtract(&b, &a, tol), half - wedge),
    ] {
        let label = format!("ε {eps:e}, θ {theta:e}, {op}");
        let out = match out {
            Ok(BooleanResult::Body(out)) => out,
            other => panic!("{label}: {other:?}"),
        };
        assert_eq!(topo::validate(&out.body), Ok(()), "{label}: validate");
        assert_eq!(topo::validate_closed(&out.body), Ok(()), "{label}: closed");
        assert_eq!(
            topo::validate_geometric(&out.body, tol),
            Ok(()),
            "{label}: tier 3"
        );
        let m = topo::mass_properties(&out.body, tol)
            .unwrap_or_else(|e| panic!("{label}: mass properties {e:?}"));
        assert!(
            (m.volume - truth).abs() <= m.volume_pad + 1e-9 * truth,
            "{label}: volume {} ± {} against the closed form {truth}",
            m.volume,
            m.volume_pad
        );
    }
}
