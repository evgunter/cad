//! **The GUI's variables** (D10, VARIABLES-DESIGN VR2/VR6): a value
//! typed at a slot mints an anonymous variable and is offered the
//! existing variables of equal value and kind; accepting makes the slot
//! read one, declining keeps the typed one distinct; a name is proposed
//! and stored only when committed; and the value doors read a defined
//! variable as held.
// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]

use crate::common;
use editor_core::ExtrudeSide;
use pncad::document::{
    Dimension, Doc, DocEdit, EditError, FreeVar, Node, ProfileProgram, RecipeNodeId, SlotId, VarId,
    VarName,
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
        profile: profile.into(),
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
        name: None,
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

/// **A drag of the typed slot closes its offer**: the drag moves the
/// variable typing minted in place (Q6), and an offer is about the
/// value as typed — never about a dragged one, here equal to `b`'s.
#[test]
fn a_drag_of_the_typed_slot_closes_its_offer() {
    let (mut session, a, _b, w, _k) = two_extrudes();
    typed(&mut session, a, 0.012);
    assert_eq!(offered(&session, a), vec![w]);
    for op in [
        SessionOp::BeginGesture {
            node: a,
            slot: SlotId::Distance,
        },
        SessionOp::PreviewGesture {
            node: a,
            slot: SlotId::Distance,
            value: 0.010,
        },
        SessionOp::CommitGesture {
            node: a,
            slot: SlotId::Distance,
        },
    ] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    }
    assert!(
        offered(&session, a).is_empty(),
        "a dragged value is offered nothing"
    );
}

/// **A move of the typed variable by its own value door closes the
/// offer** — the same rule as a drag, by the other route there is.
#[test]
fn moving_the_typed_variable_closes_its_offer() {
    let (mut session, a, _b, w, _k) = two_extrudes();
    typed(&mut session, a, 0.012);
    assert_eq!(offered(&session, a), vec![w]);
    let minted = reads(&session, a);
    let moved = session.perform(SessionOp::SetVariable {
        var: minted,
        value: SlotValue::Continuous(0.010),
    });
    assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    assert_eq!(reads(&session, a), minted, "moved in place");
    assert!(offered(&session, a).is_empty());
}

/// **An edit elsewhere leaves the offer standing** (choice 2 rejects
/// closing it on any later edit): moving `k` touches neither the slot
/// nor the value typed there.
#[test]
fn an_edit_elsewhere_leaves_the_offer_standing() {
    let (mut session, a, _b, w, k) = two_extrudes();
    typed(&mut session, a, 0.012);
    let moved = session.perform(SessionOp::SetVariable {
        var: k,
        value: SlotValue::Continuous(0.5),
    });
    assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    assert_eq!(moved.committed.len(), 1, "a document edit");
    assert_eq!(offered(&session, a), vec![w]);
}

/// **A value moved away and back is not offered again**: the drag
/// closed the offer for good, and the move back is no typing.
#[test]
fn a_value_moved_back_is_not_offered_again() {
    let (mut session, a, _b, w, _k) = two_extrudes();
    typed(&mut session, a, 0.012);
    assert_eq!(offered(&session, a), vec![w]);
    let minted = reads(&session, a);
    for value in [0.010, 0.012] {
        let moved = session.perform(SessionOp::SetVariable {
            var: minted,
            value: SlotValue::Continuous(value),
        });
        assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    }
    assert!(offered(&session, a).is_empty());
}

/// **An offer is made straight after a typing op, and neither an undo
/// nor a redo makes one**: the redo puts the typed value back, and the
/// offer the undo closed stays closed.
#[test]
fn a_redo_does_not_revive_an_offer() {
    let (mut session, a, _b, w, _k) = two_extrudes();
    typed(&mut session, a, 0.012);
    assert_eq!(offered(&session, a), vec![w]);
    for op in [SessionOp::Undo, SessionOp::Redo] {
        let outcome = session.perform(op);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        assert!(offered(&session, a).is_empty());
    }
}

/// **Equal is equal at the bits** (choice 3): a length holding `-0.0`
/// is not offered to a slot typed `0`, though `-0.0 == 0.0`.
#[test]
fn a_negative_zero_is_not_offered_for_a_typed_zero() {
    let (mut session, a, _b, _w, _k) = two_extrudes();
    for (name, value) in [("z", -0.0), ("o", 0.0)] {
        let declared = session.perform(SessionOp::DeclareVar {
            name: VarName::new(name).expect("a name"),
            value: FreeVar::continuous(Dimension::Length, value),
        });
        assert!(declared.refusal.is_none(), "{name}: {:?}", declared.refusal);
    }
    let z = common::var_of(session.committed_doc(), "z");
    let o = common::var_of(session.committed_doc(), "o");
    typed(&mut session, a, 0.0);
    let named: Vec<VarId> = offered(&session, a)
        .into_iter()
        .filter(|&var| session.committed_doc().var_name(var).is_some())
        .collect();
    assert_eq!(named, vec![o], "+0 is offered and -0 ({z:?}) is not");
}

/// **Only a variable on offer is accepted**: once a retype moved the
/// offer off `w`, accepting `w` is refused, so a stale button cannot
/// join the slot to it.
#[test]
fn accepting_what_is_not_offered_is_refused() {
    let (mut session, a, _b, w, _k) = two_extrudes();
    typed(&mut session, a, 0.012);
    assert_eq!(offered(&session, a), vec![w]);
    typed(&mut session, a, 0.010);
    let minted = reads(&session, a);
    let accepted = session.perform(SessionOp::SetSlotVariable {
        node: a,
        slot: SlotId::Distance,
        var: w,
        name: None,
    });
    assert!(
        matches!(&accepted.refusal, Some(Refusal::NotOffered(var)) if var.id() == w),
        "{:?}",
        accepted.refusal
    );
    assert!(accepted.committed.is_empty());
    assert_eq!(reads(&session, a), minted, "the slot is untouched");
}

