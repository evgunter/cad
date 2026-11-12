//! **A placement's target is its own property, and no mate changes it**
//! (Ev, `[ev]` PR 6600; `crates/viewer/README.md`, the placement
//! section): the placement editor's first row is the target, "into:
//! <space>", and re-seating it is the one edit that changes a
//! placement's target — a live change, one `SetBundle` carrying it, one
//! undo step. Every door that re-targets a placement uses that edit: the
//! row, Add to the product, and Into a space of its own, a door's change
//! over a copy whose own constraints read its space held in that copy's
//! editor; the combine re-targets nothing. The held cross-space `to` and its "Into <space>"
//! are `place_live`'s rows.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::document::{
    AuthoredNode, Bodies, BodyRead, Doc, Frame, Maintenance, Node, Operand, ProfileProgram,
    RecipeNodeId, SolvedPoses, VarId, stand,
};
use pncad::geom_core::Tol;
use pncad::prelude::{CapEnd, EntityKind, RoleSeg, StableName};
use viewer::matetool::MateTool;
use viewer::session::{BooleanSpec, DocSession, PlaceTarget, Refusal, SessionOp};
use viewer::tree::RowStatus;

/// The bolt: 5 × 5 × 20 mm.
const BOLT: [f64; 3] = [0.005, 0.005, 0.02];
/// The plate: 40 × 40 × 5 mm.
const PLATE: [f64; 3] = [0.04, 0.04, 0.005];

/// `body`'s cap face at `end`, by the name the extrude gives it.
fn cap(body: RecipeNodeId, end: CapEnd) -> StableName {
    StableName {
        kind: EntityKind::Face,
        node: body,
        path: vec![RoleSeg::Cap(end)],
    }
}

/// One Place of `body` into its own space, landed.
fn placed(session: &mut DocSession, body: RecipeNodeId) -> RecipeNodeId {
    let op = SessionOp::place_where_it_stands(session.committed_doc(), body.into())
        .expect("a body in a space");
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    out.made.expect("a copy")
}

/// `node` inserted by hand: a fresh landed session and the node.
fn with(session: &DocSession, node: AuthoredNode, tol: Tol) -> (DocSession, RecipeNodeId) {
    let (doc, id) = viewer::test_support::inserted(session.committed_doc(), node, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    (session, id)
}

/// The bolt, and two copies of it Placed in its own space, copy 2
/// resting on copy 1's top cap: the session, the bolt, and the copies.
fn mated_bolts(tag: &str, tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived(tag, tol), tol);
    let bolt = common::xy_box_in(&mut session, BOLT);
    session.pump();
    let c1 = placed(&mut session, bolt);
    let c2 = placed(&mut session, bolt);
    common::commit_mate(
        &mut session,
        SessionOp::AddMate {
            placement: c2,
            mate: Box::new(crate::fixture::rest(
                Operand::select(Operand::Node(bolt), vec![cap(bolt, CapEnd::Start)]),
                Operand::select(Operand::Node(c1), vec![cap(bolt, CapEnd::End)]),
            )),
            values: Vec::new(),
        },
    );
    (session, bolt, c1, c2)
}

/// The editor's change offered as the panel offers it.
// The door answers the session's own `Refusal`, whatever its width.
#[allow(clippy::result_large_err)]
fn offered(session: &DocSession, tool: &mut MateTool) -> Option<SessionOp> {
    let (doc, ev) = session.landed_pair().expect("landed");
    tool.offer(doc, ev, session.tol(), |op| session.bundle_door(op))
}

/// Where `placement`'s copy stands in its space.
fn stands(session: &DocSession, placement: RecipeNodeId) -> Option<Frame> {
    let (doc, ev) = session.landed_pair().expect("landed");
    SolvedPoses::of(doc, ev).relative(placement)
}

/// Whether two frames agree to well inside the witness tolerance.
fn near(a: &Frame, b: &Frame) -> bool {
    a.columns
        .iter()
        .flatten()
        .chain(&a.translation)
        .zip(b.columns.iter().flatten().chain(&b.translation))
        .all(|(x, y)| (x - y).abs() < 1e-9)
}

/// `placement` as authored.
fn held(doc: &Doc<ProfileProgram>, placement: RecipeNodeId) -> Option<AuthoredNode> {
    doc.node(placement).map(|node| node.authored(doc))
}

/// `placement`'s bundle as authored.
fn bundle(
    doc: &Doc<ProfileProgram>,
    placement: RecipeNodeId,
) -> Vec<pncad::document::Constraint<pncad::document::Formula>> {
    match held(doc, placement) {
        Some(Node::Place { constraints, .. }) => constraints,
        other => panic!("{placement:?} is a placement: {other:?}"),
    }
}

