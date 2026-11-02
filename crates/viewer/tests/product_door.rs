//! **A body reaches the product through one door**
//! (`crates/viewer/README.md`, the bold paragraph of that name):
//! `SessionOp::AddToProduct`'s three branches — a copy in the world is
//! added to the slot; a placement's copy nothing reads has its placement
//! re-targeted into the world, the `new_space()` it leaves empty
//! deleted, and is added; any other body is placed into the world by the
//! identity bundle and that copy added — each one action that one undo
//! takes back, and the refusal of a copy the product reads already.
//! Where the door is offered (a viewport pick, a tree row, the product's
//! and the `unused` finding's recourses) is `app`'s rows, drawn through
//! the whole app.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

use crate::common;
use pncad::document::{
    Bodies, BodyRead, Doc, DocEdit, Node, Operand, ProfileProgram, RecipeNodeId, VarId,
};
use pncad::geom_core::Tol;
use viewer::session::{DocSession, OpOutcome, Refusal, SessionOp};

/// The box: 20 × 20 × 10 mm, centred on its sketch origin.
const BOX: [f64; 3] = [0.02, 0.02, 0.01];

/// A session holding a fresh box, landed: the session and the extrude.
fn fresh_box(tol: Tol) -> (DocSession, RecipeNodeId) {
    let mut session = common::inline_without_door(Doc::empty_derived("product-door", tol), tol);
    let body = common::xy_box_in(&mut session, BOX);
    session.pump();
    (session, body)
}

/// `node`'s body through the door, landed.
fn add(session: &mut DocSession, node: RecipeNodeId) -> OpOutcome {
    let outcome = session.perform(SessionOp::AddToProduct { node, body: None });
    session.pump();
    outcome
}

/// What the product slot reads, piece by piece.
fn product(session: &DocSession) -> Vec<VarId> {
    viewer::world::product_reads(session.committed_doc())
}

/// **One undo takes the door's action back whole**: the document the
/// undo lands on is `before`, node for node, and its product slot.
fn undone_to(session: &mut DocSession, before: &Doc<ProfileProgram>) {
    let undone = session.perform(SessionOp::Undo);
    assert!(undone.refusal.is_none(), "{:?}", undone.refusal);
    session.pump();
    let doc = session.committed_doc();
    assert_eq!(doc.ids(), before.ids(), "one undo takes every edit back");
    assert_eq!(doc.product(), before.product(), "and the product slot");
}

/// **Any other body is placed into the world and that copy added**: a
/// fresh box lands unplaced, so the door inserts one `Place` of it into
/// the world by the identity bundle and sets the product to its copy —
/// two edits, one action. The box is then drawn as that copy alone.
///
/// *Breaks if* the door adds the box itself (the slot would read a body
/// outside the world), places it anywhere but the world, writes a mate
/// or a value off the identity, or records the two edits as two undos.
#[test]
fn a_fresh_body_is_placed_into_the_world_and_its_copy_added() {
    let tol = Tol::witness();
    let (mut session, body) = fresh_box(tol);
    let before = session.committed_doc().clone();
    assert!(
        before.product().is_none(),
        "nothing joins the product unasked"
    );

    let outcome = add(&mut session, body);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 2, "the placement and the slot");
    let doc = session.committed_doc();
    let [copy] = viewer::world::placements_of(doc, body)[..] else {
        panic!("one copy of the box")
    };
    assert!(
        matches!(doc.node(copy), Some(Node::Place { .. }))
            && common::solve(&session, doc, tol)
                .relative(copy)
                .is_some_and(|at| at.is_identity_bits()),
        "a `Place` standing by the identity bundle"
    );
    assert_eq!(doc.node_space(copy), Some(VarId::WORLD), "into the world");
    assert_eq!(
        product(&session),
        doc.outputs(copy),
        "its copy is the product"
    );
    assert!(
        session.product_fault().is_none(),
        "{:?}",
        session.product_fault()
    );
    assert_eq!(
        viewer::world::drawn_nodes(session.committed_doc()),
        [copy],
        "the box is drawn as its copy"
    );

    undone_to(&mut session, &before);
}

