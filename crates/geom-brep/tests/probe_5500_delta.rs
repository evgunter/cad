//! Delta-verifier probes for PR 5500 (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::tol::band;
use geom::{Curve3, Surface};
use geom_brep::{PcurveCertifyError, PcurveCheck, chart_pcurve, chart_pcurve_over};
use geom_core::{Interval, Point3, Real, Vec3};

fn cone(alpha: f64) -> Surface<f64> {
    Surface::Cone { apex: Point3::origin(), axis: Vec3::unit_z(), half_angle: alpha, u_ref: Vec3::unit_x() }
}
fn cyl() -> Surface<f64> {
    Surface::Cylinder { origin: Point3::origin(), axis: Vec3::unit_z(), radius: 1.0, u_ref: Vec3::unit_x() }
}
fn mid() -> f64 { 0.5 * (band().zero() + band().escalate()) }

fn kind(r: &Result<geom_brep::Pcurve<impl Real>, PcurveCertifyError>) -> String {
    match r {
        Ok(_) => "OK".into(),
        Err(PcurveCertifyError::CarrierOffChart { .. }) => "off".into(),
        Err(PcurveCertifyError::Escalated { check, .. }) => format!("esc:{check:?}"),
        Err(PcurveCertifyError::NoGeneralLane { gap, .. }) => format!("nolane:{gap:?}"),
        Err(e) => format!("other:{e:?}"),
    }
}

fn both(c: &Curve3<f64>, s: &Surface<f64>, span: Option<(f64, f64)>) -> (String, String) {
    let ci = c.map_scalar(Interval::from_f64);
    let si = s.map_scalar(Interval::from_f64);
    match span {
        None => (kind(&chart_pcurve(c, s, band())), kind(&chart_pcurve(&ci, &si, band()))),
        Some((a, b)) => (
            kind(&chart_pcurve_over(c, a, b, s, band())),
            kind(&chart_pcurve_over(&ci, Interval::from_f64(a), Interval::from_f64(b), &si, band())),
        ),
    }
}

#[test]
fn p1c_probes() {
    let m = mid();
    let line = |o: Point3<f64>, d: Vec3<f64>| Curve3::Line { origin: o, dir: d.normalize() };
    let spiric = Curve3::Spiric { center: Point3::origin(), axis: Vec3::unit_y(), u_ref: Vec3::unit_x(),
        major_radius: 2.0, minor_radius: 0.75, offset: 0.5 };
    let crossed = Surface::Torus { center: Point3::origin(),
        axis: (Vec3::unit_x() + Vec3::unit_y() * (m / 2.0)).normalize(), major_radius: 2.0, minor_radius: 0.75,
        u_ref: Vec3::unit_z() };
    let d5 = Vec3::new(0.0, 1.0, 0.3).normalize();
    let near_axis_circle = |cx: f64| Curve3::Circle { center: Point3::new(cx, 0.0, 0.0),
        axis: Vec3::new(1.0, 0.0, m).normalize(), radius: 1.0, u_ref: Vec3::unit_y() };
    let cases: Vec<(&str, Curve3<f64>, Surface<f64>, Option<(f64, f64)>)> = vec![
        ("cone line (1,0,mid) +y", line(Point3::new(1.0, 0.0, m), Vec3::unit_y()), cone(0.5), None),
        ("cone line (1,0,0) (0,1,mid)", line(Point3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, m)), cone(0.5), None),
        ("cone line (1,0,mid) (0,1,1)", line(Point3::new(1.0, 0.0, m), Vec3::new(0.0, 1.0, 1.0)), cone(0.5), None),
        ("5m line origin midband", line(Point3::new(5.0, 0.0, m), d5), cone(0.5), None),
        ("5m line moved 1m", line(Point3::new(5.0, 0.0, m) + d5, d5), cone(0.5), None),
        ("5m line moved 1m, edge span", line(Point3::new(5.0, 0.0, m) + d5, d5), cone(0.5), Some((-1.0, 2.0))),
        ("near-apex-plane line hs mid", line(Point3::new(0.3, 0.2, 0.0), Vec3::new(1.0, 0.0, m)), cone(0.5), None),
        ("apex line hs mid (through apex)", line(Point3::origin(), Vec3::new(1.0, 0.0, m)), cone(0.5), None),
        ("cyl circle plane ~contains axis, centred", near_axis_circle(0.0), cyl(), None),
        ("cyl circle plane ~contains axis, 0.5 out", near_axis_circle(0.5), cyl(), None),
        ("spiric crossed torus", spiric, crossed, None),
    ];
    let mut wrong = Vec::new();
    for (what, c, s, span) in &cases {
        let (f, i) = both(c, s, *span);
        eprintln!("PROBE eps={} {what}: f64={f} iv={i}", geom_core::Tol::witness().eps());
        for g in [&f, &i] {
            if !(g == "off" || g == "esc:ChartIncidence") {
                wrong.push(format!("{what}: {g}"));
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// P1d: on-chart rulings and circles still image.
#[test]
fn p1d_on_chart_still_images() {
    let a: f64 = 0.5;
    let g = Vec3::new(a.sin(), 0.0, a.cos());
    let ruling = Curve3::Line { origin: Point3::origin() + g * 2.0, dir: g };
    let ruling_mirror = Curve3::Line { origin: Point3::origin() + g * -2.0, dir: g };
    let rim = Curve3::Circle { center: Point3::new(0.0, 0.0, 2.0), axis: Vec3::unit_z(), radius: 2.0 * a.tan(), u_ref: Vec3::unit_x() };
    let sph = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
    let n = Vec3::new(0.3, 0.1, 1.0).normalize();
    let gen_circle = Curve3::Circle { center: Point3::origin() + n * 0.5, axis: n, radius: 0.75f64.sqrt(), u_ref: n.cross(Vec3::unit_x()).normalize() };
    let ruling_in_cyl = Curve3::Line { origin: Point3::new(1.0, 0.0, 3.0), dir: Vec3::unit_z() };
    for (what, c, s) in [
        ("ruling", ruling, cone(a)),
        ("ruling mirror nappe", ruling_mirror, cone(a)),
        ("rim", rim, cone(a)),
        ("general sphere circle", gen_circle, sph),
        ("cyl ruling", ruling_in_cyl, cyl()),
    ] {
        let (f, i) = both(&c, &s, None);
        eprintln!("PROBE-ON {what}: f64={f} iv={i}");
        assert_eq!((f.as_str(), i.as_str()), ("OK", "OK"), "{what}");
    }
}

/// P5: the apex-plane line's margin arm, reachable after incidence-first?
#[test]
fn p5_apex_plane_line() {
    let l = Curve3::Line { origin: Point3::origin(), dir: Vec3::unit_x() };
    for (a, b) in [(0.0, 1.0), (0.0, 1e-3), (0.0, 1e-10), (-1e-10, 1e-10)] {
        let r = chart_pcurve_over(&l, a, b, &cone(0.5), band());
        eprintln!("PROBE-APEX span ({a},{b}): {}", match &r { Ok(_) => "OK".into(), Err(e) => format!("{e}") });
    }
}
