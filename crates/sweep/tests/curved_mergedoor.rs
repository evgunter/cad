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

/// The declared-carrier records: every one carries the cylinder
/// reason and names the SAME pair (`declared_surface_pairs` lowers
/// each of the nine wall face pairs to the one surface pair the
/// three-arc walls share, so the door is handed nine copies — measured
/// 9 on C, D and F); every recorded face is live and on a cylinder; no
/// cylinder face was glued. Every other record is a curved run's
/// `PeriodClosure` (the three-arc sectors on one key), which predates
/// this door's arm.
fn assert_cylinder_records<'a>(
    label: &str,
    bb: &'a BooleanBody<f64>,
) -> Vec<&'a topo::SkippedMerge> {
    let mut declared = Vec::new();
    for skip in &bb.naming.merge_skipped {
        match skip.reason {
            MergeCoplanarError::DeclaredCarrierUnsupported {
                kind: geom_brep::SurfaceKind::Cylinder,
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
    assert!(
        !declared.is_empty(),
        "{label}: the declared cylinder pairs are recorded"
    );
    let first = &declared[0].reason;
    for skip in &declared {
        assert_eq!(
            &skip.reason, first,
            "{label}: every record names the one declared pair"
        );
        assert_eq!(
            skip.faces, declared[0].faces,
            "{label}: every record names the same faces"
        );
    }
    for (kept, absorbed) in &bb.naming.merge_groups {
        for f in core::iter::once(kept).chain(absorbed) {
            assert!(
                !is_cylinder(&bb.body, *f),
                "{label}: no cylinder face is glued, but {f:?} is in a merge group"
            );
        }
    }
    declared
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

/// Row 1 (C): the door runs, the body is honest, and the cylindrical
/// declared pairs are recorded — not refused, not dropped.
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
            body.set_face_surface(f, topo::FaceSurface::New(described))
                .unwrap(),
        );
    }
    assert_eq!(keys.len(), 3);
    assert_eq!(validate_closed(&body), Ok(()));
    (body, keys)
}

/// Row 1′ (public door): a declined pair is recorded even when the
/// call finds NOTHING else to merge — the early return carries the
/// records, never an empty outcome.
#[test]
fn declined_pair_survives_a_call_with_nothing_to_merge() {
    let (mut body, keys) = peg_with_split_wall_keys();
    let before = format!("{body:?}");
    let outcome = body
        .merge_coplanar_faces_declared(&[(keys[0], keys[1])], Tol::witness())
        .expect("a non-planar declared pair is recorded, never refused");
    assert!(outcome.groups.is_empty(), "{:?}", outcome.groups);
    assert_eq!(outcome.skipped.len(), 1, "{:?}", outcome.skipped);
    let skip = &outcome.skipped[0];
    assert_eq!(
        skip.reason,
        MergeCoplanarError::DeclaredCarrierUnsupported {
            pair: (keys[0], keys[1]),
            kind: geom_brep::SurfaceKind::Cylinder,
        }
    );
    let mut faces: Vec<_> = body
        .faces()
        .filter(|(_, f)| f.surface == keys[0] || f.surface == keys[1])
        .map(|(k, _)| k)
        .collect();
    faces.sort();
    let mut recorded = skip.faces.clone();
    recorded.sort();
    assert_eq!(
        recorded, faces,
        "the record names every live face on either key"
    );
    assert_eq!(
        format!("{body:?}"),
        before,
        "nothing to merge: the body is untouched"
    );
}

/// Row 2 (A, B) — RED-THE-DAY: freed of the false `InvalidDeclaration`,
/// the floating and mid-bore pegs stop at the zip's own defect — a
/// `Line` chord minted on the bore wall where a cap rim cuts it
/// mid-height, refused by `describe_minted_edges`' certification and
/// reported as `JoinDesync` (the certification payload is dropped
/// there). A typed refusal of a real defect where a false one stood.
/// This row flips when `work/curved/rest-zip-seam-chord-on-cylinder-wall`
/// lands; the honest outcome then is the one rows 1 and 3 pin.
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

/// Row 3 (C, D): the door classifies EACH pair by its own kind — the
/// planar pair unions through the equivalence while the cylindrical
/// ones record — and both outcomes are visible on the result. The
/// record's `pair` is what the user declared: on both scenes one side
/// of it was consumed by the zip and its surface is gone from the
/// shipped body (held at the door only by an edge curve's reference);
/// `faces` is the live set, the other side's remainder.
#[test]
fn partial_engagement_records_beside_the_planar_pair() {
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
    // The same declarations with the planar pair LAST: each pair is
    // classed by its own kind, so the order the caller lists them in
    // changes nothing the result shows.
    let mut reversed = BooleanDeclarations::none();
    reversed.coincident_faces = d.coincident_faces.iter().rev().cloned().collect();
    let rb = union_honest("D (planar pair last)", &p, &q, &reversed);
    let reversed_records = assert_cylinder_records("D (planar pair last)", &rb);
    assert_eq!(reversed_records.len(), records.len());
    assert_eq!(
        rb.naming.merge_groups, bb.naming.merge_groups,
        "D: glue is order-independent"
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
/// each lower sector sharing its rim edge with the upper one. That is
/// the curved declared rung's consumer (shape 2), at the door today;
/// the arm that would glue it is a successor's, and this record is
/// what shows the pair waiting for it.
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
    assert!(
        share_an_edge(&bb.body, &keys),
        "F: the lower and upper sectors meet along the z = 1 rim"
    );
}
