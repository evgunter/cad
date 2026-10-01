//! **The edge-mint doors read the plane × NURBS lane off the scalar's
//! policy** — the M7-8 cube (`fixture::m7_8`) re-described, split and
//! re-charted through the public doors at `f64`, and the same doors at a
//! dual refusing the class typed, naming the scalar.
//!
//! The fan `mev`'s re-basing gate is pinned beside its fixture, in
//! `euler`'s own tests (the pillow is crate-internal).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use crate::fixture::m7_8::{edge_findings, m7_8_cube, m7_8_edges, nurbs_wall};

use geom_core::{Dual64, Point3, Real, Tol};
use topo::{Body, EdgeKey, EulerOpError, FaceKey, Rechart};

/// The M7-8 edges of `body` with their restated specs, in arena order.
fn wall_edges<T: Real>(body: &Body<T>) -> Vec<(EdgeKey, geom_brep::EdgeCurveSpec<T>)> {
    body.edges()
        .filter_map(|(k, e)| match body.get_curve_geom(e.curve) {
            Some(topo::CurveGeom::Certified(c))
                if matches!(c.carrier(), geom::Curve3::Nurbs(_))
                    && matches!(
                        c.description(),
                        geom_brep::EdgeDescription::Intersection { .. }
                    ) =>
            {
                Some((k, c.restated_spec()))
            }
            _ => None,
        })
        .collect()
}

/// The face on the plane `z = 1`, which meets the wall along one edge.
fn top_face(body: &Body<f64>) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(body.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, normal, .. })
                    if normal.z.abs() > 0.5 && origin.z == 1.0)
        })
        .map(|(k, _)| k)
        .unwrap()
}

#[test]
fn set_edge_curve_re_describes_every_m7_8_edge_at_f64() {
    let mut body = m7_8_cube::<f64>();
    let edges = wall_edges(&body);
    assert_eq!(edges.len(), 4, "the wall's four edges are the class");
    for (edge, spec) in edges {
        body.set_edge_curve(edge, spec, Tol::witness())
            .unwrap_or_else(|e| panic!("edge {edge:?} re-describes at f64: {e:?}"));
    }
    assert_eq!(
        m7_8_edges(&body),
        4,
        "the class survives the re-description"
    );
    assert_eq!(edge_findings(&body), 0, "check 2 is clean after it");
}

#[test]
fn split_edge_splits_an_m7_8_edge_into_two_of_the_class_at_f64() {
    let mut body = m7_8_cube::<f64>();
    let (edge, _) = wall_edges(&body)[0].clone();
    body.split_edge(edge, 0.5, Tol::witness())
        .expect("an M7-8 edge splits at f64");
    assert_eq!(m7_8_edges(&body), 5, "both children are of the class");
    assert_eq!(edge_findings(&body), 0, "check 2 is clean after the split");
}

/// The top face moves onto a fresh copy of its own plane, and every
/// edge on it — one of them the wall's — re-certifies on the new chart.
#[test]
fn a_rechart_beside_the_wall_re_certifies_its_m7_8_edge_at_f64() {
    let mut body = m7_8_cube::<f64>();
    let top = top_face(&body);
    let face = body.get_face(top).unwrap().clone();
    let plane = body.get_surface(face.surface).unwrap().clone();
    let specs: Vec<_> = body
        .edges()
        .filter(|(_, e)| {
            [e.he_plus, e.he_minus]
                .iter()
                .any(|&h| body.face_of_half_edge(h) == Some(top))
        })
        .map(|(k, e)| match body.get_curve_geom(e.curve) {
            Some(topo::CurveGeom::Certified(c)) => (k, c.restated_spec()),
            other => panic!("edge {k:?} on the top face is certified: {other:?}"),
        })
        .collect();
    assert!(
        specs
            .iter()
            .any(|(_, s)| matches!(s.carrier, geom::Curve3::Nurbs(_))),
        "the listed edges include the wall's"
    );
    body.set_face_surfaces_describing(
        vec![Rechart::new(plane, top, face.sense)],
        &specs,
        Tol::witness(),
    )
    .expect("the top face re-charts with its M7-8 edge at f64");
    assert_ne!(
        body.get_face(top).unwrap().surface,
        face.surface,
        "the face wears the new chart"
    );
    assert_eq!(m7_8_edges(&body), 4, "the class survives the re-chart");
    assert_eq!(edge_findings(&body), 0, "check 2 is clean after it");
}

