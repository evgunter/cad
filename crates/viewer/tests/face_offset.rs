//! **A body placed at an offset from a face, through the placement
//! editor** (DESIGN `[placement.mates]`: "a number a kind's own equation
//! fixes (a standoff along a plane's normal) is a construction on the
//! target, never a value"). The editor's mate row carries an offset on
//! its far side: setting it slides the far side's plane along its own
//! normal (`PoseDef::Slide`), clearing it takes the slide off, and each
//! is one undoable `SetBundle`. The copy follows the face: an edit to
//! the face's body moves it, and the offset may read a variable.
//!
//! Each row drives the editor the way the chrome does: a change, its
//! offer to the session's door, the op performed, and the answer.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use pncad::document::pose::PoseDef;
use pncad::document::{
    Coord, Dimension, Doc, Formula, Mate, Node, Operand, ProfileProgram, RecipeNodeId, SlotId,
    TargetAxis, VarKind,
};
use pncad::geom_core::Tol;
use viewer::matetool::MateTool;
use viewer::session::{DocSession, SessionOp};
use viewer::tools::{ToolKind, Tools};

/// The base: a plate at the origin, its top the face the copy stands
/// off.
const BASE: [f64; 3] = [0.04, 0.05, 0.005];
/// The block placed off it, drawn clear of the base's footprint.
const BLOCK: [f64; 2] = [0.02, 0.03];
const BLOCK_HEIGHT: f64 = 0.01;
const BLOCK_AT: [f64; 2] = [0.1, 0.0];
/// The standoff the rows write.
const GAP: f64 = 0.005;

/// A scene: the base, the block, the block placed where it stands and
/// the editor open on its copy, no mate. Answers the session, the
/// tools holding the editor, the base's extrude and the copy's
/// placement.
fn scene(label: &str) -> (DocSession, Tools, RecipeNodeId, RecipeNodeId) {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived(label, tol), tol);
    let base = common::xy_box_in(&mut session, BASE);
    let profile = common::session_insert(
        &mut session,
        SessionOp::AddProfile {
            loops: vec![common::rectangle_at(BLOCK_AT, BLOCK[0], BLOCK[1])],
        },
    );
    let block = common::session_insert(
        &mut session,
        SessionOp::AddExtrude {
            profile,
            distance: common::len(BLOCK_HEIGHT),
        },
    );
    session.pump();
    let (tool, copy) = common::place_and_edit(&mut session, block.into());
    let mut tools = Tools::new();
    tools.open(ToolKind::Mate);
    *tools.mate_mut().expect("open") = tool;
    (session, tools, base, copy)
}

fn editor(tools: &mut Tools) -> &mut MateTool {
    tools.mate_mut().expect("the editor is open")
}

/// **The editor's change offered as the panel offers it**: against the
/// landed pair, judged by the session's door.
fn offered(session: &DocSession, tool: &mut MateTool) -> Option<SessionOp> {
    let (doc, ev) = session.landed_pair().expect("landed");
    tool.offer(doc, ev, session.tol(), |op| session.bundle_door(op))
}

/// **The editor's change landed**: `op` performed and pumped, and the
/// tools answered as the chrome answers them.
fn landed(session: &mut DocSession, tools: &mut Tools, op: SessionOp) {
    let outcome = session.perform(op.clone());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    tools.answered(&op, outcome.made, true, session.committed_doc());
}

/// **The open change offered and landed**, checked to be one history
/// state.
fn commit(session: &mut DocSession, tools: &mut Tools, what: &str) {
    let states = session.history().len();
    let op = offered(session, editor(tools)).unwrap_or_else(|| {
        panic!(
            "{what}: the door takes the change: {:?}",
            editor(tools).pending()
        )
    });
    assert!(
        matches!(op, SessionOp::SetBundle { .. }),
        "{what}: one SetBundle: {op:?}"
    );
    landed(session, tools, op);
    assert_eq!(
        session.history().len(),
        states + 1,
        "{what}: one undo step"
    );
}

/// **The block's lower cap rested on the base's top**, picked and
/// committed into the base's space as one change: the block's bottom
/// from below, the base's top from above, the row at its own sense
/// (opposed).
fn rested(session: &mut DocSession, tools: &mut Tools, kind: VarKind) {
    let index = common::asm::index_of(session);
    let bottom = common::displayed_face_at(
        session,
        &index,
        &common::up_at(BLOCK_AT[0], BLOCK_AT[1]),
    );
    let top = common::displayed_face_at(session, &index, &common::down_at(0.0, 0.0));
    let tool = editor(tools);
    assert!(tool.pick(session.doc(), bottom).is_none());
    assert!(tool.pick(session.doc(), top).is_none());
    if kind != VarKind::Plane {
        tool.choose_kind(0, kind);
    }
    common::into_to_space(session, tool);
    commit(session, tools, "the rest");
}

