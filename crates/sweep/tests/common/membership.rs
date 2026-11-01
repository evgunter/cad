//! **A built body against its analytic region**: its volume against the
//! closed form, tiers 3 and 3′, and `point_in_solid` at a lattice of
//! points against a region read as a signed margin (positive inside).
//! The oracle a row adopts where a door moves from a refusal to a build,
//! so the build is shown right, not only present.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Point3, Tol};
use topo::{
    AtRestBody, ContactRecords, SolidContainment, point_in_solid, validate_geometric,
    validate_pseudomanifold,
};

/// A region, read as a signed margin: positive inside, negative outside.
pub type Region = Box<dyn Fn(Point3<f64>) -> f64>;

/// The ball of radius `r` about `c`.
pub fn ball(c: Point3<f64>, r: f64) -> Region {
    Box::new(move |p| r - (p - c).norm())
}

/// The box over `x × y × z`.
pub fn cuboid(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Region {
    Box::new(move |p: Point3<f64>| {
        (p.x - x.0)
            .min(x.1 - p.x)
            .min(p.y - y.0)
            .min(y.1 - p.y)
            .min(p.z - z.0)
            .min(z.1 - p.z)
    })
}

/// Holds `b` to the volume `want` (within its pad and rounding), to
/// tiers 3 and 3′, and to `region` at the `n³` lattice over `lo..hi`,
/// each point off the region's boundary by `1e-3·|hi − lo|` asked
/// `point_in_solid`.
pub fn holds(
    what: &str,
    b: &AtRestBody<f64>,
    want: f64,
    region: &dyn Fn(Point3<f64>) -> f64,
    (lo, hi): (Point3<f64>, Point3<f64>),
    n: usize,
) {
    let tol = Tol::witness();
    let mp = topo::test_support::pinned_volume(what, b, tol);
    let rounding = 64.0 * f64::EPSILON * want.abs().max(1.0);
    assert!(
        (mp.volume - want).abs() <= mp.volume_pad + rounding,
        "{what}: holds {} ± {:e}, not {want}",
        mp.volume,
        mp.volume_pad
    );
    assert_eq!(validate_geometric(b, tol), Ok(()), "{what}: tier 3");
    assert_eq!(
        validate_pseudomanifold(b, &ContactRecords::default(), tol),
        Ok(()),
        "{what}: tier 3′"
    );
    let band = Band::linear(tol).unwrap();
    let span = hi - lo;
    let off = 1e-3 * span.norm();
    // Offsets off the lattice's own symmetry, so no point sits on a
    // plane of the region by construction.
    let at = |t: usize, o: f64, a: f64, w: f64| a + w * (t as f64 + o) / (n as f64 - 1.0 + 0.37);
    let mut asked = 0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                let p = Point3::new(
                    at(i, 0.11, lo.x, span.x),
                    at(j, 0.23, lo.y, span.y),
                    at(k, 0.07, lo.z, span.z),
                );
                let s = region(p);
                if s.abs() < off {
                    continue;
                }
                asked += 1;
                let inside = if s > 0.0 {
                    SolidContainment::In
                } else {
                    SolidContainment::Out
                };
                assert_eq!(
                    point_in_solid(b, p, band, tol),
                    Ok(inside),
                    "{what}: places {p:?} (region {s})"
                );
            }
        }
    }
    assert!(
        asked * 10 > n * n * n * 8,
        "{what}: the lattice asked {asked} of {} points",
        n * n * n
    );
}
