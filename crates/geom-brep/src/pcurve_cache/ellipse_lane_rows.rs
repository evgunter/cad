//! probe
use geom_core::{Bounds, Interval};

use super::circle_lane_rows::{
    certifies_projected, cone, cylinder, derived, in_band, torus, world, world_dir,
};
use super::harmonic_sup_rows::{band, zero};
use super::*;

/// The ellipse centred at `c` with semi-axis `a` along the unit `u` and
/// `b` along the unit `v` (`u ⟂ v`), in `surface`'s chart frame.
pub(super) fn framed_ellipse(
    surface: &Surface<f64>,
    c: Vec3<f64>,
    (u, a): (Vec3<f64>, f64),
    (v, b): (Vec3<f64>, f64),
) -> Curve3<f64> {
    let (u, v) = (world_dir(surface, u), world_dir(surface, v));
    Curve3::Ellipse(geom::Ellipse::new(
        geom::Frame::new(world(surface, c), u.cross(v).normalize(), u.normalize()),
        a,
        b,
    ))
}

fn sphere(radius: f64) -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.2, -0.5, 0.1),
        radius,
        axis: Vec3::new(0.1, 0.2, 1.0).normalize(),
        u_ref: Vec3::new(1.0, 0.0, -0.1).normalize(),
    }
}

/// The needle ellipse along the cone's generator at azimuth `0` on the
/// nappe `sigma` names: centred at slant `s` from the apex, its major
/// semi-axis `a` along the generator, its minor `b` across it, and
/// stood `off` along the cone's outward normal.
pub(super) fn cone_needle(surface: &Surface<f64>, sigma: f64, s: f64, (a, b): (f64, f64), off: f64) -> Curve3<f64> {
    let Surface::Cone { half_angle, .. } = *surface else {
        unreachable!("a cone needle is a cone's")
    };
    let (sa, ca) = half_angle.sin_cos();
    let g = Vec3::new(sa, 0.0, ca * sigma);
    let n = Vec3::new(ca, 0.0, -sa * sigma);
    framed_ellipse(surface, g * s + n * off, (g, a), (Vec3::unit_y(), b))
}

fn show(what: &str, e: &Curve3<f64>, sf: &Surface<f64>) {
    let (x, y) = derived(e, (0.3, 2.5), sf);
    eprintln!("{what}: {:?} / {:?}", x.as_ref().map(|p| matches!(p, Pcurve::Projected(_))), y.as_ref().map(|p| matches!(p, Pcurve::Projected(_))));
    if matches!(x, Ok(Pcurve::Projected(_))) && matches!(y, Ok(Pcurve::Projected(_))) {
        let r = std::panic::catch_unwind(|| certifies_projected(what, e, (0.3, 2.5), sf));
        eprintln!("  {:?}", r.map_err(|_| "panicked"));
    } else {
        eprintln!("  {x:?}");
        if let Ok(img) = x {
            eprintln!("  certify: {:?}", PcurveCache::certify(img, 0.3, 2.5, e, sf, band()).map(|c| c.certificate().envelope));
        }
    }
}

#[test]
fn probe_tiny() {
    let e = zero();
    let a = (0.2 * e).sqrt();
    let sph = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let el = Curve3::Ellipse(geom::Ellipse::new(geom::Frame::new(Point3::new(0.0, 0.0, 1.0 - a * a / 4.0), Vec3::unit_z(), Vec3::unit_x()), a, 0.5 * a));
    for c in [chart_pcurve(&el, &sph, band()).map(|_| ())] { eprintln!("tiny sphere: {c:?}"); }
    let tor = Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: 2.0, minor_radius: 0.5, u_ref: Vec3::unit_x() };
    let b = (0.05 * e).sqrt();
    let el = Curve3::Ellipse(geom::Ellipse::new(geom::Frame::new(Point3::new(2.5 - 0.05 * e, 0.0, 0.0), Vec3::unit_x(), Vec3::unit_z()), b, 0.5 * b));
    let r = chart_pcurve(&el, &tor, band());
    eprintln!("tiny torus: {:?}", r.as_ref().map(|_| ()));
    if let Ok(img) = r { eprintln!("  certify {:?}", PcurveCache::certify(img, 0.0, 6.2, &el, &tor, band()).map(|c| c.certificate().envelope)); }
}

#[test]
fn probe() {
    let esc = band().escalate();
    for sigma in [1.0, -1.0] {
        let c = cone(0.5);
        let Surface::Cone { half_angle, .. } = c else { unreachable!() };
        let (sa, ca) = half_angle.sin_cos();
        let g = Vec3::new(sa, 0.0, ca * sigma);
        let n = Vec3::new(ca, 0.0, -sa * sigma);
        for (b, cx) in [(1.002 * esc, 0.998), (1.0005 * esc, 0.9993), (0.5 * esc, 1.0)] {
            let sx = (1.0f64 - cx * cx).sqrt();
            let v = Vec3::unit_y() * cx + n * sx;
            let e = framed_ellipse(&c, g * 1.2, (g, 0.3), (v, b));
            show(&format!("cone {sigma} b={b:e} cos={cx}"), &e, &c);
        }
    }
    let cy = cylinder(1.0);
    for b in [3e-5, 0.5 * esc] {
        let e = framed_ellipse(&cy, Vec3::new(1.0, 0.0, 0.5), (Vec3::unit_z(), 0.3), (Vec3::unit_y(), b));
        show(&format!("cyl tangent b={b:e}"), &e, &cy);
        let e = framed_ellipse(&cy, Vec3::new(1.0, 0.0, 0.5), (Vec3::unit_z(), 0.3), (Vec3::unit_x(), b));
        show(&format!("cyl meridian b={b:e}"), &e, &cy);
    }
    let t = torus(2.0, 0.5);
    for d in [1e-5, 3e-5] {
        let e = framed_ellipse(&t, Vec3::new(0.0, 0.0, 0.5), (Vec3::unit_x(), 2.0), (Vec3::unit_y(), 2.0 - d));
        show(&format!("torus d={d:e}"), &e, &t);
    }
}

#[test]
fn probe_sphere() {
    eprintln!("zero {:e} in_band {:e} escalate {:e}", zero(), in_band(), band().escalate());
    let s = sphere(1.0);
    let (z0, r0) = (0.6, 0.8);
    for k in [0.4, 3.0] {
        let delta = k * zero() / r0;
        let e = framed_ellipse(&s, Vec3::new(0.0, 0.0, z0), (Vec3::unit_x(), r0), (Vec3::unit_y(), r0 - delta * r0));
        let (a, b) = derived(&e, (0.3, 2.5), &s);
        eprintln!("sphere k={k}: {:?} / {:?}", a.as_ref().map(|_| "ok"), b.as_ref().map(|_| "ok"));
        if a.is_ok() && b.is_ok() {
            eprintln!("  {:?}", certifies_projected("sphere", &e, (0.3, 2.5), &s));
        }
    }
}
