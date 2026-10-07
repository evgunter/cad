//! **The GUI's variables** (D10, VARIABLES-DESIGN VR2/VR6): a value
//! typed at a slot mints an anonymous variable and is offered the
//! existing variables of equal value and kind; accepting makes the slot
//! read one, declining keeps the typed one distinct; a name is proposed
//! and stored only when committed; and the value doors read a defined
//! variable as held.

use crate::common;
use editor_core::ExtrudeSide;
use pncad::document::{
    Dimension, Doc, DocEdit, FreeVar, Node, ProfileProgram, RecipeNodeId, SlotId, VarId, VarName,
};
use pncad::geom_core::Tol;
use viewer::props::{self, SlotValue};
use viewer::session::{BoundsTarget, DocSession, Refusal, SessionOp};

/// Two extrudes of one square, depths 8 mm and 10 mm, and two declared
/// variables holding the same number at different kinds: `w`, a
/// 12 mm length, and `k`, the scalar 0.012.
fn two_extrudes() -> (DocSession, RecipeNodeId, RecipeNodeId, VarId, VarId) {
    let tol = Tol::witness();
    let doc: Doc<ProfileProgram> = Doc::empty_derived("gui-variables", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let extrude = |distance: f64| Node::Extrude {
        profile,
        distance: common::len(distance),
        side: ExtrudeSide::Along,
    };
    let (doc, a) = common::inserted(&doc, extrude(0.008), tol);
    let (doc, b) = common::inserted(&doc, extrude(0.010), tol);
    let mut session = DocSession::inline(doc, tol);
    for (name, dimension) in [("w", Dimension::Length), ("k", Dimension::Scalar)] {
        let declared = session.perform(SessionOp::DeclareVar {
            name: VarName::new(name).expect("a name"),
            value: FreeVar::continuous(dimension, 0.012),
        });
        assert!(declared.refusal.is_none(), "{name}: {:?}", declared.refusal);
    }
    let w = common::var_of(session.committed_doc(), "w");
    let k = common::var_of(session.committed_doc(), "k");
    (session, a, b, w, k)
}

fn typed(session: &mut DocSession, node: RecipeNodeId, value: f64) {
    let outcome = session.perform(SessionOp::SetSlot {
        node,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(value),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 1, "typing {value} is one edit");
}

fn offered(session: &DocSession, node: RecipeNodeId) -> Vec<VarId> {
    session
        .offered(node, SlotId::Distance)
        .into_iter()
        .map(|offer| offer.var)
        .collect()
}

fn reads(session: &DocSession, node: RecipeNodeId) -> VarId {
    session
        .committed_doc()
        .slot(node, SlotId::Distance)
        .expect("the extrude reads its distance")
}

/// **A typed value is offered every variable of equal value and of the
/// slot's kind, and no other.** `k` holds the same number as a scalar;
/// `b`'s depth is a length of another value until a value equal to it
/// is typed, when the offer names it by the slot that reads it.
#[test]
fn a_typed_value_is_offered_the_variables_of_equal_value_and_kind() {
    let (mut session, a, b, w, _k) = two_extrudes();
    assert!(
        offered(&session, a).is_empty(),
        "nothing typed, nothing offered"
    );

    typed(&mut session, a, 0.012);
    assert_eq!(offered(&session, a), vec![w], "w alone: k is a scalar");
    assert!(
        offered(&session, b).is_empty(),
        "the offer is the typed slot's"
    );
    let labels: Vec<String> = session
        .offered(a, SlotId::Distance)
        .into_iter()
        .map(|offer| offer.label)
        .collect();
    assert_eq!(labels, vec!["w".to_owned()], "a named variable by its name");

    typed(&mut session, a, 0.010);
    let b_reads = reads(&session, b);
    assert_eq!(offered(&session, a), vec![b_reads], "b's anonymous depth");
    let said = &session.offered(a, SlotId::Distance)[0].label;
    assert_eq!(
        said,
        &format!("the distance of {}", session.committed_doc().spoken(b)),
        "an anonymous variable is said by the slot that reads it (VR2)"
    );
}

/// **Accepting is the slot-write gesture**: one `SetParam` whose
/// formula is the variable, one undo step, the slot reading `w`, and
/// the offer closed.
#[test]
fn accepting_an_offer_makes_the_slot_read_the_variable() {
    let (mut session, a, _b, w, _k) = two_extrudes();
    typed(&mut session, a, 0.012);
    let minted = reads(&session, a);
    let before = session.history().len();

    let accepted = session.perform(SessionOp::SetSlotVariable {
        node: a,
        slot: SlotId::Distance,
        var: w,
    });
    assert!(accepted.refusal.is_none(), "{:?}", accepted.refusal);
    assert!(
        matches!(accepted.committed.as_slice(), [DocEdit::SetParam { .. }]),
        "{:?}",
        accepted.committed
    );
    assert_eq!(session.history().len(), before + 1, "one undo step");
    assert_eq!(reads(&session, a), w, "the slot reads w");
    assert!(
        session.committed_doc().var(minted).is_none(),
        "the typed variable, read by nothing now, is gone (VR7)"
    );
    assert!(offered(&session, a).is_empty(), "an accepted offer closes");
}

/// **Declining keeps the typed variable distinct and moves no
/// document**: no history step, the slot still reads what typing
/// minted, and nothing is offered after.
#[test]
fn declining_keeps_the_typed_variable_and_moves_no_document() {
    let (mut session, a, _b, w, _k) = two_extrudes();
    typed(&mut session, a, 0.012);
    let minted = reads(&session, a);
    let before = session.history().len();

    let declined = session.perform(SessionOp::DeclineOffer {
        node: a,
        slot: SlotId::Distance,
    });
    assert!(declined.refusal.is_none(), "{:?}", declined.refusal);
    assert!(declined.committed.is_empty(), "declining edits nothing");
    assert_eq!(session.history().len(), before, "no undo step");
    assert_eq!(reads(&session, a), minted, "the typed variable stands");
    assert_ne!(minted, w, "distinct from the one offered");
    assert!(offered(&session, a).is_empty(), "a declined offer closes");
}

/// **An offer is about the value typed, and stands only while the slot
/// reads it**: an undo puts back what the slot read before, and the
/// offer goes with it.
#[test]
fn an_offer_stands_only_while_the_slot_reads_what_was_typed() {
    let (mut session, a, _b, w, _k) = two_extrudes();
    typed(&mut session, a, 0.012);
    assert_eq!(offered(&session, a), vec![w]);
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    assert!(offered(&session, a).is_empty(), "the typed value is undone");
    // `w` moved to the depth the slot reads again: equal now, and still
    // offered nothing, since nobody typed that depth.
    let moved = session.perform(SessionOp::SetVariable {
        var: w,
        value: SlotValue::Continuous(0.008),
    });
    assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    assert!(
        offered(&session, a).is_empty(),
        "no offer for an untyped value"
    );
}

/// **Typed text is a typed value too, and a formula is not**: `12 mm`
/// at the slot mints and is offered `w`; `w` at the slot reads `w`
/// itself, and nothing is offered for it.
#[test]
fn typed_text_is_offered_and_a_formula_is_not() {
    let (mut session, a, _b, w, _k) = two_extrudes();
    let text = |session: &mut DocSession, text: &str| {
        let outcome = session.perform(SessionOp::SetSlotExpression {
            node: a,
            slot: SlotId::Distance,
            text: text.to_owned(),
        });
        assert!(outcome.refusal.is_none(), "{text}: {:?}", outcome.refusal);
    };
    text(&mut session, "12 mm");
    assert_eq!(offered(&session, a), vec![w], "a written quantity mints");
    text(&mut session, "w");
    assert_eq!(reads(&session, a), w);
    assert!(
        offered(&session, a).is_empty(),
        "a read of w offers nothing"
    );
}

/// **Two slots that share a variable are made two by a value typed at
/// either** (D10: declining is what makes two distinct, and typing is
/// a new variable whatever the slot read).
#[test]
fn a_value_typed_at_a_shared_slot_makes_it_its_own() {
    let (mut session, a, b, _w, _k) = two_extrudes();
    typed(&mut session, a, 0.010);
    let shared = reads(&session, b);
    let accepted = session.perform(SessionOp::SetSlotVariable {
        node: a,
        slot: SlotId::Distance,
        var: shared,
    });
    assert!(accepted.refusal.is_none(), "{:?}", accepted.refusal);
    assert_eq!(reads(&session, a), shared, "a and b read one variable");

    typed(&mut session, a, 0.020);
    assert_ne!(reads(&session, a), shared, "a reads a new variable");
    assert_eq!(reads(&session, b), shared, "b keeps the one it read");
    assert_eq!(
        session.committed_doc().free(shared).map(FreeVar::dim),
        Some(Dimension::Length),
        "and it still holds b's depth"
    );
}

/// **A name is proposed and stored only on commit** (VR2): the proposal
/// is the slot's word, stepped past a name the document holds; the
/// document holds no name until `RenameVar` lands, and naming moves no
/// reader.
#[test]
fn a_proposed_name_is_stored_only_when_committed() {
    let (mut session, a, b, _w, _k) = two_extrudes();
    let doc = session.committed_doc();
    let proposed = props::proposed_name(doc, SlotId::Distance).expect("a proposal");
    assert_eq!(proposed.as_str(), "distance");
    assert!(
        doc.var_named("distance").is_none(),
        "proposing stores nothing"
    );

    let var = reads(&session, a);
    let named = session.perform(SessionOp::RenameVar {
        var,
        name: Some(proposed.clone()),
    });
    assert!(named.refusal.is_none(), "{:?}", named.refusal);
    let doc = session.committed_doc();
    assert_eq!(doc.var_name(var), Some(&proposed), "stored on commit");
    assert_eq!(reads(&session, a), var, "naming moves no reader");
    assert_eq!(
        props::proposed_name(session.committed_doc(), SlotId::Distance)
            .expect("a proposal")
            .as_str(),
        "distance_2",
        "the next proposal steps past the name now held"
    );
    let _ = b;
}

/// A declared `d`, defined as `w * 2.0`.
fn with_defined(session: &mut DocSession) -> VarId {
    let declared = session.perform(SessionOp::DeclareVar {
        name: VarName::from_static("d"),
        value: FreeVar::continuous(Dimension::Length, 0.0),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    let d = common::var_of(session.committed_doc(), "d");
    let defined = session.perform(SessionOp::SetVariableText {
        var: d,
        text: "w * 2.0".to_owned(),
    });
    assert!(defined.refusal.is_none(), "{:?}", defined.refusal);
    assert!(session.committed_doc().free(d).is_none(), "d is defined");
    d
}

/// **The range probe reads a defined variable as held, and says it is
/// defined**: not the absent variable's refusal, which it is not.
#[test]
fn the_range_probe_says_a_defined_variable_is_defined() {
    let (mut session, _a, _b, _w, _k) = two_extrudes();
    let d = with_defined(&mut session);
    let refused = session
        .perform(SessionOp::ProbeBounds {
            target: BoundsTarget::Variable { var: d },
        })
        .refusal
        .expect("a defined variable has no value to probe");
    assert!(
        matches!(&refused, Refusal::VariableIsDefined(var) if var.id() == d),
        "{refused:?}"
    );
    let said = refused.to_string();
    assert!(said.starts_with("d is defined by a formula"), "{said}");

    let absent = pncad::document::VarId(0x0123_4567_89ab_cdef);
    let refused = session
        .perform(SessionOp::ProbeBounds {
            target: BoundsTarget::Variable { var: absent },
        })
        .refusal
        .expect("an absent variable refuses");
    assert!(
        matches!(refused, Refusal::NoSuchVariable(var) if var == absent),
        "{refused:?}"
    );
}

/// **The add-variable form's exists-notice reads a defined variable as
/// holding its name**, at its kind's dimension — the declare door
/// refuses the name either way (`EditError::VarNameTaken`).
#[test]
fn the_exists_notice_reads_a_defined_variable_as_holding_its_name() {
    let (mut session, _a, _b, w, _k) = two_extrudes();
    let d = with_defined(&mut session);
    let doc = session.committed_doc();
    assert_eq!(
        props::named_variable(doc, &VarName::from_static("d")),
        Some((d, Dimension::Length))
    );
    assert_eq!(
        props::named_variable(doc, &VarName::from_static("w")),
        Some((w, Dimension::Length))
    );
    assert_eq!(props::named_variable(doc, &VarName::from_static("q")), None);
}