/// **Place carries its target, and the row re-targets in one step**:
/// Place names the body's own space; re-seating the editor's first row
/// at the world is one `SetBundle` carrying the target, one history
/// state, every value keeping its number; one undo takes it back.
/// Setting the target back in the draft restores it (it writes
/// nothing), and setting it back once landed restores the placement.
#[test]
fn the_target_row_re_targets_a_placement_in_one_step() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("target-row", tol), tol);
    let bolt = common::xy_box_in(&mut session, BOLT);
    let plate = common::xy_box_in(&mut session, PLATE);
    session.pump();
    let own = session
        .committed_doc()
        .node_space(bolt)
        .expect("the bolt is in a space");
    assert_ne!(
        own,
        VarId::WORLD,
        "premise: a construction is in a space of its own"
    );
    let op = SessionOp::place_where_it_stands(session.committed_doc(), bolt.into())
        .expect("a body in a space");
    assert_eq!(
        op,
        SessionOp::Place {
            body: bolt.into(),
            target: Operand::Var(own)
        },
        "the viewer's Place names the body's own space"
    );
    let copy = placed(&mut session, bolt);
    let lift = common::transformed(&mut session, copy, &common::slides([0.0, 0.0, 0.01]));
    assert!(session.perform(lift).refusal.is_none());
    session.pump();
    let before = session.committed_doc().clone();
    let was = stands(&session, copy).expect("the copy stands");

    // In the draft: the world, then back where it is, writes nothing.
    let mut tool = MateTool::edit(session.committed_doc(), copy).expect("a placement");
    assert_eq!(
        tool.target(),
        Some(&PlaceTarget::space(own)),
        "into: the bolt's space"
    );
    tool.set_target(PlaceTarget::space(VarId::WORLD));
    assert!(tool.differs_from_base(), "a target change is a change");
    tool.set_target(PlaceTarget::space(own));
    assert!(
        !tool.differs_from_base(),
        "setting the target back restores the draft"
    );
    assert_eq!(offered(&session, &mut tool), None, "and it writes nothing");

    // A body picked after "pick a body" names its space, and seats no mate.
    let with_plate = session.committed_doc();
    tool.pick_target();
    assert!(tool.picking_target());
    let picked = tool.pick(
        with_plate,
        viewer::session::FaceSelection {
            name: cap(plate, CapEnd::End),
            node: plate,
            body: 0,
        },
    );
    assert!(picked.is_none(), "the pick is taken: {picked:?}");
    assert_eq!(
        tool.target(),
        with_plate
            .node_space(plate)
            .map(PlaceTarget::space)
            .as_ref(),
        "the picked body's space"
    );
    assert!(tool.rows().is_empty(), "no mate row is seated");
    assert!(!tool.picking_target(), "one pick names the target");

    // Landed: one `SetBundle` carrying the target, one undo step.
    tool.set_target(PlaceTarget::space(VarId::WORLD));
    let op = offered(&session, &mut tool).expect("the change passes");
    assert!(
        matches!(&op, SessionOp::SetBundle { target, constraints, base, .. }
            if *target == PlaceTarget::space(VarId::WORLD) && *constraints == base.constraints),
        "one SetBundle, the target its one change: {op:?}"
    );
    let states = session.history().len();
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert_eq!(session.history().len(), states + 1, "one history state");
    assert_eq!(session.committed_doc().node_space(copy), Some(VarId::WORLD));
    assert_eq!(
        bundle(session.committed_doc(), copy),
        bundle(&before, copy),
        "every value keeps its number"
    );
    let now = stands(&session, copy).expect("the copy stands");
    assert!(near(&now, &was), "on the world's axes: {now:?} vs {was:?}");
    let mut back = MateTool::edit(session.committed_doc(), copy).expect("a placement");
    back.set_target(PlaceTarget::space(own));
    let op = offered(&session, &mut back).expect("back passes");
    assert!(session.perform(op).refusal.is_none());
    session.pump();
    assert_eq!(
        held(session.committed_doc(), copy),
        held(&before, copy),
        "setting the target back restores the placement"
    );
    for step in 0..2 {
        let undone = session.perform(SessionOp::Undo);
        assert!(
            undone.refusal.is_none(),
            "undo {step}: {:?}",
            undone.refusal
        );
    }
    assert!(
        session.committed_doc().bit_eq(&before),
        "each change was one undo step"
    );
}

