//! **The flush detector at the body seat** (`topo::flush`;
//! VERB-SEAT-DESIGN §1 S3, SELECT-DESIGN §3): what a caller holding
//! two bodies and no document can see, and what it can hand to a
//! declared boolean.
//!
//! The rows are the protocol's own claims. A definite flush pair is
//! FOUND; a definitely-apart pair is ABSENT; an in-band pair is
//! neither — it refuses, naming the pair and the verifier's own funnel
//! site, because a finding is only ever definite. `declare_all` turns
//! inspected findings into the `BooleanDeclarations` the op door
//! takes, which is the producer `BooleanDeclarations` did not have.
//! And the order findings arrive in is arena order, pinned here
//! because callers index into it.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::prism_z;
use geom_core::Tol;
use topo::flush::{FlushRefusal, FlushRung, declare, declare_all, find_flush_candidates};
use topo::{
    Body, BooleanResult, ContactClass, FaceKey, PlaneRelation, mass_properties, query, union_with,
};

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    prism_z::<f64>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// A flush stack: two bricks meeting on z = 1, independently authored
/// (so no shared source — the geometric rung decides).
fn stacked() -> (Body<f64>, Body<f64>) {
    (
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        brick((0.5, 1.5), (0.25, 1.25), (1.0, 2.0)),
    )
}

/// The face of `body` whose plane is `z = at` — the pair the stack's
/// contact is about, named independently of the detector so the row
/// below is a claim about WHICH pair, not just how many.
fn cap_at(body: &Body<f64>, at: f64) -> FaceKey {
    let hits: Vec<FaceKey> = query::all_faces(body)
        .into_iter()
        .filter(|&f| match body.get_face(f).and_then(|f| body.get_surface(f.surface)) {
            Some(topo::Surface::Plane { origin, normal, .. }) => {
                normal.x.abs() < 0.5 && normal.y.abs() < 0.5 && (origin.z - at).abs() < 1e-12
            }
            _ => false,
        })
        .collect();
    let [f] = hits[..] else {
        panic!("expected exactly one z = {at} cap, got {hits:?}");
    };
    f
}

// ------------------------------------------------------------------
// 1. A definite flush pair is FOUND, with the verifier's own verdict.
// ------------------------------------------------------------------

#[test]
fn the_stacks_shared_cap_is_one_same_opposite_finding() {
    let (a, b) = stacked();
    let found = find_flush_candidates(&a, &b, Tol::witness()).expect("the stack decides definitely");
    assert_eq!(
        found.len(),
        1,
        "one contact, one finding — the stack has exactly one coincident plane pair: {found:?}"
    );
    let finding = &found[0];
    assert_eq!(finding.pair, (cap_at(&a, 1.0), cap_at(&b, 1.0)));
    assert_eq!(finding.class, ContactClass::Rest);
    assert_eq!(
        finding.evidence.relation,
        PlaneRelation::SameOpposite,
        "resting contact: the two outward normals oppose"
    );
    assert_eq!(
        finding.evidence.rung,
        FlushRung::DecidedCoincident,
        "independently authored bricks share no source, so the geometric rung decides"
    );
}

// ------------------------------------------------------------------
// 2. A definitely-apart pair is ABSENT.
// ------------------------------------------------------------------

#[test]
fn a_separated_stack_has_no_findings() {
    let (a, _) = stacked();
    let far = brick((0.5, 1.5), (0.25, 1.25), (2.0, 3.0));
    let found = find_flush_candidates(&a, &far, Tol::witness()).expect("a clear gap decides");
    assert!(
        found.is_empty(),
        "definitely-apart parallel caps are no finding at all: {found:?}"
    );
}

// ------------------------------------------------------------------
// 3. An in-band pair refuses, naming the pair.
// ------------------------------------------------------------------

