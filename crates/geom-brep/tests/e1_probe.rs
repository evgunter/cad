//! Review probes for PCERT E1 (reviewer branch only).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use geom::{Curve3, Surface};
use geom_brep::recourse::Reading;
use geom_brep::{Pcurve, PcurveCache, PcurveCertifyError, chart_pcurve};
use geom_core::{Band, Decide, Interval, Point3, Real, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}
fn eps() -> f64 {
    Tol::witness().get().eps
}
fn mid() -> f64 {
    0.5 * (band().zero() + band().escalate())
}
fn cone(alpha: f64) -> Surface<f64> {
    Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: alpha,
        u_ref: Vec3::unit_x(),
    }
}
fn report<T: Decide>(name: &str, got: &Result<Pcurve<T>, PcurveCertifyError>) {
    match got {
        Ok(p) => eprintln!("PROBE {name}: Ok({})", format!("{p:?}").split('(').next().unwrap().split(' ').next().unwrap()),
        Err(e) => eprintln!(
            "PROBE {name}: Err({e:?})\n   check={:?}\n   build-ending={:?}\n   rest-ending={:?}",
            e.refused_check(),
            e.ending(Reading::Build),
            e.ending(Reading::AtRest)
        ),
    }
}
fn both(name: &str, carrier: &Curve3<f64>, surface: &Surface<f64>) {
    let got = chart_pcurve(carrier, surface, band());
    report(&format!("{name} [f64]"), &got);
    if let Ok(p) = got {
        let c = PcurveCache::certify(p, 0.0, core::f64::consts::TAU, carrier, surface, band());
        eprintln!("PROBE {name} [f64] certify: {:?}", c.as_ref().map(|_| ()));
    }
    let at = <Interval as Real>::from_f64;
    let ci = carrier.map_scalar(at);
    let si = surface.map_scalar(at);
    let got = chart_pcurve(&ci, &si, band());
    report(&format!("{name} [Interval]"), &got);
    if let Ok(p) = got {
        let c = PcurveCache::certify(p, at(0.0), at(core::f64::consts::TAU), &ci, &si, band());
        eprintln!("PROBE {name} [Interval] certify: {:?}", c.as_ref().map(|_| ()));
    }
}

#[test]
fn probe_cone_lines_near_the_apex_plane() {
    eprintln!("eps={:e} band zero={:e} esc={:e}", eps(), band().zero(), band().escalate());
    // Off the cone by ~cos(alpha): a line in a plane mid-band above the apex plane.
    both(
        "line apex-plane +mid (off cone)",
        &Curve3::Line { origin: Point3::new(1.0, 0.0, mid()), dir: Vec3::unit_y() },
        &cone(0.5),
    );
    both(
        "line apex-plane exact (off cone)",
        &Curve3::Line { origin: Point3::new(1.0, 0.0, 0.0), dir: Vec3::unit_y() },
        &cone(0.5),
    );
    // hs in band, h0 exact Zero.
    both(
        "line apex-plane slope mid (off cone)",
        &Curve3::Line {
            origin: Point3::new(1.0, 0.0, 0.0),
            dir: Vec3::new(0.0, 1.0, mid()).normalize(),
        },
        &cone(0.5),
    );
    // Off the cone by a metre, origin height mid band, slope definite: not through the apex.
    both(
        "line skew definite slope, origin mid band (off cone)",
        &Curve3::Line {
            origin: Point3::new(1.0, 0.0, mid()),
            dir: Vec3::new(0.0, 1.0, 1.0).normalize(),
        },
        &cone(0.5),
    );
    // A true ruling whose origin sits mid band off the apex.
    let g = Vec3::new(0.5f64.sin(), 0.0, 0.5f64.cos());
    both(
        "ruling origin near apex (on cone)",
        &Curve3::Line { origin: Point3::origin() + g * (mid() / 0.5f64.cos()), dir: g },
        &cone(0.5),
    );
}

#[test]
fn probe_cone_rims_and_circles() {
    let surface = cone(0.5);
    for (name, h) in [("rim h=0 exact", 0.0), ("rim h=0.2eps", 0.2 * eps()), ("rim h=mid", mid())] {
        both(
            name,
            &Curve3::Circle { center: Point3::new(0.0, 0.0, h), axis: Vec3::unit_z(), radius: 0.3, u_ref: Vec3::unit_x() },
            &surface,
        );
    }
    // A circle on the cone: the rim at h=1 tilted so its axial reading is mid band (on-ish: second order),
    // and the same moved off the cone along the axis by 4Kε.
    let (h, a) = (1.0f64, 0.5f64);
    let rho = h * a.tan();
    let th = (mid() / rho).asin();
    let n = Vec3::new(0.0, -th.sin(), th.cos());
    let k = Tol::witness().k();
    for (name, dz) in [("tilted rim mid band", 0.0), ("tilted rim moved off 4Kε", 4.0 * k * eps()), ("tilted rim moved off 1e3Kε", 1e3 * k * eps())] {
        both(
            name,
            &Curve3::Circle { center: Point3::new(0.0, 0.0, h + dz), axis: n, radius: rho, u_ref: Vec3::unit_x() },
            &surface,
        );
    }
    // A rim moved off its radius: a closed-form class with a residual miss.
    both(
        "rim radius off 4Kε",
        &Curve3::Circle { center: Point3::new(0.0, 0.0, h), axis: Vec3::unit_z(), radius: rho + 4.0 * k * eps(), u_ref: Vec3::unit_x() },
        &surface,
    );
}