/// **A re-target holds on a mate reading the old space, until it is
/// struck**: copy 2 rests on copy 1 in the bolt's space; its target set
/// to the world, its mate still reads the bolt's space, so the copy
/// would not stand and the door holds the change
/// (`Refusal::PlacementFails`), the row marked. "Strike the 1 that read
/// <old space>" is one change, its freed values measured where the copy
/// stood, and the whole lands as one step.
#[test]
fn a_re_target_holds_on_an_old_space_mate_until_it_is_struck() {
    let tol = Tol::witness();
    let (mut session, _, _, c2) = mated_bolts("target-old-mate", tol);
    let old = session.committed_doc().node_space(c2).expect("placed");
    let was = stands(&session, c2).expect("copy 2 stands on copy 1");
    let mut tool = MateTool::edit(session.committed_doc(), c2).expect("a placement");
    tool.set_target(PlaceTarget::space(VarId::WORLD));
    assert_eq!(
        offered(&session, &mut tool),
        None,
        "the door holds the change"
    );
    assert!(
        matches!(tool.pending(), Some(Refusal::PlacementFails { .. })),
        "the copy would not stand: {:?}",
        tool.pending()
    );
    let (doc, ev) = session.landed_pair().expect("landed");
    assert_eq!(
        tool.off_target(doc, ev),
        [(viewer::matetool::Reading::Mate(0), old)],
        "the mate's row is marked as reading the old space"
    );
    let states = session.history().len();
    tool.strike_reading(doc, ev, old);
    assert!(tool.rows()[0].struck(), "the row is struck");
    let op = offered(&session, &mut tool).expect("the struck draft passes");
    let SessionOp::SetBundle { constraints, .. } = &op else {
        panic!("a SetBundle: {op:?}")
    };
    assert!(
        constraints.iter().all(|c| c.mate().is_none()),
        "the mate is struck: {constraints:?}"
    );
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert_eq!(session.history().len(), states + 1, "one step");
    assert_eq!(session.committed_doc().node_space(c2), Some(VarId::WORLD));
    let now = stands(&session, c2).expect("copy 2 stands in the world");
    assert!(
        near(&now, &was),
        "the freed values are measured where it stood: {now:?} vs {was:?}"
    );
}

/// **A re-target admits the reads of the copy it leaves in the old
/// space, and reports each**: a `⊔` reads copy 1 in the bolt's space;
/// re-targeting copy 1 into the world lands, the outcome names the `⊔`
/// (`Maintenance::StrandedSpace`), the `⊔`'s tree row is marked failed,
/// and one undo takes it back.
#[test]
fn a_re_target_admits_and_reports_a_stranded_reader() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("target-strand", tol), tol);
    let bolt = common::xy_box_in(&mut session, BOLT);
    session.pump();
    let c1 = placed(&mut session, bolt);
    let c2 = placed(&mut session, bolt);
    let (mut session, reader) = with(
        &session,
        Node::Assemble {
            members: Bodies::Spelled(vec![
                BodyRead::plain(Operand::Node(c1)),
                BodyRead::plain(Operand::Node(c2)),
            ]),
        },
        tol,
    );
    let before = session.committed_doc().clone();
    let mut tool = MateTool::edit(session.committed_doc(), c1).expect("a placement");
    tool.set_target(PlaceTarget::space(VarId::WORLD));
    let op = offered(&session, &mut tool).expect("the re-target is admitted");
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        outcome.maintenance.iter().any(|row| matches!(row,
            Maintenance::StrandedSpace { node, .. } if node.id() == reader)),
        "the outcome names the stranded reader: {:?}",
        outcome.maintenance
    );
    session.pump();
    let row = session
        .tree_rows()
        .into_iter()
        .find(|row| row.id == reader)
        .expect("the reader's row");
    assert!(
        matches!(row.status, RowStatus::Failed { .. }),
        "its row is marked: {row:?}"
    );
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo takes it back"
    );
}

/// **A door's target change over a copy whose own constraints read its
/// space opens the placement editor holding it**: `op` refuses
/// `Refusal::TargetHeld` naming `target`, recording nothing, and the
/// tools open the editor on `placement` with that target set — answered.
fn held_in_editor(
    session: &mut DocSession,
    op: SessionOp,
    placement: RecipeNodeId,
    target: &PlaceTarget,
) -> MateTool {
    let states = session.history().len();
    let refused = session.perform(op);
    let Some(refusal) = refused.refusal else {
        panic!("the door holds the change")
    };
    assert!(
        matches!(&refusal, Refusal::TargetHeld { placement: p, target: t, reading: 1, .. }
            if *p == placement && t == target),
        "held, naming the copy and the target: {refusal:?}"
    );
    assert_eq!(session.history().len(), states, "nothing is recorded");
    let mut tools = viewer::tools::Tools::new();
    assert!(
        tools.held_target(&refusal, session.committed_doc()),
        "the editor opens"
    );
    let tool = tools.mate().expect("the placement editor is open").clone();
    assert_eq!(tool.door().map(|door| door.placement), Some(placement));
    assert_eq!(tool.target(), Some(target), "with the door's target set");
    tool
}

