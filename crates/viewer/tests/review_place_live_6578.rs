//! Review probes for PR 6578 (`placing-review-6578`): the live
//! placement editor against the profile editor's states, and Place's
//! second placement. Each row states what it shows; a row that pins a
//! divergence passes while the divergence stands.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::document::{Coord, Doc, Node, ProfileProgram, RecipeNodeId, TargetAxis};
use pncad::geom_core::Tol;
use viewer::matetool::{MateTool, MateToolEvent};
use viewer::session::{DocSession, Refusal, Selection, SessionOp};
use viewer::tools::{ToolKind, Tools};

const SMALL: [f64; 3] = [0.02, 0.03, 0.01];
const WIDE: [f64; 3] = [0.04, 0.05, 0.005];
const WIDE_TOP: [f64; 2] = [0.015, 0.02];

fn session(label: &str, tol: Tol) -> DocSession {
    common::inline_without_door(Doc::empty_derived(label, tol), tol)
}

#[allow(clippy::result_large_err)]
fn offered(session: &DocSession, tool: &mut MateTool) -> Option<SessionOp> {
    let (doc, ev) = session.landed_pair().expect("landed");
    tool.offer(doc, ev, session.tol(), |op| session.bundle_door(op))
}

fn landed(session: &mut DocSession, tools: &mut Tools, op: SessionOp) -> bool {
    let outcome = session.perform(op.clone());
    session.pump();
    let committed = outcome.refusal.is_none();
    tools.answered(&op, outcome.made, committed, session.committed_doc());
    committed
}

fn face_at(session: &DocSession, at: [f64; 2]) -> Selection {
    let index = common::asm::index_of(session);
    Selection::Face(common::displayed_face_at(
        session,
        &index,
        &common::down_at(at[0], at[1]),
    ))
}

fn slide_of(doc: &Doc<ProfileProgram>, placement: RecipeNodeId, axis: TargetAxis) -> f64 {
    let Some(Node::Place { constraints, .. }) = doc.node(placement).map(|n| n.written(doc)) else {
        panic!("a placement")
    };
    constraints
        .iter()
        .filter_map(|c| c.value())
        .find(|v| v.coord == Coord::slide(axis))
        .map_or(f64::NAN, |v| {
            if v.by.is_zero() {
                0.0
            } else {
                v.by.literal_value().expect("literal")
            }
        })
}

fn opened(label: &str) -> (DocSession, Tools, RecipeNodeId) {
    let tol = Tol::witness();
    let mut session = session(label, tol);
    let small = common::xy_box_in(&mut session, SMALL);
    let _wide = common::xy_box_in(&mut session, WIDE);
    session.pump();
    let (tool, copy) = common::place_and_edit(&mut session, small.into());
    let mut tools = Tools::new();
    tools.open(ToolKind::Mate);
    *tools.mate_mut().expect("open") = tool;
    (session, tools, copy)
}

/// Hold a flush across spaces in the open editor: refused, pending.
fn hold_flush(session: &DocSession, tools: &mut Tools) {
    let Selection::Face(top) = face_at(session, [0.0, 0.0]) else {
        unreachable!()
    };
    let Selection::Face(wide_top) = face_at(session, WIDE_TOP) else {
        unreachable!()
    };
    let tool = tools.mate_mut().expect("open");
    assert!(tool.pick(session.doc(), top).is_none());
    tool.set_sense(0, false);
    assert!(tool.pick(session.doc(), wide_top).is_none());
    assert!(offered(session, tool).is_none(), "the door refuses");
    assert!(tool.pending().is_some(), "held");
}

/// **Undo with a held draft steps the history, and the draft goes.**
/// The profile editor's Undo takes back a held draft first
/// (`Drafts::undo_pending`, `ViewerApp::perform_batch`); nothing asks
/// the placement editor, so the Undo takes back the last committed
/// change of the placement and the held flush is reloaded away.
#[test]
fn probe_undo_with_a_held_mate_draft_steps_history_and_drops_the_draft() {
    let (mut session, mut tools, copy) = opened("probe-undo-held");
    let tool = tools.mate_mut().expect("open");
    tool.type_value(Coord::slide(TargetAxis::X), common::len(0.01));
    let op = offered(&session, tool).expect("passes");
    assert!(landed(&mut session, &mut tools, op));
    assert_eq!(slide_of(session.committed_doc(), copy, TargetAxis::X), 0.01);
    hold_flush(&session, &mut tools);
    let undo = session.perform(SessionOp::Undo);
    assert!(undo.refusal.is_none());
    session.pump();
    assert_eq!(
        slide_of(session.committed_doc(), copy, TargetAxis::X),
        0.0,
        "the undo took back the committed x, not the held draft"
    );
    let events = tools.mate_mut().expect("open").reconcile(
        session.committed_doc(),
        session.landed_pair().expect("landed"),
    );
    assert!(
        events
            .iter()
            .any(|e| matches!(e, MateToolEvent::Reloaded { .. })),
        "{events:?}"
    );
    let tool = tools.mate().expect("open");
    assert!(tool.rows().is_empty(), "the held flush is gone");
    assert!(tool.pending().is_none());
}