fn sphere() -> Surface<f64> {
    Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() }
}

#[test]
fn probe_sphere_projected_lane() {
    let (h, rho) = (0.6, 0.8);
    for frac in [0.0, 0.5, 1.0, 1.5, 3.0] {
        // near-parallel: tilt about x so axial reading is frac band-widths (scaled between zero and esc).
        let m = band().zero() + frac * (band().escalate() - band().zero());
        let th = (m / rho).asin();
        let n = Vec3::new(0.0, -th.sin(), th.cos());
        both(
            &format!("sphere near-parallel frac={frac}"),
            &Curve3::Circle { center: Point3::origin() + n * h, axis: n, radius: rho, u_ref: Vec3::unit_x() },
            &sphere(),
        );
        // near-meridian: a great-ish circle in a plane holding the axis, tilted so a×b·axis is m.
        let tilt = (m / 1.0f64).asin();
        let nm = Vec3::new(tilt.cos(), 0.0, tilt.sin()); // plane normal nearly ⊥ axis
        let c0 = nm * 0.3;
        let r = (1.0f64 - 0.09).sqrt();
        both(
            &format!("sphere near-meridian (offset plane) frac={frac}"),
            &Curve3::Circle { center: Point3::origin() + c0, axis: nm, radius: r, u_ref: Vec3::unit_z().cross(nm).normalize() },
            &sphere(),
        );
        both(
            &format!("sphere near-meridian (great) frac={frac}"),
            &Curve3::Circle { center: Point3::origin(), axis: nm, radius: 1.0, u_ref: Vec3::unit_z().cross(nm).normalize() },
            &sphere(),
        );
        // off-centre parallel: centre moved radially (off the sphere to first order δρ/R).
        both(
            &format!("sphere parallel centre off axis frac={frac}"),
            &Curve3::Circle { center: Point3::new(m, 0.0, h), axis: Vec3::unit_z(), radius: rho, u_ref: Vec3::unit_x() },
            &sphere(),
        );
    }
}

#[test]
fn probe_torus_circles() {
    let (major, minor) = (2.0f64, 0.7f64);
    let torus = Surface::Torus { center: Point3::origin(), axis: Vec3::unit_z(), major_radius: major, minor_radius: minor, u_ref: Vec3::unit_x() };
    let k = Tol::witness().k();
    let delta = 2.0 * k * eps();
    for (name, dz) in [("crest off-axis δ, on", 0.0), ("crest off-axis δ, moved up 4Kε", 4.0 * k * eps()), ("crest off-axis δ, moved up 1e3Kε", 1e3 * k * eps())] {
        both(
            name,
            &Curve3::Circle { center: Point3::new(delta, 0.0, minor + dz), axis: Vec3::unit_z(), radius: major, u_ref: Vec3::unit_x() },
            &torus,
        );
    }
    // Villarceau exact.
    let tilt = (minor / major).asin();
    let d = Vec3::unit_x();
    let lean = d.cross(Vec3::unit_z()) * tilt.cos() + Vec3::unit_z() * tilt.sin();
    let v = Curve3::Circle { center: Point3::origin() + d * minor, axis: d.cross(lean), radius: major, u_ref: d };
    both("villarceau exact", &v, &torus);
    // Villarceau tilted a mid-band off (about d): on? off by ~first order
    let lean2 = d.cross(Vec3::unit_z()) * (tilt + mid() / major).cos() + Vec3::unit_z() * (tilt + mid() / major).sin();
    both("villarceau tilt +mid/R", &Curve3::Circle { center: Point3::origin() + d * minor, axis: d.cross(lean2), radius: major, u_ref: d }, &torus);
    // Parallel tilted mid band (axial reading in band), on the crest.
    let th = (mid() / major).asin();
    let n = Vec3::new(0.0, -th.sin(), th.cos());
    both("crest tilted mid band", &Curve3::Circle { center: Point3::new(0.0, 0.0, minor), axis: n, radius: major, u_ref: Vec3::unit_x() }, &torus);
    both("crest tilted mid band, moved up 1e3Kε", &Curve3::Circle { center: Point3::new(0.0, 0.0, minor + 1e3 * k * eps()), axis: n, radius: major, u_ref: Vec3::unit_x() }, &torus);
}

