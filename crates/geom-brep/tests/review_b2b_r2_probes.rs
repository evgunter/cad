//! Review probes for PCERT B2b (PR 5343): a row stored a whole period
//! off its placed image, and the sphere twin on a parallel, at `f64`
//! and at `Interval`. The rows print what they read; the asserts pin
//! the claims the PR makes.

#![allow(clippy::unwrap_used, clippy::panic)]

use crate::shared::tol::band;
use core::f64::consts::{FRAC_PI_2, PI, TAU};
use geom::{Curve3, Surface};
use geom_brep::{
    ChartMap, EnvelopeTerm, Pcurve, PcurveCache, PcurveCertifyError, PcurveCheck, Period,
    chart_pcurve_over,
};
use geom_core::{Decide, Interval, Point3, Real, Vec3};

fn read<T: Decide>(r: &Result<PcurveCache<T>, PcurveCertifyError>) -> String {
    match r {
        Ok(_) => "certifies".to_string(),
        Err(PcurveCertifyError::ResidualExceeded { check, .. }) => format!("refused at {check:?}"),
        Err(e) => format!("other: {e:?}"),
    }
}

fn lifted<T: Decide>(
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
    (t0, t1): (f64, f64),
    shift: (f64, f64),
    twin: bool,
) -> String {
    let f = T::from_f64;
    let (c, s) = (carrier.map_scalar(f), surface.map_scalar(f));
    let mut image = chart_pcurve_over(&c, f(t0), f(t1), &s, band()).unwrap();
    if twin {
        image = image.map_affine(ChartMap::sphere_involution()).unwrap();
    }
    let image = image.translate(f(shift.0), f(shift.1));
    read(&PcurveCache::certify(image, f(t0), f(t1), &c, &s, band()))
}

fn both(
    what: &str,
    carrier: &Curve3<f64>,
    surface: &Surface<f64>,
    span: (f64, f64),
    shift: (f64, f64),
    twin: bool,
) -> (String, String) {
    let a = lifted::<f64>(carrier, surface, span, shift, twin);
    let b = lifted::<Interval>(carrier, surface, span, shift, twin);
    println!("PROBE {what}: f64 {a} | Interval {b}");
    (a, b)
}

fn cylinder() -> Surface<f64> {
    Surface::Cylinder {
        origin: Point3::new(0.1, -0.2, 0.3),
        axis: Vec3::unit_z(),
        radius: 0.5,
        u_ref: Vec3::unit_x(),
    }
}

