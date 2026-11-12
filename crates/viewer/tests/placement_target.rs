//! **A placement's target is its own property, and no mate changes it**
//! (Ev, `[ev]` PR 6600; `crates/viewer/README.md`, the placement
//! section): the placement editor's first row is the target, "into:
//! <space>", and re-seating it is the one edit that changes a
//! placement's target — a live change, one `SetBundle` carrying it, one
//! undo step. Every door that re-targets a placement uses that edit: the
//! row, Add to the product, and Into a space of its own; the combine
//! re-targets nothing. The held cross-space `to` and its "Into <space>"
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
        [(0, old)],
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

/// **Into a space of its own is the target edit**: a `new_space()` and
/// the placement re-targeted into it, the values kept rather than
/// zeroed; a copy whose mate reads the space it leaves refuses, as the
/// row's change holds.
#[test]
fn into_own_space_keeps_the_values_and_holds_on_a_mate() {
    let tol = Tol::witness();
    let (mut session, bolt, c1, c2) = mated_bolts("target-own-space", tol);
    let lift = common::transformed(&mut session, c1, &common::slides([0.0, 0.0, 0.01]));
    assert!(session.perform(lift).refusal.is_none());
    session.pump();
    let was = stands(&session, c1).expect("copy 1 stands");
    let kept = bundle(session.committed_doc(), c1);
    let states = session.history().len();
    let refused = session.perform(SessionOp::IntoOwnSpace { placement: c2 });
    assert!(
        matches!(refused.refusal, Some(Refusal::PlacementFails { .. })),
        "copy 2's mate reads the space it would leave: {:?}",
        refused.refusal
    );
    assert_eq!(session.history().len(), states, "nothing is recorded");
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
