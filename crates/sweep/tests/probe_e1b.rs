//! Review probes for E1b (not for merge).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use geom::{Curve3, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, MappedCurve, MappedSource};
use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::test_support::bulge_loop;
use profile::{Profile, SketchPlane};
use topo::{Body, CurveGeom, EdgeKey, FaceSurface};

fn dummy() -> MappedCurve<f64> {
    MappedCurve::whole(MappedSource::ExtrudedPoint {
        point: Point2::new(0.0, 0.0),
        place: Affine3::identity(),
        vec: Vec3::new(0.0, 0.0, 1.0),
    })
}

fn extruded(chain: Vec<(Point2<f64>, f64)>) -> Body<f64> {
    let tol = Tol::witness();
    let p = Profile::new(SketchPlane::<f64>::xy(), vec![bulge_loop(chain)])
        .validate(tol)
        .unwrap();
    sweep::extrude(
        &p,
        sweep::Extrusion::Distance { depth: 1.0, side: sweep::ExtrudeSide::Along },
        tol,
    )
    .unwrap()
    .body
}

fn hemisphere() -> Body<f64> {
    let tol = Tol::witness();
    let quarter = bulge_loop(vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), (core::f64::consts::PI / 8.0).tan()),
        (Point2::new(0.0, 1.0), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![quarter]).validate(tol).unwrap();
    let axis = sweep::RevolveAxis { origin: Point2::new(0.0, 0.0), dir: Vec2::new(0.0, 1.0) };
    sweep::revolve(&profile, axis, sweep::Revolution::Full, tol).unwrap().body
}

fn census(body: &Body<f64>, label: &str) {
    for (k, e) in body.edges() {
        let c = body.get_curve_geom(e.curve).and_then(CurveGeom::certified);
        if let Some(c) = c {
            eprintln!("{label}: edge {k:?} {:?} declared={}", c.carrier().kind(), c.authority().is_declared());
        }
    }
}

fn faces_of(body: &Body<f64>, edge: EdgeKey) -> Vec<topo::FaceKey> {
    let e = body.get_edge(edge).unwrap();
    [e.he_plus, e.he_minus]
        .into_iter()
        .filter_map(|he| body.face_of_half_edge(he))
        .collect()
}

/// A vertical line edge of `body` re-described as a declared (or not)
/// arc bowing off the face `pick` selects.
fn bowed_arc(mut body: Body<f64>, label: &str, want: geom::SurfaceKind, declared: bool) {
    let (edge, a, b) = body
        .edges()
        .find_map(|(k, e)| {
            let c = body.get_curve_geom(e.curve).and_then(CurveGeom::certified)?;
            match c.carrier() {
                Curve3::Line { dir, .. } if dir.z.abs() > 0.9 => {
                    let (t0, t1) = c.params();
                    Some((k, c.carrier().eval(t0), c.carrier().eval(t1)))
                }
                _ => None,
            }
        })
        .unwrap();
    let face = faces_of(&body, edge)
        .into_iter()
        .find(|&f| body.get_surface(body.get_face(f).unwrap().surface).unwrap().kind() == want)
        .unwrap();
    let skey = body.get_face(face).unwrap().surface;
    let off = match body.get_surface(skey).unwrap() {
        Surface::Plane { normal, .. } => *normal,
        Surface::Cylinder { origin, .. } => {
            let r = Vec3::new(a.x - origin.x, a.y - origin.y, 0.0);
            r / r.norm()
        }
        s => panic!("{s:?}"),
    };
    let mid = Point3::new((a.x + b.x) / 2.0, (a.y + b.y) / 2.0, (a.z + b.z) / 2.0);
    let center = mid + off * 2.0;
    let ua = a - center;
    let r = ua.norm();
    let u_ref = ua / r;
    let ub = b - center;
    let axis = u_ref.cross(ub);
    let axis = axis / axis.norm();
    let phi = (u_ref.dot(ub) / r).acos();
    let carrier = Curve3::Circle { center, axis, radius: r, u_ref };
    let d = EdgeDescriptionSpec::chart(skey);
    let spec = EdgeCurveSpec {
        description: if declared { d.declared_by(dummy()) } else { d },
        carrier,
        param_start: 0.0,
        param_end: phi,
    };
    let out = body.set_edge_curve(edge, spec, Tol::witness());
    match &out {
        Err(topo::EulerOpError::Certification { error }) => eprintln!(
            "{label} declared={declared}: {error:?}\n  display: {error}\n  decision: {:?}\n  build: {:?}\n  rest: {:?}",
            error.decision().map(|d| d.0),
            error.ending(geom_brep::recourse::Reading::Build),
            error.ending(geom_brep::recourse::Reading::AtRest),
        ),
        other => eprintln!("{label} declared={declared}: {other:?}"),
    }
}

#[test]
fn probe_declared_arc_off_plane_and_cylinder() {
    let sq = vec![
        (Point2::new(0.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 0.0),
        (Point2::new(1.0, 1.0), 0.0),
        (Point2::new(0.0, 1.0), 0.0),
    ];
    let disc = vec![(Point2::new(1.0, 0.0), 1.0), (Point2::new(-1.0, 0.0), 1.0)];
    census(&extruded(sq.clone()), "box");
    census(&extruded(disc.clone()), "cyl");
    for declared in [true, false] {
        bowed_arc(extruded(sq.clone()), "PLANE", geom::SurfaceKind::Plane, declared);
        bowed_arc(extruded(disc.clone()), "CYLINDER", geom::SurfaceKind::Cylinder, declared);
    }
}

#[test]
fn probe_at_rest_sphere_moved_under_its_edges() {
    let mut body = hemisphere();
    census(&body, "hemi");
    let (fk, skey) = body
        .faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, f)| (k, f.surface))
        .unwrap();
    let Some(Surface::Sphere { center, axis, u_ref, .. }) = body.get_surface(skey).cloned() else {
        panic!()
    };
    // Lifts RechartStrandsDescriptions/RechartUnvouched: the probe's premise is a sphere moved under its edges.
    body.set_face_surface_unvouched_for_tests(
        fk,
        FaceSurface::New { surface: Surface::Sphere { center, radius: 1.5, axis, u_ref }, sense: true },
    )
    .unwrap();
    match topo::validate::validate_geometric(&body, Tol::witness()) {
        Ok(()) => eprintln!("REST: ok"),
        Err(v) => {
            for e in v {
                eprintln!("REST: {e}\n   {e:?}");
            }
        }
    }
}
