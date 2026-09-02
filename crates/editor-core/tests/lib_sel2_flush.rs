//! **LIB-SEL2 — the detect / declare protocol** (`docs/SELECT-DESIGN.md`
//! §3, ratified incl. the round-2 no-fusion amendment).
//!
//! What is pinned, in the design's own order:
//!
//! 1. **The detector is the C4 verifier in candidate-generation mode.**
//!    `find_flush_candidates` reports exactly the pairs the declared
//!    rung will later verify — so detect → declare → boolean SUCCEEDS
//!    on the same evaluation where the undeclared boolean REFUSES.
//!    That round trip is the anti-twin rule as a test: no drift is
//!    possible because there is no twin.
//! 2. **Findings are definite values.** `Rest` class, the verifier's
//!    own relation verdict as evidence (`SameOpposite` = resting
//!    contact, `SameOriented` = flush walls), names never keys.
//! 3. **In-band pairs refuse.** A cap gap inside the ambiguity band is
//!    neither reported nor dropped: `SelectRefusal::PairInBand` names
//!    the pair (§3a / §2's honesty obligation).
//! 4. **The sugar is thin and never detects.** `declare`/`declare_all`
//!    insert a `Node::Declare` from findings the CALLER passes;
//!    an empty set refuses (`NoFindings`) rather than inserting a
//!    pretend-declaration.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    BooleanOp, BooleanValue, CancelToken, ContactClass, DeclareError, EvalOptions, FlushRung, Node,
    NodeErrorKind, NodeResult, ProfileDoc, RecipeNodeId, SelectRefusal, ValuePayload, declare,
    declare_all, evaluate, find_flush_candidates,
};
use topo::{PlaneRelation, mass_properties};

use fixture::{desc, insert, len, on_frame};
use geom_core::Tol;

fn eval(doc: &ProfileDoc) -> editor_core::Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// A box extruded from a square footprint sketched at height `z0`.
fn box_at(
    doc: ProfileDoc,
    z0: f64,
    (x0, y0): (f64, f64),
    (x1, y1): (f64, f64),
    height: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = on_frame(
        doc,
        [0.0, 0.0, z0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(x0, y0), (x1, y0), (x1, y1), (x0, y1)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: p,
            distance: len(height),
        },
    )
}

/// Base box `[0,1]³` and a smaller box resting on its top cap
/// (`z ∈ [1, 1.5]`, footprint `[0.25, 0.75]²`): ONE flush plane —
/// the resting contact at z = 1.
fn stacked() -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, base) = box_at(
        ProfileDoc::empty_derived("lib_sel2_flush", Tol::witness()),
        0.0,
        (0.0, 0.0),
        (1.0, 1.0),
        1.0,
    );
    let (doc, top) = box_at(doc, 1.0, (0.25, 0.25), (0.75, 0.75), 0.5);
    (doc, base, top)
}

// ------------------------------------------------------------------
// 1. Detect: findings are definite values with the verifier's verdict.
// ------------------------------------------------------------------

/// The stacked pair has exactly one flush plane, and the finding
/// carries the REST posture: `SameOpposite` (the two outward normals
/// oppose across the contact), decided by the geometric rungs (the
/// two caps come from different recipe nodes, so no shared source).
#[test]
fn resting_contact_is_one_same_opposite_finding() {
    let (doc, base, top) = stacked();
    let ev = eval(&doc);
    let findings = find_flush_candidates(&ev, base, top, Tol::witness()).unwrap();
    assert_eq!(findings.len(), 1, "{findings:?}");
    let f = &findings[0];
    assert_eq!(f.class, ContactClass::Rest);
    assert_eq!(f.evidence.relation, PlaneRelation::SameOpposite);
    assert_eq!(f.evidence.rung, FlushRung::DecidedCoincident);
    // Names, never keys — and each side names its OWN node's face.
    assert_eq!(f.pair.0.node, base);
    assert_eq!(f.pair.1.node, top);
}

/// Flush WALLS (corner-table shape): a post overlapping a slab with
/// three shared outer wall planes plus the shared floor — every
/// finding is the merge-stage flavor, `SameOriented`.
#[test]
fn flush_walls_are_same_oriented_findings() {
    let (doc, slab) = box_at(
        ProfileDoc::empty_derived("lib_sel2_flush", Tol::witness()),
        0.0,
        (0.0, 0.0),
        (2.0, 1.0),
        0.5,
    );
    let (doc, post) = box_at(doc, 0.0, (0.0, 0.0), (0.5, 1.0), 1.0);
    let ev = eval(&doc);
    let findings = find_flush_candidates(&ev, slab, post, Tol::witness()).unwrap();
    // x = 0, y = 0, y = 1 walls, and the two z = 0 floors.
    assert_eq!(findings.len(), 4, "{findings:?}");
    for f in &findings {
        assert_eq!(f.class, ContactClass::Rest);
        assert_eq!(f.evidence.relation, PlaneRelation::SameOriented, "{f:?}");
    }
}