/// **The held change in the editor a door opened**: offered, it holds
/// `PlacementFails` with the mate reading `old` marked; "strike the 1
/// that read <old>" lands it as one step. Answers the step's outcome.
fn struck_and_landed(
    session: &mut DocSession,
    mut tool: MateTool,
    old: VarId,
) -> viewer::session::OpOutcome {
    assert_eq!(offered(session, &mut tool), None, "the change is held");
    assert!(
        matches!(tool.pending(), Some(Refusal::PlacementFails { .. })),
        "{:?}",
        tool.pending()
    );
    let (doc, ev) = session.landed_pair().expect("landed");
    assert_eq!(
        tool.off_target(doc, ev),
        [(viewer::matetool::Reading::Mate(0), old)],
        "the mate's row is marked"
    );
    tool.strike_reading(doc, ev, old);
    let op = offered(session, &mut tool).expect("the struck change passes");
    let states = session.history().len();
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    assert_eq!(session.history().len(), states + 1, "one step");
    outcome
}

/// **Into a space of its own is the target edit**: a `new_space()` and
/// the placement re-targeted into it, the values kept rather than
/// zeroed. On a copy whose own mate reads the space it leaves, the change
/// is held in the copy's placement editor, its target set to a new space;
/// striking the mate there lands it.
#[test]
fn into_own_space_keeps_the_values_and_holds_a_mated_copy_in_its_editor() {
    let tol = Tol::witness();
    let (mut session, bolt, c1, c2) = mated_bolts("target-own-space", tol);
    let lift = common::transformed(&mut session, c1, &common::slides([0.0, 0.0, 0.01]));
    assert!(session.perform(lift).refusal.is_none());
    session.pump();
    let was = stands(&session, c1).expect("copy 1 stands");
    let kept = bundle(session.committed_doc(), c1);
    let old = session.committed_doc().node_space(c2).expect("placed");
    let tool = held_in_editor(
        &mut session,
        SessionOp::IntoOwnSpace { placement: c2 },
        c2,
        &PlaceTarget::NewSpace,
    );
    struck_and_landed(&mut session, tool, old);
    let doc = session.committed_doc();
    let own = doc.node_space(c2).expect("placed");
    assert!(
        doc.defined_by(own)
            .is_some_and(|(node, _)| matches!(doc.node(node), Some(Node::Space))),
        "copy 2 in a space of its own"
    );

    let outcome = session.perform(SessionOp::IntoOwnSpace { placement: c1 });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let doc = session.committed_doc();
    let space = doc.node_space(c1).expect("placed");
    assert!(
        doc.defined_by(space)
            .is_some_and(|(node, _)| matches!(doc.node(node), Some(Node::Space))),
        "a space of its own"
    );
    assert_ne!(Some(space), doc.node_space(bolt));
    assert_eq!(bundle(doc, c1), kept, "the values are kept, not zeroed");
    let now = stands(&session, c1).expect("copy 1 stands");
    assert!(near(&now, &was), "standing as it stood, on the new axes");
}

/// **Add to the product moves an unread copy by the target edit**: the
/// copy's placement re-targeted into the world, its values kept.
#[test]
fn add_to_the_product_re_targets_by_the_one_edit() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("target-product", tol), tol);
    let bolt = common::xy_box_in(&mut session, BOLT);
    session.pump();
    let copy = placed(&mut session, bolt);
    let lift = common::transformed(&mut session, copy, &common::slides([0.0, 0.0, 0.01]));
    assert!(session.perform(lift).refusal.is_none());
    session.pump();
    let kept = bundle(session.committed_doc(), copy);
    let outcome = session.perform(SessionOp::AddToProduct {
        node: copy,
        body: None,
    });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let doc = session.committed_doc();
    assert_eq!(doc.node_space(copy), Some(VarId::WORLD), "into the world");
    assert_eq!(bundle(doc, copy), kept, "its values kept");
}

