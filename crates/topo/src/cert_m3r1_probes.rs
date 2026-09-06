//! **The M7-8 at-rest door table** — which door catches a corrupt plane ×
//! NURBS wall at `f64`, executed rather than argued.
//!
//! A reviewer probe, adopted. In-crate because the corruption route needs
//! `Body::surfaces` (`pub(crate)`): the wall is replaced UNDER its own key
//! after its four edges were attached through the lane door, so the stored
//! certificates no longer hold and nothing else about the body moves.
//! `set_face_surface` cannot do it — `FaceSurface::New` mints a fresh key
//! and the description keeps the old surface alive.
//!
//! What the row pins is the split this unit landed: check 2 re-derives the
//! M7-8 class at every door whose BOUND names the certification right, and
//! at no door that does not. The header below is the adoption's only
//! addition — the sibling in-src probe module (`n2r1_probes.rs`) carries
//! the identical line, and without it a `#[cfg(test)]` module inside
//! `src/` is linted as production code.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    dead_code,
    clippy::too_many_lines
)]

use crate::boolean::ContactRecords;
use crate::validate::{self, ValidationError};
use crate::{Body, FaceSurface, MefCreated, MefSite, MevCreated, MevSite, MvfsCreated};
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec, newell_plane};
use geom_core::spline::KnotVector;
use geom_core::{Band, Decide, Point3, Real, Tol};
use std::sync::Arc;

struct GeoCube<T: Real> {
    body: Body<T>,
    seed: MvfsCreated,
    mevs: [MevCreated; 7],
    mefs: [MefCreated; 5],
}

fn line<T: Real>(p0: Point3<T>, p1: Point3<T>) -> EdgeCurveSpec<T> {
    EdgeCurveSpec::line_between(p0, p1)
}

fn plane<T: Decide>(corners: &[Point3<T>]) -> Surface<T> {
    newell_plane(corners, Band::linear(Tol::witness()).unwrap()).unwrap()
}

/// `topo/tests/common::geometric_cube`, copied verbatim (in-crate).
fn geometric_cube<T: Decide>() -> GeoCube<T> {
    let c = |x: f64, y: f64, z: f64| Point3::new(T::from_f64(x), T::from_f64(y), T::from_f64(z));
    let (a, b, cc, d) = (
        c(0.0, 0.0, 0.0),
        c(1.0, 0.0, 0.0),
        c(1.0, 1.0, 0.0),
        c(0.0, 1.0, 0.0),
    );
    let (a1, b1, c1, d1) = (
        c(0.0, 0.0, 1.0),
        c(1.0, 0.0, 1.0),
        c(1.0, 1.0, 1.0),
        c(0.0, 1.0, 1.0),
    );
    let mut body = Body::<T>::new();
    let seed = body.mvfs(a).unwrap();
    let e_ab = body
        .mev(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            b,
            line(a, b),
            Tol::witness(),
        )
        .unwrap();
    let strut = |body: &mut Body<T>, at, from, to| {
        body.mev(
            MevSite::Fan { he1: at, he2: at },
            to,
            line(from, to),
            Tol::witness(),
        )
        .unwrap()
    };
    let e_bc = strut(&mut body, e_ab.he_minus, b, cc);
    let e_cd = strut(&mut body, e_bc.he_minus, cc, d);
    let he_dc = body
        .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
        .unwrap();
    let f_bottom = body
        .mef(
            MefSite::Chords {
                he1: he_dc,
                he2: e_ab.he_plus,
            },
            line(d, a),
            FaceSurface::New(plane(&[a, d, cc, b])),
            Tol::witness(),
        )
        .unwrap();
    let e_aa = strut(&mut body, e_ab.he_plus, a, a1);
    let e_bb = strut(&mut body, e_bc.he_plus, b, b1);
    let e_cc = strut(&mut body, e_cd.he_plus, cc, c1);
    let e_dd = strut(&mut body, f_bottom.he_plus, d, d1);
    let f_front = body
        .mef(
            MefSite::Chords {
                he1: e_aa.he_minus,
                he2: e_bb.he_minus,
            },
            line(a1, b1),
            FaceSurface::New(plane(&[a, b, b1, a1])),
            Tol::witness(),
        )
        .unwrap();
    let f_right = body
        .mef(
            MefSite::Chords {
                he1: e_bb.he_minus,
                he2: e_cc.he_minus,
            },
            line(b1, c1),
            FaceSurface::New(plane(&[b, cc, c1, b1])),
            Tol::witness(),
        )
        .unwrap();
    let f_back = body
        .mef(
            MefSite::Chords {
                he1: e_cc.he_minus,
                he2: e_dd.he_minus,
            },
            line(c1, d1),
            FaceSurface::New(plane(&[cc, d, d1, c1])),
            Tol::witness(),
        )
        .unwrap();
    let f_left = body
        .mef(
            MefSite::Chords {
                he1: e_dd.he_minus,
                he2: f_front.he_plus,
            },
            line(d1, a1),
            FaceSurface::New(plane(&[d, a, a1, d1])),
            Tol::witness(),
        )
        .unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(plane(&[a1, b1, c1, d1])))
        .unwrap();
    GeoCube {
        body,
        seed,
        mevs: [e_ab, e_bc, e_cd, e_aa, e_bb, e_cc, e_dd],
        mefs: [f_bottom, f_front, f_right, f_back, f_left],
    }
}

