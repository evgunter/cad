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
//!
//! # The other drag's table
//!
//! The session has TWO independent drags and they refuse different
//! sets, so there are two tables and this file checks both.
//! [`SessionOp::permitted_during_free_move`] has two refusals rather
//! than 26 and they have a name, so it is NOT restated here as a
//! second copy of 39 rows: `replaces_the_document` says the property
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
        perform(&mut session, SessionOp::PreviewGesture { value: drag_to });
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
        perform(&mut session, SessionOp::PreviewFreeMove { frame: probe });
        assert_eq!(
            session.display_view().moved_roots.get(&pattern),
            Some(&probe),
            "{slot:?}: the previewed probe reaches its drawn root under a scratch document"
        );
        perform(&mut session, SessionOp::CommitFreeMove);
        assert_eq!(session.display().free_move_of(post), Some(&probe));

        // The value gesture lands its own value over a committed
        // probe: one edit, no supersession, probe intact.
        let outcome = perform(&mut session, SessionOp::CommitGesture);
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
                value: drag_to + 1.0,
            },
        );
        perform(&mut session, SessionOp::CommitGesture);
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
/// that a fortieth operation cannot join the enum without answering
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
        | SessionOp::CommitGesture
        | SessionOp::Undo
        | SessionOp::Redo
        | SessionOp::CancelEvaluation
        | SessionOp::Reevaluate
        | SessionOp::Open(_)
        | SessionOp::Save(_)
        | SessionOp::SetInstanceHidden { .. }
        | SessionOp::BeginFreeMove { .. }
        | SessionOp::PreviewFreeMove { .. }
        | SessionOp::CommitFreeMove
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
    session.pump();
    let index = common::asm::index_of(&session);
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
            .perform(SessionOp::PreviewGesture { value: 0.0 })
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
            .perform(SessionOp::PreviewFreeMove { frame: probe })
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
/// **Exhaustive on purpose**, like `expected`: a fortieth `SessionOp`
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
        | SessionOp::CommitGesture
        | SessionOp::CancelGesture
        | SessionOp::Undo
        | SessionOp::Redo
        | SessionOp::CancelEvaluation
        | SessionOp::Reevaluate
        | SessionOp::Save(_)
        | SessionOp::SetInstanceHidden { .. }
        | SessionOp::BeginFreeMove { .. }
        | SessionOp::PreviewFreeMove { .. }
        | SessionOp::CommitFreeMove
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
/// sample roster the value table is checked on — so a fortieth
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
        let ends_the_drag = matches!(op, SessionOp::CommitFreeMove | SessionOp::CancelFreeMove);
        assert!(
            session.display().probing().is_some()
                || ends_the_drag
                || outcome.withdrawn.killed_gesture.is_some(),
            "{op:?} ended the drag under the pointer and said nothing: {outcome:?}"
        );
    }
    std::fs::remove_dir_all(&dir).expect("the fixture directory is removable");
}
