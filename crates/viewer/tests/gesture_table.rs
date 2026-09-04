//! **The mid-gesture policy as one executable table**: what a value
//! gesture — a slot or document-parameter drag — refuses, asserted per
//! operation.
//!
//! The policy used to be 23 copies of one guard spread through
//! `DocSession::perform`'s dispatch, and no row could execute it: a
//! reader answering "is X safe mid-drag" had to walk every arm, and a
//! new operation joined the enum without the question being put. It is
//! now [`SessionOp::permitted_during_value_gesture`], consulted once
//! before dispatch. The rows below do not all deliver the same thing,
//! and reading them as one story overstates each of them, so each one
//! says what it is worth.
//!
//! # What `expected` delivers
//!
//! [`expected`] is a SECOND, hand-written copy of the answers, so an
//! accidental edit to the predicate fails here rather than passing by
//! agreeing with itself. Its match is exhaustive: a fortieth
//! `SessionOp` does not compile until someone writes down whether a
//! drag refuses it, which is the property the table exists to buy.
//! Its index half, checked against `OP_COUNT`, is what makes a MISSING
//! sample fail too — an unasserted variant is the same silence in a
//! different place.
//!
//! **This is the only row here that can catch a WRONG table entry**,
//! and it catches one by disagreeing with a second hand-written copy,
//! not by consulting behaviour. Both copies were written by one author
//! in one commit, so the honest scope is narrower than "the answers are
//! checked": reversing the whole table — every op permitted, in the
//! predicate and in `expected` together — turns five tests red across
//! the viewer suite, which witnesses 20 of the 26 refusals from outside
//! this file. The six with no external witness are
//! [`SessionOp::DeleteNode`], [`SessionOp::ProbeBounds`],
//! [`SessionOp::SetSlotUnit`], [`SessionOp::CreateParam`],
//! [`SessionOp::BeginParamGesture`] and [`SessionOp::AddMate`]; for
//! those, `expected` is the only place the answer is written down
//! rather than a check on a written answer. That is strictly more than
//! the dispatch recorded before the table existed, and it is not the
//! same as an independent confirmation.
//!
//! # What the behavioural rows deliver
//!
//! The table is a claim about `DocSession::perform`, not about a bool,
//! so `every_op_behaves_as_the_table_says` opens a real gesture and
//! performs every operation against it. It compares `perform`'s fencing
//! against the very predicate `perform` reads, so it CANNOT catch a
//! wrong table entry — flip one answer in the predicate alone and this
//! row stays green while `the_table_answers_for_every_op` goes red.
//! What it does deliver is that `perform` consults the table AT ALL,
//! that the refusal it raises is [`Refusal::GestureInFlight`] and not
//! some other, that a fenced op commits nothing, and that no arm has
//! re-added a guard of its own — which is exactly what deleting 23
//! guards put at risk. A permitted op may still refuse for its own
//! reasons (an `Open` of a path that is not there, an `Undo` with no
//! history) — the assertion is about WHICH refusal, never about
//! success. `nothing_is_fenced_when_no_gesture_is_in_flight` is the
//! same shape with the gesture closed.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::{len, len3, scl3};
use pncad::document::{
    Alignment, AxisSense, BooleanOp, Dimension, Doc, DocParam, DocumentId, Expr, Frame, MateFrame,
    MatePrimitive, Node, ParamName, ProfileProgram, RecipeNodeId, SlotId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{EntityKind, MM, StableName};
use pncad::select::ContactClass;
use viewer::props::SlotValue;
use viewer::session::{
    BoundsTarget, DatumSpec, DocSession, FaceSelection, Hovered, PatternRuleSpec, Refusal,
    Selection, SessionOp,
};

/// The number of `SessionOp` variants, which is also the number of
/// samples `every_op` owes. Not a free-standing claim: `expected`'s
/// exhaustive match hands out the indices `0..OP_COUNT`, and
/// `the_table_answers_for_every_op` checks the samples land on each
/// exactly once — so a variant added without a sample fails, and one
/// added without an answer does not compile.
const OP_COUNT: usize = 39;

/// A document with a literal-driven extrude — a slot a gesture can
/// actually open on, which the expression-driven fixture is not.
fn fixture(tol: Tol) -> (DocSession, RecipeNodeId) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("view1b-gesture-table", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, extrude) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: len(0.005),
        },
        tol,
    );
    (DocSession::inline(doc, tol), extrude)
}

/// A well-formed face name. Nothing here resolves it: every op that
/// carries one is either refused before it looks or stores it as-is.
fn face(node: RecipeNodeId) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node,
        path: vec![],
    }
}

/// A seat for the mate door — well-formed and never evaluated here.
fn alignment() -> Alignment {
    Alignment {
        a: MateFrame {
            origin: [0.0; 3],
            axis: [0.0, 0.0, 1.0],
            reference: [1.0, 0.0, 0.0],
        },
        b: MateFrame {
            origin: [0.0; 3],
            axis: [0.0, 0.0, -1.0],
            reference: [1.0, 0.0, 0.0],
        },
        primitive: MatePrimitive::FrameCoincidence,
        sense: AxisSense::Opposed,
        clocking: None,
    }
}

