//! Shared by the seam-region suites: reading a sphere face's own side,
//! its seam siblings and region, flipping sets of faces, and splitting a
//! lune along a meridian with `mef`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use std::f64::consts::PI;
use topo::{Body, FaceKey, FaceSurface, MefSite, ValidationError};

pub(crate) fn v([x, y, z]: [f64; 3]) -> Vec3<f64> {
    Vec3::new(x, y, z)
}

fn face(body: &Body<f64>, key: FaceKey) -> &topo::Face {
    body.faces().find(|(k, _)| *k == key).unwrap().1
}

/// Whether `key`'s own loop encodes no side: one loop, read `Unencoded`.
pub(crate) fn unencoded(body: &Body<f64>, key: FaceKey, tol: Tol) -> bool {
    let f = face(body, key);
    let Ok((outer, _)) = topo::props::loop_edges(body, f.outer) else {
        return false;
    };
    let band = geom_core::Band::linear(tol).unwrap();
    f.rings.is_empty()
        && geom_brep::props::boundary_material_sign(
            body.get_surface(f.surface).unwrap(),
            &outer,
            band,
        ) == Ok(geom_brep::props::MaterialSign::Unencoded)
}

pub(crate) fn sphere_faces(body: &Body<f64>) -> Vec<FaceKey> {
    body.faces()
        .filter(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(k, _)| k)
        .collect()
}

/// The faces across `key`'s loops, with every half-edge on them.
fn across(body: &Body<f64>, key: FaceKey) -> Vec<FaceKey> {
    let f = face(body, key);
    core::iter::once(f.outer)
        .chain(f.rings.iter().copied())
        .flat_map(|lk| topo::props::loop_edges(body, lk).unwrap().1)
        .map(|h| body.face_of_half_edge(body.mate(h).unwrap()).unwrap())
        .collect()
}

/// `key`'s seam region: the faces of its surface entity it reaches
/// across shared edges, and whether the region has a boundary (an edge
/// one of its faces shares with a face outside it).
pub(crate) fn seam_region(body: &Body<f64>, key: FaceKey) -> (Vec<FaceKey>, bool) {
    let carrier = face(body, key).surface;
    let mut members = vec![key];
    let mut bounded = false;
    let mut i = 0;
    while let Some(&f) = members.get(i) {
        for other in across(body, f) {
            if face(body, other).surface != carrier {
                bounded = true;
            } else if !members.contains(&other) {
                members.push(other);
            }
        }
        i += 1;
    }
    (members, bounded)
}

/// The pairs of faces on one sphere surface that share an edge and
/// each encode no side alone.
pub(crate) fn seam_siblings(body: &Body<f64>, tol: Tol) -> Vec<[FaceKey; 2]> {
    let mut pairs = Vec::new();
    for k in sphere_faces(body) {
        if !unencoded(body, k, tol) {
            continue;
        }
        for other in across(body, k) {
            if other > k
                && face(body, other).surface == face(body, k).surface
                && unencoded(body, other, tol)
                && !pairs.contains(&[k, other])
            {
                pairs.push([k, other]);
            }
        }
    }
    pairs
}

pub(crate) fn flipped(body: &Body<f64>, set: &[FaceKey]) -> Body<f64> {
    set.iter().fold(body.clone(), |b, &k| {
        b.flipped_face_sense_for_tests(k).unwrap()
    })
}

/// Flips every face of `set` and asserts tier 3 refuses the body,
/// naming each of `named` `CurvedSenseInverted`.
pub(crate) fn assert_refused(
    label: &str,
    body: &Body<f64>,
    set: &[FaceKey],
    named: &[FaceKey],
    tol: Tol,
) {
    let flipped = flipped(body, set);
    let errs = topo::validate_geometric(&flipped, tol)
        .err()
        .unwrap_or_else(|| {
            panic!(
                "{label}: flipping {set:?} passes tier 3 (measures {:?})",
                topo::mass_properties(&flipped, tol).map(|m| m.volume)
            )
        });
    for &face in named {
        assert!(
            errs.iter().any(
                |e| matches!(e, ValidationError::CurvedSenseInverted { face: f } if *f == face)
            ),
            "{label}: flipping {set:?} does not name {face:?} CurvedSenseInverted: {errs:?}"
        );
    }
}

/// The sphere `key` lies on: centre, radius and unit polar axis.
pub(crate) fn sphere_of(body: &Body<f64>, key: FaceKey) -> (Point3<f64>, f64, Vec3<f64>) {
    match body.get_surface(face(body, key).surface) {
        Some(&Surface::Sphere {
            center,
            radius,
            axis,
            ..
        }) => (center, radius, axis.normalize()),
        _ => panic!("not a sphere face"),
    }
}

/// Splits `key`, whose loop holds both poles as vertices, from pole to
/// pole along the meridian leaving the north pole in direction `dir`
/// (normal to the polar axis), by `mef`, as a face of the same surface.
pub(crate) fn split_at_meridian(body: &mut Body<f64>, key: FaceKey, dir: Vec3<f64>, tol: Tol) {
    let f = face(body, key).clone();
    let (center, radius, pole) = sphere_of(body, key);
    let (_, hes) = topo::props::loop_edges(body, f.outer).unwrap();
    let points: Vec<_> = body.vertex_points().collect();
    let at = |h: topo::HalfEdgeKey| {
        let v = body.get_half_edge(h).unwrap().start;
        points.iter().find(|(k, _)| *k == v).unwrap().1
    };
    let vertex = |p: Point3<f64>| {
        *hes.iter()
            .find(|&&h| (at(h) - p).norm() < 1e-9)
            .expect("a pole vertex on the loop")
    };
    let (he1, he2) = (
        vertex(center + pole * radius),
        vertex(center - pole * radius),
    );
    let circle = Curve3::Circle {
        center,
        axis: pole.cross(dir),
        radius,
        u_ref: pole,
    };
    body.mef(
        MefSite::Chords { he1, he2 },
        EdgeCurveSpec::arc_of_circle(circle, 0.0, PI)
            .unwrap()
            .at_rest_in_chart(f.surface, false),
        FaceSurface::Inherit,
        tol,
    )
    .unwrap();
}
