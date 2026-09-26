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
//!
//! The cube this drives, its two construction steps and the chord-line
//! and Newell-plane specs under them are [`crate::test_support_fixtures`]'s
//! — the crate's shared Euler-op fixture family, named by path because
//! this module is in-crate. Nothing box-shaped is built here.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    dead_code,
    clippy::too_many_lines
)]

use crate::boolean::ContactRecords;
use crate::test_support_fixtures::{describe_as_intersections, face_surface_of_he, geometric_cube};
use crate::validate::{self, ValidationError};
use crate::{Body, FaceSurface};
use geom::{NurbsSurface, Surface};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Tol};
use std::sync::Arc;

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

/// The unit cube with its front wall restated as a described NURBS and
/// the wall's four edges re-described as plane × NURBS `Intersection`s
/// through the lane door (M7-8). Returns the wall's surface key and the
/// four edge keys.
fn m7_8_cube() -> (
    Body<f64>,
    crate::geometry::SurfaceKey,
    Vec<crate::entity::EdgeKey>,
) {
    let cube = geometric_cube::<f64>(Tol::witness());
    let mut body = cube.body;
    describe_as_intersections(&mut body, Tol::witness());
    let front = cube.mefs[1].face;
    // `mefs[1]` is the y = 0 wall by the bundle's ORDER, which is a
    // property of `CubeOps` and not of anything the lengths pin: a
    // reordering that kept `[MefCreated; 5]` full would move this probe
    // onto another face and still pass everything below.
    {
        let plane = body
            .get_surface(body.get_face(front).unwrap().surface)
            .unwrap();
        let Surface::Plane { origin, normal, .. } = plane else {
            panic!("the cube's walls are planes");
        };
        assert!(
            origin.y.abs() < 1e-12
                && normal.y < -0.5
                && normal.x.abs() < 1e-12
                && normal.z.abs() < 1e-12,
            "`CubeOps::mefs[1]` is the outward-−y wall this probe corrupts"
        );
    }
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

/// Six at-rest doors — the three passes' plain forms and their three
/// `_structural` twins, in that order; the full roster is
/// `validate.rs`'s module-doc table.
const DOOR_NAMES: [&str; 6] = [
    "validate_geometric",
    "validate_pseudomanifold",
    "contact_marks",
    "validate_geometric_structural",
    "validate_pseudomanifold_structural",
    "contact_marks_structural",
];

fn six_doors(body: &Body<f64>) -> [String; 6] {
    let tol = Tol::witness();
    let records = ContactRecords::default();
    [
        edge_cert_count(&validate::validate_geometric(body, tol)),
        edge_cert_count(&validate::validate_pseudomanifold(body, &records, tol)),
        edge_cert_count(&validate::contact_marks(body, tol).map(|_| ())),
        edge_cert_count(&validate::validate_geometric_structural(body, tol)),
        edge_cert_count(&validate::validate_pseudomanifold_structural(
            body, &records, tol,
        )),
        edge_cert_count(&validate::contact_marks_structural(body, tol).map(|_| ())),
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
/// `_structural` twins do not — at `f64` as much as at a dual, since
/// what decides is the BOUND and not the scalar.
#[test]
fn m3_a_corrupt_m7_8_wall_is_caught_at_every_door_whose_bound_names_the_right() {
    let (mut body, wall, lane_edges) = m7_8_cube();
    let before = six_doors(&body);
    for (name, d) in DOOR_NAMES.iter().zip(&before) {
        eprintln!("[m3 door table] certified body  {name:34} {d}");
    }
    // The described-NURBS face has no flux the closed form computes, and
    // no certified flux lane either, so check 7 answers
    // `VolumeUncomputable` at every door that reaches it; what this row
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
            "{} holds no lane and makes no check-2 claim about this class — a change here is \
             a coverage change and wants its own argument: {}",
            DOOR_NAMES[i],
            after[i]
        );
    }
}

/// **The fold's content, pinned as the exact verdict at the four plain
/// tier-3′/marks names** — not only which side of the line each door
/// falls on, but what each says on the corrupt wall and what it no
/// longer says. `validate_pseudomanifold`, `validate_pseudomanifold_certificate`,
/// `contact_marks` and `contact_marks_declared` each report ONE
/// `EdgeCertification` per lane edge and nothing else: check 2
/// re-derives the four certificates through the plane × NURBS lane and
/// every one is false, so the pass stops there and check 7 — gated on
/// checks 1–6 — is never made. The lane-keeping bodies that used to
/// carry these names skipped the class and answered a single
/// `VolumeUncomputable` (the described-NURBS face has no certified flux
/// lane) with no `EdgeCertification` at all; that verdict is the
/// `_structural` twins' now, and a plain name that answered it here
/// would have dropped check 2's lane. The bodies these four names carry
/// are `f64`-only in production: the `pncad` prelude, `pncad-py`'s
/// `Body.validate_pseudomanifold`, `step-import`'s aggregate gate and
/// the tour's scenes all read through them, and no public door can
/// build this body (the corruption writes `Body::surfaces`, which is
/// `pub(crate)`), which is why the pin is in-crate.
#[test]
fn m3_the_plain_names_report_the_corrupt_m7_8_wall_edge_by_edge_and_nothing_else() {
    let (mut body, wall, lane_edges) = m7_8_cube();
    body.surfaces[wall] = nurbs_wall(0.05);
    let tol = Tol::witness();
    let records = ContactRecords::default();
    let verdicts: [(&str, Result<(), Vec<ValidationError>>); 4] = [
        (
            "validate_pseudomanifold",
            validate::validate_pseudomanifold(&body, &records, tol),
        ),
        (
            "validate_pseudomanifold_certificate",
            validate::validate_pseudomanifold_certificate(&body, &records, tol).map(|_| ()),
        ),
        (
            "contact_marks",
            validate::contact_marks(&body, tol).map(|_| ()),
        ),
        (
            "contact_marks_declared",
            validate::contact_marks_declared(&body, &[], tol).map(|_| ()),
        ),
    ];
    let mut expect = lane_edges.clone();
    expect.sort();
    for (door, verdict) in verdicts {
        let errors = verdict.expect_err("the corrupt wall is refused at every plain name");
        let mut edges: Vec<_> = errors
            .iter()
            .map(|e| match e {
                ValidationError::EdgeCertification { edge, .. } => *edge,
                other => panic!(
                    "{door}: reports {other:?} beside the edge findings — a `VolumeUncomputable` \
                     here is the lane-keeping body's answer, which this name no longer gives"
                ),
            })
            .collect();
        edges.sort();
        assert_eq!(
            edges, expect,
            "{door}: one `EdgeCertification` per lane edge, each lane edge once"
        );
    }
}