/// **Add to the product on a copy whose own mate reads its space holds
/// the move in the copy's editor**: no fresh world copy is placed; the
/// editor opens with the target set to the world, and once the mate is
/// struck the copy is in the world and the door adds it.
#[test]
fn add_to_the_product_holds_a_mated_copy_in_its_editor() {
    let tol = Tol::witness();
    let (mut session, _, _, c2) = mated_bolts("target-product-mated", tol);
    let placements = session.committed_doc().placements().len();
    let old = session.committed_doc().node_space(c2).expect("placed");
    let tool = held_in_editor(
        &mut session,
        SessionOp::AddToProduct {
            node: c2,
            body: None,
        },
        c2,
        &PlaceTarget::space(VarId::WORLD),
    );
    assert_eq!(
        session.committed_doc().placements().len(),
        placements,
        "no fresh copy is placed"
    );
    struck_and_landed(&mut session, tool, old);
    assert_eq!(session.committed_doc().node_space(c2), Some(VarId::WORLD));
    let added = session.perform(SessionOp::AddToProduct {
        node: c2,
        body: None,
    });
    assert!(added.refusal.is_none(), "{:?}", added.refusal);
    let copy = session.committed_doc().output(c2, 0).expect("a copy");
    assert!(viewer::world::product_reads(session.committed_doc()).contains(&copy));
}

/// **A product copy moved out of the world and mated there keeps its
/// recourse**: the product reads copy 2, re-targeted back into the bolt's
/// space and rested on copy 1 there. The `ProductOutsideWorld` recourse
/// still offers "Move … into the world", and Add to the product on it
/// holds the move in its editor rather than saying it is in the product.
#[test]
fn a_mated_product_copy_outside_the_world_keeps_its_recourse() {
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("target-outside", tol), tol);
    let bolt = common::xy_box_in(&mut session, BOLT);
    session.pump();
    let c1 = placed(&mut session, bolt);
    let c2 = placed(&mut session, bolt);
    let home = session.committed_doc().node_space(c2).expect("placed");
    let added = session.perform(SessionOp::AddToProduct {
        node: c2,
        body: None,
    });
    assert!(added.refusal.is_none(), "{:?}", added.refusal);
    session.pump();
    common::into_space(&mut session, c2, home);
    session.pump();
    common::commit_mate(
        &mut session,
        SessionOp::AddMate {
            placement: c2,
            mate: Box::new(crate::fixture::rest(
                Operand::select(Operand::Node(bolt), vec![cap(bolt, CapEnd::Start)]),
                Operand::select(Operand::Node(c1), vec![cap(bolt, CapEnd::End)]),
            )),
            values: Vec::new(),
        },
    );
    let fault = session.product_fault();
    assert!(
        matches!(
            fault.map(pncad::document::ProductError::kind),
            Some(pncad::document::ProductErrorKind::ProductOutsideWorld)
        ),
        "the premise: the product reads a copy outside the world: {fault:?}"
    );
    let recourse = viewer::frame::product_recourse(fault, session.committed_doc());
    let [(label, op)] = &recourse[..] else {
        panic!("one recourse: {recourse:?}")
    };
    assert!(label.starts_with("Move "), "{label}");
    assert_eq!(
        *op,
        SessionOp::AddToProduct {
            node: c2,
            body: None
        }
    );
    held_in_editor(
        &mut session,
        op.clone(),
        c2,
        &PlaceTarget::space(VarId::WORLD),
    );
}

/// **A value read off the old space is marked and struck like a mate**:
/// a copy placed by a slide along another body's edge direction in the
/// bolt's space, re-targeted into the world. The value reads the bolt's
/// space, so the change is held with that value marked; "strike the 1
/// that read <old>" measures its freedom afresh, and the change lands.
#[test]
fn a_value_read_off_the_old_space_is_marked_and_struck() {
    use pncad::document::pose::{PoseDef, PoseRole, RoleRead};
    let tol = Tol::witness();
    let mut session = common::inline_without_door(Doc::empty_derived("target-value", tol), tol);
    let bolt = common::xy_box_in(&mut session, BOLT);
    session.pump();
    let c1 = placed(&mut session, bolt);
    let space = session.committed_doc().node_space(c1).expect("placed");
    let along = RoleRead::through(
        Operand::Pose(Box::new(PoseDef::Plane {
            face: Operand::select(Operand::Node(c1), vec![cap(bolt, CapEnd::End)]),
        })),
        PoseRole::Normal,
    );
    let values = vec![(pncad::document::Coord::SlideAlong(along), common::len(0.01))];
    let (mut session, c2) = with(
        &session,
        Node::place(
            BodyRead::plain(Operand::Node(bolt)),
            Operand::Var(space),
            pncad::document::Placed::unmated(values).bundle(),
        ),
        tol,
    );
    let was = stands(&session, c2).expect("copy 2 stands");
    let mut tool = MateTool::edit(session.committed_doc(), c2).expect("a placement");
    tool.set_target(PlaceTarget::space(VarId::WORLD));
    assert_eq!(offered(&session, &mut tool), None, "the change is held");
    let (doc, ev) = session.landed_pair().expect("landed");
    let off = tool.off_target(doc, ev);
    let [(viewer::matetool::Reading::Value(id), read)] = off[..] else {
        panic!("the value is marked as reading the bolt's space: {off:?}")
    };
    assert_eq!(read, space);
    tool.strike_reading(doc, ev, space);
    assert_eq!(tool.struck_values(), [id], "the value is struck");
    let op = offered(&session, &mut tool).expect("the struck change passes");
    let outcome = session.perform(op);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    let doc = session.committed_doc();
    assert_eq!(doc.node_space(c2), Some(VarId::WORLD));
    assert!(
        bundle(doc, c2).iter().all(|c| c
            .target_reads()
            .into_iter()
            .all(|read| doc.operand_space(read) != Some(space))),
        "nothing in the bundle reads the bolt's space"
    );
    let now = stands(&session, c2).expect("copy 2 stands in the world");
    assert!(
        near(&now, &was),
        "measured where it stood: {now:?} vs {was:?}"
    );
}

