//! The planar merge door and a declared pair on a CYLINDER carrier.
//!
//! A declared `Rest` pair whose faces both survive a boolean is handed
//! to `merge_coplanar_faces_declared` whatever its carrier. The door's
//! declared-pair rung is planar; a cylindrical pair is a legal
//! declaration it has no rung for, and the door RECORDS it as a
//! `SkippedMerge` carrying `DeclaredCarrierUnsupported` — visible in
//! `BooleanNaming::merge_skipped`, never an `InvalidDeclaration` refusal
//! blaming the caller. Scenes C, D and F reach that record and ship an
//! honest body; scenes A and B, freed of the false refusal, stop at the
//! zip's own defect (`work/curved/rest-zip-seam-chord-on-cylinder-wall`);
//! scene E stops at the reduction.
//!
//! Scenes A–D are `mate2_common`'s; scene D's plate/peg builders are
//! copied from `r1_probes_m9_3` (private there).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::operands::{plate6, plate6_cyl};
use crate::mate2_common;
use geom_brep::SurfaceKind;
use geom_core::{Affine3, Point2, Tol, Vec2, Vec3};
use mate2_common::{
    assert_additive, body_of, boolean_body, collar, collar_at, peg_at, plane_face, volume,
    wall_decls, walls_at,
};
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
use topo::{
    Body, BooleanBody, BooleanDeclarations, BooleanError, ContactClass, FacePairDeclaration,
    FaceSurface, MergeCoplanarError, SkippedMerge, validate_closed, validate_geometric,
    validate_pseudomanifold,
};

const BORE_R: f64 = 0.5;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// Scene A: the peg floats in the bore (z ∈ [1.5, 2.5] against a bore
/// z ∈ [1, 2]); nine wall `Rest`s.
fn scene_a() -> (Body<f64>, Body<f64>, BooleanDeclarations) {
    let c = collar();
    let p = peg_at(0.0, 1.5, 1.0);
    let d = wall_decls(&c, &p);
    (c, p, d)
}

/// Scene B: the peg ends mid-bore (z ∈ [0.5, 1.5]).
fn scene_b() -> (Body<f64>, Body<f64>, BooleanDeclarations) {
    let c = collar_at(0.0);
    let p = peg_at(0.0, 0.5, 1.0);
    let d = wall_decls(&c, &p);
    (c, p, d)
}

/// Scene C: flush at the bottom, proud at the top (z ∈ [1, 2.5]).
fn scene_c() -> (Body<f64>, Body<f64>, BooleanDeclarations) {
    let c = collar_at(0.0);
    let p = peg_at(0.0, 1.0, 1.5);
    let d = wall_decls(&c, &p);
    (c, p, d)
}