/// Where the copy stands in the base's space, the space it is placed
/// into: its translation.
fn at(session: &DocSession, copy: RecipeNodeId) -> [f64; 3] {
    let (doc, ev) = session.landed_pair().expect("landed");
    pncad::document::SolvedPoses::of(doc, ev)
        .relative(copy)
        .expect("the copy has a pose")
        .translation
}

fn near(got: f64, want: f64, what: &str) {
    assert!((got - want).abs() < 1e-9, "{what}: {got} vs {want}");
}

/// The copy's one mate, as the bundle writes it.
fn the_mate(doc: &Doc<ProfileProgram>, copy: RecipeNodeId) -> Mate<Formula> {
    let Some(Node::Place { constraints, .. }) = doc.node(copy).map(|node| node.authored(doc))
    else {
        panic!("{copy:?} is a placement")
    };
    let mates: Vec<_> = constraints.iter().filter_map(|c| c.mate().cloned()).collect();
    let [mate] = &mates[..] else {
        panic!("one mate: {mates:?}")
    };
    mate.clone()
}

/// Whether `side` slides anything, at any depth.
fn slides(side: &Operand) -> bool {
    match side {
        Operand::Pose(def) => match &**def {
            PoseDef::Slide { .. } => true,
            other => other.reads().iter().any(|(_, read, _, _)| slides(read)),
        },
        _ => false,
    }
}

/// The base's extrude grown to `height`, as the properties panel writes
/// it.
fn grown(session: &mut DocSession, base: RecipeNodeId, height: f64) {
    let outcome = session.perform(SessionOp::SetSlot {
        node: base,
        slot: SlotId::Distance,
        value: viewer::props::SlotValue::of(Dimension::Length, height).expect("a length"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
}

/// **No gesture of the editor slides the far side, and no value stands
/// in for a standoff.** The rest the picks write reads the base's top as
/// it is, and the freedoms it leaves are the two in-plane slides and the
/// spin: a slide typed along the target's `z` is dropped. A direction
/// mate does leave that slide, but a value on it is measured from the
/// space's origin, not from the face: growing the base leaves the copy
/// where it was, the face moving out from under it.
///
/// *Breaks if* a rest's freedoms come to include the normal, or a slide
/// value starts measuring from a face.
#[test]
fn a_value_cannot_stand_in_for_a_standoff() {
    let (mut session, mut tools, _, copy) = scene("face-offset-premise");
    rested(&mut session, &mut tools, VarKind::Plane);
    let doc = session.committed_doc();
    assert!(
        !slides(&the_mate(doc, copy).to),
        "the picks write the face's plane, unslid"
    );
    near(at(&session, copy)[2], BASE[2], "the block rests on the base");
    let (doc, ev) = session.landed_pair().expect("landed");
    let mut typed = editor(&mut tools).clone();
    typed.type_value(Coord::slide(TargetAxis::Z), common::len(GAP));
    let proposal = typed
        .proposal(doc, ev, session.tol())
        .expect("the rest proposes");
    assert!(
        proposal
            .values
            .iter()
            .all(|row| row.coord != Coord::slide(TargetAxis::Z)),
        "a rest leaves no slide along its normal: {:?}",
        proposal.values
    );
    assert!(
        proposal
            .dropped
            .iter()
            .any(|row| row.coord == Coord::slide(TargetAxis::Z)),
        "the typed normal slide is dropped: {:?}",
        proposal.dropped
    );

    let (mut session, mut tools, base_d, copy_d) = scene("face-offset-premise-direction");
    rested(&mut session, &mut tools, VarKind::Direction);
    editor(&mut tools).type_value(Coord::slide(TargetAxis::Z), common::len(BASE[2] + GAP));
    commit(&mut session, &mut tools, "the normal slide as a value");
    near(at(&session, copy_d)[2], BASE[2] + GAP, "stood off by a value");
    grown(&mut session, base_d, BASE[2] + 0.003);
    near(
        at(&session, copy_d)[2],
        BASE[2] + GAP,
        "the value is measured from the origin: the copy stays as the face moves",
    );
}