/// **A combine moves no existing copy** (PLACING's probe, filed as
/// `work/placing/a-combine-re-targets-a-copy-another-copy-is-mated-to.md`):
/// a plate copy in a space of its own, and copy 1 of the bolt, which
/// copy 2 rests on. Their union refuses, naming copy 1 and pointing at
/// its "into" row; nothing is recorded, so copy 2 is not stranded. A
/// body nothing places is still placed into the first operand's space.
#[test]
fn the_combine_refuses_a_copy_in_another_space() {
    let tol = Tol::witness();
    let (session, _, c1, _) = mated_bolts("target-combine", tol);
    let (mut session, room) = with(&session, Node::Space, tol);
    let room = session.committed_doc().output(room, 0).expect("a space");
    let plate_box = common::xy_box_in(&mut session, PLATE);
    session.pump();
    let (mut session, plate) = with(
        &session,
        Node::place(
            BodyRead::plain(Operand::Node(plate_box)),
            Operand::Var(room),
            stand().bundle(),
        ),
        tol,
    );
    let before = session.committed_doc().clone();
    let refused = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![plate.into(), c1.into()]),
    });
    assert!(
        matches!(&refused.refusal, Some(Refusal::CopyInAnotherSpace { copy, into, .. })
            if copy.id() == c1 && into.id() == room),
        "the copy in another space refuses, naming it: {:?}",
        refused.refusal
    );
    let said = refused
        .refusal
        .as_ref()
        .map(ToString::to_string)
        .unwrap_or_default();
    assert!(
        said.contains("\"into\" row"),
        "it points at the copy's into row: {said}"
    );
    assert!(
        session.committed_doc().bit_eq(&before),
        "nothing is recorded"
    );

    let loose = common::xy_box_in(&mut session, BOLT);
    session.pump();
    let union = common::session_insert(
        &mut session,
        SessionOp::AddBoolean {
            spec: BooleanSpec::Union(vec![plate.into(), loose.into()]),
        },
    );
    let doc = session.committed_doc();
    assert_eq!(
        doc.node_space(union),
        Some(room),
        "the union is in the plate's space"
    );
    assert!(
        doc.placements()
            .into_iter()
            .any(|p| doc.copied_read(p) == doc.read_of_node(loose)
                && doc.node_space(p) == Some(room)),
        "the body nothing placed is placed into the first operand's space"
    );
}

/// The reviewer's bench (PR 6850's review): [`mated_bolts`] and the plate
/// beside them, in a space of its own.
fn bolts_and_plate(
    tag: &str,
    tol: Tol,
) -> (
    DocSession,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let (mut session, bolt, c1, c2) = mated_bolts(tag, tol);
    let plate = common::xy_box_in(&mut session, PLATE);
    session.pump();
    (session, bolt, plate, c1, c2)
}

/// `bolt`'s underside resting on `to`.
fn rest_on(bolt: RecipeNodeId, to: Operand) -> pncad::document::Mate<pncad::document::Formula> {
    crate::fixture::rest(
        Operand::select(Operand::Node(bolt), vec![cap(bolt, CapEnd::Start)]),
        to,
    )
}