/// Scene D: partial engagement — a plate with a peg reaching halfway up
/// the through-bore of a plate above it; one planar `Rest` (the plates'
/// mating faces) plus nine wall `Rest`s.
fn scene_d() -> (Body<f64>, Body<f64>, BooleanDeclarations) {
    let p = body_of(
        topo::union(
            &plate6(0.0),
            &plate6_cyl(2.0, 0.4, 1.1, BORE_R),
            Tol::witness(),
        )
        .unwrap(),
    );
    let q = body_of(
        topo::subtract(
            &plate6(1.0),
            &plate6_cyl(2.0, 0.8, 1.4, BORE_R),
            Tol::witness(),
        )
        .unwrap(),
    );
    let mut d = BooleanDeclarations::none();
    d.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&p, 1.0, true),
        plane_face(&q, 1.0, false),
        ContactClass::Rest,
    ));
    for &fa in &walls_at(&p, BORE_R) {
        for &fb in &walls_at(&q, BORE_R) {
            d.coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    (p, q, d)
}

/// The union must run, be exactly additive, and be tier-3 and
/// 3′-clean — the skip has exposed the zip's body, and that body has
/// to be right on its own.
fn union_honest(
    label: &str,
    a: &Body<f64>,
    b: &Body<f64>,
    d: &BooleanDeclarations,
) -> BooleanBody<f64> {
    let bb = boolean_body(
        topo::union_with(a, b, d, Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: the door must not refuse: {e:?}")),
    );
    assert_additive(volume(&bb.body), volume(a), volume(b));
    assert_eq!(validate_closed(&bb.body), Ok(()), "{label}: tier 2");
    assert_eq!(
        validate_geometric(&bb.body, Tol::witness()),
        Ok(()),
        "{label}: tier 3"
    );
    assert_eq!(
        validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness()),
        Ok(()),
        "{label}: tier 3′"
    );
    bb
}

fn is_cylinder(body: &Body<f64>, face: topo::FaceKey) -> bool {
    body.get_face(face)
        .and_then(|f| body.get_surface(f.surface))
        .is_some_and(|s| matches!(s, geom::Surface::Cylinder { .. }))
}

/// The declared-carrier records of a boolean result: exactly ONE per
/// declared surface pair (the door dedups the copies
/// `declared_surface_pairs` hands it, one per wall FACE pair on the
/// three-arc walls' shared key), carrying the cylinder reason, its
/// faces live and on a cylinder. Every other record is a curved run's
/// `PeriodClosure` (the three-arc sectors on one key), which predates
/// this door's arm.
fn assert_cylinder_records<'a>(label: &str, bb: &'a BooleanBody<f64>) -> Vec<&'a SkippedMerge> {
    let mut declared = Vec::new();
    for skip in &bb.naming.merge_skipped {
        match skip.reason {
            MergeCoplanarError::DeclaredCarrierUnsupported {
                kind: SurfaceKind::Cylinder,
                ..
            } => declared.push(skip),
            MergeCoplanarError::PeriodClosure { .. } => continue,
            ref other => panic!("{label}: unexpected skip reason {other:?}"),
        }
        assert!(!skip.faces.is_empty(), "{label}: a record names its faces");
        for &f in &skip.faces {
            assert!(
                bb.body.get_face(f).is_some(),
                "{label}: recorded face {f:?} is live"
            );
            assert!(
                is_cylinder(&bb.body, f),
                "{label}: recorded face {f:?} is on a cylinder"
            );
        }
    }
    assert_eq!(
        declared.len(),
        1,
        "{label}: one record for the one declared cylinder pair: {declared:?}"
    );
    declared
}

/// The declared-carrier records of a public-door outcome.
fn carrier_records(outcome: &topo::MergeCoplanarOutcome) -> Vec<&SkippedMerge> {
    outcome
        .skipped
        .iter()
        .filter(|s| {
            matches!(
                s.reason,
                MergeCoplanarError::DeclaredCarrierUnsupported { .. }
            )
        })
        .collect()
}

/// The surviving r = 0.5 faces: `(face, sense)` in face-arena order.
fn bore_radius_faces(body: &Body<f64>) -> Vec<(topo::FaceKey, bool)> {
    walls_at(body, BORE_R)
        .into_iter()
        .map(|f| (f, body.get_face(f).unwrap().sense))
        .collect()
}

fn face_of(body: &Body<f64>, he: topo::HalfEdgeKey) -> topo::FaceKey {
    body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
        .unwrap()
        .face
}

/// Edges shared by two DISTINCT faces of `set`, as `(edge, f1, f2)`.
fn shared_edges(
    body: &Body<f64>,
    set: &[topo::FaceKey],
) -> Vec<(topo::EdgeKey, topo::FaceKey, topo::FaceKey)> {
    body.edges()
        .filter_map(|(ek, e)| {
            let (f1, f2) = (face_of(body, e.he_plus), face_of(body, e.he_minus));
            (f1 != f2 && set.contains(&f1) && set.contains(&f2)).then_some((ek, f1, f2))
        })
        .collect()
}

/// Every live face on either key, sorted.
fn faces_on(body: &Body<f64>, k1: topo::SurfaceKey, k2: topo::SurfaceKey) -> Vec<topo::FaceKey> {
    let mut faces: Vec<_> = body
        .faces()
        .filter(|(_, f)| f.surface == k1 || f.surface == k2)
        .map(|(k, _)| k)
        .collect();
    faces.sort();
    faces
}

fn sorted(mut faces: Vec<topo::FaceKey>) -> Vec<topo::FaceKey> {
    faces.sort();
    faces
}

/// Row 1 (C): the door runs, the body is honest, and the cylindrical
/// declared pair is recorded — not refused, not dropped, once.
#[test]
fn proud_peg_declared_walls_union_records_the_cylinder_skip() {
    let (c, p, d) = scene_c();
    let bb = union_honest("C", &c, &p, &d);
    assert_cylinder_records("C", &bb);
}

