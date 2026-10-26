//! Review probes for PCERT E1b (PR 5797): declared curves off plane,
//! cylinder and sphere faces through `set_edge_curve`, and a declared
//! `OnPair` description off the sphere.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::recourse::Reading;
use geom_brep::{
    CertifyError, EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve, MappedSource, SurfaceKey,
    SurfacePair,
};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::{Body, CurveGeom, EdgeKey, EulerOpError};

fn dummy() -> MappedCurve<f64> {
    MappedCurve::whole(MappedSource::ExtrudedPoint {
        point: Point2::new(0.0, 0.0),
        place: Affine3::identity(),
        vec: Vec3::new(0.0, 0.0, 1.0),
    })
}

fn ends(body: &Body<f64>, edge: EdgeKey) -> (Point3<f64>, Point3<f64>) {
    let c = body
        .get_edge(edge)
        .and_then(|e| body.get_curve_geom(e.curve))
        .and_then(CurveGeom::certified)
        .unwrap();
    let (t0, t1) = c.params();
    (c.carrier().eval(t0), c.carrier().eval(t1))
}

fn edge_between(body: &Body<f64>, a: Point3<f64>, b: Point3<f64>) -> EdgeKey {
    body.edges()
        .find(|(k, _)| {
            body.get_edge(*k)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(CurveGeom::certified)
                .is_some()
                && {
                    let (p, q) = ends(body, *k);
                    (p.distance(a) < 1e-9 && q.distance(b) < 1e-9)
                        || (p.distance(b) < 1e-9 && q.distance(a) < 1e-9)
                }
        })
        .map(|(k, _)| k)
        .expect("edge")
}

/// The semicircle from `a` to `b` bulging toward unit `n` (⊥ ab).
fn bulge(a: Point3<f64>, b: Point3<f64>, n: Vec3<f64>) -> (Curve3<f64>, f64, f64) {
    let m = Point3::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0, (a.z + b.z) / 2.0);
    let r = a.distance(b) / 2.0;
    let u = (a - m) / r;
    let axis = u.cross(n);
    (
        Curve3::Circle {
            center: m,
            axis,
            radius: r,
            u_ref: u,
        },
        0.0,
        core::f64::consts::PI,
    )
}

fn surface_of(body: &Body<f64>, pick: impl Fn(&Surface<f64>) -> bool) -> SurfaceKey {
    body.faces()
        .map(|(_, f)| f.surface)
        .find(|&k| body.get_surface(k).is_some_and(&pick))
        .expect("surface")
}

fn attempt(
    mut body: Body<f64>,
    edge: EdgeKey,
    description: EdgeDescriptionSpec<f64>,
    carrier: (Curve3<f64>, f64, f64),
    tag: &str,
) -> Option<CertifyError> {
    let spec = EdgeCurveSpec {
        description,
        carrier: carrier.0,
        param_start: carrier.1,
        param_end: carrier.2,
    };
    let r = body.set_edge_curve(edge, spec, Tol::witness());
    match r {
        Err(EulerOpError::Certification { error }) => {
            eprintln!(
                "PROBE {tag}: {error:?}\n  decision {:?}\n  display: {error}\n  build: {:?}\n  rest: {:?}",
                error.decision().map(|(c, _)| c),
                error.ending(Reading::Build),
                error.ending(Reading::AtRest)
            );
            Some(error)
        }
        other => {
            eprintln!("PROBE {tag}: other {other:?}");
            None
        }
    }
}

fn quarter() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    )
}

#[test]
fn e1b_probe_plane() {
    let body = quarter();
    for (k, e) in body.edges() {
        let _ = e;
        if body
            .get_edge(k)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(CurveGeom::certified)
            .is_some()
        {
            eprintln!(
                "edge {:?} {:?}",
                ends(&body, k),
                body.get_edge(k).map(|e| e.curve)
            );
        }
    }
    let a = Point3::new(0.0, 0.0, 0.0);
    let b = Point3::new(0.0, 1.0, 0.0);
    let edge = edge_between(&body, a, b);
    // The plane z = 0 (the profile's own wall).
    let plane = surface_of(&body, |s| match s {
        Surface::Plane { normal, .. } => normal.z.abs() > 0.9,
        _ => false,
    });
    let off = bulge(a, b, Vec3::new(0.0, 0.0, -1.0));
    let d = attempt(
        body.clone(),
        edge,
        EdgeDescriptionSpec::chart(plane).declared_by(dummy()),
        off.clone(),
        "plane declared",
    );
    let u = attempt(
        body,
        edge,
        EdgeDescriptionSpec::chart(plane),
        off,
        "plane derived",
    );
    assert!(d.is_some() && u.is_some());
}

#[test]
fn e1b_probe_cylinder() {
    let body = quarter();
    let edge = edge_between(
        &body,
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
    );
    let (a, b) = ends(&body, edge);
    let cyl = surface_of(&body, |s| matches!(s, Surface::Cylinder { .. }));
    let off = bulge(a, b, Vec3::new(1.0, 0.0, 0.0));
    let d = attempt(
        body.clone(),
        edge,
        EdgeDescriptionSpec::chart(cyl).declared_by(dummy()),
        off.clone(),
        "cylinder declared",
    );
    let u = attempt(
        body,
        edge,
        EdgeDescriptionSpec::chart(cyl),
        off,
        "cylinder derived",
    );
    assert!(d.is_some() && u.is_some());
}