/// **"Into" the loaded space keeps what the draft holds** (PR 6850's
/// review, M1): copy 2's target set to the world is held, and x typed
/// over it. Its row offers "Into <bolt's space>", the space it was
/// loaded in; clicking it sets the target back, x still in the draft and
/// offered with it — nothing set aside, nothing replayed later.
#[test]
fn into_the_loaded_space_keeps_the_typed_value() {
    let tol = Tol::witness();
    let (mut session, _, _, _, c2) = bolts_and_plate("target-into-back", tol);
    let old = session.committed_doc().node_space(c2).expect("placed");
    let mut tool = MateTool::edit(session.committed_doc(), c2).expect("a placement");
    tool.set_target(PlaceTarget::space(VarId::WORLD));
    assert_eq!(offered(&session, &mut tool), None, "held");
    let (x, by) = common::slides([0.003, 0.0, 0.0]).remove(0);
    tool.type_value(x, by);
    assert_eq!(offered(&session, &mut tool), None, "still held");
    let (doc, ev) = session.landed_pair().expect("landed");
    assert_eq!(
        tool.off_target(doc, ev),
        [(viewer::matetool::Reading::Mate(0), old)],
        "Into <old> offered"
    );
    tool.retarget_into(old);
    assert_eq!(tool.typed().len(), 1, "x is kept in the draft");
    let op = offered(&session, &mut tool).expect("x passes over the loaded target");
    assert!(
        matches!(&op, SessionOp::SetBundle { target, .. } if *target == PlaceTarget::space(old)),
        "{op:?}"
    );
    let mut tools = viewer::tools::Tools::new();
    tools.open_mate(tool);
    let outcome = session.perform(op.clone());
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    session.pump();
    tools.answered(&op, outcome.made, true, session.committed_doc());
    let editor = tools.mate_mut().expect("open");
    assert!(
        !editor.differs_from_base(),
        "nothing waits to be made again"
    );
    assert_eq!(offered(&session, editor), None, "and nothing is replayed");
}

/// **A held "Into" that does not pass alone puts back what it set aside,
/// whatever the offer answered** (M1's class): with x typed over the
/// loaded target and a rest onto the plate, "Into <plate's space>" sets
/// both aside; the target alone is refused (the loaded rest reads the
/// bolt's space), so they are put back at once and counted as the
/// draft's.
#[test]
fn a_refused_into_counts_and_puts_back_what_it_set_aside() {
    let tol = Tol::witness();
    let (mut session, bolt, plate, _, c2) = bolts_and_plate("target-into-mix", tol);
    let plate_space = session
        .committed_doc()
        .node_space(plate)
        .expect("in a space");
    let before = session.committed_doc().clone();
    let states = session.history().len();
    let mut tool = MateTool::edit(session.committed_doc(), c2).expect("a placement");
    tool.strike(0, true);
    tool.add_written(
        session.committed_doc(),
        &rest_on(
            bolt,
            Operand::select(Operand::Node(plate), vec![cap(plate, CapEnd::End)]),
        ),
    );
    let (x, by) = common::slides([0.003, 0.0, 0.0]).remove(0);
    tool.type_value(x, by);
    assert_eq!(
        offered(&session, &mut tool),
        None,
        "the cross-space to is held"
    );
    let rows = tool.rows().to_vec();
    tool.retarget_into(plate_space);
    assert!(
        tool.differs_from_base(),
        "what is set aside counts as the draft's"
    );
    assert!(tool.holds_changes(), "and an undo would take it first");
    let first = offered(&session, &mut tool);
    let op = first
        .or_else(|| offered(&session, &mut tool))
        .unwrap_or_else(|| panic!("the whole draft passes: {:?}", tool.pending()));
    assert_eq!(tool.rows(), &rows[..], "the rows are put back");
    let SessionOp::SetBundle {
        target,
        constraints,
        ..
    } = &op
    else {
        panic!("{op:?}")
    };
    assert_eq!(*target, PlaceTarget::space(plate_space));
    assert_eq!(constraints.iter().filter(|c| c.mate().is_some()).count(), 1);
    let out = session.perform(op);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    assert_eq!(session.history().len(), states + 1, "one step");
    let after = session.committed_doc().clone();
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(session.committed_doc().bit_eq(&before), "one undo");
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    assert!(session.committed_doc().bit_eq(&after), "one redo");
}

