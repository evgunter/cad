//! **Deleting a feature out of a chain-shaped model.**
//!
//! The die is the shape this suite is about: a blank, twenty-one
//! placed pips subtracted one boolean at a time, and two fillets over
//! the result. Every node in that chain but the outermost is an input
//! to something, so `DocEdit::DeleteNode` alone can delete exactly one
//! node in the whole document — the reason a user reports that
//! deleting a feature is impossible.
//!
//! The recipe SHAPE is what these rows are about, so they build it
//! from cheap geometry rather than loading the tour's gallery file:
//! the gallery is a generated artifact, and a test that reads one is
//! pinned to whether somebody has run the demo. Nothing here
//! evaluates — the cascade is a statement about the document, and the
//! session's inline evaluator only runs inside `pump`.

// Panicking is a test's failure mechanism (workspace lint note).
#![allow(clippy::expect_used)]
#![allow(clippy::panic)]

mod common;

use pncad::document::{
    BooleanOp, Dimension, Doc, DocEdit, Expr, Node, ProfileProgram, RecipeNodeId,
    cascade_delete_order,
};
use pncad::geom_core::Tol;
use viewer::session::{DocSession, SessionOp};

/// How many pips the fixture cuts — the die's own count, which is what
/// makes the label sentence this suite pins a realistic one.
const PIPS: usize = 21;

/// A die-shaped recipe: `PIPS` booleans in a chain, each subtracting
/// one placed pip from the last, under two stacked fillets.
struct Die {
    doc: Doc<ProfileProgram>,
    /// The pip placements — inputs to the chain, downstream of nothing.
    transforms: Vec<RecipeNodeId>,
    /// The subtraction chain, in order.
    booleans: Vec<RecipeNodeId>,
    /// The inner and outer fillet, in that order.
    fillets: [RecipeNodeId; 2],
}

fn die_shaped(tol: Tol) -> Die {
    let doc: Doc<ProfileProgram> = Doc::empty_derived("cascade-die", tol);
    let (doc, blank_profile) = common::framed_square(&doc, 0.04, tol);
    let (doc, blank) = common::inserted(
        &doc,
        Node::Extrude {
            profile: blank_profile,
            distance: common::len(0.04),
        },
        tol,
    );
    let (doc, pip_profile) = common::framed_square(&doc, 0.004, tol);
    let (mut doc, pip) = common::inserted(
        &doc,
        Node::Extrude {
            profile: pip_profile,
            distance: common::len(0.004),
        },
        tol,
    );
    let mut transforms = Vec::new();
    let mut booleans = Vec::new();
    let mut body = blank;
    for i in 0..PIPS {
        let (next, placed) = common::inserted(
            &doc,
            Node::Transform {
                input: pip,
                translation: [
                    common::len(0.001 * f64::from(u32::try_from(i).expect("a small index"))),
                    common::len(0.0),
                    common::len(0.0),
                ],
                rotation_axis: [common::scl(0.0), common::scl(0.0), common::scl(1.0)],
                rotation_angle: Expr::literal(0.0, Dimension::Angle).expect("a finite angle"),
            },
            tol,
        );
        let (next, cut) = common::inserted(
            &next,
            Node::Boolean {
                op: BooleanOp::Subtract,
                a: body,
                b: placed,
                declare: None,
            },
            tol,
        );
        doc = next;
        transforms.push(placed);
        booleans.push(cut);
        body = cut;
    }
    let (doc, inner) = common::inserted(
        &doc,
        Node::Fillet {
            target: body,
            radius: common::len(0.001),
            selection: Vec::new(),
        },
        tol,
    );
    let (doc, outer) = common::inserted(
        &doc,
        Node::Fillet {
            target: inner,
            radius: common::len(0.0005),
            selection: Vec::new(),
        },
        tol,
    );
    Die {
        doc,
        transforms,
        booleans,
        fillets: [inner, outer],
    }
}

/// The live node ids, sorted — a document's identity for these rows.
fn live(doc: &Doc<ProfileProgram>) -> Vec<RecipeNodeId> {
    let mut ids = doc.order().to_vec();
    ids.sort_unstable();
    ids
}

