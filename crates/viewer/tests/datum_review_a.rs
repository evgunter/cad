//! **Review A of PR 7535** (the datum's viewer half): rows that try to
//! break the PR's claims. A row that fails here is a finding.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]
// The bundle door's refusal is the session's whole `Refusal`.
#![allow(clippy::result_large_err)]

use pncad::document::pose::{PoseRole, RoleRead};
use pncad::document::{
    Dimension, Doc, Node, Operand, ProfileProgram, RecipeNodeId, SlotId, VarId, VarKind,
};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use viewer::matetool::{MateTool, MateToolError};
use viewer::session::{DocSession, FaceSelection, SessionOp};

fn copy_of(doc: &Doc<ProfileProgram>, block: RecipeNodeId) -> RecipeNodeId {
    doc.placements()
        .into_iter()
        .find(|&p| {
            doc.copied_read(p)
                .is_some_and(|read| doc.operation_of(doc.through_sources(read).read) == Some(block))
        })
        .expect("the block's copy")
}

fn cap(block: RecipeNodeId, on: RecipeNodeId, end: CapEnd) -> FaceSelection {
    FaceSelection {
        name: StableName {
            kind: EntityKind::Face,
            node: block,
            path: vec![RoleSeg::Cap(end)],
        },
        node: on,
        body: 0,
    }
}

fn add_datum(session: &mut DocSession, space: VarId, _kind: VarKind) -> RecipeNodeId {
    let outcome = session.perform(SessionOp::AddDatum { space });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    outcome.made.expect("the datum")
}

fn commit(session: &mut DocSession, tool: &MateTool, what: &str) {
    session.pump();
    let mut tool = tool.clone();
    let tol = session.tol();
    let (doc, ev) = session.landed_pair().expect("landed");
    let op = tool
        .offer(doc, ev, tol, |op| session.bundle_door(op))
        .unwrap_or_else(|| panic!("{what}: the door takes it: {:?}", tool.pending()));
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{what}: {:?}", outcome.refusal);
    session.pump();
}

fn at(session: &DocSession, node: RecipeNodeId) -> [f64; 3] {
    let (doc, ev) = session.landed_pair().expect("landed");
    pncad::document::SolvedPoses::of(doc, ev)
        .relative(node)
        .expect("a pose")
        .translation
}

fn datum_frame(session: &DocSession, datum: RecipeNodeId) -> ([f64; 3], [f64; 3]) {
    let (_, ev) = session.landed_pair().expect("landed");
    let Some(pncad::document::NodeResult::Ok(value)) = ev.nodes.get(&datum) else {
        panic!("the datum evaluates: {:?}", ev.nodes.get(&datum))
    };
    let pncad::document::ValuePayload::Datum(frame) = &value.payload else {
        panic!("a frame")
    };
    let o = frame.origin();
    let z = frame.w().get();
    ([o.x, o.y, o.z], [z.x, z.y, z.z])
}

/// The block's world copy rested on a world datum with an offset:
/// the copy's bottom cap, opposed (its default), the datum picked whole.
fn rested_on_datum(label: &str, gap: f64) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block(label, tol);
    let mut session = DocSession::inline(doc, tol);
    let datum = add_datum(&mut session, VarId::WORLD, VarKind::Plane);
    let doc = session.doc();
    let copy = copy_of(doc, block);
    let mut tool = MateTool::moving(doc, copy, None).expect("opens");
    tool.add_mate(VarKind::Plane);
    assert!(tool.pick(doc, cap(block, copy, CapEnd::Start)).is_none());
    tool.pick_datum(doc, datum);
    tool.set_offset(0, Some(viewer::test_support::len(gap)));
    commit(&mut session, &tool, "the rest");
    (session, copy, datum)
}

