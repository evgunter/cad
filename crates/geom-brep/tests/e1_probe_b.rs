//! Reviewer probes for PCERT E1 (PR 5500). Not for merge.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::recourse::Reading;
use geom_brep::{Pcurve, PcurveCache, PcurveCertifyError, chart_pcurve};
use geom_core::{Decide, Interval, Point3, Real, Vec3};

fn mid() -> f64 {
    0.5 * (band().zero() + band().escalate())
}

fn show<T: Decide>(what: &str, got: &Result<Pcurve<T>, PcurveCertifyError>) -> String {
    let s = match got {
        Ok(Pcurve::Projected(_)) => "Ok(Projected)".to_string(),
        Ok(p) => format!("Ok({:?})", core::mem::discriminant(p)),
        Err(e) => format!(
            "Err({e:?})\n    display: {e}\n    ending(Build): {:?}",
            e.ending(Reading::Build)
        ),
    };
    println!("[probe] {what}: {s}");
    s
}

fn cone<T: Real>(at: &impl Fn(f64) -> T) -> Surface<T> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: 0.5,
        u_ref: Vec3::unit_x(),
    }
    .map_scalar(at)
}

fn lanes(name: &str, f: impl Fn(&dyn Fn(f64) -> f64) -> String, g: impl Fn(&dyn Fn(f64) -> Interval) -> String) -> (String, String) {
    let a = f(&|x| x);
    let b = g(&<Interval as Real>::from_f64);
    let _ = name;
    (a, b)
}

/// A line 5 m off a cone whose ORIGIN sits mid-band off the apex height.
#[test]
fn p1_cone_line_off_chart_origin_in_band() {
    fn read<T: Decide>(at: &dyn Fn(f64) -> T, slope: f64, shift: f64) -> String {
        let dir = Vec3::new(0.0, 1.0, slope).normalize();
        let origin = Point3::new(5.0, 0.0, mid()) + dir * shift;
        let line = Curve3::Line { origin, dir }.map_scalar(at);
        let s = cone(&at);
        let got = chart_pcurve(&line, &s, band());
        let got = match got {
            Ok(img) => PcurveCache::certify(img.clone(), at(0.0), at(1.0), &line, &s, band()).map(|_| img),
            e => e,
        };
        show(&format!("cone line slope {slope} shift {shift}"), &got)
    }
    for slope in [0.0, 1e-3] {
        for shift in [0.0, 1.0] {
            let _ = lanes("p1", |at| read(at, slope, shift), |at| read(at, slope, shift));
        }
    }
}

/// A circle off a cylinder (only touches at two points) whose
/// orientation reading sits mid-band.
#[test]
fn p2_cylinder_circle_off_chart_orientation_in_band() {
    fn read<T: Decide>(at: &dyn Fn(f64) -> T) -> String {
        let nz = mid();
        let n = Vec3::new(0.0, (1.0 - nz * nz).sqrt(), nz);
        let c = Curve3::Circle {
            center: Point3::origin(),
            axis: n,
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        }
        .map_scalar(at);
        let s = Surface::Cylinder {
            origin: Point3::origin(),
            axis: Vec3::unit_z(),
            radius: 1.0,
            u_ref: Vec3::unit_x(),
        }
        .map_scalar(at);
        show("cylinder circle", &chart_pcurve(&c, &s, band()))
    }
    let _ = lanes("p2", |at| read(at), |at| read(at));
}

fn unit_sphere<T: Real>(at: &dyn Fn(f64) -> T) -> Surface<T> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: 1.0,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
    .map_scalar(at)
}

fn sphere_cert<T: Decide>(what: &str, at: &dyn Fn(f64) -> T, c: Curve3<f64>) -> String {
    let c = c.map_scalar(at);
    let s = unit_sphere(at);
    let got = chart_pcurve(&c, &s, band());
    let got = match got {
        Ok(img) => PcurveCache::certify(img.clone(), at(0.0), at(core::f64::consts::TAU), &c, &s, band())
            .map(|_| img),
        e => e,
    };
    show(what, &got)
}

