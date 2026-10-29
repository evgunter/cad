//! **New-edge mints carry the plane × NURBS class** (a reviewer's rows):
//! `mef` with `FaceSurface::Shared` puts its new face on a different
//! surface from its parent, so the new edge's halves are on two surfaces,
//! and a strut `mev` has no adjacency gate. At `f64` the policy's lane
//! certifies the class through both, where main refused it; at a dual
//! both refuse naming the scalar.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;
use crate::fixture::m7_8::{m7_8_edges, nurbs_wall};
use geom_core::spline::Knots;
use topo::construction::Construct;

use geom_core::{Dual64, Point3, Real, Tol};
use topo::construction::{FaceSurface, MefSite, MevSite};
use topo::{Body, EulerOpError};

/// The cube with its front face on the described wall, and a `mef`
/// across the wall face that doubles one wall edge: the new edge's
/// `he_plus` stays on the wall (NURBS), its `he_minus` bounds a new face
/// sharing the neighbour's PLANE key. The spec is the plane × NURBS
/// `Intersection` of those two keys.
fn mef_the_class<T>() -> (
    Body<T>,
    Result<topo::construction::MefCreated, EulerOpError>,
)
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
            FaceSurface::New {
                surface: nurbs_wall::<T>(),
                sense: true,
            },
        )
        .unwrap();
    let (he1, h) = body
        .half_edges()
        .find(|(k, _)| body.face_of_half_edge(*k) == Some(wall_face))
        .map(|(k, h)| (k, h.clone()))
        .unwrap();
    let he2 = h.next;
    let edge = body.get_edge(h.edge).unwrap().clone();
    let twin = if edge.he_plus == he1 {
        edge.he_minus
    } else {
        edge.he_plus
    };
    let neighbour = body.face_of_half_edge(twin).unwrap();
    let nface = body.get_face(neighbour).unwrap().clone();
    assert_ne!(nface.surface, wall);
    assert!(matches!(
        body.get_surface(nface.surface),
        Some(geom::Surface::Plane { .. })
    ));
    let point = |he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let (p0, p1): (Point3<T>, Point3<T>) = (point(he1), point(he2));
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let spec = geom_brep::EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(wall, nface.surface),
            selector: geom_brep::CurveSelector::Witness(p0.lerp(p1, T::from_f64(0.5))),
        },
        carrier: geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                Knots::from(kv).lift(geom_core::Real::from_f64),
                vec![p0, p1],
                vec![Real::one(); 2],
            )
            .unwrap(),
        )),
        param_start: T::zero(),
        param_end: T::one(),
    };
    let r = body.mef(
        MefSite::Chords { he1, he2 },
        spec,
        FaceSurface::Shared {
            key: nface.surface,
            sense: nface.sense,
        },
        tol,
    );
    (body, r)
}

/// At `f64` the policy's lane certifies the class through `mef`.
#[test]
fn review_mef_mints_the_plane_x_nurbs_class_at_f64() {
    let (body, r) = mef_the_class::<f64>();
    let created = r.unwrap_or_else(|e| panic!("mef at f64: {e:?}"));
    let he_minus_face = body.face_of_half_edge(created.he_minus).unwrap();
    let he_plus_face = body.face_of_half_edge(created.he_plus).unwrap();
    assert_ne!(
        body.get_face(he_minus_face).unwrap().surface,
        body.get_face(he_plus_face).unwrap().surface,
        "the new edge's halves are on two surfaces"
    );
    assert_eq!(m7_8_edges(&body), 1, "mef minted one edge of the class");
}

/// At a dual the same `mef` mints the edge through the dual's door
/// (DL3).
#[test]
fn review_mef_at_a_dual_mints_the_class() {
    let (body, r) = mef_the_class::<Dual64>();
    assert!(r.is_ok(), "mef at the dual: {r:?}");
    assert_eq!(m7_8_edges(&body), 1, "mef minted one edge of the class");
}

/// A strut `mev` into the wall face whose spec is the plane × NURBS
/// `Intersection` of the wall and its neighbour plane: both halves of
/// the new edge are in ONE face, yet neither `IntersectionSameSurface`
/// (the keys differ) nor `DescriptionNotAdjacent` (no adjacency gate at
/// `mev`) comes first, so the edge door decides.
fn mev_the_class<T>() -> Result<topo::construction::MevCreated, EulerOpError>
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
            FaceSurface::New {
                surface: nurbs_wall::<T>(),
                sense: true,
            },
        )
        .unwrap();
    let (he1, h) = body
        .half_edges()
        .find(|(k, _)| body.face_of_half_edge(*k) == Some(wall_face))
        .map(|(k, h)| (k, h.clone()))
        .unwrap();
    let edge = body.get_edge(h.edge).unwrap().clone();
    let twin = if edge.he_plus == he1 {
        edge.he_minus
    } else {
        edge.he_plus
    };
    let plane = body
        .get_face(body.face_of_half_edge(twin).unwrap())
        .unwrap()
        .surface;
    let point = |he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let (p0, p1): (Point3<T>, Point3<T>) = (point(he1), point(h.next));
    let mid = p0.lerp(p1, T::from_f64(0.5));
    let kv = geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let spec = geom_brep::EdgeCurveSpec {
        description: geom_brep::EdgeDescriptionSpec::Intersection {
            pair: geom_brep::SurfacePair::new(wall, plane),
            selector: geom_brep::CurveSelector::Witness(p0.lerp(mid, T::from_f64(0.5))),
        },
        carrier: geom::Curve3::Nurbs(std::sync::Arc::new(
            geom::NurbsCurve3::new(
                Knots::from(kv).lift(geom_core::Real::from_f64),
                vec![p0, mid],
                vec![Real::one(); 2],
            )
            .unwrap(),
        )),
        param_start: T::zero(),
        param_end: T::one(),
    };
    body.mev(MevSite::Fan { he1, he2: he1 }, mid, spec, tol)
}

#[test]
fn review_mev_strut_reaches_the_edge_door_at_both_scalars() {
    let f = mev_the_class::<f64>().map(|_| ());
    assert!(f.is_ok(), "a strut of the class at f64: {f:?}");
    let d = mev_the_class::<Dual64>().map(|_| ());
    assert!(d.is_ok(), "a strut of the class at the dual: {d:?}");
}
