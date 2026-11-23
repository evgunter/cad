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
    // Turned to flush and back before it is offered: the draft is the
    // bundle it read, and writes nothing.
    let mut back = fresh.clone();
    back.set_sense(0, false);
    back.set_sense(0, true);
    assert!(
        offered(&session, &mut back).is_none() && back.pending().is_none(),
        "toggled and back, it writes the bundle it read: {:?}",
        back.pending()
    );
    // Turned to flush and landed: the loaded side toggled, its offset
    // kept, the far plane where it was; one undo takes it back.
    let mut flush = fresh;
    flush.set_sense(0, false);
    let op = offered(&session, &mut flush).expect("flush is a change");
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    near(
        at(&session, copy)[2],
        BASE[2] + GAP,
        "flush keeps the far plane",
    );
    let reread = MateTool::edit(session.committed_doc(), copy).expect("a placement");
    assert_eq!(
        (reread.rows()[0].flip(), reread.rows()[0].offset()),
        (false, Some(&common::len(GAP))),
        "flush, the offset read back"
    );
    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    reconciled(&session, &mut tools);
    near(at(&session, copy)[2], BASE[2] + GAP, "opposed again");

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
        assert_eq!(
            offset_side(off, role, common::len(2.0 * GAP)),
            offset_side(side.clone(), role, common::len(2.0 * GAP)),
            "{what}: a second offset replaces the first, never stacks"
        );
        assert_eq!(side_offset(&side, role), (side, None), "{what} unslid");
    }
    let foreign = Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(flip(plane.clone())),
        along: RoleRead::through(flip(plane.clone()), PoseRole::Normal),
        by,
    }));
    assert_eq!(
        side_offset(&foreign, PoseRole::Normal),
        (foreign.clone(), None),
        "a slide of a flipped plane is left as written"
    );
    let over = offset_side(foreign.clone(), PoseRole::Normal, common::len(GAP));
    assert_eq!(
        side_offset(&over, PoseRole::Normal),
        (foreign.clone(), Some(common::len(GAP))),
        "an offset over it sits beside it, the foreign slide beneath as written"
    );
    assert!(
        matches!(&over, Operand::Pose(def) if matches!(&**def,
            PoseDef::Slide { along, .. } if along.read == plane && along.role == Some(PoseRole::Normal))),
        "its direction is the face's own outward normal: {over:?}"
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

// --- review rows (formwords-review-face-offset) ----------------------

/// **The copy mated `on` the face a ray `on` hits on the block and `to`
/// the face a ray `to` hits elsewhere**, as `rested` mates its pair, at
/// the row's own sense (opposed).
fn mated_by(
    session: &mut DocSession,
    tools: &mut Tools,
    on: &pncad::select::Ray,
    to: &pncad::select::Ray,
) {
    let index = common::asm::index_of(session);
    let on = common::displayed_face_at(session, &index, on);
    let to = common::displayed_face_at(session, &index, to);
    let tool = editor(tools);
    assert!(tool.pick(session.doc(), on).is_none());
    assert!(tool.pick(session.doc(), to).is_none());
    common::into_to_space(session, tool);
    commit(session, tools, "the mate");
}

/// The unnamed variables nothing reads (VR7 says there are none).
fn unread_unnamed(doc: &Doc<ProfileProgram>) -> Vec<pncad::document::VarId> {
    doc.vars()
        .keys()
        .copied()
        .filter(|&var| {
            var != pncad::document::VarId::WORLD
                && !doc.var_names().contains_key(&var)
                && !doc.is_read(var)
        })
        .collect()
}

/// **A positive offset moves the copy out of the face, whatever the
/// face's normal and whichever sense the row reads it in**: on the
/// base's top (`+z`), its underside (`-z`) and its `+x` wall, the copy's
/// mated face stands `+GAP` along the target face's outward normal, and
/// toggling the sense with the offset on leaves that plane where it is.
/// Signs, not magnitudes.
///
/// *Breaks if* the offset runs along the flipped normal for any of the
/// three, or a toggle moves the far plane.
#[test]
fn review_a_positive_offset_runs_out_of_the_face_in_every_direction_and_sense() {
    let ([bx, by], h) = (BLOCK_AT, BLOCK_HEIGHT);
    // (what, on-ray, to-ray, axis, the outward normal's sign, the face's
    // coordinate, where the copy's origin sits along `axis` at rest)
    let cases = [
        (
            "the top, +z",
            common::up_at(bx, by),
            common::down_at(0.0, 0.0),
            2,
            1.0,
            BASE[2],
            BASE[2],
        ),
        (
            "the underside, -z",
            common::down_at(bx, by),
            common::up_at(0.0, 0.0),
            2,
            -1.0,
            0.0,
            -h,
        ),
        (
            "the +x wall",
            common::up_at(bx, by),
            common::along_x(-1.0, 0.02, BASE[2] / 2.0),
            0,
            1.0,
            BASE[0] / 2.0,
            BASE[0] / 2.0,
        ),
    ];
    for (what, on, to, axis, sign, _face, rest_at) in cases {
        let (mut session, mut tools, _, copy) = scene("face-offset-review-sign");
        mated_by(&mut session, &mut tools, &on, &to);
        let rest = at(&session, copy)[axis];
        near(rest, rest_at, &format!("{what}: the rest"));
        stood_off(&mut session, &mut tools, Some(common::len(GAP)), what);
        let off = at(&session, copy)[axis];
        near(
            off - rest,
            sign * GAP,
            &format!("{what}: +GAP runs out of the face"),
        );
        // Flush, the offset still on: the copy's own face (its local
        // `z = 0`, so its origin) stays on the far plane.
        if axis == 2 && sign > 0.0 {
            editor(&mut tools).set_sense(0, false);
            commit(&mut session, &mut tools, "flush");
            near(
                at(&session, copy)[axis],
                BASE[2] + GAP,
                &format!("{what}: flush keeps the far plane"),
            );
            assert_eq!(
                editor(&mut tools).rows()[0].offset(),
                Some(&common::len(GAP)),
                "{what}: flush reads its offset back"
            );
            // The offset changed while flush: still out of the face.
            stood_off(&mut session, &mut tools, Some(common::len(2.0 * GAP)), what);
            near(
                at(&session, copy)[axis],
                BASE[2] + 2.0 * GAP,
                &format!("{what}: a flush offset runs out of the face"),
            );
            editor(&mut tools).set_sense(0, true);
            commit(&mut session, &mut tools, "opposed again");
            near(
                at(&session, copy)[axis],
                BASE[2] + 2.0 * GAP,
                &format!("{what}: back to opposed keeps the far plane"),
            );
            assert_eq!(
                editor(&mut tools).rows()[0].offset(),
                Some(&common::len(2.0 * GAP)),
                "{what}: a double flip collapses, the offset still read"
            );
        }
    }
}

/// **On a face of a mirrored body the offset still runs out of the
/// face**: the base mirrored across its own top, the block rested on the
/// image's top (the mirror of the base's underside) and stood off.
#[test]
fn review_a_positive_offset_runs_out_of_a_mirrored_face() {
    let (mut session, mut tools, base, copy) = scene("face-offset-review-mirror");
    let index = common::asm::index_of(&session);
    let plane = common::displayed_face_at(&session, &index, &common::down_at(0.0, 0.0));
    let out = session.perform(SessionOp::AddMirror {
        body: base.into(),
        plane: viewer::session::ToolPlane::Face(plane),
    });
    assert!(out.refusal.is_none(), "mirror: {:?}", out.refusal);
    session.pump();
    let (bx, by) = (BLOCK_AT[0], BLOCK_AT[1]);
    mated_by(
        &mut session,
        &mut tools,
        &common::up_at(bx, by),
        &common::down_at(0.0, 0.0),
    );
    near(at(&session, copy)[2], 2.0 * BASE[2], "on the image's top");
    stood_off(&mut session, &mut tools, Some(common::len(GAP)), "mirror");
    near(
        at(&session, copy)[2],
        2.0 * BASE[2] + GAP,
        "+GAP off the image's top",
    );
}

/// **A scene whose copy rests by the API's other spelling**,
/// `Slide(Flip P, (Flip P).Normal, d)` (`asm::shelf_rest_op`'s): the
/// session, the tools, the copy and the mate as written.
fn foreign_rest(label: &str, d: f64) -> (DocSession, Tools, RecipeNodeId, Mate<Formula>) {
    use pncad::document::pose::{PoseRole, RoleRead};
    let (mut session, mut tools, _, copy) = scene(label);
    rested(&mut session, &mut tools, VarKind::Plane);
    let rest = the_mate(session.committed_doc(), copy);
    let seat = rest.to.clone();
    let to = Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(seat.clone()),
        along: RoleRead::through(seat, PoseRole::Normal),
        by: common::len(d),
    }));
    let foreign_mate = Mate {
        on: rest.on.clone(),
        to,
        orient: rest.orient,
    };
    editor(&mut tools).strike(0, true);
    editor(&mut tools).add_written(session.committed_doc(), &foreign_mate);
    commit(&mut session, &mut tools, "the foreign rest");
    assert_eq!(
        the_mate(session.committed_doc(), copy),
        foreign_mate,
        "the bundle holds the API's spelling"
    );
    near(
        at(&session, copy)[2],
        BASE[2] - d,
        "the foreign slide sinks by d",
    );
    let mut fresh = Tools::new();
    fresh.open(ToolKind::Mate);
    *fresh.mate_mut().expect("open") =
        MateTool::edit(session.committed_doc(), copy).expect("a placement");
    (session, fresh, copy, foreign_mate)
}