/// A gap strictly inside the ambiguity band: neither reported nor
/// dropped. The refusal names the two FACE KEYS — the body seat's own
/// vocabulary — and carries the VERIFIER's funnel site, because
/// detection mints none of its own.
#[test]
fn an_in_band_gap_refuses_naming_the_pair() {
    let tol = Tol::witness();
    let raw = tol.get();
    let gap = 0.5 * (raw.eps + raw.k * raw.eps);
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick((0.5, 1.5), (0.25, 1.25), (1.0 + gap, 2.0));
    match find_flush_candidates(&a, &b, tol) {
        Err(FlushRefusal::PairInBand { pair, source }) => {
            assert_eq!(pair, (cap_at(&a, 1.0), cap_at(&b, 1.0 + gap)));
            assert_eq!(source.predicate, Some("bool_plane_offset"));
        }
        other => panic!("expected PairInBand, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// 4. The declared round trip: findings → declarations → a union that
//    builds.
// ------------------------------------------------------------------

/// What the ~55-line hand declarers existed to do, done by the two
/// library doors: detect, INSPECT (the findings are values in the
/// caller's hands — GS-Q3's no-fusion boundary), declare, union. The
/// undeclared call refuses, so the declaration is what unlocks the
/// lane rather than a measurement standing in for it.
#[test]
fn declare_all_round_trips_into_a_union_that_builds() {
    let (a, b) = stacked();
    topo::union(&a, &b, Tol::witness()).expect_err("undeclared, the kiss refuses");

    let found = find_flush_candidates(&a, &b, Tol::witness()).expect("the stack decides");
    let decls = declare_all(&found);
    assert_eq!(decls.coincident_faces.len(), found.len());
    assert!(
        decls
            .coincident_faces
            .iter()
            .all(|d| d.class == ContactClass::Rest),
        "the finding's class travels into the declaration"
    );
    let BooleanResult::Body(built) =
        union_with(&a, &b, &decls, Tol::witness()).expect("the declared union builds")
    else {
        panic!("an overlapping union cannot be Empty");
    };
    let volume = mass_properties(&built.body, Tol::witness()).unwrap().volume;
    assert!(
        (volume - 2.0).abs() < 1e-9,
        "two unit bricks meeting on a face, so no material is shared: {volume}"
    );
}

/// The one-finding arm of the ruled both-arities boundary declares
/// exactly its own pair.
#[test]
fn declare_declares_exactly_one_finding() {
    let (a, b) = stacked();
    let found = find_flush_candidates(&a, &b, Tol::witness()).expect("the stack decides");
    let one = declare(&found[0]);
    assert_eq!(one.coincident_faces.len(), 1);
    assert_eq!(one.coincident_faces[0].a, found[0].pair.0);
    assert_eq!(one.coincident_faces[0].b, found[0].pair.1);
    assert!(declare_all(&[]).is_empty(), "nothing found declares nothing");
}

// ------------------------------------------------------------------
// 5. The order is arena order, and it is stable.
// ------------------------------------------------------------------

/// Findings arrive in `a`'s arena order major, `b`'s minor — the order
/// the enumeration walks (D9: slot-index order is deterministic given
/// identical construction history). Pinned because a caller indexes
/// into the vector it gets back.
#[test]
fn findings_arrive_in_arena_order_and_repeat() {
    // A shared wall AND a shared cap, so there is more than one
    // finding to order: `b` sits on top of `a` and their x = 1 walls
    // are flush.
    let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick((0.5, 1.0), (0.25, 0.75), (1.0, 2.0));
    let found = find_flush_candidates(&a, &b, Tol::witness()).expect("the pair decides");
    assert!(
        found.len() >= 2,
        "the fixture must exercise ORDER, so it needs at least two findings: {found:?}"
    );
    let (fa, fb) = (query::all_faces(&a), query::all_faces(&b));
    let expected: Vec<(FaceKey, FaceKey)> = fa
        .iter()
        .flat_map(|&ka| fb.iter().map(move |&kb| (ka, kb)))
        .filter(|pair| found.iter().any(|f| f.pair == *pair))
        .collect();
    let actual: Vec<(FaceKey, FaceKey)> = found.iter().map(|f| f.pair).collect();
    assert_eq!(actual, expected, "arena order, a major and b minor");

    let again = find_flush_candidates(&a, &b, Tol::witness()).expect("the rerun decides");
    assert_eq!(found, again, "same bodies, same findings, same order");
}