/// Claim 3 (sign): a copy rested opposed on a datum and offset `+g`
/// stands `g` on the datum's own `+z` side, as an offset from a face
/// stands out of the face.
#[test]
fn a_copy_offset_off_a_datum_stands_on_its_z_side() {
    let (session, copy, _) = rested_on_datum("review-a-sign", 0.005);
    let z = at(&session, copy)[2];
    assert!(
        (z - 0.005).abs() < 1e-9,
        "the copy's bottom stands 0.005 above the datum, out along its z: at z = {z}"
    );
}

/// Claim 3 (the offset row): reopening the editor on a copy offset off
/// a datum reads its offset back, and a new offset replaces it.
#[test]
fn a_datum_far_sides_offset_reloads_and_replaces() {
    let (mut session, copy, _) = rested_on_datum("review-a-reload", 0.005);
    let before = at(&session, copy)[2];
    let mut tool = MateTool::edit(session.doc(), copy).expect("reopens");
    let loaded = tool.rows()[0].offset().cloned();
    tool.set_offset(0, Some(viewer::test_support::len(0.01)));
    commit(&mut session, &tool, "the new offset");
    let after = at(&session, copy)[2];
    assert!(
        loaded.is_some() && (after - before).abs() > 1e-9 && (after.abs() - 0.01).abs() < 1e-9,
        "loaded offset {loaded:?}; before {before}, after {after}: a reloaded offset replaces, \
         never stacks"
    );
}

