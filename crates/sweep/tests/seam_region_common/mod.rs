//! Shared by the seam-region suites: reading a sphere face's own side,
//! its seam siblings and region, flipping sets of faces, and splitting a
//! lune along a meridian with `mef`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{Curve3, Surface};
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Tol, Vec3};
use std::f64::consts::PI;
use topo::construction::Construct;
use topo::construction::{FaceSurface, MefSite};
use topo::{Body, FaceKey, ValidationError};

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
                // Raw door: a failure message, measuring the flip only where tier 3 passed it.
                topo::test_support::measure_unvalidated(
                    &flipped,
                    tol,
                    topo::test_support::pinned_resolution(tol)
                )
                .map(|m| m.enclosure.volume)
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
    // The door reads the same seam regions, so it refuses the same flips
    // without tier 3.
    // Raw door: the subject is the door's parity with tier 3 over a flip this helper asserts tier 3 refuses.
    let door = topo::test_support::measure_unvalidated(
        &flipped,
        tol,
        topo::test_support::pinned_resolution(tol),
    );
    assert!(
        door.is_err(),
        "{label}: flipping {set:?}: tier 3 refuses, the door measures {:?}",
        door.map(|m| m.enclosure.volume)
    );
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

// Splits of the review of PR 5516 (R2), lifted into rows.

/// `body`'s volume through the typed door, or `None` where tier 3 refuses
/// it (an over-cover) or the measurement does.
pub(crate) fn volume(body: &Body<f64>, tol: Tol) -> Option<f64> {
    let finished = topo::AtRestBody::validate(Body::clone(body), tol).ok()?;
    topo::mass_properties(&finished, tol, topo::test_support::pinned_resolution(tol))
        .ok()
        .map(|m| m.enclosure.volume)
}

/// Splits the face of `body` that a meridian in direction `dir` passes
/// through INSIDE it (the split that keeps the volume, and so leaves a
/// body tier 3 finishes), returning whether one did.
pub(crate) fn split_inside(body: &mut Body<f64>, dir: Vec3<f64>, tol: Tol) -> bool {
    let before = volume(body, tol).unwrap();
    for f in sphere_faces(body) {
        let mut b = body.clone();
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            split_at_meridian(&mut b, f, dir, tol)
        }))
        .is_ok();
        if ok && volume(&b, tol).is_some_and(|w| (w - before).abs() <= 1e-9 * before.abs()) {
            *body = b;
            return true;
        }
    }
    false
}

/// Splits the face of `body` that a meridian in direction `dir` passes
/// OUTSIDE (the split that adds a cover), returning whether one did: the
/// volume moves, or the door refuses the split as an over-cover.
pub(crate) fn split_outside(body: &mut Body<f64>, dir: Vec3<f64>, tol: Tol) -> bool {
    let before = volume(body, tol).unwrap();
    for f in sphere_faces(body) {
        let mut b = body.clone();
        let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            split_at_meridian(&mut b, f, dir, tol)
        }))
        .is_ok();
        // Raw door: the measured body is a seam region covered twice, which tier 3 refuses (SeamRegionOvercovered).
        let moved = match topo::test_support::measure_unvalidated(
            &b,
            tol,
            topo::test_support::pinned_resolution(tol),
        ) {
            Ok(m) => (m.enclosure.volume - before).abs() > 1e-6 * before.abs(),
            Err(topo::MassPropsError::Face {
                source: geom_brep::props::PropsError::SeamRegionOvercovered { .. },
                ..
            }) => true,
            Err(_) => false,
        };
        if ok && moved {
            *body = b;
            return true;
        }
    }
    false
}

pub(crate) fn points(body: &Body<f64>) -> Vec<(topo::VertexKey, Point3<f64>)> {
    body.vertex_points().collect()
}

/// Splits every sphere edge running pole to pole at the equator.
pub(crate) fn split_meridians_at_equator(body: &mut Body<f64>, tol: Tol) {
    let f0 = sphere_faces(body)[0];
    let (center, radius, pole) = sphere_of(body, f0);
    let mut todo = Vec::new();
    for (ek, e) in body.edges() {
        let Some(c) = body.get_curve_geom(e.curve).and_then(|g| g.certified()) else {
            continue;
        };
        let carrier = c.carrier().clone();
        if !matches!(carrier, Curve3::Circle { .. }) {
            continue;
        }
        let (t0, t1) = c.params();
        let h = |t: f64| (carrier.eval(t) - center).dot(pole);
        let (a, b) = (h(t0), h(t1));
        if (a.abs() - radius).abs() > 1e-9 || (b.abs() - radius).abs() > 1e-9 || a * b > 0.0 {
            continue;
        }
        let (mut lo, mut hi) = (t0, t1);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if (h(mid) > 0.0) == (a > 0.0) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        todo.push((ek, 0.5 * (lo + hi)));
    }
    for (ek, t) in todo {
        body.split_edge(ek, t, tol).unwrap();
    }
}

/// Splits each sphere face whose loop holds both poles along the
/// equator, through its own interior.
pub(crate) fn split_faces_at_equator(body: &mut Body<f64>, tol: Tol) {
    let f0 = sphere_faces(body)[0];
    let (center, radius, pole) = sphere_of(body, f0);
    for f in sphere_faces(body) {
        let face = body.faces().find(|(k, _)| *k == f).unwrap().1.clone();
        if !face.rings.is_empty() {
            continue;
        }
        let (_, hes) = topo::props::loop_edges(body, face.outer).unwrap();
        let pts = points(body);
        let at = |h: topo::HalfEdgeKey| {
            let v = body.get_half_edge(h).unwrap().start;
            pts.iter().find(|(k, _)| *k == v).unwrap().1
        };
        let on_eq: Vec<_> = hes
            .iter()
            .copied()
            .filter(|&h| (at(h) - center).dot(pole).abs() < 1e-9)
            .collect();
        if on_eq.len() != 2 {
            continue;
        }
        // The half-edge leaving the equator downward starts the face's
        // CCW (about the pole, under sense) sweep.
        let next_z = |h: topo::HalfEdgeKey| {
            let i = hes.iter().position(|&x| x == h).unwrap();
            let n = hes[(i + 1) % hes.len()];
            (at(n) - center).dot(pole)
        };
        let (down, up) = if next_z(on_eq[0]) < 0.0 {
            (on_eq[0], on_eq[1])
        } else {
            (on_eq[1], on_eq[0])
        };
        let a = (at(down) - center) / radius;
        let b = (at(up) - center) / radius;
        let p = pole;
        let w = p.cross(a);
        let mut ang = b.dot(w).atan2(b.dot(a));
        if !face.sense {
            ang = -ang;
        }
        if ang <= 0.0 {
            ang += std::f64::consts::TAU;
        }
        let axis = if face.sense { p } else { -p };
        let circle = Curve3::Circle {
            center,
            axis,
            radius,
            u_ref: a,
        };
        let spec = EdgeCurveSpec::arc_of_circle(circle, 0.0, ang)
            .unwrap()
            .at_rest_in_chart(face.surface, false);
        let r = body.mef(
            MefSite::Chords { he1: down, he2: up },
            spec,
            FaceSurface::Inherit,
            tol,
        );
        if let Err(e) = r {
            panic!("equator mef on {f:?} refused: {e:?}");
        }
    }
}