/// **A copy in the world is added to the slot**, and nothing else is
/// written: one `SetProduct` beside what the product read. Asked again,
/// the door refuses `InTheProduct` and records nothing.
///
/// *Breaks if* the door places a second copy of a copy already in the
/// world, drops what the product read, or writes the slot again.
#[test]
fn a_world_copy_is_added_to_the_slot() {
    let tol = Tol::witness();
    let (session, a) = fresh_box(tol);
    let (doc, a_copy) = common::world_copy(session.committed_doc(), a, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let b = common::xy_box_in(&mut session, BOX);
    session.pump();
    // The second box through the door first, so the slot reads one body.
    assert!(add(&mut session, b).refusal.is_none());
    let [b_copy] = viewer::world::placements_of(session.committed_doc(), b)[..] else {
        panic!("one copy of the second box")
    };
    let before = session.committed_doc().clone();
    let ids = before.ids().len();

    let outcome = add(&mut session, a_copy);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let doc = session.committed_doc();
    assert_eq!(
        doc.ids().len(),
        ids + 1,
        "no copy placed: only the `⊔` the two-body slot reads"
    );
    assert_eq!(
        product(&session),
        [b_copy, a_copy].map(|copy| doc.output(copy, 0).expect("a copy")),
        "the copy joins what the product read, after it"
    );

    let again = add(&mut session, a_copy);
    assert!(
        matches!(&again.refusal, Some(Refusal::InTheProduct { copy }) if copy.id() == a_copy),
        "a copy the product reads is refused: {:?}",
        again.refusal
    );
    assert!(again.committed.is_empty(), "and nothing is recorded");

    undone_to(&mut session, &before);
}

/// **A placement's copy nothing reads is moved into the world, not
/// copied**: a world copy given a space of its own by a later gesture
/// stands alone there, so the door re-targets its placement into the
/// world and adds the copy — two edits, one action, no second placement.
/// The space was minted by another step than the copy's, so the door
/// leaves it: it deletes only a space minted with the copy it moves
/// (`instance_authoring`'s inserted part).
///
/// *Breaks if* the door places a copy of the copy, deletes a space it
/// did not mint with the copy, or moves the placement without adding it.
#[test]
fn a_copy_nothing_reads_is_moved_into_the_world_and_a_space_it_did_not_mint_kept() {
    let tol = Tol::witness();
    let (session, body) = fresh_box(tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let own = session.perform(SessionOp::IntoOwnSpace { placement: copy });
    assert!(own.refusal.is_none(), "{:?}", own.refusal);
    session.pump();
    let space = session
        .committed_doc()
        .node_space(copy)
        .filter(|&space| space != VarId::WORLD)
        .expect("the copy stands in a space of its own");
    let before = session.committed_doc().clone();

    let outcome = add(&mut session, copy);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed.len(), 2, "the re-target and the slot");
    let doc = session.committed_doc();
    assert_eq!(
        viewer::world::placements_of(doc, body),
        [copy],
        "no second copy"
    );
    assert_eq!(
        doc.node_space(copy),
        Some(VarId::WORLD),
        "the copy is in the world"
    );
    assert!(
        doc.defined_by(space).is_some(),
        "a space another step minted stays"
    );
    assert_eq!(
        product(&session),
        doc.outputs(copy),
        "and the copy is the product"
    );

    undone_to(&mut session, &before);
}

/// **A copy the slot reads from outside the world is moved back, and
/// the pane does not offer to add it again.** A box through the door,
/// then its copy given a space of its own: the slot reads a body outside
/// the world (`ProductOutsideWorld`). The fault's recourse is the door
/// on that copy, which re-targets it into the world and writes nothing
/// else — the slot already reads it. Asked again, it refuses
/// `InTheProduct`.
///
/// *Breaks if* the fault names no action, the door refuses the stray as
/// in the product, or it writes the slot again.
#[test]
fn a_stray_the_product_reads_is_moved_back_into_the_world() {
    let tol = Tol::witness();
    let (mut session, body) = fresh_box(tol);
    assert!(add(&mut session, body).refusal.is_none());
    let [copy] = viewer::world::placements_of(session.committed_doc(), body)[..] else {
        panic!("one copy of the box")
    };
    let own = session.perform(SessionOp::IntoOwnSpace { placement: copy });
    assert!(own.refusal.is_none(), "{:?}", own.refusal);
    session.pump();
    assert!(
        matches!(
            session.product_fault(),
            Some(pncad::document::ProductError::ProductOutsideWorld { .. })
        ),
        "{:?}",
        session.product_fault()
    );
    let recourse =
        viewer::frame::product_recourse(session.product_fault(), session.committed_doc());
    let [(label, op)] = &recourse[..] else {
        panic!("one action, on the stray copy: {recourse:?}")
    };
    assert!(label.starts_with("Move "), "{label}");
    assert!(
        matches!(op, SessionOp::AddToProduct { node, body: None } if *node == copy),
        "{op:?}"
    );
    let product_before = product(&session);
    let moved = session.perform(SessionOp::AddToProduct {
        node: copy,
        body: None,
    });
    assert!(moved.refusal.is_none(), "{:?}", moved.refusal);
    assert!(
        matches!(&moved.committed[..], [DocEdit::SetParam { .. }]),
        "the re-target alone: {:?}",
        moved.committed
    );
    session.pump();
    assert_eq!(product(&session), product_before, "the slot as it was");
    assert!(
        session.product_fault().is_none(),
        "{:?}",
        session.product_fault()
    );
    let again = add(&mut session, copy);
    assert!(
        matches!(&again.refusal, Some(Refusal::InTheProduct { copy: c }) if c.id() == copy),
        "{:?}",
        again.refusal
    );
}

/// **A body joins the product once**: the door on a box already in the
/// product through its world copy refuses `InTheProduct`, naming that
/// copy, rather than place a second; and on a box whose world copy the
/// slot does not read, it adds that copy rather than stack another on it.
///
/// *Breaks if* the door places the box a second time either way.
#[test]
fn a_body_with_a_world_copy_is_not_placed_again() {
    let tol = Tol::witness();
    let (mut session, body) = fresh_box(tol);
    assert!(add(&mut session, body).refusal.is_none());
    let [copy] = viewer::world::placements_of(session.committed_doc(), body)[..] else {
        panic!("one copy of the box")
    };
    let twice = add(&mut session, body);
    assert!(
        matches!(&twice.refusal, Some(Refusal::InTheProduct { copy: c }) if c.id() == copy),
        "{:?}",
        twice.refusal
    );
    assert!(twice.committed.is_empty());

    let (session, other) = fresh_box(tol);
    let (doc, held) = common::world_copy(session.committed_doc(), other, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let outcome = add(&mut session, other);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let doc = session.committed_doc();
    assert_eq!(
        viewer::world::placements_of(doc, other),
        [held],
        "no second copy"
    );
    assert_eq!(
        product(&session),
        doc.outputs(held),
        "the copy it had is added"
    );
}

/// **A body already in the world is added as itself**: a `⊔` of two
/// world copies, written by hand, stands in the world, so the door sets
/// the slot to it — one `SetProduct`, no copy of it placed on top.
///
/// *Breaks if* the door recognises only placements as world bodies.
#[test]
fn a_world_union_is_added_as_itself() {
    let tol = Tol::witness();
    let (mut session, a) = fresh_box(tol);
    let b = common::xy_box_in(&mut session, BOX);
    let (doc, a_copy) = common::world_copy(session.committed_doc(), a, tol);
    let (doc, b_copy) = common::world_copy(&doc, b, tol);
    let (doc, union) = common::inserted(
        &doc,
        Node::Assemble {
            members: Bodies::Spelled(
                [a_copy, b_copy]
                    .map(|copy| BodyRead::plain(Operand::Node(copy)))
                    .to_vec(),
            ),
        },
        tol,
    );
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let placements = session.committed_doc().placements();
    let outcome = add(&mut session, union);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert!(
        matches!(&outcome.committed[..], [DocEdit::SetProduct { .. }]),
        "{:?}",
        outcome.committed
    );
    let doc = session.committed_doc();
    assert_eq!(doc.placements(), placements, "nothing placed");
    assert_eq!(
        doc.product(),
        doc.output(union, 0),
        "the slot reads the `⊔`"
    );
}

/// **A world copy that carries a mate, and one a mate reads, are added
/// as they stand**: the bench's shelf stands in the world, pinned there
/// by its mate, and the post's seat mate reads its face. Each added is
/// one write of the slot and no re-target: the bundles are unchanged.
///
/// *Breaks if* the world copy's branch is lost and the door re-targets
/// a copy already in the world (resetting a mated bundle) or places
/// another.
#[test]
fn a_mated_world_copy_is_added_with_its_bundle_untouched() {
    let tol = Tol::witness();
    let bench = common::asm::bench("product-door-mated", tol);
    let mut session = common::asm::open_bench(&bench, tol);
    common::asm::unpin(&mut session, bench.post_b);
    common::commit_mate(
        &mut session,
        common::asm::seat_op(&bench, bench.post_b, common::asm::middle_seat_values()),
    );
    let emptied = session.perform(SessionOp::SetProduct { bodies: vec![] });
    assert!(emptied.refusal.is_none(), "{:?}", emptied.refusal);
    session.pump();
    for placement in [bench.shelf_i, bench.post_b] {
        let doc = session.committed_doc();
        assert_eq!(doc.node_space(placement), Some(VarId::WORLD));
        let bundle = crate::fixture::bundle(doc, placement);
        let placements = doc.placements();
        let outcome = add(&mut session, placement);
        assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
        assert!(
            outcome.committed.iter().all(|edit| !matches!(
                edit,
                DocEdit::SetParam { .. } | DocEdit::SetConstraints { .. }
            )),
            "{placement:?}: no re-target, no bundle written: {:?}",
            outcome.committed
        );
        assert_eq!(
            outcome
                .committed
                .iter()
                .filter(|edit| matches!(edit, DocEdit::SetProduct { product: Some(_) }))
                .count(),
            1,
            "one write of the slot"
        );
        let doc = session.committed_doc();
        assert_eq!(crate::fixture::bundle(doc, placement), bundle);
        assert_eq!(doc.placements(), placements, "nothing placed");
    }
    assert_eq!(
        product(&session),
        [bench.shelf_i, bench.post_b]
            .map(|p| session.committed_doc().output(p, 0).expect("a copy"))
    );
}

/// **A copy something in its space reads stays where it is**: the copy
/// in a space of its own read by a `⊔` beside it is drawn as that `⊔`'s
/// piece, and moving its placement would move what the `⊔` reads — so
/// the door places the copy into the world by the identity bundle and
/// adds that one, leaving the placement and its space as they were.
///
/// *Breaks if* the door re-targets a copy something reads, or deletes
/// the space a reader still reads in.
#[test]
fn a_copy_something_reads_is_placed_rather_than_moved() {
    let tol = Tol::witness();
    let (session, body) = fresh_box(tol);
    let (doc, copy) = common::world_copy(session.committed_doc(), body, tol);
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    assert!(
        session
            .perform(SessionOp::IntoOwnSpace { placement: copy })
            .refusal
            .is_none()
    );
    let space = session.committed_doc().node_space(copy);
    let read = session.committed_doc().output(copy, 0).expect("a copy");
    let (doc, _) = common::inserted(
        session.committed_doc(),
        Node::Assemble {
            members: Bodies::Spelled(vec![BodyRead::plain(Operand::Var(read))]),
        },
        tol,
    );
    let mut session = common::inline_without_door(doc, tol);
    session.pump();
    let before = session.committed_doc().clone();

    let outcome = add(&mut session, copy);
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    let doc = session.committed_doc();
    assert_eq!(
        doc.node_space(copy),
        space,
        "the read copy stays in its space"
    );
    let placed: Vec<RecipeNodeId> = doc
        .placements()
        .into_iter()
        .filter(|&p| matches!(doc.node(p), Some(Node::Place { source, .. }) if source.read == read))
        .collect();
    let [world] = placed[..] else {
        panic!("one placement of the copy: {placed:?}")
    };
    assert_eq!(doc.node_space(world), Some(VarId::WORLD));
    assert_eq!(product(&session), doc.outputs(world), "that copy is added");

    undone_to(&mut session, &before);
}

/// **A node that names no one body refuses**, typed, and records
/// nothing: a split's halves are two bodies, so a tree row (no pick)
/// names neither, while a pick names the half the ray met.
///
/// *Breaks if* the door guesses a half, or refuses the pick too.
#[test]
fn a_split_needs_the_half_a_pick_names() {
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
    let row = add(&mut session, split);
    assert!(
        matches!(row.refusal, Some(Refusal::NotABodyRead { .. })),
        "{:?}",
        row.refusal
    );
    assert!(row.committed.is_empty());
    let picked = session.perform(SessionOp::AddToProduct {
        node: split,
        body: Some(1),
    });
    assert!(picked.refusal.is_none(), "{:?}", picked.refusal);
    let doc = session.committed_doc();
    let half = doc.output(split, 1).expect("the upper half");
    let [copy] = doc
        .placements()
        .into_iter()
        .filter(|&p| matches!(doc.node(p), Some(Node::Place { source, .. }) if source.read == half))
        .collect::<Vec<_>>()[..]
    else {
        panic!("the half the pick named is placed")
    };
    assert_eq!(product(&session), doc.outputs(copy));
}