/// Claim 3: a datum offset from the block's bottom cap stands out of
/// it (below, `z = -1`), and follows the block when it grows.
#[test]
fn a_datum_offset_off_a_bottom_cap_stands_below_and_follows_edits() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("review-a-bottom", tol);
    let mut session = DocSession::inline(doc, tol);
    let datum = add_datum(&mut session, VarId::WORLD, VarKind::Plane);
    let doc = session.doc();
    let copy = copy_of(doc, block);
    let mut tool = MateTool::edit(doc, datum).expect("opens");
    tool.add_mate(VarKind::Plane);
    assert!(tool.pick(doc, cap(block, copy, CapEnd::Start)).is_none());
    tool.set_offset(0, Some(viewer::test_support::len(0.005)));
    commit(&mut session, &tool, "below");
    let (o, z) = datum_frame(&session, datum);
    assert!(
        (o[2] + 0.005).abs() < 1e-9 && (z[2] + 1.0).abs() < 1e-9,
        "{o:?} {z:?}"
    );
    // The top cap, and the block grown: the datum follows.
    let mut tool = MateTool::edit(session.doc(), datum).expect("opens");
    let doc = session.doc();
    tool.add_mate(VarKind::Plane);
    tool.strike(0, true);
    assert!(tool.pick(doc, cap(block, copy, CapEnd::End)).is_none());
    tool.set_offset(1, Some(viewer::test_support::len(0.005)));
    commit(&mut session, &tool, "on top");
    let depth = viewer::test_support::BOSS_BLOCK[2];
    let (o, _) = datum_frame(&session, datum);
    assert!((o[2] - (depth + 0.005)).abs() < 1e-9, "{o:?}");
    let outcome = session.perform(SessionOp::SetSlot {
        node: block,
        slot: SlotId::Distance,
        value: viewer::props::SlotValue::of(Dimension::Length, 2.0 * depth).expect("a length"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let (o, _) = datum_frame(&session, datum);
    assert!(
        (o[2] - (2.0 * depth + 0.005)).abs() < 1e-9,
        "follows: {o:?}"
    );
}

/// Claim 2: a datum picked whole on rows of each kind reads through
/// the row's role: an axis its `z`, a frame itself.
#[test]
fn a_datum_picked_whole_reads_through_each_rows_role() {
    let tol = Tol::witness();
    let (doc, _, _) = viewer::test_support::boss_on_block("review-a-kinds", tol);
    let mut session = DocSession::inline(doc, tol);
    let datum = add_datum(&mut session, VarId::WORLD, VarKind::Plane);
    let other = add_datum(&mut session, VarId::WORLD, VarKind::Plane);
    let own = Operand::Output {
        node: other,
        port: 0,
    };
    for (kind, want) in [(VarKind::Axis, Some(PoseRole::Z)), (VarKind::Frame, None)] {
        let mut tool = MateTool::edit(session.doc(), datum).expect("opens");
        tool.add_mate(kind);
        tool.pick_datum(session.doc(), other);
        let (doc, ev) = session.landed_pair().expect("landed");
        let proposal = tool.proposal(doc, ev, tol).expect("proposes");
        let to = &proposal.mates[0].mate.to;
        let plain = match want {
            Some(role) => RoleRead::through(own.clone(), role),
            None => RoleRead::plain(own.clone()),
        };
        assert_eq!(*to, plain, "{kind:?}");
    }
}

/// Claim 5: a point row whose far side is a datum, its sense set
/// turned, writes the datum's origin as it is.
#[test]
fn a_point_rows_datum_side_is_never_a_flipped_origin() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("review-a-point", tol);
    let mut session = DocSession::inline(doc, tol);
    let datum = add_datum(&mut session, VarId::WORLD, VarKind::Point);
    let mut tool = MateTool::edit(session.doc(), datum).expect("opens");
    // A datum's own editor: a frame row on another datum, then a point.
    let other = add_datum(&mut session, VarId::WORLD, VarKind::Point);
    let doc = session.doc();
    let _ = block;
    tool.add_mate(VarKind::Point);
    tool.pick_datum(doc, other);
    for flip in [false, true] {
        tool.set_sense(0, flip);
        let (doc, ev) = session.landed_pair().expect("landed");
        let proposal = tool.proposal(doc, ev, tol).expect("proposes");
        let to = &proposal.mates[0].mate.to;
        assert_eq!(
            *to,
            RoleRead::through(
                Operand::Output {
                    node: other,
                    port: 0
                },
                PoseRole::Origin
            ),
            "flip {flip}: the origin as it is"
        );
        let on = &proposal.mates[0].mate.on;
        assert_eq!(
            *on,
            RoleRead::through(
                Operand::Output {
                    node: datum,
                    port: 0
                },
                PoseRole::Origin
            )
        );
    }
}

/// Claim 2: a direction row on a datum (door or far side) is refused
/// typed.
#[test]
fn a_direction_row_on_a_datum_is_refused_typed() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("review-a-direction", tol);
    let mut session = DocSession::inline(doc, tol);
    let datum = add_datum(&mut session, VarId::WORLD, VarKind::Plane);
    let other = add_datum(&mut session, VarId::WORLD, VarKind::Plane);
    let _ = block;
    let mut tool = MateTool::edit(session.doc(), datum).expect("opens");
    tool.add_mate(VarKind::Direction);
    tool.pick_datum(session.doc(), other);
    let (doc, ev) = session.landed_pair().expect("landed");
    let e = tool.proposal(doc, ev, tol).err();
    assert!(
        matches!(e, Some(MateToolError::DatumKind { row: 0 })),
        "typed: {e:?}"
    );
}

/// The realistic New-plane flow: the block selected in the tree puts
/// the datum in the block's own space; the face the person then picks
/// in the viewport is on the world copy. What does the editor do?
#[test]
fn a_datum_in_the_blocks_space_picks_the_copys_face() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("review-a-cross", tol);
    let space = doc.node_space(block).expect("its space");
    let mut session = DocSession::inline(doc, tol);
    let datum = add_datum(&mut session, space, VarKind::Plane);
    let doc = session.doc();
    let copy = copy_of(doc, block);
    let mut tool = MateTool::edit(doc, datum).expect("opens");
    tool.add_mate(VarKind::Plane);
    let event = tool.pick(doc, cap(block, copy, CapEnd::End));
    let (doc, ev) = session.landed_pair().expect("landed");
    let proposal = tool.proposal(doc, ev, tol);
    match proposal {
        Ok(p) => {
            let outcome = session.perform(p.op());
            println!(
                "cross-space: pick event {event:?}; refusal {:?}; datum space {:?}",
                outcome.refusal,
                session.doc().node_space(datum)
            );
        }
        Err(e) => println!("cross-space: pick event {event:?}; proposal refused: {e}"),
    }
}

