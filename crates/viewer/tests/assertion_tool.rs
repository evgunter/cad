//! **The assertion tool** (`SessionOp::AddAssertion`): one op authors
//! a clearance, a `Gap` or a `Distance` claim between two seats, in the
//! kernel's own node vocabulary, as one action and one undo.
//!
//! The scene is two 20 × 20 × 10 mm blocks side by side along `x`,
//! touching: the left block's wall at `x = 20 mm` rests on the right
//! block's, an at-rest coincidence the lint reports until an assertion
//! says it ([`assertions.quiet`]).

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use pncad::document::{
    AssertionRelation, CheckEvidence, CheckId, ClearanceBounds, Doc, Node, Operand, ProfileProgram,
    RecipeNodeId,
};
use pncad::geom_core::Tol;
use viewer::session::{
    AssertedClaim, AssertedSeat, DocSession, FaceSelection, Refusal, SessionOp, ValueMeasure,
};
use viewer::test_support::{CLEARANCE_SIDE as SIDE, inserted, len, placed, rectangle};

/// **Two blocks touching along `x`**, each placed into the world once:
/// the session, landed, and each block's world copy.
fn touching(tol: Tol) -> (DocSession, [RecipeNodeId; 2]) {
    let mut doc = Doc::<ProfileProgram>::empty_derived("two-blocks-touching", tol);
    let mut copies = [RecipeNodeId::WORLD; 2];
    for (i, x) in [0.0, SIDE].into_iter().enumerate() {
        let (with, profile) = inserted(&doc, rectangle([x, 0.0], SIDE, SIDE), tol);
        let (with, block) = inserted(
            &with,
            Node::Extrude {
                profile: profile.into(),
                distance: len(0.01),
            },
            tol,
        );
        let (with, copy) = placed(&with, block, tol);
        doc = with;
        copies[i] = copy;
    }
    let mut session = DocSession::inline(doc, tol);
    session.pump();
    (session, copies)
}

/// The wall of the block `copy` draws that lies in `x = SIDE`, as a pick
/// in the picture takes it.
fn wall(session: &DocSession, copy: RecipeNodeId) -> FaceSelection {
    let (_, eval) = session.landed_pair().expect("the scene landed");
    let lateral = pncad::select::Selector::of(
        pncad::select::NamePat::of_kind(pncad::select::EntityKind::Face)
            .seg(pncad::select::SegPat::tag(pncad::select::SegTag::Lateral)),
    );
    pncad::select::select(eval, copy, &lateral)
        .into_iter()
        .find(|name| {
            pncad::select::face_frame(eval, copy, name)
                .is_ok_and(|pose| (pose.origin.x - SIDE).abs() < 1e-9 && pose.axis.x.abs() > 0.5)
        })
        .map(|name| FaceSelection {
            name,
            node: copy,
            body: 0,
        })
        .expect("the block has a wall at x = SIDE")
}

/// The two walls' seats, the left block's first.
fn walls(session: &DocSession, copies: [RecipeNodeId; 2]) -> [AssertedSeat; 2] {
    copies.map(|copy| AssertedSeat::Face(wall(session, copy)))
}

/// `value relation 0` over `measure`.
fn valued(measure: ValueMeasure, relation: AssertionRelation) -> AssertedClaim {
    AssertedClaim::Value {
        measure,
        relation,
        bound: len(0.0),
    }
}

/// The landed report's coincidence findings of two faces resting
/// opposed — the walls' contact, and nothing else in the scene.
fn contacts(session: &DocSession) -> Vec<pncad::document::CheckFinding> {
    let report = session.checks().expect("the registry ran");
    assert!(
        report.pending.is_empty(),
        "the inline door settled: {report}"
    );
    report
        .findings
        .iter()
        .filter(|f| {
            f.check == CheckId::UnprovenCoincidence
                && matches!(
                    &f.evidence,
                    CheckEvidence::UnprovenCoincidence { row, .. }
                        if row.relation == pncad::topo::Relation::SameOpposite
                )
        })
        .cloned()
        .collect()
}

/// The one contact finding.
fn contact(session: &DocSession) -> pncad::document::CheckFinding {
    let [found] = contacts(session)
        .try_into()
        .unwrap_or_else(|found: Vec<_>| panic!("one contact, the walls': {found:?}"));
    found
}

/// Perform `op`, answering the node it made.
fn add(session: &mut DocSession, op: SessionOp) -> RecipeNodeId {
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let made = outcome.made.expect("an assertion is a node");
    session.pump();
    made
}