fn hemisphere() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), (core::f64::consts::PI / 8.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        Tol::witness(),
    )
}

#[test]
fn e1b_probe_sphere() {
    let body = hemisphere();
    for (k, _) in body.edges() {
        if body
            .get_edge(k)
            .and_then(|e| body.get_curve_geom(e.curve))
            .and_then(CurveGeom::certified)
            .is_some()
        {
            eprintln!("edge {:?}", ends(&body, k));
        }
    }
    let sphere = surface_of(&body, |s| matches!(s, Surface::Sphere { .. }));
    let other = surface_of(&body, |s| !matches!(s, Surface::Sphere { .. }));
    let seam = body
        .edges()
        .find(|(k, _)| {
            body.get_edge(*k)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(CurveGeom::certified)
                .is_some_and(|c| match c.carrier() {
                    Curve3::Circle { axis, .. } => axis.y.abs() < 0.5,
                    _ => false,
                })
        })
        .map(|(k, _)| k)
        .unwrap();
    let (a, b) = ends(&body, seam);
    let m = Vec3::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0, (a.z + b.z) / 2.0);
    let n = m / m.norm();
    // A semicircle on the chord bulging outward: off the sphere.
    let off = bulge(a, b, n);
    let chord = {
        let len = a.distance(b);
        (
            Curve3::Line {
                origin: a,
                dir: (b - a) / len,
            },
            0.0,
            len,
        )
    };
    attempt(
        body.clone(),
        seam,
        EdgeDescriptionSpec::chart(sphere).declared_by(dummy()),
        off.clone(),
        "sphere declared circle",
    );
    attempt(
        body.clone(),
        seam,
        EdgeDescriptionSpec::chart(sphere),
        off,
        "sphere derived circle",
    );
    let pair = SurfacePair::new(sphere, other);
    attempt(
        body.clone(),
        seam,
        EdgeDescriptionSpec::on_pair(pair).declared_by(dummy()),
        chord.clone(),
        "sphere onpair declared chord",
    );
    attempt(
        body.clone(),
        seam,
        EdgeDescriptionSpec::on_pair(pair),
        chord.clone(),
        "sphere onpair derived chord",
    );
    attempt(
        body,
        seam,
        EdgeDescriptionSpec::chart(sphere).declared_by(dummy()),
        chord,
        "sphere declared chord",
    );
}

#[test]
fn e1b_probe_cone() {
    let body = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Partial(core::f64::consts::FRAC_PI_2),
        Tol::witness(),
    );
    let edge = edge_between(
        &body,
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    );
    let (a, b) = ends(&body, edge);
    let cone = surface_of(&body, |s| matches!(s, Surface::Cone { .. }));
    let n = Vec3::new(1.0, 1.0, 0.0) / 2f64.sqrt();
    let off = bulge(a, b, n);
    attempt(
        body.clone(),
        edge,
        EdgeDescriptionSpec::chart(cone).declared_by(dummy()),
        off.clone(),
        "cone declared circle",
    );
    attempt(
        body,
        edge,
        EdgeDescriptionSpec::chart(cone),
        off,
        "cone derived circle",
    );
}

#[test]
fn e1b_probe_display_at_build() {
    use geom_brep::{EdgeAuthority, OffMargin, PcurveCertifyError as C, PcurveCheck};
    let declared = EdgeAuthority::Declared(dummy()).kind();
    let derived = EdgeAuthority::<f64>::Derived.kind();
    let band = geom_core::Band::new(1e-9, 1e-8).unwrap();
    let errors = [
        C::CarrierOffChart {
            chart: geom::SurfaceKind::Sphere,
            carrier: geom::CurveKind::Line,
            why: "a sphere holds no line",
            margin: None,
        },
        C::CarrierOffChart {
            chart: geom::SurfaceKind::Cone,
            carrier: geom::CurveKind::Line,
            why: "at the apex pose",
            margin: Some(OffMargin::InBand(geom_core::MarginDiag::value(4e-10))),
        },
        C::Escalated {
            check: PcurveCheck::ChartIncidence,
            sample: 0,
            cause: geom_core::Indeterminate {
                margin: geom_core::MarginDiag::value(5e-9),
                band,
                predicate: Some("pcurve_sphere_chart_incident"),
                terminal_sliver: false,
            },
        },
    ];
    for error in errors {
        for (name, authority) in [("declared", declared), ("derived", derived)] {
            let m = topo::PcurveMintError::Certify {
                half_edge: topo::HalfEdgeKey::default(),
                error: error.clone(),
                authority,
            };
            eprintln!("PROBE build {name}: {m}");
            let v = topo::ValidationError::Pcurve { finding: m };
            eprintln!("PROBE rest  {name}: {v}");
        }
    }
}