/// Separated bodies sharing NO plane: no findings, infallibly. (A
/// separated body that DOES share a wall plane is a finding — the v1
/// detector reports coplanarity, and chart-space overlap is
/// deliberately not in the plane-level ladder, exactly as at the
/// verify-at-use site.) And a node with no value in the evaluation
/// answers EMPTY (the `select` posture), not an error.
#[test]
fn separated_and_unevaluated_answer_empty() {
    let (doc, base) = box_at(
        ProfileDoc::empty_derived("lib_sel2_flush", Tol::witness()),
        0.0,
        (0.0, 0.0),
        (1.0, 1.0),
        1.0,
    );
    let (doc, far) = box_at(doc, 3.0, (2.0, 2.0), (3.0, 3.0), 1.0);
    let ev = eval(&doc);
    assert!(
        find_flush_candidates(&ev, base, far, Tol::witness())
            .unwrap()
            .is_empty()
    );
    // A foreign id has no value here.
    let foreign = RecipeNodeId(999);
    assert!(
        find_flush_candidates(&ev, base, foreign, Tol::witness())
            .unwrap()
            .is_empty()
    );
}

// ------------------------------------------------------------------
// 2. The round trip: undeclared refuses; detect → declare → verifies.
// ------------------------------------------------------------------

/// The protocol end to end, against the SAME geometry: the undeclared
/// union refuses with the typed MENU (`UndeclaredContact`, R3 — the
/// declare arm is this module; the move-the-geometry arm is the
/// recipe's); the
/// declared union — declarations authored FROM the findings through
/// the sugar — succeeds with the exact volume. Detect-then-declare
/// cannot disagree with verify-at-use because both are the same doors.
#[test]
fn detect_declare_boolean_round_trip() {
    let (doc, base, top) = stacked();

    // Arm zero: no declaration — the boolean refuses, loudly.
    let (undeclared, refused) = insert(
        doc.clone(),
        Node::Boolean {
            op: BooleanOp::Union,
            a: base,
            b: top,
            declare: None,
        },
    );
    let ev = eval(&undeclared);
    let menu = match ev.nodes.get(&refused) {
        Some(NodeResult::Failed(e)) => match &e.kind {
            // R3 (LIB-PYG5): the refusal IS the menu — it carries the
            // candidate declaration in the detector's own value shape,
            // built from what the raise site held (no re-detection on
            // the error path).
            NodeErrorKind::UndeclaredContact { finding, diag } => {
                // Exactly-on contact: the verifier's decided-zero
                // encoding, on the verify door's own site.
                assert!(
                    matches!(diag.margin, geom_core::MarginDiag::Invalid),
                    "{diag:?}"
                );
                assert_eq!(diag.predicate, Some("bool_plane_offset"));
                (**finding).clone()
            }
            other => panic!("undeclared union must refuse with the menu, got {other:?}"),
        },
        other => panic!("undeclared union must refuse, got {other:?}"),
    };

    // The declare arm: findings (values, inspected above) → sugar →
    // Declare node → the boolean's declare input.
    let ev = eval(&doc);
    let findings = find_flush_candidates(&ev, base, top, Tol::witness()).unwrap();
    // Menu/detector parity: the refusal named a pair the detector
    // also reports, name for name, relation for relation — same
    // doors underneath, so they cannot disagree.
    assert!(
        findings.contains(&menu),
        "menu payload {menu:?} not among the detector's findings {findings:?}"
    );
    let (doc, decl) = declare_all(&doc, &findings, Tol::witness()).unwrap();
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: base,
            b: top,
            declare: Some(decl),
        },
    );
    let ev = eval(&doc);
    match &ev.value(union).expect("declared union evaluates").payload {
        ValuePayload::Boolean(BooleanValue::Body { body, .. }) => {
            let m = mass_properties(body, Tol::witness()).expect("mass properties");
            // 1³ + 0.5² · 0.5 (dyadic, exact).
            assert_eq!(m.volume, 1.125);
        }
        other => panic!("expected a body, got {}", other.kind_name()),
    }
}

// ------------------------------------------------------------------
// 3. In-band pairs refuse (§3a).
// ------------------------------------------------------------------

