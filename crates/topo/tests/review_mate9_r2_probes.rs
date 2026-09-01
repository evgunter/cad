//! **R2 review probes for MATE-9** (the `EdgeEdgeCross` backing rung,
//! PR #1496 at `b873d783`). Adversarial rows only — nothing here is a
//! blessing of the unit's behaviour; each row DOCUMENTS what the rung
//! does today so the fix pass can decide whether that is what it
//! should do.
//!
//! Three probes:
//!
//! 1. **The dropped precondition.** `classify_material_pairing`'s own
//!    doc says it answers "at an on-locus point where the tangent
//!    planes already classified [`DihedralClass::Smooth`]", and both
//!    existing consumers (`validate.rs` tier-3 check 4, gated on
//!    `all_smooth`; `boolean/rim_wedge.rs`, which comments "the
//!    material arm, now validly posed") establish that first. The
//!    crossing rung does NOT: it hands the algebra any declared pair
//!    that merely holds the crossing point. Probe 1 shows the pair the
//!    unit's own undecided row uses is `Transverse`, not `Smooth`, and
//!    that the resulting `Indeterminate` carries `MarginDiag::Invalid`
//!    — "the question was never validly posed here" — not an in-band
//!    residue. `CrossingSideVerdict::Undecided`'s doc ("could not
//!    decide at this ε") therefore misdescribes its only in-suite
//!    instance.
//!
//! 2. **The escalation a losing candidate leaves behind.** The rung
//!    pushes `CensusEscalated` from inside its candidate loop and
//!    never retracts it, so a crossing that a LATER pair backs can
//!    still carry a hard escalation raised by an earlier candidate.
//!    Probe 2 pins that the census's error list depends on a
//!    declaration that answers for nothing.
//!
//! 3. **`pair_region_verified` is unguarded.** Bypassing the entire
//!    verified-interface half of the rung (Door 1
//!    `contact_pair_verdict(Rest)` + Door 2 `declared_overlap`, MATE-8's
//!    witness rung included) leaves the whole topo suite AND the viewer
//!    windmill story green. Probe 3 is the row that would go red: an
//!    OPPOSED, point-holding, but UNVERIFIED declared pair must not
//!    back a crossing. It passes at this head — it is a coverage row,
//!    filed because nothing else in the tree is.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use geom::Surface;
use geom_core::{Band, Point3, Tol, Vec3};
use topo::{Body, CensusContact, ContactRecords, FaceKey, PatchContact, ValidationError};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The straddle seat of the MATE-9 suite, verbatim: a cap
/// `[0.30, 0.60] x [0.20, 0.42]` under a shelf `[0, 0.9] x [0, 0.30]`,
/// contact plane `z = 0.5`, the cap's side edges crossing the shelf's
/// `y = 0.30` boundary at `(0.30, 0.30)` and `(0.60, 0.30)`.
fn straddle_parts() -> (Body<f64>, FaceKey, FaceKey, FaceKey, FaceKey) {
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        0.5,
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let post_side_x030 = post.side_faces[3];
    let mut body = post.body;
    let keys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_bottom = keys.face(shelf.bottom_face).unwrap();
    let shelf_side_y030 = keys.face(shelf.side_faces[2]).unwrap();
    (
        body,
        post.top_face,
        post_side_x030,
        shelf_bottom,
        shelf_side_y030,
    )
}

fn declared(pairs: &[(FaceKey, FaceKey)]) -> ContactRecords {
    ContactRecords {
        patches: pairs
            .iter()
            .map(|&(a, b)| PatchContact {
                face_a: a,
                face_b: b,
            })
            .collect(),
        ..ContactRecords::default()
    }
}

fn errors(body: &Body<f64>, records: &ContactRecords) -> Vec<ValidationError> {
    match topo::validate_pseudomanifold(body, records, Tol::witness()) {
        Ok(()) => Vec::new(),
        Err(e) => e,
    }
}

fn crossings(errors: &[ValidationError]) -> Vec<(CensusContact, String)> {
    errors
        .iter()
        .filter_map(|e| match e {
            ValidationError::UndeclaredContact {
                contact: contact @ CensusContact::EdgeEdgeCross { .. },
                witness,
            } => Some((*contact, witness.clone())),
            _ => None,
        })
        .collect()
}

fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    let u_ref = if normal.x.abs() < 0.5 {
        Vec3::new(1.0, 0.0, 0.0)
    } else {
        Vec3::new(0.0, 1.0, 0.0)
    };
    Surface::Plane {
        origin,
        normal,
        u_ref: normal.cross(u_ref).cross(normal).normalize(),
    }
}

/// **Probe 1 — the imported fold's precondition is dropped.**
///
/// The unit's `a_perpendicular_pair_holding_the_point_escalates_undecided`
/// row reaches the third verdict arm through the post's `x = 0.30`
/// side face against the shelf's `y = 0.30` side face: two
/// PERPENDICULAR planes. Its prose calls the outcome an exact-zero ε
/// margin. But `classify_material_pairing`'s contract is a
/// `DihedralClass::Smooth` site, and this pair classifies
/// `Transverse` — so the algebra is being consulted outside its
/// documented domain, and the `Indeterminate` it returns is
/// `MarginDiag::Invalid` ("the pairing question is not validly posed
/// at this site", per its own `# Errors` section), NOT the in-band
/// residue `CrossingSideVerdict::Undecided`'s doc claims.
#[test]
fn r2_the_undecided_arm_is_a_precondition_violation_not_an_epsilon_residue() {
    let q = Point3::new(0.30, 0.30, 0.5);
    // The post's x = 0.30 side face (outward -x) and the shelf's
    // y = 0.30 side face (outward +y), both through q.
    let s_post = plane(q, Vec3::new(-1.0, 0.0, 0.0));
    let s_shelf = plane(q, Vec3::new(0.0, 1.0, 0.0));
    let extent = 0.22_f64; // the shorter crossing edge, roughly
    let arm = geom_brep::folded_lever_arm(&s_post, &s_shelf, q, extent);

    // (a) The site is NOT smooth — the precondition both existing
    //     consumers establish before calling the pairing algebra.
    let dihedral = geom_brep::classify_dihedral(&s_post, &s_shelf, q, extent, band()).unwrap();
    assert_eq!(
        dihedral,
        geom_brep::DihedralClass::Transverse,
        "the perpendicular declared pair is a TRANSVERSE dihedral site, \
         not the Smooth one classify_material_pairing documents as its \
         domain"
    );

    // (b) And the pairing call's failure is Invalid — the
    //     question-not-posed diagnosis — at every band, not an
    //     in-band ε residue.
    let err = geom_brep::classify_material_pairing(&s_post, 1.0, &s_shelf, 1.0, q, arm, band())
        .expect_err("perpendicular normals decide neither aligned nor opposed");
    assert_eq!(
        err.margin,
        geom_core::MarginDiag::Invalid,
        "the undecided arm's cause is `Invalid` (the question was not \
         validly posed), which is not what `CrossingSideVerdict::\
         Undecided`'s doc — \"could not decide at this ε\" — says: {err:?}"
    );
    // Widening the band does not change it: nothing about this is a
    // tolerance question.
    let wide = Band::new(1e-3, 1e-2).unwrap();
    let err_wide = geom_brep::classify_material_pairing(&s_post, 1.0, &s_shelf, 1.0, q, arm, wide)
        .expect_err("still no verdict at a 1e-3 band");
    assert_eq!(
        err_wide.margin,
        geom_core::MarginDiag::Invalid,
        "ε-independent: {err_wide:?}"
    );
}

