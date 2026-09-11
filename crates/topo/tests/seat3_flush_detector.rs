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

use crate::common;

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

/// The one planar face of `body` whose stored plane satisfies `want`
/// (given `origin.x`, `origin.z`, `normal.x`, `normal.z`) — a
/// POSITIONAL pick, so the rows below can claim WHICH pairs the
/// detector answers with rather than only how many.
///
/// This is the same hand-rolled arena scan the demo scenes carry
/// (`twopeg::plane_face` and its siblings), and it is here for their
/// reason: the flush detector says which face pairs WOULD verify, and
/// nothing on the plain-body API says which one an AUTHOR meant. That
/// selection half is the two-doors gap (#1345) and this detector does
/// not close it. Picking out of `find_flush_candidates`'s own answer
/// would close it here and cost the rows their subject — a test that
/// selects from the answer cannot then assert the answer.
fn plane_face(body: &Body<f64>, want: impl Fn(f64, f64, f64, f64) -> bool) -> FaceKey {
    let hits: Vec<FaceKey> = query::all_faces(body)
        .into_iter()
        .filter(
            |&f| match body.get_face(f).and_then(|f| body.get_surface(f.surface)) {
                Some(topo::Surface::Plane { origin, normal, .. }) => {
                    want(origin.x, origin.z, normal.x, normal.z)
                }
                _ => false,
            },
        )
        .collect();
    let [f] = hits[..] else {
        panic!("expected exactly one matching planar face, got {hits:?}");
    };
    f
}

/// The face whose plane is `z = at`, outward normal along z.
fn cap_at(body: &Body<f64>, at: f64) -> FaceKey {
    plane_face(body, |_, oz, nx, nz| {
        nx.abs() < 0.5 && nz.abs() > 0.5 && (oz - at).abs() < 1e-12
    })
}

/// The face whose plane is `x = at`, outward normal along x.
fn wall_at(body: &Body<f64>, at: f64) -> FaceKey {
    plane_face(body, |ox, _, nx, nz| {
        nz.abs() < 0.5 && nx.abs() > 0.5 && (ox - at).abs() < 1e-12
    })
}

// ------------------------------------------------------------------
// 1. A definite flush pair is FOUND, with the verifier's own verdict.
// ------------------------------------------------------------------

#[test]
fn the_stacks_shared_cap_is_one_same_opposite_finding() {
    let (a, b) = stacked();
    let found =
        find_flush_candidates(&a, &b, Tol::witness()).expect("the stack decides definitely");
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
    assert!(
        declare_all(&[]).is_empty(),
        "nothing found declares nothing"
    );
}

/// **A finding is a report about GEOMETRY, not a promise that the op
/// will run** — the honest boundary of what detection buys, pinned on
/// the arm that shows it.
///
/// The stepped fixture carries a `SameOriented` flush wall pair (the
/// merge-stage flavor) beside its resting cap pair. Declare BOTH — no
/// declaration is contradicted, the verifier agrees each pair is one
/// plane — and the union still refuses, at `RestZipUnsupported`: a
/// named capability frontier of the declared zip, downstream of every
/// verification the declarations pass. Detection cannot see that
/// frontier and does not claim to.
///
/// The subset arm is pinned in the same row because it is the other
/// half of the same lesson: declaring only the wall leaves the cap
/// coincidence undeclared, and the op says so
/// (`UndeclaredCoincidence`) rather than proceeding. A report is a
/// SET, and declaring part of it declares part of it.
#[test]
fn a_declared_same_oriented_finding_can_still_meet_a_typed_lane_frontier() {
    let (a, b) = stepped();
    let found = find_flush_candidates(&a, &b, Tol::witness()).expect("the pair decides");
    let wall = found
        .iter()
        .find(|f| f.evidence.relation == PlaneRelation::SameOriented)
        .expect("the flush wall pair is a finding");

    let partial = union_with(&a, &b, &declare(wall), Tol::witness())
        .expect_err("the cap pair is still undeclared");
    assert!(
        matches!(partial, topo::BooleanError::UndeclaredCoincidence { .. }),
        "declaring one finding of a report declares one finding: {partial:?}"
    );

    let err = union_with(&a, &b, &declare_all(&found), Tol::witness())
        .expect_err("the fully declared union meets the zip's frontier");
    assert!(
        matches!(err, topo::BooleanError::RestZipUnsupported { .. }),
        "a typed lane frontier, NOT a contact contradiction — the declarations are true \
         and the op is what cannot proceed: {err:?}"
    );
}

// ------------------------------------------------------------------
// 5. The order is arena order, and it is stable.
// ------------------------------------------------------------------

/// The order fixture: `b` rests on `a`'s top cap AND their x = 1 walls
/// are flush, so the report holds two findings of different relations.
fn stepped() -> (Body<f64>, Body<f64>) {
    (
        brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)),
        brick((0.5, 1.0), (0.25, 0.75), (1.0, 2.0)),
    )
}

/// Findings arrive in `a`'s arena order major, `b`'s minor — the order
/// the enumeration walks (D9: slot-index order is deterministic given
/// identical construction history). Pinned because a caller indexes
/// into the vector it gets back.
///
/// MEMBERSHIP is pinned first, and separately: the expected ORDER
/// below is derived from the answer (it filters the full arena product
/// by what came back), so on its own it could not tell a right answer
/// from a wrong one delivered in arena order.
#[test]
fn findings_arrive_in_arena_order_and_repeat() {
    let (a, b) = stepped();
    let found = find_flush_candidates(&a, &b, Tol::witness()).expect("the pair decides");
    let members: Vec<(FaceKey, FaceKey)> = found.iter().map(|f| f.pair).collect();
    assert_eq!(
        members,
        vec![
            (cap_at(&a, 1.0), cap_at(&b, 1.0)),
            (wall_at(&a, 1.0), wall_at(&b, 1.0)),
        ],
        "exactly two contacts exist between these bricks — the resting cap pair and the \
         flush x = 1 wall pair — and the report is exactly them: {found:?}"
    );
    assert_eq!(found[0].evidence.relation, PlaneRelation::SameOpposite);
    assert_eq!(found[1].evidence.relation, PlaneRelation::SameOriented);

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