/// A peg whose three wall sectors each sit on their OWN surface key
/// (same description): no hard rung fires anywhere, so the door has
/// nothing to merge and only the declaration to answer.
fn peg_with_split_wall_keys() -> (Body<f64>, Vec<topo::SurfaceKey>) {
    let mut body = peg_at(0.0, 0.0, 1.0);
    let mut keys = Vec::new();
    for f in walls_at(&body, BORE_R) {
        let described = body
            .get_surface(body.get_face(f).unwrap().surface)
            .unwrap()
            .clone();
        keys.push(
            body.set_face_surface(f, FaceSurface::New(described))
                .unwrap(),
        );
    }
    assert_eq!(keys.len(), 3);
    assert_eq!(validate_closed(&body), Ok(()));
    (body, keys)
}

/// Row 1′ (public door): a declined pair is recorded even when the
/// call finds NOTHING else to merge — the early return carries the
/// record, never an empty outcome — and copies of the pair collapse to
/// one record.
#[test]
fn declined_pair_survives_a_call_with_nothing_to_merge() {
    let (mut body, keys) = peg_with_split_wall_keys();
    let before = format!("{body:?}");
    let pair = (keys[0], keys[1]);
    let outcome = body
        .merge_coplanar_faces_declared(&[pair, pair, pair], Tol::witness())
        .expect("a non-planar declared pair is recorded, never refused");
    assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
    assert_eq!(outcome.skipped.len(), 1, "{:?}", outcome.skipped);
    let skip = &outcome.skipped[0];
    assert_eq!(
        skip.reason,
        MergeCoplanarError::DeclaredCarrierUnsupported {
            pair,
            kind: SurfaceKind::Cylinder,
        }
    );
    assert_eq!(
        sorted(skip.faces.clone()),
        faces_on(&body, keys[0], keys[1]),
        "the record names every live face on either key"
    );
    assert_eq!(
        format!("{body:?}"),
        before,
        "nothing to merge: the body is untouched"
    );
}

/// Row 1″ (public door): a planar pair and a cylinder pair in ONE
/// call are each classed by their own kind — the planar pair joins
/// the equivalence (two cap planes that never meet: a licensed no-op),
/// the cylinder pair is recorded — in either order.
#[test]
fn mixed_list_classifies_each_pair_by_its_own_kind() {
    let (mut body, keys) = peg_with_split_wall_keys();
    let caps = (
        body.get_face(plane_face(&body, 0.0, false))
            .unwrap()
            .surface,
        body.get_face(plane_face(&body, 1.0, true)).unwrap().surface,
    );
    let cyl = (keys[0], keys[1]);
    let before = format!("{body:?}");
    for declared in [[caps, cyl], [cyl, caps]] {
        let outcome = body
            .merge_coplanar_faces_declared(&declared, Tol::witness())
            .unwrap_or_else(|e| panic!("{declared:?}: each pair is its own kind: {e:?}"));
        assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
        assert_eq!(outcome.skipped.len(), 1, "{:?}", outcome.skipped);
        assert_eq!(
            outcome.skipped[0].reason,
            MergeCoplanarError::DeclaredCarrierUnsupported {
                pair: cyl,
                kind: SurfaceKind::Cylinder,
            }
        );
        assert_eq!(
            format!("{body:?}"),
            before,
            "nothing to merge: the body is untouched"
        );
    }
}