/// `topo/tests/common::describe_as_intersections`, copied verbatim.
fn describe_as_intersections<T: Decide>(body: &mut Body<T>) {
    let band = Band::linear(Tol::witness()).unwrap();
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    for (edge_key, edge) in edges {
        let s1 = face_surface_of_he(body, edge.he_plus);
        let s2 = face_surface_of_he(body, edge.he_minus);
        let start = body.get_half_edge(edge.he_plus).unwrap().start;
        let end = body.half_edge_end(edge.he_plus).unwrap();
        let p0 = *body
            .get_point(body.get_vertex(start).unwrap().point)
            .unwrap();
        let p1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
        let witness = p0.lerp(p1, T::from_f64(0.5));
        let (surf1, surf2) = (
            body.get_surface(s1).unwrap().clone(),
            body.get_surface(s2).unwrap().clone(),
        );
        match geom_brep::classify_dihedral(&surf1, &surf2, witness, p0.distance(p1), band).unwrap()
        {
            geom_brep::DihedralClass::Smooth => continue,
            geom_brep::DihedralClass::Transverse => {}
        }
        let mut spec = EdgeCurveSpec::line_between(p0, p1);
        spec.description = EdgeDescriptionSpec::Intersection { s1, s2, witness };
        body.set_edge_curve(edge_key, spec, Tol::witness()).unwrap();
    }
}

/// A DESCRIBED (non-placeholder) degree-2 NURBS patch on the plane
/// `y = 0`, u along +x, v along +z (normal −y = the front wall's outward
/// normal). `bow` lifts the centre control point off the plane.
fn nurbs_wall(bow: f64) -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let ticks = [-1.0, 0.5, 2.0];
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for (i, &x) in ticks.iter().enumerate() {
        for (j, &z) in ticks.iter().enumerate() {
            let y = if i == 1 && j == 1 { bow } else { 0.0 };
            control.push(Point3::new(x, y, z));
            weights.push(1.0);
        }
    }
    let n = NurbsSurface::new(k.clone(), k, control, weights).unwrap();
    assert!(!n.is_placeholder());
    Surface::Nurbs(Arc::new(n))
}

fn face_surface_of_he<T: Decide>(
    body: &Body<T>,
    he: crate::entity::HalfEdgeKey,
) -> crate::geometry::SurfaceKey {
    let he_data = body.get_half_edge(he).unwrap();
    let loop_data = body.get_loop(he_data.parent_loop).unwrap();
    body.get_face(loop_data.face).unwrap().surface
}

/// The unit cube with its front wall restated as a described NURBS and
/// the wall's four edges re-described as plane × NURBS `Intersection`s
/// through the lane door (M7-8). Returns the wall's surface key and the
/// four edge keys.
fn m7_8_cube() -> (
    Body<f64>,
    crate::geometry::SurfaceKey,
    Vec<crate::entity::EdgeKey>,
) {
    let cube = geometric_cube::<f64>();
    let mut body = cube.body;
    describe_as_intersections(&mut body);
    let front = cube.mefs[1].face;
    let wall = body
        .set_face_surface(front, FaceSurface::New(nurbs_wall(0.0)))
        .unwrap();
    let edges: Vec<_> = body.edges().map(|(k, e)| (k, e.clone())).collect();
    let mut lane_edges = Vec::new();
    for (edge_key, edge) in edges {
        let s1 = face_surface_of_he(&body, edge.he_plus);
        let s2 = face_surface_of_he(&body, edge.he_minus);
        if s1 != wall && s2 != wall {
            continue;
        }
        let start = body.get_half_edge(edge.he_plus).unwrap().start;
        let end = body.half_edge_end(edge.he_plus).unwrap();
        let p0 = *body
            .get_point(body.get_vertex(start).unwrap().point)
            .unwrap();
        let p1 = *body.get_point(body.get_vertex(end).unwrap().point).unwrap();
        let witness = p0.lerp(p1, 0.5);
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        let carrier = geom::Curve3::Nurbs(Arc::new(
            geom::NurbsCurve3::new(kv, vec![p0, p1], vec![1.0, 1.0]).unwrap(),
        ));
        let spec = EdgeCurveSpec {
            description: EdgeDescriptionSpec::Intersection { s1, s2, witness },
            carrier,
            param_start: 0.0,
            param_end: 1.0,
        };
        body.set_edge_curve_nurbs_lane(edge_key, spec, Tol::witness())
            .unwrap_or_else(|e| {
                panic!("the plane x flat-NURBS edge attaches through the lane: {e:?}")
            });
        lane_edges.push(edge_key);
    }
    assert_eq!(lane_edges.len(), 4, "the front wall has four edges");
    (body, wall, lane_edges)
}

