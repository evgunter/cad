//! **A new body lands unplaced** (`crates/viewer/README.md`, *The mate
//! tool commits into a placement's bundle*, Ev 2026-10-31): no gesture
//! writes a world `Place` on the person's behalf. A split or a pattern of
//! a fresh box takes the box over, so every drawn body stays pickable;
//! a takeover re-points what read the body along the path it was seen
//! through — a copy's source, a `⊔` operand, the product slot — in the
//! one action; and "Add part…" lands each part in a space of its own,
//! drawn clear of what is drawn, which an undo and a redo keep.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use common::asm;
use pncad::document::{
    Bodies, BodyRead, Doc, DocEdit, Node, Operand, ProfileProgram, RecipeNodeId, VarId,
};
use pncad::geom_core::Tol;
use viewer::session::{DocSession, PatternRuleSpec, SessionOp};

/// The box: 20 × 20 × 10 mm, centred on its sketch origin.
const BOX: [f64; 3] = [0.02, 0.02, 0.01];

/// A session holding the box as it lands, landed: the session and the
/// extrude.
fn fresh_box(tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived("lands-unplaced", tol), tol);
    let body = common::xy_box_in(&mut session, BOX);
    session.pump();
    (session, body)
}

/// A pattern of `input`, two members `2·BOX[0]` apart along `x`.
fn pattern_op(input: RecipeNodeId, fused: bool) -> SessionOp {
    let rule = PatternRuleSpec::Linear {
        direction: common::scl3([1.0, 0.0, 0.0]),
        spacing: common::len(2.0 * BOX[0]),
    };
    if fused {
        SessionOp::AddFusedPattern {
            input: input.into(),
            count: 2,
            rule,
        }
    } else {
        SessionOp::AddPattern {
            input: input.into(),
            count: 2,
            rule,
        }
    }
}

/// **A split and a pattern of a fresh box leave every drawn body
/// pickable** (`a-split-commits-and-the-picture-does-not-change`, and
/// `an-instances-pattern-draws-nothing`): the box lands unplaced, so the
/// split reads it as a body and is drawn in its place — a ray from above
/// meets the split's upper half, not two coincident bodies — and a
/// pattern, fused or not, draws its family where the box stood, member 0
/// on the box's footprint and member 1 beside it, with the box itself
/// no longer drawn.
///
/// *Breaks if* a creation writes a world copy of the box (the box's copy
/// and the split coincide and the pick is ambiguous), or the instances
/// pattern draws nothing.
#[test]
fn a_split_and_a_pattern_of_a_fresh_box_leave_every_drawn_body_pickable() {
    let tol = Tol::witness();
    let (mut session, body) = fresh_box(tol);
    let split = common::session_insert(
        &mut session,
        SessionOp::AddSplit {
            target: body.into(),
            origin: common::len3([0.0, 0.0, BOX[2] / 2.0]),
            normal: common::scl3([0.0, 0.0, 1.0]),
        },
    );
    session.pump();
    assert_eq!(
        viewer::world::drawn_nodes(session.committed_doc()),
        [split],
        "the split takes the box over"
    );
    let top = asm::pick_face(&session, &common::down_at(0.0, 0.0));
    assert_eq!(top.node, split, "the ray from above meets the split");
    let under = asm::pick_face(&session, &common::up_at(0.0, 0.0));
    assert_eq!(under.node, split, "and from below too");
    assert_ne!(top.body, under.body, "on its two halves");

    for fused in [false, true] {
        let (mut session, body) = fresh_box(tol);
        let made = common::session_insert(&mut session, pattern_op(body, fused));
        session.pump();
        let doc = session.committed_doc();
        assert_eq!(
            viewer::world::drawn_nodes(doc),
            [made],
            "fused {fused}: the family is drawn and the box is not"
        );
        for x in [0.0, 2.0 * BOX[0]] {
            let hit = asm::pick_face(&session, &common::down_at(x, 0.0));
            assert_eq!(
                hit.node, made,
                "fused {fused}: the member at x = {x} is picked"
            );
        }
    }
}

/// Two boxes placed into the world by hand, a union of the two copies
/// in the world, and the product a `⊔` of that union and a second copy
/// of the second box: the session, the union and the product's `⊔`.
fn product_of_a_union(tol: Tol) -> (DocSession, RecipeNodeId, RecipeNodeId) {
    let (mut session, a) = fresh_box(tol);
    let b = common::xy_box_in(&mut session, BOX);
    let (doc, a_copy) = common::world_copy(session.committed_doc(), a, tol);
    let (doc, b_copy) = common::world_copy(&doc, b, tol);
    let copy = |node| Operand::Var(doc.output(node, 0).expect("a copy's output"));
    let (doc, union) = common::inserted(
        &doc,
        Node::Union {
            members: Bodies::Spelled(vec![
                BodyRead::plain(copy(a_copy)),
                BodyRead::plain(copy(b_copy)),
            ]),
        },
        tol,
    );
    let (doc, other) = common::inserted(
        &doc,
        Node::place(
            BodyRead::plain(Operand::Node(b)),
            Operand::Var(VarId::WORLD),
            pncad::document::stand().bundle(),
        ),
        tol,
    );
    let (doc, assemble) = common::inserted(
        &doc,
        Node::Assemble {
            members: Bodies::Spelled(vec![
                BodyRead::plain(Operand::Node(union)),
                BodyRead::plain(Operand::Node(other)),
            ]),
        },
        tol,
    );
    let (doc, _) = common::edited(
        &doc,
        DocEdit::SetProduct {
            product: Some(Operand::Node(assemble)),
        },
        tol,
    );
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    (session, union, assemble)
}