/// Near-parallel / near-meridian / off-centre circles on the sphere,
/// each class reading a band-width (several multiples) off.
#[test]
fn p3_sphere_in_band_circles_take_the_projected_lane() {
    let mut bad = vec![];
    for k in [0.3, 0.5, 0.9, 1.0, 1.5, 3.0] {
        let m = band().zero() + k * (band().escalate() - band().zero());
        // near-parallel, tilted by m (axial sup = rho·sinθ = m)
        let (h, rho) = (0.6, 0.8);
        let th = (m / rho).asin();
        let n = Vec3::new(0.0, -th.sin(), th.cos());
        let par = Curve3::Circle { center: Point3::origin() + n * h, axis: n, radius: rho, u_ref: Vec3::unit_x() };
        // near-meridian great circle, coax reading m
        let n2 = Vec3::new(0.0, (1.0 - m * m).sqrt(), m);
        let mer = Curve3::Circle { center: Point3::origin(), axis: n2, radius: 1.0, u_ref: Vec3::unit_x() };
        // meridian-plane circle off centre by m
        let off = Curve3::Circle { center: Point3::new(0.0, m, 0.0), axis: Vec3::unit_y(), radius: (1.0 - m * m).sqrt(), u_ref: Vec3::unit_x() };
        // near-polar small circle centred off the axis by m (centred gate)
        let pol = Curve3::Circle { center: Point3::new(m, 0.0, 0.0) + Vec3::unit_z() * 0.6, axis: Vec3::unit_z(), radius: (1.0f64 - 0.36 - m*m).sqrt(), u_ref: Vec3::unit_x() };
        for (name, c) in [("parallel", par), ("meridian", mer), ("offcentre", off), ("polar-offaxis", pol)] {
            for (lane, s) in [
                ("f64", sphere_cert(&format!("{name} k={k} f64"), &|x| x, c.clone())),
                ("Interval", sphere_cert(&format!("{name} k={k} Interval"), &<Interval as Real>::from_f64, c.clone())),
            ] {
                if !s.starts_with("Ok(") {
                    bad.push(format!("{name} k={k} {lane}: {s}"));
                }
            }
        }
    }
    println!("[probe] p3 not-ok: {}", bad.len());
    for b in &bad { println!("[probe]   {b}"); }
}

/// A cone circle whose rim gate reads mid-band, moved 1e-3 off the cone.
#[test]
fn p4_cone_circle_off_chart_with_in_band_gate() {
    fn read<T: Decide>(at: &dyn Fn(f64) -> T, push: f64, centre_off: bool) -> String {
        let h = 2.0;
        let r = h * 0.5f64.tan() + push;
        let (n, c) = if centre_off {
            (Vec3::unit_z(), Point3::new(mid(), 0.0, h))
        } else {
            let th = (mid() / r).asin();
            (Vec3::new(0.0, -th.sin(), th.cos()), Point3::new(0.0, 0.0, h))
        };
        let circ = Curve3::Circle { center: c, axis: n, radius: r, u_ref: Vec3::unit_x() }.map_scalar(at);
        show(&format!("cone circle push {push} centre_off {centre_off}"), &chart_pcurve(&circ, &cone(&at), band()))
    }
    for push in [0.0, 1e-3] {
        for co in [false, true] {
            let _ = lanes("p4", |at| read(at, push, co), |at| read(at, push, co));
        }
    }
}