fn edge_cert_count(r: &Result<(), Vec<ValidationError>>) -> String {
    match r {
        Ok(()) => "Ok".to_owned(),
        Err(errs) => {
            let n = errs
                .iter()
                .filter(|e| matches!(e, ValidationError::EdgeCertification { .. }))
                .count();
            let other: Vec<_> = errs
                .iter()
                .filter(|e| !matches!(e, ValidationError::EdgeCertification { .. }))
                .map(|e| format!("{e:?}"))
                .map(|s| s.chars().take(60).collect::<String>())
                .collect();
            format!("Err({} EdgeCertification, other={other:?})", n)
        }
    }
}

/// The six at-rest doors, in one order: the three whose bound names the
/// certification right, then the three that keep their lane.
const DOOR_NAMES: [&str; 6] = [
    "validate_geometric",
    "validate_pseudomanifold_certified",
    "contact_marks_certified",
    "validate_geometric_structural",
    "validate_pseudomanifold",
    "contact_marks",
];

fn six_doors(body: &Body<f64>) -> [String; 6] {
    let tol = Tol::witness();
    let records = ContactRecords::default();
    [
        edge_cert_count(&validate::validate_geometric(body, tol)),
        edge_cert_count(&validate::validate_pseudomanifold_certified(
            body, &records, tol,
        )),
        edge_cert_count(&validate::contact_marks_certified(body, tol).map(|_| ())),
        edge_cert_count(&validate::validate_geometric_structural(body, tol)),
        edge_cert_count(&validate::validate_pseudomanifold(body, &records, tol)),
        edge_cert_count(&validate::contact_marks(body, tol).map(|_| ())),
    ]
}

/// **A corrupt plane × NURBS wall at `f64`: which at-rest door catches
/// it.** The wall's four edges certified through the lane door at attach
/// time; the wall then bows 0.05 under its own key, so every one of those
/// certificates is false. Check 2 is the check that re-derives them.
///
/// The row asserts the WHOLE table rather than one door, because the
/// split's content is which side of the line each door falls on: the
/// three doors bounded on the certification right catch it, the three
/// that keep their lane do not — at `f64` as much as at a dual, since
/// what decides is the BOUND and not the scalar.
#[test]
fn m3_a_corrupt_m7_8_wall_is_caught_at_every_door_whose_bound_names_the_right() {
    let (mut body, wall, lane_edges) = m7_8_cube();
    let before = six_doors(&body);
    for (name, d) in DOOR_NAMES.iter().zip(&before) {
        eprintln!("[m3 door table] certified body  {name:34} {d}");
    }
    // The described-NURBS face has no certified flux lane, so check 7
    // answers `VolumeUncomputable` at the composed doors; what this row
    // measures is check 2, so the premise is "no EdgeCertification".
    for d in &before {
        assert!(
            !d.contains("EdgeCertification") || d.contains("(0 EdgeCertification"),
            "the certified body must raise no EdgeCertification anywhere: {d}"
        );
    }
    // `needs_nurbs_lane` answers YES on exactly the four wall edges.
    let mut needs = 0;
    for (k, e) in body.edges() {
        let Some(crate::CurveGeom::Certified(c)) = body.get_curve_geom(e.curve) else {
            continue;
        };
        if c.needs_nurbs_lane(|s| body.surfaces.get(s).cloned()) {
            needs += 1;
            assert!(
                lane_edges.contains(&k),
                "only the wall's edges need the lane"
            );
        }
    }
    assert_eq!(needs, 4, "the front wall has four M7-8 edges");

    // CORRUPT: the wall bows 0.05 at its centre, under its own key.
    body.surfaces[wall] = nurbs_wall(0.05);
    let after = six_doors(&body);
    let caught =
        |d: &String| d.contains("EdgeCertification") && !d.contains("(0 EdgeCertification");
    for (name, d) in DOOR_NAMES.iter().zip(&after) {
        eprintln!(
            "[m3 door table] CORRUPT wall    {name:34} {d}  catches={}",
            caught(d)
        );
    }
    for i in 0..3 {
        assert!(
            caught(&after[i]),
            "{} names the certification right, so check 2 re-derives the class: {}",
            DOOR_NAMES[i],
            after[i]
        );
    }
    for i in 3..6 {
        assert!(
            !caught(&after[i]),
            "{} keeps its lane and makes no check-2 claim about this class — a change here is \
             a coverage change and wants its own argument: {}",
            DOOR_NAMES[i],
            after[i]
        );
    }
}
