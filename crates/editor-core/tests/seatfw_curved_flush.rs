//! **The flush detector on the curved `Rest` rungs** (`topo::flush`,
//! `editor_core::names::flush`; VERB-SEAT-DESIGN §1 S3, SELECT-DESIGN
//! §3) — the rows the widening from `flush_pair_relation` to
//! `carrier_pair_relation` is answerable for.
//!
//! The detector detects what the verifier verifies, and the verifier's
//! `Rest` ladder has carried plane, sphere, cylinder and torus rungs
//! since M9-3. These rows are the curved half of that sentence, on the
//! smallest geometry that shows it: a peg standing in a bore of the
//! SAME radius, its block overshooting both ways so that **no plane of
//! one body coincides with a plane of the other**. Every finding here
//! is therefore cylindrical or there is none.
//!
//! Both seats are pinned in one file because they are one door: the
//! body seat answers keys, the document seat answers names, and the
//! per-pair test under both is `topo::flush::pair_finding`. A row that
//! passed at one seat and failed at the other would be the twin the
//! anti-twin rule forbids.
//!
//! What the last row pins is the boundary of the claim: a finding says
//! "declared, this pair VERIFIES", never "the op will build". The
//! purely cylindrical mate is declared, the declaration is verified,
//! and the reduction then meets its own curved-face frontier — typed,
//! downstream, and no business of the detector's.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;

use editor_core::{
    BooleanOp, CancelToken, ContactClass, EvalOptions, Evaluation, FlushRung, LoopProgram, Node,
    NodeErrorKind, NodeResult, ProfileDoc, ProfileProgram, RecipeNodeId, ValuePayload, declare_all,
    evaluate, find_flush_candidates,
};
use geom_brep::SurfaceKind;
use geom_core::Tol;
use topo::{Body, PlaneRelation, query};

use fixture::{insert, len, square};

fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(
        doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        Tol::witness(),
    )
}

/// The peg and its block, with the bore's radius a knob: the peg is a
/// circle of radius `PEG_R` extruded `z ∈ [0, 1]`, the block a square
/// with an inner circle of radius `bore_r` on the SAME centre,
/// extruded `z ∈ [−0.5, 1.5]`.
///
/// The block overshoots the peg at both ends ON PURPOSE: with the caps
/// clear of each other the two bodies share no plane at all, so the
/// findings below are the curved rungs' own answer and nothing else.
const PEG_R: f64 = 0.25;

fn peg_in_bore(bore_r: f64) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("seatfw_curved_flush", Tol::witness());
    let circle = |r: f64| LoopProgram::Circle {
        centre: [len(0.0), len(0.0)],
        radius: len(r),
    };
    let (doc, peg_plane) = insert(doc, fixture::frame([0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [
        0.0, 1.0, 0.0,
    ]));
    let (doc, peg_profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: peg_plane,
            loops: vec![circle(PEG_R)],
        }),
    );
    let (doc, peg) = insert(
        doc,
        Node::Extrude {
            profile: peg_profile,
            distance: len(1.0),
        },
    );
    let (doc, block_plane) = insert(doc, fixture::frame([0.0, 0.0, -0.5], [1.0, 0.0, 0.0], [
        0.0, 1.0, 0.0,
    ]));
    let (doc, block_profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            plane: block_plane,
            loops: vec![
                LoopProgram::polygon(square(0.0, 0.0, 1.0)).expect("finite corners"),
                circle(bore_r),
            ],
        }),
    );
    let (doc, block) = insert(
        doc,
        Node::Extrude {
            profile: block_profile,
            distance: len(2.0),
        },
    );
    (doc, peg, block)
}

/// The evaluated body of one node — the body seat's operand, taken
/// out of the same evaluation the document seat is asked about, so the
/// two seats are asked about ONE pair of bodies.
fn body_of(ev: &Evaluation<f64>, n: RecipeNodeId) -> &Body<f64> {
    match &ev.value(n).expect("the extrude evaluates").payload {
        ValuePayload::Body(b) => b,
        other => panic!("an extrude's payload is a body, got {}", other.kind_name()),
    }
}

fn cylinder_faces(body: &Body<f64>) -> usize {
    query::all_faces(body)
        .into_iter()
        .filter(|&f| query::face_surface_kind(body, f) == Some(SurfaceKind::Cylinder))
        .count()
}

// ------------------------------------------------------------------
// 1. A cylindrical cosurface pair is a finding, at both seats.
// ------------------------------------------------------------------

