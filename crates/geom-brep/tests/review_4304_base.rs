//! Review probe (PR 4304) at the merge base: the Hermite route on
//! encircling tilted circles.
use geom::{Curve3, Surface};
use geom_brep::{FittedLane, PcurveCache};
use geom_core::tolerance::Tol;
use geom_core::{Band, Bounds, Interval, Point3, Real, Vec3};

fn tilted(tilt: f64, d: f64) -> Curve3<f64> {
    let n = Vec3::new(tilt.sin(), 0.0, tilt.cos());
    Curve3::Circle {
        center: Point3::origin() + n * d,
        axis: n,
        radius: (1.0 - d * d).sqrt(),
        u_ref: Vec3::new(tilt.cos(), 0.0, -tilt.sin()),
    }
}

#[test]
fn review_base_hermite_encircling() {
    let b = Band::linear(Tol::witness()).unwrap();
    let s = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let tau = std::f64::consts::TAU;
    for (name, c, t0, t1) in [
        ("encircling full turn", tilted(0.3, 0.2), 0.0, tau),
        ("encircling 0.95 turn", tilted(0.3, 0.2), 0.0, 0.95 * tau),
        ("encircling 0.8 turn", tilted(0.3, 0.2), 0.0, 0.8 * tau),
        ("encircling 0.7 turn", tilted(0.3, 0.2), 0.0, 0.7 * tau),
        ("non-encircling full turn", tilted(1.2, 0.5), 0.0, tau),
    ] {
        let lane = FittedLane::<f64>::certified();
        let r = lane.sphere_circle_image(&c, t0, t1, &s, b).and_then(|img| {
            PcurveCache::certify_fitted(std::sync::Arc::new(img), t0, t1, &c, &s, None, b, lane)
        });
        let ci = c.map_scalar(Interval::from_f64);
        let si = s.map_scalar(Interval::from_f64);
        let (i0, i1) = (Interval::from_f64(t0), Interval::from_f64(t1));
        let li = FittedLane::<Interval>::certified();
        let ri = li.sphere_circle_image(&ci, i0, i1, &si, b).and_then(|img| {
            PcurveCache::certify_fitted(std::sync::Arc::new(img), i0, i1, &ci, &si, None, b, li)
        });
        eprintln!("[review base] {name}: f64 {:?} | iv {:?}", r.map(|c| c.certificate().envelope), ri.map(|c| c.certificate().envelope.hi()));
    }
}