/// **A `Gap = 0` between the two touching walls quiets their contact,
/// naming the assertion; a clearance between the same picks does not.**
/// The `Gap` is the measure and the assertion over its output, which is
/// what `[assertions.quiet]` reads; a clearance claim is no value, so
/// no row is quiet under it, whatever its bound.
///
/// The same `Gap`, its seats read as given ([`AssertedSeat::Read`])
/// rather than picked, quiets it too.
///
/// Red if the op writes a `Gap` claim in any other vocabulary than the
/// one the quieting rule reads, the clearance arm writes a value, or a
/// read seat is written as anything but the read it was given.
#[test]
fn a_gap_of_zero_between_touching_walls_quiets_their_contact_and_a_clearance_does_not() {
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    assert!(
        contact(&session).quiet.is_none(),
        "the premise: nothing asserted, the contact is loud"
    );

    let [a, b] = walls(&session, copies);
    let clearance = add(
        &mut session,
        SessionOp::AddAssertion {
            a: a.clone(),
            b: b.clone(),
            claim: AssertedClaim::Clearance(ClearanceBounds::Equal(len(0.0))),
        },
    );
    assert!(
        matches!(
            session.committed_doc().node(clearance),
            Some(Node::Assertion(pncad::document::Claim::Clearance { .. }))
        ),
        "the clearance arm writes a clearance claim"
    );
    assert_eq!(
        contact(&session).quiet,
        None,
        "a clearance claim quiets no coincidence"
    );

    let gap = add(
        &mut session,
        SessionOp::AddAssertion {
            a,
            b,
            claim: valued(ValueMeasure::Gap, AssertionRelation::Equal),
        },
    );
    assert_eq!(
        contact(&session).quiet,
        Some(gap),
        "the Gap = 0 quiets the contact, named"
    );

    // Filled from values rather than the tool: each seat the selection
    // of the wall's name in the copy that draws it, read as given — the
    // shape a caller already holding the claim, a check's finding,
    // fills.
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    session.pump();
    assert_eq!(contact(&session).quiet, None, "undone, the contact is loud");
    let [a, b] = copies.map(|copy| {
        let face = wall(&session, copy);
        AssertedSeat::Read(Operand::select(copy, vec![face.name]))
    });
    let read = add(
        &mut session,
        SessionOp::AddAssertion {
            a,
            b,
            claim: valued(ValueMeasure::Gap, AssertionRelation::Equal),
        },
    );
    assert_eq!(
        contact(&session).quiet,
        Some(read),
        "the Gap = 0 over seats read as given quiets it too"
    );
}

/// **Every claim, undone, leaves no trace**: a clearance, a `Gap` and a
/// `Distance`, each one action, and its undo the document it started
/// from, bit for bit — the measure a value claim writes goes with it.
///
/// Red if a value claim's measure and assertion are two actions, or the
/// undo leaves the measure behind.
#[test]
fn every_claim_undone_leaves_no_trace() {
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    let [a, b] = walls(&session, copies);
    for (claim, nodes) in [
        (
            AssertedClaim::Clearance(ClearanceBounds::AtLeast(len(0.0))),
            1,
        ),
        (valued(ValueMeasure::Gap, AssertionRelation::Equal), 2),
        (
            valued(ValueMeasure::Distance, AssertionRelation::AtLeast),
            2,
        ),
    ] {
        let before = session.committed_doc().clone();
        let outcome = session.perform(SessionOp::AddAssertion {
            a: a.clone(),
            b: b.clone(),
            claim: claim.clone(),
        });
        assert!(
            outcome.refusal.is_none(),
            "{claim:?}: {:?}",
            outcome.refusal
        );
        assert_eq!(
            session.committed_doc().ids().len(),
            before.ids().len() + nodes,
            "{claim:?}: its nodes"
        );
        session.pump();
        let undone = session.perform(SessionOp::Undo);
        assert!(undone.refusal.is_none(), "{claim:?}: {:?}", undone.refusal);
        assert!(
            session.committed_doc().bit_eq(&before),
            "{claim:?}: the undo is the document before"
        );
    }
}

/// **A seat the measure cannot read is refused in the kernel's words**:
/// a `Gap` over a body reads no face, an edge or a vertex, so the
/// insert door refuses it, and the op says the door's own refusal — the
/// one the kernel's measure door answers the same primitive with —
/// leaving the document as it was.
///
/// Red if the op checks the seat itself, or drops the door's refusal.
#[test]
fn a_seat_the_measure_cannot_read_is_refused_in_the_kernels_words() {
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    let [_, b] = walls(&session, copies);
    let before = session.committed_doc().clone();
    let outcome = session.perform(SessionOp::AddAssertion {
        a: AssertedSeat::Body(copies[0].into()),
        b,
        claim: valued(ValueMeasure::Gap, AssertionRelation::Equal),
    });
    let Some(Refusal::Edit(said)) = &outcome.refusal else {
        panic!("the insert door refuses: {:?}", outcome.refusal);
    };
    let wall = wall(&session, copies[1]);
    let Err(kernel) = pncad::document::measure(
        &before,
        &[pncad::document::MeasurePrimitive::Gap {
            outer: Operand::Node(copies[0]),
            inner: Operand::select(wall.node, vec![wall.name]),
        }],
        tol,
    ) else {
        panic!("the kernel's door refuses the same primitive");
    };
    assert_eq!(said.to_string(), kernel.to_string(), "the kernel's words");
    assert!(
        session.committed_doc().bit_eq(&before),
        "a refusal writes nothing"
    );
}