/// A cycle through a datum: a datum mated on the copy's top, then the
/// copy rested on the datum. Refused loudly, never a panic.
#[test]
fn a_copy_rested_on_a_datum_built_on_it_is_refused() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("review-a-cycle", tol);
    let mut session = DocSession::inline(doc, tol);
    let datum = add_datum(&mut session, VarId::WORLD, VarKind::Plane);
    let doc = session.doc();
    let copy = copy_of(doc, block);
    let mut tool = MateTool::edit(doc, datum).expect("opens");
    tool.add_mate(VarKind::Plane);
    assert!(tool.pick(doc, cap(block, copy, CapEnd::End)).is_none());
    commit(&mut session, &tool, "the datum on the copy");
    let doc = session.doc();
    let mut tool = MateTool::moving(doc, copy, None).expect("opens");
    tool.add_mate(VarKind::Plane);
    assert!(tool.pick(doc, cap(block, copy, CapEnd::Start)).is_none());
    tool.pick_datum(doc, datum);
    let (doc, ev) = session.landed_pair().expect("landed");
    match tool.proposal(doc, ev, tol) {
        Ok(p) => {
            let outcome = session.perform(p.op());
            println!(
                "cycle: refusal {:?}",
                outcome.refusal.map(|r| r.to_string())
            );
            assert!(session.doc().node(copy).is_some(), "the document stands");
        }
        Err(e) => println!("cycle: proposal refused: {e}"),
    }
}

/// Claim 1: the button's op opens the editor (`Tools::datum_made`)
/// with one row of its kind, and the create and the edit are one undo
/// step each.
#[test]
fn new_axis_opens_its_editor_and_each_step_undoes_alone() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("review-a-undo", tol);
    let mut session = DocSession::inline(doc, tol);
    let states = session.history().len();
    let op = SessionOp::AddDatum {
        space: VarId::WORLD,
    };
    let outcome = session.perform(op.clone());
    assert!(outcome.refusal.is_none());
    session.pump();
    let datum = outcome.made.expect("made");
    assert_eq!(
        session.history().len(),
        states + 1,
        "the create is one step"
    );
    let mut tools = viewer::tools::Tools::new();
    tools.awaiting_datum(VarKind::Axis);
    tools.datum_made(&op, Some(datum), session.committed_doc());
    let tool = tools.mate_mut().expect("the editor opened").clone();
    assert_eq!(tool.door().map(|d| d.placement), Some(datum));
    assert_eq!(tool.rows().len(), 1);
    assert_eq!(tool.rows()[0].kind(), VarKind::Axis);
    // The editor opened by New axis; a plane row on the copy's top cap
    // is the edit (an axis row on a datum refuses Backwards, see
    // `a_datum_mates_on_a_datum_at_zero_by_every_kind`).
    let mut tool = tool;
    let doc = session.doc();
    let copy = copy_of(doc, block);
    tool.choose_kind(0, VarKind::Plane);
    let states = session.history().len();
    assert!(
        tool.pick(session.doc(), cap(block, copy, CapEnd::End))
            .is_none()
    );
    commit(&mut session, &tool, "the plane");
    assert_eq!(session.history().len(), states + 1, "the edit is one step");
    let outcome = session.perform(SessionOp::Undo);
    assert!(outcome.refusal.is_none());
    session.pump();
    assert!(
        matches!(session.doc().node(datum), Some(Node::Datum { constraints, .. }) if constraints.iter().all(|c| c.mate().is_none())),
        "one undo takes the mate off and keeps the datum"
    );
}