fn sphere() -> Surface<f64> {
    Surface::Sphere {
        center: Point3::new(0.3, -0.2, 1.1),
        radius: 0.8,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

fn torus() -> Surface<f64> {
    Surface::Torus {
        center: Point3::new(0.3, -0.2, 0.5),
        axis: Vec3::unit_z(),
        major_radius: 2.0,
        minor_radius: 0.7,
        u_ref: Vec3::unit_x(),
    }
}

/// A parallel of the sphere at latitude `lat`.
fn sphere_parallel(lat: f64) -> Curve3<f64> {
    Curve3::Circle {
        center: Point3::new(0.3, -0.2, 1.1 + 0.8 * lat.sin()),
        axis: Vec3::unit_z(),
        radius: 0.8 * lat.cos(),
        u_ref: Vec3::unit_x(),
    }
}

const FU: &str = "refused at EnvelopeTerm(FidelityU)";
const FV: &str = "refused at EnvelopeTerm(FidelityV)";

#[test]
fn p1_whole_period_shifts_refuse_as_their_term() {
    let rim = Curve3::Circle {
        center: Point3::new(0.1, -0.2, 0.7),
        axis: Vec3::unit_z(),
        radius: 0.5,
        u_ref: Vec3::new(0.6, 0.8, 0.0),
    };
    let seam = Curve3::Line {
        origin: Point3::new(0.6, -0.2, 0.0),
        dir: Vec3::unit_z(),
    };
    let mut bad = vec![];
    for k in [1i32, -1, 3, -3, 1 << 20, 1 << 30, i32::MAX, i32::MIN] {
        let d = Period::new(TAU).times(k);
        for (what, c, span) in [("rim", &rim, (0.3, 2.0)), ("seam", &seam, (0.0, 1.0))] {
            let (a, b) = both(&format!("cyl {what} u+{k}τ"), c, &cylinder(), span, (d, 0.0), false);
            if a != FU || b != FU {
                bad.push(format!("cyl {what} k={k}: {a} | {b}"));
            }
        }
        let (a, b) = both(&format!("sphere parallel u+{k}τ"), &sphere_parallel(0.4), &sphere(), (0.3, 2.0), (d, 0.0), false);
        if a != FU || b != FU {
            bad.push(format!("sphere parallel k={k}: {a} | {b}"));
        }
    }
    // v + τ on the angular second channels.
    let tparallel = Curve3::Circle {
        center: Point3::new(0.3, -0.2, 0.5 + 0.7 * 0.6_f64.sin()),
        axis: Vec3::unit_z(),
        radius: 2.0 + 0.7 * 0.6_f64.cos(),
        u_ref: Vec3::unit_x(),
    };
    for (what, c, s) in [
        ("torus parallel v+τ", &tparallel, torus()),
        ("sphere parallel v+τ", &sphere_parallel(0.4), sphere()),
    ] {
        let (a, b) = both(what, c, &s, (0.3, 2.0), (0.0, TAU), false);
        if a != FV || b != FV {
            bad.push(format!("{what}: {a} | {b}"));
        }
    }
    // Near-band: a whole period plus a hair, and a hair alone.
    let (a, b) = both("cyl rim u+τ+1e-13", &rim, &cylinder(), (0.3, 2.0), (TAU + 1e-13, 0.0), false);
    if a != FU || b != FU {
        bad.push(format!("near band: {a} | {b}"));
    }
    let (a, b) = both("cyl rim u+1e-15", &rim, &cylinder(), (0.3, 2.0), (1e-15, 0.0), false);
    if a != "certifies" || b != "certifies" {
        bad.push(format!("in band: {a} | {b}"));
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

#[test]
fn p3_the_parallel_twin_is_one_sign() {
    let c = sphere_parallel(0.4);
    let s = sphere();
    let span = (0.3, 2.0);
    // (what, shift in u, as twin, expected)
    let eps = band().zero();
    let rows: Vec<(&str, f64, bool, &str)> = vec![
        ("P_d", 0.0, false, "certifies"),
        ("twin", 0.0, true, "certifies"),
        ("P_d + in-band", 0.1 * eps, false, "certifies"),
        ("twin + in-band", 0.1 * eps, true, "certifies"),
        ("P_d + π/2", FRAC_PI_2, false, FU),
        ("twin + π/2", FRAC_PI_2, true, FU),
        ("P_d + π (not the twin: v unreflected)", PI, false, "?"),
        ("P_d + π − 1e-3", PI - 1e-3, false, "?"),
        ("twin − 1e-3", -1e-3, true, FU),
        ("twin + τ", TAU, true, FU),
        ("twin − τ (u − π)", -TAU, true, FU),
    ];
    let mut bad = vec![];
    for (what, d, twin, want) in rows {
        let (a, b) = both(what, &c, &s, span, (d, 0.0), twin);
        if want != "?" && (a != want || b != want) {
            bad.push(format!("{what}: want {want}, got {a} | {b}"));
        }
    }
    // On the equator the twin is `u + π` with v unmoved.
    let eq = sphere_parallel(0.0);
    let (a, b) = both("equator P_d + π", &eq, &s, span, (PI, 0.0), false);
    if a != "certifies" || b != "certifies" {
        bad.push(format!("equator u+π: {a} | {b}"));
    }
    assert!(bad.is_empty(), "{bad:#?}");
}

/// The other closed-form lanes: is a lifted copy refused there too?
#[test]
fn p1b_other_lanes_lifted_copy() {
    // A Villarceau circle of `torus()`.
    let (big, r) = (2.0_f64, 0.7_f64);
    let tilt = (r / big).asin();
    let d = Vec3::unit_x();
    let axis = Vec3::unit_z();
    let lean = d.cross(axis) * tilt.cos() + axis * tilt.sin();
    let vill = Curve3::Circle {
        center: Point3::new(0.3, -0.2, 0.5) + d * r,
        axis: d.cross(lean),
        radius: big,
        u_ref: d,
    };
    for (what, shift) in [("vill u+τ", (TAU, 0.0)), ("vill v+τ", (0.0, TAU)), ("vill u+τ v−τ", (TAU, -TAU))] {
        both(what, &vill, &torus(), (0.3, 2.0), shift, false);
    }
    // A sphere's general circle: the projected lane.
    let n = Vec3::new(0.3, 0.1, 1.0).normalize();
    let u = (Vec3::unit_x() - n * n.x).normalize();
    let general = Curve3::Circle {
        center: Point3::new(0.3, -0.2, 1.1) + n * 0.3,
        axis: n,
        radius: (0.8_f64 * 0.8 - 0.09).sqrt(),
        u_ref: u,
    };
    both("sphere general u+τ", &general, &sphere(), (0.3, 2.0), (TAU, 0.0), false);
    both("sphere general v+τ", &general, &sphere(), (0.3, 2.0), (0.0, TAU), false);
    // A cone rim.
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: 0.5,
        u_ref: Vec3::unit_x(),
    };
    let crim = Curve3::Circle {
        center: Point3::new(0.0, 0.0, 1.0),
        axis: Vec3::unit_z(),
        radius: 0.5_f64.tan(),
        u_ref: Vec3::unit_x(),
    };
    both("cone rim u+τ", &crim, &cone, (0.3, 2.0), (TAU, 0.0), false);
}