#[test]
fn probe_spiric_chart_axis_near_perpendicular() {
    let (r, rr, d) = (2.0f64, 0.7f64, 0.3f64);
    let carrier = Curve3::Spiric { center: Point3::origin(), axis: Vec3::unit_y(), u_ref: Vec3::unit_x(), major_radius: r, minor_radius: rr, offset: d };
    // A chart torus whose axis is within the band of ⊥ the carrier's axis: the oval cannot lie on it.
    let ax = Vec3::new(1.0, mid() / r, 0.0).normalize();
    let chart = Surface::Torus { center: Point3::origin(), axis: ax, major_radius: r, minor_radius: rr, u_ref: Vec3::unit_z() };
    let got = chart_pcurve(&carrier, &chart, band());
    report("spiric on a torus ⊥-in-band [f64]", &got);
    let chart0 = Surface::Torus { center: Point3::origin(), axis: Vec3::unit_x(), major_radius: r, minor_radius: rr, u_ref: Vec3::unit_z() };
    report("spiric on a torus exactly ⊥ [f64]", &chart_pcurve(&carrier, &chart0, band()));
}

#[test]
fn probe_check4_definite_other() {
    let (h, rho) = (0.6, 0.8);
    let parallel = Curve3::Circle { center: Point3::new(0.0, 0.0, h), axis: Vec3::unit_z(), radius: rho, u_ref: Vec3::unit_x() };
    for frac in [0.5, 1.5, 3.0, 100.0] {
        let m = band().zero() + frac * (band().escalate() - band().zero());
        let th = (m / rho).asin();
        let n = Vec3::new(0.0, -th.sin(), th.cos());
        let tilted = Curve3::Circle { center: Point3::origin() + n * h, axis: n, radius: rho, u_ref: Vec3::unit_x() };
        let img = chart_pcurve(&parallel, &sphere(), band()).unwrap();
        let got = PcurveCache::certify(img, 0.0, core::f64::consts::TAU, &tilted, &sphere(), band());
        eprintln!("PROBE check4 parallel image vs tilted frac={frac} [f64]: {:?}", got.as_ref().map(|_| ()).map_err(|e| format!("{e:?}").chars().take(160).collect::<String>()));
        let at = <Interval as Real>::from_f64;
        let img = chart_pcurve(&parallel.map_scalar(at), &sphere().map_scalar(at), band()).unwrap();
        let got = PcurveCache::certify(img, at(0.0), at(core::f64::consts::TAU), &tilted.map_scalar(at), &sphere().map_scalar(at), band());
        eprintln!("PROBE check4 parallel image vs tilted frac={frac} [Interval]: {:?}", got.as_ref().map(|_| ()).map_err(|e| format!("{e:?}").chars().take(160).collect::<String>()));
    }
}

#[test]
fn probe_edge_certificate() {
    use geom_brep::{EdgeCurve, EdgeCurveSpec, EdgeDescriptionSpec};
    let m = 0.5 * (band().zero() + band().escalate());
    let tilt = m.asin();
    let nm = Vec3::new(tilt.cos(), 0.0, tilt.sin());
    let cases: Vec<(&str, Curve3<f64>, Surface<f64>, f64, f64)> = vec![
        ("off-cone line, origin mid band", Curve3::Line { origin: Point3::new(1.0, 0.0, mid()), dir: Vec3::unit_y() }, cone(0.5), 0.0, 1.0),
        ("near-meridian great circle in band", Curve3::Circle { center: Point3::origin(), axis: nm, radius: 1.0, u_ref: Vec3::unit_z().cross(nm).normalize() }, sphere(), 0.0, 1.0),
    ];
    for (name, carrier, surface, t0, t1) in cases {
        let (keys, lookup) = crate::shared::surf::table(vec![surface]);
        let got = EdgeCurve::certify(
            EdgeCurveSpec { description: EdgeDescriptionSpec::chart(keys[0]), carrier: carrier.clone(), param_start: t0, param_end: t1 },
            carrier.eval(t0), carrier.eval(t1), &lookup, 1.0, band(),
        );
        match got {
            Ok(_) => eprintln!("PROBE edge {name}: Ok"),
            Err(e) => eprintln!("PROBE edge {name}: {:?}\n   decision={:?}\n   build={:?}\n   rest={:?}", e, e.decision().map(|(c, _)| c), e.ending(Reading::Build), e.ending(Reading::AtRest)),
        }
    }
}