/// A torus crest parallel whose centred gate reads mid-band, moved
/// 1e-3 off; and its tilt gate mid-band, moved off.
#[test]
fn p5_torus_circle_off_chart_with_in_band_gate() {
    fn read<T: Decide>(at: &dyn Fn(f64) -> T, push: f64, tilt: bool) -> String {
        let t = Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: 3.0,
            minor_radius: 1.0,
            u_ref: Vec3::unit_x(),
        }
        .map_scalar(at);
        let r = 3.0 + push;
        let (n, c) = if tilt {
            let th = (mid() / r).asin();
            (Vec3::new(0.0, -th.sin(), th.cos()), Point3::new(0.0, 0.0, 1.0))
        } else {
            (Vec3::unit_z(), Point3::new(mid(), 0.0, 1.0))
        };
        let circ = Curve3::Circle { center: c, axis: n, radius: r, u_ref: Vec3::unit_x() }.map_scalar(at);
        show(&format!("torus circle push {push} tilt {tilt}"), &chart_pcurve(&circ, &t, band()))
    }
    for push in [0.0, 1e-3] {
        for tilt in [false, true] {
            let _ = lanes("p5", |at| read(at, push, tilt), |at| read(at, push, tilt));
        }
    }
}

/// Zero degeneracies: rim exactly at the apex, mid-band; line in apex plane exactly / mid-band slope.
#[test]
fn p7_zero_degeneracies_and_their_margins() {
    fn rim<T: Decide>(at: &dyn Fn(f64) -> T, h: f64) -> String {
        let c = Curve3::Circle { center: Point3::new(0.0, 0.0, h), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() }.map_scalar(at);
        show(&format!("rim at h={h:e}"), &chart_pcurve(&c, &cone(&at), band()))
    }
    fn line<T: Decide>(at: &dyn Fn(f64) -> T, h: f64, slope: f64) -> String {
        let l = Curve3::Line { origin: Point3::new(5.0, 0.0, h), dir: Vec3::new(0.0, 1.0, slope).normalize() }.map_scalar(at);
        show(&format!("line h={h:e} slope={slope:e}"), &chart_pcurve(&l, &cone(&at), band()))
    }
    fn ell<T: Decide>(at: &dyn Fn(f64) -> T, h: f64) -> String {
        let e = Curve3::Ellipse { center: Point3::new(0.0, 0.0, h), axis: Vec3::new(0.1, 0.0, 1.0).normalize(), semi_u: 1.0, semi_v: 0.5, u_ref: Vec3::new(1.0, 0.0, -0.1).normalize() }.map_scalar(at);
        show(&format!("ellipse centred h={h:e}"), &chart_pcurve(&e, &cone(&at), band()))
    }
    for h in [0.0, mid()] {
        let _ = lanes("rim", |at| rim(at, h), |at| rim(at, h));
        let _ = lanes("ell", |at| ell(at, h), |at| ell(at, h));
    }
    let _ = lanes("line", |at| line(at, 0.0, 0.0), |at| line(at, 0.0, 0.0));
    let _ = lanes("line", |at| line(at, 0.0, mid()), |at| line(at, 0.0, mid()));
}

/// The ending a near-meridian in-band circle on the sphere gets.
#[test]
fn p3b_near_meridian_ending() {
    for k in [0.5, 1.0] {
        let m = band().zero() + k * (band().escalate() - band().zero());
        let n2 = Vec3::new(0.0, (1.0 - m * m).sqrt(), m);
        let mer = Curve3::Circle { center: Point3::origin(), axis: n2, radius: 1.0, u_ref: Vec3::unit_x() };
        let s = unit_sphere(&|x| x);
        let got = chart_pcurve(&mer, &s, band());
        let _ = show(&format!("near-meridian k={k} chart_pcurve"), &got);
        // exact meridian, exact great tilted circle for reference
        for (nm, n) in [("exact meridian", Vec3::unit_y()), ("tilted 0.3", Vec3::new(0.0, 0.3f64.cos(), 0.3f64.sin()))] {
            let c = Curve3::Circle { center: Point3::origin(), axis: n, radius: 1.0, u_ref: Vec3::unit_x() };
            let _ = show(nm, &chart_pcurve(&c, &s, band()));
        }
    }
}