/// Claim 6: a datum in a space whose body is copied elsewhere is drawn
/// where that space's body is drawn: the boss stands on the block at
/// `z = depth`, through its copy; a datum in the boss's space at its
/// origin is drawn on the boss's bottom, there.
#[test]
fn a_datum_in_a_copied_space_is_drawn_with_the_copy() {
    let tol = Tol::witness();
    let (doc, _, boss) = viewer::test_support::boss_on_block("review-a-drawn", tol);
    let space = doc.node_space(boss).expect("the boss's space");
    let mut session = DocSession::inline(doc, tol);
    let datum = add_datum(&mut session, space, VarKind::Plane);
    let shown = viewer::world::shown_spaces(session.doc());
    let (doc, ev) = session.landed_pair().expect("landed");
    let drawn = viewer::sketch::committed(doc, ev, 1e-4, None);
    let d = drawn.drawn.iter().find(|d| d.node == datum).expect("drawn");
    let o = d.plane.frame().origin();
    let depth = viewer::test_support::BOSS_BLOCK[2];
    assert!(
        (o.z - depth).abs() < 1e-9,
        "the boss's space {space:?} is shown: {}; its datum drawn at {o:?}, the boss's copy at \
         z = {depth}",
        shown.contains(&space)
    );
}

/// Design: "a mate may read a datum at all-zero values". A datum mated
/// on another datum at zero, through each kind, at each sense, lands.
#[test]
#[ignore = "work/formwords/a-datum-mated-on-a-datum-at-zero-levers-its-pole-over-nothing.md"]
fn a_datum_mates_on_a_datum_at_zero_by_every_kind() {
    let tol = Tol::witness();
    let mut failed = Vec::new();
    for kind in [
        VarKind::Plane,
        VarKind::Axis,
        VarKind::Point,
        VarKind::Frame,
    ] {
        for flip in [false, true] {
            let (doc, _, _) = viewer::test_support::boss_on_block("review-a-dd", tol);
            let mut session = DocSession::inline(doc, tol);
            let datum = add_datum(&mut session, VarId::WORLD, kind);
            let other = add_datum(&mut session, VarId::WORLD, VarKind::Frame);
            let mut tool = MateTool::edit(session.doc(), datum).expect("opens");
            tool.add_mate(kind);
            tool.pick_datum(session.doc(), other);
            tool.set_sense(0, flip);
            session.pump();
            let (doc, ev) = session.landed_pair().expect("landed");
            if tool
                .offer(doc, ev, tol, |op| session.bundle_door(op))
                .is_none()
            {
                failed.push(format!("{kind:?} flip {flip}: {:?}", tool.pending()));
            }
        }
    }
    assert!(failed.is_empty(), "{failed:#?}");
}

/// Claim 3: a datum offset off the copy's top cap follows the block as
/// it grows.
#[test]
fn a_datum_offset_off_a_top_cap_follows_the_block() {
    let tol = Tol::witness();
    let (doc, block, _) = viewer::test_support::boss_on_block("review-a-follows", tol);
    let mut session = DocSession::inline(doc, tol);
    let datum = add_datum(&mut session, VarId::WORLD, VarKind::Plane);
    let doc = session.doc();
    let copy = copy_of(doc, block);
    let mut tool = MateTool::edit(doc, datum).expect("opens");
    tool.add_mate(VarKind::Plane);
    assert!(tool.pick(doc, cap(block, copy, CapEnd::End)).is_none());
    tool.set_offset(0, Some(viewer::test_support::len(0.005)));
    commit(&mut session, &tool, "on top");
    let depth = viewer::test_support::BOSS_BLOCK[2];
    let (o, _) = datum_frame(&session, datum);
    assert!((o[2] - (depth + 0.005)).abs() < 1e-9, "{o:?}");
    let outcome = session.perform(SessionOp::SetSlot {
        node: block,
        slot: SlotId::Distance,
        value: viewer::props::SlotValue::of(Dimension::Length, 2.0 * depth).expect("a length"),
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let (o, _) = datum_frame(&session, datum);
    assert!(
        (o[2] - (2.0 * depth + 0.005)).abs() < 1e-9,
        "follows: {o:?}"
    );
}
