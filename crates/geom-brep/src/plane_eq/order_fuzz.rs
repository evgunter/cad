//! **The one-plane reading is a symmetric function of the pair**: a
//! counterexample search over plane pairs near the band, each read in
//! both operand orders (module row below).

#![allow(clippy::unwrap_used, clippy::panic)]

test_utils::gated_to![
    "crates/geom-brep/src/plane_eq.rs",
    "crates/geom-brep/src/carrier_eq.rs",
];

use super::{PlaneDesc, plane_ladder};
use crate::carrier_eq::{CarrierDesc, ConsumedExtent, pair_door_reading};
use geom_core::{Band, Decide, Interval, Point3, Real, Tol, Vec3};
use test_utils::fuzz::Rng;

fn unit(rng: &mut Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        let n = v.norm();
        if n > 0.1 && n < 1.0 {
            return v / n;
        }
    }
}

fn perp(n: Vec3<f64>) -> Vec3<f64> {
    let a = if n.x.abs() < 0.9 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    let u = n.cross(a);
    u / u.norm()
}

/// A pair near the band over a ball of radius `r`: the first plane
/// through a point of the ball, the second tilted by up to `2·z/r`
/// about an in-plane axis through a point of the ball and offset by up
/// to `2.5·z`, each origin stated up to a thousand radii along its
/// plane, the second's normal flipped half the time.
fn pose(rng: &mut Rng, r: f64, z: f64) -> ([PlaneDesc<f64>; 2], Point3<f64>) {
    let c = Point3::new(
        rng.range(-10.0, 10.0),
        rng.range(-10.0, 10.0),
        rng.range(-10.0, 10.0),
    );
    let n1 = unit(rng);
    let foot1 = c + n1 * rng.range(-r, r);
    let u = perp(n1);
    let ang = rng.range(0.0, std::f64::consts::TAU);
    let axis = u * ang.cos() + n1.cross(u) * ang.sin();
    let tilt = rng.range(-2.0, 2.0) * z / r;
    let side = axis.cross(n1);
    let n2 = n1 * tilt.cos() + side * tilt.sin();
    let n2 = n2 / n2.norm();
    let pivot = foot1 + axis * rng.range(-r, r) + side * rng.range(-r, r);
    let off = rng.range(-2.5, 2.5) * z;
    let flip = if rng.unit() < 0.5 { -1.0 } else { 1.0 };
    let far = [0.0, 1.0, 30.0, 1e3][rng.below(4)] * r;
    (
        [
            PlaneDesc {
                origin: foot1 + perp(n1) * rng.range(-far, far),
                normal: n1,
            },
            PlaneDesc {
                origin: pivot + n2 * off + perp(n2) * far,
                normal: n2 * flip,
            },
        ],
        c,
    )
}

/// The ladder's and the pair door's readings of `a` against `b`, as
/// printed: verdicts and margins to the bit.
fn readings<T: Decide>(
    a: &PlaneDesc<T>,
    b: &PlaneDesc<T>,
    extent: &ConsumedExtent<T>,
    band: Band,
) -> String {
    format!(
        "{:?} | {:?}",
        plane_ladder(a, b, extent, band),
        pair_door_reading(
            &CarrierDesc::Plane(*a),
            &CarrierDesc::Plane(*b),
            extent,
            band
        )
    )
}

/// **Exchanging the two planes changes nothing**: over poses within a
/// few bands of one plane, at radii from `2e-8` to `1e3`, the ladder's
/// and the pair door's readings are the same to the bit with the planes
/// exchanged, at `f64` and (every twentieth pose) at `Interval`: the
/// verdict, the relation and every margin. Red where the offset or the
/// bound is read in one order only (`d₁ − (n₁·n₂)·d₂`): the two orders'
/// margins part at second order in the tilt, and the verdict with them
/// at the band's edge, about once in three thousand poses.
#[test]
fn a_plane_pair_reads_alike_in_either_order() {
    let band = Band::linear(Tol::witness()).unwrap();
    let z = band.zero();
    let mut rng = test_utils::fuzz::start("plane-pair-order");
    for case in 0..test_utils::fuzz::scaled(20_000) {
        let r = [2e-8, 1e-6, 1e-3, 1.0, 1e3][case % 5];
        let ([a, b], c) = pose(&mut rng, r, z);
        let extent = ConsumedExtent::new(crate::ExtentBall::fixture(c, r));
        assert_eq!(
            readings(&a, &b, &extent, band),
            readings(&b, &a, &extent, band),
            "f64, case {case}, R {r:e}: {a:?} {b:?} about {c:?}; {}",
            test_utils::fuzz::replay()
        );
        if case % 20 == 0 {
            let lift = |p: &PlaneDesc<f64>| PlaneDesc {
                origin: Point3::new(
                    Interval::from_f64(p.origin.x),
                    Interval::from_f64(p.origin.y),
                    Interval::from_f64(p.origin.z),
                ),
                normal: Vec3::new(
                    Interval::from_f64(p.normal.x),
                    Interval::from_f64(p.normal.y),
                    Interval::from_f64(p.normal.z),
                ),
            };
            let (ai, bi) = (lift(&a), lift(&b));
            let extent = ConsumedExtent::new(crate::ExtentBall::fixture(
                Point3::new(
                    Interval::from_f64(c.x),
                    Interval::from_f64(c.y),
                    Interval::from_f64(c.z),
                ),
                Interval::from_f64(r),
            ));
            assert_eq!(
                readings(&ai, &bi, &extent, band),
                readings(&bi, &ai, &extent, band),
                "Interval, case {case}, R {r:e}: {a:?} {b:?} about {c:?}; {}",
                test_utils::fuzz::replay()
            );
        }
    }
}