/// A prism whose y = 0 wall is split in two coplanar faces by a
/// straight-angle vertex, and whose right end is two arcs of one
/// circle (centre (2.25, 0.5), meeting the straight walls at 26.6°, so
/// no joint is tangent). Returns the two wall keys (made distinct)
/// and two keys of the one cylinder (made distinct).
fn d_prism_with_split_keys() -> (
    Body<f64>,
    (topo::SurfaceKey, topo::SurfaceKey),
    (topo::SurfaceKey, topo::SurfaceKey),
) {
    // The arc's centre is (2.25, 0.5); its far point (2.25 + r, 0.5)
    // splits the 233° sweep into two equal arcs, bulge = tan(θ/4).
    let r = (0.25f64 * 0.25 + 0.5 * 0.5).sqrt();
    let sweep = 2.0 * core::f64::consts::PI - 2.0 * 0.5f64.atan2(0.25);
    let bulge = (sweep / 2.0 / 4.0).tan();
    let lp = bulge_loop(vec![
        (p2(0.0, 0.0), 0.0),
        (p2(1.0, 0.0), 0.0),
        (p2(2.0, 0.0), bulge),
        (p2(2.25 + r, 0.5), bulge),
        (p2(2.0, 1.0), 0.0),
        (p2(0.0, 1.0), 0.0),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, 0.0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let mut body = extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    let y0_walls: Vec<_> = body
        .faces()
        .filter(|(_, f)| match body.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. }) => {
                origin.y.abs() < 1e-12 && normal.y < -0.5
            }
            _ => false,
        })
        .map(|(k, _)| k)
        .collect();
    assert_eq!(
        y0_walls.len(),
        2,
        "the straight-angle vertex splits the y = 0 wall"
    );
    let walls = distinct_keys(&mut body, y0_walls[0], y0_walls[1]);
    let arcs = walls_at(&body, r);
    assert_eq!(arcs.len(), 2, "two arcs of one circle");
    let cyls = distinct_keys(&mut body, arcs[0], arcs[1]);
    assert_eq!(validate_closed(&body), Ok(()));
    (body, walls, cyls)
}

/// The two faces' keys, re-homing `b` onto a fresh copy of its
/// description when the constructor put both on one key.
fn distinct_keys(
    body: &mut Body<f64>,
    a: topo::FaceKey,
    b: topo::FaceKey,
) -> (topo::SurfaceKey, topo::SurfaceKey) {
    let ka = body.get_face(a).unwrap().surface;
    let kb = body.get_face(b).unwrap().surface;
    if ka != kb {
        return (ka, kb);
    }
    let described = body.get_surface(kb).unwrap().clone();
    let fresh = body
        .set_face_surface(b, FaceSurface::New(described))
        .unwrap();
    (ka, fresh)
}

/// Row 1‴ (public door): a `(Plane, Plane)` pair that genuinely meets
/// at an edge GLUES through the equivalence, beside a cylinder pair
/// that is recorded — the two halves of the classification in one
/// call, both visible on the result.
#[test]
fn planar_pair_glues_beside_the_recorded_cylinder_pair() {
    let (mut body, walls, cyls) = d_prism_with_split_keys();
    let faces_before = body.faces().count();
    let outcome = body
        .merge_coplanar_faces_declared(&[walls, cyls], Tol::witness())
        .unwrap_or_else(|e| panic!("{e:?}"));
    assert_eq!(outcome.groups.len(), 1, "{:?}", outcome.groups);
    let group = &outcome.groups[0];
    assert_eq!(group.absorbed.len(), 1, "{group:?}");
    assert!(
        body.get_face(group.kept).is_some() && body.get_face(group.absorbed[0]).is_none(),
        "the planar pair glued: {group:?}"
    );
    assert_eq!(body.faces().count(), faces_before - 1);
    assert_eq!(validate_closed(&body), Ok(()));
    let records = carrier_records(&outcome);
    assert_eq!(records.len(), 1, "{:?}", outcome.skipped);
    assert_eq!(
        records[0].reason,
        MergeCoplanarError::DeclaredCarrierUnsupported {
            pair: cyls,
            kind: SurfaceKind::Cylinder,
        }
    );
    assert_eq!(
        sorted(records[0].faces.clone()),
        faces_on(&body, cyls.0, cyls.1)
    );
}

/// Row 1⁗ (public door): a declined pair BESIDE a committing curved
/// same-key run on one of its own keys — two sectors on `k` glue
/// (sub-period, the kind-agnostic hard rung), the third sits on `k2` —
/// and the record names only faces that are live AFTER the commit.
/// (Construction after the R1 review probe and R2's MAJOR 1.)
#[test]
fn record_beside_a_committing_curved_run_names_only_live_faces() {
    let mut body = peg_at(0.0, 0.0, 1.0);
    let walls = walls_at(&body, BORE_R);
    let k = body.get_face(walls[0]).unwrap().surface;
    let described = body.get_surface(k).unwrap().clone();
    let k2 = body
        .set_face_surface(walls[2], FaceSurface::New(described))
        .unwrap();
    let faces_before = body.faces().count();
    let outcome = body
        .merge_coplanar_faces_declared(&[(k, k2)], Tol::witness())
        .unwrap_or_else(|e| panic!("{e:?}"));
    assert_eq!(
        outcome.groups.len(),
        1,
        "the two same-key sectors glue (a sub-period run commits): {:?}",
        outcome.groups
    );
    assert_eq!(body.faces().count(), faces_before - 1);
    let records = carrier_records(&outcome);
    assert_eq!(records.len(), 1, "{:?}", outcome.skipped);
    for &f in &records[0].faces {
        assert!(body.get_face(f).is_some(), "recorded face {f:?} is live");
    }
    assert_eq!(
        sorted(records[0].faces.clone()),
        faces_on(&body, k, k2),
        "faces are every live face on either key, read after the commit"
    );
}

