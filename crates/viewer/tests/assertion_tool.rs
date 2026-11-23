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
    AssertionRelation, CheckEvidence, CheckId, Claim, ClearanceBounds, Dimension, Doc, Formula,
    MeasureVerb, Node, Operand, ProfileProgram, RecipeNodeId,
};
use pncad::geom_core::Tol;
use viewer::session::{AssertedClaim, AssertedSeat, DocSession, FaceSelection, Refusal, SessionOp};
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
fn valued(measure: MeasureVerb, relation: AssertionRelation) -> AssertedClaim {
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

/// **Through the tool, a `Gap = 0` between the two touching walls
/// quiets their contact, naming the assertion; a clearance between the
/// same picks does not.** The `Gap` is the measure and the assertion
/// over its output, which is what `[assertions.quiet]` reads; a
/// clearance claim is no value, so no row is quiet under it, whatever
/// its bound. The tool writes the relation its bound choice names, and
/// offers no `Gap` claim for a band; a `Gap ≥ 0`, a `Gap ≤ 0` and a
/// `Distance = 0` between the walls leave the contact loud.
///
/// The same `Gap`, its seats the references the contact's finding
/// carries ([`AssertedSeat::Minted`]) rather than picks, quiets it too.
///
/// Red if the tool writes a `Gap` claim in any other vocabulary than
/// the one the quieting rule reads, writes another relation than the
/// one chosen, the clearance arm writes a value, or a minted seat is
/// written as anything but the reference the kernel minted.
#[test]
fn a_gap_of_zero_between_touching_walls_quiets_their_contact_and_a_clearance_does_not() {
    use viewer::combine::{AssertionMeasure, AssertionTool, BoundChoice};
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    assert!(
        contact(&session).quiet.is_none(),
        "the premise: nothing asserted, the contact is loud"
    );
    let mut tool = AssertionTool::new();
    for copy in copies {
        let face = wall(&session, copy);
        tool.seat_face(session.committed_doc(), face);
    }
    let tool_op = |measure: AssertionMeasure, bound: BoundChoice| {
        let claim = measure
            .claim(bound, len(0.0))
            .unwrap_or_else(|| panic!("{measure:?} {bound:?} is a claim"));
        tool.op(claim).expect("both seats are held")
    };

    let clearance = add(
        &mut session,
        tool_op(AssertionMeasure::Clearance, BoundChoice::Equal),
    );
    assert!(
        matches!(
            session.committed_doc().node(clearance),
            Some(Node::Assertion(Claim::Clearance { .. }))
        ),
        "the clearance arm writes a clearance claim"
    );
    assert_eq!(
        contact(&session).quiet,
        None,
        "a clearance claim quiets no coincidence"
    );

    assert_eq!(
        AssertionMeasure::Gap.claim(BoundChoice::Between, len(0.0)),
        None,
        "a band bounds no value"
    );
    for bound in [BoundChoice::AtLeast, BoundChoice::AtMost] {
        let outcome = session.perform(tool_op(AssertionMeasure::Gap, bound));
        assert!(
            outcome.refusal.is_none(),
            "{bound:?}: {:?}",
            outcome.refusal
        );
        let made = outcome.made.expect("an assertion is a node");
        assert!(
            matches!(
                session.committed_doc().node(made),
                Some(Node::Assertion(Claim::Value { relation, .. }))
                    if Some(*relation) == bound.relation()
            ),
            "{bound:?}: the relation chosen"
        );
        session.pump();
        assert_eq!(contact(&session).quiet, None, "{bound:?} 0 quiets nothing");
        let undone = session.perform(SessionOp::Undo);
        assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    }
    let distance = session.perform(tool_op(AssertionMeasure::Distance, BoundChoice::Equal));
    assert!(distance.refusal.is_none(), "{:?}", distance.refusal);
    session.pump();
    assert_eq!(
        contact(&session).quiet,
        None,
        "a Distance = 0 between two faces says no rest of them"
    );
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    let gap = add(
        &mut session,
        tool_op(AssertionMeasure::Gap, BoundChoice::Equal),
    );
    assert_eq!(
        contact(&session).quiet,
        Some(gap),
        "the Gap = 0 quiets the contact, named"
    );

    // Filled from the finding rather than the tool: each seat the
    // reference the kernel minted for the contact.
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    session.pump();
    assert_eq!(contact(&session).quiet, None, "undone, the contact is loud");
    let [a, b] = minted(&session);
    let read = add(
        &mut session,
        SessionOp::AddAssertion {
            a: AssertedSeat::Minted(a),
            b: AssertedSeat::Minted(b),
            claim: valued(MeasureVerb::Gap, AssertionRelation::Equal),
        },
    );
    assert_eq!(
        contact(&session).quiet,
        Some(read),
        "the Gap = 0 over the finding's own references quiets it too"
    );
}

/// The two references the loud contact's finding carries for the `Gap`
/// that would quiet it, outer then inner.
fn minted(session: &DocSession) -> [pncad::document::QuietingRef; 2] {
    let CheckEvidence::UnprovenCoincidence {
        quieting: Some(quieting),
        ..
    } = contact(session).evidence
    else {
        panic!("the walls' contact carries the assertion that quiets it")
    };
    assert_eq!(quieting.measure(), MeasureVerb::Gap, "a rest is a Gap");
    quieting.references().clone()
}

/// **A value claim is the kernel's measure and assertion pair**, bit
/// for bit: for `Gap` and `Distance` under `=`, `≥` and `≤` at a 3 mm
/// bound, the op over two minted seats writes the document the kernel's
/// `measure` door and an assertion over its output write — the pair the
/// Python binding writes. Over two picks, the measure's first reference
/// is the first seat's face.
///
/// Red if the op writes another measure, relation or bound, swaps the
/// seats, or reads the output at another dimension.
#[test]
fn a_value_claim_is_the_kernels_measure_and_assertion_pair_in_seat_order() {
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    let [ra, rb] = minted(&session);
    let (a, b) = (ra.operand(), rb.operand());
    for measure in [MeasureVerb::Gap, MeasureVerb::Distance] {
        for relation in [
            AssertionRelation::Equal,
            AssertionRelation::AtLeast,
            AssertionRelation::AtMost,
        ] {
            let why = format!("{measure:?} {relation:?}");
            let before = session.committed_doc().clone();
            let outcome = session.perform(SessionOp::AddAssertion {
                a: AssertedSeat::Minted(ra.clone()),
                b: AssertedSeat::Minted(rb.clone()),
                claim: AssertedClaim::Value {
                    measure,
                    relation,
                    bound: len(0.003),
                },
            });
            assert!(outcome.refusal.is_none(), "{why}: {:?}", outcome.refusal);
            let measured = pncad::document::measure(
                &before,
                &[match measure {
                    MeasureVerb::Gap => pncad::document::MeasurePrimitive::Gap {
                        outer: a.clone(),
                        inner: b.clone(),
                    },
                    _ => pncad::document::MeasurePrimitive::Distance {
                        a: a.clone(),
                        b: b.clone(),
                    },
                }],
                tol,
            )
            .unwrap_or_else(|e| panic!("{why}: the kernel measures: {e}"));
            let [output] = measured.measured.outputs[..] else {
                panic!("{why}: one output");
            };
            let (kernel, _) = inserted(
                &measured.doc,
                Node::Assertion(Claim::Value {
                    value: Formula::var(output, Dimension::Length),
                    bound: len(0.003),
                    relation,
                }),
                tol,
            );
            assert!(
                session.committed_doc().bit_eq(&kernel),
                "{why}: the kernel's pair"
            );
            let undone = session.perform(SessionOp::Undo);
            assert!(undone.refusal.is_none(), "{why}: {:?}", undone.refusal);
        }
    }

    let [first, second] = walls(&session, copies);
    let outcome = session.perform(SessionOp::AddAssertion {
        a: first,
        b: second,
        claim: valued(MeasureVerb::Gap, AssertionRelation::Equal),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let doc = session.committed_doc();
    let Some(Node::Assertion(Claim::Value { value, .. })) =
        outcome.made.and_then(|made| doc.node(made))
    else {
        panic!("a value claim");
    };
    let Some(Node::Measure {
        primitive: pncad::document::MeasurePrimitive::Gap { outer, .. },
    }) = doc
        .var(*value)
        .and_then(|var| var.def().output())
        .and_then(|(measure, _)| doc.node(measure))
    else {
        panic!("a Gap's output");
    };
    assert_eq!(
        doc.selection(*outer).map(|s| s.names.clone()),
        Some(vec![wall(&session, copies[0]).name]),
        "the Gap's outer is the first seat's face"
    );
}

/// **Every claim, undone, leaves no trace**: a clearance, a `Gap` and a
/// `Distance`, each one action, and its undo the document it started
/// from, bit for bit — the measure a value claim writes goes with it;
/// its redo is the document it made.
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
        (valued(MeasureVerb::Gap, AssertionRelation::Equal), 2),
        (valued(MeasureVerb::Distance, AssertionRelation::AtLeast), 2),
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
        let after = session.committed_doc().clone();
        session.pump();
        let undone = session.perform(SessionOp::Undo);
        assert!(undone.refusal.is_none(), "{claim:?}: {:?}", undone.refusal);
        assert!(
            session.committed_doc().bit_eq(&before),
            "{claim:?}: the undo is the document before"
        );
        let redone = session.perform(SessionOp::Redo);
        assert!(redone.refusal.is_none(), "{claim:?}: {:?}", redone.refusal);
        assert!(
            session.committed_doc().bit_eq(&after),
            "{claim:?}: the redo is the document after"
        );
        let undone = session.perform(SessionOp::Undo);
        assert!(undone.refusal.is_none(), "{claim:?}: {:?}", undone.refusal);
    }
}

/// **A seat the measure cannot read is refused in the kernel's words**:
/// a `Gap` or a `Distance` over a body reads no face, an edge or a
/// vertex, so the insert door refuses it, and the op says the door's
/// own refusal — the one the kernel's measure door answers the same
/// primitive with — leaving the document as it was. A clearance over
/// the same seats is written.
///
/// Red if the op checks the seat itself, or drops the door's refusal.
#[test]
fn a_seat_the_measure_cannot_read_is_refused_in_the_kernels_words() {
    use pncad::document::MeasurePrimitive as M;
    let tol = Tol::witness();
    let (mut session, copies) = touching(tol);
    let face = wall(&session, copies[1]);
    let before = session.committed_doc().clone();
    let body = Operand::Node(copies[0]);
    let picked = Operand::select(face.node, vec![face.name.clone()]);
    for (measure, primitive) in [
        (
            MeasureVerb::Gap,
            M::Gap {
                outer: body.clone(),
                inner: picked.clone(),
            },
        ),
        (
            MeasureVerb::Distance,
            M::Distance {
                a: body.clone(),
                b: picked.clone(),
            },
        ),
    ] {
        let outcome = session.perform(SessionOp::AddAssertion {
            a: AssertedSeat::Body(copies[0].into()),
            b: AssertedSeat::Face(face.clone()),
            claim: valued(measure, AssertionRelation::Equal),
        });
        let Some(Refusal::Edit(said)) = &outcome.refusal else {
            panic!(
                "{measure:?}: the insert door refuses: {:?}",
                outcome.refusal
            );
        };
        let Err(kernel) = pncad::document::measure(&before, &[primitive], tol) else {
            panic!("{measure:?}: the kernel's door refuses the same primitive");
        };
        assert_eq!(
            said.to_string(),
            kernel.to_string(),
            "{measure:?}: the kernel's words"
        );
        assert!(
            session.committed_doc().bit_eq(&before),
            "{measure:?}: a refusal writes nothing"
        );
    }
    let clearance = session.perform(SessionOp::AddAssertion {
        a: AssertedSeat::Body(copies[0].into()),
        b: AssertedSeat::Face(face),
        claim: AssertedClaim::Clearance(ClearanceBounds::AtLeast(len(0.0))),
    });
    assert!(
        clearance.refusal.is_none(),
        "a clearance reads a body: {:?}",
        clearance.refusal
    );
}
