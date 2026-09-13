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
//! agreeing with itself. Its match is exhaustive: a forty-second
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
//!
//! # The other drag's table
//!
//! The session has TWO independent drags and they refuse different
//! sets, so there are two tables and this file checks both.
//! [`SessionOp::permitted_during_free_move`] has two refusals rather
//! than 26 and they have a name, so it is NOT restated here as a
//! second copy of 41 rows: `replaces_the_document` says the property
//! the table encodes — an operation that puts a different document
//! under the session — and
//! `the_free_move_table_refuses_exactly_the_replacement_doors` checks
//! the table against it over the same sample roster. That is a
//! different row from `expected`'s, and a stronger one, because the
//! two sides are not one statement written twice.
//!
//! `no_operation_dissolves_an_in_flight_free_move_in_silence` is the
//! behavioural half, and it asserts the INVARIANT over every operation
//! rather than two rows about the two doors that used to break it: a
//! probe ends because the user ended it, or because a prune reported
//! killing it, or the row fails.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::{len, len3, scl3};
use pncad::document::{
    Alignment, AxisSense, BooleanOp, Dimension, Doc, DocEdit, DocParam, DocumentId, Expr, Frame,
    MateFrame, MatePrimitive, Node, ParamName, ProfileProgram, RecipeNodeId, SitedRef, SlotId,
};
use pncad::geom_core::Tol;
use pncad::prelude::{EntityKind, MM, StableName};
use pncad::select::ContactClass;
use viewer::display::DisplayFault;
use viewer::props::SlotValue;
use viewer::session::{
    BoundsTarget, CancelDoor, DatumSpec, DocSession, FaceSelection, Hovered, PatternRuleSpec,
    Refusal, Selection, SessionOp,
};