/// Row 1⁵ (public door): a declared pair with NO live face on either
/// surface — a surface kept alive only by an edge description after
/// every face left it — gets NO record: the declaration served
/// nothing at this door, and a record naming nothing is the silent
/// shape. (Construction after the R1/R2 review probes: scene C's
/// shipped body with every peg-wall face re-keyed.)
#[test]
fn pair_with_no_live_faces_mints_no_record() {
    let (c, p, d) = scene_c();
    let mut body = union_honest("C", &c, &p, &d).body;
    let walls = walls_at(&body, BORE_R);
    let pk = body.get_face(walls[0]).unwrap().surface;
    assert!(
        walls
            .iter()
            .all(|&f| body.get_face(f).unwrap().surface == pk)
    );
    let mut fresh = Vec::new();
    for &f in &walls {
        let described = body.get_surface(pk).unwrap().clone();
        fresh.push(
            body.set_face_surface(f, FaceSurface::New(described))
                .unwrap(),
        );
    }
    assert!(
        body.get_surface(pk).is_some(),
        "{pk:?} stays live with zero faces (held by an edge description) — if the \
         orphan rule changed, re-pin this row on another construction"
    );
    assert!(faces_on(&body, pk, pk).is_empty());
    assert_eq!(validate_closed(&body), Ok(()));
    let before = format!("{body:?}");
    let outcome = body
        .merge_coplanar_faces_declared(&[(pk, pk)], Tol::witness())
        .unwrap_or_else(|e| panic!("{e:?}"));
    assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
    assert!(
        carrier_records(&outcome).is_empty(),
        "no record for a pair naming nothing: {:?}",
        outcome.skipped
    );
    assert_eq!(format!("{body:?}"), before, "the body is untouched");
    // Beside a live key the pair IS recorded, naming that key's faces.
    let outcome = body
        .merge_coplanar_faces_declared(&[(pk, fresh[0])], Tol::witness())
        .unwrap_or_else(|e| panic!("{e:?}"));
    let records = carrier_records(&outcome);
    assert_eq!(records.len(), 1, "{:?}", outcome.skipped);
    assert_eq!(
        sorted(records[0].faces.clone()),
        faces_on(&body, pk, fresh[0])
    );
}

