//! The planar merge door and a declared pair on a CYLINDER carrier.
//!
//! A declared `Rest` pair whose faces both survive a boolean is handed
//! to `merge_coplanar_faces_declared` whatever its carrier. The door's
//! declared-pair rung is planar; a cylindrical pair is a legal
//! declaration it has no rung for, and the door RECORDS it as a
//! `SkippedMerge` carrying `DeclaredCarrierUnsupported` — visible in
//! `BooleanNaming::merge_skipped`, never an `InvalidDeclaration` refusal
//! blaming the caller. Four peg-in-bore scenes reach that record; two
//! stacked-peg scenes stop upstream and are reported as the door after.
//!
//! Scenes A–D are `mate2_common`'s; scene D's plate/peg builders are
//! copied from `r1_probes_m9_3` (private there).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common;
use geom_core::{Affine3, Point2, Tol, Vec3};
use mate2_common::{
    assert_additive, body_of, boolean_body, collar, collar_at, peg_at, plane_face, volume,
    wall_decls, walls_at,
};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{
    Body, BooleanBody, BooleanDeclarations, BooleanError, ContactClass, FacePairDeclaration,
    MergeCoplanarError, validate_closed, validate_geometric, validate_pseudomanifold,
};

const BORE_R: f64 = 0.5;

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// `r1_probes_m9_3::plate6`, copied: a 6×4 plate, z ∈ [z0, z0 + 1].
fn plate6(z0: f64) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(0.0, 0.0), p2(6.0, 0.0), p2(6.0, 4.0), p2(0.0, 4.0)]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body
}