/// The peg's wall against the bore's wall: one carrier, opposite
/// material sides — `SameOpposite`, which IS `Rest` (the C4 lemma
/// spelled on a curved carrier). The count is the product of the two
/// walls' face counts, because a circular extrude mints its wall in
/// halves and every half-against-half pair sits on the shared
/// cylinder.
#[test]
fn a_cylindrical_cosurface_pair_is_a_finding_at_both_seats() {
    let (doc, peg, block) = peg_in_bore(PEG_R);
    let ev = eval(&doc);
    let (a, b) = (body_of(&ev, peg), body_of(&ev, block));
    let walls = cylinder_faces(a) * cylinder_faces(b);
    assert!(walls > 0, "the fixture must have curved walls to pair");

    let keys = topo::flush::find_flush_candidates(a, b, Tol::witness())
        .expect("the fixture is authored exactly, so every pair decides definitely");
    assert_eq!(
        keys.len(),
        walls,
        "every wall-against-wall pair sits on the shared cylinder: {keys:?}"
    );
    for f in &keys {
        assert_eq!(f.class, ContactClass::Rest);
        assert_eq!(f.evidence.relation, PlaneRelation::SameOpposite, "{f:?}");
        assert_eq!(f.evidence.rung, FlushRung::DecidedCoincident, "{f:?}");
    }

    let names = find_flush_candidates(&ev, peg, block, Tol::witness())
        .expect("the document seat decides the same pairs");
    assert_eq!(
        names.len(),
        keys.len(),
        "one verifier under both seats: keys {keys:?} vs names {names:?}"
    );
    for f in &names {
        assert_eq!(f.class, ContactClass::Rest);
        assert_eq!(f.evidence.relation, PlaneRelation::SameOpposite, "{f:?}");
        assert_eq!(f.pair.0.node, peg);
        assert_eq!(f.pair.1.node, block);
    }
}

// ------------------------------------------------------------------
// 2. A different carrier is no finding.
// ------------------------------------------------------------------

/// The same peg in a WIDER bore: definitely distinct carriers, so
/// nothing is reported — at either seat. Without this row the row
/// above cannot tell "the curved rungs decide" from "the curved arm
/// reports whatever it is handed".
#[test]
fn a_bore_on_another_carrier_is_no_finding_at_either_seat() {
    let (doc, peg, block) = peg_in_bore(PEG_R + 0.05);
    let ev = eval(&doc);
    let (a, b) = (body_of(&ev, peg), body_of(&ev, block));
    let keys = topo::flush::find_flush_candidates(a, b, Tol::witness())
        .expect("a definite radius difference decides");
    assert!(keys.is_empty(), "distinct cylinders are no contact: {keys:?}");
    let names =
        find_flush_candidates(&ev, peg, block, Tol::witness()).expect("the document seat decides");
    assert!(names.is_empty(), "{names:?}");
}

// ------------------------------------------------------------------
// 3. What a finding promises, and what it does not.
// ------------------------------------------------------------------

/// The declared round trip on a PURELY cylindrical mate: the findings
/// declare, the declared rung VERIFIES them — no undeclared-contact
/// refusal, no contradiction, because the detector and the verifier
/// are one door — and the boolean then meets a LANE frontier further
/// in, at the seam join, which this fixture names.
///
/// The row is here so that the detector's claim stays the narrow one
/// it is: "declared, this pair verifies" — never "the op will build".
/// WHICH frontier is a measurement, not the claim: a curved mate
/// reaches a different one per configuration (the plant's purely
/// cylindrical mate stops earlier, in the reduction's curved-pierce
/// arm — issue #1032), and a lane that opens moves this row's payload
/// without touching what it is about.
#[test]
fn a_declared_curved_finding_verifies_and_then_meets_the_lane_frontier() {
    let (doc, peg, block) = peg_in_bore(PEG_R);
    let ev = eval(&doc);
    let findings = find_flush_candidates(&ev, peg, block, Tol::witness()).expect("the pairs decide");
    assert!(!findings.is_empty());
    let (doc, decl) = declare_all(&doc, &findings, Tol::witness()).expect("findings declare");
    let (doc, union) = insert(
        doc,
        Node::Boolean {
            op: BooleanOp::Union,
            a: peg,
            b: block,
            declare: Some(decl),
        },
    );
    let ev = eval(&doc);
    let Some(NodeResult::Failed(e)) = ev.nodes.get(&union) else {
        panic!("a purely cylindrical mate does not reach the zip today (issue #1032)");
    };
    let NodeErrorKind::Boolean(err) = &e.kind else {
        panic!("the boolean is what refuses: {e:?}");
    };
    assert!(
        !matches!(
            err,
            topo::BooleanError::UndeclaredCoincidence { .. }
                | topo::BooleanError::DeclarationContradicted { .. }
                | topo::BooleanError::ContactContradicted { .. }
        ),
        "the declared curved pairs are ADMITTED and verified — the detector cannot \
         propose a pair the declared rung then rejects: {err:?}"
    );
    assert!(
        matches!(err, topo::BooleanError::Join(_)),
        "what stops this mate is a lane frontier past verification, at the join: {err:?}"
    );
}
