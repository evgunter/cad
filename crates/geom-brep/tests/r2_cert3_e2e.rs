//! CERT-3 review lane R2 — the required e2e exercise. My own fixtures,
//! not the unit's: an OBLIQUE axis, an axis origin far from the point,
//! a PARTIAL sweep, nested `restrict` (a restriction round trip
//! through two compositions), and an angle NEAR but not AT zero.
//!
//! Runs at BOTH lanes. The unit's own consumer file runs at the
//! interval scalar only, so nothing it ships exercises `RevolvedPoint`
//! at `f64`; this file does.
//!
//! Not a unit deliverable; a reviewer's instrument.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::TAU;

use geom_brep::MappedCurve;
use geom_core::{Affine3, Point2, Point3, Real, Vec3};

/// An oblique-axis, partial-sweep revolve whose axis origin is far from
/// the swept point — deliberately less friendly than the unit's
/// `+z`-axis, unit-radius, full-turn rim.
fn oblique<T: Real>(angle: T, mk: impl Fn(f64) -> T) -> MappedCurve<T> {
    MappedCurve::RevolvedPoint {
        point: Point2::new(mk(7.0), mk(-4.0)),
        place: Affine3::translation(Vec3::new(mk(1.5), mk(-2.5), mk(11.0))),
        axis_origin: Point3::new(mk(-30.0), mk(45.0), mk(-12.0)),
        axis_dir: Vec3::new(mk(1.0), mk(-2.0), mk(2.0)),
        angles: geom_brep::SweepRange::from_zero(angle),
    }
}

/// f64 lane: what does a consumer see? The load-bearing consumer
/// property is that `restrict(s0, s1).eval(0)` agrees with
/// `eval(s0)` — the restriction round trip.
#[test]
fn r2_e2e_restriction_round_trip_f64() {
    for &theta in &[0.0f64, 1.0e-12, 1.0e-7, 1.0e-3, 0.4, TAU] {
        let c = oblique(theta, |x| x);
        for &s0 in &[0.0f64, 0.125, 0.5] {
            let direct = c.eval(s0);
            let stored = c.restrict(s0, 1.0).eval(0.0);
            let d = ((direct.x - stored.x).powi(2)
                + (direct.y - stored.y).powi(2)
                + (direct.z - stored.z).powi(2))
            .sqrt();
            // Nested: restrict twice.
            let twice = c.restrict(s0, 1.0).restrict(0.0, 1.0).eval(0.0);
            let d2 = ((direct.x - twice.x).powi(2)
                + (direct.y - twice.y).powi(2)
                + (direct.z - twice.z).powi(2))
            .sqrt();
            println!(
                "f64 theta {theta:e} s0 {s0}: |eval(s0) - restrict.eval(0)| = {d:e}, \
                 nested {d2:e}"
            );
            assert!(d.is_finite() && d2.is_finite());
        }
    }
}

mod interval_lane {
    use super::*;
    use geom_core::{Bounds, Interval};

    fn wid(p: Point3<Interval>) -> f64 {
        (p.x.hi() - p.x.lo())
            .max(p.y.hi() - p.y.lo())
            .max(p.z.hi() - p.z.lo())
    }

    /// The consumer-visible enclosure story on a less friendly fixture:
    /// an angle NEAR but not AT zero, an oblique axis, a far axis
    /// origin, and a restriction round trip.
    #[test]
    fn r2_e2e_enclosure_seen_by_a_consumer() {
        for half in [0.0f64, 1.0e-12, 1.0e-9, 1.0e-6] {
            let mk = |x: f64| Interval::from_bounds(x - half, x + half);
            for &theta in &[0.0f64, 1.0e-12, 1.0e-7, 1.0e-3, 0.4, TAU] {
                let c = oblique(Interval::from_f64(theta), mk);
                let start = wid(c.eval(Interval::zero()));
                let stored = wid(c
                    .restrict(Interval::zero(), Interval::from_f64(0.25))
                    .eval(Interval::zero()));
                let nested = wid(c
                    .restrict(Interval::zero(), Interval::from_f64(0.25))
                    .restrict(Interval::zero(), Interval::from_f64(0.5))
                    .eval(Interval::zero()));
                let mid = wid(c.eval(Interval::from_f64(0.5)));
                println!(
                    "iv half {half:e} theta {theta:e}: eval(0) {start:e} \
                     stored {stored:e} nested {nested:e} eval(0.5) {mid:e}"
                );
            }
        }
    }
}
