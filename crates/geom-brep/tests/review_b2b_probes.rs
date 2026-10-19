//! Reviewer probes for PCERT B2b (PR 5343): check 4 decides no whole
//! period. Each probe certifies a stored row moved off its placed image
//! and prints the outcome at f64 and at Interval.


use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom::{Curve3, Surface};
use geom_brep::{
    EnvelopeTerm, Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck, Period, chart_pcurve_over,
};
use geom_core::{Band, Interval, Point3, Real, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn read<T: std::fmt::Debug>(r: &Result<T, PcurveCertifyError>) -> String {
    match r {
        Ok(_) => "certifies".into(),
        Err(PcurveCertifyError::ResidualExceeded {
            check: PcurveCheck::EnvelopeTerm(t),
            ..
        }) => format!("refuses {t:?}"),
        Err(e) => format!("OTHER {e:?}"),
    }
}

/// Certify `image(T)` shifted by (du, dv) at both lanes.
fn both(
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
    (t0, t1): (f64, f64),
    du: f64,
    dv: f64,
    twin: bool,
) -> (String, String) {
    let f = Interval::from_f64;
    let row64 = {
        let p = chart_pcurve_over(carrier, t0, t1, surface, band()).unwrap();
        let p = if twin {
            p.map_affine(geom_brep::ChartMap::sphere_involution()).unwrap()
        } else {
            p
        };
        p.translate(du, dv)
    };
    let a = read(&PcurveCache::certify(row64, t0, t1, carrier, surface, band()));
    let ci = carrier.map_scalar(f);
    let si = surface.map_scalar(f);
    let rowi = {
        let p = chart_pcurve_over(&ci, f(t0), f(t1), &si, band()).unwrap();
        let p = if twin {
            p.map_affine(geom_brep::ChartMap::sphere_involution()).unwrap()
        } else {
            p
        };
        p.translate(f(du), f(dv))
    };
    let b = read(&PcurveCache::certify(rowi, f(t0), f(t1), &ci, &si, band()));
    (a, b)
}

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::origin(),
        axis: Vec3::unit_z(),
        radius: 0.5,
        u_ref: Vec3::unit_x(),
    }
}

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::origin(),
        radius: 0.5,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

fn torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::origin(),
        axis: Vec3::unit_z(),
        major_radius: 2.0,
        minor_radius: 0.5,
        u_ref: Vec3::unit_x(),
    }
}

fn circle(c: Point3<f64>, r: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: c,
        axis: Vec3::unit_z(),
        radius: r,
        u_ref: Vec3::unit_x(),
    }
}