/// The read at member `k` of the `⊔` `assemble`.
fn member(doc: &Doc<ProfileProgram>, assemble: RecipeNodeId, k: usize) -> VarId {
    let Some(Node::Assemble {
        members: Bodies::Spelled(reads),
    }) = doc.node(assemble)
    else {
        panic!("a spelled `⊔`: {:?}", doc.node(assemble));
    };
    reads[k].read
}

/// **A takeover re-points a `⊔` operand and the product slot**
/// (`a-takeover-re-points-along-the-seen-path-and-a-creation-joins-the-product`,
/// half (a)): a fused pattern of a body the product's `⊔` reads takes
/// that operand, an instances pattern takes it through a `⊔` over its
/// family (an operand reads one body), and a body the product slot reads
/// itself is re-pointed in the slot — each in the gesture's one action,
/// so one undo puts the reader back.
///
/// *Breaks if* the takeover re-points placement sources alone (the
/// operand and the slot keep reading the body, which the pattern then
/// draws beside its family), or writes the re-point as a second action.
#[test]
fn a_takeover_re_points_a_union_operand_and_the_product_slot() {
    let tol = Tol::witness();
    let (mut session, union, assemble) = product_of_a_union(tol);
    let read_union = session.committed_doc().output(union, 0);
    assert_eq!(
        Some(member(session.committed_doc(), assemble, 0)),
        read_union
    );

    // A fused pattern: the operand reads the fused union.
    let before = session.committed_doc().clone();
    let fused = common::session_insert(&mut session, pattern_op(union, true));
    let doc = session.committed_doc();
    assert_eq!(
        Some(member(doc, assemble, 0)),
        doc.output(fused, 0),
        "the `⊔` operand reads the fused pattern"
    );
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    assert!(
        session.committed_doc().bit_eq(&before),
        "one undo takes the pattern and the re-point together"
    );

    // An instances pattern: the operand reads a `⊔` over the family.
    let row = common::session_insert(&mut session, pattern_op(union, false));
    let doc = session.committed_doc();
    let whole = doc
        .operation_of(member(doc, assemble, 0))
        .expect("the operand reads a node");
    assert!(
        matches!(
            doc.node(whole),
            Some(Node::Assemble { members: Bodies::Family(family) })
                if Some(family.read) == doc.output(row, 0)
        ),
        "the operand reads a `⊔` over the family: {:?}",
        doc.node(whole)
    );
    assert!(session.perform(SessionOp::Undo).refusal.is_none());

    // The product slot reading the union itself.
    let set = session.perform(SessionOp::SetProduct {
        bodies: vec![Operand::Node(union)],
    });
    assert!(set.refusal.is_none(), "{:?}", set.refusal);
    let fused = common::session_insert(&mut session, pattern_op(union, true));
    let doc = session.committed_doc();
    assert_eq!(
        doc.product(),
        doc.output(fused, 0),
        "the product slot reads the fused pattern"
    );
}

/// **Two "Add part…" of one file are drawn apart, and stay apart across
/// an undo and a redo** (`a-new-body-lands-placed-in-the-world`, *Two
/// copies of one part land on each other*): each lands in a fresh space
/// by the identity bundle; the second's space is written a display frame
/// stepping it clear of the first along `x` once its picture lands; an
/// undo of the second insert keeps that frame and the redo, which mints
/// the same space again (IDENTITY DI1), draws it there.
///
/// *Breaks if* the inserts share a space or land in the world, the
/// second is drawn on the first, or the undo's prune drops the frame.
#[test]
fn two_inserted_parts_are_drawn_apart_and_stay_apart_across_undo_and_redo() {
    let tol = Tol::witness();
    let bench = asm::bench("lands-two-parts", tol);
    let mut session = DocSession::inline(Doc::empty_derived("lands-two-parts", tol), tol);
    let saved = session.perform(SessionOp::Save(bench.dir.join("two-parts.pncad")));
    assert!(saved.refusal.is_none(), "{:?}", saved.refusal);
    let first = common::instance_in(&mut session, bench.post.id);
    session.pump();
    let second = common::instance_in(&mut session, bench.post.id);
    session.pump();
    let doc = session.committed_doc();
    let [one, two] = [first, second].map(|i| doc.node_space(i).expect("a space"));
    assert!(
        one != two && one != VarId::WORLD && two != VarId::WORLD,
        "each part in a fresh space of its own"
    );
    assert!(
        session.display().free_move_of(one).is_none(),
        "the first part, with nothing else drawn, stands at the identity"
    );
    let frame = *session
        .display()
        .free_move_of(two)
        .expect("the second part's space is drawn clear of the first");
    assert!(
        frame.translation[0] > 0.0 && frame.translation[1] == 0.0 && frame.translation[2] == 0.0,
        "stepped along x: {frame:?}"
    );
    assert!(session.perform(SessionOp::Undo).refusal.is_none());
    session.pump();
    assert!(
        session.committed_doc().var(two).is_none(),
        "the undo took the second part's space"
    );
    assert_eq!(
        session.display().free_move_of(two).copied(),
        Some(frame),
        "and kept its frame"
    );
    assert!(session.perform(SessionOp::Redo).refusal.is_none());
    session.pump();
    assert_eq!(
        session.committed_doc().node_space(second),
        Some(two),
        "the redo mints the same space"
    );
    assert_eq!(
        session.display().free_move_of(two).copied(),
        Some(frame),
        "drawn under the same frame, clear of the first"
    );
}