/// The cube at `T` with its front face on the described wall, before
/// any wall edge is re-described, and the plane × NURBS spec of one wall
/// edge, its witness on both surfaces. Returns the body, that edge, the
/// spec and the wall's face.
fn bare_wall<T>() -> (Body<T>, EdgeKey, geom_brep::EdgeCurveSpec<T>, FaceKey)
where
    T: geom_core::Decide + topo::AtRestPolicy,
{
    let tol = Tol::witness();
    let cube = common::geometric_cube::<T>(tol);
    let mut body = cube.body;
    let wall_face = cube.mefs[1].face;
    let wall = body
        .set_face_surface(
            wall_face,
            topo::FaceSurface::New {
                surface: nurbs_wall::<T>(),
                sense: true,
            },
        )
        .unwrap();
    let (edge, e) = body
        .edges()
        .find(|(_, e)| {
            common::face_surface_of_he(&body, e.he_plus) == wall
                || common::face_surface_of_he(&body, e.he_minus) == wall
        })
        .map(|(k, e)| (k, e.clone()))
        .unwrap();
    let (s1, s2) = (
        common::face_surface_of_he(&body, e.he_plus),
        common::face_surface_of_he(&body, e.he_minus),
    );
    let point = |he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let (p0, p1): (Point3<T>, Point3<T>) = (point(e.he_plus), point(e.he_minus));
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let spec = geom_brep::EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            s1,
            s2,
            witness: p0.lerp(p1, T::from_f64(0.5)),
        },
        carrier: geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0]).unwrap(),
        )),
        param_start: T::zero(),
        param_end: T::one(),
    };
    (body, edge, spec, wall_face)
}

/// The control the dual rows lean on: the same construction certifies
/// at `f64`, so their refusal is the lane's absence and not a residual
/// the spec carries.
#[test]
fn the_bare_wall_spec_certifies_at_f64() {
    let (mut body, edge, spec, _) = bare_wall::<f64>();
    body.set_edge_curve(edge, spec, Tol::witness())
        .expect("the spec certifies at f64");
    assert_eq!(m7_8_edges(&body), 1);
}

#[test]
fn at_a_dual_set_edge_curve_refuses_the_class_naming_the_scalar() {
    let (mut body, edge, spec, _) = bare_wall::<Dual64>();
    let before = body.get_edge(edge).unwrap().curve;
    assert_eq!(
        body.set_edge_curve(edge, spec, Tol::witness()),
        Err(EulerOpError::NurbsLaneUnsupported {
            edge: Some(edge),
            scalar: Dual64::NAME,
        })
    );
    assert_eq!(body.get_edge(edge).unwrap().curve, before, "body untouched");
}

/// The wall face re-charts onto a fresh copy of its own net, and the
/// listed spec names the wall's current key: the re-chart reads the
/// same policy as the edge setter.
#[test]
fn at_a_dual_a_rechart_refuses_the_class_naming_the_scalar() {
    let (mut body, edge, spec, wall_face) = bare_wall::<Dual64>();
    let face = body.get_face(wall_face).unwrap().clone();
    let net = body.get_surface(face.surface).unwrap().clone();
    let stranded: Vec<_> = body
        .edges()
        .filter(|(_, e)| {
            [e.he_plus, e.he_minus]
                .iter()
                .any(|&h| body.face_of_half_edge(h) == Some(wall_face))
        })
        .map(|(k, _)| k)
        .collect();
    assert!(stranded.contains(&edge));
    assert_eq!(
        body.set_face_surfaces_describing(
            vec![Rechart::new(net, wall_face, face.sense)],
            &[(edge, spec)],
            Tol::witness(),
        )
        .map(|_| ()),
        Err(EulerOpError::NurbsLaneUnsupported {
            edge: Some(edge),
            scalar: Dual64::NAME,
        })
    );
    assert_eq!(body.get_face(wall_face).unwrap().surface, face.surface);
}

/// `kev_describing` killing the edge that leaves a wall corner off the
/// wall, with the wall edge at that corner listed under its plane × NURBS
/// spec. Returns the door's answer.
fn kev_describing_the_class<T>() -> Result<(), EulerOpError>
where
    T: geom_core::Decide + topo::AtRestPolicy,
{
    let (mut body, edge, spec, wall_face) = bare_wall::<T>();
    let e = body.get_edge(edge).unwrap().clone();
    let corner = body.get_half_edge(e.he_plus).unwrap().start;
    let (he, _) = body
        .half_edges()
        .find(|(k, h)| {
            body.half_edge_end(*k) == Some(corner)
                && body.face_of_half_edge(*k) != Some(wall_face)
                && {
                    let ed = body.get_edge(h.edge).unwrap();
                    body.face_of_half_edge(ed.he_plus) != Some(wall_face)
                        && body.face_of_half_edge(ed.he_minus) != Some(wall_face)
                }
        })
        .map(|(k, h)| (k, h.clone()))
        .unwrap();
    body.kev_describing(he, &[(edge, spec)], Tol::witness())
        .map(|_| ())
}

#[test]
fn at_a_dual_kev_describing_refuses_the_class_naming_the_scalar() {
    let dual = kev_describing_the_class::<Dual64>();
    let (_, edge, _, _) = bare_wall::<Dual64>();
    assert_eq!(
        dual,
        Err(EulerOpError::NurbsLaneUnsupported {
            edge: Some(edge),
            scalar: Dual64::NAME,
        })
    );
    // The control: at f64 the lane is held, so the listed spec reaches
    // its own checks and the answer is not the lane's absence.
    let f = kev_describing_the_class::<f64>();
    assert!(
        !matches!(f, Err(EulerOpError::NurbsLaneUnsupported { .. })),
        "{f:?}"
    );
}