/// **A loaded rest written the other way round (`Slide(Flip P)`, the
/// spelling `asm::shelf_rest_op` writes) takes a new offset out of the
/// face too**: the API writes the rest slid by `d` along the flipped
/// plane's normal (into the base); the editor reads no offset of its
/// own there, and a `+GAP` set on the row must still move the block
/// `+GAP` out of the face.
///
/// *Red at `86c1734a1`*: `offset_side` slides the foreign side along its
/// own (flipped, inward) normal, and the block sinks `GAP` further.
#[test]
fn review_an_offset_on_a_foreign_slide_runs_out_of_the_face() {
    let (mut session, mut tools, copy, _) = foreign_rest("face-offset-review-foreign", 0.002);
    let foreign = at(&session, copy)[2];
    assert_eq!(
        editor(&mut tools).rows()[0].offset(),
        None,
        "no offset of its own"
    );
    stood_off(
        &mut session,
        &mut tools,
        Some(common::len(GAP)),
        "an offset over the foreign slide",
    );
    near(
        at(&session, copy)[2] - foreign,
        GAP,
        "+GAP over a foreign slide runs out of the face",
    );
}

/// **A foreign slide survives the editor's other gestures**: a value
/// typed, the sense toggled and back, each leaves the API's slide as
/// written (the toggle's double flip collapsing at the door).
#[test]
fn review_a_foreign_slide_survives_the_editors_other_gestures() {
    let (mut session, mut tools, copy, foreign) =
        foreign_rest("face-offset-review-foreign-kept", 0.002);
    editor(&mut tools).type_value(Coord::slide(TargetAxis::X), common::len(0.001));
    commit(&mut session, &mut tools, "an unrelated value");
    assert_eq!(the_mate(session.committed_doc(), copy), foreign, "a value");
    editor(&mut tools).set_sense(0, false);
    commit(&mut session, &mut tools, "flush");
    assert!(
        slides(&the_mate(session.committed_doc(), copy).to),
        "flush keeps the slide"
    );
    near(
        at(&session, copy)[2],
        BASE[2] - 0.002,
        "flush keeps the far plane",
    );
    editor(&mut tools).set_sense(0, true);
    commit(&mut session, &mut tools, "opposed");
    assert_eq!(
        the_mate(session.committed_doc(), copy),
        foreign,
        "toggled twice, the API's spelling exactly"
    );
}