/// **A non-pick change made while the last change is landing is
/// dropped without a word.** `MateTool::changed` keeps `Landing`
/// ("only a pick can be"), and `landed` reloads the bundle, so a value
/// typed between the offer and its answer neither lands nor is held.
#[test]
fn probe_a_value_typed_while_landing_is_dropped_silently() {
    let (mut session, mut tools, copy) = opened("probe-landing-typed");
    let tool = tools.mate_mut().expect("open");
    tool.type_value(Coord::slide(TargetAxis::X), common::len(0.01));
    let op = offered(&session, tool).expect("passes");
    tool.type_value(Coord::slide(TargetAxis::Y), common::len(0.02));
    assert!(tool.landing().is_some(), "still landing");
    assert!(landed(&mut session, &mut tools, op));
    let tool = tools.mate_mut().expect("open");
    assert!(tool.typed().is_empty(), "the y typed is gone");
    assert!(tool.pending().is_none(), "and not held");
    assert!(offered(&session, tool).is_none(), "and never offered");
    assert_eq!(slide_of(session.committed_doc(), copy, TargetAxis::Y), 0.0);
}

/// **An undo in the batch after the change lands takes it back, with a
/// redo left**: the profile editor drops both ops instead
/// (`perform_batch`), so its change never enters the history.
#[test]
fn probe_an_undo_in_the_same_batch_leaves_a_redo() {
    let (mut session, mut tools, copy) = opened("probe-undo-batch");
    let states = session.history().len();
    let tool = tools.mate_mut().expect("open");
    tool.type_value(Coord::slide(TargetAxis::X), common::len(0.01));
    let op = offered(&session, tool).expect("passes");
    assert!(landed(&mut session, &mut tools, op));
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    session.pump();
    assert_eq!(session.history().len(), states + 1, "the change is a state");
    assert!(session.history().can_redo(), "a redo of it remains");
    assert_eq!(slide_of(session.committed_doc(), copy, TargetAxis::X), 0.0);
}

/// **An undo ahead of the change in the batch is the `BundleStale`
/// case**: the `SetBundle` formed over the bundle the undo took away is
/// refused, so judgement (a) has a live reader.
#[test]
fn probe_an_undo_ahead_of_the_change_is_refused_stale() {
    let (mut session, mut tools, _copy) = opened("probe-undo-ahead");
    let tool = tools.mate_mut().expect("open");
    tool.type_value(Coord::slide(TargetAxis::X), common::len(0.01));
    let first = offered(&session, tool).expect("passes");
    assert!(landed(&mut session, &mut tools, first));
    let tool = tools.mate_mut().expect("open");
    tool.type_value(Coord::slide(TargetAxis::Y), common::len(0.01));
    let second = offered(&session, tool).expect("passes");
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    let outcome = session.perform(second.clone());
    assert!(
        matches!(outcome.refusal, Some(Refusal::BundleStale { .. })),
        "{:?}",
        outcome.refusal
    );
    tools.answered(&second, None, false, session.committed_doc());
    assert!(tools.mate().expect("open").landing().is_none());
}

/// **What the second placement is**: drawn, outside the product, and
/// neither placement marks the body placed (`world::is_placed`).
#[test]
fn probe_the_second_placement_is_drawn_but_the_body_is_not_marked_placed() {
    let tol = Tol::witness();
    let mut session = session("probe-second", tol);
    let body = common::xy_box_in(&mut session, SMALL);
    session.pump();
    let first = common::placed_by(&mut session, SessionOp::Place { body: body.into() });
    let second = common::placed_by(&mut session, SessionOp::Place { body: body.into() });
    let doc = session.committed_doc();
    let drawn = viewer::world::drawn_nodes(doc);
    eprintln!("first {first:?} second {second:?} drawn {drawn:?}");
    eprintln!("product reads {:?}", viewer::world::product_reads(doc));
    eprintln!(
        "is_placed {} placements_of {:?}",
        viewer::world::is_placed(doc, body),
        viewer::world::placements_of(doc, body)
    );
    assert!(drawn.contains(&first) && drawn.contains(&second));
    assert!(
        !viewer::world::is_placed(doc, body),
        "the badge misses Place"
    );
}
