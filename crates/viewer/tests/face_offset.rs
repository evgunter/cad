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
// The door answers the session's own `Refusal`, whatever its width.
#[allow(clippy::result_large_err)]
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
    assert_eq!(session.history().len(), states + 1, "{what}: one undo step");
}

/// **The block's lower cap rested on the base's top**, picked and
/// committed into the base's space as one change: the block's bottom
/// from below, the base's top from above, the row at its own sense
/// (opposed).
fn rested(session: &mut DocSession, tools: &mut Tools, kind: VarKind) {
    let index = common::asm::index_of(session);
    let bottom =
        common::displayed_face_at(session, &index, &common::up_at(BLOCK_AT[0], BLOCK_AT[1]));
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

/// The copy's one mate, as the bundle writes it: each anonymous pose a
/// side reads as its definition.
fn the_mate(doc: &Doc<ProfileProgram>, copy: RecipeNodeId) -> Mate<Formula> {
    let Some(Node::Place { constraints, .. }) = doc.node(copy).map(|node| node.written(doc)) else {
        panic!("{copy:?} is a placement")
    };
    let mates: Vec<_> = constraints
        .iter()
        .filter_map(|c| c.mate().cloned())
        .collect();
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
    near(
        at(&session, copy)[2],
        BASE[2],
        "the block rests on the base",
    );
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
    near(
        at(&session, copy_d)[2],
        BASE[2] + GAP,
        "stood off by a value",
    );
    grown(&mut session, base_d, BASE[2] + 0.003);
    near(
        at(&session, copy_d)[2],
        BASE[2] + GAP,
        "the value is measured from the origin: the copy stays as the face moves",
    );
}

/// The editor brought up to the session's document, as the chrome does
/// each frame.
fn reconciled(session: &DocSession, tools: &mut Tools) {
    let pair = session.landed_pair().expect("landed");
    editor(tools).reconcile(session.committed_doc(), pair);
}

/// **The first row's offset, set through the editor and committed.**
fn stood_off(session: &mut DocSession, tools: &mut Tools, by: Option<Formula>, what: &str) {
    editor(tools).set_offset(0, by);
    commit(session, tools, what);
}

/// **A block placed 5 mm off a face follows the face, and each offset
/// change is one undo step.** The offset row slides the base's top along
/// its own outward normal under the rest's flip; the block stands 5 mm
/// above the top, and stays 5 mm above it as the base grows. An editor
/// opened afresh reads the offset back off the bundle and writes the
/// bundle it read. Undo takes the offset off, redo puts it back, and
/// the editor follows both. Cleared, the offset unwraps; set to `0`,
/// it is a written offset of zero.
///
/// *Breaks if* the offset slides along the flipped normal (the block
/// sinks into the base), is measured from anything but the face, or a
/// load reads it as part of a side as written.
#[test]
fn a_block_placed_off_a_face_follows_the_face() {
    let (mut session, mut tools, base, copy) = scene("face-offset-follows");
    rested(&mut session, &mut tools, VarKind::Plane);
    near(at(&session, copy)[2], BASE[2], "the rest");
    let rest = the_mate(session.committed_doc(), copy);

    stood_off(
        &mut session,
        &mut tools,
        Some(common::len(GAP)),
        "the offset",
    );
    let offset = the_mate(session.committed_doc(), copy);
    near(at(&session, copy)[2], BASE[2] + GAP, "5 mm off the top");
    let role = viewer::matetool::offset_role(VarKind::Plane).expect("a plane takes an offset");
    assert_eq!(
        offset.to,
        viewer::matetool::offset_side(rest.to.clone(), role, common::len(GAP)),
        "the far side is the rest's, slid"
    );
    assert_eq!(offset.on, rest.on, "the near side is untouched");

    let fresh = MateTool::edit(session.committed_doc(), copy).expect("a placement");
    assert_eq!(
        fresh.rows()[0].offset(),
        Some(&common::len(GAP)),
        "a fresh editor reads the offset back"
    );
    let mut fresh = fresh;
    fresh.set_sense(0, true);
    assert!(
        offered(&session, &mut fresh).is_none() && fresh.pending().is_none(),
        "and writes the bundle it read: {:?}",
        fresh.pending()
    );

    grown(&mut session, base, BASE[2] + 0.003);
    near(
        at(&session, copy)[2],
        BASE[2] + 0.003 + GAP,
        "the block follows the grown face",
    );

    for (step, want, offset) in [
        (SessionOp::Undo, BASE[2] + GAP, Some(common::len(GAP))),
        (SessionOp::Undo, BASE[2], None),
        (SessionOp::Redo, BASE[2] + GAP, Some(common::len(GAP))),
    ] {
        let what = format!("{step:?} to {want}");
        let outcome = session.perform(step);
        assert!(outcome.refusal.is_none(), "{what}: {:?}", outcome.refusal);
        session.pump();
        reconciled(&session, &mut tools);
        near(at(&session, copy)[2], want, &what);
        assert_eq!(
            editor(&mut tools).rows()[0].offset(),
            offset.as_ref(),
            "{what}: the editor follows"
        );
        assert_eq!(
            slides(&the_mate(session.committed_doc(), copy).to),
            offset.is_some(),
            "{what}: the bundle's far side"
        );
    }

    stood_off(&mut session, &mut tools, None, "the offset cleared");
    near(at(&session, copy)[2], BASE[2], "cleared, the block rests");
    assert_eq!(
        the_mate(session.committed_doc(), copy).to,
        rest.to,
        "cleared, the far side is unwrapped"
    );
    let zero = Formula::zero(Dimension::Length);
    stood_off(
        &mut session,
        &mut tools,
        Some(zero.clone()),
        "an offset of 0",
    );
    near(at(&session, copy)[2], BASE[2], "an offset of zero rests");
    assert_eq!(
        the_mate(session.committed_doc(), copy).to,
        viewer::matetool::offset_side(rest.to, role, zero),
        "a written zero keeps its slide"
    );
}

/// **The offset takes a formula**: typed `2 * gap`, it reads the
/// document's `gap`, and one edit to `gap` moves the block. The field
/// left empty takes the offset off, and an angle is refused.
///
/// *Breaks if* the offset field reads only numbers, or a variable's
/// edit does not reach the standoff.
#[test]
fn the_offset_takes_a_formula() {
    let (mut session, mut tools, _, copy) = scene("face-offset-formula");
    let declared = session.perform(SessionOp::DeclareVar {
        name: pncad::document::VarName::from_static("gap"),
        value: pncad::document::FreeVar::continuous(Dimension::Length, GAP),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    session.pump();
    rested(&mut session, &mut tools, VarKind::Plane);
    let doc = session.committed_doc();
    assert!(
        matches!(viewer::matetool::typed_offset(doc, "  "), Ok(None)),
        "an empty field is no offset"
    );
    assert!(
        matches!(
            viewer::matetool::typed_offset(doc, "3 deg"),
            Err(viewer::matetool::TypedValueError::OffsetDimension { .. })
        ),
        "an angle is no offset"
    );
    let by = viewer::matetool::typed_offset(doc, "2 * gap").expect("a length formula");
    stood_off(&mut session, &mut tools, by, "the formula");
    near(
        at(&session, copy)[2],
        BASE[2] + 2.0 * GAP,
        "2 * gap off the top",
    );
    let gap = common::var_of(session.committed_doc(), "gap");
    let moved = session.perform(SessionOp::SetVariable {
        var: gap,
        value: viewer::props::SlotValue::Continuous(0.002),
    });
    assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    session.pump();
    near(
        at(&session, copy)[2],
        BASE[2] + 0.004,
        "gap's edit moves it",
    );
}

/// **The offset's spelling round-trips, and is only ever its own**:
/// [`viewer::matetool::side_offset`] takes off exactly what
/// [`viewer::matetool::offset_side`] put on — on a plane, on a flipped
/// one, on a frame along its `z` — and a slide it did not write (along
/// a flipped plane's normal, the other sign) is no offset, left as
/// written. Only a plane and a frame take one.
///
/// *Breaks if* a load reads a foreign slide as an offset, whose rewrite
/// would move the copy to the other side of the face.
#[test]
fn an_offset_is_read_back_only_where_the_editor_wrote_it() {
    use pncad::document::pose::{PoseRole, RoleRead};
    use viewer::matetool::{offset_role, offset_side, side_offset};
    let face = Operand::Var(pncad::document::VarId::WORLD);
    let plane = Operand::Pose(Box::new(PoseDef::Plane { face: face.clone() }));
    let flip = |pose: Operand| {
        Operand::Pose(Box::new(PoseDef::Flip {
            pose: RoleRead::plain(pose),
        }))
    };
    let frame = Operand::Pose(Box::new(PoseDef::Through {
        axis: RoleRead::plain(Operand::Pose(Box::new(PoseDef::Axis { of: face.clone() }))),
        point: RoleRead::plain(Operand::Pose(Box::new(PoseDef::Point { of: face }))),
    }));
    let by = common::len(GAP);
    for (what, side, role) in [
        ("a plane", plane.clone(), PoseRole::Normal),
        ("a flipped plane", flip(plane.clone()), PoseRole::Normal),
        ("a frame", frame.clone(), PoseRole::Z),
        ("a turned frame", flip(frame), PoseRole::Z),
    ] {
        let off = offset_side(side.clone(), role, by.clone());
        assert_eq!(
            side_offset(&off, role),
            (side.clone(), Some(by.clone())),
            "{what}"
        );
        assert_eq!(side_offset(&side, role), (side, None), "{what} unslid");
    }
    let foreign = Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(flip(plane.clone())),
        along: RoleRead::through(flip(plane), PoseRole::Normal),
        by,
    }));
    assert_eq!(
        side_offset(&foreign, PoseRole::Normal),
        (foreign.clone(), None),
        "a slide of a flipped plane is left as written"
    );
    let takes: Vec<_> = [
        VarKind::Plane,
        VarKind::Axis,
        VarKind::Point,
        VarKind::Direction,
        VarKind::Frame,
    ]
    .into_iter()
    .filter(|&kind| offset_role(kind).is_some())
    .collect();
    assert_eq!(
        takes,
        [VarKind::Plane, VarKind::Frame],
        "the kinds that take one"
    );
}
