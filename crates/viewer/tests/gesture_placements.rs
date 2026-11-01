//! **A gesture's delete undoes its takeover**: what a takeover gesture
//! re-pointed from the body it took over — a copy's source, a `⊔`
//! operand, the product slot — its delete hands back, so adding a
//! feature to a placed body and deleting it leaves the world as it was.

#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;

use common::len;
use pncad::document::{Doc, Node, Operand, ProfileProgram, RecipeNodeId, VarId};
use pncad::geom_core::Tol;
use viewer::session::{DocSession, PatternRuleSpec, SessionOp};

const SIDE: f64 = 0.01;

/// A box and its one copy in the world, placed by hand, landed: the
/// session, the box and the copy.
fn placed_box(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived("placed-box", tol), tol);
    let body = common::xy_box_in(&mut session, [SIDE; 3]);
    let (mut session, copy) = common::world_copy_in(&session, body, tol);
    session.pump();
    (session, body, copy)
}

/// The variable `copy`'s placement copies.
fn source_of(doc: &Doc<ProfileProgram>, copy: RecipeNodeId) -> Option<VarId> {
    match doc.node(copy) {
        Some(Node::Place { source, .. }) => Some(source.read),
        _ => None,
    }
}

/// **`doc` holds the nodes `before` held, each as it was, and draws
/// what it drew**: the identity on the world.
fn assert_world_as(doc: &Doc<ProfileProgram>, before: &Doc<ProfileProgram>, what: &str) {
    let mut ids = doc.ids().to_vec();
    let mut was = before.ids().to_vec();
    ids.sort_unstable();
    was.sort_unstable();
    assert_eq!(ids, was, "{what}: the same nodes");
    for id in ids {
        assert_eq!(
            doc.node(id),
            before.node(id),
            "{what}: node {id:?} as it was"
        );
    }
    assert_eq!(
        viewer::world::drawn_nodes(doc),
        viewer::world::drawn_nodes(before),
        "{what}: the same bodies drawn"
    );
}

/// A fillet of one edge of the copy `copy` of `body`.
fn fillet_of(session: &DocSession, body: RecipeNodeId, copy: RecipeNodeId) -> SessionOp {
    let eval = session.evaluation().expect("the inline seam landed");
    let names = pncad::select::all_edges(eval, body);
    SessionOp::AddFillet {
        target: copy,
        radius: len(0.001),
        selection: names[..1].to_vec(),
    }
}

/// **Add a fillet to a placed body, delete it, and the body is placed
/// as it was**: the fillet takes the copy's source over; its delete
/// re-points the source back to the box in the same action, deletes
/// the fillet alone, and leaves every node as the add found it. The
/// button prices that one node, undo is one step back to the filleted
/// copy, and redo one step forward again. Deleting the box itself still
/// takes the copy: nothing upstream is left to hand it to.
///
/// *Breaks if* the delete cascades the copy with the fillet (the body
/// leaves the world), hands it back to anything but the box, or splits
/// the action across history steps.
#[test]
fn deleting_a_feature_hands_its_copy_back_to_the_body() {
    let tol = Tol::witness();
    let (mut session, body, copy) = placed_box(tol);
    let before = session.committed_doc().clone();
    let boxed = source_of(&before, copy).expect("the copy places the box");
    let op = fillet_of(&session, body, copy);
    let fillet = common::session_insert(&mut session, op);
    let filleted = session.committed_doc().output(fillet, 0);
    assert_eq!(
        source_of(session.committed_doc(), copy),
        filleted,
        "the add takes the copy's source over"
    );

    let affordance = session.delete_affordance(fillet);
    assert_eq!(
        affordance.cascade,
        vec![fillet],
        "the button prices the fillet alone"
    );
    assert_eq!(affordance.hover, None, "and names no dependent");
    let steps = session.history().len();
    let out = session.perform(SessionOp::DeleteNode { node: fillet });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert_eq!(
        source_of(doc, copy),
        Some(boxed),
        "the copy places the box again"
    );
    assert!(doc.node(fillet).is_none(), "the fillet is gone");
    assert_world_as(doc, &before, "add then delete");
    assert_eq!(session.history().len(), steps + 1, "one action");

    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert_eq!(
        source_of(session.committed_doc(), copy),
        filleted,
        "one undo restores the fillet and the copy reading it"
    );
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    assert_world_as(
        session.committed_doc(),
        &before,
        "one redo deletes it again",
    );

    let out = session.perform(SessionOp::DeleteNode { node: body });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert!(
        session.committed_doc().node(copy).is_none(),
        "deleting the body takes its copy"
    );
}

/// **A pattern's delete hands the product slot back**: the product
/// reads the copy, a pattern of the copy takes the slot over through a
/// `⊔` of its family, and deleting the pattern takes the `⊔` with it
/// and points the slot at the copy again, as one action.
///
/// *Breaks if* the delete leaves the product reading nothing, or reads
/// the `⊔` it deleted.
#[test]
fn deleting_a_pattern_hands_the_product_back() {
    let tol = Tol::witness();
    let (mut session, _body, copy) = placed_box(tol);
    common::product_is(&mut session, &[copy]);
    let before = viewer::world::product_reads(session.committed_doc());
    assert_eq!(
        before,
        vec![session.committed_doc().output(copy, 0).expect("the copy")],
        "the product reads the copy"
    );
    let row = common::session_insert(
        &mut session,
        SessionOp::AddPattern {
            input: Operand::Node(copy),
            count: 2,
            rule: PatternRuleSpec::Linear {
                direction: common::scl3([1.0, 0.0, 0.0]),
                spacing: len(2.0 * SIDE),
            },
        },
    );
    assert_ne!(
        viewer::world::product_reads(session.committed_doc()),
        before,
        "the pattern takes the product over"
    );
    let steps = session.history().len();
    let out = session.perform(SessionOp::DeleteNode { node: row });
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    let doc = session.committed_doc();
    assert!(doc.node(row).is_none(), "the pattern is gone");
    assert_eq!(
        viewer::world::product_reads(doc),
        before,
        "the product reads the copy again"
    );
    assert_eq!(session.history().len(), steps + 1, "one action");
}
