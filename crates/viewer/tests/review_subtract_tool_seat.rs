//! **Review rows for the boolean tool's subtraction tool list** (PR
//! 7426): the first pick is the body kept and every later pick a tool;
//! a vanished tool drops out, undo/redo and a re-pick of the kept body
//! do what the seats rule says; and the held-picks line says ", every
//! member" at a `Bodies` seat only.
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::len;
use pncad::document::{BodyRead, Doc, Formula, RecipeNodeId, Speaker};
use pncad::geom_core::Tol;
use pncad::topo::BooleanOp;
use viewer::combine::BooleanTool;
use viewer::seats::{Seat, SeatEvent};
use viewer::session::{BooleanSpec, DocSession, PatternRuleSpec, SessionOp};

const PLATE: [f64; 3] = [0.08, 0.04, 0.01];
const HOLE: [f64; 3] = [0.01, 0.01, 0.03];
const HOLE_AT: [f64; 3] = [-0.02, 0.0, -0.01];

fn plate_and_holes(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let mut session =
        common::inline_without_door(Doc::empty_derived("review-subtract-tool-seat", tol), tol);
    let plate = common::xy_box_in(&mut session, PLATE);
    let raw = common::xy_box_in(&mut session, HOLE);
    let (doc, hole) = crate::fixture::slid(session.committed_doc().clone(), plate, raw, HOLE_AT);
    let mut session = common::inline_without_door(doc, tol);
    let holes = common::session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: hole.into(),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: viewer::PatternDirection::Axis(pncad::document::TargetAxis::X),
                spacing: len(0.04),
            },
        },
    );
    (session, plate, hole, holes)
}

/// **The family picked whole at the body kept is not said as "every
/// member"** (it is a `Body` seat), while at the tool seat it is. The
/// row pins what the line says at each: red if the kept body's line
/// claims ", every member", or if the tool's stops saying it.
#[test]
fn every_member_is_said_at_the_tool_seat_only() {
    let tol = Tol::witness();
    let (session, plate, _, holes) = plate_and_holes(tol);
    let doc = session.committed_doc().clone();
    let mut tool = BooleanTool::new();
    tool.set_operation(BooleanOp::Subtract);
    tool.pick(&doc, holes);
    tool.pick(&doc, plate);
    let line = tool.line(Speaker::of(&doc));
    eprintln!("family at the kept body: {line}");
    assert!(
        !line.contains("every member"),
        "the kept body is a Body seat: {line}"
    );
    tool.clear();
    tool.pick(&doc, plate);
    tool.pick(&doc, holes);
    let line = tool.line(Speaker::of(&doc));
    assert!(line.contains(", every member"), "the tool seat: {line}");
    // A union's member seat says it too.
    let mut union = BooleanTool::new();
    union.pick(&doc, holes);
    let line = union.line(Speaker::of(&doc));
    assert!(line.contains(", every member"), "a member seat: {line}");
}

/// **Removing one of several tools drops that tool alone**, closing up
/// the list and keeping the body kept; the labels follow the count
/// (`body removed 1`, `body removed 2`, then `body removed` alone).
#[test]
fn a_vanished_tool_among_several_drops_alone_and_the_labels_follow() {
    let tol = Tol::witness();
    let (mut session, plate, hole, holes) = plate_and_holes(tol);
    let raw = common::xy_box_in(&mut session, HOLE);
    let doc = session.committed_doc().clone();
    let mut tool = BooleanTool::new();
    tool.set_operation(BooleanOp::Subtract);
    tool.pick(&doc, plate);
    tool.pick(&doc, raw);
    tool.pick_at(&doc, holes, Some(1));
    assert_eq!(tool.picks(), [plate, raw, holes]);
    let line = tool.line(Speaker::of(&doc));
    assert!(
        line.contains("body removed 1:") && line.contains("body removed 2:"),
        "{line}"
    );
    assert!(
        session
            .perform(SessionOp::DeleteNode { node: raw })
            .refusal
            .is_none()
    );
    let events = tool.reconcile(session.committed_doc());
    assert!(
        matches!(
            events.as_slice(),
            [SeatEvent::PickLost { seat: Seat::SubtractTool, node }] if node.id() == raw
        ),
        "{events:?}"
    );
    assert_eq!(tool.picks(), [plate, holes]);
    tool.respeak(session.committed_doc());
    let line = tool.line(Speaker::of(session.committed_doc()));
    assert!(
        !line.contains("body removed 1") && line.contains("body removed"),
        "one tool left is called by its role alone: {line}"
    );
    let spec = tool.spec().expect("seated");
    assert_eq!(
        spec,
        BooleanSpec::Subtract {
            from: plate.into(),
            tool: vec![BodyRead::indexed(holes, vec![Formula::count(1)])],
        },
        "the member read survives its neighbour's drop"
    );
    let _ = hole;
}