/// One sample of every `SessionOp` variant, in `expected`'s order.
///
/// The values are well-formed and otherwise arbitrary: a refused op
/// never reaches its own validation, and a permitted one is asserted
/// on WHICH refusal it gives, not on succeeding.
fn every_op(node: RecipeNodeId, save_to: &std::path::Path) -> Vec<SessionOp> {
    let param = ParamName::new("thickness");
    vec![
        SessionOp::Select(Selection::Node(node)),
        SessionOp::Hover(Some(Hovered::Face(FaceSelection {
            name: face(node),
            node,
            body: 0,
        }))),
        SessionOp::DeleteNode { node },
        SessionOp::SetSlot {
            node,
            slot: SlotId::Distance,
            value: SlotValue::Continuous(0.02),
        },
        SessionOp::ProbeBounds {
            target: BoundsTarget::Slot {
                node,
                slot: SlotId::Distance,
            },
        },
        SessionOp::SetSlotUnit {
            node,
            slot: SlotId::Distance,
            unit: MM.def(),
        },
        SessionOp::SetSlotExpression {
            node,
            slot: SlotId::Distance,
            text: "1.0 m".to_owned(),
        },
        SessionOp::SetParam {
            name: param.clone(),
            value: SlotValue::Continuous(0.02),
        },
        SessionOp::CreateParam {
            name: param.clone(),
            value: DocParam::continuous(Dimension::Length, 0.005),
        },
        SessionOp::BeginGesture {
            node,
            slot: SlotId::Distance,
        },
        SessionOp::BeginParamGesture { name: param },
        SessionOp::PreviewGesture { value: 0.01 },
        SessionOp::CommitGesture,
        SessionOp::CancelGesture,
        SessionOp::Undo,
        SessionOp::Redo,
        SessionOp::CancelEvaluation,
        SessionOp::Reevaluate,
        SessionOp::Open(save_to.with_file_name("no-such-document.pncad")),
        SessionOp::Save(save_to.to_path_buf()),
        SessionOp::SetInstanceHidden {
            instance: node,
            hidden: true,
        },
        SessionOp::BeginFreeMove { instance: node },
        SessionOp::PreviewFreeMove {
            frame: Frame::translation([0.0, 0.0, 0.02]),
        },
        SessionOp::CommitFreeMove,
        SessionOp::CancelFreeMove,
        SessionOp::AddMate {
            a: face(node),
            b: face(node),
            class: ContactClass::Rest,
            alignment: alignment(),
        },
        SessionOp::NewDocument {
            name: "view1b-fresh".to_owned(),
        },
        SessionOp::AddDatum {
            datum: DatumSpec::Frame {
                origin: len3([0.0; 3]),
                u: scl3([1.0, 0.0, 0.0]),
                v: scl3([0.0, 1.0, 0.0]),
            },
        },
        SessionOp::AddProfile {
            plane: node,
            loops: vec![],
        },
        SessionOp::AddExtrude {
            profile: node,
            distance: len(0.01),
        },
        SessionOp::AddRevolve {
            profile: node,
            axis: node,
            angle: Expr::literal(1.0, Dimension::Angle).expect("a finite angle"),
        },
        SessionOp::AddBoolean {
            op: BooleanOp::Union,
            a: node,
            b: node,
        },
        SessionOp::AddSplit {
            target: node,
            tool: node,
        },
        SessionOp::AddTransform {
            input: node,
            translation: len3([0.0; 3]),
            rotation_axis: scl3([0.0, 0.0, 1.0]),
            rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("a finite angle"),
        },
        SessionOp::AddPattern {
            input: node,
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
        SessionOp::AddPlacedUnion {
            input: node,
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.05),
            },
        },
        SessionOp::AddFillet {
            target: node,
            radius: len(0.001),
            selection: vec![face(node)],
        },
        SessionOp::AddChamfer {
            target: node,
            distance: len(0.001),
            selection: vec![face(node)],
        },
        SessionOp::AddInstance {
            id: DocumentId::derive("view1b-no-such-part"),
        },
    ]
}