/// Deleting a boolean out of the middle of the chain takes exactly the
/// nodes downstream of it — the rest of the chain and both fillets —
/// and leaves everything that merely FED it alone.
#[test]
fn a_mid_chain_delete_takes_exactly_the_downstream_cone() {
    let tol = Tol::witness();
    let die = die_shaped(tol);
    let cut = 10;
    let target = die.booleans[cut];

    // The order the operation applies: consumers first, target last.
    let mut expected = vec![die.fillets[1], die.fillets[0]];
    expected.extend(die.booleans[cut..].iter().rev().copied());
    assert_eq!(
        cascade_delete_order(&die.doc, target),
        expected,
        "the cone is the rest of the chain plus both fillets, consumers first"
    );

    let before = live(&die.doc);
    let mut session = DocSession::inline(die.doc.clone(), tol);
    let outcome = session.perform(SessionOp::DeleteNode { node: target });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(
        outcome.committed.len(),
        expected.len(),
        "one edit per node the cone holds"
    );

    let after = session.committed_doc();
    for &gone in &expected {
        assert!(after.node(gone).is_none(), "node {gone:?} is gone");
    }
    for (i, &kept) in die.booleans.iter().enumerate().take(cut) {
        assert!(
            after.node(kept).is_some(),
            "boolean {i} is upstream and stays"
        );
    }
    for (i, &placed) in die.transforms.iter().enumerate() {
        assert!(
            after.node(placed).is_some(),
            "pip {i}'s placement only FED the cut and stays — the splice that \
             would rejoin it to the chain is issue #1324"
        );
    }
    assert_eq!(
        after.order().len(),
        before.len() - expected.len(),
        "nothing else moved"
    );

    // The whole cascade is ONE history state, hence one undo.
    assert_eq!(session.history().len(), 2, "root plus one action");
    let current = session.history().current();
    assert_eq!(
        session.history().entry(current).edits().len(),
        expected.len(),
        "the action holds every edit it applied"
    );
    assert!(session.history().can_undo());

    // And one undo restores all of it.
    let out = session.perform(SessionOp::Undo);
    assert!(out.refusal.is_none(), "{:?}", out.refusal);
    assert_eq!(
        live(session.committed_doc()),
        before,
        "one undo brings the whole cone back"
    );
    assert!(
        session.history().can_redo(),
        "and redo can take it away again"
    );
}

/// A feature nothing depends on still takes the single-edit path: one
/// edit in the outcome, one edit in the history entry.
#[test]
fn deleting_a_leaf_commits_exactly_one_edit() {
    let tol = Tol::witness();
    let die = die_shaped(tol);
    let outer = die.fillets[1];
    assert_eq!(cascade_delete_order(&die.doc, outer), vec![outer]);

    let mut session = DocSession::inline(die.doc, tol);
    let outcome = session.perform(SessionOp::DeleteNode { node: outer });
    assert!(outcome.refusal.is_none(), "{:?}", outcome.refusal);
    assert_eq!(outcome.committed, vec![DocEdit::DeleteNode { id: outer }]);
    let current = session.history().current();
    assert_eq!(session.history().entry(current).edits().len(), 1);
}

/// The button states the cost before it is paid: the count is in the
/// LABEL, and the hover breaks it down by kind.
#[test]
fn the_delete_affordance_names_the_count_and_the_kinds() {
    let tol = Tol::witness();
    let die = die_shaped(tol);
    let session = DocSession::inline(die.doc, tol);

    let leaf = session.delete_affordance(die.fillets[1]);
    assert_eq!(leaf.label, "Delete feature 'Fillet'");
    assert_eq!(
        leaf.hover, None,
        "nothing depends on it, so nothing to warn"
    );

    let mid = session.delete_affordance(die.booleans[10]);
    assert_eq!(
        mid.label,
        "Delete feature 'Boolean' and 12 dependent features"
    );
    assert_eq!(
        mid.hover.as_deref(),
        Some("Also deletes 12 features that depend on it: 10 × Boolean, 2 × Fillet"),
        "grouped by kind, most numerous first"
    );

    // The blank is under the entire chain: everything but the pips and
    // their placements goes with it.
    let blank_cascade = session.delete_affordance(die.booleans[0]);
    assert_eq!(
        blank_cascade.label,
        "Delete feature 'Boolean' and 22 dependent features"
    );

    // Singular reads as singular.
    let one = session.delete_affordance(die.fillets[0]);
    assert_eq!(one.label, "Delete feature 'Fillet' and 1 dependent feature");
    assert_eq!(
        one.hover.as_deref(),
        Some("Also deletes 1 feature that depends on it: 1 × Fillet")
    );
}

/// The saved log is FLAT: the file records edits, so a reopened
/// document walks the cascade back one edit at a time. That is the
/// price of grouping being viewer-local state, and it is pinned rather
/// than assumed.
#[test]
fn a_saved_cascade_replays_as_its_individual_edits() {
    let tol = Tol::witness();
    let die = die_shaped(tol);
    let target = die.booleans[10];
    let cone = cascade_delete_order(&die.doc, target).len();
    let mut session = DocSession::inline(die.doc, tol);
    assert!(
        session
            .perform(SessionOp::DeleteNode { node: target })
            .refusal
            .is_none()
    );
    assert_eq!(session.history().path_edits().len(), cone);

    let dir = common::tempdir("cascade-delete");
    let file = dir.join("die.pncad");
    assert!(
        session
            .perform(SessionOp::Save(file.clone()))
            .refusal
            .is_none(),
        "the cascaded document saves, so it is valid"
    );

    let mut reopened = DocSession::inline(die_shaped(tol).doc, tol);
    assert!(reopened.perform(SessionOp::Open(file)).refusal.is_none());
    assert_eq!(
        live(reopened.committed_doc()),
        live(session.committed_doc()),
        "the reopened document is the one that was saved"
    );
    assert_eq!(reopened.history().path_edits().len(), cone);
    assert_eq!(
        reopened.history().len(),
        cone + 1,
        "root plus one state per LOGGED edit: grouping did not survive the file"
    );
}