/// One change of the editor's draft.
type Step = Box<dyn Fn(&mut MateTool)>;

/// **Each offset change leaves no unread unnamed variable behind**
/// (VR7) and is one undo step: set, changed, toggled, cleared.
#[test]
fn review_offset_rewrites_leave_no_unread_variable() {
    let (mut session, mut tools, _, _) = scene("face-offset-review-vr7");
    rested(&mut session, &mut tools, VarKind::Plane);
    let before = unread_unnamed(session.committed_doc());
    let steps: [(&str, Step); 5] = [
        ("set", Box::new(|t| t.set_offset(0, Some(common::len(GAP))))),
        (
            "changed",
            Box::new(|t| t.set_offset(0, Some(common::len(2.0 * GAP)))),
        ),
        ("flush", Box::new(|t| t.set_sense(0, false))),
        ("opposed", Box::new(|t| t.set_sense(0, true))),
        ("cleared", Box::new(|t| t.set_offset(0, None))),
    ];
    for (what, step) in steps {
        step(editor(&mut tools));
        commit(&mut session, &mut tools, what);
        assert_eq!(
            unread_unnamed(session.committed_doc()),
            before,
            "{what}: no unread unnamed variable"
        );
    }
}

/// **A negative formula offset sinks the copy, and one reading a
/// variable later deleted leaves the copy unplaced**, the editor still
/// reading the offset.
#[test]
fn review_an_offset_reading_a_deleted_variable() {
    let (mut session, mut tools, _, copy) = scene("face-offset-review-deleted");
    let declared = session.perform(SessionOp::DeclareVar {
        name: pncad::document::VarName::from_static("gap"),
        value: pncad::document::FreeVar::continuous(Dimension::Length, GAP),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    session.pump();
    rested(&mut session, &mut tools, VarKind::Plane);
    let doc = session.committed_doc();
    let by = viewer::matetool::typed_offset(doc, "-gap").expect("a length formula");
    stood_off(&mut session, &mut tools, by, "a negative formula");
    near(at(&session, copy)[2], BASE[2] - GAP, "-gap sinks the block");
    let gap = common::var_of(session.committed_doc(), "gap");
    let deleted = session.perform(SessionOp::DeleteVar { var: gap });
    assert!(deleted.refusal.is_none(), "{:?}", deleted.refusal);
    session.pump();
    reconciled(&session, &mut tools);
    // DeleteVar leaves its readers reading an id the document no longer
    // holds (VR7): the offset still reads it, and the copy has no pose.
    assert!(
        editor(&mut tools).rows()[0].offset().is_some(),
        "the offset still reads the deleted variable"
    );
    let (doc, ev) = session.landed_pair().expect("landed");
    assert!(
        pncad::document::SolvedPoses::of(doc, ev)
            .relative(copy)
            .is_none(),
        "a standoff reading a deleted variable places nothing"
    );
}

/// **What the offset field shows: empty for no offset, the formula for
/// one that reads zero.** A cleared offset is no slide, and its row
/// shows an empty field; an offset typed as `gap` with `gap` at `0`
/// keeps its slide, and its row shows `gap` (`matetool::value_text`),
/// never an empty field or a bare `0`, so clearing and a zero are told
/// apart on screen as in the bundle.
///
/// *Breaks if* a formula reading zero unwraps, or the field shows it
/// as the empty field of no offset.
#[test]
fn a_zero_offset_is_shown_as_written_and_no_offset_as_empty() {
    let (mut session, mut tools, _, copy) = scene("face-offset-zero-shown");
    let declared = session.perform(SessionOp::DeclareVar {
        name: pncad::document::VarName::from_static("gap"),
        value: pncad::document::FreeVar::continuous(Dimension::Length, 0.0),
    });
    assert!(declared.refusal.is_none(), "{:?}", declared.refusal);
    session.pump();
    rested(&mut session, &mut tools, VarKind::Plane);
    let shown = |tools: &mut Tools, session: &DocSession| {
        editor(tools).rows()[0]
            .offset()
            .map(|by| viewer::matetool::value_text(session.committed_doc(), by))
            .unwrap_or_default()
    };
    assert_eq!(shown(&mut tools, &session), "", "no offset: an empty field");
    let by = viewer::matetool::typed_offset(session.committed_doc(), "gap").expect("a length");
    stood_off(&mut session, &mut tools, by, "gap at zero");
    near(at(&session, copy)[2], BASE[2], "gap at zero rests");
    assert!(
        slides(&the_mate(session.committed_doc(), copy).to),
        "a formula reading zero keeps its slide"
    );
    assert_eq!(
        shown(&mut tools, &session),
        "gap",
        "the field shows the formula"
    );
    stood_off(&mut session, &mut tools, None, "cleared");
    assert!(
        !slides(&the_mate(session.committed_doc(), copy).to),
        "cleared, no slide"
    );
    assert_eq!(shown(&mut tools, &session), "", "cleared: an empty field");
}