/// A cap gap strictly inside the ambiguity band: the pair is neither
/// a finding nor silently dropped — `PairInBand`, naming the pair and
/// the VERIFIER'S funnel site (`bool_plane_offset`; detection mints
/// no sites of its own).
#[test]
fn in_band_gap_refuses_pair_in_band() {
    let tol = geom_core::Tol::witness().get();
    // The band's midpoint: strictly inside (eps, k·eps) for any k > 1.
    let gap = 0.5 * (tol.eps + tol.k * tol.eps);
    let (doc, base) = box_at(
        ProfileDoc::empty_derived("lib_sel2_flush", Tol::witness()),
        0.0,
        (0.0, 0.0),
        (1.0, 1.0),
        1.0,
    );
    let (doc, top) = box_at(doc, 1.0 + gap, (0.25, 0.25), (0.75, 0.75), 0.5);
    let ev = eval(&doc);
    match find_flush_candidates(&ev, base, top, Tol::witness()) {
        Err(SelectRefusal::PairInBand {
            pair, predicate, ..
        }) => {
            assert_eq!(pair.0.node, base);
            assert_eq!(pair.1.node, top);
            assert_eq!(predicate, "bool_plane_offset");
        }
        other => panic!("expected PairInBand, got {other:?}"),
    }
}

// ------------------------------------------------------------------
// 4. The sugar is thin, and refuses emptiness.
// ------------------------------------------------------------------

/// `declare_all` on nothing is a refusal, not an empty Declare node:
/// inserting one would record the LOOK of declared intent with no
/// content.
#[test]
fn empty_findings_refuse() {
    let doc = ProfileDoc::empty_derived("lib_sel2_flush", Tol::witness());
    assert!(matches!(
        declare_all(&doc, &[], Tol::witness()),
        Err(DeclareError::NoFindings)
    ));
}

/// `declare(finding)` inserts exactly the finding's pair as a
/// `Node::Declare` — the single-finding arm of the ruled
/// both-arities-ship boundary.
#[test]
fn declare_inserts_the_pair() {
    let (doc, base, top) = stacked();
    let ev = eval(&doc);
    let findings = find_flush_candidates(&ev, base, top, Tol::witness()).unwrap();
    let (doc, decl) = declare(&doc, &findings[0], Tol::witness()).unwrap();
    match doc.node(decl) {
        Some(Node::Declare { pairs }) => {
            assert_eq!(pairs, &[(findings[0].pair.clone(), findings[0].class)])
        }
        other => panic!("expected a Declare node, got {other:?}"),
    }
}

/// **The verification-arm falsifier** (adopted from the #304 review's
/// planted-drift probe, then aimed both ways). Two resting pairs
/// TILTED so their angular margins, levered at the SHARED
/// verification arm, land just inside the ambiguity band's two ends:
/// at the correct arm BOTH refuse `PairInBand` at
/// `bool_plane_parallel`. An arm drifted UP by ~2% turns the
/// near-escalate tilt definite (silent empty result); an arm drifted
/// DOWN by ~1% decides the near-zero tilt parallel and yields a
/// finding. Either way this row fails — the arm is pinned, not
/// hand-mirrored (review MINOR-1).
#[test]
fn tilted_in_band_pairs_pin_the_verification_arm() {
    let tol = geom_core::Tol::witness().get();
    for theta in [1.01 * tol.eps, 0.99 * tol.k * tol.eps] {
        let (c, s) = (theta.cos(), theta.sin());
        let (doc, base) = box_at(
            ProfileDoc::empty_derived("lib_sel2_flush", Tol::witness()),
            0.0,
            (0.0, 0.0),
            (1.0, 1.0),
            1.0,
        );
        // A block resting on the top cap, sketched on a plane through
        // z = 1 tilted by theta about the x axis.
        let (doc, p) = on_frame(
            doc,
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 0.0],
            [0.0, c, s],
            vec![vec![(0.25, 0.25), (0.75, 0.25), (0.75, 0.75), (0.25, 0.75)]],
        );
        let (doc, tilted) = insert(
            doc,
            Node::Extrude {
                profile: p,
                distance: len(0.5),
            },
        );
        let ev = eval(&doc);
        match find_flush_candidates(&ev, base, tilted, Tol::witness()) {
            Err(SelectRefusal::PairInBand { predicate, .. }) => {
                assert_eq!(predicate, "bool_plane_parallel", "theta = {theta:e}");
            }
            other => panic!("theta = {theta:e}: expected PairInBand, got {other:?}"),
        }
    }
}