/// The number of `SessionOp` variants, which is also the number of
/// samples `every_op` owes. Not a free-standing claim: `expected`'s
/// exhaustive match hands out the indices `0..OP_COUNT`, and
/// `the_table_answers_for_every_op` checks the samples land on each
/// exactly once — so a variant added without a sample fails, and one
/// added without an answer does not compile.
const OP_COUNT: usize = 41;

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
        SessionOp::BeginParamGesture {
            name: param.clone(),
        },
        SessionOp::PreviewGesture {
            node,
            slot: SlotId::Distance,
            value: 0.01,
        },
        SessionOp::CommitGesture {
            node,
            slot: SlotId::Distance,
        },
        SessionOp::PreviewParamGesture {
            name: param.clone(),
            value: 0.01,
        },
        SessionOp::CommitParamGesture { name: param },
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
            instance: node,
            frame: Frame::translation([0.0, 0.0, 0.02]),
        },
        SessionOp::CommitFreeMove { instance: node },
        SessionOp::CancelFreeMove,
        SessionOp::AddMate {
            a: SitedRef::at_mint(face(node)),
            b: SitedRef::at_mint(face(node)),
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
        // The gesture's own driving doors: a guard here would leave a
        // drag with no way to end. Permitted BY THIS TABLE is the
        // whole of what these rows say — the four that name a target
        // are refused `WrongGesture` from inside their own arms when
        // the target is not the open gesture's, which is a question
        // about a payload and not about an operation.
        SessionOp::PreviewGesture { .. } => (11, true),
        SessionOp::CommitGesture { .. } => (12, true),
        SessionOp::PreviewParamGesture { .. } => (13, true),
        SessionOp::CommitParamGesture { .. } => (14, true),
        SessionOp::CancelGesture => (15, true),
        SessionOp::Undo => (16, false),
        SessionOp::Redo => (17, false),
        SessionOp::CancelEvaluation => (18, true),
        SessionOp::Reevaluate => (19, true),
        SessionOp::Open(_) => (20, false),
        // Save writes the COMMITTED history, which a preview is not
        // in. Whether a save under an open drag should be permitted at
        // all is an open question; this row records today's answer and
        // makes a change to it visible.
        SessionOp::Save(_) => (21, true),
        SessionOp::SetInstanceHidden { .. } => (22, true),
        // The free-move gesture is a SECOND drag with its own state
        // and its own in-flight refusal; a value gesture says nothing
        // about it in either direction.
        SessionOp::BeginFreeMove { .. } => (23, true),
        SessionOp::PreviewFreeMove { .. } => (24, true),
        SessionOp::CommitFreeMove { .. } => (25, true),
        SessionOp::CancelFreeMove => (26, true),
        SessionOp::AddMate { .. } => (27, false),
        SessionOp::NewDocument { .. } => (28, false),
        SessionOp::AddDatum { .. } => (29, false),
        SessionOp::AddProfile { .. } => (30, false),
        SessionOp::AddExtrude { .. } => (31, false),
        SessionOp::AddRevolve { .. } => (32, false),
        SessionOp::AddBoolean { .. } => (33, false),
        SessionOp::AddSplit { .. } => (34, false),
        SessionOp::AddTransform { .. } => (35, false),
        SessionOp::AddPattern { .. } => (36, false),
        SessionOp::AddPlacedUnion { .. } => (37, false),
        SessionOp::AddFillet { .. } => (38, false),
        SessionOp::AddChamfer { .. } => (39, false),
        SessionOp::AddInstance { .. } => (40, false),
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

/// **The four `*FreeMove` rows, exercised rather than asserted.**
///
/// The table permits the free-move quartet during a value gesture, so
/// the two drags can be open at once. What makes that sound is one
/// identity: a value gesture's edits leave the recipe's NODE GRAPH
/// alone, and every display predicate is a function of that graph — so
/// the scratch document the view resolves against, the committed one
/// `free_move_check` admits against and the shown one the panel draws
/// the control from all give the same answer, and the prune a
/// gesture's commit runs cannot discard what it is holding.
///
/// **The gesture here drags a node's slots, both kinds.** A pattern
/// over the probed instance carries a continuous `Spacing`
/// (`DocEdit::SetParam`) and a structural `Count`
/// (`DocEdit::SetStructuralParam`) — the two of the three value-gesture
/// edits that write into `doc.nodes` at all, and the ones the identity
/// is actually about. Dragging a document parameter instead would
/// write only `doc.params`, which no display predicate reads, and every
/// assertion below would hold for any implementation of them.
///
/// The pattern also puts the instance UNDER a root rather than at one,
/// so `drawn_targets` has a propagation to resolve rather than a
/// singleton to return.
///
/// Where it goes red: give a value gesture an edit that changes the
/// node graph and the identity block fails outright (`free_move_check`
/// disagrees across the two documents); leave the identity and break
/// the prune instead and the last two blocks fail — the committed probe
/// vanishes, and the in-flight free-move is killed. That kill is
/// REPORTED, in `killed_gesture` and deliberately not in `superseded`,
/// which `review_gui4_r1.rs:823-855` holds in both directions. (This
/// sentence used to cite `:815-819` and to call the silence current
/// behaviour recorded rather than endorsed; the number named a
/// scene-stats assertion several blocks earlier and was already wrong
/// when it was written, and the silence it described was ended by the
/// third withdrawal kind.) Reversing the table's four rows fails the
/// first block, at `BeginFreeMove`.
#[test]
fn a_value_gesture_and_a_free_move_probe_do_not_disturb_each_other() {
    let tol = Tol::witness();
    let bench = common::asm::bench("view7-two-gestures", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    let post = bench.post_a;

    fn perform(session: &mut DocSession, op: SessionOp) -> viewer::session::OpOutcome {
        let outcome = session.perform(op.clone());
        assert!(outcome.refusal.is_none(), "{op:?}: {:?}", outcome.refusal);
        outcome
    }

    // A pattern over the probed instance: the slots a value gesture can
    // open on in an assembly of bare instances, and the reason the
    // instance's display state has a root to propagate to.
    let pattern = perform(
        &mut session,
        SessionOp::AddPattern {
            input: post,
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: scl3([1.0, 0.0, 0.0]),
                spacing: len(0.03),
            },
        },
    );
    assert_eq!(pattern.committed.len(), 1);
    let pattern = *session
        .doc()
        .order()
        .last()
        .expect("the pattern is the last node inserted");
    assert_eq!(
        viewer::display::drawn_targets(session.doc(), post),
        Ok(std::iter::once(pattern).collect()),
        "the probe on the instance is drawn under the pattern root"
    );

    // Both node-writing gesture doors, one after the other.
    for slot in [SlotId::Spacing, SlotId::Count] {
        let drag_to = match slot {
            SlotId::Count => 3.0,
            _ => 0.04,
        };
        let probe = Frame::translation([0.0, 0.0, 0.011]);

        perform(
            &mut session,
            SessionOp::BeginGesture {
                node: pattern,
                slot,
            },
        );
        perform(
            &mut session,
            SessionOp::PreviewGesture {
                node: pattern,
                slot,
                value: drag_to,
            },
        );
        // The gesture really is in flight and really is previewing a
        // DIFFERENT document: without that the identity below is a
        // comparison of one document with itself.
        assert_ne!(
            session.doc(),
            session.committed_doc(),
            "{slot:?}: a preview puts a scratch document on screen"
        );
        assert!(matches!(
            session.perform(SessionOp::Undo).refusal,
            Some(Refusal::GestureInFlight)
        ));

        // The identity, read off the two documents the display layer
        // actually consults — the shown one (the panel's admission
        // test and the view's resolution) and the committed one (the
        // op's admission test).
        assert_eq!(
            viewer::display::free_move_check(session.doc(), post),
            viewer::display::free_move_check(session.committed_doc(), post),
            "{slot:?}: the shown and committed documents admit the same probes"
        );
        assert_eq!(
            viewer::display::drawn_targets(session.doc(), post),
            viewer::display::drawn_targets(session.committed_doc(), post),
            "{slot:?}: the two documents draw the probe on the same roots"
        );

        // A whole free-move gesture, mid-value-gesture, through
        // `perform` — and the view, resolved against the SCRATCH
        // document, puts the previewed frame on the pattern root.
        perform(&mut session, SessionOp::BeginFreeMove { instance: post });
        perform(
            &mut session,
            SessionOp::PreviewFreeMove {
                instance: post,
                frame: probe,
            },
        );
        assert_eq!(
            session.display_view().moved_roots.get(&pattern),
            Some(&probe),
            "{slot:?}: the previewed probe reaches its drawn root under a scratch document"
        );
        perform(&mut session, SessionOp::CommitFreeMove { instance: post });
        assert_eq!(session.display().free_move_of(post), Some(&probe));

        // The value gesture lands its own value over a committed
        // probe: one edit, no supersession, probe intact.
        let outcome = perform(
            &mut session,
            SessionOp::CommitGesture {
                node: pattern,
                slot,
            },
        );
        assert_eq!(
            outcome.committed.len(),
            1,
            "{slot:?}: one edit for the whole drag"
        );
        // The door actually taken, so the claim above is executed
        // rather than described: these are the value-gesture edits
        // that write into `doc.nodes`.
        assert!(
            matches!(
                (&outcome.committed[0], slot),
                (DocEdit::SetParam { .. }, SlotId::Spacing)
                    | (DocEdit::SetStructuralParam { .. }, SlotId::Count)
            ),
            "{slot:?} took an unexpected door: {:?}",
            outcome.committed[0]
        );
        assert!(
            outcome.withdrawn.superseded.is_empty(),
            "{slot:?}: a slot drag supersedes no probe: {:?}",
            outcome.withdrawn.superseded
        );
        assert_eq!(session.display().free_move_of(post), Some(&probe));

        // And the same over an IN-FLIGHT free-move, which the prune
        // kills outright rather than reporting.
        perform(&mut session, SessionOp::BeginFreeMove { instance: post });
        perform(
            &mut session,
            SessionOp::BeginGesture {
                node: pattern,
                slot,
            },
        );
        perform(
            &mut session,
            SessionOp::PreviewGesture {
                node: pattern,
                slot,
                value: drag_to + 1.0,
            },
        );
        perform(
            &mut session,
            SessionOp::CommitGesture {
                node: pattern,
                slot,
            },
        );
        assert_eq!(
            session.display().probing(),
            Some(post),
            "{slot:?}: committing a slot drag left the free-move gesture in flight"
        );
        perform(&mut session, SessionOp::CancelFreeMove);
    }
}

// --- the cancel doors -----------------------------------------------

/// **Which operations cancel a GESTURE**, written down exhaustively so
/// that a forty-second operation cannot join the enum without answering
/// whether the chrome owes it a door.
///
/// The rule ranges over what an operation cancels, NOT over what it is
/// called. [`SessionOp::CancelEvaluation`] is spelled `Cancel` and
/// cancels a RUN: its control is the one beside the spinner that
/// reports the run, and it is available exactly while a run is
/// outstanding rather than while a gesture is. A name-shaped sweep —
/// every variant whose identifier starts `Cancel` — would hand it a
/// gesture door, and would keep agreeing with itself while the door
/// was wrong.
fn cancels_a_gesture(op: &SessionOp) -> bool {
    match op {
        SessionOp::CancelGesture | SessionOp::CancelFreeMove => true,
        SessionOp::Select(_)
        | SessionOp::Hover(_)
        | SessionOp::DeleteNode { .. }
        | SessionOp::SetSlot { .. }
        | SessionOp::ProbeBounds { .. }
        | SessionOp::SetSlotUnit { .. }
        | SessionOp::SetSlotExpression { .. }
        | SessionOp::SetParam { .. }
        | SessionOp::CreateParam { .. }
        | SessionOp::BeginGesture { .. }
        | SessionOp::BeginParamGesture { .. }
        | SessionOp::PreviewGesture { .. }
        | SessionOp::CommitGesture { .. }
        | SessionOp::PreviewParamGesture { .. }
        | SessionOp::CommitParamGesture { .. }
        | SessionOp::Undo
        | SessionOp::Redo
        | SessionOp::CancelEvaluation
        | SessionOp::Reevaluate
        | SessionOp::Open(_)
        | SessionOp::Save(_)
        | SessionOp::SetInstanceHidden { .. }
        | SessionOp::BeginFreeMove { .. }
        | SessionOp::PreviewFreeMove { .. }
        | SessionOp::CommitFreeMove { .. }
        | SessionOp::AddMate { .. }
        | SessionOp::NewDocument { .. }
        | SessionOp::AddDatum { .. }
        | SessionOp::AddProfile { .. }
        | SessionOp::AddExtrude { .. }
        | SessionOp::AddRevolve { .. }
        | SessionOp::AddBoolean { .. }
        | SessionOp::AddSplit { .. }
        | SessionOp::AddTransform { .. }
        | SessionOp::AddPattern { .. }
        | SessionOp::AddPlacedUnion { .. }
        | SessionOp::AddFillet { .. }
        | SessionOp::AddChamfer { .. }
        | SessionOp::AddInstance { .. } => false,
    }
}

/// Same variant, without asking [`SessionOp`] for an equality it does
/// not have: a door's operation carries no payload, so the
/// discriminant is the whole of its identity.
fn same_variant(a: &SessionOp, b: &SessionOp) -> bool {
    core::mem::discriminant(a) == core::mem::discriminant(b)
}

/// **Every gesture cancel has a chrome door, and nothing else does.**
///
/// The population is [`cancels_a_gesture`]'s — a match over `SessionOp`
/// the compiler completes — asked of `every_op`'s samples, which
/// `the_table_answers_for_every_op` holds to one per variant. So this
/// row ranges over the whole enum and not over the two variants its
/// author had in mind: adding a third gesture with a cancel and no
/// door reds here, and so does a door for an operation that cancels no
/// gesture.
///
/// It is also the row that fails if `cancel_doors` loses a door. The
/// stranded-drag row below exercises one door; this one is what says
/// there are exactly as many as there are operations behind them.
#[test]
fn every_gesture_cancel_has_a_chrome_door() {
    let tol = Tol::witness();
    let dir = common::tempdir("view-cancel-door-census");
    let (session, node) = fixture(tol);
    let doors = session.cancel_doors();
    let mut cancels = 0;
    for op in every_op(node, &dir.join("saved.pncad")) {
        let wanted = usize::from(cancels_a_gesture(&op));
        let found = doors
            .iter()
            .filter(|door| same_variant(&door.op, &op))
            .count();
        assert_eq!(found, wanted, "chrome doors for {op:?}");
        cancels += wanted;
    }
    assert_eq!(
        doors.len(),
        cancels,
        "a door with no cancelling operation behind it"
    );
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// **A door that cannot act says the refusal its OWN operation gives.**
///
/// The sentence a disabled control shows is composed from the refusal
/// the operation answers with rather than written beside the button, so
/// the two cannot come to disagree — the defect
/// `work/view/environmental-facts-answer-usable-as-a-bool-with-the-
/// reason-elsewhere.md` is open about one facility over. Both doors,
/// with no gesture of either kind open.
#[test]
fn a_closed_door_says_what_its_own_operation_refuses() {
    let tol = Tol::witness();
    for door in fixture(tol).0.cancel_doors() {
        let CancelDoor { label, op, blocked } = door;
        let blocked = blocked.expect("a fresh session holds no gesture of either kind");
        let (mut session, _) = fixture(tol);
        let refusal = session
            .perform(op)
            .refusal
            .expect("a cancel with no gesture behind it refuses");
        assert_eq!(
            blocked.to_string(),
            refusal.to_string(),
            "{label}: the disabled control's words"
        );
        assert_eq!(
            format!("{blocked:?}"),
            format!("{refusal:?}"),
            "{label}: and the same refusal, not merely the same sentence"
        );
    }
}

/// **Strand the fixture's distance drag**, and assert the strand: the
/// drag's own preview takes the extrude to zero height, the picked
/// face stops resolving, and the panel is handed no row — so the
/// release event that is the drag's only ordinary exit has no field to
/// fire on.
///
/// Two rows continue from here and they ask different things of it:
/// one closes the stranded drag through the chrome's door, the other
/// drags a second field while it is open.
fn strand_the_distance_drag(session: &mut DocSession, extrude: RecipeNodeId) {
    session.pump();
    let index = common::asm::index_of(session);
    let face = index
        .face_at(
            session.evaluation().expect("the inline seam landed"),
            &common::asm::down_at(0.0, 0.0),
        )
        .expect("the pick is not refused")
        .expect("the plate is under a ray straight down at the origin");
    session.perform(SessionOp::Select(Selection::Face(face)));
    assert!(session.standing().live(), "the picked face resolves");
    assert_eq!(
        session.slot_rows().len(),
        1,
        "and the extrude's distance row — the field the drag opens on — is drawn"
    );

    // The drag opens, and its first preview takes the distance to zero.
    assert!(
        session
            .perform(SessionOp::BeginGesture {
                node: extrude,
                slot: SlotId::Distance,
            })
            .refusal
            .is_none(),
        "the drag opens on a literal slot"
    );
    assert!(
        session
            .perform(SessionOp::PreviewGesture {
                node: extrude,
                slot: SlotId::Distance,
                value: 0.0,
            })
            .refusal
            .is_none(),
        "a drag through zero is an ordinary drag"
    );
    session.pump();

    assert!(
        !session.standing().live(),
        "a zero-height extrude has no face for the picked name"
    );
    assert!(
        session.slot_rows().is_empty(),
        "the panel is handed no row. THIS ROW ASSERTS ONLY THAT: that a \
         group absent from the list is not drawn, and so reports no \
         release, is `properties_ui`'s `for group in &groups` and is \
         read rather than executed here"
    );
    assert!(
        matches!(
            session.perform(SessionOp::Undo).refusal,
            Some(Refusal::GestureInFlight)
        ),
        "while the drag is still open: every document move now refuses, \
         naming a remedy the release event can no longer deliver"
    );
}

/// **A slot drag can lose its only exit under the pointer still
/// holding it — traced, not supposed — and the door is what is left.**
///
/// The drag's exit is the release event on the field that opened it, so
/// the exit exists only on a frame the field is drawn. `slot_rows`
/// answers NOTHING for a selection whose standing is not live
/// (`Standing::live`: a face whose name did not resolve is not live),
/// and `pane::properties` draws one row per group it is handed — so on
/// such a frame the field is not there to report the release.
///
/// The drag itself is what kills the standing: every frame it moves
/// submits its scratch document, and a preview that takes the extrude's
/// distance to zero lands an evaluation the picked face does not
/// survive. No second pointer, no relayout and no other operation is
/// involved; `Select` and the feature rows are click-driven and cannot
/// fire under a held pointer, which is why the item that filed this
/// could not trace it through them.
///
/// **What this row establishes and what it does not.** It establishes
/// that the state the release would have to be reported in is a state
/// with no row to report it — the session half, end to end, through the
/// ops the widget emits. The last link, *a group that is not in the
/// list is not drawn and so reports no release*, is
/// `pane::properties_ui`'s `for group in &groups` and this crate has no
/// headless egui harness to execute it (`panel_display.rs` says the
/// same of the field's own wiring). It is read, not run.
///
/// Where it goes red: give `CancelGesture` back to the no-op it would
/// be if `perform` stopped taking the gesture, or take the door out of
/// `cancel_doors`, and the recovery half fails; make a dead standing
/// keep its rows and the first half fails.
#[test]
fn a_drags_own_preview_can_strand_it_and_the_door_closes_it() {
    let tol = Tol::witness();
    let (mut session, extrude) = fixture(tol);
    strand_the_distance_drag(&mut session, extrude);

    // The door, which is drawn whatever the panel is showing.
    let door = session
        .cancel_doors()
        .into_iter()
        .find(|door| same_variant(&door.op, &SessionOp::CancelGesture))
        .expect("the drag's door");
    assert!(
        door.blocked.is_none(),
        "the door is live exactly while there is a gesture to close: {:?}",
        door.blocked
    );
    assert!(
        session.perform(door.op).refusal.is_none(),
        "and it closes the stranded drag"
    );
    session.pump();
    assert!(
        session.standing().live(),
        "after which the cancelled preview is off the screen and the \
         picked face is back"
    );
    assert!(
        session
            .perform(SessionOp::SetSlot {
                node: extrude,
                slot: SlotId::Distance,
                value: SlotValue::of(Dimension::Length, 0.006),
            })
            .refusal
            .is_none(),
        "and the document moves again — this fixture's history is empty, \
         so the op that shows the fence is gone is an edit and not an undo"
    );
}

/// **The free-move door is live exactly while the probe is**, and
/// closing it through the door takes the preview away.
///
/// The value drag's stranding trace above does not carry over to this
/// gesture and this row does not claim it does: the probe's field is
/// drawn off the shown document (`display::is_instance` and
/// `display::free_move_check`), not off the landed evaluation, and a
/// document change while a probe is in flight is pruned rather than
/// stranded. What holds here is the other half of the item — the
/// operation had no emitter at all, so the refusal that tells a reader
/// to *finish the free-move first* named a remedy the chrome did not
/// offer.
#[test]
fn the_free_move_door_is_live_exactly_while_the_probe_is() {
    let tol = Tol::witness();
    let bench = common::asm::bench("view-cancel-door-free-move", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    let post = bench.post_a;

    let door = |session: &DocSession| {
        session
            .cancel_doors()
            .into_iter()
            .find(|door| same_variant(&door.op, &SessionOp::CancelFreeMove))
            .expect("the probe's door")
    };
    assert!(
        door(&session).blocked.is_some(),
        "no probe, no door to open"
    );

    assert!(
        session
            .perform(SessionOp::BeginFreeMove { instance: post })
            .refusal
            .is_none()
    );
    let probe = Frame::translation([0.0, 0.0, 0.011]);
    assert!(
        session
            .perform(SessionOp::PreviewFreeMove {
                instance: post,
                frame: probe,
            })
            .refusal
            .is_none()
    );
    let open = door(&session);
    assert!(
        open.blocked.is_none(),
        "a probe in flight opens the door: {:?}",
        open.blocked
    );
    assert!(session.perform(open.op).refusal.is_none(), "and it closes");
    assert_eq!(session.display().probing(), None, "the probe is gone");
    assert_eq!(
        session.display().free_move_of(post),
        None,
        "and it left no committed placement behind — a cancel is not a commit"
    );
    assert!(
        door(&session).blocked.is_some(),
        "the door is shut again, with the reason the operation gives"
    );
}

/// **The doors reach a user, which is the whole of what this item was
/// filed about**: both operations had zero emitters in the crate, so
/// `perform` held two arms no chrome could reach.
///
/// **The population is reads of `DocSession::cancel_doors` under
/// `crates/viewer/src`, outside the module that declares it**, and that
/// is the rule rather than a search for the operations' own names: the
/// chrome pushes `door.op` and never spells `SessionOp::CancelGesture`,
/// so a name-shaped sweep finds only `perform`'s arms and reports the
/// defect as still open. One read, in the toolbar.
///
/// What it cannot see is whether that read is REACHED — the toolbar is
/// an `egui` closure and this crate has no headless harness for one.
/// This row holds the emitter count against going back to zero, which
/// is the state the item describes; the call site being three lines of
/// a panel drawn on every frame is the rest of it.
#[test]
fn the_cancel_doors_have_a_reader_in_the_chrome() {
    let dir = test_utils::source::crate_dir(env!("CARGO_MANIFEST_DIR"));
    let app = test_utils::source::code_only(
        &std::fs::read_to_string(dir.join("src/app.rs")).expect("src/app.rs"),
    );
    assert_eq!(
        app.matches("cancel_doors()").count(),
        1,
        "the toolbar's read of the cancel doors"
    );
    let session = test_utils::source::code_only(
        &std::fs::read_to_string(dir.join("src/session.rs")).expect("src/session.rs"),
    );
    assert_eq!(
        session.matches("fn cancel_doors").count(),
        1,
        "declared once, so the read above is a read of this door"
    );
}

/// **The free-move table's answer, restated as the PROPERTY it encodes
/// rather than as a second copy of its rows.**
///
/// `expected` above is a hand-written copy of
/// `permitted_during_value_gesture` because that table has 26 refusals
/// with no shorter description than the list itself. The free-move
/// table has two, and they have a name: an operation that REPLACES the
/// document the session is about — as against one that moves it, which
/// a prune answers for by reporting. So this says the name, and
/// `the_free_move_table_refuses_exactly_the_replacement_doors` checks
/// the table against it. A row that disagrees is either a table entry
/// that is wrong or a property that has stopped being the reason, and
/// both are things to find out.
///
/// **Exhaustive on purpose**, like `expected`: a forty-second `SessionOp`
/// does not compile until someone says whether it replaces the
/// document.
fn replaces_the_document(op: &SessionOp) -> bool {
    match op {
        // The two doors that put a different document under the
        // session, dropping the whole of its display state with
        // `DocSession::clear_for_new_document`.
        SessionOp::Open(_) | SessionOp::NewDocument { .. } => true,
        SessionOp::Select(_)
        | SessionOp::Hover(_)
        | SessionOp::DeleteNode { .. }
        | SessionOp::SetSlot { .. }
        | SessionOp::ProbeBounds { .. }
        | SessionOp::SetSlotUnit { .. }
        | SessionOp::SetSlotExpression { .. }
        | SessionOp::SetParam { .. }
        | SessionOp::CreateParam { .. }
        | SessionOp::BeginGesture { .. }
        | SessionOp::BeginParamGesture { .. }
        | SessionOp::PreviewGesture { .. }
        | SessionOp::CommitGesture { .. }
        | SessionOp::PreviewParamGesture { .. }
        | SessionOp::CommitParamGesture { .. }
        | SessionOp::CancelGesture
        | SessionOp::Undo
        | SessionOp::Redo
        | SessionOp::CancelEvaluation
        | SessionOp::Reevaluate
        | SessionOp::Save(_)
        | SessionOp::SetInstanceHidden { .. }
        | SessionOp::BeginFreeMove { .. }
        | SessionOp::PreviewFreeMove { .. }
        | SessionOp::CommitFreeMove { .. }
        | SessionOp::CancelFreeMove
        | SessionOp::AddMate { .. }
        | SessionOp::AddDatum { .. }
        | SessionOp::AddProfile { .. }
        | SessionOp::AddExtrude { .. }
        | SessionOp::AddRevolve { .. }
        | SessionOp::AddBoolean { .. }
        | SessionOp::AddSplit { .. }
        | SessionOp::AddTransform { .. }
        | SessionOp::AddPattern { .. }
        | SessionOp::AddPlacedUnion { .. }
        | SessionOp::AddFillet { .. }
        | SessionOp::AddChamfer { .. }
        | SessionOp::AddInstance { .. } => false,
    }
}

/// The free-move table is exactly the replacement doors, on the same
/// sample roster the value table is checked on — so a forty-second
/// operation is answered for both drags or does not compile.
#[test]
fn the_free_move_table_refuses_exactly_the_replacement_doors() {
    let tol = Tol::witness();
    let (_, node) = fixture(tol);
    let dir = common::tempdir("view-free-move-table");
    for op in every_op(node, &dir.join("saved.pncad")) {
        assert_eq!(
            op.permitted_during_free_move(),
            !replaces_the_document(&op),
            "the free-move table's answer for {op:?}"
        );
    }
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// **No operation dissolves an in-flight free move in silence** — the
/// invariant, over every operation, rather than two rows about the two
/// doors that used to break it.
///
/// A free-move drag can end in exactly three ways and the third is the
/// defect this row closes: the user ends it (`CommitFreeMove`,
/// `CancelFreeMove`); the document stops admitting the instance under
/// it, and the prune says so (`killed_gesture`); or it vanishes with
/// nothing said. `Open` and `NewDocument` were the third, through
/// `clear_for_new_document`, and a report could not have been the fix
/// — a `Withdrawn` names an instance of the OUTGOING document and the
/// only document left to ask about it is the incoming one
/// (`DisplayState::clear` carries that argument). So they refuse.
///
/// **The refusal is `FreeMoveInFlight` and not `GestureInFlight`.** The
/// two drags are different values with different vocabularies, and
/// *"finish the free-move first"* names a door the user has
/// (`CancelFreeMove`, in the toolbar) where the other sentence names a
/// drag they are not holding.
///
/// `BeginFreeMove` is fenced too and NOT by this table: a second begin
/// under an open drag is refused by `DisplayState::begin_free_move`
/// itself, with the same refusal one layer down. The expectation below
/// says so rather than smoothing it over, because a table row added
/// there would be a second spelling of one answer.
///
/// Where it goes red: delete either `perform` check (the two doors stop
/// refusing AND the drag disappears, failing both halves); return the
/// wrong refusal (the first half); or make a permitted operation clear
/// the display state without reporting (the second half, for whichever
/// operation did it).
#[test]
fn no_operation_dissolves_an_in_flight_free_move_in_silence() {
    let tol = Tol::witness();
    let bench = common::asm::bench("view-free-move-doors", tol);
    let dir = common::tempdir("view-free-move-doors-ops");
    let instance = bench.post_a;
    for op in every_op(instance, &dir.join("saved.pncad")) {
        let mut session = common::asm::open_bench(&bench, tol);
        assert!(
            session
                .perform(SessionOp::BeginFreeMove { instance })
                .refusal
                .is_none(),
            "the fixture's probe opens"
        );
        assert!(
            session.display().probing().is_some(),
            "and is in flight before {op:?} runs"
        );

        let outcome = session.perform(op.clone());
        let fenced = matches!(
            outcome.refusal,
            Some(Refusal::Display(DisplayFault::FreeMoveInFlight))
        );
        let begins_a_second_probe = matches!(op, SessionOp::BeginFreeMove { .. });
        assert_eq!(
            fenced,
            replaces_the_document(&op) || begins_a_second_probe,
            "{op:?} refused {:?} with a probe in flight",
            outcome.refusal
        );
        if replaces_the_document(&op) {
            assert!(
                outcome.committed.is_empty(),
                "{op:?} committed while fenced"
            );
            assert!(
                session.display().probing().is_some(),
                "{op:?} was refused, so the drag it refused FOR is still there \
                 — a refusal that dissolved the gesture anyway would be the \
                 defect with a sentence in front of it"
            );
        }

        // The invariant, over the whole roster: the drag is still in
        // flight, or something said where it went.
        let ends_the_drag = matches!(
            op,
            SessionOp::CommitFreeMove { .. } | SessionOp::CancelFreeMove
        );
        assert!(
            session.display().probing().is_some()
                || ends_the_drag
                || outcome.withdrawn.killed_gesture.is_some(),
            "{op:?} ended the drag under the pointer and said nothing: {outcome:?}"
        );
    }
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}

/// The fixture with a SECOND literal-driven extrude and a document
/// parameter beside the first: the two shapes a field that is not the
/// one being dragged can have, and the two doors a value gesture opens
/// through.
fn two_fields(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, ParamName) {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("view-gesture-identity", tol);
    let (doc, profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, first) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: len(0.005),
        },
        tol,
    );
    let (doc, second) = common::inserted(
        &doc,
        Node::Extrude {
            profile,
            distance: len(0.003),
        },
        tol,
    );
    let mut session = DocSession::inline(doc, tol);
    let param = ParamName::new("thickness");
    assert!(
        session
            .perform(SessionOp::CreateParam {
                name: param.clone(),
                value: DocParam::continuous(Dimension::Length, 0.004),
            })
            .refusal
            .is_none(),
        "the fixture's parameter is declared"
    );
    (session, first, second, param)
}

/// One node's committed `Distance`, read the way the panel reads it.
fn committed_distance(
    session: &DocSession,
    node: RecipeNodeId,
) -> Result<SlotValue, viewer::props::SlotFault> {
    viewer::props::slot_rows(session.committed_doc(), node)
        .into_iter()
        .find(|row| row.slot == SlotId::Distance)
        .expect("the extrude's distance row")
        .value
}

/// **A drag that could not open cannot steer the one that did**, at
/// both value-gesture doors.
///
/// The chrome emits a drag as a triple — begin, previews, commit
/// (`widgets::drag_gesture_ops`) — and only the begin is refused while
/// another drag is open (`permitted_during_value_gesture`). The
/// preview and the commit have to be permitted or a drag could never
/// end, so what keeps the second field's numbers out of the first
/// field's slot is that they NAME their field and the open gesture
/// answers for the name.
///
/// The second field is taken twice, because a value gesture has two
/// doors and they are addressed differently: another node's literal
/// slot, and a document parameter.
///
/// Where it goes red: drop the name check in `preview_gesture` and the
/// refused rows turn into previews against the open gesture's slot;
/// drop it in `commit_gesture` and the second field's release commits
/// the first field's edit.
#[test]
fn a_drag_on_another_field_cannot_steer_the_open_one() {
    let tol = Tol::witness();
    let (mut session, first, second, param) = two_fields(tol);

    // The drag that is open, moved once so it has a value to land.
    assert!(
        session
            .perform(SessionOp::BeginGesture {
                node: first,
                slot: SlotId::Distance,
            })
            .refusal
            .is_none()
    );
    assert_eq!(
        session
            .perform(SessionOp::PreviewGesture {
                node: first,
                slot: SlotId::Distance,
                value: 0.011,
            })
            .previewed
            .len(),
        1,
        "the open drag previews its own value"
    );

    // The other node's slot: the whole triple, as the widget emits it.
    assert!(
        matches!(
            session
                .perform(SessionOp::BeginGesture {
                    node: second,
                    slot: SlotId::Distance,
                })
                .refusal,
            Some(Refusal::GestureInFlight)
        ),
        "the second field cannot open a drag of its own"
    );
    let previewed = session.perform(SessionOp::PreviewGesture {
        node: second,
        slot: SlotId::Distance,
        value: 0.012,
    });
    assert!(
        matches!(previewed.refusal, Some(Refusal::WrongGesture)),
        "and its preview names a drag that is not in flight: {:?}",
        previewed.refusal
    );
    assert!(previewed.previewed.is_empty(), "so it previews nothing");
    let committed = session.perform(SessionOp::CommitGesture {
        node: second,
        slot: SlotId::Distance,
    });
    assert!(
        matches!(committed.refusal, Some(Refusal::WrongGesture)),
        "and its release lands nothing: {:?}",
        committed.refusal
    );
    assert!(committed.committed.is_empty());

    // The parameter door, the same three ops in the other spelling.
    assert!(matches!(
        session
            .perform(SessionOp::BeginParamGesture {
                name: param.clone()
            })
            .refusal,
        Some(Refusal::GestureInFlight)
    ));
    assert!(matches!(
        session
            .perform(SessionOp::PreviewParamGesture {
                name: param.clone(),
                value: 0.012,
            })
            .refusal,
        Some(Refusal::WrongGesture)
    ));
    assert!(matches!(
        session
            .perform(SessionOp::CommitParamGesture {
                name: param.clone()
            })
            .refusal,
        Some(Refusal::WrongGesture)
    ));

    // A refused commit is not a release: the drag nobody let go of is
    // still open, still on its own slot, and still carrying its own
    // value.
    let landed = session.perform(SessionOp::CommitGesture {
        node: first,
        slot: SlotId::Distance,
    });
    assert!(landed.refusal.is_none());
    assert_eq!(landed.committed.len(), 1, "one edit for the whole drag");
    assert!(
        matches!(
            landed.committed.first(),
            Some(DocEdit::SetParam { node, slot, .. })
                if *node == first && *slot == SlotId::Distance
        ),
        "and it is the open drag's own slot that moved: {:?}",
        landed.committed.first()
    );
    assert_eq!(
        committed_distance(&session, first),
        Ok(SlotValue::Continuous(0.011)),
        "carrying the value that field was dragged to"
    );
    assert_eq!(
        committed_distance(&session, second),
        Ok(SlotValue::Continuous(0.003)),
        "and the field the user dragged second is where the document left it"
    );
}

/// **The same field dragged again IS the open drag**, which is the
/// difference between naming a gesture by its target and minting a
/// token for it.
///
/// A drag stranded by `strand_the_distance_drag` leaves its field
/// undrawn; when the panel comes back, the natural thing a reader does
/// is drag that field. Its begin is refused — one gesture at a time —
/// and its previews and its release go to the drag that is already
/// open, which is the same slot, against the same base document,
/// because nothing that moves the document is permitted mid-drag. So
/// the field lands the number the user dragged it to and the drag
/// ends.
#[test]
fn the_open_drags_own_field_dragged_again_lands_its_number() {
    let tol = Tol::witness();
    let (mut session, first, _, _) = two_fields(tol);
    assert!(
        session
            .perform(SessionOp::BeginGesture {
                node: first,
                slot: SlotId::Distance,
            })
            .refusal
            .is_none()
    );

    assert!(matches!(
        session
            .perform(SessionOp::BeginGesture {
                node: first,
                slot: SlotId::Distance,
            })
            .refusal,
        Some(Refusal::GestureInFlight)
    ));
    assert!(
        session
            .perform(SessionOp::PreviewGesture {
                node: first,
                slot: SlotId::Distance,
                value: 0.009,
            })
            .refusal
            .is_none(),
        "the second drag's preview names the drag that is open"
    );
    let landed = session.perform(SessionOp::CommitGesture {
        node: first,
        slot: SlotId::Distance,
    });
    assert!(landed.refusal.is_none());
    assert_eq!(landed.committed.len(), 1);
    assert_eq!(
        committed_distance(&session, first),
        Ok(SlotValue::Continuous(0.009)),
    );
    assert!(session.cancel_doors().iter().all(|door| {
        !same_variant(&door.op, &SessionOp::CancelGesture) || door.blocked.is_some()
    }));
}

/// **The free-move quartet's half of the same rule**: a probe
/// operation names the instance it is probing.
///
/// The identity is one node rather than a target, so there is one door
/// rather than two, and the refusal is the display layer's own
/// (`DisplayFault::WrongFreeMove`) because the state it is about is
/// `DisplayState`'s.
///
/// **Reached here through the operation vocabulary and not through the
/// chrome.** The probe's three fields are drawn off the shown document
/// rather than the landed evaluation, so the strand that reaches the
/// value gesture's copy of this defect does not carry over, and no
/// other route to a second probe under an open one has been traced
/// (`work/view/free-move-in-flight-refusal-has-no-reachable-producer.md`
/// owns that question). The rule is owed anyway: `SessionOp` is the
/// crate's API and every driver of it — this suite included — emits
/// these ops directly.
#[test]
fn a_probe_on_another_instance_cannot_steer_the_open_one() {
    let tol = Tol::witness();
    let bench = common::asm::bench("view-free-move-identity", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    let (open, other) = (bench.post_a, bench.post_b);

    assert!(
        session
            .perform(SessionOp::BeginFreeMove { instance: open })
            .refusal
            .is_none()
    );
    let held = Frame::translation([0.0, 0.0, 0.011]);
    assert!(
        session
            .perform(SessionOp::PreviewFreeMove {
                instance: open,
                frame: held,
            })
            .refusal
            .is_none()
    );

    assert!(
        matches!(
            session
                .perform(SessionOp::BeginFreeMove { instance: other })
                .refusal,
            Some(Refusal::Display(DisplayFault::FreeMoveInFlight))
        ),
        "the second instance cannot open a probe of its own"
    );
    assert!(
        matches!(
            session
                .perform(SessionOp::PreviewFreeMove {
                    instance: other,
                    frame: Frame::translation([0.02, 0.0, 0.0]),
                })
                .refusal,
            Some(Refusal::Display(DisplayFault::WrongFreeMove))
        ),
        "and its preview names a probe that is not in flight"
    );
    assert_eq!(
        session.display_view().moved.get(&open),
        Some(&held),
        "so the open probe still shows its own frame"
    );
    assert!(matches!(
        session
            .perform(SessionOp::CommitFreeMove { instance: other })
            .refusal,
        Some(Refusal::Display(DisplayFault::WrongFreeMove))
    ));
    assert_eq!(
        session.display().probing(),
        Some(open),
        "a refused commit leaves the probe it does not name in flight"
    );
    assert_eq!(
        session.display().free_move_of(other),
        None,
        "and nothing landed on the instance that was dragged"
    );
}

/// **The open probe's own instance, driven again, lands its frame** —
/// the probe's half of
/// `the_open_drags_own_field_dragged_again_lands_its_number`, and the
/// reason a probe is named by the instance it is on rather than by a
/// token its begin mints.
///
/// The probe has the stranded-field state the value drag has, reached
/// by a different door: `SessionOp::Select` is permitted mid-probe
/// (`permitted_during_free_move`) and `pane::properties`' `instance_ui`
/// draws the probe row for `selection().node()` alone, so a selection
/// change under an open probe takes the field away with the drag still
/// live. It is the hole
/// `free-move-in-flight-refusal-has-no-reachable-producer` named and
/// left open, and the hand that reaches it is the one that row closed
/// on: every `Select` producer in the chrome is a `clicked()`, and
/// egui answers `clicked()` for a focused widget's Space/Enter and for
/// an AccessKit `Action::Click` with no pointer anywhere.
///
/// The reader's recovery is to select the instance again and drag a
/// box, which is a whole begin/preview/commit batch on a probe that is
/// already open. Its begin is refused — one probe at a time — and its
/// preview and its commit name the instance the open probe is on, so
/// they land the frame the user dragged it to and the probe ends. A
/// token minted per begin would refuse them and strand the reader a
/// second time; an instance accepts them, which is what *one probe per
/// instance, driven by whoever names it* buys.
///
/// Where it goes red: make `begin_free_move` permit a second begin and
/// the recovery opens a fresh probe over the held frame instead of
/// continuing it; make `preview_free_move` or `commit_free_move` refuse
/// a name that matches the open probe and the reader can no longer end
/// the drag from the panel at all.
#[test]
fn the_open_probes_own_instance_driven_again_lands_its_frame() {
    let tol = Tol::witness();
    let bench = common::asm::bench("view-probe-driven-again", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    let (probed, other) = (bench.post_a, bench.post_b);

    assert!(
        session
            .perform(SessionOp::BeginFreeMove { instance: probed })
            .refusal
            .is_none()
    );
    let held = Frame::translation([0.0, 0.0, 0.011]);
    assert!(
        session
            .perform(SessionOp::PreviewFreeMove {
                instance: probed,
                frame: held,
            })
            .refusal
            .is_none()
    );

    // The strand: the selection moves off the probed instance, so the
    // panel stops drawing the row the probe is being driven from.
    assert!(
        session
            .perform(SessionOp::Select(Selection::Node(other)))
            .refusal
            .is_none(),
        "a selection change is permitted under a probe"
    );
    assert_eq!(
        session.display().probing(),
        Some(probed),
        "and it leaves the probe live with its field no longer drawn"
    );
    assert_eq!(
        session.display_view().moved.get(&probed),
        Some(&held),
        "still showing the frame it was dragged to"
    );

    // The recovery: select it again and drive the row a second time.
    assert!(
        session
            .perform(SessionOp::Select(Selection::Node(probed)))
            .refusal
            .is_none()
    );
    assert!(
        matches!(
            session
                .perform(SessionOp::BeginFreeMove { instance: probed })
                .refusal,
            Some(Refusal::Display(DisplayFault::FreeMoveInFlight))
        ),
        "the second batch's begin is refused — one probe at a time"
    );
    let again = Frame::translation([0.0, 0.0, 0.017]);
    assert!(
        session
            .perform(SessionOp::PreviewFreeMove {
                instance: probed,
                frame: again,
            })
            .refusal
            .is_none(),
        "and its preview names the probe that is open"
    );
    let landed = session.perform(SessionOp::CommitFreeMove { instance: probed });
    assert!(landed.refusal.is_none());
    assert!(
        landed.committed.is_empty(),
        "a probe reaches no document edit"
    );
    assert_eq!(
        session.display().free_move_of(probed),
        Some(&again),
        "the instance keeps the frame the second drive left it at"
    );
    assert_eq!(
        session.display().probing(),
        None,
        "and the probe the first drive opened is the one that ended"
    );
    assert_eq!(
        session.display().free_move_of(other),
        None,
        "nothing landed on the instance the selection passed through"
    );
}

/// **The route the two halves meet on**: the drag that stranded
/// itself, and the field a reader drags when the panel comes back.
///
/// `strand_the_distance_drag` leaves a drag open with no pointer
/// behind it and its own field undrawn. Every document move then
/// refuses *"finish the drag first"* — a remedy the toolbar's cancel
/// door delivers and the panel does not — so the reader's other move
/// is to drag something else. That drag's begin is refused and its
/// preview and its release are refused with it, which is the whole of
/// what this row adds to the two above: the state they assert the rule
/// in is a state the chrome can actually be in.
#[test]
fn the_field_dragged_after_a_strand_does_not_land_in_the_stranded_slot() {
    let tol = Tol::witness();
    let (mut session, extrude) = fixture(tol);
    let param = ParamName::new("thickness");
    assert!(
        session
            .perform(SessionOp::CreateParam {
                name: param.clone(),
                value: DocParam::continuous(Dimension::Length, 0.004),
            })
            .refusal
            .is_none()
    );
    strand_the_distance_drag(&mut session, extrude);

    // The parameter row is drawn whatever the selection's standing is
    // — it is the document's, not the selected node's — so it is a
    // field the reader can still reach.
    assert!(
        viewer::props::param_rows(session.doc())
            .iter()
            .any(|row| row.name == param),
        "the parameter the reader drags next is on the panel"
    );
    assert!(matches!(
        session
            .perform(SessionOp::BeginParamGesture {
                name: param.clone()
            })
            .refusal,
        Some(Refusal::GestureInFlight)
    ));
    assert!(matches!(
        session
            .perform(SessionOp::PreviewParamGesture {
                name: param.clone(),
                value: 0.012,
            })
            .refusal,
        Some(Refusal::WrongGesture)
    ));
    assert!(matches!(
        session
            .perform(SessionOp::CommitParamGesture {
                name: param.clone()
            })
            .refusal,
        Some(Refusal::WrongGesture)
    ));
    assert_eq!(
        committed_distance(&session, extrude),
        Ok(SlotValue::Continuous(0.005)),
        "the stranded drag's slot is where the document left it"
    );
    assert_eq!(
        session.history().len(),
        2,
        "and the parameter's declaration is the only edit in the history"
    );
}