/// **Typed text is a typed value too, and a formula is not**: `12 mm`
/// at the slot mints and is offered `w`; `w` at the slot reads `w`
/// itself, and nothing is offered for it — not even `b`'s typed 12 mm,
/// which equals it.
#[test]
fn typed_text_is_offered_and_a_formula_is_not() {
    let (mut session, a, b, w, _k) = two_extrudes();
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
    // `b` holds a typed 12 mm too, so a read of `w` at `a` has an equal
    // variable an offer could name: only the formula keeps it unoffered.
    typed(&mut session, b, 0.012);
    text(&mut session, "w");
    assert_eq!(reads(&session, a), w);
    assert!(
        offered(&session, a).is_empty(),
        "a read of w offers nothing"
    );
}

/// **Accepting an unnamed offer names it, in the same step** (VR2: a
/// variable two slots share has a name): accepted with no name the door
/// refuses `SharedVarNeedsName` and nothing moves; accepted with one, the
/// name and the share land as one history step.
#[test]
fn an_unnamed_offer_is_accepted_under_a_name_in_one_step() {
    let (mut session, a, b, _w, _k) = two_extrudes();
    typed(&mut session, a, 0.010);
    let shared = reads(&session, b);
    assert!(session.committed_doc().var_name(shared).is_none());
    let before = session.history().len();
    let accept = |name| SessionOp::SetSlotVariable {
        node: a,
        slot: SlotId::Distance,
        var: shared,
        name,
    };
    let refused = session.perform(accept(None));
    assert!(
        matches!(
            &refused.refusal,
            Some(Refusal::Edit(edit))
                if matches!(**edit, EditError::SharedVarNeedsName { ref var } if var.id() == shared)
        ),
        "an unnamed offer accepted with no name refuses, naming it: {:?}",
        refused.refusal
    );
    assert_ne!(reads(&session, a), shared, "nothing moved");
    assert_eq!(session.history().len(), before);

    let name = VarName::from_static("depth");
    let accepted = session.perform(accept(Some(name.clone())));
    assert!(accepted.refusal.is_none(), "{:?}", accepted.refusal);
    assert_eq!(reads(&session, a), shared, "a and b read one variable");
    assert_eq!(session.committed_doc().var_name(shared), Some(&name));
    assert_eq!(session.history().len(), before + 1, "one step");
}

/// **Two slots that share a variable are made two by the text door,
/// not by a typed value** (VR2: a shared variable is named, and a slot
/// reading a named variable is driven by it): a value typed at either
/// is refused, naming the variable, and moves nothing; a written
/// quantity set as its text makes it its own.
#[test]
fn a_shared_slot_is_made_its_own_by_its_text() {
    let (mut session, a, b, _w, _k) = two_extrudes();
    typed(&mut session, a, 0.010);
    let shared = reads(&session, b);
    let name = VarName::from_static("depth");
    let accepted = session.perform(SessionOp::SetSlotVariable {
        node: a,
        slot: SlotId::Distance,
        var: shared,
        name: Some(name.clone()),
    });
    assert!(accepted.refusal.is_none(), "{:?}", accepted.refusal);
    assert_eq!(reads(&session, a), shared, "a and b read one variable");

    let refused = session.perform(SessionOp::SetSlot {
        node: a,
        slot: SlotId::Distance,
        value: SlotValue::Continuous(0.020),
    });
    let named: Option<Vec<_>> = match &refused.refusal {
        Some(Refusal::DrivenByExpression { variables, .. }) => {
            Some(variables.iter().map(|var| var.name().cloned()).collect())
        }
        _ => None,
    };
    assert_eq!(
        named,
        Some(vec![Some(name.clone())]),
        "a typed value at a shared slot is refused, naming depth: {:?}",
        refused.refusal
    );
    assert_eq!(reads(&session, a), shared, "nothing moved");

    let own = session.perform(SessionOp::SetSlotExpression {
        node: a,
        slot: SlotId::Distance,
        text: "20 mm".to_owned(),
    });
    assert!(own.refusal.is_none(), "{:?}", own.refusal);
    assert_ne!(reads(&session, a), shared, "a reads a new variable");
    assert_eq!(reads(&session, b), shared, "b keeps the one it read");
    assert_eq!(
        session.committed_doc().free(shared).map(FreeVar::dim),
        Some(Dimension::Length),
        "and it still holds b's depth"
    );
}

/// **A name is stored only on commit, and naming moves no reader**
/// (VR2): the kernel mints none, so the typed variable is unnamed until
/// `RenameVar` lands.
#[test]
fn a_name_is_stored_only_when_committed() {
    let (mut session, a, _b, _w, _k) = two_extrudes();
    let var = reads(&session, a);
    assert!(session.committed_doc().var_name(var).is_none());
    let name = VarName::from_static("depth");
    let named = session.perform(SessionOp::RenameVar {
        var,
        name: Some(name.clone()),
    });
    assert!(named.refusal.is_none(), "{:?}", named.refusal);
    let doc = session.committed_doc();
    assert_eq!(doc.var_name(var), Some(&name), "stored on commit");
    assert_eq!(reads(&session, a), var, "naming moves no reader");
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

    let absent = pncad::document::VarId::new(0, 0x0123_4567_89ab_cdef);
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