/// A ball of radius `r` (a revolved semicircle).
fn ball(r: f64) -> Body<f64> {
    let lp = bulge_loop(vec![(p2(0.0, -r), 1.0), (p2(0.0, r), 0.0)]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(&profile, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

/// A donut (a revolved circle off the axis).
fn donut() -> Body<f64> {
    let lp = bulge_loop(vec![(p2(1.0, -0.3), 1.0), (p2(1.0, 0.3), 1.0)]);
    let profile = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    revolve(&profile, axis_y(), Revolution::Full, Tol::witness())
        .unwrap()
        .body
}

fn axis_y() -> RevolveAxis<f64> {
    RevolveAxis {
        origin: p2(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    }
}

/// Two distinct keys of one kind on `body`: the first two faces of that
/// kind, one re-homed onto a fresh copy of its description when the
/// constructor put them all on one key.
fn two_keys_of(body: &mut Body<f64>, kind: SurfaceKind) -> (topo::SurfaceKey, topo::SurfaceKey) {
    let faces: Vec<_> = body
        .faces()
        .filter(|(_, f)| body.get_surface(f.surface).map(SurfaceKind::of) == Some(kind))
        .map(|(k, _)| k)
        .collect();
    assert!(faces.len() >= 2, "{kind:?}: need two faces, got {faces:?}");
    let keys: std::collections::BTreeSet<_> = faces
        .iter()
        .map(|&f| body.get_face(f).unwrap().surface)
        .collect();
    if keys.len() >= 2 {
        let mut it = keys.into_iter();
        return (it.next().unwrap(), it.next().unwrap());
    }
    distinct_keys(body, faces[0], faces[1])
}

/// Row 1⁶ (public door): the record carries the pair's OWN kind and
/// the text names it — a sphere pair and a torus pair. (After the R1
/// review probe.)
#[test]
fn sphere_and_torus_pairs_record_their_kind() {
    for (label, mut body, kind, name) in [
        ("ball", ball(0.7), SurfaceKind::Sphere, "sphere"),
        ("donut", donut(), SurfaceKind::Torus, "torus"),
    ] {
        assert_eq!(validate_closed(&body), Ok(()), "{label}: tier 2");
        let (k1, k2) = two_keys_of(&mut body, kind);
        let before = format!("{body:?}");
        let outcome = body
            .merge_coplanar_faces_declared(&[(k1, k2)], Tol::witness())
            .unwrap_or_else(|e| panic!("{label}: {e:?}"));
        let records = carrier_records(&outcome);
        assert_eq!(records.len(), 1, "{label}: {:?}", outcome.skipped);
        assert_eq!(
            records[0].reason,
            MergeCoplanarError::DeclaredCarrierUnsupported {
                pair: (k1, k2),
                kind
            }
        );
        let text = records[0].reason.to_string();
        assert!(
            text.contains(&format!("lies on a {name} carrier"))
                && text.contains(&format!("no {name} arm")),
            "{label}: {text}"
        );
        assert_eq!(sorted(records[0].faces.clone()), faces_on(&body, k1, k2));
        assert_eq!(format!("{body:?}"), before, "{label}: body untouched");
    }
}

/// Row 2 (A, B) — RED-THE-DAY: freed of the false `InvalidDeclaration`,
/// the floating and mid-bore pegs stop at the zip's own defect — a
/// `Line` chord minted on the bore wall where a cap rim cuts it
/// mid-height, reached by the output stage's seam-edge re-description
/// and refused by its certification, reported as `JoinDesync` (the
/// certification payload is dropped there). A typed refusal of a real
/// defect where a false one stood. This row flips when
/// `work/curved/rest-zip-seam-chord-on-cylinder-wall` lands; the
/// honest outcome then is the one rows 1 and 3 pin.
#[test]
fn floating_and_mid_bore_pegs_refuse_at_the_zip_seam_chord_today() {
    for (label, (a, b, d)) in [("A", scene_a()), ("B", scene_b())] {
        let err = topo::union_with(&a, &b, &d, Tol::witness())
            .err()
            .unwrap_or_else(|| panic!("{label}: the zip's seam chord is fixed — re-pin this row"));
        assert!(
            matches!(
                err,
                BooleanError::JoinDesync {
                    what: "minted-edge description failed certification"
                }
            ),
            "{label}: {err:?}"
        );
    }
}

/// Row 3 (C, D): one side of the declared pair is CONSUMED by the zip
/// — its surface is gone from the shipped body (held at the door only
/// by an edge curve's reference) — and the door still records the pair
/// once, its `faces` the other side's live remainder. D's planar pair
/// never reaches the door (the zip consumes its faces), so the nine
/// wall pairs are all it is handed, in whichever order the caller
/// lists them.
#[test]
fn consumed_side_of_the_pair_is_gone_and_one_record_ships() {
    let (c, p, d) = scene_c();
    let bb = union_honest("C", &c, &p, &d);
    let records = assert_cylinder_records("C", &bb);
    let MergeCoplanarError::DeclaredCarrierUnsupported { pair, .. } = &records[0].reason else {
        unreachable!()
    };
    let pair = *pair;
    assert!(
        bb.body.get_surface(pair.0).is_none() && bb.body.get_surface(pair.1).is_some(),
        "C: the bore side of the pair is consumed, the peg side lives: {pair:?}"
    );

    let (p, q, d) = scene_d();
    let bb = union_honest("D", &p, &q, &d);
    let records = assert_cylinder_records("D", &bb);
    let MergeCoplanarError::DeclaredCarrierUnsupported { pair, .. } = &records[0].reason else {
        unreachable!()
    };
    let pair = *pair;
    assert!(
        bb.body.get_surface(pair.0).is_none() && bb.body.get_surface(pair.1).is_some(),
        "D: the peg side of the pair is consumed, the bore side lives: {pair:?}"
    );
    let mut reversed = BooleanDeclarations::none();
    reversed.coincident_faces = d.coincident_faces.iter().rev().cloned().collect();
    let rb = union_honest("D (planar pair last)", &p, &q, &reversed);
    let reversed_records = assert_cylinder_records("D (planar pair last)", &rb);
    assert_eq!(
        reversed_records[0].reason, records[0].reason,
        "D: the record is order-independent"
    );
    assert_eq!(reversed_records[0].faces, records[0].faces);
}

/// Row 3′ (C): the declined pairs' records LEAD the group records —
/// the declaration's answer before the surgery's.
#[test]
fn declined_records_lead_the_group_records() {
    let (c, p, d) = scene_c();
    let bb = union_honest("C", &c, &p, &d);
    let skipped = &bb.naming.merge_skipped;
    let first_group = skipped
        .iter()
        .position(|s| matches!(s.reason, MergeCoplanarError::PeriodClosure { .. }))
        .expect("C: the three-arc sectors' full-period runs are recorded");
    let last_declined = skipped
        .iter()
        .rposition(|s| {
            matches!(
                s.reason,
                MergeCoplanarError::DeclaredCarrierUnsupported { .. }
            )
        })
        .expect("C: the declared pair is recorded");
    assert!(last_declined < first_group, "{skipped:?}");
}

/// A peg body's planar cap key and one cylinder wall key.
fn plane_and_cylinder_keys(body: &Body<f64>) -> (topo::SurfaceKey, topo::SurfaceKey) {
    let plane = body.get_face(plane_face(body, 0.0, false)).unwrap().surface;
    let cyl = body.get_face(walls_at(body, BORE_R)[0]).unwrap().surface;
    (plane, cyl)
}

/// Row 4: a pair of two KINDS is a torn argument and keeps refusing at
/// the public door, typed, naming the SECOND key (the one whose kind
/// disagrees with the first's), with the body untouched — never
/// swallowed as a carrier record; and a torn pair anywhere in a longer
/// list refuses the whole call.
#[test]
fn mixed_kind_declared_pair_still_refuses_typed() {
    let mut body = peg_at(0.0, 0.0, 1.0);
    let (plane, cyl) = plane_and_cylinder_keys(&body);
    let before = format!("{body:?}");
    for (k1, k2) in [(plane, cyl), (cyl, plane)] {
        let err = body
            .merge_coplanar_faces_declared(&[(k1, k2)], Tol::witness())
            .expect_err("a plane/cylinder pair refuses");
        assert!(
            matches!(
                err,
                MergeCoplanarError::InvalidDeclaration {
                    surface,
                    what: "declared surfaces are not one kind",
                } if surface == k2
            ),
            "({k1:?}, {k2:?}): {err:?}"
        );
        assert_eq!(format!("{body:?}"), before, "body untouched on refusal");
    }
    let caps = (
        plane,
        body.get_face(plane_face(&body, 1.0, true)).unwrap().surface,
    );
    for list in [
        vec![(plane, cyl), caps, (cyl, cyl)],
        vec![caps, (plane, cyl), (cyl, cyl)],
        vec![caps, (cyl, cyl), (plane, cyl)],
    ] {
        let err = body
            .merge_coplanar_faces_declared(&list, Tol::witness())
            .expect_err("a torn pair anywhere in the list refuses");
        assert!(
            matches!(
                err,
                MergeCoplanarError::InvalidDeclaration {
                    surface,
                    what: "declared surfaces are not one kind",
                } if surface == cyl
            ),
            "{list:?}: {err:?}"
        );
        assert_eq!(format!("{body:?}"), before, "body untouched on refusal");
    }
}

/// Row 5: a key that does not resolve refuses BEFORE any kind is
/// read, whichever kind its partner has.
#[test]
fn unresolved_declared_key_still_refuses() {
    let mut body = peg_at(0.0, 0.0, 1.0);
    let (plane, cyl) = plane_and_cylinder_keys(&body);
    // The collar has more surfaces than the peg, so its highest key
    // names a slot the peg's arena does not have.
    let dangling = collar().surfaces().map(|(k, _)| k).max().unwrap();
    assert!(
        body.get_surface(dangling).is_none(),
        "{dangling:?} must not resolve in the peg"
    );
    let before = format!("{body:?}");
    for pair in [
        (dangling, plane),
        (plane, dangling),
        (dangling, cyl),
        (cyl, dangling),
    ] {
        let err = body
            .merge_coplanar_faces_declared(&[pair], Tol::witness())
            .expect_err("an unresolved key refuses");
        assert!(
            matches!(
                err,
                MergeCoplanarError::InvalidDeclaration {
                    surface,
                    what: "declared surface key does not resolve",
                } if surface == dangling
            ),
            "{pair:?}: {err:?}"
        );
        assert_eq!(format!("{body:?}"), before, "body untouched on refusal");
    }
}

/// Row 6 (C): the record's text names the DOOR's missing arm, not the
/// declaration — the declaration was legal and served the op.
#[test]
fn rendered_skip_names_the_door_not_the_declaration() {
    let (c, p, d) = scene_c();
    let bb = union_honest("C", &c, &p, &d);
    let skip = assert_cylinder_records("C", &bb)[0];
    let MergeCoplanarError::DeclaredCarrierUnsupported { pair: (k1, k2), .. } = &skip.reason else {
        panic!("{:?}", skip.reason)
    };
    let text = skip.reason.to_string();
    assert_eq!(
        text,
        format!(
            "merge_coplanar_faces: declared pair ({k1:?}, {k2:?}) lies on a cylinder carrier — \
             the declaration is legal and served the op, but this door's declared-pair rung is \
             planar and has no cylinder arm; the pair is left unmerged and recorded"
        )
    );
    assert!(text.contains("declaration is legal"), "{text}");
    assert!(text.contains("no cylinder arm"), "{text}");
    assert!(!text.to_lowercase().contains("invalid"), "{text}");
}

/// Row 7 (E, F): two equal pegs stacked end to end. With only the
/// caps declared (E) the reduction refuses `CurvedPierceUnsupported`.
/// With every wall pair declared too (F) the mate REACHES this door:
/// the union is exact and honest, the nine wall pairs are recorded as
/// one cylinder pair, and six wall faces survive — all of one sense,
/// each lower sector meeting its upper across the z = 1 rim (three
/// CROSS-key shared edges, beside the seam edges each ring's sectors
/// already share). That is the curved declared rung's consumer (shape
/// 2), at the door today; the arm that would glue it is a successor's,
/// and this record is what shows the pair waiting for it.
#[test]
fn stacked_equal_pegs_same_sense_walls() {
    let lo = peg_at(0.0, 0.0, 1.0);
    let hi = peg_at(0.0, 1.0, 1.0);
    let mut caps = BooleanDeclarations::none();
    caps.coincident_faces.push(FacePairDeclaration::new(
        plane_face(&lo, 1.0, true),
        plane_face(&hi, 1.0, false),
        ContactClass::Rest,
    ));
    let err = topo::union_with(&lo, &hi, &caps, Tol::witness())
        .err()
        .unwrap_or_else(|| panic!("E: the caps-only stacked pegs reach a new door — re-pin"));
    assert!(
        matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
        "E caps only: {err:?}"
    );

    let mut both = caps.clone();
    for &fa in &walls_at(&lo, BORE_R) {
        for &fb in &walls_at(&hi, BORE_R) {
            both.coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    let bb = union_honest("F", &lo, &hi, &both);
    let v = volume(&bb.body);
    assert!(
        (v - core::f64::consts::FRAC_PI_2).abs()
            <= 4.0 * f64::EPSILON * core::f64::consts::FRAC_PI_2,
        "F: two unit-height r = 1/2 pegs: {v}"
    );
    assert_cylinder_records("F", &bb);
    let faces = bore_radius_faces(&bb.body);
    assert_eq!(
        faces.len(),
        6,
        "F: both walls survive, split at z = 1: {faces:?}"
    );
    assert!(
        faces.iter().all(|&(_, s)| s == faces[0].1),
        "F: one sense throughout — a continuation, not a slit: {faces:?}"
    );
    let keys: Vec<_> = faces.iter().map(|&(f, _)| f).collect();
    let cross: Vec<_> = shared_edges(&bb.body, &keys)
        .into_iter()
        .filter(|&(_, f1, f2)| {
            bb.body.get_face(f1).unwrap().surface != bb.body.get_face(f2).unwrap().surface
        })
        .collect();
    assert_eq!(
        cross.len(),
        3,
        "F: each lower sector meets its upper across the z = 1 rim: {cross:?}"
    );
}