/// `r1_probes_m9_3::cyl_at`, copied: a three-arc cylinder centred at
/// `(cx, 2)`, z ∈ [z0, z0 + h].
fn cyl_at(cx: f64, z0: f64, h: f64, r: f64) -> Body<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th = deg.to_radians();
        p2(cx + r * th.cos(), 2.0 + r * th.sin())
    };
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(at(0.0), b120),
        ProfileVertex::new(at(120.0), b120),
        ProfileVertex::new(at(240.0), b120),
    ]);
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, vec![lp])
        .validate(Tol::witness())
        .unwrap();
    extrude(&profile, Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
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
    let p =
        body_of(topo::union(&plate6(0.0), &cyl_at(2.0, 0.4, 1.1, BORE_R), Tol::witness()).unwrap());
    let q = body_of(
        topo::subtract(&plate6(1.0), &cyl_at(2.0, 0.8, 1.4, BORE_R), Tol::witness()).unwrap(),
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

/// Every record's reason is the cylinder-carrier skip, every recorded
/// face is live and on a cylinder, and no cylinder face was glued.
fn assert_cylinder_records(label: &str, bb: &BooleanBody<f64>) {
    for skip in &bb.naming.merge_skipped {
        assert!(
            matches!(
                skip.reason,
                MergeCoplanarError::DeclaredCarrierUnsupported {
                    kind: geom_brep::SurfaceKind::Cylinder,
                    ..
                }
            ),
            "{label}: the record carries the cylinder-carrier reason, got {:?}",
            skip.reason
        );
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
    for (kept, absorbed) in &bb.naming.merge_groups {
        for f in core::iter::once(kept).chain(absorbed) {
            assert!(
                !is_cylinder(&bb.body, *f),
                "{label}: no cylinder face is glued, but {f:?} is in a merge group"
            );
        }
    }
}

/// The surviving r = 0.5 faces: `(face, sense)` in face-arena order.
fn bore_radius_faces(body: &Body<f64>) -> Vec<(topo::FaceKey, bool)> {
    walls_at(body, BORE_R)
        .into_iter()
        .map(|f| (f, body.get_face(f).unwrap().sense))
        .collect()
}

/// Whether any edge is shared by two distinct faces of `set`.
fn share_an_edge(body: &Body<f64>, set: &[topo::FaceKey]) -> bool {
    body.edges().any(|(_, e)| {
        let face_of = |he| {
            body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
                .unwrap()
                .face
        };
        let (f1, f2) = (face_of(e.he_plus), face_of(e.he_minus));
        f1 != f2 && set.contains(&f1) && set.contains(&f2)
    })
}

/// Row 1 (A): the door runs, the body is honest, and the cylindrical
/// declared pairs are recorded — not refused, not dropped.
#[test]
fn floating_peg_declared_walls_union_records_the_cylinder_skip() {
    let (c, p, d) = scene_a();
    let bb = union_honest("A", &c, &p, &d);
    assert!(
        !bb.naming.merge_skipped.is_empty(),
        "A: the cylindrical declared pairs are recorded, never silently dropped"
    );
    assert_cylinder_records("A", &bb);
}

/// Row 2 (A, B): what survives on the bore carrier is a SLIT — the bore
/// wall's remainder (sense in) against the peg wall's remainder (sense
/// out), sharing no edge — so no rung, planar or curved, has a merge
/// candidate here. A curved declared rung must never glue this pair.
#[test]
fn opened_door_leaves_a_slit_not_a_merge_candidate() {
    for (label, (a, b, d)) in [("A", scene_a()), ("B", scene_b())] {
        let bb = union_honest(label, &a, &b, &d);
        let faces = bore_radius_faces(&bb.body);
        assert!(
            faces.iter().any(|&(_, s)| s) && faces.iter().any(|&(_, s)| !s),
            "{label}: both senses survive on the bore carrier: {faces:?}"
        );
        let keys: Vec<_> = faces.iter().map(|&(f, _)| f).collect();
        assert!(
            !share_an_edge(&bb.body, &keys),
            "{label}: the surviving bore-carrier faces share no edge: {faces:?}"
        );
        assert!(
            bb.naming.merge_groups.iter().all(|(kept, absorbed)| {
                !keys.contains(kept) && absorbed.iter().all(|f| !keys.contains(f))
            }),
            "{label}: the slit is not a merge candidate"
        );
    }
}

/// Scene C: the bore is fully consumed, so its surfaces are orphaned
/// and the pairs drop before the door — measured by row 0.
const C_RECORDS: usize = 0;
/// Scene D: the peg wall is fully consumed and the bore remainder
/// survives alone — measured by row 0.
const D_RECORDS: usize = 0;

/// Row 3 (C, D): the door classifies EACH pair by its own kind — the
/// planar pair unions through the equivalence while the cylindrical
/// ones record (or drop upstream once a side's surface is orphaned) —
/// and both outcomes are visible on the result.
#[test]
fn partial_engagement_records_beside_the_planar_pair() {
    let (c, p, d) = scene_c();
    let bb = union_honest("C", &c, &p, &d);
    assert_cylinder_records("C", &bb);
    assert_eq!(
        bb.naming.merge_skipped.len(),
        C_RECORDS,
        "C: {:?}",
        bb.naming.merge_skipped
    );

    let (p, q, d) = scene_d();
    let bb = union_honest("D", &p, &q, &d);
    assert_cylinder_records("D", &bb);
    assert_eq!(
        bb.naming.merge_skipped.len(),
        D_RECORDS,
        "D: {:?}",
        bb.naming.merge_skipped
    );
    // The same declarations with the planar pair LAST: each pair is
    // classed by its own kind, so the order the caller lists them in
    // changes nothing the result shows.
    let mut reversed = BooleanDeclarations::none();
    reversed.coincident_faces = d.coincident_faces.iter().rev().cloned().collect();
    let rb = union_honest("D (planar pair last)", &p, &q, &reversed);
    assert_cylinder_records("D (planar pair last)", &rb);
    assert_eq!(rb.naming.merge_skipped.len(), D_RECORDS);
    assert_eq!(
        rb.naming.merge_groups, bb.naming.merge_groups,
        "D: glue is order-independent"
    );
    // The planar declared pair is not among the records under any
    // count: every record is a cylinder record (asserted above), and
    // the mating faces at z = 1 are gone from the result — consumed by
    // the zip — so nothing planar was left for the door to decline.
    assert!(
        bb.body.faces().all(|(_, f)| !matches!(
            bb.body.get_surface(f.surface),
            Some(geom::Surface::Plane { origin, .. }) if (origin.z - 1.0).abs() < 1e-12
        )),
        "D: no z = 1 face survives the declared planar zip"
    );
}

/// A peg body's planar cap key and one cylinder wall key.
fn plane_and_cylinder_keys(body: &Body<f64>) -> (topo::SurfaceKey, topo::SurfaceKey) {
    let plane = body.get_face(plane_face(body, 0.0, false)).unwrap().surface;
    let cyl = body.get_face(walls_at(body, BORE_R)[0]).unwrap().surface;
    (plane, cyl)
}

/// Row 4: a pair of two KINDS is a torn argument and keeps refusing at
/// the public door, typed, with the body untouched — never swallowed
/// as a carrier record.
#[test]
fn mixed_kind_declared_pair_still_refuses_typed() {
    let mut body = peg_at(0.0, 0.0, 1.0);
    let (plane, cyl) = plane_and_cylinder_keys(&body);
    let before = format!("{body:?}");
    for pair in [(plane, cyl), (cyl, plane)] {
        let err = body
            .merge_coplanar_faces_declared(&[pair], Tol::witness())
            .expect_err("a plane/cylinder pair refuses");
        assert!(
            matches!(
                err,
                MergeCoplanarError::InvalidDeclaration {
                    what: "declared surfaces are not one kind",
                    ..
                }
            ),
            "{err:?}"
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

/// Row 6 (A): the record's text names the DOOR's missing arm, not the
/// declaration — the declaration was legal and served the op.
#[test]
fn rendered_skip_names_the_door_not_the_declaration() {
    let (c, p, d) = scene_a();
    let bb = union_honest("A", &c, &p, &d);
    let skip = bb
        .naming
        .merge_skipped
        .first()
        .expect("A records at least one cylinder pair");
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

/// Row 7 (E, F): two equal pegs stacked end to end — the one
/// configuration whose same-sense cosurface walls would WANT a curved
/// merge — stop at the reduction (`CurvedPierceUnsupported`), with and
/// without the walls declared, and never reach this door. This row
/// turns red when `work/curved/cosurface-disjoint-curved-walls-refuse`
/// lands; the merge door's curved declared rung is the door after,
/// and `DeclaredCarrierUnsupported { kind: Cylinder }` with a shared
/// edge is what will show the pair arriving there.
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
    let mut both = caps.clone();
    for &fa in &walls_at(&lo, BORE_R) {
        for &fb in &walls_at(&hi, BORE_R) {
            both.coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    for (label, d) in [("E caps only", &caps), ("F caps + walls", &both)] {
        let err = topo::union_with(&lo, &hi, d, Tol::witness())
            .err()
            .unwrap_or_else(|| panic!("{label}: the stacked pegs reach a new door — re-scope"));
        eprintln!("{label}: refused {err:?}");
        assert!(
            matches!(err, BooleanError::CurvedPierceUnsupported { .. }),
            "{label}: stops at the reduction today: {err:?}"
        );
    }
}
