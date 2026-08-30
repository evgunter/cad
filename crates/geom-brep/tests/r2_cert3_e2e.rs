//! CERT-3 review lane R2 — the required e2e exercise. My own fixtures,
//! not the unit's: an OBLIQUE axis, an axis origin far from the point,
//! a PARTIAL sweep, nested `restrict` (a stored placement round trip
//! through two compositions), and an angle NEAR but not AT zero.
//!
//! Runs at BOTH lanes. The unit's own consumer file is
//! `#![cfg(feature = "interval")]`, so nothing it ships exercises
//! `RevolvedPoint` at `f64`; this file does.
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
        angle,
    }
}

/// f64 lane: what does a consumer see? The load-bearing consumer
/// property is that `restrict(s0, s1).eval(0)` agrees with
/// `eval(s0)` — the stored placement round trip.
#[test]
fn r2_e2e_stored_placement_round_trip_f64() {
    for &theta in &[0.0f64, 1.0e-12, 1.0e-7, 1.0e-3, 0.4, TAU] {
        let c = oblique(theta, |x| x);
        for &s0 in &[0.0f64, 0.125, 0.5] {
            let direct = c.eval(s0);
            let stored = c.restrict(s0, 1.0).eval(0.0);
            let d = ((direct.x - stored.x).powi(2)
                + (direct.y - stored.y).powi(2)
                + (direct.z - stored.z).powi(2))
            .sqrt();
            // Nested: restrict twice, i.e. two stored compositions.
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

#[cfg(feature = "interval")]
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
    /// origin, and a stored placement round trip.
    #[test]
    fn r2_e2e_enclosure_seen_by_a_consumer() {
        for half in [0.0f64, 1.0e-12, 1.0e-9, 1.0e-6] {
            let mk = |x: f64| Interval::from_bounds(x - half, x + half);
            for &theta in &[0.0f64, 1.0e-12, 1.0e-7, 1.0e-3, 0.4, TAU] {
                let c = oblique(Interval::from_f64(theta), mk);
                let start = wid(c.eval(Interval::zero()));
                let stored = wid(c.restrict(Interval::zero(), Interval::from_f64(0.25)).eval(Interval::zero()));
                let nested = wid(
                    c.restrict(Interval::zero(), Interval::from_f64(0.25))
                        .restrict(Interval::zero(), Interval::from_f64(0.5))
                        .eval(Interval::zero()),
                );
                let mid = wid(c.eval(Interval::from_f64(0.5)));
                println!(
                    "iv half {half:e} theta {theta:e}: eval(0) {start:e} \
                     stored {stored:e} nested {nested:e} eval(0.5) {mid:e}"
                );
            }
        }
    }
}

#[cfg(feature = "interval")]
mod restrict_accumulation {
    use super::*;
    use geom_core::{Bounds, Interval};

    /// The consumer behaviour the unit's fixtures cannot show: `restrict`
    /// is what `split_edge` calls, and it STORES a composed placement.
    /// On a fixture where every input is exact, how does the stored
    /// enclosure behave as an edge is split repeatedly?
    #[test]
    fn r2_e2e_repeated_restriction_accumulates_width() {
        let ex = |x: f64| Interval::from_f64(x);
        let mut c = MappedCurve::RevolvedPoint {
            point: Point2::new(ex(2.0), ex(2.0)),
            place: Affine3::translation(Vec3::new(ex(0.0), ex(0.0), ex(3.0))),
            axis_origin: Point3::new(ex(1.0), ex(2.0), ex(3.0)),
            axis_dir: Vec3::new(ex(0.0), ex(0.0), ex(1.0)),
            angle: ex(TAU),
        };
        // The unit's OWN fixture, exact axis — then split it repeatedly
        // from the start, which is what a consumer does.
        for i in 0..=8 {
            let p = c.eval(Interval::zero());
            let w = (p.x.hi() - p.x.lo())
                .max(p.y.hi() - p.y.lo())
                .max(p.z.hi() - p.z.lo());
            println!("after {i} restrictions: eval(0) width {w:e}");
            c = c.restrict(ex(0.0), ex(0.5));
        }
    }
}