/// The answers, restated by hand: `(index, permitted mid-drag)`.
///
/// **Exhaustive on purpose.** A new `SessionOp` fails to compile here
/// until its row is written, which is the whole reason the policy is a
/// table rather than a scattering of guards.
fn expected(op: &SessionOp) -> (usize, bool) {
    match op {
        // Layer-3 moves: neither the document nor the history is
        // touched, so a drag has nothing to protect from them.
        SessionOp::Select(_) => (0, true),
        SessionOp::Hover(_) => (1, true),
        SessionOp::DeleteNode { .. } => (2, false),
        SessionOp::SetSlot { .. } => (3, false),
        SessionOp::ProbeBounds { .. } => (4, false),
        SessionOp::SetSlotUnit { .. } => (5, false),
        SessionOp::SetSlotExpression { .. } => (6, false),
        SessionOp::SetParam { .. } => (7, false),
        SessionOp::CreateParam { .. } => (8, false),
        SessionOp::BeginGesture { .. } => (9, false),
        SessionOp::BeginParamGesture { .. } => (10, false),
        // The gesture's own three doors: a guard here would leave a
        // drag with no way to end.
        SessionOp::PreviewGesture { .. } => (11, true),
        SessionOp::CommitGesture => (12, true),
        SessionOp::CancelGesture => (13, true),
        SessionOp::Undo => (14, false),
        SessionOp::Redo => (15, false),
        SessionOp::CancelEvaluation => (16, true),
        SessionOp::Reevaluate => (17, true),
        SessionOp::Open(_) => (18, false),
        // Save writes the COMMITTED history, which a preview is not
        // in. Whether a save under an open drag should be permitted at
        // all is an open question; this row records today's answer and
        // makes a change to it visible.
        SessionOp::Save(_) => (19, true),
        SessionOp::SetInstanceHidden { .. } => (20, true),
        // The free-move gesture is a SECOND drag with its own state
        // and its own in-flight refusal; a value gesture says nothing
        // about it in either direction.
        SessionOp::BeginFreeMove { .. } => (21, true),
        SessionOp::PreviewFreeMove { .. } => (22, true),
        SessionOp::CommitFreeMove => (23, true),
        SessionOp::CancelFreeMove => (24, true),
        SessionOp::AddMate { .. } => (25, false),
        SessionOp::NewDocument { .. } => (26, false),
        SessionOp::AddDatum { .. } => (27, false),
        SessionOp::AddProfile { .. } => (28, false),
        SessionOp::AddExtrude { .. } => (29, false),
        SessionOp::AddRevolve { .. } => (30, false),
        SessionOp::AddBoolean { .. } => (31, false),
        SessionOp::AddSplit { .. } => (32, false),
        SessionOp::AddTransform { .. } => (33, false),
        SessionOp::AddPattern { .. } => (34, false),
        SessionOp::AddPlacedUnion { .. } => (35, false),
        SessionOp::AddFillet { .. } => (36, false),
        SessionOp::AddChamfer { .. } => (37, false),
        SessionOp::AddInstance { .. } => (38, false),
    }
}

#[test]
fn the_table_answers_for_every_op() {
    let tol = Tol::witness();
    let (_, node) = fixture(tol);
    let dir = common::tempdir("view1b-gesture-table");
    let ops = every_op(node, &dir.join("saved.pncad"));
    let mut seen = [false; OP_COUNT];
    for op in &ops {
        let (index, want) = expected(op);
        assert!(!seen[index], "two samples for index {index}: {op:?}");
        seen[index] = true;
        assert_eq!(
            op.permitted_during_value_gesture(),
            want,
            "the table's answer for {op:?}"
        );
    }
    let missing: Vec<usize> = (0..OP_COUNT).filter(|i| !seen[*i]).collect();
    assert!(missing.is_empty(), "variants with no sample: {missing:?}");
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// **That `perform` consults the table, not that the table is right.**
/// Every op runs against a real in-flight gesture and the fencing is
/// compared with the predicate `perform` itself reads, so a wrong entry
/// agrees with itself here; a re-added arm-level guard, or a `perform`
/// that stopped consulting the table, does not.
#[test]
fn every_op_behaves_as_the_table_says() {
    let tol = Tol::witness();
    let dir = common::tempdir("view1b-gesture-behaviour");
    let (_, node) = fixture(tol);
    for op in every_op(node, &dir.join("saved.pncad")) {
        let (mut session, extrude) = fixture(tol);
        assert!(
            session
                .perform(SessionOp::BeginGesture {
                    node: extrude,
                    slot: SlotId::Distance,
                })
                .refusal
                .is_none(),
            "the fixture's gesture opens"
        );
        let outcome = session.perform(op.clone());
        let fenced = matches!(outcome.refusal, Some(Refusal::GestureInFlight));
        assert_eq!(
            fenced,
            !op.permitted_during_value_gesture(),
            "{op:?} refused {:?} with a gesture in flight",
            outcome.refusal
        );
        if fenced {
            assert!(
                outcome.committed.is_empty(),
                "{op:?} committed while fenced"
            );
        }
    }
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// The other half of "unchanged behaviour": with no gesture open, the
/// in-flight refusal is unreachable for every operation.
#[test]
fn nothing_is_fenced_when_no_gesture_is_in_flight() {
    let tol = Tol::witness();
    let dir = common::tempdir("view1b-gesture-absent");
    let (_, node) = fixture(tol);
    for op in every_op(node, &dir.join("saved.pncad")) {
        let (mut session, _) = fixture(tol);
        let outcome = session.perform(op.clone());
        assert!(
            !matches!(outcome.refusal, Some(Refusal::GestureInFlight)),
            "{op:?} claimed a gesture that is not in flight"
        );
    }
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}