/// **Probe 2 — the rung's escalation is not a property of the
/// crossing: it duplicates, and an unrelated backing silences it.**
///
/// The side test pushes `CensusEscalated` from inside the candidate
/// loop and the loop neither de-duplicates nor unwinds. Two
/// consequences, both pinned here on the unit's own perpendicular
/// fixture:
///
/// 1. **Duplication.** `Declared::index` stores BOTH orientations of a
///    face pair, so one declared pair holding one crossing point runs
///    the side test twice and pushes TWO identical escalations for one
///    crossing. The unit's own row asserts only `.any(..)`, so the
///    duplicate is invisible there.
/// 2. **Silencing.** Adding a second, unrelated declaration that backs
///    the crossing makes the loop `return` before the escalating
///    candidate is reached, and the undecided side question vanishes
///    from the output entirely — no escalation, no named verdict.
///    Whether an undecidable side question is reported therefore
///    depends on which pair sorts first in `Declared::faces`, not on
///    the geometry.
#[test]
fn r2_the_side_escalation_duplicates_and_an_unrelated_backing_silences_it() {
    let (body, post_top, post_side_x030, shelf_bottom, shelf_side_y030) = straddle_parts();

    let perp_only = errors(&body, &declared(&[(post_side_x030, shelf_side_y030)]));
    let escalations = perp_only
        .iter()
        .filter(|e| matches!(e, ValidationError::CensusEscalated { .. }))
        .count();
    assert_eq!(
        escalations, 2,
        "one declared pair, one crossing point: the side test runs once \
         per stored orientation and pushes a duplicate escalation — \
         {perp_only:?}"
    );

    // The seat certifies alone through the resting pair.
    let clean = errors(&body, &declared(&[(post_top, shelf_bottom)]));
    assert!(clean.is_empty(), "the seat certifies alone: {clean:?}");

    // Both declarations together: the resting pair sorts first, backs,
    // and the loop returns — the perpendicular pair's undecidable side
    // question is never asked and never reported.
    let both = errors(
        &body,
        &declared(&[(post_top, shelf_bottom), (post_side_x030, shelf_side_y030)]),
    );
    assert!(crossings(&both).is_empty(), "crossings backed: {both:?}");
    assert!(
        !both
            .iter()
            .any(|e| matches!(e, ValidationError::CensusEscalated { .. })),
        "the escalation the same declaration raised on its own is GONE \
         once an unrelated pair backs first: {both:?}"
    );
}

/// **Probe 3 — the coverage row `pair_region_verified` does not have.**
///
/// Replacing the whole `pair_region_verified(..)` call with `true` (the
/// two doors, `contact_pair_verdict(Rest)` + `declared_overlap` with
/// MATE-8's witness rung, deleted) leaves `cargo test -p topo` at
/// 445/445 and the viewer windmill story green — measured at this
/// head. Nothing in the tree goes red when the verified-interface half
/// of the unified strength is removed.
///
/// This is the row that would: an OPPOSED, point-holding, but
/// UNVERIFIED declared pair. The post's top face rests at `z = 0.5`
/// under the shelf; a SECOND shelf sits at `z = 0.5` too but its
/// footprint `[0.62, 0.9] x [0, 0.30]` shares no area with the post
/// `[0.30, 0.60] x [0.20, 0.42]` — the pair is coplanar and opposed in
/// sense, and its region overlap is EMPTY, so Door 2 refuses it. It
/// holds no crossing point either, so the row is a control on the
/// conjunction rather than on Door 2 in isolation; a Door-2-isolating
/// fixture needs a pair whose closed regions both hold the crossing
/// point while their overlap has no positive area, which is the row
/// this unit owes.
#[test]
fn r2_an_unverified_opposed_pair_backs_no_crossing() {
    let (body_base, post_top, _, shelf_bottom, _) = straddle_parts();
    let _ = (post_top, shelf_bottom);
    let post: common::Prism<f64> = common::prism_z(
        &[(0.30, 0.20), (0.60, 0.20), (0.60, 0.42), (0.30, 0.42)],
        0.0,
        0.5,
    );
    let shelf: common::Prism<f64> = common::prism_z(
        &[(0.0, 0.0), (0.9, 0.0), (0.9, 0.30), (0.0, 0.30)],
        0.5,
        0.54,
    );
    let far: common::Prism<f64> = common::prism_z(
        &[(0.62, 0.02), (0.86, 0.02), (0.86, 0.26), (0.62, 0.26)],
        0.2,
        0.5,
    );
    let mut body = post.body;
    let skeys = topo::graft_disjoint_all_keyed(&mut body, &shelf.body, Tol::witness()).unwrap();
    let shelf_b = skeys.face(shelf.bottom_face).unwrap();
    let fkeys = topo::graft_disjoint_all_keyed(&mut body, &far.body, Tol::witness()).unwrap();
    let far_top = fkeys.face(far.top_face).unwrap();
    let _ = body_base;

    let bare = crossings(&errors(&body, &ContactRecords::default()));
    // The far post genuinely rests under the shelf, so declaring it is
    // honest; what it must NOT do is answer for the OTHER post's
    // crossings.
    let with = crossings(&errors(&body, &declared(&[(far_top, shelf_b)])));
    assert_eq!(
        format!("{bare:?}"),
        format!("{with:?}"),
        "a pair that does not hold the crossing points answers for \
         nothing about them"
    );
    assert_eq!(bare.len(), 2, "{bare:?}");
}