/// **Undo brings a tool's node back but not the pick; redo of the
/// subtract it authored restores the subtract's tools**; and a vanished
/// body kept empties its seat without promoting a tool — the next pick
/// refills the body kept.
#[test]
fn undo_redo_and_a_vanished_kept_body() {
    let tol = Tol::witness();
    let (mut session, plate, _, holes) = plate_and_holes(tol);
    let raw = common::xy_box_in(&mut session, HOLE);
    let doc = session.committed_doc().clone();
    let mut tool = BooleanTool::new();
    tool.set_operation(BooleanOp::Subtract);
    tool.pick(&doc, raw);
    tool.pick(&doc, holes);
    // The body kept is `raw`; delete it, and the tool stays a tool.
    assert!(
        session
            .perform(SessionOp::DeleteNode { node: raw })
            .refusal
            .is_none()
    );
    let events = tool.reconcile(session.committed_doc());
    assert!(
        matches!(
            events.as_slice(),
            [SeatEvent::PickLost { seat: Seat::SubtractFrom, node }] if node.id() == raw
        ),
        "{events:?}"
    );
    assert_eq!(tool.picks(), [holes], "the tool is not promoted");
    assert!(matches!(
        tool.spec(),
        Err(viewer::seats::SeatError::Empty {
            seat: Seat::SubtractFrom
        })
    ));
    // Undo the delete: the node is back, and the next pick fills the
    // open body kept rather than appending a tool.
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(session.committed_doc().node(raw).is_some());
    tool.pick(session.committed_doc(), plate);
    let spec = tool.spec().expect("seated");
    assert_eq!(
        spec,
        BooleanSpec::Subtract {
            from: plate.into(),
            tool: vec![holes.into()],
        },
        "the open body kept is refilled first"
    );
    // Commit it, undo it, redo it: the subtract keeps its tool list.
    let made = common::session_insert(&mut session, SessionOp::AddBoolean { spec });
    let node = |s: &DocSession| s.committed_doc().node(made).cloned();
    let before = node(&session).expect("committed");
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(node(&session).is_none());
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    assert_eq!(node(&session), Some(before), "redo restores the tool list");
}

/// **A pattern whose copy is the body kept, cut by the pattern whole**:
/// `subtract(holes[0], holes)` takes every member, its own copy among
/// them, so it is the typed empty body; and `subtract(holes[0],
/// holes[1])` keeps copy 0 whole (the copies are apart).
#[test]
fn a_copy_kept_and_its_pattern_as_the_tool() {
    let tol = Tol::witness();
    let (mut session, _, _, holes) = plate_and_holes(tol);
    let doc = session.committed_doc().clone();
    let mut tool = BooleanTool::new();
    tool.set_operation(BooleanOp::Subtract);
    tool.pick_at(&doc, holes, Some(0));
    tool.pick(&doc, holes);
    let line = tool.line(Speaker::of(&doc));
    eprintln!("copy kept, pattern removed: {line}");
    let spec = tool.spec().expect("seated");
    assert_eq!(
        spec,
        BooleanSpec::Subtract {
            from: BodyRead::indexed(holes, vec![Formula::count(0)]),
            tool: vec![holes.into()],
        }
    );
    let made = common::session_insert(&mut session, SessionOp::AddBoolean { spec });
    session.pump();
    let ev = session.evaluation().expect("landed");
    let payload = format!("{:?}", ev.value(made).map(|v| &v.payload));
    assert!(
        payload.contains("Empty"),
        "a copy cut by its own pattern is empty: {}",
        &payload[..payload.len().min(300)]
    );
}