/// **An undo between "Into" and the held mate reloads the editor**
/// (PR 6850's review, N1; the editor's rule for any draft over a bundle
/// an undo changed, `MateToolEvent::Reloaded`): the target lands as step
/// one and the held rest is made again over it; one undo restores the
/// document exactly, and the editor reloads the bundle as it now is, the
/// held rest gone with the step it waited on.
#[test]
fn an_undo_between_into_and_the_held_mate_reloads_the_editor() {
    let tol = Tol::witness();
    let (mut session, bolt, plate, c1, _) = bolts_and_plate("target-undo-mid", tol);
    let plate_space = session
        .committed_doc()
        .node_space(plate)
        .expect("in a space");
    let before = session.committed_doc().clone();
    let mut tool = MateTool::edit(session.committed_doc(), c1).expect("a placement");
    tool.add_written(
        session.committed_doc(),
        &rest_on(
            bolt,
            Operand::select(Operand::Node(plate), vec![cap(plate, CapEnd::End)]),
        ),
    );
    assert_eq!(offered(&session, &mut tool), None, "held");
    let mut tools = viewer::tools::Tools::new();
    tools.open_mate(tool);
    tools.mate_mut().expect("open").retarget_into(plate_space);
    let into = offered(&session, tools.mate_mut().expect("open")).expect("the target alone");
    let out = session.perform(into.clone());
    session.pump();
    tools.answered(
        &into,
        out.made,
        out.refusal.is_none(),
        session.committed_doc(),
    );
    assert_eq!(session.committed_doc().node_space(c1), Some(plate_space));
    assert_eq!(
        tools.mate().expect("open").rows().len(),
        1,
        "the held mate is made again over it"
    );
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    session.pump();
    assert!(session.committed_doc().bit_eq(&before), "one undo");
    let committed = session.committed_doc().clone();
    let events = tools
        .mate_mut()
        .expect("open")
        .reconcile(&committed, session.landed_pair().expect("landed"));
    assert!(
        events
            .iter()
            .any(|event| matches!(event, viewer::matetool::MateToolEvent::Reloaded { .. })),
        "the reload is said: {events:?}"
    );
    let editor = tools.mate().expect("open");
    assert!(editor.rows().is_empty(), "the bundle as it now is");
    assert!(!editor.differs_from_base());
}

/// **"Into" clicked while a change lands is a step of its own** (PR
/// 6850's review, N2): x typed and landing, then "Into <plate's space>"
/// and a y typed after it. Once x lands, the target is offered alone over
/// it — y set aside — and y follows as the next step.
#[test]
fn into_made_while_a_change_lands_is_a_step_of_its_own() {
    let tol = Tol::witness();
    let mut session =
        common::inline_without_door(Doc::empty_derived("target-into-landing", tol), tol);
    let bolt = common::xy_box_in(&mut session, BOLT);
    let plate = common::xy_box_in(&mut session, PLATE);
    session.pump();
    let copy = placed(&mut session, bolt);
    let plate_space = session
        .committed_doc()
        .node_space(plate)
        .expect("in a space");
    let mut tools = viewer::tools::Tools::new();
    tools.open_mate(MateTool::edit(session.committed_doc(), copy).expect("a placement"));
    let [x, y, _] = <[_; 3]>::try_from(common::slides([0.003, 0.004, 0.0])).expect("three slides");
    tools.mate_mut().expect("open").type_value(x.0, x.1);
    let first = offered(&session, tools.mate_mut().expect("open")).expect("x passes");
    tools.mate_mut().expect("open").retarget_into(plate_space);
    tools.mate_mut().expect("open").type_value(y.0.clone(), y.1);
    let states = session.history().len();
    let out = session.perform(first.clone());
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    tools.answered(&first, out.made, true, session.committed_doc());
    let into = offered(&session, tools.mate_mut().expect("open")).expect("the target alone");
    let SessionOp::SetBundle {
        target,
        constraints,
        base,
        ..
    } = &into
    else {
        panic!("{into:?}")
    };
    assert_eq!(*target, PlaceTarget::space(plate_space), "the target");
    assert_eq!(*constraints, base.constraints, "alone: y waits");
    let out = session.perform(into.clone());
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    session.pump();
    tools.answered(&into, out.made, true, session.committed_doc());
    let then = offered(&session, tools.mate_mut().expect("open")).expect("y passes");
    let SessionOp::SetBundle { constraints, .. } = &then else {
        panic!("{then:?}")
    };
    assert!(
        constraints
            .iter()
            .any(|c| c.value().is_some_and(|v| v.coord == y.0)),
        "y follows as its own step"
    );
    assert!(session.perform(then).refusal.is_none());
    assert_eq!(
        session.history().len(),
        states + 3,
        "x, the target, y: three steps"
    );
}

/// **A combine refuses an unmated copy in another space too** (PR
/// 6850's review): a third copy of the bolt, never mated, and the plate.
#[test]
fn the_combine_refuses_an_unmated_copy_in_another_space() {
    let tol = Tol::witness();
    let (mut session, bolt, plate, _, _) = bolts_and_plate("target-combine-unmated", tol);
    let c3 = placed(&mut session, bolt);
    let before = session.committed_doc().clone();
    let out = session.perform(SessionOp::AddBoolean {
        spec: BooleanSpec::Union(vec![plate.into(), c3.into()]),
    });
    assert!(
        matches!(&out.refusal, Some(Refusal::CopyInAnotherSpace { copy, .. }) if copy.id() == c3),
        "{:?}",
        out.refusal
    );
    assert!(session.committed_doc().bit_eq(&before));
}