#[test]
fn probe_whole_period_offsets() {
    let zero = band().zero();
    let rim = circle(Point3::new(0.0, 0.0, 0.3), 0.5);
    let seam = Curve3::Line {
        origin: Point3::new(0.5, 0.0, 0.0),
        dir: Vec3::unit_z(),
    };
    let v0 = 0.4_f64;
    let parallel = circle(Point3::new(0.0, 0.0, 0.5 * v0.sin()), 0.5 * v0.cos());
    let tparallel = circle(Point3::new(0.0, 0.0, 0.5), 2.0);
    let mut out = Vec::new();
    let span = (0.2, 2.9);
    let big = f64::from(1 << 20) * TAU;
    let imax = f64::from(i32::MAX) * TAU;
    let cases: Vec<(&str, &Curve3<f64>, Surface<f64>, (f64, f64), f64, f64, bool)> = vec![
        ("cyl rim exact", &rim, cylinder(), span, 0.0, 0.0, false),
        ("cyl rim u+τ", &rim, cylinder(), span, TAU, 0.0, false),
        ("cyl rim u−τ", &rim, cylinder(), span, -TAU, 0.0, false),
        ("cyl rim u+3τ", &rim, cylinder(), span, 3.0 * TAU, 0.0, false),
        ("cyl rim u−3τ", &rim, cylinder(), span, -3.0 * TAU, 0.0, false),
        ("cyl rim u+2^20τ", &rim, cylinder(), span, big, 0.0, false),
        ("cyl rim u+i32::MAXτ", &rim, cylinder(), span, imax, 0.0, false),
        ("cyl rim u+0.5 zero/r", &rim, cylinder(), span, 0.5 * zero / 0.5, 0.0, false),
        ("cyl rim u+4 zero/r", &rim, cylinder(), span, 4.0 * zero / 0.5, 0.0, false),
        ("cyl rim u+τ+0.5zero/r", &rim, cylinder(), span, TAU + zero, 0.0, false),
        ("seam u+τ", &seam, cylinder(), (0.0, 1.0), TAU, 0.0, false),
        ("seam u−τ", &seam, cylinder(), (0.0, 1.0), -TAU, 0.0, false),
        ("seam exact", &seam, cylinder(), (0.0, 1.0), 0.0, 0.0, false),
        ("torus parallel v+τ", &tparallel, torus(), span, 0.0, TAU, false),
        ("torus parallel u+τ", &tparallel, torus(), span, TAU, 0.0, false),
        ("torus parallel exact", &tparallel, torus(), span, 0.0, 0.0, false),
        ("sphere parallel exact", &parallel, sphere(), span, 0.0, 0.0, false),
        ("sphere parallel v+τ", &parallel, sphere(), span, 0.0, TAU, false),
        ("sphere parallel u+τ", &parallel, sphere(), span, TAU, 0.0, false),
        ("sphere parallel u+π/2", &parallel, sphere(), span, FRAC_PI_2, 0.0, false),
        ("sphere parallel u+π/2+0.1zero", &parallel, sphere(), span, FRAC_PI_2 + 0.1 * zero, 0.0, false),
        ("sphere parallel u+π−1e-3", &parallel, sphere(), span, PI - 1e-3, 0.0, false),
        ("sphere parallel u+π (v kept)", &parallel, sphere(), span, PI, 0.0, false),
        ("sphere parallel u+3π (v kept)", &parallel, sphere(), span, 3.0 * PI, 0.0, false),
        ("sphere parallel twin", &parallel, sphere(), span, 0.0, 0.0, true),
        ("sphere parallel twin+τ", &parallel, sphere(), span, TAU, 0.0, true),
        ("sphere parallel twin−1e-3", &parallel, sphere(), span, -1e-3, 0.0, true),
        ("sphere parallel twin+0.5zero/r", &parallel, sphere(), span, 0.4 * zero, 0.0, true),
        ("sphere parallel twin v+τ", &parallel, sphere(), span, 0.0, TAU, true),
    ];
    for (name, c, s, sp, du, dv, tw) in cases {
        let (a, b) = both(c, &s, sp, du, dv, tw);
        out.push(format!("{name:34} f64: {a:28} Interval: {b}"));
    }
    println!("{}", out.join("\n"));
}

/// A Villarceau row (focal-section lane) and the u-only lifted copy.
#[test]
fn probe_focal_section_lifted_copy() {
    let band = band();
    let (big, r) = (2.0_f64, 0.7_f64);
    let axis = Vec3::unit_z();
    let u_ref = Vec3::unit_x();
    let center = Point3::origin();
    let surface = Surface::Torus {
        center,
        axis,
        major_radius: big,
        minor_radius: r,
        u_ref,
    };
    let tilt = (r / big).asin();
    let d = u_ref;
    let lean = d.cross(axis) * tilt.cos() + axis * tilt.sin();
    let carrier = Curve3::Circle {
        center: center + d * r,
        axis: d.cross(lean),
        radius: big,
        u_ref: d,
    };
    let (t0, t1) = (0.3, 1.3);
    let base = chart_pcurve_over(&carrier, t0, t1, &surface, band).unwrap();
    println!("kind: {}", match base {
        Pcurve::FocalSection(_) => "focal",
        _ => "other",
    });
    for k in [0, 1, -1, 3] {
        let row = base.shift_branch(k, Period::new(TAU));
        let got = PcurveCache::certify(row, t0, t1, &carrier, &surface, band);
        println!("focal u+{k}τ: {}", read(&got));
        let row = base.translate(0.0, f64::from(k) * TAU);
        let got = PcurveCache::certify(row, t0, t1, &carrier, &surface, band);
        println!("focal v+{k}τ: {}", read(&got));
    }
    let _ = Interval::from_f64(0.0).is_bounded();
}
